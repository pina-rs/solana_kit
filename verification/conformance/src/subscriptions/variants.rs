//! Signature failure precision and every unstable slot/block result variant.

use super::*;
use solana_kit_client::methods::{
    submission::{InstructionFailure, KnownTransactionFailure, TransactionFailure},
    transaction_results as tr,
};
use solana_kit_core::Signature;

fn signature_failure(raw: &str, expected: Option<KnownTransactionFailure>) -> Check {
    let mut owner = Connection::new(GENERATION, Budget::default());
    let signature = owner
        .subscribe(
            SignatureNotifications(
                Signature::from_bytes([0; 64]),
                Some(SignatureConfig {
                    commitment: None,
                    enable_received_notification: Some(true),
                }),
            ),
            10,
        )
        .map_err(|_| "signature subscribe failed")?;
    wire(
        &mut owner,
        r#"{"jsonrpc":"2.0","id":1,"method":"signatureSubscribe","params":["1111111111111111111111111111111111111111111111111111111111111111",{"enableReceivedNotification":true}]}"#,
    )?;
    let root = owner
        .subscribe(Root, SUBSCRIBE_DEADLINE)
        .map_err(|_| "peer subscribe failed")?;
    wire(
        &mut owner,
        r#"{"jsonrpc":"2.0","id":2,"method":"rootSubscribe","params":[]}"#,
    )?;
    ack(&mut owner, 1, "7")?;
    ack(&mut owner, 2, "8")?;
    notify(
        &mut owner,
        "signatureNotification",
        SERVER,
        &contextual(r#""receivedSignature""#),
    )?;

    crate::require(
        matches!(
            value(&mut owner, &signature)?.value,
            SignatureInfo::Received(_)
        ),
        "signature receipt missing",
    )?;
    notify(
        &mut owner,
        "signatureNotification",
        SERVER,
        &contextual(&format!(r#"{{"err":{raw}}}"#)),
    )?;
    let result = value(&mut owner, &signature)?;
    crate::require(result.context.slot == u64::MAX, "signature context lost")?;

    execution_error(result.value, expected, raw)?;
    crate::require(
        matches!(owner.next(&signature), Ok(Drain::Closed)) && owner.outbound().is_none(),
        "one-shot signature lifetime differs",
    )?;
    owner
        .forget(signature)
        .map_err(|_| "signature cleanup failed")?;
    notify(&mut owner, "rootNotification", PEER, "18446744073709551615")?;
    crate::require(
        value(&mut owner, &root)? == u64::MAX,
        "signature failure closed peer",
    )
}

fn execution_error(
    value: SignatureInfo,
    expected: Option<KnownTransactionFailure>,
    raw: &str,
) -> Check {
    match (value, expected) {
        (
            SignatureInfo::Processed {
                err: Some(TransactionFailure::Known(actual)),
            },
            Some(expected),
        ) => crate::require(actual == expected, "known signature error differs"),
        (
            SignatureInfo::Processed {
                err: Some(TransactionFailure::Unknown(actual)),
            },
            None,
        ) => crate::require(
            actual.get() == raw,
            "future signature error lost raw spelling",
        ),
        _ => Err("signature error missing"),
    }
}

fn signatures() -> Check {
    signature_failure(
        r#""AccountInUse""#,
        Some(KnownTransactionFailure::AccountInUse),
    )?;
    signature_failure(
        r#"{"InstructionError":[255,{"Custom":4294967295}]}"#,
        Some(KnownTransactionFailure::InstructionError(
            u8::MAX,
            InstructionFailure::Custom(u32::MAX),
        )),
    )?;
    signature_failure(
        r#"{"FutureError":{"x":18446744073709551616,"decimal":1.2300}}"#,
        None,
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum UpdateKind {
    First,
    Completed,
    Created,
    Frozen,
    Dead,
    Confirmed,
    Root,
}

fn slot_kind(result: SlotUpdate) -> Check<UpdateKind> {
    let kind = match result {
        SlotUpdate::FirstShredReceived {
            slot: 1,
            timestamp: 2,
        } => UpdateKind::First,
        SlotUpdate::Completed {
            slot: 1,
            timestamp: 2,
        } => UpdateKind::Completed,
        SlotUpdate::CreatedBank {
            slot: 1,
            timestamp: 2,
            parent: 0,
        } => UpdateKind::Created,
        SlotUpdate::Frozen {
            slot: 1,
            timestamp: 2,
            stats,
        } => {
            frozen_stats(stats)?;
            UpdateKind::Frozen
        }
        SlotUpdate::Dead {
            slot: 1,
            timestamp: 2,
            err,
        } if err == "bank failure" => UpdateKind::Dead,
        SlotUpdate::OptimisticConfirmation {
            slot: 1,
            timestamp: 2,
        } => UpdateKind::Confirmed,
        SlotUpdate::Root {
            slot: 1,
            timestamp: 2,
        } => UpdateKind::Root,
        _ => return Err("slot update fields differ"),
    };

    Ok(kind)
}

fn frozen_stats(stats: SlotStats) -> Check {
    crate::require(
        stats.num_transaction_entries == u64::MAX
            && stats.num_successful_transactions == 0
            && stats.num_failed_transactions == 1
            && stats.max_transactions_per_entry == 9007199254740993,
        "frozen statistics lost precision",
    )
}

fn slots() -> Check {
    let entries = [
        (
            r#"{"type":"firstShredReceived","slot":1,"timestamp":2}"#,
            UpdateKind::First,
        ),
        (
            r#"{"type":"completed","slot":1,"timestamp":2}"#,
            UpdateKind::Completed,
        ),
        (
            r#"{"type":"createdBank","slot":1,"parent":0,"timestamp":2}"#,
            UpdateKind::Created,
        ),
        (
            r#"{"type":"frozen","slot":1,"timestamp":2,"stats":{"numTransactionEntries":18446744073709551615,"numSuccessfulTransactions":0,"numFailedTransactions":1,"maxTransactionsPerEntry":9007199254740993}}"#,
            UpdateKind::Frozen,
        ),
        (
            r#"{"type":"dead","slot":1,"timestamp":2,"err":"bank failure"}"#,
            UpdateKind::Dead,
        ),
        (
            r#"{"type":"optimisticConfirmation","slot":1,"timestamp":2}"#,
            UpdateKind::Confirmed,
        ),
        (
            r#"{"type":"root","slot":1,"timestamp":2}"#,
            UpdateKind::Root,
        ),
    ];

    for (raw, tag) in entries {
        let result = delivered(
            SlotsUpdates,
            r#"{"jsonrpc":"2.0","id":1,"method":"slotsUpdatesSubscribe","params":[]}"#,
            "slotsUpdatesNotification",
            raw,
        )?;

        crate::require(slot_kind(result)? == tag, "slot update variant differs")?;
    }

    Ok(())
}

fn blocks() -> Check {
    let request = || {
        Block(
            BlockFilter::All,
            Some(BlockConfig(tr::BlockConfig {
                transaction_details: Some(tr::TransactionDetails::None),
                rewards: Some(false),
                ..Default::default()
            })),
        )
    };
    let expected = r#"{"jsonrpc":"2.0","id":1,"method":"blockSubscribe","params":["all",{"showRewards":false,"transactionDetails":"none"}]}"#;
    let raw = contextual(&format!(
        r#"{{"slot":18446744073709551615,"err":null,"block":{{"blockhash":"{ADDRESS}","previousBlockhash":"{ADDRESS}","parentSlot":9007199254740993,"blockTime":-9223372036854775808,"blockHeight":18446744073709551615}}}}"#
    ));
    let result = delivered(request(), expected, "blockNotification", &raw)?;
    let block = result.value.block.ok_or("block absent")?;
    crate::require(
        result.value.slot == u64::MAX
            && block.parent_slot == 9007199254740993
            && block.block_time == Some(i64::MIN)
            && block.block_height == Some(u64::MAX)
            && matches!(block.transactions, tr::BlockTransactions::None),
        "typed block differs",
    )?;

    for (error, version) in [
        (r#""BlockStoreError""#, false),
        (r#"{"UnsupportedTransactionVersion":255}"#, true),
    ] {
        let raw = contextual(&format!(r#"{{"slot":1,"err":{error},"block":null}}"#));
        let result = delivered(request(), expected, "blockNotification", &raw)?;

        let valid = match result.value.err {
            Some(BlockUpdateError::BlockStoreError) => !version,
            Some(BlockUpdateError::UnsupportedTransactionVersion(255)) => version,
            _ => false,
        };

        crate::require(valid && result.value.block.is_none(), "block error differs")?;
    }

    Ok(())
}

pub(super) fn variants() -> Check {
    signatures()?;
    slots()?;
    blocks()?;
    super::block_modes::modes()
}
