//! Source-grounded literal RPC fixtures exercise the public Client boundary.
#[path = "transaction_results/regressions.rs"]
mod regressions;

use futures::executor::block_on;
use serde_json::{Value, json};
use solana_kit_client::methods::Method;
use solana_kit_client::methods::transaction_results::*;
use solana_kit_client::{Client, Error, Limits, Reply, Request, Transport, TransportError};
use solana_kit_core::Signature;
use std::future::{Ready, ready};

const BLOCK_SLOT: u64 = 42;
const INVALID_FRACTION: f64 = 1.5;
const ADDRESS: &str = "11111111111111111111111111111111";
const SIGNATURE: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const RAW_MESSAGE: &str = r#"{
    "header":{"numRequiredSignatures":1,"numReadonlySignedAccounts":0,
              "numReadonlyUnsignedAccounts":0},
    "accountKeys":["11111111111111111111111111111111"],
    "recentBlockhash":"11111111111111111111111111111111",
    "instructions":[{"programIdIndex":0,"accounts":[0,0],"data":"12",
                     "stackHeight":null}],
    "addressTableLookups":[],
    "transactionConfig":{"priorityFee":18446744073709551615,
       "computeUnitLimit":4294967295,"loadedAccountsDataSizeLimit":null,
       "heapSize":32768}}
"#;
const META: &str = r#"{
    "err":null,"status":{"Ok":null},"fee":18446744073709551615,
    "preBalances":[18446744073709551615],"postBalances":[0],
    "innerInstructions":[{"index":0,"instructions":[{"programIdIndex":0,
       "accounts":[0],"data":"12","stackHeight":2}]}],
    "logMessages":["fixture"],"preTokenBalances":[],"postTokenBalances":[{
      "accountIndex":0,"mint":"11111111111111111111111111111111",
      "owner":"11111111111111111111111111111111",
      "programId":"11111111111111111111111111111111",
      "uiTokenAmount":{"amount":"18446744073709551615","decimals":18,
          "uiAmount":18.446744073709551615,"uiAmountString":"18.446744073709551615"}}],
    "loadedAddresses":{"writable":[],"readonly":[]},"rewards":[{
      "pubkey":"11111111111111111111111111111111","lamports":-9223372036854775808,
      "postBalance":18446744073709551615,"rewardType":"voting",
      "commission":100,"commissionBps":10000}],
    "returnData":{"programId":"11111111111111111111111111111111",
                  "data":["AAE=","base64"]},
    "computeUnitsConsumed":18446744073709551615,"costUnits":18446744073709551615}
