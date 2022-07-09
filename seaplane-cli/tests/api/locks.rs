use httpmock::{prelude::*, Method};
use seaplane_cli::printer::printer;
use serde_json::json;

use super::{test_main, then, when_json, MOCK_SERVER};

#[test]
fn locks_acquire() {
    let resp_json = json!({
        "id": "D4lbVpdBE_U",
        "sequencer": 3,
    });

    let mut mock = MOCK_SERVER.mock(|w, t| {
        when_json(w, PUT, "/v1/locks/base64:Zm9v");
        then(t, &resp_json);
    });

    let res = test_main(
        &cli!("locks acquire foo --client-id bar --ttl 30"),
        MOCK_SERVER.base_url(),
    );
    assert!(res.is_ok());
    mock.assert_hits(1);
    assert_eq!(printer().as_string().trim(), "LOCK-NAME: Zm9v");
    printer().clear();

    let res = test_main(
        &cli!("locks acquire Zm9v --client-id bar --ttl 60 --base64"),
        MOCK_SERVER.base_url(),
    );

    assert!(res.is_ok());
    mock.assert_hits(2);
    assert_eq!(printer().as_string().trim(), "LOCK-NAME: Zm9v");
    printer().clear();

    mock.delete();
}

#[test]
fn locks_list() {
    let resp = json!({
        "name": "foo",
        "id": "D4lbVpdBE_U",
        "info": {
            "ttl": 5,
            "client-id": "test-client",
            "ip": "192.0.2.137"
        },
    });

    let mut mock = MOCK_SERVER.mock(|w, t| {
        when_json(w, GET, "/v1/locks/base64:Zm9v");
        then(t, &resp);
    });

    let res = test_main(&cli!("locks list Zm9v --base64"), MOCK_SERVER.base_url());
    assert!(res.is_ok());
    mock.assert_hits(1);

    assert_eq!(printer().as_string().trim(), "LOCK-NAME: Zm9v");
    printer().clear();

    let res = test_main(&cli!("locks list foo --decode"), MOCK_SERVER.base_url());
    assert!(res.is_ok());
    mock.assert_hits(2);
    assert_eq!(printer().as_string().trim(), "LOCK-NAME: foo");
    printer().clear();

    let res = test_main(
        &cli!("locks list foo -D --display-encoding hex"),
        MOCK_SERVER.base_url(),
    );
    assert!(res.is_ok());
    mock.assert_hits(3);
    assert_eq!(printer().as_string().trim(), "LOCK-NAME: 666f6f");
    printer().clear();

    mock.delete();
}

#[test]
fn locks_renew() {
    let resp_json = json!({"status": 200u32, "title": "Ok"});

    let mock = MOCK_SERVER.mock(|w, t| {
        when_json(w, Method::PATCH, "/v1/locks/base64:Zm9v");
        then(t, &resp_json);
    });

    let res = test_main(
        &cli!("locks renew foo --lock-id bar --ttl 20"),
        MOCK_SERVER.base_url(),
    );

    assert!(res.is_ok());
    mock.assert_hits(1);
    assert_eq!(
        printer().as_string().trim(),
        "Successfully renewed the lock"
    );
    printer().clear();

    let res = test_main(
        &cli!("locks renew Zm9v --lock-id YmFy --ttl 20 --base64"),
        MOCK_SERVER.base_url(),
    );

    assert!(res.is_ok());
    mock.assert_hits(2);
    assert_eq!(
        printer().as_string().trim(),
        "Successfully renewed the lock"
    );
    printer().clear();
}

#[test]
fn locks_release() {
    let resp_json = json!({"status": 200u32, "title": "Ok"});

    let mock = MOCK_SERVER.mock(|w, t| {
        when_json(w, DELETE, "/v1/locks/base64:Zm9v");
        then(t, &resp_json);
    });

    let res = test_main(
        &cli!("locks release foo --lock-id bar"),
        MOCK_SERVER.base_url(),
    );

    assert!(res.is_ok());
    mock.assert_hits(1);
    assert_eq!(
        printer().as_string().trim(),
        "Successfully released the lock"
    );
    printer().clear();
}
