use httpmock::{prelude::*, Method, Then, When};
use once_cell::sync::Lazy;
use seaplane::api::v1::{ConfigRequestBuilder, RangeQueryContext};
use serde_json::json;

static MOCK_SERVER: Lazy<MockServer> = Lazy::new(|| MockServer::start());

fn when(when: When, m: Method, p: &str) -> When {
    when.method(m)
        .path(p)
        .header("authorization", "Bearer abc123")
        .header("accept", "*/*")
        .header(
            "host",
            &format!("{}:{}", MOCK_SERVER.host(), MOCK_SERVER.port()),
        )
}

fn then(then: Then, resp_body: serde_json::Value) -> Then {
    then.status(200)
        .header("content-type", "application/json")
        .json_body(resp_body)
}

fn partial_build() -> ConfigRequestBuilder {
    ConfigRequestBuilder::new()
        .token("abc123")
        .base_url(MOCK_SERVER.base_url())
}

// GET /config/base64:{key}
#[test]
fn get_value() {
    let resp_json = json!({"key": "foo", "value": "bar"});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/config/base64:foo");
        then(t, resp_json.clone());
    });

    let req = partial_build().encoded_key("foo").build().unwrap();
    let resp = req.get_value().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}

// GET /config/[base64:{dir}/][?after=base64:{key}]
#[test]
fn get_root_values() {
    let resp_json = json!({"more": false, "last": "baz", "kvs": [{"key": "foo", "value": "bar"}, {"key": "baz", "value": "buz"}]});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/config/");
        then(t, resp_json.clone());
    });

    let range = RangeQueryContext {
        dir: None,
        after: None,
    };

    let req = partial_build().range(range).build().unwrap();
    let resp = req.get_page().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}
