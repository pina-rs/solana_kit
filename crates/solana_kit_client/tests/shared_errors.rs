use solana_kit_client::{
    RemoteError,
    diagnostics::{json_rpc_error, transaction_error},
    methods::submission::TransactionFailure,
};
use solana_kit_core::errors::{ContextValue, ErrorCode};
use std::error::Error;

#[test]
fn strict_rpc_boundaries_preserve_unknown_integers_and_original_payloads() {
    for value in [i64::MIN, -32002, 8078999, i64::MAX] {
        let json = format!(r#"{{"code":{value},"message":"opaque","data":{{"future":true}}}}"#);
        let diagnostic = json_rpc_error(&json).unwrap();
        assert_eq!(diagnostic.code().value(), value);
        let remote = diagnostic
            .source()
            .unwrap()
            .downcast_ref::<RemoteError>()
            .unwrap();
        assert_eq!(remote.code, value);
        assert_eq!(remote.message, "opaque");
        assert!(!format!("{diagnostic:?}").contains("opaque"));
    }

    for json in [
        r#"{"code":1.5,"message":"bad"}"#,
        r#"{"code":9223372036854775808,"message":"bad"}"#,
        r#"{"code":true,"message":"bad"}"#,
        r#"{"code":1}"#,
        r#"[]"#,
    ] {
        assert!(json_rpc_error(json).is_err(), "{json}");
    }
}

#[test]
fn actual_execution_parser_maps_custom_indices_and_rejects_invalid_payloads() {
    let diagnostic =
        transaction_error(r#"{"InstructionError":[7,{"Custom":4294967295}]}"#).unwrap();
    assert_eq!(diagnostic.code(), ErrorCode::INSTRUCTION_ERROR_CUSTOM);
    assert_eq!(
        diagnostic.context().get("index"),
        Some(&ContextValue::Unsigned(7))
    );
    assert_eq!(
        diagnostic.context().get("code"),
        Some(&ContextValue::Unsigned(u32::MAX.into()))
    );
    assert!(
        diagnostic
            .source()
            .unwrap()
            .downcast_ref::<TransactionFailure>()
            .is_some()
    );
    let unknown = transaction_error(r#"{"FutureFailure":{"opaque":"retained"}}"#).unwrap();
    assert_eq!(unknown.code(), ErrorCode::TRANSACTION_ERROR_UNKNOWN);

    match unknown
        .source()
        .unwrap()
        .downcast_ref::<TransactionFailure>()
        .unwrap()
    {
        TransactionFailure::Unknown(raw) => {
            assert_eq!(raw.get(), r#"{"FutureFailure":{"opaque":"retained"}}"#)
        }
        _ => panic!("future variant lost"),
    }
    for json in [
        r#"{"InstructionError":[256,"GenericError"]}"#,
        r#"{"InstructionError":[0,{"Custom":4294967296}]}"#,
        r#"{"DuplicateInstruction":-1}"#,
        r#"{"AccountInUse":1,"AccountNotFound":2}"#,
        r#"{}"#,
    ] {
        assert!(transaction_error(json).is_err(), "{json}");
    }
}

#[test]
fn source_context_encoding_handles_nested_values_and_strict_corruption() {
    use solana_kit_client::diagnostics::{decode_context, encode_context};
    use solana_kit_core::errors::{Context, ContextError};
    let mut context = Context::default();
    context.insert("count", ContextValue::Integer(42)).unwrap();
    context.insert("key", ContextValue::Null).unwrap();
    context
        .insert(
            "items",
            ContextValue::List(vec![
                ContextValue::Text("a".into()),
                ContextValue::Text("b".into()),
            ]),
        )
        .unwrap();
    context
        .insert(
            "nested",
            ContextValue::Object(std::collections::BTreeMap::from([(
                "x".into(),
                ContextValue::Integer(1),
            )])),
        )
        .unwrap();
    let encoded = encode_context(&context).unwrap();
    let decoded = decode_context(&encoded).unwrap();
    assert_eq!(decoded.get("count"), Some(&ContextValue::Integer(42)));

    assert_eq!(decoded.get("key"), Some(&ContextValue::Null));
    assert_eq!(
        decoded.get("items"),
        Some(&ContextValue::Text("a,b".into()))
    );
    assert_eq!(decoded.get("nested"), context.get("nested"));
    assert_eq!(decode_context("%%%"), Err(ContextError::Encoding));
    assert_eq!(decode_context("az0lRkc="), Err(ContextError::Encoding));
    assert_eq!(encode_context(&Context::default()).unwrap(), "");
    assert_eq!(decode_context("").unwrap(), Context::default());
}

#[test]
fn independent_dart_execution_oracles_cover_every_pinned_variant() {
    let fixture = include_str!("shared_errors/execution.tsv");

    for row in fixture.lines() {
        let fields: Vec<_> = row.split('\t').collect();
        let diagnostic = transaction_error(fields[0]).unwrap();
        assert_eq!(
            diagnostic.code().value(),
            fields[1].parse::<i64>().unwrap(),
            "{}",
            fields[0]
        );
        let expected: std::collections::BTreeMap<String, String> =
            serde_json::from_str(fields[2]).unwrap();
        let actual: std::collections::BTreeMap<_, _> = diagnostic
            .context()
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_string()))
            .collect();
        assert_eq!(actual, expected, "{}", fields[0]);
    }

    assert_eq!(fixture.lines().count(), 93);
}

#[test]
fn preflight_projection_defaults_typed_cause_and_server_message() {
    let diagnostic = json_rpc_error(r#"{"code":-32002,"message":"simulation","data":{"err":{"InstructionError":[2,{"Custom":99}]},"fee":5000,"preTokenBalances":["public"],"logs":[]}}"#).unwrap();
    assert_eq!(diagnostic.context().iter().count(), 13);
    assert_eq!(
        diagnostic.context().get("fee"),
        Some(&ContextValue::Integer(5000))
    );
    assert_eq!(
        diagnostic.context().get("returnData"),
        Some(&ContextValue::Null)
    );
    assert_eq!(diagnostic.context().get("err"), None);
    let cause = diagnostic
        .simulation_cause()
        .downcast_ref::<solana_kit_core::errors::Diagnostic>()
        .unwrap();
    assert_eq!(cause.code(), ErrorCode::INSTRUCTION_ERROR_CUSTOM);
    assert_eq!(
        cause.context().get("code"),
        Some(&ContextValue::Unsigned(99))
    );

    assert!(
        cause
            .source()
            .unwrap()
            .downcast_ref::<TransactionFailure>()
            .is_some()
    );
    let empty = json_rpc_error(r#"{"code":-32002,"message":"simulation"}"#).unwrap();
    assert_eq!(empty.context().iter().count(), 13);
    let server =
        json_rpc_error(r#"{"code":-32603,"message":"server detail","data":{"discarded":1}}"#)
            .unwrap();
    assert_eq!(
        server.context().get("__serverMessage"),
        Some(&ContextValue::Text("server detail".into()))
    );
    assert_eq!(server.context().get("discarded"), None);
}

#[test]
fn generic_enum_callback_preserves_payload_and_checks_offsets() {
    use solana_kit_client::diagnostics::{RpcEnumError, RpcEnumErrorConfig, rpc_enum_error};
    use solana_kit_core::errors::Context;
    let mut calls = 0;
    let diagnostic = rpc_enum_error(
        RpcEnumErrorConfig {
            base: ErrorCode::INSTRUCTION_ERROR_GENERIC_ERROR,
            names: &["GenericError"],
        },
        r#"{"Future":9223372036854775807}"#,
        |code, name, payload| {
            calls += 1;
            assert_eq!(code, ErrorCode::INSTRUCTION_ERROR_UNKNOWN);
            assert_eq!(name, "Future");
            assert_eq!(payload, Some(ContextValue::Integer(i64::MAX.into())));
            let mut context = Context::default();
            context.insert("value", payload.unwrap())?;
            Ok(context)
        },
    )
    .unwrap();
    assert_eq!(calls, 1);
    assert_eq!(
        diagnostic
            .source()
            .unwrap()
            .downcast_ref::<RpcEnumError>()
            .unwrap()
            .raw(),
        r#"{"Future":9223372036854775807}"#
    );
}

#[test]
fn independent_dart_context_wire_literals() {
    use solana_kit_client::diagnostics::encode_context;
    use solana_kit_core::errors::Context;
    use std::collections::BTreeMap;
    let mut cases = vec![Context::default(); 5];
    cases[1].insert("count", ContextValue::Integer(42)).unwrap();
    cases[1].insert("key", ContextValue::Null).unwrap();
    cases[2]
        .insert(
            "items",
            ContextValue::List(vec![
                ContextValue::Text("a".into()),
                ContextValue::Text("b".into()),
            ]),
        )
        .unwrap();
    cases[3]
        .insert(
            "nested",
            ContextValue::Object(BTreeMap::from([("x".into(), ContextValue::Integer(1))])),
        )
        .unwrap();
    cases[4]
        .insert(
            "items",
            ContextValue::List(vec![
                ContextValue::Text("a b".into()),
                ContextValue::Text("%".into()),
            ]),
        )
        .unwrap();
    let literals: Vec<_> = include_str!("shared_errors/context.txt").lines().collect();
    assert_eq!(literals.len(), cases.len());

    for (context, expected) in cases.iter().zip(literals) {
        assert_eq!(encode_context(context).unwrap(), expected);
    }
}

#[test]
fn direct_instruction_helper_handles_future_scalar_and_retains_known_context() {
    use solana_kit_client::diagnostics::{RpcEnumError, instruction_error};
    let future = instruction_error(0, "42").unwrap();
    assert_eq!(future.code(), ErrorCode::INSTRUCTION_ERROR_UNKNOWN);
    assert_eq!(
        future.context().get("errorName"),
        Some(&ContextValue::Text("42".into()))
    );
    let known = instruction_error(3, r#"{"BorshIoError":"opaque"}"#).unwrap();
    assert_eq!(known.code(), ErrorCode::INSTRUCTION_ERROR_BORSH_IO_ERROR);
    assert_eq!(
        known
            .source()
            .unwrap()
            .downcast_ref::<RpcEnumError>()
            .unwrap()
            .raw(),
        r#"{"BorshIoError":"opaque"}"#
    );
    assert!(instruction_error(0, r#"{"Custom":4294967296}"#).is_err());
}

#[test]
fn absent_simulation_cause_and_null_future_context_follow_source() {
    let empty =
        json_rpc_error(r#"{"code":-32002,"message":"failed","data":{"err":null}}"#).unwrap();
    let unwrapped = empty
        .simulation_cause()
        .downcast_ref::<solana_kit_core::errors::Diagnostic>()
        .unwrap();
    assert!(std::ptr::eq(unwrapped, &empty));
    let unknown = transaction_error(r#"{"InstructionError":[0,{"Future":null}]}"#).unwrap();
    assert_eq!(unknown.context().get("instructionErrorContext"), None);
    let newer = transaction_error(r#""CommitCancelled""#).unwrap();
    assert_eq!(
        newer.context().get("errorName"),
        Some(&ContextValue::Text("CommitCancelled".into()))
    );
}

#[test]
fn retained_payloads_are_redacted_from_native_debug() {
    let remote: RemoteError = serde_json::from_str(
        r#"{"code":-32002,"message":"credential","data":{"secret":"credential"}}"#,
    )
    .unwrap();
    assert!(!format!("{remote:?}").contains("credential"));
    for input in [
        r#"{"InstructionError":[0,{"BorshIoError":"credential"}]}"#,
        r#"{"Future":"credential"}"#,
    ] {
        let failure: TransactionFailure = serde_json::from_str(input).unwrap();
        assert!(!format!("{failure:?}").contains("credential"));
    }
}
