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
        when_json(w, POST, "/v1/locks/base64:Zm9v");
        then(t, &resp_json);
    });

    let res = test_main(
        &cli!("locks acquire foo --client-id bar --ttl 30"),
        MOCK_SERVER.base_url(),
    );
    assert!(res.is_ok());
    mock.assert_hits(1);
    assert_eq!(
        printer().as_string().trim(),
        "LOCK-ID: D4lbVpdBE_U\nSEQUENCER: 3"
    );
    printer().clear();

    let res = test_main(
        &cli!("locks acquire Zm9v --client-id bar --ttl 60 --base64"),
        MOCK_SERVER.base_url(),
    );

    assert!(res.is_ok());
    mock.assert_hits(2);
    assert_eq!(
        printer().as_string().trim(),
        "LOCK-ID: D4lbVpdBE_U\nSEQUENCER: 3"
    );
    printer().clear();

    mock.delete();
}

#[test]
fn locks_list_all() {
    let p1 = json!({
        "next": "page2",
        "infos": [
            {
                "name": "foo",
                "id": "D4lbVpdBE_U",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            }
        ]
    });

    let p2 = json!({
        "next": "page3",
        "infos": []
    });

    let p3 = json!({
        "next": null,
        "infos": [
            {
                "name": "bar",
                "id": "D4lbVpdBD_U",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client2",
                    "ip": "192.0.2.137"
                }
            }
        ]
    });

    let mut mock3 = MOCK_SERVER.mock(|w, t| {
        when_json(w, GET, "/v1/locks/").query_param("from", "base64:page3");
        then(t, &p3);
    });

    let mut mock2 = MOCK_SERVER.mock(|w, t| {
        when_json(w, GET, "/v1/locks/").query_param("from", "base64:page2");
        then(t, &p2);
    });

    // order matters here. If mock1 is defined before the others,
    // it will match every request regardless of params.
    let mut mock1 = MOCK_SERVER.mock(|w, t| {
        when_json(w, GET, "/v1/locks/");
        then(t, &p1);
    });

    let res = test_main(&cli!("locks list"), MOCK_SERVER.base_url());
    assert!(res.is_ok());
    mock1.assert_hits(1);
    mock2.assert_hits(1);
    mock3.assert_hits(1);

    assert_eq!(
        printer().as_string().trim(),
        "LOCK-NAME: foo\n\
         LOCK-ID: D4lbVpdBE_U\n\
         CLIENT-ID: test-client\n\
         CLIENT-IP: 192.0.2.137\n\
         TTL: 5\n\
         LOCK-NAME: bar\n\
         LOCK-ID: D4lbVpdBD_U\n\
         CLIENT-ID: test-client2\n\
         CLIENT-IP: 192.0.2.137\n\
         TTL: 5"
    );

    mock1.delete();
    mock2.delete();
    mock3.delete();
    printer().clear();
}

#[test]
fn locks_list() {
    let resp = json!({
        "name": "Zm9v",
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

    assert_eq!(printer().as_string().trim(),
        "LOCK-NAME: Zm9v\nLOCK-ID: D4lbVpdBE_U\nCLIENT-ID: test-client\nCLIENT-IP: 192.0.2.137\nTTL: 5");
    printer().clear();

    let res = test_main(&cli!("locks list foo --decode"), MOCK_SERVER.base_url());
    assert!(res.is_ok());
    mock.assert_hits(2);
    assert_eq!(printer().as_string().trim(),
        "LOCK-NAME: foo\nLOCK-ID: D4lbVpdBE_U\nCLIENT-ID: test-client\nCLIENT-IP: 192.0.2.137\nTTL: 5");
    printer().clear();

    let res = test_main(
        &cli!("locks list foo -D --display-encoding hex"),
        MOCK_SERVER.base_url(),
    );
    assert!(res.is_ok());
    mock.assert_hits(3);
    assert_eq!(printer().as_string().trim(),
        "LOCK-NAME: 666f6f\nLOCK-ID: D4lbVpdBE_U\nCLIENT-ID: test-client\nCLIENT-IP: 192.0.2.137\nTTL: 5");
    printer().clear();

    let res = test_main(
        &cli!("locks list foo --format json"),
        MOCK_SERVER.base_url(),
    );
    assert!(res.is_ok());
    mock.assert_hits(4);
    assert_eq!(printer().as_string().trim(),
        "{\"name\":\"Zm9v\",\"id\":\"D4lbVpdBE_U\",\"info\":{\"ttl\":5,\"client-id\":\"test-client\",\"ip\":\"192.0.2.137\"}}");
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
        &cli!("locks renew foo --lock-id ATlcuG7mmF4 --ttl 20"),
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
        &cli!("locks renew Zm9v --lock-id ATlcuG7mmF4 --ttl 20 --base64"),
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
