//! Malformed inputs must fail before any trace is exposed.

use super::fixtures::*;
use serde_json::json;
use solana_kit_client::introspection::{Error, decode_rpc_transaction};
use solana_kit_client::methods::transaction_results::TransactionEncoding;

const EXCESS_SIGNERS: u8 = 5;
const OUT_OF_RANGE_INDEX: u8 = 7;

#[test]
fn rejects_header_signature_and_instruction_reference_mismatches() {
    let mutations = [
        ("numRequiredSignatures", EXCESS_SIGNERS),
        ("numReadonlySignedAccounts", 3),
        ("numReadonlySignedAccounts", 2),
        ("numReadonlyUnsignedAccounts", 3),
    ];

    for (field, value) in mutations {
        let mut value_json = raw();
        value_json["transaction"]["message"]["header"][field] = json!(value);
        assert_eq!(
            decode_rpc_transaction(&parse(value_json, TransactionEncoding::Json)).unwrap_err(),
            Error::Header
        );
    }

    let mut value = raw();
    value["transaction"]["signatures"] = json!([]);
    assert_eq!(
        decode_rpc_transaction(&parse(value, TransactionEncoding::Json)).unwrap_err(),
        Error::Signatures
    );

    for field in ["accounts", "programIdIndex"] {
        let mut value = raw();
        value["transaction"]["message"]["instructions"][0][field] = if field == "accounts" {
            json!([OUT_OF_RANGE_INDEX])
        } else {
            json!(OUT_OF_RANGE_INDEX)
        };

        assert_eq!(
            decode_rpc_transaction(&parse(value, TransactionEncoding::Json)).unwrap_err(),
            Error::Index
        );
    }
}

#[test]
fn rejects_missing_null_partial_extra_and_swapped_loaded_partitions() {
    for loaded in [
        json!(null),
        json!({"writable":[],"readonly":[address(LOADED_READONLY).to_string()]}),
        json!({"writable":[address(LOADED_WRITABLE).to_string(),
            address(LOADED_READONLY).to_string()],"readonly":[]}),
    ] {
        let mut value = with_lookup();
        value["meta"]["loadedAddresses"] = loaded;
        assert!(decode_rpc_transaction(&parse(value, TransactionEncoding::Json)).is_err());
    }

    let mut value = with_lookup();
    value["meta"]
        .as_object_mut()
        .unwrap()
        .remove("loadedAddresses");
    assert_eq!(
        decode_rpc_transaction(&parse(value, TransactionEncoding::Json)).unwrap_err(),
        Error::MissingLoadedAddresses
    );
    let mut value = with_lookup();
    value["version"] = json!("legacy");
    assert_eq!(
        decode_rpc_transaction(&parse(value, TransactionEncoding::Json)).unwrap_err(),
        Error::LoadedAddresses
    );
}

#[test]
fn rejects_duplicate_empty_and_orphan_inner_groups() {
    for groups in [
        json!([{"index":0,"instructions":[]},{"index":0,"instructions":[]}]),
        json!([{"index":2,"instructions":[]}]),
    ] {
        let mut value = raw();
        value["meta"]["innerInstructions"] = groups;
        assert_eq!(
            decode_rpc_transaction(&parse(value, TransactionEncoding::Json)).unwrap_err(),
            Error::InnerGroup
        );
    }
}
