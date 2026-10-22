extern crate std;
use std::prelude::v1::*;
use std::vec;
// Independent public-consumer controls for transaction introspection.
use super::fixtures::*;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};
use solana_kit_client::Presence;
use solana_kit_client::introspection::{
    Error, InstructionTrace, ResolvedInstruction, decode_rpc_transaction, decode_rpc_wire,
};
use solana_kit_client::methods::transaction_results::{TransactionEncoding, TransactionView};
use solana_kit_core::{Signature, transaction::Lifetime};

fn lookup_wire() -> Vec<u8> {
    let mut bytes = literal_wire(0);
    bytes.pop();
    bytes.extend([1]);
    bytes.extend([10; 32]);
    bytes.extend([1, 9, 1, 11]);
    bytes
}

fn parsed() -> Value {
    let mut value = with_lookup();
    let message = &mut value["transaction"]["message"];
    message.as_object_mut().unwrap().remove("header");
    message["accountKeys"] = json!([
        {"pubkey":address(1).to_string(),"signer":true,"writable":true,"source":"transaction"},
        {"pubkey":address(2).to_string(),"signer":true,"writable":false,"source":"transaction"},
        {"pubkey":address(3).to_string(),"signer":false,"writable":true,"source":"transaction"},
        {"pubkey":address(4).to_string(),"signer":false,"writable":false,"source":"transaction"},
        {"pubkey":address(LOADED_WRITABLE).to_string(),"signer":false,"writable":true,"source":"lookupTable"},
        {"pubkey":address(LOADED_READONLY).to_string(),"signer":false,"writable":false,"source":"lookupTable"}
    ]);
    message["instructions"] = json!([
        {"program":"opaque","programId":address(4).to_string(),
            "parsed":{"huge":u64::MAX},"stackHeight":1},
        {"programId":address(4).to_string(),"accounts":[address(LOADED_WRITABLE).to_string()],
            "data":"2","stackHeight":null}
    ]);
    value
}

pub(super) fn wire_only_preserves_signed_v0_under_missing_null_empty_and_contradictory_metadata()
-> Result<(), &'static str> {
    let bytes = lookup_wire();
    let cases = [
        Value::Null,
        meta(),
        {
            let mut metadata = meta();
            metadata["loadedAddresses"] = Value::Null;
            metadata
        },
        {
            let mut metadata = meta();
            metadata["loadedAddresses"] = json!({"writable":[],"readonly":[]});
            metadata
        },
    ];

    for (index, metadata) in cases.into_iter().enumerate() {
        let mut value = with_lookup();
        value["transaction"] = json!([STANDARD.encode(&bytes), "base64"]);
        value["meta"] = metadata;
        let source = parse(value, TransactionEncoding::Base64);
        let exact = decode_rpc_wire(&source).unwrap();
        check_eq!(exact.encode(false).unwrap(), bytes);
        check_eq!(exact.message().bytes(), &bytes[65..]);

        check_eq!(exact.signatures(), [Some(Signature::from_bytes([7; 64]))]);
        check!(matches!(
            exact.message().lifetime(),
            Lifetime::Unresolved(_)
        ));
        check_eq!(exact.message().lookups()[0].writable_indices(), [9]);
        check_eq!(exact.message().lookups()[0].readonly_indices(), [11]);
        let expected = if index == 3 {
            Error::LoadedAddresses
        } else {
            Error::MissingLoadedAddresses
        };

        check_eq!(decode_rpc_transaction(&source).unwrap_err(), expected);
    }

    Ok(())
}

pub(super) fn wire_only_rejects_all_reported_version_mismatches_and_trailing_bytes()
-> Result<(), &'static str> {
    for actual in [255, 0, 1] {
        for reported in [json!("legacy"), json!(0), json!(1)] {
            check_reported_version(actual, reported)?;
        }

        let mut bytes = literal_wire(actual);
        bytes.push(0);
        let mut value = raw();

        value["version"] = match actual {
            255 => json!("legacy"),
            other => json!(other),
        };

        value["transaction"] = json!([STANDARD.encode(bytes), "base64"]);
        check!(matches!(
            decode_rpc_wire(&parse(value, TransactionEncoding::Base64)),
            Err(Error::Wire(_))
        ));
    }

    Ok(())
}

pub(super) fn check_reported_version(actual: u8, reported: Value) -> Result<(), &'static str> {
    let correct = match actual {
        255 => json!("legacy"),
        other => json!(other),
    };

    let mut value = raw();
    value["version"] = reported.clone();
    value["transaction"] = json!([STANDARD.encode(literal_wire(actual)), "base64"]);
    let source = parse(value, TransactionEncoding::Base64);

    if reported == correct {
        check!(decode_rpc_wire(&source).is_ok());

        return Ok(());
    }

    check_eq!(decode_rpc_wire(&source).unwrap_err(), Error::Version);
    Ok(())
}