"#;
struct Fixture {
    method: &'static str,
    params: Value,
    result: String,
}
impl Transport for Fixture {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let actual: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(
            actual,
            json!({"jsonrpc":"2.0","id":1,
            "method":self.method,"params":self.params})
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
fn call<M: Method>(method: M, params: Value, result: &str) -> Result<M::Response, Error> {
    let fixture = Fixture {
        method: M::NAME,
        params,
        result: result.into(),
    };
    block_on(
        Client::new(
            fixture,
            Limits {
                bytes: 32768,
                depth: 32,
            },
        )
        .call(method),
    )
}
fn signature() -> Signature {
    SIGNATURE.parse().unwrap()
}
fn full(transaction: &str, meta: &str, version: &str) -> String {
    format!(
        r#"{{"slot":18446744073709551615,"blockTime":-9223372036854775808,
       "transactionIndex":4294967295,"transaction":{transaction},"meta":{meta}{version}}}"#
    )
}
fn transaction(message: &str) -> String {
    format!(r#"{{"signatures":["{SIGNATURE}"],"message":{message}}}"#)
}
fn block(extra: &str) -> String {
    format!(
        r#"{{"blockhash":"{ADDRESS}","previousBlockhash":"{ADDRESS}",
        "parentSlot":18446744073709551615,"blockTime":null,"blockHeight":null{extra}}}"#
    )
}
#[test]
fn transaction_encodings_are_request_selected_and_bytes_are_not_verified() {
    for (encoding, tag, wire) in [
        (TransactionEncoding::Base58, "base58", "12"),
        (TransactionEncoding::Base64, "base64", "AAE="),
    ] {
        let config = TransactionReadConfig {
            encoding: Some(encoding),
            max_supported_transaction_version: Some(1),
            ..Default::default()
        };
        let result = full(&format!(r#"["{wire}","{tag}"]"#), META, r#","version":1"#);
        let value = call(
            Transaction(signature(), Some(config)),
            json!([SIGNATURE,{"encoding":tag,"maxSupportedTransactionVersion":1}]),
            &result,
        )
        .unwrap()
        .unwrap();
        assert_eq!(value.slot, u64::MAX);
        assert_eq!(value.block_time, Some(i64::MIN));

        assert!(matches!(value.details.transaction, TransactionView::Binary(v) if v == [0,1]));
        let meta = value.details.meta.unwrap();
        assert_eq!(meta.fee, u64::MAX);
        let solana_kit_client::Presence::Present(Some(balances)) = meta.post_token_balances else {
            panic!("missing token balances")
        };
        assert_eq!(balances[0].ui_token_amount.amount, "18446744073709551615");
    }
}
#[test]
fn json_and_parsed_messages_have_typed_routing_and_exact_opaque_program_data() {
    let config = TransactionReadConfig {
        max_supported_transaction_version: Some(1),
        ..Default::default()
    };
    let result = full(&transaction(RAW_MESSAGE), META, r#","version":1"#);
    let value = call(
        Transaction(signature(), Some(config)),
        json!([SIGNATURE,{"maxSupportedTransactionVersion":1}]),
        &result,
    )
    .unwrap()
    .unwrap();
    let TransactionView::Json(tx) = value.details.transaction else {
        panic!("wrong variant")
    };
    assert_eq!(tx.message.account_keys.len(), 1);
    assert_eq!(tx.message.instructions[0].data, [0, 1]);
    let parsed = format!(
        r#"{{"accountKeys":[{{"pubkey":"{ADDRESS}","signer":true,
      "writable":true,"source":"transaction"}}],"recentBlockhash":"{ADDRESS}",
      "instructions":[{{"programId":"{ADDRESS}","program":"fixture",
      "parsed": {{ "amount":1e400 }},"stackHeight":null}},
      {{"programId":"{ADDRESS}","accounts":["{ADDRESS}"],"data":"12"}}]}}"#
    );

    let result = full(&transaction(&parsed), "null", "");
    let config = TransactionReadConfig {
        encoding: Some(TransactionEncoding::JsonParsed),
        ..Default::default()
    };
    let value = call(
        Transaction(signature(), Some(config)),
        json!([SIGNATURE,{"encoding":"jsonParsed"}]),
        &result,
    )
    .unwrap()
    .unwrap();
    let TransactionView::JsonParsed(tx) = value.details.transaction else {
        panic!("wrong variant")
    };
    let ParsedOrPartialInstruction::Parsed(ix) = &tx.message.instructions[0] else {
        panic!("wrong instruction")
    };
    assert_eq!(ix.parsed.get(), r#"{ "amount":1e400 }"#);
}

#[test]
fn block_details_and_rewards_match_the_request() {
    for mode in [
        TransactionDetails::None,
        TransactionDetails::Signatures,
        TransactionDetails::Accounts,
        TransactionDetails::Full,
    ] {
        check_block_mode(mode, false);
        check_block_mode(mode, true);
    }

    assert!(call(Block(1, None), json!([1]), &block("")).is_err());
    assert!(call(Block(1, None), json!([1]), "null").unwrap().is_none());

    assert!(
        call(Transaction(signature(), None), json!([SIGNATURE]), "null")
            .unwrap()
            .is_none()
    );
}
#[test]
// Required baseline members are removed before independent scalar type mutations.
fn required_fields_and_typed_numbers_fail_source_backed_mutation() {
    let result = full(&transaction(RAW_MESSAGE), META, "");
    let original: Value = serde_json::from_str(&result).unwrap();

    for field in ["slot", "blockTime", "transaction", "meta"] {
        let mut mutation = original.clone();
        mutation.as_object_mut().unwrap().remove(field);
        assert!(
            call(
                Transaction(signature(), None),
                json!([SIGNATURE]),
                &mutation.to_string()
            )
            .is_err(),
            "missing {field}"
        );
    }

    for field in ["err", "status", "fee", "preBalances", "postBalances"] {
        let mut mutation = original.clone();

        mutation["meta"].as_object_mut().unwrap().remove(field);
        assert!(
            call(
                Transaction(signature(), None),
                json!([SIGNATURE]),
                &mutation.to_string()
            )
            .is_err(),
            "meta missing {field}"
        );
    }
}

