const BUDGET: usize = 64;
const ABOVE_RANGE: u8 = 9;
const RANGE_MAX: u8 = 8;
const HEX_RADIX: u32 = 16;
const OVER_TEXT_LIMIT: i64 = 1025;
const FIXED_POINT_CODE: i64 = 8090000;
const HISTORICAL_ACCOUNT_TYPO: i64 = 32300001;
const UNDEFINED_CODEC: i64 = 8078998;

const BELOW_JSON_RPC: i64 = 32769;
use solana_kit_core::{
    Address,
    accounts::{AccountError, MaybeAccount},
    codecs::{Bool, Constant, Decoder, Error, Limits, Number, Reader, ensure_number_range},
    errors::{Context, ContextError, ContextValue, Diagnostic, ErrorCode, ErrorDomain, ErrorFacts},
};
use std::{collections::HashSet, error::Error as _};

#[test]
fn exact_pinned_codes_domains_templates_and_unknown_values() {
    let mut values = HashSet::new();
    let fixture = include_str!("shared_errors/catalog.tsv");

    for (definition, row) in ErrorCode::definitions().zip(fixture.lines()) {
        let fields: Vec<_> = row.split('\t').collect();
        let value: i64 = fields[1].parse().unwrap();
        assert!(values.insert(value));
        assert_eq!(definition.code.value(), value);
        assert_eq!(definition.name, fields[0]);

        let domain = format!("{:?}", definition.code.domain());
        assert_eq!(
            format!("{}{}", domain[..1].to_lowercase(), &domain[1..]),
            fields[2]
        );
        let message: Vec<_> = fields[3]
            .as_bytes()
            .chunks(2)
            .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), HEX_RADIX).unwrap())
            .collect();
        assert_eq!(definition.template.as_bytes(), message);
        assert_eq!(definition.code.definition(), Some(definition));
        assert!(
            !definition
                .code
                .message(&Context::default())
                .unwrap()
                .is_empty()
        );
    }

    assert_eq!(values.len(), 365);
    assert_eq!(fixture.lines().count(), 365);
    assert_eq!(ErrorCode::definitions().count(), 365);
}

#[test]
fn unknown_codes_preserve_exact_integers() {
    for value in [
        i64::MIN,
        i64::MAX,
        -BELOW_JSON_RPC,
        0,
        UNDEFINED_CODEC,
        HISTORICAL_ACCOUNT_TYPO,
    ] {
        let code = ErrorCode::from_value(value);
        assert_eq!(code.value(), value);

        if value != HISTORICAL_ACCOUNT_TYPO {
            assert_eq!(code.definition(), None);
        }
    }

    assert_eq!(
        ErrorCode::from_value(HISTORICAL_ACCOUNT_TYPO).domain(),
        ErrorDomain::Accounts
    );
    assert_eq!(ErrorCode::from_value(11).domain(), ErrorDomain::Unknown);
    assert_eq!(
        ErrorCode::from_value(FIXED_POINT_CODE).domain(),
        ErrorDomain::Unknown
    );
    assert_eq!(ErrorCode::from_value(-32099).domain(), ErrorDomain::JsonRpc);
}

#[test]
fn malformed_boundaries_keep_distinct_facts_cursors_and_input() {
    let limits = Limits::new(BUDGET, BUDGET);
    let bytes = [255, 8, 9];
    let original = bytes;
    let mut reader = Reader::new(&bytes, limits).unwrap();
    let boolean = Bool.read(&mut reader).unwrap_err();
    assert_eq!(boolean, Error::InvalidBoolean { value: 255 });
    assert_eq!(reader.position(), 1);
    assert_eq!(
        boolean.error_code(),
        Some(ErrorCode::CODECS_INVALID_BOOLEAN)
    );

    assert_eq!(
        boolean.error_context().get("value"),
        Some(&ContextValue::Unsigned(255))
    );
    let constant = Constant(&[1, 2]).read(&mut reader).unwrap_err();
    assert_eq!(
        constant,
        Error::InvalidConstant {
            offset: 1,
            length: 2
        }
    );
    assert_eq!(reader.position(), 3);
    assert_eq!(
        constant.error_code(),
        Some(ErrorCode::CODECS_INVALID_CONSTANT)
    );
    assert_eq!(bytes, original);
}

