//! Literal fixtures exercise both operations through the actual bounded Client.
use futures::executor::block_on;
use serde_json::{Value, json};
use solana_kit_client::methods::{
    Commitment, Method, accounts::AccountData, address_history::*, history::HistoricalCommitment,
    simulation::*, submission::WireEncoding, transaction_results::*,
};
use solana_kit_client::{
    Client, Error, Limits, Presence, Reply, Request, Transport, TransportError,
};
use solana_kit_core::{Address, accounts::MaybeAccount};
use std::future::{Ready, ready};

const ADDRESS: &str = "11111111111111111111111111111111";
const SIGNATURE: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const MIN_TIME: i64 = i64::MIN;
const MAX_SLOT: u64 = u64::MAX;
const OVERSIZED_WIRE: usize = 4096;
const EXECUTION: &str = r#"{"err":null,"fee":18446744073709551615,
 "loadedAddresses":null,"logs":null,"preBalances":null,"postBalances":null,
 "preTokenBalances":null,"postTokenBalances":null,"returnData":null,
 "replacementBlockhash":null,"accounts":null}"#;
struct Fixture {
    name: &'static str,
    params: Value,
    result: String,
    id: u32,
}
impl Transport for Fixture {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(
            body,
            json!({"jsonrpc":"2.0","id":1,"method":self.name,"params":self.params})
        );
        ready(Ok(Reply {
            status: 200,
            body: format!(
                r#"{{"jsonrpc":"2.0","id":{},"result":{}}}"#,
                self.id, self.result
            )
            .into_bytes(),
        }))
    }
}
fn call<M: Method>(method: M, params: Value, result: &str) -> Result<M::Response, Error> {
    block_on(
        Client::new(
            Fixture {
                name: M::NAME,
                params,
                result: result.into(),
                id: 1,
            },
            Limits {
                bytes: 65536,
                depth: 32,
            },
        )
        .call(method),
    )
}
fn address() -> Address {
    ADDRESS.parse().unwrap()
}
fn page(entry: Value) -> String {
    json!({"data":[entry],"paginationToken":null}).to_string()
}
fn signature_entry() -> Value {
    json!({"signature":SIGNATURE,"slot":MAX_SLOT,"transactionIndex":u32::MAX,
      "blockTime":MIN_TIME,"confirmationStatus":null,"memo":null,"err":null})
}
fn signatures_config() -> AddressHistoryConfig {
    AddressHistoryConfig {
        details: Some(AddressDetails::Signatures),
        ..Default::default()
    }
}
fn simulation(config: SimulationConfig) -> SimulateTransaction<'static> {
    SimulateTransaction {
        wire: &[0, 1],
        encoding: WireEncoding::Base64,
        config,
    }
}
fn contextual(value: &str) -> String {
    format!(r#"{{"context":{{"slot":18446744073709551615}},"value":{value}}}"#)
}
fn execute_sim(
    value: &str,
) -> Result<solana_kit_client::methods::Contextual<SimulationResult>, Error> {
    call(
        simulation(SimulationConfig::default()),
        json!(["AAE=",{"encoding":"base64"}]),
        &contextual(value),
    )
}
#[test]
fn signature_pages_keep_required_positions_and_exact_nullable_history() {
    let result = call(
        TransactionsForAddress(address(), Some(signatures_config())),
        json!([ADDRESS,{"transactionDetails":"signatures"}]),
        &page(signature_entry()),
    )
    .unwrap();
    let AddressTransactions::Signatures(entries) = result.data else {
        panic!("wrong details")
    };
    assert_eq!(entries[0].slot, MAX_SLOT);
    assert_eq!(entries[0].block_time, Some(MIN_TIME));
    assert_eq!(entries[0].transaction_index, Presence::Present(u32::MAX));
    assert_eq!(result.pagination_token, None);

    for field in [
        "transactionIndex",
        "blockTime",
        "confirmationStatus",
        "memo",
        "err",
        "signature",
        "slot",
    ] {
        let mut entry = signature_entry();
        entry.as_object_mut().unwrap().remove(field);
        assert!(
            call(
                TransactionsForAddress(address(), Some(signatures_config())),
                json!([ADDRESS,{"transactionDetails":"signatures"}]),
                &page(entry)
            )
            .is_err(),
            "{field}"
        );
    }
}
fn full_filter_config() -> AddressHistoryConfig {
    let signature = SignatureBound(SIGNATURE.parse().unwrap());
    AddressHistoryConfig {
        commitment: Some(HistoricalCommitment::Confirmed),
        limit: Some(1000),
        min_context_slot: Some(MAX_SLOT),
        pagination_token: Some(PaginationToken {
            slot: MAX_SLOT,
            position: MAX_SLOT,
        }),
        sort_order: Some(SortOrder::Asc),
        details: Some(AddressDetails::Signatures),
        filters: Some(AddressFilters {
            slot: Some(Comparison {
                gt: Some(0),
                gte: Some(1),
                lt: Some(MAX_SLOT),
                lte: Some(MAX_SLOT),
            }),
            signature: Some(Comparison {
                gt: Some(signature),
                gte: Some(signature),
                lt: Some(signature),
                lte: Some(signature),
            }),
            block_time: Some(TimeComparison {
                eq: Some(MIN_TIME),
                range: Comparison {
                    gt: Some(MIN_TIME),
                    gte: Some(0),
                    lt: Some(i64::MAX),
                    lte: Some(i64::MAX),
                },
            }),
            status: Some(ExecutionFilter::Failed),
            token_accounts: Some(TokenAccountsFilter::BalanceChanged),
        }),
    }
}
#[test]
fn every_address_filter_and_cursor_is_serialized_without_integer_loss() {
    let config = full_filter_config();
    let params = json!([ADDRESS,{"commitment":"confirmed","limit":1000,"minContextSlot":MAX_SLOT,
      "paginationToken":"18446744073709551615:18446744073709551615",
      "sortOrder":"asc","transactionDetails":"signatures",
      "filters":{"slot":{"gt":0,"gte":1,"lt":MAX_SLOT,"lte":MAX_SLOT},
       "signature":{"gt":SIGNATURE,"gte":SIGNATURE,"lt":SIGNATURE,"lte":SIGNATURE},
       "blockTime":{"eq":MIN_TIME,"gt":MIN_TIME,"gte":0,"lt":i64::MAX,"lte":i64::MAX},
       "status":"failed","tokenAccounts":"balanceChanged"}}]);
    let result = call(
        TransactionsForAddress(address(), Some(config)),
        params,
        r#"{"data":[],"paginationToken":"0:0"}"#,
    )
    .unwrap();
    assert_eq!(
        result.pagination_token,
        Some(PaginationToken {
            slot: 0,
            position: 0
        })
    );
}
#[test]
// Cross each representation with both numbered versions to catch dispatch drift.
fn address_pages_decode_all_encodings_through_the_shared_transaction_graph() {
    let raw = json!({"signatures":[SIGNATURE],"message":{"header":{"numRequiredSignatures":1,
      "numReadonlySignedAccounts":0,"numReadonlyUnsignedAccounts":0},"accountKeys":[ADDRESS],
      "recentBlockhash":ADDRESS,"instructions":[],"addressTableLookups":[]}});
    let parsed = json!({"signatures":[SIGNATURE],"message":{"accountKeys":[{"pubkey":ADDRESS,
      "writable":true,"signer":true}],"recentBlockhash":ADDRESS,
      "instructions":[],"addressTableLookups":[]}});

    for (encoding, tag, tx) in [
        (
            TransactionEncoding::Base58,
            "base58",
            json!(["12", "base58"]),
        ),
        (
            TransactionEncoding::Base64,
            "base64",
            json!(["AAE=", "base64"]),
        ),
        (TransactionEncoding::Json, "json", raw),
        (TransactionEncoding::JsonParsed, "jsonParsed", parsed),
    ] {
        for version in [0, 1] {
            let entry = json!({"slot":MAX_SLOT,"blockTime":null,
                "transactionIndex":0,"transaction":tx,"meta":null,"version":version});
            let config = AddressHistoryConfig {
                details: Some(AddressDetails::Full(FullOptions {
                    encoding: Some(encoding),
                    max_supported_transaction_version: Some(1),
                })),
                ..Default::default()
            };
            let params = json!([ADDRESS,{"transactionDetails":"full",
                "encoding":tag,"maxSupportedTransactionVersion":1}]);
            let result = call(
                TransactionsForAddress(address(), Some(config)),
                params,
                &page(entry),
            )
            .unwrap();
            let AddressTransactions::Full(entries) = result.data else {
                panic!("wrong details")
            };
            assert_eq!(entries[0].slot, MAX_SLOT);
            let Presence::Present(TransactionVersion::Number(actual)) = entries[0].details.version
            else {
                panic!("wrong version")
            };
            assert_eq!(actual, version);
        }
    }
}

#[test]
fn address_defaults_limits_and_malformed_cursors_are_checked() {
    assert!(
        call(
            TransactionsForAddress(address(), None),
            json!([ADDRESS]),
            r#"{"data":[],"paginationToken":null}"#
        )
        .is_ok()
    );
    assert!(
        call(
            TransactionsForAddress(address(), Some(Default::default())),
            json!([ADDRESS, {}]),
            r#"{"data":[],"paginationToken":null}"#
        )
        .is_ok()
    );

    for details in [None, Some(AddressDetails::Signatures)] {
        for limit in [0, 1001] {
            let method = TransactionsForAddress(
                address(),
                Some(AddressHistoryConfig {
                    details,
                    limit: Some(limit),
                    ..Default::default()
                }),
            );
            assert!(serde_json::to_string(&method).is_err());
        }
    }
}
#[test]
fn malformed_cursors_and_page_fields_are_checked() {
    for cursor in [
        "",
        ":1",
        "1:",
        "-1:0",
        "+1:0",
        "01:0",
        "0:01",
        "1:2:3",
        "18446744073709551616:0",
        "0: 1",
    ] {
        let body = json!({"data":[],"paginationToken":cursor}).to_string();
        assert!(
            call(
                TransactionsForAddress(address(), None),
                json!([ADDRESS]),
                &body
            )
            .is_err(),
            "{cursor}"
        );
    }

    for body in [
        r#"{"data":[]}"#,
        r#"{"data":null,"paginationToken":null}"#,
        r#"{"data":[],"paginationToken":1}"#,
    ] {
        assert!(
            call(
                TransactionsForAddress(address(), None),
                json!([ADDRESS]),
                body
            )
            .is_err()
        );
    }
}
#[test]
fn simulation_full_options_and_replacement_hash_preserve_exact_values() {
    let config = SimulationConfig {
        commitment: Some(Commitment::Processed),
        min_context_slot: Some(MAX_SLOT),
        inner_instructions: Some(false),
        sig_verify: Some(false),
        replace_recent_blockhash: Some(true),
        accounts: None,
    };
    let mut execution: Value = serde_json::from_str(EXECUTION).unwrap();
    execution["replacementBlockhash"] =
        json!({"blockhash":ADDRESS,"lastValidBlockHeight":MAX_SLOT});
    execution["unitsConsumed"] = json!(MAX_SLOT);
    execution["loadedAccountsDataSize"] = json!(u32::MAX);
    let params = json!(["AAE=",{"encoding":"base64","commitment":"processed",
        "minContextSlot":MAX_SLOT,"innerInstructions":false,"sigVerify":false,
        "replaceRecentBlockhash":true}]);
    let result = call(
        simulation(config),
        params,
        &contextual(&execution.to_string()),
    )
    .unwrap();
    assert_eq!(result.context.slot, MAX_SLOT);

    assert_eq!(result.value.execution.fee, Some(MAX_SLOT));
    assert_eq!(
        result.value.execution.units_consumed,
        Presence::Present(Some(MAX_SLOT))
    );
    assert_eq!(
        result
            .value
            .execution
            .replacement_blockhash
            .unwrap()
            .last_valid_block_height,
        MAX_SLOT
    );
}
#[test]
// Every account representation must preserve the same identity/null mapping.
fn simulation_reuses_rich_account_decoding_and_preserves_positional_nulls() {
    for (encoding, tag, data) in [
        (
            SimulationAccountEncoding::Base64,
            "base64",
            json!(["AAE=", "base64"]),
        ),
        (
            SimulationAccountEncoding::Base64Zstd,
            "base64+zstd",
            json!(["AAE=", "base64+zstd"]),
        ),
        (
            SimulationAccountEncoding::JsonParsed,
            "jsonParsed",
            json!({"program":"fixture","space":2,"parsed":{"units":MAX_SLOT}}),
        ),
    ] {
        let mut value: Value = serde_json::from_str(EXECUTION).unwrap();
        value["accounts"] = json!([{"executable":false,"lamports":MAX_SLOT,
            "owner":ADDRESS,"rentEpoch":MAX_SLOT,"space":2,"data":data},null]);
        let config = SimulationConfig {
            accounts: Some(SimulationAccounts {
                addresses: vec![address(), address()],
                encoding: Some(encoding),
            }),
            ..Default::default()
        };
        let params = json!(["AAE=",{"encoding":"base64",
            "accounts":{"addresses":[ADDRESS,ADDRESS],"encoding":tag}}]);
        let result = call(simulation(config), params, &contextual(&value.to_string())).unwrap();
        let accounts = result.value.accounts.unwrap();
        let MaybeAccount::Present(account) = &accounts[0] else {
            panic!("missing account")
        };
        assert_eq!(account.lamports, MAX_SLOT);
        assert_eq!(account.address, address());

        assert!(matches!(accounts[1], MaybeAccount::Missing(identity) if identity == address()));

        check_account_data(&account.data, encoding);
    }
}
fn check_account_data(data: &AccountData, encoding: SimulationAccountEncoding) {
    match (data, encoding) {
        (AccountData::Bytes { bytes, .. }, SimulationAccountEncoding::Base64) => {
            assert_eq!(bytes, &[0, 1])
        }
        (AccountData::Zstd { compressed, .. }, SimulationAccountEncoding::Base64Zstd) => {
            assert_eq!(compressed, &[0, 1])
        }
        (AccountData::Parsed(parsed), SimulationAccountEncoding::JsonParsed) => {
            assert_eq!(parsed.program, "fixture")
        }
        _ => panic!("wrong account encoding"),
    }
}
#[test]
fn simulation_required_nullable_fields_reject_absence_and_scalar_mutations() {
    let original: Value = serde_json::from_str(EXECUTION).unwrap();
    assert!(execute_sim(EXECUTION).is_ok());

    for key in original.as_object().unwrap().keys() {
        let mut value = original.clone();
        value.as_object_mut().unwrap().remove(key);
        assert!(execute_sim(&value.to_string()).is_err(), "missing {key}");
    }

    for (key, invalid) in [
        ("fee", json!(-1)),
        ("fee", json!("1")),
        ("fee", json!({"$serde_json::private::Number":"1"})),
        ("unitsConsumed", json!(-1)),
        ("unitsConsumed", json!({"$serde_json::private::Number":"1"})),
        ("loadedAccountsDataSize", json!(MAX_SLOT)),
        (
            "loadedAccountsDataSize",
            json!({"$serde_json::private::Number":"1"}),
        ),
        ("logs", json!([1])),
        ("preBalances", json!([-1])),
        ("loadedAddresses", json!({})),
        (
            "returnData",
            json!({"programId":ADDRESS,"data":["AB==","base64"]}),
        ),
    ] {
        let mut value = original.clone();
        value[key] = invalid;
        assert!(execute_sim(&value.to_string()).is_err(), "invalid {key}");
    }
}
#[test]
fn simulation_conflicts_cardinality_and_account_allocation_cannot_succeed() {
    let config = SimulationConfig {
        sig_verify: Some(true),
        replace_recent_blockhash: Some(true),
        ..Default::default()
    };
    assert!(serde_json::to_string(&simulation(config)).is_err());
    let mut value: Value = serde_json::from_str(EXECUTION).unwrap();
    value["accounts"] = json!([]);
    assert!(execute_sim(&value.to_string()).is_err());

    for accounts in [
        json!([]),
        json!([{"executable":false,"lamports":0,"owner":ADDRESS,
            "rentEpoch":0,"space":3,"data":["AAE=","base64"]}]),
    ] {
        value["accounts"] = accounts;
        let config = SimulationConfig {
            accounts: Some(SimulationAccounts {
                addresses: vec![address()],
                encoding: None,
            }),
            ..Default::default()
        };
        assert!(
            call(
                simulation(config),
                json!(["AAE=",{"encoding":"base64","accounts":{"addresses":[ADDRESS]}}]),
                &contextual(&value.to_string())
            )
            .is_err()
        );
    }
}
#[test]
fn simulation_base58_does_not_inherit_legacy_packet_size() {
    let wire = vec![0u8; OVERSIZED_WIRE];
    let method = SimulateTransaction {
        wire: &wire,
        encoding: WireEncoding::Base58,
        config: Default::default(),
    };
    let result = call(
        method,
        json!(["1".repeat(OVERSIZED_WIRE),{"encoding":"base58"}]),
        &contextual(EXECUTION),
    )
    .unwrap();
    assert_eq!(result.value.execution.fee, Some(MAX_SLOT));
}
#[test]
fn simulation_inner_instruction_routing_and_exact_program_payload_reuse() {
    let value = EXECUTION.strip_suffix('}').unwrap().to_string()
        + &format!(
            r#", "innerInstructions":[{{"index":0,"instructions":[
            {{"program":"fixture","programId":"{ADDRESS}","parsed":{{ "units":1e400 }}}},
            {{"programId":"{ADDRESS}","accounts":[],"data":"12"}}]}}]}}"#
        );
    let config = SimulationConfig {
        inner_instructions: Some(true),
        ..Default::default()
    };
    let result = call(
        simulation(config),
        json!(["AAE=",{"encoding":"base64","innerInstructions":true}]),
        &contextual(&value),
    )
    .unwrap();
    let Presence::Present(Some(groups)) = result.value.execution.inner_instructions else {
        panic!("missing inner")
    };
    let InnerInstruction::Resolved(ParsedOrPartialInstruction::Parsed(parsed)) =
        &groups[0].instructions[0]
    else {
        panic!("wrong inner")
    };

    assert_eq!(parsed.parsed.get(), r#"{ "units":1e400 }"#);
    assert!(execute_sim(&value).is_err());
}

