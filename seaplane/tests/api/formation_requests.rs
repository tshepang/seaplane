use httpmock::{prelude::*, Method, Then, When};
use once_cell::sync::Lazy;
use seaplane::api::v1::{
    ActiveConfiguration, ActiveConfigurations, Flight, FormationConfiguration, FormationsRequest,
};
use serde_json::json;
use uuid::Uuid;

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

fn build_req() -> FormationsRequest {
    FormationsRequest::builder()
        .token("abc123")
        .base_url(MOCK_SERVER.base_url())
        .name("Stubb")
        .build()
        .unwrap()
}

// GET /formations
#[test]
fn list_names() {
    let resp_json = json!([{"name":"bar"}, {"name": "baz"}, {"name":"qux"}]);

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/formations");
        then(t, resp_json.clone());
    });

    let req = build_req();
    let resp = req.list_names().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}

// POST /formations/NAME?active=false&source=OTHER (empty body)
#[test]
fn clone_from() {
    let resp_json = json!(["557f87c3-b26c-428c-b970-cb8acac2bd68"]);

    let mock = MOCK_SERVER.mock(|w, then| {
        when(w, POST, "/v1/formations/Stubb")
            .header("content-type", "application/json")
            .query_param("active", "false")
            .query_param("source", "Ishmael");
        then.status(201)
            .header("content-type", "application/json")
            .json_body(resp_json.clone());
    });

    let req = build_req();
    let resp = req.clone_from("Ishmael", false).unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value::<Vec<_>>(resp_json).unwrap());
}

// POST /formations/NAME?active=true&source=OTHER (empty body)
#[test]
fn clone_from_active() {
    let resp_json = json!(["557f87c3-b26c-428c-b970-cb8acac2bd68"]);

    let mock = MOCK_SERVER.mock(|w, then| {
        when(w, POST, "/v1/formations/Stubb")
            .header("content-type", "application/json")
            .query_param("active", "true")
            .query_param("source", "Ishmael");
        then.status(201)
            .header("content-type", "application/json")
            .json_body(resp_json.clone());
    });

    let req = build_req();
    let resp = req.clone_from("Ishmael", true).unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value::<Vec<_>>(resp_json).unwrap());
}

fn build_configuration() -> FormationConfiguration {
    FormationConfiguration::builder()
        .add_flight(Flight::new(
            "Pequod",
            "registry.seaplanet.io/Stubb/alpine:latest",
        ))
        .add_flight(Flight::new(
            "Flask",
            "registry.seaplanet.io/Stubb/alpine:latest",
        ))
        .build()
        .unwrap()
}

// This macro just keeps us from having to duplicate the entire test by hand just to change a
// single query param from true=>false
macro_rules! test_create {
    ($fn:ident, $param:expr) => {
        #[test]
        fn $fn() {
            let resp_json = json!(["557f87c3-b26c-428c-b970-cb8acac2bd68"]);

            let mock = MOCK_SERVER.mock(|w, then| {
                when(w, POST, "/v1/formations/Stubb")
                    .header("content-type", "application/json")
                    .query_param("active", stringify!($param))
                    .json_body_obj(&build_configuration());
                then.status(201)
                    .header("content-type", "application/json")
                    .json_body(resp_json.clone());
            });

            let req = build_req();
            let resp = req.create(build_configuration(), $param).unwrap();

            // Ensure the endpoint was hit
            mock.assert();

            assert_eq!(resp, serde_json::from_value::<Vec<_>>(resp_json).unwrap());
        }
    };
}
// POST /formations/NAME?active=false
test_create!(create, false);
// POST /formations/NAME?active=true
test_create!(create_active, true);

macro_rules! test_add_configuration {
    ($fn:ident, $param:expr) => {
        #[test]
        fn $fn() {
            let resp_json = json!("557f87c3-b26c-428c-b970-cb8acac2bd68");

            let mock = MOCK_SERVER.mock(|w, then| {
                when(w, POST, "/v1/formations/Stubb/configurations")
                    .header("content-type", "application/json")
                    .query_param("active", stringify!($param))
                    .json_body_obj(&build_configuration());
                then.status(201)
                    .header("content-type", "application/json")
                    .json_body(resp_json.clone());
            });

            let req = build_req();
            let resp = req
                .add_configuration(build_configuration(), $param)
                .unwrap();

            // Ensure the endpoint was hit
            mock.assert();

            assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
        }
    };
}
// POST /formations/NAME/configurations?active=false
test_add_configuration!(add_configuration, false);
// POST /formations/NAME/configurations?active=true
test_add_configuration!(add_configuration_active, true);

macro_rules! test_remove_configuration {
    ($fn:ident, $param:expr) => {
        #[test]
        fn $fn() {
            let resp_json = json!("557f87c3-b26c-428c-b970-cb8acac2bd68");

            let mock = MOCK_SERVER.mock(|w, t| {
                when(
                    w,
                    DELETE,
                    "/v1/formations/Stubb/configurations/557f87c3-b26c-428c-b970-cb8acac2bd68",
                )
                .header("content-type", "application/json")
                .query_param("force", stringify!($param));
                then(t, resp_json.clone());
            });

            let req = build_req();
            let resp = req
                .remove_configuration(
                    "557f87c3-b26c-428c-b970-cb8acac2bd68".parse().unwrap(),
                    $param,
                )
                .unwrap();

            // Ensure the endpoint was hit
            mock.assert();

            assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
        }
    };
}
// DELETE /formations/NAME/configurations/UUID?force=false
test_remove_configuration!(remove_configuration, false);
// DELETE /formations/NAME/configurations/UUID?force=true
test_remove_configuration!(remove_configuration_force, true);

