use httpmock::prelude::*;
use seaplane::api::TokenRequest;
use serde_json::json;

fn mock_server() -> MockServer {
    MockServer::start()

    // To be used with httpmock standalone server for dev testing
    // MockServer::connect("127.0.0.1:5000")
}

fn build_req(mock_server: &MockServer) -> TokenRequest {
    TokenRequest::builder()
        .api_key("abc123")
        .base_url(mock_server.base_url())
        .build()
        .unwrap()
}

// POST /token
#[test]
fn access_token() {
    let mock_server = mock_server();
    let mock = mock_server.mock(|when, then| {
        when.method(POST)
            .path("/token")
            .header("authorization", "Bearer abc123")
            .header("accept", "*/*")
            .header(
                "host",
                &format!("{}:{}", mock_server.host(), mock_server.port()),
            );
        then.status(201).body("abc.123.def");
    });

    let req = build_req(&mock_server);
    let resp = req.access_token().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, "abc.123.def");
}

// Accept: application/json POST /token
#[test]
fn access_token_json() {
    let resp_json = json!({"token": "abc.123.def", "tenant": 1_u64, "subdomain": "pequod"});
    let mock_server = mock_server();
    let mock = mock_server.mock(|when, then| {
        when.method(POST)
            .path("/token")
            .header("authorization", "Bearer abc123")
            .header("accept", "application/json")
            .header(
                "host",
                &format!("{}:{}", mock_server.host(), mock_server.port()),
            );
        then.status(201).json_body(resp_json.clone());
    });

    let req = build_req(&mock_server);
    let resp = req.access_token_json().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}