#[test]
fn both_methods_keep_correlation_duplicate_and_body_budget_guards() {
    assert_shared_boundary(
        || TransactionsForAddress(address(), None),
        json!([ADDRESS]),
        r#"{"data":[],"paginationToken":null}"#,
    );
    assert_shared_boundary(
        || simulation(Default::default()),
        json!(["AAE=",{"encoding":"base64"}]),
        &contextual(EXECUTION),
    );

    let duplicate = r#"{"data":[],"paginationToken":null,"extra":{"key":1,"k\u0065y":2}}"#;
    assert!(
        call(
            TransactionsForAddress(address(), None),
            json!([ADDRESS]),
            duplicate
        )
        .is_err()
    );
    let duplicate =
        EXECUTION.strip_suffix('}').unwrap().to_string() + r#", "extra":{"key":1,"k\u0065y":2}}"#;
    assert!(execute_sim(&duplicate).is_err());
}
fn assert_shared_boundary<M: Method>(method: impl Fn() -> M, params: Value, result: &str) {
    let transport = Fixture {
        name: M::NAME,
        params: params.clone(),
        result: result.into(),
        id: 2,
    };
    assert!(block_on(Client::new(transport, Limits::default()).call(method())).is_err());
    let transport = Fixture {
        name: M::NAME,
        params,
        result: result.into(),
        id: 1,
    };
    assert!(
        block_on(
            Client::new(
                transport,
                Limits {
                    bytes: 1,
                    depth: 32
                }
            )
            .call(method())
        )
        .is_err()
    );
}
#[test]
fn simulation_nullable_versioned_fields_and_failed_execution_remain_typed() {
    let mut value: Value = serde_json::from_str(EXECUTION).unwrap();
    value["unitsConsumed"] = Value::Null;
    value["loadedAccountsDataSize"] = Value::Null;
    value["innerInstructions"] = Value::Null;
    value["err"] = json!({"InstructionError":[255,{"Custom":u32::MAX}]});
    let result = execute_sim(&value.to_string()).unwrap();
    assert_eq!(
        result.value.execution.units_consumed,
        Presence::Present(None)
    );
    assert!(result.value.execution.err.is_some());
}
#[test]
fn simulation_account_fallback_and_metadata_mutations_reuse_shared_validation() {
    let config = SimulationConfig {
        accounts: Some(SimulationAccounts {
            addresses: vec![address()],
            encoding: Some(SimulationAccountEncoding::JsonParsed),
        }),
        ..Default::default()
    };
    let params = json!(["AAE=",{"encoding":"base64","accounts":{"addresses":[ADDRESS],"encoding":"jsonParsed"}}]);
    let mut value: Value = serde_json::from_str(EXECUTION).unwrap();
    value["accounts"] = json!([{"executable":true,"lamports":MAX_SLOT,"owner":ADDRESS,
        "rentEpoch":MAX_SLOT,"space":2,"data":["AAE=","base64"]}]);
    assert!(
        call(
            simulation(config.clone()),
            params.clone(),
            &contextual(&value.to_string())
        )
        .is_ok()
    );

    for field in [
        "executable",
        "lamports",
        "owner",
        "rentEpoch",
        "space",
        "data",
    ] {
        let mut malformed = value.clone();
        malformed["accounts"][0]
            .as_object_mut()
            .unwrap()
            .remove(field);
        assert!(
            call(
                simulation(config.clone()),
                params.clone(),
                &contextual(&malformed.to_string())
            )
            .is_err()
        );
    }
}
#[test]
fn full_address_entries_require_each_baseline_field_and_request_version() {
    let config = AddressHistoryConfig {
        details: Some(AddressDetails::Full(FullOptions {
            encoding: Some(TransactionEncoding::Base64),
            max_supported_transaction_version: Some(1),
        })),
        ..Default::default()
    };
    let params = json!([ADDRESS,{"transactionDetails":"full","encoding":"base64","maxSupportedTransactionVersion":1}]);
    let entry = json!({"slot":MAX_SLOT,"blockTime":null,"transactionIndex":u32::MAX,
        "transaction":["AAE=","base64"],"meta":null,"version":"legacy"});
    assert!(
        call(
            TransactionsForAddress(address(), Some(config.clone())),
            params.clone(),
            &page(entry.clone())
        )
        .is_ok()
    );

    for field in [
        "slot",
        "blockTime",
        "transactionIndex",
        "transaction",
        "meta",
        "version",
    ] {
        let mut malformed = entry.clone();
        malformed.as_object_mut().unwrap().remove(field);
        assert!(
            call(
                TransactionsForAddress(address(), Some(config.clone())),
                params.clone(),
                &page(malformed)
            )
            .is_err()
        );
    }

    for (field, invalid) in [
        ("slot", json!(-1)),
        ("transactionIndex", Value::Null),
        ("transactionIndex", json!(MAX_SLOT)),
        ("blockTime", json!(MAX_SLOT)),
        ("version", json!(2)),
        ("transaction", json!(["AB==", "base64"])),
    ] {
        let mut malformed = entry.clone();
        malformed[field] = invalid;
        assert!(
            call(
                TransactionsForAddress(address(), Some(config.clone())),
                params.clone(),
                &page(malformed)
            )
            .is_err()
        );
    }
}

