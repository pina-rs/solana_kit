//! Literal fixtures exercise the public framing and typed-method boundary.
use futures::executor::block_on;
use solana_kit_client::methods::history::*;
use solana_kit_client::methods::submission::*;
use solana_kit_client::methods::{Commitment, Method, ReadConfig};
use solana_kit_client::{Client, Error, Limits, Reply, Request, Transport, TransportError};
use solana_kit_core::{Address, Signature};
use std::future::{Ready, ready};

#[path = "fixtures/submission_wire.rs"]
mod submission_wire;

struct Fixture {
    method: &'static str,
    params: serde_json::Value,
    result: String,
}
impl Transport for Fixture {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let actual: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(
            actual,
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":self.method,"params":self.params})
        );
        ready(Ok(Reply {
            status: 200,
            body: format!(
                "{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{}}}",
                self.result
            )
            .into_bytes(),
        }))
    }
}
fn call<M: Method>(
    method: M,
    params: serde_json::Value,
    result: &str,
) -> Result<M::Response, Error> {
    let fixture = Fixture {
        method: M::NAME,
        params,
        result: result.to_owned(),
    };
    block_on(Client::new(fixture, Limits::default()).call(method))
}
fn address() -> Address {
    Address::from_bytes([0; 32])
}
fn signature() -> Signature {
    Signature::from_bytes([0; 64])
}

#[test]
fn historical_scalar_and_null_results() {
    assert_eq!(
        call(
            BlockTime(u64::MAX),
            serde_json::json!([u64::MAX]),
            "-9223372036854775808"
        )
        .unwrap(),
        Some(i64::MIN)
    );
    assert_eq!(
        call(BlockTime(1), serde_json::json!([1]), "null").unwrap(),
        None
    );
    assert_eq!(
        call(
            FirstAvailableBlock,
            serde_json::json!([]),
            "18446744073709551615"
        )
        .unwrap(),
        u64::MAX
    );
    assert_eq!(
        call(MinimumLedgerSlot, serde_json::json!([]), "0").unwrap(),
        0
    );

    for invalid in ["1.0", "1e2", "\"1\"", "9223372036854775808", "{}"] {
        assert!(matches!(
            call(BlockTime(1), serde_json::json!([1]), invalid),
            Err(Error::Schema(_))
        ));
    }

    for invalid in ["null", "-1", "18446744073709551616", "false"] {
        assert!(call(FirstAvailableBlock, serde_json::json!([]), invalid).is_err());
    }
}

#[test]
fn commitment_depth_has_fixed_shape_and_required_null() {
    let result = call(
        BlockCommitment(2),
        serde_json::json!([2]),
        r#"{"commitment":null,"totalStake":18446744073709551615}"#,
    )
    .unwrap();
    assert!(result.commitment.is_none());
    assert_eq!(result.total_stake, u64::MAX);
    let stakes = vec![u64::MAX; 32];
    let json = serde_json::json!({"commitment":stakes,"totalStake":0}).to_string();
    assert_eq!(
        call(BlockCommitment(2), serde_json::json!([2]), &json)
            .unwrap()
            .commitment
            .unwrap(),
        [u64::MAX; 32]
    );

    for invalid in [
        r#"{"totalStake":0}"#,
        r#"{"commitment":[],"totalStake":0}"#,
        r#"{"commitment":null}"#,
        r#"{"commitment":null,"totalStake":-1}"#,
    ] {
        assert!(call(BlockCommitment(2), serde_json::json!([2]), invalid).is_err());
    }
}

