// We have to go through this little bit of indirection because of how integration directory
// structure works.

use httpmock::{prelude::*, Method, Then, When};
use once_cell::sync::Lazy;
use reqwest::Url;
use seaplane_cli::context::Ctx;
use serde_json::json;

macro_rules! argv {
    ($argv:expr) => {{
        seaplane_cli::test_cli(
            const_format::concatcp!("seaplane --stateless --api-key abc123 ", $argv).split(" "),
        )
        .unwrap()
    }};
}

macro_rules! run {
    ($argv:expr) => {{
        seaplane_cli::test_main_exec_with_ctx(
            &argv!($argv),
            $crate::api::ctx_from_url(MOCK_SERVER.base_url()),
        )
    }};
}

mod account;
mod formation;
mod locks;
mod metadata;
mod restrict;

fn ctx_from_url(url: String) -> Ctx {
    let mut ctx = Ctx::default();
    let url: Url = url.parse().unwrap();
    ctx.compute_url = Some(url.clone());
    ctx.identity_url = Some(url.clone());
    ctx.metadata_url = Some(url.clone());
    ctx.locks_url = Some(url.clone());
    ctx
}

// To be used with httpmock standalone server for dev testing
// MockServer::connect("127.0.0.1:5000")
static MOCK_SERVER: Lazy<MockServer> = Lazy::new(|| {
    let resp_json =
        json!({"token": "abc.123.def", "tenant": "tnt-abcdef1234567890", "subdomain": "pequod"});
    let s = MockServer::start();
    // let s = MockServer::connect("127.0.0.1:5000");
    let _mock = s.mock(|when, then| {
        when.method(POST)
            .path("/identity/token")
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