pub(super) fn duplicate_and_empty_lookup_descriptors_and_inner_index_overflows_fail()
-> Result<(), &'static str> {
    for duplicate in [false, true] {
        let mut value = with_lookup();
        let lookups = value["transaction"]["message"]["addressTableLookups"]
            .as_array_mut()
            .unwrap();

        if duplicate {
            lookups.push(lookups[0].clone());
        } else {
            lookups[0]["writableIndexes"] = json!([]);
            lookups[0]["readonlyIndexes"] = json!([]);
        }

        check_eq!(
            decode_rpc_transaction(&parse(value, TransactionEncoding::Json)).unwrap_err(),
            Error::LoadedAddresses
        );
    }

    for instruction in [
        json!({"programIdIndex":255,"accounts":[],"data":""}),
        json!({"programIdIndex":3,"accounts":[255],"data":""}),
    ] {
        let mut value = raw();
        value["meta"]["innerInstructions"] = json!([{"index":0,"instructions":[instruction]}]);
        check_eq!(
            decode_rpc_transaction(&parse(value, TransactionEncoding::Json)).unwrap_err(),
            Error::Index
        );
    }
    Ok(())
}

pub(super) fn parsed_metadata_must_match_exact_loaded_address_and_source_ownership()
-> Result<(), &'static str> {
    for field in ["writable", "readonly"] {
        let mut value = parsed();
        value["meta"]["loadedAddresses"][field] = json!([address(99).to_string()]);
        check_eq!(
            decode_rpc_transaction(&parse(value, TransactionEncoding::JsonParsed)).unwrap_err(),
            Error::LoadedAddresses
        );
    }

    for (index, source) in [(0, "lookupTable"), (4, "transaction")] {
        let mut value = parsed();
        value["transaction"]["message"]["accountKeys"][index]["source"] = json!(source);
        check_eq!(
            decode_rpc_transaction(&parse(value, TransactionEncoding::JsonParsed)).unwrap_err(),
            Error::LoadedAddresses
        );
    }

    Ok(())
}

pub(super) fn nested_trace_depth_and_opaque_payload_borrow_stay_exact() -> Result<(), &'static str>
{
    let mut value = parsed();
    value["meta"]["innerInstructions"] = json!([{"index":0,"instructions":[
        {"program":"opaque","programId":address(4).to_string(),
            "parsed":{"nested":[true,null]},"stackHeight":u32::MAX},
        {"programId":address(4).to_string(),"accounts":[address(LOADED_READONLY).to_string()],
            "data":"2","stackHeight":null},
        {"programId":address(4).to_string(),"accounts":[],"data":""}
    ]}]);
    let source = parse(value, TransactionEncoding::JsonParsed);
    let decoded = decode_rpc_transaction(&source).unwrap();
    let walked = decoded.walk_instructions().unwrap();
    check_eq!(walked.len(), 5);
    check!(matches!(
        walked[1].trace,
        InstructionTrace::Inner {
            outer_index: 0,
            inner_index: 0,
            stack_height: Presence::Present(Some(u32::MAX))
        }
    ));

    check!(matches!(
        walked[2].trace,
        InstructionTrace::Inner {
            inner_index: 1,
            stack_height: Presence::Present(None),
            ..
        }
    ));

    check!(matches!(
        walked[3].trace,
        InstructionTrace::Inner {
            inner_index: 2,
            stack_height: Presence::Missing,
            ..
        }
    ));
    check_borrow(&source, &walked[0].instruction)?;
    check!(decoded.transaction().is_none());
    Ok(())
}

pub(super) fn check_borrow(
    source: &solana_kit_client::methods::transaction_results::TransactionResult,
    instruction: &ResolvedInstruction<'_>,
) -> Result<(), &'static str> {
    let ResolvedInstruction::Parsed(opaque) = instruction else {
        return Err("borrowed parsed instruction");
    };
    let TransactionView::JsonParsed(original) = &source.transaction else {
        return Err("parsed DTO");
    };
    let solana_kit_client::methods::transaction_results::ParsedOrPartialInstruction::Parsed(
        original,
    ) = &original.message.instructions[0]
    else {
        return Err("original parsed");
    };
    check!(std::ptr::eq(*opaque, original));
    check_eq!(opaque.parsed.get(), "{\"huge\":18446744073709551615}");
    Ok(())
}

pub(super) fn conflicting_duplicate_address_roles_cannot_be_resolved_by_address()
-> Result<(), &'static str> {
    let mut value = parsed();
    value["transaction"]["message"]["accountKeys"][2]["pubkey"] = json!(address(4).to_string());
    check_eq!(
        decode_rpc_transaction(&parse(value, TransactionEncoding::JsonParsed)).unwrap_err(),
        Error::Address
    );
    Ok(())
}

pub(super) fn v1_json_duplicate_accounts_and_instruction_bound_fail_closed()
-> Result<(), &'static str> {
    let mut duplicate = raw();
    duplicate["version"] = json!(1);
    duplicate["transaction"]["message"]["accountKeys"][2] = json!(address(1).to_string());
    check_eq!(
        decode_rpc_transaction(&parse(duplicate, TransactionEncoding::Json)).unwrap_err(),
        Error::Header
    );
    let mut excessive = raw();
    excessive["version"] = json!(1);
    excessive["transaction"]["message"]["instructions"] = json!(vec![
        json!({"programIdIndex":3,"accounts":[],"data":""});
        65
    ]);
    check_eq!(
        decode_rpc_transaction(&parse(excessive, TransactionEncoding::Json)).unwrap_err(),
        Error::Header
    );
    Ok(())
}