#[test]
fn inclusive_ranges_and_optional_config_occupy_correct_positions() {
    let config = BlocksConfig {
        commitment: Some(HistoricalCommitment::Finalized),
    };
    assert_eq!(
        call(
            Blocks {
                start: 2,
                end: Some(4),
                config: Some(config)
            },
            serde_json::json!([2,4,{"commitment":"finalized"}]),
            "[2,4]"
        )
        .unwrap(),
        [2, 4]
    );
    assert_eq!(
        call(
            Blocks {
                start: 2,
                end: None,
                config: Some(BlocksConfig::default())
            },
            serde_json::json!([2, {}]),
            "[]"
        )
        .unwrap(),
        Vec::<u64>::new()
    );
    assert_eq!(
        call(
            Blocks {
                start: u64::MAX,
                end: None,
                config: None
            },
            serde_json::json!([u64::MAX]),
            "[18446744073709551615]"
        )
        .unwrap(),
        [u64::MAX]
    );
}

#[test]
fn block_limits_and_malformed_ranges_are_rejected() {
    let config = BlocksConfig {
        commitment: Some(HistoricalCommitment::Finalized),
    };
    assert_eq!(
        call(
            BlocksWithLimit {
                start: 2,
                limit: 2,
                config: Some(config)
            },
            serde_json::json!([2,2,{"commitment":"finalized"}]),
            "[2,9]"
        )
        .unwrap(),
        [2, 9]
    );
    assert_eq!(
        call(
            BlocksWithLimit {
                start: 2,
                limit: 0,
                config: None
            },
            serde_json::json!([2, 0]),
            "[]"
        )
        .unwrap(),
        Vec::<u64>::new()
    );

    for result in ["null", "[1]", "[2,2]", "[4,2]", "[2,5]", "[2.0]"] {
        assert!(
            call(
                Blocks {
                    start: 2,
                    end: Some(4),
                    config: None
                },
                serde_json::json!([2, 4]),
                result
            )
            .is_err()
        );
    }

    assert!(
        call(
            BlocksWithLimit {
                start: 0,
                limit: 1,
                config: None
            },
            serde_json::json!([0, 1]),
            "[0,1]"
        )
        .is_err()
    );
    assert!(
        serde_json::to_string(&BlocksWithLimit {
            start: 0,
            limit: 500001,
            config: None
        })
        .is_err()
    );
    assert!(
        serde_json::to_string(&Blocks {
            start: 2,
            end: Some(1),
            config: None
        })
        .is_err()
    );
}

#[test]
fn fee_and_airdrop_exact_integer_params() {
    let read = ReadConfig {
        commitment: Some(Commitment::Processed),
        min_context_slot: Some(u64::MAX),
    };
    let result = call(
        FeeForMessage(&[0, 1, 2], Some(read)),
        serde_json::json!(["AAEC",{"commitment":"processed","minContextSlot":u64::MAX}]),
        r#"{"context":{"slot":18446744073709551615},"value":18446744073709551615}"#,
    )
    .unwrap();
    assert_eq!(result.value, Some(u64::MAX));
    assert!(
        call(
            FeeForMessage(&[], None),
            serde_json::json!([""]),
            r#"{"context":{"slot":0},"value":null}"#
        )
        .unwrap()
        .value
        .is_none()
    );
    assert!(
        call(
            FeeForMessage(&[], None),
            serde_json::json!([""]),
            r#"{"context":{"slot":0}}"#
        )
        .is_err()
    );
    let config = AirdropConfig {
        commitment: Some(Commitment::Confirmed),
    };
    let result = call(
        RequestAirdrop(address(), u64::MAX, Some(config)),
        serde_json::json!([address().to_string(),u64::MAX,{"commitment":"confirmed"}]),
        &format!("\"{}\"", signature()),
    )
    .unwrap();
    assert_eq!(result, signature());

    for invalid in ["null", "\"0\"", "\"111\"", "[]"] {
        assert!(
            call(
                RequestAirdrop(address(), 0, None),
                serde_json::json!([address().to_string(), 0]),
                invalid
            )
            .is_err()
        );
    }
}

