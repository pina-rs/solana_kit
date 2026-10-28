//! Exact pinned Surfpool models; no browser floating-point conversion is involved.

mod confidential;
mod events;

pub use confidential::*;
pub use events::*;

use serde::{Deserialize, Serialize};
use solana_kit_core::Address;

/// One account balance assignment for sequential funding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolAccountFunding {
    /// Destination account.
    pub address: Address,
    /// Balance to assign, expressed in lamports.
    pub lamports: u64,
}

object_model! {
/// Exact epoch information returned by clock mutations.
pub struct EpochInfoValue {
    /// Absolute slot number.
    pub absolute_slot: i128,
    /// Slot within the epoch.
    pub slot_index: i128,
    /// Number of slots in this epoch.
    pub slots_in_epoch: i128,
    /// Epoch number.
    pub epoch: i128,
    /// Block height.
    pub block_height: i128,
    /// Optional transaction count; absent and null both mean unknown upstream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_count: Option<i128>,
}
}

/// Mutually exclusive absolute clock target.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TimeTarget {
    /// Absolute slot, including values above JavaScript's exact number range.
    AbsoluteSlot(u64),
    /// Absolute epoch.
    AbsoluteEpoch(u64),
    /// Absolute Unix timestamp in milliseconds.
    AbsoluteTimestamp(u64),
}

/// Account-reset options from the pinned source.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetAccountOptions {
    /// Omitting this differs from explicitly requesting false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_owned_accounts: Option<bool>,
}

/// Account-stream options share the same wire fields as reset options.
pub type StreamAccountOptions = ResetAccountOptions;
