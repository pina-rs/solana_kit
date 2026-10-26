//! Sender submission uses the actual shared wire encoder and service owner.

use super::service::exchange;
use solana_kit_helius::{
    sender::{Route, Submission},
    service::{Error, Operation, Service, Verb},
};

/// One submission frame preserves caller bytes and fixes node retries at zero.
pub fn submission_frames() {
    let submission = Submission {
        wire: &[0, 1],
        id: 9,
        skip_preflight: false,
        route: Route::Fast { swqos_only: true },
    };
    let request = exchange(
        submission,
        br#"{"jsonrpc":"2.0","id":9,"result":"signature"}"#,
    );
    assert_eq!(request.service, Service::Sender);
    assert_eq!(request.method, Verb::Post);
    assert_eq!(request.path_segments, ["fast"]);
    assert_eq!(
        request.query,
        [("swqos_only".to_owned(), "true".to_owned())]
    );
    let body: serde_json::Value =
        serde_json::from_slice(request.body.as_deref().expect("submission body")).expect("JSON");
    assert_eq!(
        body,
        serde_json::json!({"jsonrpc":"2.0","id":9,"method":"sendTransaction",
        "params":["AAE=",{"encoding":"base64","skipPreflight":false,"maxRetries":0}]})
    );
    let submission = Submission {
        wire: &[0, 1],
        id: 1,
        skip_preflight: true,
        route: Route::Root,
    };
    let request = exchange(submission, br#""signature""#);
    assert!(request.path_segments.is_empty());

    assert!(request.query.is_empty());
}

/// Malformed known fields and protocol errors cannot become a successful signature.
pub fn response_failures() {
    let submission = Submission {
        wire: &[0],
        id: 1,
        skip_preflight: true,
        route: Route::Root,
    };

    for invalid in [
        br#"{"id":2,"result":"s"}"#.as_slice(),
        br#"{"jsonrpc":"1.0","result":"s"}"#,
        br#"{"result":null}"#,
        br#"{"result":42}"#,
        br#"[]"#,
        br#"{"id":null,"result":"s"}"#,
    ] {
        assert!(submission.decode(invalid).is_err());
    }

    let error = submission
        .decode(br#"{"error":{"code":-9,"message":"secret-error"},"result":"must-not-succeed"}"#)
        .expect_err("remote error");
    assert!(!format!("{error:?}").contains("secret-error"));

    match error {
        Error::Provider(raw) => assert_eq!(raw.get(), r#"{"code":-9,"message":"secret-error"}"#),
        other => panic!("wrong error {other:?}"),
    }
}