#[test]
fn send_keeps_wire_bytes_version_and_node_policy_explicit() {
    use base64::{Engine, engine::general_purpose::STANDARD};
    let wire = submission_wire::wire(255, 7);
    let expected = Signature::from_bytes([7; 64]);
    let config = SendConfig {
        skip_preflight: Some(false),
        preflight_commitment: Some(Commitment::Finalized),
        max_retries: Some(u64::MAX),
        min_context_slot: Some(u64::MAX),
    };
    let params = serde_json::json!([STANDARD.encode(&wire), {
        "encoding":"base64", "skipPreflight":false,
        "preflightCommitment":"finalized", "maxRetries":u64::MAX,
        "minContextSlot":u64::MAX
    }]);
    let method = SendTransaction {
        wire: SignedWire::new(&wire).unwrap(),
        encoding: WireEncoding::Base64,
        config,
    };
    let result = call(method, params, &format!("\"{expected}\"")).unwrap();
    assert_eq!(result, expected);
    call(
        SendTransaction {
            wire: SignedWire::new(&wire).unwrap(),
            encoding: WireEncoding::Base58,
            config: SendConfig::default(),
        },
        serde_json::json!([bs58::encode(&wire).into_string(),{"encoding":"base58"}]),
        &format!("\"{expected}\""),
    )
    .unwrap();

    let version_one_wire = submission_wire::full_v1();
    let request = SendTransaction {
        wire: SignedWire::new(&version_one_wire).unwrap(),
        encoding: WireEncoding::Base64,
        config: SendConfig::default(),
    };
    let encoded = serde_json::to_string(&request).unwrap();
    assert!(encoded.len() > 5000);
    assert!(!encoded.contains("legacy"));
}

#[test]
fn signature_history_pagination_and_metadata() {
    let config = SignaturesConfig {
        before: Some(signature()),
        until: Some(signature()),
        limit: Some(1),
        commitment: Some(HistoricalCommitment::Confirmed),
        min_context_slot: Some(u64::MAX),
    };
    let json = format!(
        r#"[{{"signature":"{}",
"slot":18446744073709551615,
"err":null,
"memo":null,
"blockTime":-9223372036854775808,
"confirmationStatus":"finalized"}}]"#,
        signature()
    );
    let params = serde_json::json!([address().to_string(), {
        "before":signature().to_string(), "until":signature().to_string(),
        "limit":1, "commitment":"confirmed", "minContextSlot":u64::MAX
    }]);
    let result = call(SignaturesForAddress(address(), Some(config)), params, &json).unwrap();
    assert_eq!(result[0].signature, signature());
    assert!(result[0].err.is_none());
    assert_eq!(result[0].block_time, Some(i64::MIN));

    for invalid in [
        json.replace(&signature().to_string(), "0"),
        json.replace("\"err\":null,", ""),
        json.replace("\"memo\":null", "\"memo\":7"),
        json.replace("finalized", "unknown"),
    ] {
        assert!(
            call(
                SignaturesForAddress(address(), None),
                serde_json::json!([address().to_string()]),
                &invalid
            )
            .is_err()
        );
    }

    let above_maximum_limit = 1001;

    for limit in [0, above_maximum_limit, u16::MAX] {
        assert!(
            serde_json::to_string(&SignaturesConfig {
                limit: Some(limit),
                ..Default::default()
            })
            .is_err()
        );
    }
}

