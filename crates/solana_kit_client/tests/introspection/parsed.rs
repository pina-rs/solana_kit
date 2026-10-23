//! Server parsing remains exact and partial instructions resolve ordinary Kit roles.

use super::fixtures::*;
use serde_json::{Value, json};
use solana_kit_client::introspection::{Error, ResolvedInstruction, decode_rpc_transaction};
use solana_kit_client::methods::transaction_results::TransactionEncoding;
use solana_kit_core::AccountRole;

fn parsed() -> Value {
    let mut value = with_lookup();
    value["transaction"]["message"]
        .as_object_mut()
        .unwrap()
        .remove("header");
    value["transaction"]["message"]["accountKeys"] = json!([
        {"pubkey":address(1).to_string(),"signer":true,"writable":true,"source":"transaction"},
        {"pubkey":address(2).to_string(),"signer":true,"writable":false,"source":"transaction"},
        {"pubkey":address(3).to_string(),"signer":false,"writable":true,"source":"transaction"},
        {"pubkey":address(4).to_string(),"signer":false,"writable":false,"source":"transaction"},
        {"pubkey":address(LOADED_WRITABLE).to_string(),
            "signer":false,"writable":true,"source":"lookupTable"},
        {"pubkey":address(LOADED_READONLY).to_string(),
            "signer":false,"writable":false,"source":"lookupTable"}
    ]);
    value["transaction"]["message"]["instructions"] = json!([
        {"program":"unknown-program","programId":address(4).to_string(),
            "parsed":{"amount":u64::MAX,"custom":[null,true]}},
        {"programId":address(4).to_string(),"accounts":[address(LOADED_READONLY).to_string(),
            address(LOADED_WRITABLE).to_string(),address(LOADED_READONLY).to_string()],"data":"19p"}
    ]);
    value["meta"]
        .as_object_mut()
        .unwrap()
        .remove("loadedAddresses");
    value
}

#[test]
fn parsed_payload_is_borrowed_exactly_and_partial_data_retains_loaded_roles() {
    let source = parse(parsed(), TransactionEncoding::JsonParsed);
    let decoded = decode_rpc_transaction(&source).unwrap();
    let instructions = decoded.get_instructions().unwrap();
    let ResolvedInstruction::Parsed(parsed) = &instructions[0].instruction else {
        panic!("parsed instruction");
    };
    assert_eq!(
        parsed.parsed.get(),
        r#"{"amount":18446744073709551615,"custom":[null,true]}"#
    );
    let ResolvedInstruction::Raw(partial) = &instructions[1].instruction else {
        panic!("partial instruction");
    };

    assert_eq!(partial.data, [0, 1, 255]);
    assert_eq!(
        partial
            .accounts
            .iter()
            .map(|account| account.role)
            .collect::<Vec<_>>(),
        [
            AccountRole::Readonly,
            AccountRole::Writable,
            AccountRole::Readonly
        ]
    );
    assert!(decoded.transaction().is_none());
}

#[test]
fn parsed_inner_calls_walk_after_their_outer_call() {
    let mut value = parsed();
    value["meta"]["innerInstructions"] = json!([{"index":0,"instructions":[
        {"programId":address(4).to_string(),"accounts":[address(1).to_string()],
            "data":"2","stackHeight":2}
    ]}]);
    let source = parse(value, TransactionEncoding::JsonParsed);
    assert_eq!(
        decode_rpc_transaction(&source)
            .unwrap()
            .walk_instructions()
            .unwrap()
            .len(),
        3
    );
}

#[test]
fn parsed_addresses_signers_sources_and_opaque_payload_shapes_are_checked() {
    for field in ["signer", "writable"] {
        let mut value = parsed();
        value["transaction"]["message"]["accountKeys"][5][field] = json!(true);
        assert!(decode_rpc_transaction(&parse(value, TransactionEncoding::JsonParsed)).is_err());
    }

    let mut value = parsed();
    value["transaction"]["message"]["instructions"][0]["parsed"] = json!(null);
    assert!(decode_rpc_transaction(&parse(value, TransactionEncoding::JsonParsed)).is_err());
    let mut value = parsed();
    value["transaction"]["message"]["instructions"][1]["accounts"] =
        json!([address(99).to_string()]);
    assert_eq!(
        decode_rpc_transaction(&parse(value, TransactionEncoding::JsonParsed)).unwrap_err(),
        Error::Address
    );
}
