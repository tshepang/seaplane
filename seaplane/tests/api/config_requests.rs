use httpmock::{prelude::*, Method, Then, When};
use once_cell::sync::Lazy;
use seaplane::api::v1::{ConfigRequestBuilder, Key, KeyValue, RangeQueryContext, Value};
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
    let resp = KeyValue {
        key: Key("foo".to_string()),
        value: Value("bar".to_string()),
    };

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/config/base64:foo");
        then(t, json!(resp));
    });

    let req = partial_build().encoded_key("foo").build().unwrap();
    let resp_val = req.get_value().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp_val, resp.value);
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

// PUT /config/base64:{key}
#[test]
fn put_value() {
    let resp_json = json!({"status": 200, "title": "Ok"});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, PUT, "/v1/config/base64:foo");
        then(t, resp_json);
    });

    let req = partial_build().encoded_key("foo").build().unwrap();
    let resp = req.put_value("bar").unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, ());
}

// DELETE /config/base64:{key}
#[test]
fn delete_value() {
    let resp_json = json!({"status": 200u32, "title": "Ok"});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, DELETE, "/v1/config/base64:foo");
        then(t, resp_json);
    });

    let req = partial_build().encoded_key("foo").build().unwrap();
    let resp = req.delete_value().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, ());
}
