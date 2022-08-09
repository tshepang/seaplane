use httpmock::{prelude::*, Method, Then, When};
use once_cell::sync::Lazy;
use seaplane::api::v1::{RestrictRequestBuilder, RestrictionDetails};
use serde_json::json;

// To be used with httpmock standalone server for dev testing
// MockServer::connect("127.0.0.1:5000");
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

fn partial_build() -> RestrictRequestBuilder {
    RestrictRequestBuilder::new()
        .token("abc123")
        .base_url(MOCK_SERVER.base_url())
}

// GET /restrict/{api}/base64:{key}/
#[test]
fn get_restriction() {
    let resp_json = json!({
        "api": "config",
        "directory": "Zm9vL2Jhcg",
        "details": {
            "regions_allowed": ["XE"],
            "regions_denied": [],
            "providers_allowed": [],
            "providers_denied": []
        },
        "state": "enforced"
    });

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/restrict/config/base64:Zm9vL2Jhcg/");
        then(t, json!(resp_json));
    });

    let req = partial_build()
        .single_restriction("config", "Zm9vL2Jhcg")
        .build()
        .unwrap();
    let resp = req.get_restriction().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}

// PUT /restrict/{api}/base64:{key}/
#[test]
fn set_restriction() {
    let resp_json = json!({"status": 200, "title": "Ok"});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, PUT, "/v1/restrict/config/base64:Zm9vL2Jhcg/")
            .header("content-type", "application/json");
        then(t, resp_json);
    });

    let req = partial_build()
        .single_restriction("config", "Zm9vL2Jhcg")
        .build()
        .unwrap();
    let details: RestrictionDetails =
        serde_json::from_str("{\"regions_allowed\": [\"xe\"]}").unwrap();

    let resp = req.set_restriction(details);

    // Ensure the endpoint was hit
    mock.assert();

    assert!(resp.is_ok())
}

// DELETE /restrict/{api}/base64:{key}/
#[test]
fn delete_restriction() {
    let resp_json = json!({"status": 200, "title": "Ok"});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, DELETE, "/v1/restrict/config/base64:Zm9vL2Jhcg/");
        then(t, resp_json);
    });

    let req = partial_build()
        .single_restriction("config", "Zm9vL2Jhcg")
        .build()
        .unwrap();
    let resp = req.delete_restriction();

    // Ensure the endpoint was hit
    mock.assert();

    assert!(resp.is_ok())
}