#[test]
fn truncated_range_and_account_failures_keep_facts() {
    let limits = Limits::new(BUDGET, BUDGET);
    let bytes = [255, 8, 9];
    let boolean = Error::InvalidBoolean { value: 255 };
    assert_eq!(
        Number::<u32>::LE
            .decode(&bytes, limits)
            .unwrap_err()
            .error_code(),
        Some(ErrorCode::CODECS_INVALID_BYTE_LENGTH)
    );
    assert_eq!(
        ensure_number_range(&ABOVE_RANGE, &0, &RANGE_MAX)
            .unwrap_err()
            .error_code(),
        Some(ErrorCode::CODECS_NUMBER_OUT_OF_RANGE)
    );

    assert_eq!(
        Address::try_from(&bytes[..]).unwrap_err().error_code(),
        Some(ErrorCode::ADDRESSES_INVALID_BYTE_LENGTH)
    );
    let missing = MaybeAccount::<()>::Missing(Address::from_bytes([0; 32]))
        .require()
        .unwrap_err();
    assert_eq!(
        missing.error_code(),
        Some(ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND)
    );
    let account = AccountError::Decode {
        address: Address::from_bytes([0; 32]),
        source: boolean,
    };
    assert_eq!(
        account.error_code(),
        Some(ErrorCode::ACCOUNTS_FAILED_TO_DECODE_ACCOUNT)
    );
    assert_eq!(
        account.source().unwrap().downcast_ref::<Error>(),
        Some(&boolean)
    );
}

#[test]
fn bounded_public_context_interpolation_and_nested_sources() {
    let mut context = Context::default();
    context
        .insert("value", ContextValue::Unsigned(255))
        .unwrap();
    assert_eq!(
        ErrorCode::CODECS_INVALID_BOOLEAN.message(&context).unwrap(),
        "Invalid boolean value. Expected 0 or 1, got 255."
    );
    assert_eq!(
        ErrorCode::CODECS_INVALID_BOOLEAN
            .message(&Context::default())
            .unwrap(),
        "Invalid boolean value. Expected 0 or 1, got $value."
    );
    assert_eq!(
        ErrorCode::from_value(i64::MAX).message(&context).unwrap(),
        "Solana error #9223372036854775807"
    );
    assert_eq!(
        context.insert("privateKey", ContextValue::Text("secret".into())),
        Err(ContextError::Key)
    );
    assert_eq!(
        context.insert(
            "message",
            ContextValue::Text("x".repeat(OVER_TEXT_LIMIT as usize))
        ),
        Err(ContextError::Limit)
    );
    let cause = Diagnostic::new(ErrorCode::CODECS_INVALID_BOOLEAN, context)
        .with_source(Error::InvalidBoolean { value: 255 });

    let outer = Diagnostic::new(
        ErrorCode::JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE,
        Context::default(),
    )
    .with_source(cause);
    let inner = outer
        .simulation_cause()
        .downcast_ref::<Diagnostic>()
        .unwrap();
    assert_eq!(inner.code(), ErrorCode::CODECS_INVALID_BOOLEAN);
    assert_eq!(
        inner.source().unwrap().downcast_ref::<Error>(),
        Some(&Error::InvalidBoolean { value: 255 })
    );
    assert_eq!(outer.to_string(), "SolanaError#-32002");
    assert!(!format!("{outer:?}").contains("255"));
}

#[cfg(feature = "crypto")]
#[test]
fn actual_pda_failures_expose_codes_without_seed_bytes() {
    use solana_kit_core::crypto::find_program_address;
    let seed = [42; 33];
    let failure = find_program_address(&[&seed], &Address::from_bytes([0; 32])).unwrap_err();
    assert_eq!(
        failure.error_code(),
        Some(ErrorCode::ADDRESSES_MAX_PDA_SEED_LENGTH_EXCEEDED)
    );
    assert!(!failure.to_string().contains("42"));
}

