extern crate std;
use std::prelude::v1::*;
use std::vec;
// Independent RPC JSON and transaction wire literals.
use serde_json::{Value, json};
use solana_kit_client::methods::Method;
use solana_kit_client::methods::transaction_results::{self as rpc, TransactionEncoding};
use solana_kit_core::{Address, Signature};

pub const LOADED_WRITABLE: u8 = 5;
pub const LOADED_READONLY: u8 = 6;
const TABLE_WRITABLE_INDEX: u8 = 9;
const TABLE_READONLY_INDEX: u8 = 11;

pub fn address(byte: u8) -> Address {
    Address::from_bytes([byte; 32])
}

pub fn meta() -> Value {
    json!({"err":null,"status":{"Ok":null},"fee":u64::MAX,
        "preBalances":[u64::MAX],"postBalances":[0]})
}

pub fn raw() -> Value {
    json!({"slot":u64::MAX,"blockTime":null,"version":"legacy","meta":meta(),
        "transaction":{"signatures":[Signature::from_bytes([7;64]).to_string(), Signature::from_bytes([8;64]).to_string()],
        "message":{"header":{"numRequiredSignatures":2,"numReadonlySignedAccounts":1,"numReadonlyUnsignedAccounts":1},
        "accountKeys":[address(1).to_string(),address(2).to_string(),address(3).to_string(),address(4).to_string()],
        "recentBlockhash":Address::from_bytes([9;32]).to_string(),
        "instructions":[{"programIdIndex":3,"accounts":[2,0,2],"data":"19p"},
            {"programIdIndex":3,"accounts":[],"data":""}]}}})
}

pub fn with_lookup() -> Value {
    let mut value = raw();
    value["version"] = json!(0);
    value["transaction"]["message"]["addressTableLookups"] = json!([
            {"accountKey":address(10).to_string(),
             "writableIndexes":[TABLE_WRITABLE_INDEX],"readonlyIndexes":[TABLE_READONLY_INDEX]}
    ]);
    value["transaction"]["message"]["instructions"][0]["accounts"] = json!([5, 4, 5]);
    value["meta"]["loadedAddresses"] = json!({"writable":[address(LOADED_WRITABLE).to_string()],
            "readonly":[address(LOADED_READONLY).to_string()]});
    value
}

pub fn parse(value: Value, encoding: TransactionEncoding) -> rpc::TransactionResult {
    rpc::Transaction(
        Signature::from_bytes([7; 64]),
        Some(rpc::TransactionReadConfig {
            encoding: Some(encoding),
            max_supported_transaction_version: Some(1),
            ..Default::default()
        }),
    )
    .decode(&value.to_string())
    .unwrap()
    .unwrap()
    .details
}

pub fn literal_wire(version: u8) -> Vec<u8> {
    match version {
        1 => literal_v1(),
        0 => literal_legacy(true),
        _ => literal_legacy(false),
    }
}

fn literal_v1() -> Vec<u8> {
    let mut message = vec![0x81, 1, 0, 1, 0, 0, 0, 0];
    message.extend([9; 32]);
    message.extend([1, 2]);
    message.extend([1; 32]);
    message.extend([4; 32]);
    message.extend([1, 2, 3, 0, 0, 0, 0, 1, 255]);
    message.extend([7; 64]);

    message
}

fn literal_legacy(versioned: bool) -> Vec<u8> {
    let mut message = Vec::new();

    if versioned {
        message.push(0x80);
    }

    message.extend([1, 0, 1, 2]);
    message.extend([1; 32]);
    message.extend([4; 32]);
    message.extend([9; 32]);
    message.extend([1, 1, 2, 0, 0, 3, 0, 1, 255]);

    if versioned {
        message.push(0);
    }

    let mut transaction = vec![1];
    transaction.extend([7; 64]);
    transaction.extend(message);
    transaction
}