// GET /formations/NAME/configurations/UUID
#[test]
fn get_configuration() {
    let mock = MOCK_SERVER.mock(|w, then| {
        when(
            w,
            GET,
            "/v1/formations/Stubb/configurations/557f87c3-b26c-428c-b970-cb8acac2bd68",
        )
        .header("content-type", "application/json");
        then.status(200).json_body_obj(&build_configuration());
    });

    let req = build_req();
    let resp = req
        .get_configuration("557f87c3-b26c-428c-b970-cb8acac2bd68".parse().unwrap())
        .unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, build_configuration());
}

// GET /formations/NAME/configurations
#[test]
fn list_configuration_ids() {
    let resp_json = json!([
        "557f87c3-b26c-428c-b970-cb8acac2bd68",
        "aa3b6eaf-dd1b-4055-93b7-21d024d2acc9"
    ]);

    let mock = MOCK_SERVER.mock(|w, then| {
        when(w, GET, "/v1/formations/Stubb/configurations")
            .header("content-type", "application/json");
        then.status(200).json_body(resp_json.clone());
    });

    let req = build_req();
    let resp = req.list_configuration_ids().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value::<Vec<_>>(resp_json).unwrap());
}

macro_rules! test_delete {
    ($fn:ident, $param:expr) => {
        #[test]
        fn $fn() {
            let resp_json = json!(["557f87c3-b26c-428c-b970-cb8acac2bd68"]);

            let mock = MOCK_SERVER.mock(|w, t| {
                when(w, DELETE, "/v1/formations/Stubb")
                    .header("content-type", "application/json")
                    .query_param("force", stringify!($param));
                then(t, resp_json.clone());
            });

            let req = build_req();
            let resp = req.delete($param).unwrap();

            // Ensure the endpoint was hit
            mock.assert();

            assert_eq!(resp, serde_json::from_value::<Vec<_>>(resp_json).unwrap());
        }
    };
}
// DELETE /formations/NAME?force=false
test_delete!(delete, false);
// DELETE /formations/NAME?force=true
test_delete!(delete_force, true);

// GET /formations/NAME/containers
#[test]
fn get_containers() {
    let resp_json = json!(
        [{
            "uuid" : "557f87c3-b26c-428c-b970-cb8acac2bd68",
            "status" : "started"
        },
        {
            "uuid" : "91f191f5-be32-4d44-860f-0eccca325e0f",
            "status" : "running"
        }]
    );

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/formations/Stubb/containers");
        then(t, resp_json.clone());
    });

    let req = build_req();
    let resp = req.get_containers().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}

// GET /formations/NAME/containers/UUID
#[test]
fn get_container() {
    let resp_json = json!(
        {
            "uuid" : "91f191f5-be32-4d44-860f-0eccca325e0f",
            "status" : "running"
        }
    );

    let mock = MOCK_SERVER.mock(|w, t| {
        when(
            w,
            GET,
            "/v1/formations/Stubb/containers/91f191f5-be32-4d44-860f-0eccca325e0f",
        );
        then(t, resp_json.clone());
    });

    let req = build_req();
    let resp = req
        .get_container("91f191f5-be32-4d44-860f-0eccca325e0f".parse().unwrap())
        .unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}

// GET /formations/NAME/activeConfiguration
#[test]
fn get_active_configurations() {
    let resp_json = json!([
        {
            "configuration_id" : "91f191f5-be32-4d44-860f-0eccca325e0f",
            "traffic_weight" : 9
        },
        {
            "configuration_id" : "876034e4-b5d2-4860-9522-60478fca47f6",
            "traffic_weight" : 2
        }
    ]);

    let mock = MOCK_SERVER.mock(|w, t| {
        when(w, GET, "/v1/formations/Stubb/activeConfiguration");
        then(t, resp_json.clone());
    });

    let req = build_req();
    let resp = req.get_active_configurations().unwrap();

    // Ensure the endpoint was hit
    mock.assert();

    assert_eq!(resp, serde_json::from_value(resp_json).unwrap());
}

fn build_active_connections() -> ActiveConfigurations {
    ActiveConfigurations::new()
        .add_configuration(
            ActiveConfiguration::builder()
                .uuid(
                    "91f191f5-be32-4d44-860f-0eccca325e0f"
                        .parse::<Uuid>()
                        .unwrap(),
                )
                .traffic_weight(9)
                .build()
                .unwrap(),
        )
        .add_configuration(
            ActiveConfiguration::builder()
                .uuid(
                    "876034e4-b5d2-4860-9522-60478fca47f6"
                        .parse::<Uuid>()
                        .unwrap(),
                )
                .traffic_weight(2)
                .build()
                .unwrap(),
        )
}

macro_rules! test_set_active_configurations {
    ($fn:ident, $param:expr) => {
        #[test]
        fn $fn() {
            let mock = MOCK_SERVER.mock(|w, then| {
                when(w, PUT, "/v1/formations/Stubb/activeConfiguration")
                    .query_param("force", stringify!($param));
                then.status(200).body("success");
            });

            let req = build_req();
            let resp = req.set_active_configurations(build_active_connections(), $param);

            // Ensure the endpoint was hit
            mock.assert();

            assert!(resp.is_ok(),);
        }
    };
}
// PUT /formations/NAME/activeConfiguration?force=false
test_set_active_configurations!(set_active_configurations, false);
// PUT /formations/NAME/activeConfiguration?force=true
test_set_active_configurations!(set_active_configurations_force, true);

// DELETE /formations/NAME/activeConfiguration
#[test]
fn stop() {
    let mock = MOCK_SERVER.mock(|w, then| {
        when(w, DELETE, "/v1/formations/Stubb/activeConfiguration");
        then.status(200).body("success");
    });

    let req = build_req();
    let resp = req.stop();

    // Ensure the endpoint was hit
    mock.assert();

    assert!(resp.is_ok());
}
