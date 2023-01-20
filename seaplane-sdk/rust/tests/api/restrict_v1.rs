use httpmock::{prelude::*, Method, Then, When};
use seaplane::api::{
    restrict::v1::{RestrictRequestBuilder, RestrictionDetails},
    shared::v1::RangeQueryContext,
};
use serde_json::json;

use super::MOCK_SERVER;

fn when(when: When, m: Method, p: &str) -> When {
    when.method(m)
        .path(p)
        .header("authorization", "Bearer abc123")
        .header("accept", "*/*")
        .header("host", format!("{}:{}", MOCK_SERVER.host(), MOCK_SERVER.port()))
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
        "api": "Config",
        "directory": "Zm9vL2Jhcg",
        "details": {
            "regions_allowed": ["XE"],
            "regions_denied": [],
            "providers_allowed": [],
            "providers_denied": []
        },
        "state": "Enforced"
    });

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/restrict/config/base64:Zm9vL2Jhcg/");
        then(t, resp_json.clone());
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

// GET /restrict/config/?from=base64:{from_key}
#[test]
fn get_api_page() {
    let resp_json = json!({
        "restrictions": [
            {
            "api": "Config",
            "directory": "Zm9vL2Jhcg",
            "details": {
                "regions_allowed": ["XE"],
                "regions_denied": [],
                "providers_allowed": [],
                "providers_denied": []
            },
            "state": "Enforced"
        },
        {
            "api": "Config",
            "directory": "Zm9vL2Jheg",
            "details": {
                "regions_allowed": ["XN"],
                "regions_denied": [],
                "providers_allowed": [],
                "providers_denied": []
            },
            "state": "Enforced"
        }
    ]});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/restrict/config/");
        then(t, resp_json.clone());
    });

    let context = RangeQueryContext::new();
    let req = partial_build()
        .api_range("config", context)
        .build()
        .unwrap();
    let resp = req.get_page().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}

// GET /restrict/config/?from=base64:{from_key}
#[test]
fn get_all_page() {
    let resp_json = json!({
        "restrictions": [
        {
            "api": "Config",
            "directory": "Zm9vL2Jhcg",
            "details": {
                "regions_allowed": ["XE"],
                "regions_denied": [],
                "providers_allowed": [],
                "providers_denied": []
            },
            "state": "Enforced"
        },
        {
            "api": "Locks",
            "directory": "Zm9vL2Jheg",
            "details": {
                "regions_allowed": ["XN"],
                "regions_denied": [],
                "providers_allowed": [],
                "providers_denied": []
            },
            "state": "Enforced"
        }
    ]});

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/restrict/");
        then(t, resp_json.clone());
    });

    let context = RangeQueryContext::new();
    let req = partial_build()
        .all_range::<String>(None, context)
        .build()
        .unwrap();

    let resp = req.get_page().unwrap();

    // Ensure the endpoint was hit
    mock.assert();
    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}