#[test]
fn exact_detail_caps_and_response_cardinality_are_enforced() {
    for (details, limit, valid) in [
        (None, 100, true),
        (None, 101, true),
        (None, 1000, true),
        (None, 1001, false),
        (
            Some(AddressDetails::Full(FullOptions::default())),
            100,
            true,
        ),
        (
            Some(AddressDetails::Full(FullOptions::default())),
            101,
            false,
        ),
        (Some(AddressDetails::Signatures), 1000, true),
        (Some(AddressDetails::Signatures), 1001, false),
    ] {
        let method = TransactionsForAddress(
            address(),
            Some(AddressHistoryConfig {
                details,
                limit: Some(limit),
                ..Default::default()
            }),
        );
        assert_eq!(serde_json::to_string(&method).is_ok(), valid);
    }
    let config = AddressHistoryConfig {
        limit: Some(1),
        ..signatures_config()
    };
    let result = json!({"data":[signature_entry(),signature_entry()],"paginationToken":null});
    assert!(
        call(
            TransactionsForAddress(address(), Some(config)),
            json!([ADDRESS,{"transactionDetails":"signatures","limit":1}]),
            &result.to_string()
        )
        .is_err()
    );
}

#[test]
fn simulation_token_display_accepts_only_actual_numeric_scalars() {
    let mut value: Value = serde_json::from_str(EXECUTION).unwrap();
    value["postTokenBalances"] = json!([{"accountIndex":0,"mint":ADDRESS,
        "uiTokenAmount":{"amount":"1","decimals":0,"uiAmount":1,"uiAmountString":"1"}}]);
    assert!(execute_sim(&value.to_string()).is_ok());
    value["postTokenBalances"][0]["uiTokenAmount"]["uiAmount"] =
        json!({"$serde_json::private::Number":"1"});
    assert!(execute_sim(&value.to_string()).is_err());
    value["postTokenBalances"][0]["uiTokenAmount"]["uiAmount"] = json!({"unexpected":1});
    assert!(execute_sim(&value.to_string()).is_err());
}
