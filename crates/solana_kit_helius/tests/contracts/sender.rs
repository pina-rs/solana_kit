//! Sender submission uses the actual shared wire encoder and service owner.

use super::service::exchange;
use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_client::{Error as RpcError, methods::submission::SignedWire};
use solana_kit_core::Signature;
use solana_kit_helius::{
    sender::{Route, Submission},
    service::{Error, Operation, Service, Verb},
};

/// One submission frame preserves caller bytes and fixes node retries at zero.
pub fn submission_frames() {
    let bytes = wire();
    let signature = Signature::from_bytes([7; 64]);
    let submission = Submission {
        wire: SignedWire::new(&bytes).expect("signed fixture"),
        id: 9,
        skip_preflight: false,
        route: Route::Fast { swqos_only: true },
    };
    let request = exchange(
        submission,
        format!(r#"{{"jsonrpc":"2.0","id":9,"result":"{signature}"}}"#).as_bytes(),
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
        "params":[STANDARD.encode(&bytes),{"encoding":"base64","skipPreflight":false,"maxRetries":0}]})
    );
    let submission = Submission {
        wire: SignedWire::new(&bytes).expect("signed fixture"),
        id: 1,
        skip_preflight: true,
        route: Route::Root,
    };
    let request = exchange(submission, format!(r#""{signature}""#).as_bytes());
    assert!(request.path_segments.is_empty());

    assert!(request.query.is_empty());
}

/// Malformed known fields and protocol errors cannot become a successful signature.
pub fn response_failures() {
    let bytes = wire();
    let submission = Submission {
        wire: SignedWire::new(&bytes).expect("signed fixture"),
        id: 1,
        skip_preflight: true,
        route: Route::Root,
    };
    invalid_fields(&submission);
    identity_fields(&submission);
    acknowledgements(&submission);
    provider_error(&submission);
}

/// Keep every original malformed response control, plus arbitrary success text.
fn invalid_fields(submission: &Submission<'_>) {
    for invalid in [
        br#"{"id":2,"result":"s"}"#.as_slice(),
        br#"{"jsonrpc":"1.0","result":"s"}"#,
        br#"{"result":null}"#,
        br#"{"result":42}"#,
        br#"[]"#,
        br#"{"id":null,"result":"s"}"#,
        br#""arbitrary""#,
        br#"{"jsonrpc":"2.0","id":1,"result":"arbitrary"}"#,
    ] {
        assert!(submission.decode(invalid).is_err());
    }
}

/// Even a matching signature requires the emitted envelope version and ID.
fn identity_fields(submission: &Submission<'_>) {
    let expected = Signature::from_bytes([7; 64]);
    for invalid in [
        format!(r#"{{"result":"{expected}"}}"#),
        format!(r#"{{"jsonrpc":"2.0","result":"{expected}"}}"#),
        format!(r#"{{"id":1,"result":"{expected}"}}"#),
        format!(r#"{{"jsonrpc":"2.0","id":2,"result":"{expected}"}}"#),
        format!(r#"{{"jsonrpc":"1.0","id":1,"result":"{expected}"}}"#),
    ] {
        assert!(submission.decode(invalid.as_bytes()).is_err());
    }
}

/// Both compatibility grammars share canonical parsing and identity binding.
fn acknowledgements(submission: &Submission<'_>) {
    let expected = Signature::from_bytes([7; 64]);
    let foreign = Signature::from_bytes([8; 64]);
    for response in [
        format!(r#""{foreign}""#),
        format!(r#"{{"jsonrpc":"2.0","id":1,"result":"{foreign}"}}"#),
    ] {
        let error = submission
            .decode(response.as_bytes())
            .expect_err("foreign identity");
        let Error::Submission(RpcError::SubmissionSignatureMismatch {
            expected: retained,
            actual: reported,
        }) = error
        else {
            panic!("expected identity mismatch");
        };
        assert_eq!(retained, expected);
        assert_eq!(*reported, foreign);
    }
    for response in [
        format!(r#""{expected}""#),
        format!(r#"{{"jsonrpc":"2.0","id":1,"result":"{expected}"}}"#),
    ] {
        assert_eq!(
            submission
                .decode(response.as_bytes())
                .expect("bound acknowledgement")
                .0,
            expected
        );
    }
}

/// Provider errors retain exact data and remain redacted in diagnostics.
fn provider_error(submission: &Submission<'_>) {
    let error = submission
        .decode(br#"{"jsonrpc":"2.0","id":1,"error":{"code":-9,"message":"secret-error"},"result":"must-not-succeed"}"#)
        .expect_err("remote error");
    assert!(!format!("{error:?}").contains("secret-error"));

    match error {
        Error::Provider(raw) => assert_eq!(raw.get(), r#"{"code":-9,"message":"secret-error"}"#),
        other => panic!("wrong error {other:?}"),
    }
}

/// Independently framed signed legacy transaction, without cryptographic trust.
fn wire() -> Vec<u8> {
    let mut bytes = vec![1];
    bytes.extend([7; 64]);
    bytes.extend([1, 0, 0, 1]);
    bytes.extend([1; 32]);
    bytes.extend([2; 32]);
    bytes.push(0);

    bytes
}
