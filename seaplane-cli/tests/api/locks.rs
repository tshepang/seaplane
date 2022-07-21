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
fn locks_list_output_pages() {
    let server_page = json!({
        "next": null,
        "infos": [
            {
                "name": "long name davis, home of the long names",
                "id": "MQo",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "1",
                "id": "Mgo",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "2",
                "id": "Mwo",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "3",
                "id": "NAo",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "4",
                "id": "NQo",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "5",
                "id": "OTEK",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "6",
                "id": "OAo",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "7",
                "id": "MTAK",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "8",
                "id": "MTEK",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "9",
                "id": "MTIK",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "10",
                "id": "MTMK",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "11",
                "id": "MTQK",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
            {
                "name": "12",
                "id": "MTYK",
                "info": {
                    "ttl": 5,
                    "client-id": "test-client",
                    "ip": "192.0.2.137"
                }
            },
        ]
    });

    // The point of the test is to see us page during output
    // so make sure that OUTPUT_PAGE_SIZE < server_page.infos.len

    let mut mock = MOCK_SERVER.mock(|w, t| {
        when_json(w, GET, "/v1/locks/");
        then(t, &server_page);
    });

    let res = test_main(&cli!("locks list"), MOCK_SERVER.base_url());
    assert!(res.is_ok());
    mock.assert_hits(1);

    assert_eq!(
        printer().as_string().trim(),
        "LOCK-NAME                                LOCK-ID  CLIENT-ID    CLIENT-IP    TTL\n\
         long name davis, home of the long names  MQo      test-client  192.0.2.137  5\n\
         1                                        Mgo      test-client  192.0.2.137  5\n\
         2                                        Mwo      test-client  192.0.2.137  5\n\
         3                                        NAo      test-client  192.0.2.137  5\n\
         4                                        NQo      test-client  192.0.2.137  5\n\
         5                                        OTEK     test-client  192.0.2.137  5\n\
         6                                        OAo      test-client  192.0.2.137  5\n\
         7                                        MTAK     test-client  192.0.2.137  5\n\
         8                                        MTEK     test-client  192.0.2.137  5\n\
         9                                        MTIK     test-client  192.0.2.137  5\n\
         10  MTMK  test-client  192.0.2.137  5\n\
         11  MTQK  test-client  192.0.2.137  5\n\
         12  MTYK  test-client  192.0.2.137  5"
    );
    printer().clear();

    mock.delete();
}

#[test]
fn locks_list_server_pages() {
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
        "LOCK-NAME  LOCK-ID      CLIENT-ID     CLIENT-IP    TTL\n\
         foo        D4lbVpdBE_U  test-client   192.0.2.137  5\n\
         bar        D4lbVpdBD_U  test-client2  192.0.2.137  5"
    );
    printer().clear();

    mock1.delete();
    mock2.delete();
    mock3.delete();
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
    assert_eq!(
        printer().as_string().trim(),
        "LOCK-NAME  LOCK-ID      CLIENT-ID    CLIENT-IP    TTL\n\
         Zm9v       D4lbVpdBE_U  test-client  192.0.2.137  5"
    );
    printer().clear();

    let res = test_main(
        &cli!("locks list foo --decode --no-header"),
        MOCK_SERVER.base_url(),
    );
    assert!(res.is_ok());
    mock.assert_hits(2);
    assert_eq!(
        printer().as_string().trim(),
        "foo  D4lbVpdBE_U  test-client  192.0.2.137  5"
    );
    printer().clear();

    let res = test_main(
        &cli!("locks list foo --format json"),
        MOCK_SERVER.base_url(),
    );
    assert!(res.is_ok());
    mock.assert_hits(3);
    assert_eq!(printer().as_string().trim(),
        "{\"name\":\"Zm9v\",\"id\":\"D4lbVpdBE_U\",\"info\":{\"ttl\":5,\"client-id\":\"test-client\",\"ip\":\"192.0.2.137\"}}");
    printer().clear();

    mock.delete();
}

#[test]
fn locks_list_json() {
    let resp = json!({
        "name": "Zm9vCg",
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

    let res = test_main(
        &cli!("locks list foo --format json"),
        MOCK_SERVER.base_url(),
    );
    assert!(res.is_ok());
    mock.assert_hits(1);
    assert_eq!(
        printer().as_string().trim(),
        r#"{"name":"Zm9vCg","id":"D4lbVpdBE_U","info":{"ttl":5,"client-id":"test-client","ip":"192.0.2.137"}}"#
    );
    printer().clear();

    let res = test_main(
        &cli!("locks list foo -D --format json"),
        MOCK_SERVER.base_url(),
    );
    assert!(!res.is_ok());
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
