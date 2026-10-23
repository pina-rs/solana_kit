//! Public typed RPC introspection with literal transaction and trace oracles.

#[path = "introspection/adversarial.rs"]
mod adversarial;
#[path = "introspection/fixtures.rs"]
mod fixtures;
#[path = "introspection/oracle.rs"]
mod oracle;
#[path = "introspection/parsed.rs"]
mod parsed;
#[path = "introspection/program_boundary.rs"]
mod program_boundary;
#[path = "introspection/security.rs"]
mod security;
#[path = "introspection/wire.rs"]
mod wire;

use fixtures::*;
use solana_kit_client::Presence;
use solana_kit_client::introspection::{
    InstructionTrace, ResolvedInstruction, decode_rpc_transaction,
};
use solana_kit_client::methods::transaction_results::TransactionEncoding;
use solana_kit_core::{AccountRole, transaction::Lifetime};

#[test]
fn legacy_roles_duplicate_positions_bytes_and_unresolved_lifetime_are_exact() {
    let source = parse(raw(), TransactionEncoding::Json);
    let decoded = decode_rpc_transaction(&source).unwrap();
    let roles: Vec<_> = decoded
        .account_metas()
        .iter()
        .map(|account| account.role)
        .collect();
    assert_eq!(
        roles,
        [
            AccountRole::WritableSigner,
            AccountRole::ReadonlySigner,
            AccountRole::Writable,
            AccountRole::Readonly
        ]
    );
    assert_eq!(
        decoded.signers().collect::<Vec<_>>(),
        [address(1), address(2)]
    );
    assert_eq!(decoded.signatures().len(), 2);
    assert_eq!(decoded.metadata().unwrap().fee, u64::MAX);
    assert_eq!(
        decoded.lifetime(),
        Lifetime::Unresolved(solana_kit_core::Hash::from_bytes([9; 32]))
    );

    assert!(decoded.transaction().is_none());
    let instructions = decoded.get_instructions().unwrap();
    let ResolvedInstruction::Raw(first) = &instructions[0].instruction else {
        panic!("raw instruction");
    };
    assert_eq!(first.program_address, address(4));
    assert_eq!(
        first
            .accounts
            .iter()
            .map(|account| account.address)
            .collect::<Vec<_>>(),
        [address(3), address(1), address(3)]
    );
    assert_eq!(first.data, [0, 1, 255]);
}

#[test]
fn walk_interleaves_unsorted_groups_and_retains_depth_presence() {
    let mut json = raw();
    json["meta"]["innerInstructions"] = serde_json::json!([
        {"index":1,"instructions":[{"programIdIndex":3,"accounts":[2],"data":"2","stackHeight":3}]},
        {"index":0,"instructions":[
            {"programIdIndex":3,"accounts":[0,0],"data":"","stackHeight":null},
            {"programIdIndex":3,"accounts":[],"data":"2"}
        ]}
    ]);
    let source = parse(json, TransactionEncoding::Json);
    let decoded = decode_rpc_transaction(&source).unwrap();
    let trace: Vec<_> = decoded
        .walk_instructions()
        .unwrap()
        .into_iter()
        .map(|instruction| instruction.trace)
        .collect();
    assert_eq!(
        trace,
        [
            InstructionTrace::Outer {
                index: 0,
                stack_height: Presence::Missing
            },
            InstructionTrace::Inner {
                outer_index: 0,
                inner_index: 0,
                stack_height: Presence::Present(None)
            },
            InstructionTrace::Inner {
                outer_index: 0,
                inner_index: 1,
                stack_height: Presence::Missing
            },
            InstructionTrace::Outer {
                index: 1,
                stack_height: Presence::Missing
            },
            InstructionTrace::Inner {
                outer_index: 1,
                inner_index: 0,
                stack_height: Presence::Present(Some(3))
            },
        ]
    );
    let inner = decoded.get_inner_instructions().unwrap();
    assert!(matches!(
        inner[0].trace,
        InstructionTrace::Inner { outer_index: 1, .. }
    ));
}

#[test]
fn lookup_partitions_preserve_loaded_privileges_and_order() {
    let source = parse(with_lookup(), TransactionEncoding::Json);
    let decoded = decode_rpc_transaction(&source).unwrap();
    let loaded = &decoded.account_metas()[4..];
    assert_eq!(
        loaded
            .iter()
            .map(|account| (account.address, account.role))
            .collect::<Vec<_>>(),
        [
            (address(LOADED_WRITABLE), AccountRole::Writable),
            (address(LOADED_READONLY), AccountRole::Readonly),
        ]
    );
    let instructions = decoded.get_instructions().unwrap();
    let ResolvedInstruction::Raw(instruction) = &instructions[0].instruction else {
        panic!("raw instruction");
    };
    assert_eq!(instruction.accounts, [loaded[1], loaded[0], loaded[1]]);
}

#[test]
fn unavailable_metadata_never_establishes_execution_or_recorded_inner_calls() {
    for metadata in [serde_json::Value::Null, meta()] {
        let mut json = raw();
        json["meta"] = metadata;
        let source = parse(json, TransactionEncoding::Json);
        let decoded = decode_rpc_transaction(&source).unwrap();
        assert!(decoded.get_inner_instructions().unwrap().is_empty());
        assert_eq!(decoded.metadata().is_some(), source.meta.is_some());
    }

    for inner in [
        None,
        Some(serde_json::Value::Null),
        Some(serde_json::json!([])),
    ] {
        let mut json = raw();

        if let Some(inner) = inner {
            json["meta"]["innerInstructions"] = inner;
        }

        let source = parse(json, TransactionEncoding::Json);
        let decoded = decode_rpc_transaction(&source).unwrap();
        assert!(std::ptr::eq(decoded.source(), &source));
        assert!(std::ptr::eq(
            decoded.metadata().unwrap(),
            source.meta.as_ref().unwrap()
        ));
    }
}

#[test]
fn failed_execution_remains_failed_while_attempted_instructions_are_inspectable() {
    let mut json = raw();
    json["meta"]["err"] = serde_json::json!("AccountNotFound");
    json["meta"]["status"] = serde_json::json!({"Err":"AccountNotFound"});
    let source = parse(json, TransactionEncoding::Json);
    let decoded = decode_rpc_transaction(&source).unwrap();
    assert!(decoded.metadata().unwrap().err.is_some());
    assert_eq!(decoded.walk_instructions().unwrap().len(), 2);
}

#[test]
fn raw_v1_json_retains_the_typed_message_without_synthesizing_wire() {
    let mut json = raw();
    json["version"] = serde_json::json!(1);
    let source = parse(json, TransactionEncoding::Json);
    let decoded = decode_rpc_transaction(&source).unwrap();
    assert_eq!(decoded.version(), solana_kit_core::transaction::Version::V1);
    assert!(decoded.transaction().is_none());
    assert_eq!(decoded.get_instructions().unwrap().len(), 2);
}
