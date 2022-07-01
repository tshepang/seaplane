use httpmock::prelude::*;
use once_cell::sync::Lazy;
use seaplane_cli::printer::printer;
use serde_json::json;

use super::test_main;

// To be used with httpmock standalone server for dev testing
// MockServer::connect("127.0.0.1:5000")
// static MOCK_SERVER: Lazy<MockServer> = Lazy::new(|| MockServer::connect("127.0.0.1:5000"));
static MOCK_SERVER: Lazy<MockServer> = Lazy::new(|| MockServer::start());

#[test]
fn account_token() {
    let mut mock = MOCK_SERVER.mock(|when, then| {
        when.method(POST)
            .path("/token")
            .header("authorization", "Bearer abc123");
        then.status(201).body("abc.123.def");
    });

    let res = test_main(&cli!("account token"), MOCK_SERVER.base_url());
    assert!(res.is_ok());
    mock.assert();

    assert_eq!(printer().as_string().trim(), "abc.123.def");

    // Prep for next test to not conflict
    mock.delete();
    printer().clear();

    let resp_json = json!({"token": "abc.123.def", "tenant": 1_u64, "subdomain": "pequod"});
    let mock = MOCK_SERVER.mock(|when, then| {
        when.method(POST)
            .path("/token")
            .header("authorization", "Bearer abc123")
            .header("accept", "application/json");
        then.status(201).json_body(resp_json.clone());
    });

    let res = test_main(&cli!("account token --json"), MOCK_SERVER.base_url());
    assert!(res.is_ok());
    mock.assert();
    assert_eq!(
        printer().as_string().trim(),
        r#"{"token":"abc.123.def","tenant":1,"subdomain":"pequod"}"#
    );

    printer().clear();
}
