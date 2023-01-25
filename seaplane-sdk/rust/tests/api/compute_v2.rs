use httpmock::{prelude::*, Method, Then, When};
use seaplane::api::compute::v2::{Flight, FlightHealthStatus, Formation, FormationsRequest};
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

fn build_req() -> FormationsRequest {
    FormationsRequest::builder()
        .token("abc123")
        .base_url(MOCK_SERVER.base_url())
        .name("stubb")
        .build()
        .unwrap()
}

fn build_formation() -> Formation {
    Formation::builder()
        .add_flight(
            Flight::builder()
                .name("pequod")
                .image("registry.hub.docker.com/stubb/alpine:latest")
                .build()
                .unwrap(),
        )
        .add_flight(
            Flight::builder()
                .name("flask")
                .image("registry.hub.docker.com/stubb/alpine:latest")
                .build()
                .unwrap(),
        )
        .gateway_flight("pequod")
        .build()
        .unwrap()
}

// GET /formations
#[test]
fn list_formations() {
    let resp_json = r#"[{
        "name": "example-formation",
        "oid": "frm-agc6amh7z527vijkv2cutplwaa",
        "flights": [{
            "name": "example-flight",
            "oid": "flt-agc6amh7z527vijkv2cutplwaa",
            "image": "registry.cplane.cloud/seaplane-demo/nginxdemos/hello:latest"
        }],
        "gateway-flight": "example-flight"
    }]"#;
    let resp_t: Vec<Formation> = serde_json::from_str(resp_json).unwrap();

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v2beta/formations");
        then(t, serde_json::to_value(resp_t.clone()).unwrap());
    });

    let req = build_req();
    let resp = req.list().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, resp_t);
}

// GET /formations/OID
#[test]
fn get_formation() {
    let resp_json = r#"{
        "name": "example-formation",
        "oid": "frm-agc6amh7z527vijkv2cutplwaa",
        "flights": [{
            "name": "example-flight",
            "oid": "flt-agc6amh7z527vijkv2cutplwaa",
            "image": "registry.cplane.cloud/seaplane-demo/nginxdemos/hello:latest"
        }],
        "gateway-flight": "example-flight"
    }"#;
    let resp_t: Formation = serde_json::from_str(resp_json).unwrap();

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v2beta/formations/stubb").header("content-type", "application/json");
        then(t, serde_json::to_value(resp_t.clone()).unwrap());
    });

    let req = build_req();
    let resp = req.get().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, resp_t);
}

// GET /formations/NAME/status
#[test]
fn get_formation_status() {
    let resp_json = json!({
        "name": "example-formation",
        "oid": "frm-agc6amh7z527vijkv2cutplwaa",
        "flights": [{
            "name": "example-flight",
            "oid": "flt-agc6amh7z527vijkv2cutplwaa",
            "health": "healthy".parse::<FlightHealthStatus>().unwrap()
        }],
    });
    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v2beta/formations/frm-agc6amh7z527vijkv2cutplwaa/status")
            .header("content-type", "application/json");
        then(t, resp_json.clone());
    });

    let req = build_req();
    let resp = req.status().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}

// POST /formations/NAME
#[test]
fn create_formation() {
    let resp_body = json!({"oid":"frm-agc6amh7z527vijkv2cutplwaa"});
    let mock = MOCK_SERVER.mock(|w, then| {
        when(w, POST, "/v2beta/formations")
            .header("content-type", "application/json")
            .json_body_obj(&build_formation());
        then.status(201)
            .header("content-type", "application/json")
            .header("Location", "https://stubb.tenant.on.cplane.cloud")
            .json_body(resp_body);
    });

    let req = build_req();
    assert!(req.create(&build_formation()).is_ok());

    // Ensure the endpoint was hit
    mock.assert();
}

// DELETE /formations/NAME
#[test]
fn delete_formation() {
    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, DELETE, "/v2beta/formations/frm-agc6amh7z527vijkv2cutplwaa")
            .header("content-type", "application/json");
        t.status(200);
    });

    let req = build_req();
    assert!(req.delete().is_ok());

    // Ensure the endpoint was hit
    mock.assert();
}
