//! Raw success results for common public RPC reads.
//!
//! Each function returns the exact `result` JSON that a node sends inside a
//! JSON-RPC success envelope. Pair it with [`ScriptStep::success`] so the
//! transport correlates the request ID. These helpers cover three public read
//! fixtures rather than mirroring the complete RPC response catalog.
//!
//! [`ScriptStep::success`]: crate::ScriptStep::success

use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_core::{Hash, accounts::EncodedAccount};

/// Encodes a `getBalance` result with an exact slot and u64 lamport value.
#[must_use]
pub fn balance(slot: u64, lamports: u64) -> Vec<u8> {
    format!(r#"{{"context":{{"slot":{slot}}},"value":{lamports}}}"#).into_bytes()
}

/// Encodes a `getLatestBlockhash` result with an exact expiry bound.
#[must_use]
pub fn latest_blockhash(slot: u64, blockhash: Hash, last_valid_block_height: u64) -> Vec<u8> {
    format!(
        r#"{{"context":{{"slot":{slot}}},"value":{{"blockhash":"{blockhash}","lastValidBlockHeight":{last_valid_block_height}}}}}"#
    )
    .into_bytes()
}

/// Encodes a present `getAccountInfo` result from core account metadata.
///
/// The requested address is supplied by the RPC method and does not appear in the
/// result, so the fixture address is unused. Data is encoded as canonical base64
/// and must match the account's declared space. An absent rent epoch is encoded
/// as `u64::MAX`, the conventional non-rent-paying value.
///
/// # Panics
/// Panics if the data length differs from the account's declared space.
#[must_use]
pub fn account(slot: u64, account: &EncodedAccount) -> Vec<u8> {
    let space = u64::try_from(account.data.len()).expect("account data length fits u64");
    assert_eq!(
        space, account.space,
        "fixture account data length must equal its space"
    );
    let data = STANDARD.encode(&account.data);
    let rent_epoch = account.rent_epoch.unwrap_or(u64::MAX);
    let owner = account.owner;

    format!(
        r#"{{"context":{{"slot":{slot}}},"value":{{"lamports":{},"owner":"{owner}","executable":{},"rentEpoch":{rent_epoch},"space":{space},"data":["{data}","base64"]}}}}"#,
        account.lamports, account.executable
    )
    .into_bytes()
}

/// Encodes an absent `getAccountInfo` result.
#[must_use]
pub fn missing_account(slot: u64) -> Vec<u8> {
    format!(r#"{{"context":{{"slot":{slot}}},"value":null}}"#).into_bytes()
}
