use httpmock::prelude::*;
use seaplane_cli::printer::printer;
use serde_json::json;

use super::{then, when_json, MOCK_SERVER};

#[test]
fn restrict_get() {
    let resp = json!({
        "api": "Config",
        "directory": "Zm9vL2Jhcg",
        "details": {
            "regions_allowed": ["XE","XN"],
            "regions_denied": [],
            "providers_allowed": [],
            "providers_denied": []
        },
        "state": "Enforced"
    });

    static ENCODED: &str = "\
API     DIRECTORY   STATE     REGIONS ALLOWED  REGIONS DENIED  PROVIDERS ALLOWED  PROVIDERS DENIED
Config  Zm9vL2Jhcg  Enforced  [XE,XN]          []              []                 []";

    static DECODED: &str = "\
API     DIRECTORY  STATE     REGIONS ALLOWED  REGIONS DENIED  PROVIDERS ALLOWED  PROVIDERS DENIED
Config  foo/bar    Enforced  [XE,XN]          []              []                 []";

    let mut mock = MOCK_SERVER.mock(|w, t| {
        when_json(w, GET, "/v1/restrict/config/base64:Zm9vL2Jhcg/");
        then(t, &resp);
    });

    let res = run!("restrict get config Zm9vL2Jhcg --base64");
    assert!(res.is_ok());
    mock.assert_hits(1);
    assert_eq!(printer().as_string().trim(), ENCODED);

    printer().clear();

    let res = run!("restrict get config foo/bar");
    assert!(res.is_ok());
    mock.assert_hits(2);
    assert_eq!(printer().as_string().trim(), ENCODED);
    printer().clear();

    let res = run!("restrict get config foo/bar --decode");
    assert!(res.is_ok());
    mock.assert_hits(3);
    assert_eq!(printer().as_string().trim(), DECODED);
    printer().clear();

    let res = run!("restrict get config foo/bar --format json");
    assert!(res.is_ok());
    mock.assert_hits(4);
    assert_eq!(printer().as_string().trim(), resp.to_string());
    printer().clear();

    mock.delete();
}

#[test]
fn restrict_list() {
    let resp = json!([
        {
            "api": "Config",
            "directory": "Zm9vL2Jhcg",
            "details": {
                "regions_allowed": ["XE","XN"],
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
        },
    ]);
    let api_resp = json!({ "restrictions": resp });

    static ENCODED: &str = "\
API     DIRECTORY   STATE     REGIONS ALLOWED  REGIONS DENIED  PROVIDERS ALLOWED  PROVIDERS DENIED
Config  Zm9vL2Jhcg  Enforced  [XE,XN]          []              []                 []
Config  Zm9vL2Jheg  Enforced  [XN]             []              []                 []";

    static DECODED: &str = "\
API     DIRECTORY  STATE     REGIONS ALLOWED  REGIONS DENIED  PROVIDERS ALLOWED  PROVIDERS DENIED
Config  foo/bar    Enforced  [XE,XN]          []              []                 []
Config  foo/baz    Enforced  [XN]             []              []                 []";

    let mut mock = MOCK_SERVER.mock(|w, t| {
        when_json(w, GET, "/v1/restrict/config/");
        then(t, &api_resp);
    });

    let res = run!("restrict list config");
    println!("{res:?}");
    assert!(res.is_ok());
    mock.assert_hits(1);
    assert_eq!(printer().as_string().trim(), ENCODED);
    printer().clear();

    let res = run!("restrict list config --decode");
    assert!(res.is_ok());
    mock.assert_hits(2);
    assert_eq!(printer().as_string().trim(), DECODED);
    printer().clear();

    let res = run!("restrict list config --format json");
    assert!(res.is_ok());
    mock.assert_hits(3);
    assert_eq!(printer().as_string().trim(), resp.to_string());
    printer().clear();

    mock.delete();

    let mut mock = MOCK_SERVER.mock(|w, t| {
        when_json(w, GET, "/v1/restrict/");
        then(t, &api_resp);
    });
    let res = run!("restrict list");
    assert!(res.is_ok());
    mock.assert_hits(1);
    assert_eq!(printer().as_string().trim(), ENCODED);
    printer().clear();

    mock.delete();
}

#[test]
fn restrict_set() {
    let req_json = json!({
        "regions_allowed": ["XE"],
        "regions_denied": [],
        "providers_allowed": [],
        "providers_denied": []
    });
    let resp_json = json!({"status": 200_i32, "title": "Ok"});

    let mut mock = MOCK_SERVER.mock(|w, t| {
        when_json(w, PUT, "/v1/restrict/config/base64:Zm9vL2Jhcg/")
            .header("content-type", "application/json")
            .json_body_obj(&req_json);
        then(t, &resp_json);
    });

    let res = run!("restrict set config foo/bar --region xe");
    assert!(res.is_ok());
    mock.assert_hits(1);
    assert_eq!(
        printer().as_string().trim(),
        "Set a restriction on directory Zm9vL2Jhcg in config API"
    );
    printer().clear();

    let res = run!("restrict set config Zm9vL2Jhcg --region xe --base64");
    assert!(res.is_ok());
    mock.assert_hits(2);
    assert_eq!(
        printer().as_string().trim(),
        "Set a restriction on directory Zm9vL2Jhcg in config API"
    );
    printer().clear();

    let res = run!("restrict set config foo/bar --region xe --decode");
    assert!(res.is_ok());
    mock.assert_hits(3);
    assert_eq!(
        printer().as_string().trim(),
        "Set a restriction on directory foo/bar in config API"
    );
    printer().clear();

    let res = run!("restrict set config foo/bar --region xe --format json");
    assert!(res.is_ok());
    mock.assert_hits(4);
    assert_eq!(
        printer().as_string().trim(),
        json!({"set_restriction": {"api": "config", "directory": "Zm9vL2Jhcg"}}).to_string()
    );
    printer().clear();

    mock.delete();
}

#[test]
fn restrict_delete() {
    let resp_json = json!({"status": 200u32, "title": "Ok"});

    let mut mock = MOCK_SERVER.mock(|w, t| {
        when_json(w, DELETE, "/v1/restrict/config/base64:Zm9vL2Jhcg/");
        then(t, &resp_json);
    });

    let res = run!("restrict delete config foo/bar");
    assert!(res.is_ok());
    mock.assert_hits(1);
    assert_eq!(
        printer().as_string().trim(),
        "Deleted a restriction on directory Zm9vL2Jhcg in config API"
    );
    printer().clear();

    let res = run!("restrict delete config Zm9vL2Jhcg --base64");
    assert!(res.is_ok());
    mock.assert_hits(2);
    assert_eq!(
        printer().as_string().trim(),
        "Deleted a restriction on directory Zm9vL2Jhcg in config API"
    );
    printer().clear();

    let res = run!("restrict delete config foo/bar --decode");
    assert!(res.is_ok());
    mock.assert_hits(3);
    assert_eq!(
        printer().as_string().trim(),
        "Deleted a restriction on directory foo/bar in config API"
    );
    printer().clear();

    let res = run!("restrict delete config foo/bar --format json");
    assert!(res.is_ok());
    mock.assert_hits(4);
    assert_eq!(
        printer().as_string().trim(),
        json!({"deleted_restriction": {"api": "config", "directory": "Zm9vL2Jhcg"}}).to_string()
    );
    printer().clear();

    mock.delete();
}
