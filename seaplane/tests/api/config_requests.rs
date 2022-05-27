use httpmock::{prelude::*, Method, Then, When};
use once_cell::sync::Lazy;
use seaplane::api::v1::{ConfigRequestBuilder, Directory, Key, KeyValue, RangeQueryContext, Value};
use serde_json::json;

// To be used with httpmock standalone server for dev testing
// MockServer::connect("127.0.0.1:5000")
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
        key: Key::from_encoded("Zm9v".to_string()),
        value: Value::from_encoded("Zm9v".to_string()),
    };

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/config/base64:Zm9v");
        then(t, json!(resp));
    });

    let req = partial_build().encoded_key("Zm9v").build().unwrap();
    let resp_val = req.get_value().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp_val, resp.value);
}

// GET /config/
#[test]
fn get_root_values() {
    let resp_json = json!({"next_key": None::<String>, "kvs": [{"key": "foo", "value": "bar"}, {"key": "baz", "value": "buz"}]});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/config/");
        then(t, resp_json.clone());
    });

    let range = RangeQueryContext::new();
    let req = partial_build().range(range).build().unwrap();
    let resp = req.get_page().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}

// GET /config/[base64:{dir}/][?from=base64:{key}]
#[test]
fn get_values_from() {
    let resp_json = json!({"next_key": None::<String>, "kvs": [{"key": "foo", "value": "bar"}, {"key": "baz", "value": "buz"}]});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/config/base64:bWFuIGFzY2lp/").query_param("from", "base64:aGVsbG8");
        then(t, resp_json.clone());
    });

    let mut range = RangeQueryContext::new();
    range.set_from(Key::from_encoded("aGVsbG8"));
    range.set_directory(Directory::from_encoded("bWFuIGFzY2lp"));
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
        when(w, PUT, "/v1/config/base64:Zm9vMQ").header("content-type", "application/octet-stream");
        then(t, resp_json);
    });

    let req = partial_build().encoded_key("Zm9vMQ").build().unwrap();
    let resp = req.put_value(Value::from_encoded("YmFy"));

    // Ensure the endpoint was hit
    mock.assert();

    assert!(resp.is_ok())
}

// PUT /config/base64:{key}
#[test]
fn put_value_unencoded() {
    let resp_json = json!({"status": 200, "title": "Ok"});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, PUT, "/v1/config/base64:Zm9vMg").header("content-type", "application/octet-stream");
        then(t, resp_json);
    });

    let req = partial_build().encoded_key("Zm9vMg").build().unwrap();
    let resp = req.put_value_unencoded("bar");

    // Ensure the endpoint was hit
    mock.assert();

    assert!(resp.is_ok())
}

// DELETE /config/base64:{key}
#[test]
fn delete_value() {
    let resp_json = json!({"status": 200u32, "title": "Ok"});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, DELETE, "/v1/config/base64:Zm9v");
        then(t, resp_json);
    });

    let req = partial_build().encoded_key("Zm9v").build().unwrap();
    let resp = req.delete_value();

    // Ensure the endpoint was hit
    mock.assert();

    assert!(resp.is_ok())
}