#[test]
fn nested_keys_aggregate_storage_and_normalized_typed_causes() {
    use std::collections::BTreeMap;

    for depth in 0..=7 {
        let mut value =
            ContextValue::Object(BTreeMap::from([("x".repeat(65), ContextValue::Null)]));

        for _ in 0..depth {
            value = ContextValue::List(vec![value]);
        }

        assert_eq!(
            Context::default().insert("value", value),
            Err(ContextError::Limit)
        );
    }

    let mut context = Context::default();
    context.insert("path", ContextValue::Null).unwrap();

    context
        .insert("operation", ContextValue::Text("testOperation".into()))
        .unwrap();
    let mut inner = Context::default();
    inner.insert("slot", ContextValue::Integer(55)).unwrap();
    let wrapped = Diagnostic::wrap(
        ErrorCode::ACCOUNTS_FAILED_TO_DECODE_ACCOUNT,
        context,
        Diagnostic::new(ErrorCode::BLOCK_HEIGHT_EXCEEDED, inner),
    );
    assert_eq!(wrapped.context().get("path"), None);
    let cause = wrapped
        .source()
        .unwrap()
        .downcast_ref::<Diagnostic>()
        .unwrap();
    assert_eq!(cause.code(), ErrorCode::BLOCK_HEIGHT_EXCEEDED);
    assert_eq!(
        cause.context().get("slot"),
        Some(&ContextValue::Integer(55))
    );

    let mut bounded = Context::default();

    for index in 0..15 {
        bounded
            .insert(
                &format!("field{index}"),
                ContextValue::Text("x".repeat(1024)),
            )
            .unwrap();
    }

    assert_eq!(
        bounded.insert("last", ContextValue::Text("x".repeat(1024))),
        Err(ContextError::Limit)
    );
    assert_eq!(
        bounded.insert("accessToken", ContextValue::Text("credential".into())),
        Err(ContextError::Key)
    );
    bounded
        .insert("preTokenBalances", ContextValue::List(vec![]))
        .unwrap();
}

#[test]
fn helper_normalization_matches_independent_dart_facts() {
    let fixture = include_str!("shared_errors/helper.json");
    assert_eq!(
        fixture.trim(),
        r#"{"operation":"testOperation","hasPath":false,"causeCode":1,"causeContext":{"slot":55},"causeType":"SolanaError"}"#
    );
    let mut context = Context::default();
    context
        .insert("operation", ContextValue::Text("testOperation".into()))
        .unwrap();
    context.insert("path", ContextValue::Null).unwrap();
    let mut cause = Context::default();
    cause.insert("slot", ContextValue::Integer(55)).unwrap();
    let error = Diagnostic::wrap(
        ErrorCode::ACCOUNTS_FAILED_TO_DECODE_ACCOUNT,
        context,
        Diagnostic::new(ErrorCode::BLOCK_HEIGHT_EXCEEDED, cause),
    );

    assert_eq!(error.context().get("path"), None);
    assert_eq!(
        error.context().get("operation"),
        Some(&ContextValue::Text("testOperation".into()))
    );
    let source = error
        .source()
        .unwrap()
        .downcast_ref::<Diagnostic>()
        .unwrap();
    assert_eq!(source.code().value(), 1);
    assert_eq!(
        source.context().get("slot"),
        Some(&ContextValue::Integer(55))
    );
}

#[test]
fn sensitive_nested_keys_and_message_growth_fail_before_exposure() {
    use std::collections::BTreeMap;
    let mut context = Context::default();
    context.insert("slot", ContextValue::Integer(1)).unwrap();
    let original = context.clone();
    let nested = ContextValue::Object(BTreeMap::from([(
        "access_token".into(),
        ContextValue::Text("credential".into()),
    )]));
    assert_eq!(
        context.insert("value", ContextValue::List(vec![nested])),
        Err(ContextError::Key)
    );
    assert_eq!(context, original);
    context
        .insert(
            "address",
            ContextValue::List(vec![ContextValue::Text("x".repeat(1024)); 4]),
        )
        .unwrap();
    assert_eq!(
        ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND.message(&context),
        Err(ContextError::Limit)
    );
}
