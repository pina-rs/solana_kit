//! Signature execution failures retain typed errors and exact future JSON.
use solana_kit_client::{
    methods::{
        Method,
        submission::{InstructionFailure, KnownTransactionFailure, TransactionFailure},
    },
    subscriptions::*,
};
use solana_kit_core::Signature;

const DEADLINE: u64 = 100;

fn request() -> SignatureNotifications {
    SignatureNotifications(Signature::from_bytes([0; 64]), None)
}

fn acknowledged() -> (
    Connection,
    Subscription<SignatureNotifications>,
    Subscription<Root>,
) {
    let mut owner = Connection::new(1, Budget::default());
    let signature = owner.subscribe(request(), DEADLINE).unwrap();
    owner.outbound().unwrap();
    let root = owner.subscribe(Root, DEADLINE).unwrap();
    owner.outbound().unwrap();
    owner
        .ingest(1, br#"{"jsonrpc":"2.0","id":1,"result":7}"#)
        .unwrap();
    owner
        .ingest(1, br#"{"jsonrpc":"2.0","id":2,"result":8}"#)
        .unwrap();

    (owner, signature, root)
}

fn notify(owner: &mut Connection, result: &str) {
    let frame = format!(
        r#"{{"jsonrpc":"2.0","method":"signatureNotification","params":{{"subscription":7,"result":{result}}}}}"#
    );
    owner.ingest(1, frame.as_bytes()).unwrap();
}

fn assert_failure(value: SignatureInfo, expected: &Option<KnownTransactionFailure>, raw: &str) {
    match (value, expected) {
        (
            SignatureInfo::Processed {
                err: Some(TransactionFailure::Known(actual)),
            },
            Some(expected),
        ) => assert_eq!(&actual, expected),
        (
            SignatureInfo::Processed {
                err: Some(TransactionFailure::Unknown(actual)),
            },
            None,
        ) => assert_eq!(actual.get(), raw),
        other => panic!("unexpected signature execution result: {other:?}"),
    }
}

#[test]
fn failed_signature_completes_once_without_closing_root_or_losing_error_precision() {
    for (error, expected) in [
        (
            r#""AccountInUse""#,
            Some(KnownTransactionFailure::AccountInUse),
        ),
        (
            r#"{"InstructionError":[255,{"Custom":4294967295}]}"#,
            Some(KnownTransactionFailure::InstructionError(
                u8::MAX,
                InstructionFailure::Custom(u32::MAX),
            )),
        ),
        (
            r#"{"FutureError":{"x":18446744073709551616,"decimal":1.2300}}"#,
            None,
        ),
    ] {
        let raw =
            format!(r#"{{"context":{{"slot":18446744073709551615}},"value":{{"err":{error}}}}}"#);
        let direct = request().decode(&raw).unwrap();
        let (mut owner, signature, root) = acknowledged();
        notify(&mut owner, &raw);
        let Drain::Notification(delivered) = owner.next(&signature).unwrap() else {
            panic!("missing processed signature");
        };

        for result in [direct, delivered] {
            assert_eq!(result.context.slot, u64::MAX);
            assert_failure(result.value, &expected, error);
        }

        assert_eq!(owner.status(&signature).unwrap(), Status::Closed);
        assert!(matches!(owner.next(&signature).unwrap(), Drain::Closed));
        owner.forget(signature).unwrap();
        assert!(owner.outbound().is_none());
        assert_eq!(owner.status(&root).unwrap(), Status::Active);
        owner.ingest(1, br#"{"jsonrpc":"2.0","method":"rootNotification","params":{"subscription":8,"result":18446744073709551615}}"#).unwrap();
        assert!(matches!(
            owner.next(&root).unwrap(),
            Drain::Notification(u64::MAX)
        ));
    }
}

#[test]
fn malformed_known_signature_errors_remain_terminal_validation_failures() {
    for value in [
        r#"{"err":{"InstructionError":[256,{"Custom":1}]}}"#,
        r#"{"err":{"InstructionError":[0,{"Custom":4294967296}]}}"#,
        r#"{"err":{"AccountInUse":1}}"#,
        r#"{"err":false}"#,
        r#"{}"#,
        r#""unknownReceipt""#,
        r#"null"#,
    ] {
        let raw = format!(r#"{{"context":{{"slot":1}},"value":{value}}}"#);
        assert!(request().decode(&raw).is_err(), "{value}");
        let (mut owner, signature, root) = acknowledged();
        notify(&mut owner, &raw);
        assert!(matches!(owner.next(&signature), Err(Failure::Protocol(_))));
        assert!(matches!(owner.status(&root), Err(Failure::Closed)));
    }
}
