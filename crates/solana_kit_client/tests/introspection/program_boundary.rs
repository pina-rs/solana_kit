//! Public boundary parity controls selected from the core wire invariants.

use super::fixtures::*;
use serde_json::{Value, json};
use solana_kit_client::Presence;
use solana_kit_client::introspection::{
    Error, InstructionTrace, ResolvedInstruction, decode_rpc_transaction, get_inner_instructions,
};
use solana_kit_client::methods::transaction_results::TransactionEncoding;
use solana_kit_core::transaction::Transaction;

const STATIC_ACCOUNT_COUNT: u8 = 4;

#[test]
fn raw_json_must_reject_payer_as_invoked_program_like_core_wire() {
    let mut bytes = literal_wire(255);
    bytes[166] = 0;
    assert!(Transaction::decode(&bytes).is_err());
    let mut value = raw();
    value["transaction"]["message"]["instructions"][0]["programIdIndex"] = json!(0);
    let source = parse(value, TransactionEncoding::Json);
    assert_eq!(decode_rpc_transaction(&source).unwrap_err(), Error::Index);
}

#[test]
fn raw_json_must_reject_loaded_invoked_program_like_core_wire() {
    let mut bytes = literal_wire(0);
    bytes[167] = 2;
    bytes.pop();
    bytes.push(1);
    bytes.extend([10; 32]);
    bytes.extend([1, 9, 1, 11]);
    assert!(Transaction::decode(&bytes).is_err());

    let mut value = with_lookup();
    value["transaction"]["message"]["instructions"][0]["programIdIndex"] = json!(4);
    let source = parse(value, TransactionEncoding::Json);
    assert_eq!(decode_rpc_transaction(&source).unwrap_err(), Error::Index);
}

#[test]
fn payer_and_static_boundary_follow_core_rules_for_every_version() {
    for version in [json!("legacy"), json!(0), json!(1)] {
        for program in [0, 1, 2, 3, 4] {
            assert_static_program(version.clone(), program);
        }
    }
}

fn assert_static_program(version: Value, program: u8) {
    let mut value = raw();
    value["version"] = version;
    value["transaction"]["message"]["addressTableLookups"] = json!([]);
    value["transaction"]["message"]["instructions"][0]["programIdIndex"] = json!(program);
    let source = parse(value, TransactionEncoding::Json);
    let result = decode_rpc_transaction(&source);

    if program == 0 || program == STATIC_ACCOUNT_COUNT {
        assert_eq!(result.unwrap_err(), Error::Index);
        return;
    }

    let decoded = result.unwrap();
    let outer = decoded.get_instructions().unwrap();
    let ResolvedInstruction::Raw(instruction) = &outer[0].instruction else {
        panic!("raw instruction");
    };
    assert_eq!(instruction.program_address, address(program + 1));
}

#[test]
fn inner_loaded_programs_preserve_account_roles_and_trace() {
    for program in [4, 5] {
        let mut value = with_lookup();
        value["meta"]["innerInstructions"] = json!([{"index":0,"instructions":[{
            "programIdIndex":program,"accounts":[5,4,5],"data":"2","stackHeight":2
        }]}]);

        let source = parse(value, TransactionEncoding::Json);
        let decoded = decode_rpc_transaction(&source).unwrap();
        let inner = decoded.get_inner_instructions().unwrap();
        let standalone =
            get_inner_instructions(decoded.metadata(), decoded.account_metas()).unwrap();

        assert_eq!(standalone.len(), 1);
        assert_eq!(inner.len(), 1);
        assert_eq!(
            inner[0].trace,
            InstructionTrace::Inner {
                outer_index: 0,
                inner_index: 0,
                stack_height: Presence::Present(Some(2)),
            }
        );
        let ResolvedInstruction::Raw(instruction) = &inner[0].instruction else {
            panic!("raw inner instruction");
        };
        assert_eq!(instruction.program_address, address(program + 1));
        let loaded = &decoded.account_metas()[usize::from(STATIC_ACCOUNT_COUNT)..];

        assert_eq!(instruction.accounts, [loaded[1], loaded[0], loaded[1]]);
        assert_eq!(instruction.data, [1]);
        assert_eq!(decoded.walk_instructions().unwrap().len(), 3);
    }
}
