//! Notification-only schemas reuse the RPC scalar and execution-error owners.
use crate::methods::submission::TransactionFailure;
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;
use solana_kit_core::{Address, Hash, Signature};

/// Current slot and its chain relationship.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SlotInfo {
    /// Newly processed slot.
    pub slot: u64,
    /// Parent slot.
    pub parent: u64,
    /// Root slot.
    pub root: u64,
}
/// Transaction execution logs, including failures.
#[derive(Debug, Deserialize)]
pub struct LogsInfo {
    /// Transaction identity.
    #[serde(deserialize_with = "signature")]
    pub signature: Signature,
    /// Required nullable execution failure.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub err: Option<TransactionFailure>,
    /// Ordered log lines.
    pub logs: Vec<String>,
}
/// A received signature is distinct from completed execution.
#[derive(Debug)]
pub enum SignatureInfo {
    /// Optional preliminary notification.
    Received(ReceivedSignature),
    /// Completed execution result.
    Processed {
        /// Required nullable transaction failure.
        err: Option<TransactionFailure>,
    },
}
impl<'de> Deserialize<'de> for SignatureInfo {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Processed {
            #[serde(deserialize_with = "crate::json::required_nullable")]
            err: Option<TransactionFailure>,
        }

        // Untagged enum buffering cannot supply raw JSON to TransactionFailure.
        let raw = Box::<RawValue>::deserialize(deserializer)?;

        if raw.get().starts_with('"') {
            return serde_json::from_str(raw.get())
                .map(Self::Received)
                .map_err(serde::de::Error::custom);
        }

        let processed: Processed =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        Ok(Self::Processed { err: processed.err })
    }
}
/// Exact preliminary signature marker.
#[derive(Debug, Deserialize)]
pub enum ReceivedSignature {
    /// The RPC node received the signature.
    #[serde(rename = "receivedSignature")]
    Received,
}
/// Unstable vote notification.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteInfo {
    /// Vote hash.
    #[serde(deserialize_with = "crate::wire::hash")]
    pub hash: Hash,
    /// Vote transaction signature.
    #[serde(deserialize_with = "signature")]
    pub signature: Signature,
    /// Voted slots.
    pub slots: Vec<u64>,
    /// Nullable vote timestamp in seconds.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub timestamp: Option<i64>,
    /// Vote account.
    #[serde(deserialize_with = "crate::methods::transaction_results::address")]
    pub vote_pubkey: Address,
}
/// Unstable slot lifecycle event. Unknown variants fail explicitly.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SlotUpdate {
    /// First shred observed.
    FirstShredReceived {
        #[doc = "Slot."]
        slot: u64,
        #[doc = "Milliseconds."]
        timestamp: u64,
    },
    /// Slot completed.
    Completed {
        #[doc = "Slot."]
        slot: u64,
        #[doc = "Milliseconds."]
        timestamp: u64,
    },
    /// Bank created.
    CreatedBank {
        #[doc = "Slot."]
        slot: u64,
        #[doc = "Parent."]
        parent: u64,
        #[doc = "Milliseconds."]
        timestamp: u64,
    },
    /// Bank frozen.
    Frozen {
        #[doc = "Slot."]
        slot: u64,
        #[doc = "Milliseconds."]
        timestamp: u64,
        #[doc = "Execution statistics."]
        stats: SlotStats,
    },
    /// Slot abandoned.
    Dead {
        #[doc = "Slot."]
        slot: u64,
        #[doc = "Milliseconds."]
        timestamp: u64,
        #[doc = "Node error."]
        err: String,
    },
    /// Optimistic confirmation.
    OptimisticConfirmation {
        #[doc = "Slot."]
        slot: u64,
        #[doc = "Milliseconds."]
        timestamp: u64,
    },
    /// Root advanced.
    Root {
        #[doc = "Slot."]
        slot: u64,
        #[doc = "Milliseconds."]
        timestamp: u64,
    },
}
/// Execution statistics for a frozen slot.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotStats {
    /// Transaction entries.
    pub num_transaction_entries: u64,
    /// Successful transactions.
    pub num_successful_transactions: u64,
    /// Failed transactions.
    pub num_failed_transactions: u64,
    /// Maximum transactions per entry.
    pub max_transactions_per_entry: u64,
}
fn signature<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Signature, D::Error> {
    let mut values =
        crate::methods::transaction_results::signatures(serde::de::value::SeqDeserializer::<
            _,
            D::Error,
        >::new(std::iter::once(
            String::deserialize(d)?,
        )))?;
    Ok(values.remove(0))
}
