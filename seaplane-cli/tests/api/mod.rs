// We have to go through this little bit of indirection because of how integration directory
// structure works.

use clap::ArgMatches;
use httpmock::{prelude::*, Method, Then, When};
use once_cell::sync::Lazy;
use reqwest::Url;
use seaplane_cli::{
    cli::{CliCommand, Seaplane},
    context::Ctx,
    error::CliError,
};
use serde_json::json;

macro_rules! cli {
    ($argv:expr) => {{
        seaplane_cli::test_run(
            const_format::concatcp!("seaplane --stateless --api-key abc123 ", $argv).split(" "),
        )
        .unwrap()
    }};
}

mod account;
mod formation;
mod locks;
mod metadata;

fn test_main(matches: &ArgMatches, url: String) -> Result<(), CliError> {
    let mut ctx = Ctx::default();
    let url: Url = url.parse().unwrap();
    ctx.compute_url = Some(url.clone());
    ctx.identity_url = Some(url.clone());
    ctx.metadata_url = Some(url.clone());
    ctx.locks_url = Some(url.clone());
    test_main_with_ctx(matches, ctx)
}

fn test_main_with_ctx(matches: &ArgMatches, mut ctx: Ctx) -> Result<(), CliError> {
    let s: Box<dyn CliCommand> = Box::new(Seaplane);
    s.traverse_exec(&matches, &mut ctx)?;
    Ok(())
}

// To be used with httpmock standalone server for dev testing
// MockServer::connect("127.0.0.1:5000")
static MOCK_SERVER: Lazy<MockServer> = Lazy::new(|| {
    let resp_json = json!({"token": "abc.123.def", "tenant": 1_u64, "subdomain": "pequod"});
    let s = MockServer::start();
    // let s = MockServer::connect("127.0.0.1:5000");
    let _mock = s.mock(|when, then| {
        when.method(POST)
            .path("/token")
            .header("authorization", "Bearer abc123")
            .header("accept", "application/json");
        then.status(201).json_body(resp_json.clone());
    });
    s
});

fn when_json(when: When, m: Method, p: impl Into<String>) -> When {
    when.method(m)
        .path(p)
        .header("authorization", "Bearer abc.123.def")
        .header("content-type", "application/json")
}

fn when(when: When, m: Method, p: impl Into<String>) -> When {
    when.method(m)
        .path(p)
        .header("authorization", "Bearer abc.123.def")
}

fn then(then: Then, resp_body: &serde_json::Value) -> Then {
    then.status(200).json_body_obj(resp_body)
}
