extern crate std;
use super::fixtures::*;
use solana_kit_client::Presence;
use solana_kit_client::introspection::{
    InstructionTrace, ResolvedInstruction, decode_rpc_transaction,
};
use solana_kit_client::methods::transaction_results::TransactionEncoding;
use solana_kit_core::{AccountRole, transaction::Lifetime};
use std::prelude::v1::*;

pub(super) fn legacy_roles_duplicate_positions_bytes_and_unresolved_lifetime_are_exact()
-> Result<(), &'static str> {
    let source = parse(raw(), TransactionEncoding::Json);
    let decoded = decode_rpc_transaction(&source).unwrap();
    let roles: Vec<_> = decoded
        .account_metas()
        .iter()
        .map(|account| account.role)
        .collect();
    check_eq!(
        roles,
        [
            AccountRole::WritableSigner,
            AccountRole::ReadonlySigner,
            AccountRole::Writable,
            AccountRole::Readonly
        ]
    );
    check_eq!(
        decoded.signers().collect::<Vec<_>>(),
        [address(1), address(2)]
    );
    check_eq!(decoded.signatures().len(), 2);
    check_eq!(decoded.metadata().unwrap().fee, u64::MAX);
    check_eq!(
        decoded.lifetime(),
        Lifetime::Unresolved(solana_kit_core::Hash::from_bytes([9; 32]))
    );

    check!(decoded.transaction().is_none());
    let instructions = decoded.get_instructions().unwrap();
    let ResolvedInstruction::Raw(first) = &instructions[0].instruction else {
        return Err("raw instruction");
    };
    check_eq!(first.program_address, address(4));
    check_eq!(
        first
            .accounts
            .iter()
            .map(|account| account.address)
            .collect::<Vec<_>>(),
        [address(3), address(1), address(3)]
    );
    check_eq!(first.data, [0, 1, 255]);
    Ok(())
}

pub(super) fn walk_interleaves_unsorted_groups_and_retains_depth_presence()
-> Result<(), &'static str> {
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
    check_eq!(
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
    check!(matches!(
        inner[0].trace,
        InstructionTrace::Inner { outer_index: 1, .. }
    ));
    Ok(())
}

pub(super) fn lookup_partitions_preserve_loaded_privileges_and_order() -> Result<(), &'static str> {
    let source = parse(with_lookup(), TransactionEncoding::Json);
    let decoded = decode_rpc_transaction(&source).unwrap();
    let loaded = &decoded.account_metas()[4..];
    check_eq!(
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
        return Err("raw instruction");
    };
    check_eq!(instruction.accounts, [loaded[1], loaded[0], loaded[1]]);

    Ok(())
}

pub(super) fn unavailable_metadata_never_establishes_execution_or_recorded_inner_calls()
-> Result<(), &'static str> {
    for metadata in [serde_json::Value::Null, meta()] {
        let mut json = raw();
        json["meta"] = metadata;
        let source = parse(json, TransactionEncoding::Json);
        let decoded = decode_rpc_transaction(&source).unwrap();
        check!(decoded.get_inner_instructions().unwrap().is_empty());
        check_eq!(decoded.metadata().is_some(), source.meta.is_some());
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
        check!(std::ptr::eq(decoded.source(), &source));
        check!(std::ptr::eq(
            decoded.metadata().unwrap(),
            source.meta.as_ref().unwrap()
        ));
    }
    Ok(())
}

pub(super) fn failed_execution_remains_failed_while_attempted_instructions_are_inspectable()
-> Result<(), &'static str> {
    let mut json = raw();
    json["meta"]["err"] = serde_json::json!("AccountNotFound");
    json["meta"]["status"] = serde_json::json!({"Err":"AccountNotFound"});
    let source = parse(json, TransactionEncoding::Json);
    let decoded = decode_rpc_transaction(&source).unwrap();
    check!(decoded.metadata().unwrap().err.is_some());
    check_eq!(decoded.walk_instructions().unwrap().len(), 2);
    Ok(())
}

pub(super) fn raw_v1_json_retains_the_typed_message_without_synthesizing_wire()
-> Result<(), &'static str> {
    let mut json = raw();
    json["version"] = serde_json::json!(1);
    let source = parse(json, TransactionEncoding::Json);
    let decoded = decode_rpc_transaction(&source).unwrap();
    check_eq!(decoded.version(), solana_kit_core::transaction::Version::V1);
    check!(decoded.transaction().is_none());
    check_eq!(decoded.get_instructions().unwrap().len(), 2);
    Ok(())
}