#[test]
fn exact_integer_fields_reject_type_mutations() {
    let result = full(&transaction(RAW_MESSAGE), META, "");
    let original: Value = serde_json::from_str(&result).unwrap();

    for pointer in [
        "/slot",
        "/meta/fee",
        "/meta/preBalances/0",
        "/transaction/message/header/numRequiredSignatures",
        "/transaction/message/instructions/0/programIdIndex",
        "/transaction/message/instructions/0/accounts/0",
    ] {
        for invalid in [
            json!("1"),
            json!(-1),
            json!(INVALID_FRACTION),
            Value::Null,
            json!({}),
        ] {
            let mut mutation = original.clone();
            *mutation.pointer_mut(pointer).unwrap() = invalid;
            assert!(
                call(
                    Transaction(signature(), None),
                    json!([SIGNATURE]),
                    &mutation.to_string()
                )
                .is_err(),
                "{pointer}"
            );
        }
    }
}
#[test]
fn mismatched_encodings_versions_domain_values_and_nulls_reject() {
    let config = TransactionReadConfig {
        encoding: Some(TransactionEncoding::Base64),
        max_supported_transaction_version: Some(1),
        ..Default::default()
    };
    let params = json!([SIGNATURE,{"encoding":"base64","maxSupportedTransactionVersion":1}]);
    for (tx, version) in [
        (r#"["AAE=","base58"]"#, r#","version":1"#),
        (r#"["AB==","base64"]"#, r#","version":1"#),
        (r#"["AAE=","base64"]"#, r#","version":2"#),
        (r#"["AAE=","base64"]"#, r#","version":null"#),
        (r#"["AAE=","base64",0]"#, r#","version":1"#),
    ] {
        assert!(
            call(
                Transaction(signature(), Some(config)),
                params.clone(),
                &full(tx, "null", version)
            )
            .is_err()
        );
    }
    for (from, to) in [
        (ADDRESS, "bad-address"),
        (SIGNATURE, "bad-signature"),
        ("\"amount\":\"18446744073709551615\"", "\"amount\":\"01\""),
        (
            "\"postBalance\":18446744073709551615",
            "\"postBalance\":18446744073709551616",
        ),
    ] {
        let result = full(&transaction(RAW_MESSAGE), META, "").replace(from, to);
        assert!(call(Transaction(signature(), None), json!([SIGNATURE]), &result).is_err());
    }
}

fn check_block_mode(mode: TransactionDetails, rewards: bool) {
    let (name, data) = block_mode_data(mode);
    let reward = if rewards { r#","rewards":[]"# } else { "" };
    let config = BlockConfig {
        transaction_details: Some(mode),
        rewards: Some(rewards),
        ..Default::default()
    };
    let value = call(
        Block(BLOCK_SLOT, Some(config)),
        json!([BLOCK_SLOT,{"transactionDetails":name,"rewards":rewards}]),
        &block(&format!("{data}{reward}")),
    )
    .unwrap()
    .unwrap();

    assert_eq!(value.parent_slot, u64::MAX);
}
fn block_mode_data(mode: TransactionDetails) -> (&'static str, String) {
    match mode {
        TransactionDetails::None => ("none", String::new()),
        TransactionDetails::Signatures => {
            ("signatures", format!(r#","signatures":["{SIGNATURE}"]"#))
        }
        TransactionDetails::Accounts => (
            "accounts",
            format!(
                r#","transactions":[{{"transaction":{{"signatures":["{SIGNATURE}"],"accountKeys":[]}},"meta":null}}]"#
            ),
        ),
        TransactionDetails::Full => (
            "full",
            format!(
                r#","transactions":[{{"transaction":{},"meta":null}}]"#,
                transaction(RAW_MESSAGE)
            ),
        ),
    }
}

#[test]
fn every_full_block_encoding_executes_and_transaction_config_omits_defaults() {
    for (encoding, spelling, encoded) in [
        (TransactionEncoding::Json, "json", transaction(RAW_MESSAGE)),
        (
            TransactionEncoding::JsonParsed,
            "jsonParsed",
            transaction(&format!(
                r#"{{"accountKeys":[],"recentBlockhash":"{ADDRESS}","instructions":[]}}"#
            )),
        ),
        (
            TransactionEncoding::Base58,
            "base58",
            r#"["12","base58"]"#.into(),
        ),
        (
            TransactionEncoding::Base64,
            "base64",
            r#"["AAE=","base64"]"#.into(),
        ),
    ] {
        let config = BlockConfig {
            encoding: Some(encoding),
            rewards: Some(false),
            ..Default::default()
        };
        let result = block(&format!(
            r#","transactions":[{{"transaction":{encoded},"meta":null}}]"#
        ));
        assert!(
            call(
                Block(1, Some(config)),
                json!([1,{"encoding":spelling,"rewards":false}]),
                &result
            )
            .unwrap()
            .is_some()
        );
    }
}

#[test]
fn transaction_read_configuration_omits_unselected_options() {
    let config = TransactionReadConfig {
        commitment: Some(solana_kit_client::methods::history::HistoricalCommitment::Confirmed),
        ..Default::default()
    };
    assert!(
        call(
            Transaction(signature(), Some(config)),
            json!([SIGNATURE,{"commitment":"confirmed"}]),
            "null"
        )
        .unwrap()
        .is_none()
    );
    assert!(
        call(
            Transaction(signature(), Some(TransactionReadConfig::default())),
            json!([SIGNATURE, {}]),
            "null"
        )
        .unwrap()
        .is_none()
    );
}

#[test]
fn nested_schema_fields_are_required_and_optional_annotations_preserve_null() {
    let result = full(&transaction(RAW_MESSAGE), META, "");
    let original: Value = serde_json::from_str(&result).unwrap();
    let groups = [
        ("/transaction", &["signatures", "message"][..]),
        (
            "/transaction/message",
            &["header", "accountKeys", "recentBlockhash", "instructions"][..],
        ),
        (
            "/transaction/message/header",
            &[
                "numRequiredSignatures",
                "numReadonlySignedAccounts",
                "numReadonlyUnsignedAccounts",
            ][..],
        ),
        (
            "/transaction/message/instructions/0",
            &["programIdIndex", "accounts", "data"][..],
        ),
        (
            "/meta/postTokenBalances/0",
            &["accountIndex", "mint", "uiTokenAmount"][..],
        ),
        (
            "/meta/postTokenBalances/0/uiTokenAmount",
            &["amount", "decimals", "uiAmount", "uiAmountString"][..],
        ),
        (
            "/meta/rewards/0",
            &[
                "pubkey",
                "lamports",
                "postBalance",
                "rewardType",
                "commission",
            ][..],
        ),
        ("/meta/returnData", &["programId", "data"][..]),
        ("/meta/loadedAddresses", &["writable", "readonly"][..]),
    ];

    for (pointer, fields) in groups {
        check_member_removals(&original, pointer, fields);
    }
}

#[test]
fn old_node_optional_metadata_remains_absent() {
    let result = full(&transaction(RAW_MESSAGE), META, "");
    let mut minimal: Value = serde_json::from_str(&result).unwrap();
    let meta = minimal["meta"].as_object_mut().unwrap();
    meta.retain(|key, _| {
        ["err", "status", "fee", "preBalances", "postBalances"].contains(&key.as_str())
    });
    let value = call(
        Transaction(signature(), None),
        json!([SIGNATURE]),
        &minimal.to_string(),
    )
    .unwrap()
    .unwrap();
    assert!(matches!(
        value.details.meta.unwrap().return_data,
        solana_kit_client::Presence::Missing
    ));
}
fn check_member_removals(original: &Value, pointer: &str, fields: &[&str]) {
    for field in fields {
        let mut mutation = original.clone();
        mutation
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove(*field);
        assert!(
            call(
                Transaction(signature(), None),
                json!([SIGNATURE]),
                &mutation.to_string()
            )
            .is_err(),
            "{pointer}/{field}"
        );
    }
}

#[test]
fn shared_duplicate_and_limit_guards_precede_large_response_decoding() {
    let result = full(&transaction(RAW_MESSAGE), META, "");
    let duplicate = result.replace("\"fee\":18446744073709551615", "\"fee\":0,\"f\\u0065e\":1");
    assert!(matches!(
        call(
            Transaction(signature(), None),
            json!([SIGNATURE]),
            &duplicate
        ),
        Err(Error::Json(_))
    ));
    let fixture = Fixture {
        method: "getTransaction",
        params: json!([SIGNATURE]),
        result,
    };
    let limited = Client::new(
        fixture,
        Limits {
            bytes: 256,
            depth: 32,
        },
    );
    assert!(matches!(
        block_on(limited.call(Transaction(signature(), None))),
        Err(Error::BodyLimit)
    ));
}

#[test]
fn version_negotiation_retains_legacy_and_numbered_variants_without_downgrade() {
    for (maximum, version) in [
        (0, "0"),
        (0, "\"legacy\""),
        (1, "0"),
        (1, "1"),
        (1, "\"legacy\""),
    ] {
        let config = TransactionReadConfig {
            encoding: Some(TransactionEncoding::Base64),
            max_supported_transaction_version: Some(maximum),
            ..Default::default()
        };
        let result = full(
            r#"["AAE=","base64"]"#,
            "null",
            &format!(",\"version\":{version}"),
        );
        assert!(
            call(
                Transaction(signature(), Some(config)),
                json!([SIGNATURE,{"encoding":"base64","maxSupportedTransactionVersion":maximum}]),
                &result
            )
            .unwrap()
            .is_some()
        );
    }

    let unsupported = TransactionReadConfig {
        max_supported_transaction_version: Some(2),
        ..Default::default()
    };
    assert!(
        call(
            Transaction(signature(), Some(unsupported)),
            json!([]),
            "null"
        )
        .is_err()
    );
}

#[test]
fn malformed_block_shape_never_falls_back_to_a_different_details_variant() {
    let original: Value =
        serde_json::from_str(&block(r#","transactions":[],"rewards":[]"#)).unwrap();

    for field in [
        "blockhash",
        "previousBlockhash",
        "parentSlot",
        "blockTime",
        "blockHeight",
        "transactions",
        "rewards",
    ] {
        let mut mutation = original.clone();
        mutation.as_object_mut().unwrap().remove(field);
        assert!(
            call(Block(1, None), json!([1]), &mutation.to_string()).is_err(),
            "missing {field}"
        );
    }

    let config = BlockConfig {
        transaction_details: Some(TransactionDetails::None),
        rewards: Some(false),
        ..Default::default()
    };
    let params = json!([1,{"transactionDetails":"none","rewards":false}]);

    for extras in [
        r#","signatures":[]"#,
        r#","transactions":[]"#,
        r#","rewards":[]"#,
    ] {
        assert!(call(Block(1, Some(config)), params.clone(), &block(extras)).is_err());
    }
}

#[test]
fn inner_instruction_encoding_is_checked_against_the_requested_representation() {
    let compiled = full(r#"["AAE=","base64"]"#, META, "");
    let config = TransactionReadConfig {
        encoding: Some(TransactionEncoding::Base64),
        ..Default::default()
    };
    let resolved = compiled.replace(
        r#"{"programIdIndex":0,
       "accounts":[0],"data":"12","stackHeight":2}"#,
        &format!(r#"{{"programId":"{ADDRESS}","accounts":[],"data":"12"}}"#),
    );
    assert_ne!(compiled, resolved);
    assert!(
        call(
            Transaction(signature(), Some(config)),
            json!([SIGNATURE,{"encoding":"base64"}]),
            &resolved
        )
        .is_err()
    );
}
