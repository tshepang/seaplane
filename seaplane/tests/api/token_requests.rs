use httpmock::prelude::*;
use once_cell::sync::Lazy;
use seaplane::api::TokenRequest;

static MOCK_SERVER: Lazy<MockServer> = Lazy::new(|| MockServer::start());

fn build_req() -> TokenRequest {
    TokenRequest::builder()
        .api_key("abc123")
        .base_url(MOCK_SERVER.base_url())
        .build()
        .unwrap()
}

// POST /token
#[test]
fn access_token() {
    let mock = MOCK_SERVER.mock(|when, then| {
        when.method(POST)
            .path("/token")
            .header("authorization", "Bearer abc123")
            .header("accept", "*/*")
            .header(
                "host",
                &format!("{}:{}", MOCK_SERVER.host(), MOCK_SERVER.port()),
            );
        then.status(201).body("abc.123.def");
    });

    let req = build_req();
    let resp = req.access_token().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, "abc.123.def");
}
