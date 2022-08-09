use httpmock::prelude::*;
use seaplane_cli::printer::printer;
use serde_json::json;

use super::{test_main, then, when_json, MOCK_SERVER};

#[test]
fn restrict_get() {
    let resp = json!({
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

    static ENCODED: &str = "\
API     DIRECTORY   STATE     REGIONS ALLOWED  REGIONS DENIED  PROVIDERS ALLOWED  PROVIDERS DENIED
Config  Zm9vL2Jhcg  Enforced  XE";

    static DECODED: &str = "\
API     DIRECTORY  STATE     REGIONS ALLOWED  REGIONS DENIED  PROVIDERS ALLOWED  PROVIDERS DENIED
Config  foo/bar    Enforced  XE";

    let mut mock = MOCK_SERVER.mock(|w, t| {
        when_json(w, GET, "/v1/restrict/config/base64:Zm9vL2Jhcg/");
        then(t, &resp);
    });

    let res = test_main(
        &cli!("restrict get config Zm9vL2Jhcg --base64"),
        MOCK_SERVER.base_url(),
    );
    assert!(res.is_ok());
    mock.assert_hits(1);
    assert_eq!(printer().as_string().trim(), ENCODED);

    printer().clear();

    let res = test_main(&cli!("restrict get config foo/bar"), MOCK_SERVER.base_url());
    assert!(res.is_ok());
    mock.assert_hits(2);
    assert_eq!(printer().as_string().trim(), ENCODED);
    printer().clear();

    let res = test_main(
        &cli!("restrict get config foo/bar --decode"),
        MOCK_SERVER.base_url(),
    );
    assert!(res.is_ok());
    mock.assert_hits(3);
    assert_eq!(printer().as_string().trim(), DECODED);
    printer().clear();

    mock.delete();
}
