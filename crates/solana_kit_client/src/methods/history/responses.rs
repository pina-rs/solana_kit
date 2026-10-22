//! Historical metadata preserves nulls rather than inventing missing history.
use crate::{Presence, methods::submission::TransactionFailure};
use serde::{Deserialize, Deserializer};
use solana_kit_core::Signature;

/// Exact stake totals at each confirmation depth, when known to the node.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockCommitmentValue {
    /// Null means this node has no commitment information for the slot.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub commitment: Option<[u64; 32]>,
    /// Exact total active stake.
    pub total_stake: u64,
}

/// The confirmation state explicitly reported by the node.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConfirmationStatus {
    /// Executed by the node, without cluster confirmation.
    Processed,
    /// Confirmed by the cluster.
    Confirmed,
    /// Finalized by the cluster.
    Finalized,
}

/// One historical signature; execution failure does not become an RPC failure.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHistory {
    /// Validated 64-byte transaction signature.
    #[serde(deserialize_with = "signature")]
    pub signature: Signature,
    /// Transaction slot.
    pub slot: u64,
    /// Explicit null indicates successful execution.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub err: Option<TransactionFailure>,
    /// Explicit null indicates no memo.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub memo: Option<String>,
    /// Estimated timestamp; null means unavailable and absence is malformed.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub block_time: Option<i64>,
    /// Reported confirmation state; null means unavailable.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub confirmation_status: Option<ConfirmationStatus>,
    /// Zero-based block position, omitted by servers older than Agave 4.0.
    #[serde(default)]
    pub transaction_index: Presence<u32>,
}

pub(crate) fn signature<'de, D: Deserializer<'de>>(d: D) -> Result<Signature, D::Error> {
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}