#[test]
fn positional_statuses_preserve_null_and_execution_failure() {
    let signatures = [signature(), signature()];
    let params = serde_json::json!([
        [signature().to_string(),signature().to_string()],
        {"searchTransactionHistory":false}
    ]);
    let response = r#"{"context":{"slot":18446744073709551615},
"value":[null,
{
        "slot":18446744073709551615,
"confirmations":null,

        "err":{"InstructionError":[255,
{"Custom":4294967295}]},

        "status":{"Err":{"InstructionError":[255,
{"Custom":4294967295}]}},

        "confirmationStatus":"finalized"}]}"#;
    let config = StatusesConfig {
        search_transaction_history: Some(false),
    };
    let result = call(
        SignatureStatuses(&signatures, Some(config)),
        params,
        response,
    )
    .unwrap();
    assert!(result.value[0].is_none());
    assert!(matches!(
        result.value[1].as_ref().unwrap().err,
        Some(TransactionFailure::Known(
            KnownTransactionFailure::InstructionError(255, InstructionFailure::Custom(u32::MAX))
        ))
    ));
    let one = [signature()];
    let params = serde_json::json!([[signature().to_string()]]);

    for invalid in [
        r#"{"context":{"slot":0},"value":[]}"#,
        r#"{"context":{"slot":0},"value":[{"slot":1,"confirmations":null}]}"#,
        r#"{"context":{"slot":0},
"value":[{"slot":1,
"confirmations":null,
"err":null,
"status":{"Err":"AccountInUse"}}]}"#,
    ] {
        assert!(call(SignatureStatuses(&one, None), params.clone(), invalid).is_err());
    }

    assert!(serde_json::to_string(&SignatureStatuses(&[signature(); 257], None)).is_err());
    call(
        SignatureStatuses(&[], None),
        serde_json::json!([[]]),
        r#"{"context":{"slot":0},"value":[]}"#,
    )
    .unwrap();
}

#[test]
fn known_errors_validate_payloads_and_unknown_future_errors_preserve_text() {
    for error in [
        r#"{"InstructionError":[256,"InvalidArgument"]}"#,
        r#"{"InstructionError":[0,{"Custom":4294967296}]}"#,
        r#"{"DuplicateInstruction":256}"#,
        r#"{"InsufficientFundsForRent":{"account_index":256}}"#,
        r#"{"AccountInUse":7}"#,
        "false",
        "42",
        "[]",
        "{}",
    ] {
        assert!(
            serde_json::from_str::<TransactionFailure>(error).is_err(),
            "accepted {error}"
        );
    }

    for error in [
        r#""AccountInUse""#,
        r#"{"DuplicateInstruction":255}"#,
        r#"{"InsufficientFundsForRent":{"account_index":255}}"#,
    ] {
        assert!(matches!(
            serde_json::from_str::<TransactionFailure>(error).unwrap(),
            TransactionFailure::Known(_)
        ));
    }

    let exact = r#"{ "FutureExecutionVariant" : [1e400, 184467440737095516160] }"#;
    let TransactionFailure::Unknown(raw) = serde_json::from_str(exact).unwrap() else {
        panic!("future variant lost")
    };
    assert_eq!(raw.get(), exact);
}

#[test]
fn required_nullable_status_and_history_members_cannot_disappear() {
    let status = serde_json::json!({
        "slot":0, "confirmations":null, "err":null,
        "status":{"Ok":null}, "confirmationStatus":null
    });
    let signatures = [signature()];
    let params = serde_json::json!([[signature().to_string()]]);

    for field in [
        "slot",
        "confirmations",
        "err",
        "status",
        "confirmationStatus",
    ] {
        let mut invalid = status.clone();
        invalid.as_object_mut().unwrap().remove(field);
        let result = serde_json::json!({"context":{"slot":0},"value":[invalid]});
        assert!(
            call(
                SignatureStatuses(&signatures, None),
                params.clone(),
                &result.to_string()
            )
            .is_err()
        );
    }

    let history = serde_json::json!({
        "signature":signature().to_string(), "slot":0, "err":null,
        "memo":null, "blockTime":null, "confirmationStatus":null
    });
    let params = serde_json::json!([address().to_string()]);
    let result = format!("[{history}]");
    call(
        SignaturesForAddress(address(), None),
        params.clone(),
        &result,
    )
    .unwrap();

    for field in [
        "signature",
        "slot",
        "err",
        "memo",
        "blockTime",
        "confirmationStatus",
    ] {
        let mut invalid = history.clone();
        invalid.as_object_mut().unwrap().remove(field);
        let result = format!("[{invalid}]");
        assert!(
            call(
                SignaturesForAddress(address(), None),
                params.clone(),
                &result
            )
            .is_err()
        );
    }
}
