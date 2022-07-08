use httpmock::{prelude::*, Method, Then, When};
use once_cell::sync::Lazy;
use seaplane::api::v1::{
    Directory, HeldLock, LockId, LockInfo, LockInfoInner, LockInfoRange, LockName,
    LocksRequestBuilder, RangeQueryContext,
};
use serde_json::json;

// To be used with httpmock standalone server for dev testing
// MockServer::connect("127.0.0.1:5000")
// static MOCK_SERVER: Lazy<MockServer> = Lazy::new(|| MockServer::connect("127.0.0.1:5000"));
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
    let resp_json = json!({
       "id": "D4lbVpdBE_U",
       "sequencer": 2
    });

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, PUT, "/v1/locks/base64:Zm9v")
            .query_param("ttl", "10")
            .query_param("client-id", "test-client");
        then(t, json!(resp_json));
    });

    let req = partial_build().encoded_lock_name("Zm9v").build().unwrap();
    let resp = req.acquire(10, "test-client").unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    let lock = HeldLock::new(
        LockName::from_encoded("Zm9v"),
        LockId::from_encoded("D4lbVpdBE_U"),
        2,
    );
    assert_eq!(lock, resp);
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
        LockId::from_encoded("D4lbVpdBE_U"),
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
        LockId::from_encoded("D4lbVpdBE_U"),
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
fn list_single_lock() {
    let resp = LockInfo {
        name: LockName::from_encoded("Zm9l"),
        id: LockId::from_encoded("D4lbVpdBE_U"),
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

// GET /locks/
#[test]
fn get_root_values() {
    let resp = LockInfoRange {
        next: None,
        infos: vec![
            LockInfo {
                name: LockName::from_encoded("Zm9l"),
                id: LockId::from_encoded("D4lbVpdBE_U"),
                info: LockInfoInner {
                    ttl: 5,
                    client_id: "test-client".to_string(),
                    ip: "192.0.2.137".to_string(),
                },
            },
            LockInfo {
                name: LockName::from_encoded("Zm9j"),
                id: LockId::from_encoded("D4lbVpdBF_U"),
                info: LockInfoInner {
                    ttl: 10,
                    client_id: "test-client".to_string(),
                    ip: "192.0.2.137".to_string(),
                },
            },
        ],
    };

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/locks/");
        then(t, json!(resp));
    });

    let range = RangeQueryContext::new();
    let req = partial_build().range(range).build().unwrap();
    let resp_val = req.get_page().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp_val, resp);
}

// GET /locks/base64:{dir}/?from=base64:{from_key}
#[test]
fn get_dir() {
    let resp = LockInfoRange {
        next: None,
        infos: vec![
            LockInfo {
                name: LockName::from_encoded("dGVzdC1kaXIvb25l"),
                id: LockId::from_encoded("D4lbVpdBE_U"),
                info: LockInfoInner {
                    ttl: 5,
                    client_id: "test-client".to_string(),
                    ip: "192.0.2.137".to_string(),
                },
            },
            LockInfo {
                name: LockName::from_encoded("dGVzdC1kaXIvdHdv"),
                id: LockId::from_encoded("D4lbVpdBF_U"),
                info: LockInfoInner {
                    ttl: 10,
                    client_id: "test-client".to_string(),
                    ip: "192.0.2.137".to_string(),
                },
            },
        ],
    };

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/locks/base64:dGVzdC1kaXI/").query_param("from", "base64:dGVzdC1kaXIvbw");
        then(t, json!(resp));
    });

    let mut range: RangeQueryContext<LockName> = RangeQueryContext::new();
    range.set_from(LockName::from_unencoded("test-dir/o"));
    range.set_directory(Directory::from_unencoded("test-dir"));
    let req = partial_build().range(range).build().unwrap();
    let resp_val = req.get_page().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp_val, resp);
}
