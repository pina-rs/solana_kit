use solana_kit_client::diagnostics::{
    RpcEnumError, RpcEnumErrorConfig, decode_context, encode_context, instruction_error,
    json_rpc_error, rpc_enum_error, transaction_error,
};
use solana_kit_core::errors::{Context, ContextError, ContextValue, Diagnostic, ErrorCode};
use std::error::Error as _;

const MESSAGE_BYTES: usize = 4096;
const LAST_TEXT: usize = 986;

pub fn exact_message_render_boundary() {
    let mut context = Context::default();
    let mut items = vec![ContextValue::Text("x".repeat(1024)); 3];
    items.push(ContextValue::Text("x".repeat(LAST_TEXT)));
    context
        .insert("address", ContextValue::List(items.clone()))
        .unwrap();
    let message = ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND
        .message(&context)
        .unwrap();
    assert_eq!(message.len(), MESSAGE_BYTES);
    items.pop();
    items.push(ContextValue::Text("x".repeat(LAST_TEXT + 1)));
    context
        .insert("address", ContextValue::List(items))
        .unwrap();
    assert_eq!(
        ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND.message(&context),
        Err(ContextError::Limit)
    );
}

pub fn utf8_uri_escaping_is_exact() {
    let mut context = Context::default();
    context
        .insert("v", ContextValue::Text("é %&=+".into()))
        .unwrap();
    let encoded = encode_context(&context).unwrap();
    assert_eq!(encoded, "dj0lQzMlQTklMjAlMjUlMjYlM0QlMkI=");
    assert_eq!(decode_context(&encoded).unwrap(), context);
}

pub fn future_payloads_and_native_debug_redaction() {
    let input = r#"{"Future":{"label":"é","values":[4294967295,9223372036854775807,true]}}"#;
    let error = transaction_error(input).unwrap();
    assert_eq!(error.code(), ErrorCode::TRANSACTION_ERROR_UNKNOWN);
    assert!(error.context().get("transactionErrorContext").is_some());
    assert!(!format!("{error:?} {error}").contains("values"));
    let scalar = instruction_error(u8::MAX, "9223372036854775807").unwrap();
    assert_eq!(
        scalar.context().get("errorName"),
        Some(&ContextValue::Text(i64::MAX.to_string()))
    );
    let borsh = instruction_error(0, r#"{"BorshIoError":"opaque utf8 é"}"#).unwrap();
    let source = borsh
        .source()
        .unwrap()
        .downcast_ref::<RpcEnumError>()
        .unwrap();
    assert_eq!(source.raw(), r#"{"BorshIoError":"opaque utf8 é"}"#);
    assert!(!format!("{source:?} {source}").contains("opaque"));
}

pub fn preflight_defaults_preserve_source_identity() {
    let error =
        json_rpc_error(r#"{"code":-32002,"message":"failed","data":{"err":null}}"#).unwrap();
    assert_eq!(error.context().iter().count(), 13);
    assert!(
        error
            .context()
            .iter()
            .all(|(_, value)| *value == ContextValue::Null)
    );
    assert_eq!(error.context().get("err"), None);
    let same = error
        .simulation_cause()
        .downcast_ref::<Diagnostic>()
        .unwrap();
    assert!(std::ptr::eq(same, &error));
    assert!(
        error
            .source()
            .unwrap()
            .downcast_ref::<solana_kit_client::RemoteError>()
            .is_some()
    );
    let error = json_rpc_error(r#"{"code":-32002,"message":"failed","data":{"err":{"InstructionError":[255,{"Custom":4294967295}]}}}"#).unwrap();
    let cause = error
        .simulation_cause()
        .downcast_ref::<Diagnostic>()
        .unwrap();
    assert_eq!(cause.code(), ErrorCode::INSTRUCTION_ERROR_CUSTOM);
    assert_eq!(
        cause.context().get("code"),
        Some(&ContextValue::Unsigned(u32::MAX.into()))
    );
    assert!(
        cause
            .source()
            .unwrap()
            .downcast_ref::<solana_kit_client::methods::submission::TransactionFailure>()
            .is_some()
    );
}

pub fn checked_enum_arithmetic_does_not_invoke_callback() {
    for (base, names, input) in [
        (i64::MIN, &[][..], "\"Unknown\""),
        (i64::MAX, &["Other", "Overflow"][..], "\"Overflow\""),
    ] {
        let result = rpc_enum_error(
            RpcEnumErrorConfig {
                base: ErrorCode::from_value(base),
                names,
            },
            input,
            |_, _, _| panic!("invalid arithmetic must reject before callback"),
        );
        assert!(result.is_err());
    }

    for input in [
        r#"{"InstructionError":[256,"GenericError"]}"#,
        r#"{"InstructionError":[0,{"Custom":4294967296}]}"#,
    ] {
        assert!(transaction_error(input).is_err());
    }
}
