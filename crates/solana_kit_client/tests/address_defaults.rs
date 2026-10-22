//! Pinned address-history defaults and boundaries through the public Client.
use futures::executor::block_on;
use serde_json::{Value, json};
use solana_kit_client::methods::{Method, address_history::*};
use solana_kit_client::{Client, Error, Limits, Reply, Request, Transport, TransportError};
use solana_kit_core::{Address, Signature};
use std::future::{Ready, ready};

const BODY_BUDGET: usize = 2_000_000;
const FULL_CAP: u16 = 100;
const SIGNATURE_CAP: u16 = 1000;
const COUNTS: [usize; 6] = [0, 1, 100, 101, 1000, 1001];

struct Fixture {
    params: Value,
    entries: Vec<Value>,
}
impl Transport for Fixture {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let actual: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(
            actual,
            json!({"jsonrpc":"2.0","id":1,"method":TransactionsForAddress::NAME,
                "params":self.params})
        );
        ready(Ok(Reply {
            status: 200,
            body: serde_json::to_vec(&json!({"jsonrpc":"2.0","id":1,
                "result":{"data":self.entries,"paginationToken":null}}))
            .unwrap(),
        }))
    }
}
fn address() -> Address {
    Address::from_bytes([37; 32])
}
fn signature() -> String {
    Signature::from_bytes([95; 64]).to_string()
}
fn signature_entry() -> Value {
    json!({"signature":signature(),"slot":u64::MAX,"blockTime":null,
        "transactionIndex":u32::MAX,"confirmationStatus":"finalized","memo":null,"err":null})
}
fn full_entry() -> Value {
    json!({"slot":u64::MAX,"blockTime":null,"transactionIndex":u32::MAX,"meta":null,
        "transaction":{"signatures":[signature()],"message":{
            "header":{"numRequiredSignatures":1,"numReadonlySignedAccounts":0,
                "numReadonlyUnsignedAccounts":0},"accountKeys":[address().to_string()],
            "recentBlockhash":address().to_string(),"instructions":[]}}})
}
fn execute(
    config: Option<AddressHistoryConfig>,
    entries: Vec<Value>,
) -> Result<AddressPage, Error> {
    let mut params = json!([address().to_string()]);

    if let Some(config) = &config {
        params
            .as_array_mut()
            .unwrap()
            .push(serde_json::to_value(config).unwrap());
    }

    block_on(
        Client::new(
            Fixture { params, entries },
            Limits {
                bytes: BODY_BUDGET,
                depth: 64,
            },
        )
        .call(TransactionsForAddress(address(), config)),
    )
}
fn details() -> [(Option<AddressDetails>, u16); 3] {
    [
        (None, SIGNATURE_CAP),
        (Some(AddressDetails::Signatures), SIGNATURE_CAP),
        (Some(AddressDetails::Full(FullOptions::default())), FULL_CAP),
    ]
}
fn assert_count(page: AddressPage, full: bool, count: usize) {
    let (actual_full, actual) = match page.data {
        AddressTransactions::Signatures(entries) => (false, entries.len()),
        AddressTransactions::Full(entries) => (true, entries.len()),
    };

    assert_eq!(actual_full, full);
    assert_eq!(actual, count);
    assert_eq!(page.pagination_token, None);
}

#[test]
fn omitted_configs_and_details_decode_populated_signature_pages_without_serialization_defaults() {
    assert_eq!(
        serde_json::to_value(TransactionsForAddress(address(), None)).unwrap(),
        json!([address().to_string()])
    );
    assert_eq!(
        serde_json::to_value(TransactionsForAddress(address(), Some(Default::default()))).unwrap(),
        json!([address().to_string(), {}])
    );

    for config in [
        None,
        Some(Default::default()),
        Some(AddressHistoryConfig {
            details: Some(AddressDetails::Signatures),
            ..Default::default()
        }),
    ] {
        assert_count(execute(config, vec![signature_entry()]).unwrap(), false, 1);
    }
}

#[test]
fn selected_details_keep_request_limit_boundaries_and_literal_omissions() {
    for (details, cap) in details() {
        for limit in [0, 1, 100, 101, 1000, 1001] {
            assert_request(details, cap, limit);
        }
    }
}
fn assert_request(details: Option<AddressDetails>, cap: u16, limit: u16) {
    let config = AddressHistoryConfig {
        details,
        limit: Some(limit),
        ..Default::default()
    };
    let encoded = serde_json::to_value(TransactionsForAddress(address(), Some(config.clone())));
    let valid = limit > 0 && limit <= cap;
    assert_eq!(encoded.is_ok(), valid, "{details:?}/{limit}");

    if !valid {
        return;
    }

    let full = matches!(details, Some(AddressDetails::Full(_)));
    let mut expected = json!([address().to_string(),{"limit":limit}]);

    if details.is_some() {
        expected[1]["transactionDetails"] = json!(detail_name(full));
    }

    assert_eq!(encoded.unwrap(), expected);
    assert_count(execute(Some(config), vec![entry(full)]).unwrap(), full, 1);
}
fn detail_name(full: bool) -> &'static str {
    if full { "full" } else { "signatures" }
}
fn entry(full: bool) -> Value {
    if full {
        full_entry()
    } else {
        signature_entry()
    }
}

#[test]
fn populated_results_obey_default_caps_and_requested_limits_for_each_detail_mode() {
    for (details, cap) in details() {
        let full = matches!(details, Some(AddressDetails::Full(_)));

        for limit in [None, Some(1), Some(FULL_CAP), Some(cap)] {
            let config = AddressHistoryConfig {
                details,
                limit,
                ..Default::default()
            };
            let maximum = usize::from(limit.unwrap_or(cap));
            assert_results(Some(config), full, maximum);
        }
    }

    assert_results(None, false, usize::from(SIGNATURE_CAP));
    assert_results(
        Some(AddressHistoryConfig {
            limit: Some(FULL_CAP + 1),
            ..Default::default()
        }),
        false,
        usize::from(FULL_CAP + 1),
    );
}
fn assert_results(config: Option<AddressHistoryConfig>, full: bool, maximum: usize) {
    for count in COUNTS {
        let result = execute(config.clone(), vec![entry(full); count]);
        assert_eq!(result.is_ok(), count <= maximum, "{config:?}/{count}");

        if let Ok(page) = result {
            assert_count(page, full, count);
        }
    }
}

#[test]
fn default_and_explicit_full_never_accept_the_other_response_shape() {
    for config in [
        None,
        Some(Default::default()),
        Some(AddressHistoryConfig {
            details: Some(AddressDetails::Signatures),
            ..Default::default()
        }),
    ] {
        assert!(execute(config, vec![full_entry()]).is_err());
    }

    let config = AddressHistoryConfig {
        details: Some(AddressDetails::Full(FullOptions::default())),
        ..Default::default()
    };
    assert!(execute(Some(config), vec![signature_entry()]).is_err());
}
