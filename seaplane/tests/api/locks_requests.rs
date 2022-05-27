use httpmock::{prelude::*, Method, Then, When};
use once_cell::sync::Lazy;
use seaplane::api::v1::{HeldLock, LockID, LockInfo, LockInfoInner, LockName, LocksRequestBuilder};
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

fn partial_build() -> LocksRequestBuilder {
    LocksRequestBuilder::new()
        .token("abc123")
        .base_url(MOCK_SERVER.base_url())
}

// PUT /locks/base64:{key}?id={id}&ttl={ttl}&client-id={client_id}
#[test]
fn acquire_lock() {
    let resp = HeldLock::new(
        LockName::from_encoded("Zm9v"),
        LockID::from_encoded("D4lbVpdBE_U"),
        2,
    );

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, PUT, "/v1/locks/base64:Zm9v")
            .query_param("ttl", "10")
            .query_param("client-id", "test-client");
        then(t, json!(resp));
    });

    let req = partial_build().encoded_lock_name("Zm9v").build().unwrap();
    let resp_val = req.acquire(10, "test-client").unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp_val, resp);
}

// PATCH /locks/base64:{key}?id={id}&ttl={ttl}
#[test]
fn renew_lock() {
    let resp_json = json!({"status": 200, "title": "Ok"});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, Method::PATCH, "/v1/locks/base64:Zm9j")
            .query_param("id", "D4lbVpdBE_U")
            .query_param("ttl", "10");
        then(t, resp_json);
    });

    let lock = HeldLock::new(
        LockName::from_encoded("Zm9j"),
        LockID::from_encoded("D4lbVpdBE_U"),
        2,
    );

    let req = partial_build().held_lock(lock).build().unwrap();
    let resp = req.renew(10);

    // Ensure the endpoint was hit
    mock.assert();

    assert!(resp.is_ok())
}

// DELETE /locks/base64:{key}?id={id}
#[test]
fn release_lock() {
    let resp_json = json!({"status": 200, "title": "Ok"});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, Method::DELETE, "/v1/locks/base64:Zm9k").query_param("id", "D4lbVpdBE_U");
        then(t, resp_json);
    });

    let lock = HeldLock::new(
        LockName::from_encoded("Zm9k"),
        LockID::from_encoded("D4lbVpdBE_U"),
        2,
    );

    let req = partial_build().held_lock(lock).build().unwrap();
    let resp = req.release();

    // Ensure the endpoint was hit
    mock.assert();

    assert!(resp.is_ok())
}

// GET /locks/base64:{key}
#[test]
fn list_lock() {
    let resp = LockInfo {
        name: LockName::from_encoded("Zm9l"),
        id: LockID::from_encoded("D4lbVpdBE_U"),
        info: LockInfoInner {
            ttl: 5,
            client_id: "test-client".to_string(),
            ip: "192.0.2.137".to_string(),
        },
    };

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/locks/base64:Zm9l");
        then(t, json!(resp));
    });

    let req = partial_build().encoded_lock_name("Zm9l").build().unwrap();
    let resp_val = req.get_lock_info().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp_val, resp);
}
