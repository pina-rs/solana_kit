//! Typed Privacy Rings indexer methods.
//!
//! Names and fields follow the Helius API reference, OpenAPI 0.51.2. Each
//! method is a JSON-RPC 2.0 object whose `method` equals the last segment of
//! [`crate::Cluster::indexer_url`]. The Kit client sends the body; the
//! transport chooses that URL and adds the API key.

use serde::{Deserialize, Serialize};
use solana_kit_client::methods::Method;

use crate::Error;

/// Slot bounds. Supplied fields combine with AND.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotBounds {
    /// Include transactions after this slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<u64>,
    /// Include transactions at or after this slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<u64>,
    /// Include transactions before this slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<u64>,
    /// Include transactions at or before this slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<u64>,
}

/// Optional slot filter object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryFilters {
    /// Slot bounds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<SlotBounds>,
}

/// Highest slot the indexer has persisted, and its block time.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerContext {
    /// Indexer block time.
    pub block_time: i64,
    /// Persisted slot.
    pub slot: u64,
}

/// Where an output commitment lives.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputContext {
    /// Commitment hash, base58.
    pub hash: String,
    /// State tree account. The API marks this deprecated in favor of `tree_id`.
    pub tree: String,
    /// State tree id.
    pub tree_id: u16,
    /// Leaf index in that tree.
    pub leaf_index: u64,
}

/// One encrypted output inside an indexer transaction.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputSlot {
    /// View tag, base58.
    pub view_tag: String,
    /// Commitment location.
    pub output_context: OutputContext,
    /// Borsh `OutputDataEncoding` bytes, standard base64.
    pub payload: String,
}

/// One encrypted output match from `getEncryptedUtxosByTags`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedUtxo {
    /// Slot of the Solana transaction.
    pub slot: u64,
    /// Transaction signature, base58.
    pub tx_signature: String,
    /// Tagged transaction viewing key, standard base64, when the event has one.
    pub tx_viewing_pk: Option<String>,
    /// 16-byte salt, standard base64, when the event has one.
    pub salt: Option<String>,
    /// Output commitment, tag, and payload.
    pub output_slot: OutputSlot,
}

/// Transactions returned by `getEncryptedUtxosByTags`, grouped by signature.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoTransaction {
    /// Slot of the transaction.
    pub slot: u64,
    /// Transaction signature, base58.
    pub signature: String,
    /// Matching outputs.
    pub outputs: Vec<EncryptedUtxo>,
}

/// One page of encrypted outputs.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoPage {
    /// Indexer context.
    pub context: IndexerContext,
    /// Newest unpaused output tree, when the indexer knows it.
    #[serde(default)]
    pub output_tree_id: Option<u16>,
    /// Matching transactions.
    pub data: Vec<UtxoTransaction>,
    /// Continuation token. `None` ends the page.
    pub pagination_token: Option<String>,
}

/// Published message that is not an output commitment.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RingsMessage {
    /// View tag, base58.
    pub view_tag: String,
    /// Payload, standard base64.
    pub payload: String,
}

/// One shielded transaction event.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShieldedTransaction {
    /// Slot of the Solana transaction.
    pub slot: u64,
    /// Transaction signature, base58.
    pub tx_signature: String,
    /// Event index when the indexer includes it.
    #[serde(default)]
    pub event_index: Option<u16>,
    /// Tagged transaction viewing key, standard base64.
    pub tx_viewing_pk: Option<String>,
    /// Shared salt, standard base64.
    pub salt: Option<String>,
    /// Outputs in event order. The position is the decryption slot index.
    pub output_slots: Vec<OutputSlot>,
    /// Unbound messages.
    pub messages: Vec<RingsMessage>,
    /// Nullifier hashes, base58.
    pub nullifiers: Vec<String>,
    /// True when an output in this event is proofless.
    pub proofless: bool,
    /// Ring config account, when a custom ring authorized the event.
    #[serde(default)]
    pub ring_config: Option<String>,
    /// Ring program, when the registration has been indexed.
    #[serde(default)]
    pub ring_program_id: Option<String>,
}

/// Events of one Solana transaction, in event order.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ShieldedTransactionGroup {
    /// Slot.
    pub slot: u64,
    /// Signature, base58.
    pub signature: String,
    /// Events.
    pub events: Vec<ShieldedTransaction>,
}

/// A page of shielded history.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShieldedPage {
    /// Indexer context.
    pub context: IndexerContext,
    /// Newest unpaused output tree, when known.
    #[serde(default)]
    pub output_tree_id: Option<u16>,
    /// Matching transactions.
    pub data: Vec<ShieldedTransactionGroup>,
    /// Continuation token.
    pub pagination_token: Option<String>,
}

/// Events for one signature. This method is not paginated.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShieldedSignaturePage {
    /// Indexer context.
    pub context: IndexerContext,
    /// Newest unpaused output tree, when known.
    #[serde(default)]
    pub output_tree_id: Option<u16>,
    /// Events ordered by `event_index`.
    pub transactions: Vec<IndexedShieldedTransaction>,
}

/// One event inside a signature lookup.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedShieldedTransaction {
    /// Position of the event in the Solana transaction.
    pub event_index: u16,
    /// Shielded event.
    pub transaction: ShieldedTransaction,
}

/// Tree identity on a proof.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerkleContext {
    /// Tree type code from the indexer.
    pub tree_type: u16,
    /// Tree account, base58.
    pub tree: String,
}

/// Inclusion proof for one state-tree commitment.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerkleProof {
    /// Requested leaf, base58.
    pub leaf: String,
    /// Tree identity.
    pub merkle_context: MerkleContext,
    /// Sibling hashes, base58, leaf to root.
    pub path: Vec<String>,
    /// Leaf index.
    pub leaf_index: u64,
    /// Root hash, base58.
    pub root: String,
    /// Completed slot for a state root, or the update sequence for an indexed tree.
    pub root_seq: u64,
    /// On-chain root-history position.
    pub root_index: u16,
}

/// Inclusion proofs in request order.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct MerkleProofs {
    /// Indexer context.
    pub context: IndexerContext,
    /// One proof per requested leaf.
    pub proofs: Vec<MerkleProof>,
}

/// Non-inclusion proof for one nullifier.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonInclusionProof {
    /// Requested nullifier, base58.
    pub leaf: String,
    /// Tree identity.
    pub merkle_context: MerkleContext,
    /// Sibling path, base58.
    pub path: Vec<String>,
    /// Low element, base58.
    pub low_element: String,
    /// Low element index.
    pub low_element_index: u64,
    /// High element, base58.
    pub high_element: String,
    /// High element index.
    pub high_element_index: u64,
    /// Root, base58.
    pub root: String,
    /// Root sequence.
    pub root_seq: u64,
    /// Root history position.
    pub root_index: u16,
}

/// Non-inclusion proofs in request order.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct NonInclusionProofs {
    /// Indexer context.
    pub context: IndexerContext,
    /// One proof per requested nullifier.
    pub proofs: Vec<NonInclusionProof>,
}

/// Public registry record for one wallet.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRecord {
    /// Solana owner, base58.
    pub owner: String,
    /// Nullifier public key, base58. It does not rotate.
    pub nullifier_pubkey: String,
    /// Tagged X25519 viewing key, standard base64 (`0x04 || u`).
    pub viewing_pubkey: String,
    /// `0` disables merging. `1` selects PoseidonCtrV1.
    pub merge_encryption: u8,
}

/// Registry records in request order. `None` means the owner has no record.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct UserRecords {
    /// Slot at which the accounts were read is `context.slot` or later.
    pub context: IndexerContext,
    /// One entry per requested owner.
    pub records: Vec<Option<UserRecord>>,
}

/// `getEncryptedUtxosByTags`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEncryptedUtxosByTags {
    /// Base58 view tags. An empty list requires `ring_program_id`.
    pub tags: Vec<String>,
    /// Slot bounds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HistoryFilters>,
    /// Page size, from 1 to 1,000 transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Minimum indexer slot. The server returns `-32016` when it is behind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    /// Token from the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// Ring program. Required when `tags` is empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_program_id: Option<String>,
}

/// `getShieldedTransactionsByTags`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetShieldedTransactionsByTags {
    /// Base58 view tags.
    pub tags: Vec<String>,
    /// Slot bounds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HistoryFilters>,
    /// Page size, from 1 to 1,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Minimum indexer slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    /// Continuation token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// Ring program. Required when `tags` is empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_program_id: Option<String>,
}

/// `getShieldedTransactionsBySignature`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetShieldedTransactionsBySignature {
    /// Base58 transaction signature.
    pub tx_signature: String,
}

/// `getShieldedTransactionsByNullifiers`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetShieldedTransactionsByNullifiers {
    /// Base58 nullifiers.
    pub nullifiers: Vec<String>,
    /// Slot bounds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HistoryFilters>,
    /// Page size, from 1 to 1,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Minimum indexer slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    /// Continuation token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

/// `getMerkleProofs`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMerkleProofs {
    /// State tree account, base58.
    pub tree_account: String,
    /// Commitment hashes, base58.
    pub leaves: Vec<String>,
}

/// `getNonInclusionProofs`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNonInclusionProofs {
    /// Nullifier or state tree account, base58.
    pub tree_account: String,
    /// Nullifier values, base58.
    pub leaves: Vec<String>,
}

/// `getUserRecords`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GetUserRecords {
    /// Solana owners, base58. One to 100 keys.
    pub owners: Vec<String>,
}

impl GetEncryptedUtxosByTags {
    /// Checks the documented page and tag bounds. It does not contact the network.
    ///
    /// # Errors
    /// Rejects more than 1,000 tags, an empty tag list without a ring, or a
    /// limit outside 1..=1,000.
    pub fn validate(&self) -> Result<(), Error> {
        validate_tags(&self.tags, self.ring_program_id.as_deref())?;
        validate_limit(self.limit)
    }
}

impl GetShieldedTransactionsByTags {
    /// Checks the documented page and tag bounds.
    ///
    /// # Errors
    /// Rejects more than 1,000 tags, an empty tag list without a ring, or a
    /// limit outside 1..=1,000.
    pub fn validate(&self) -> Result<(), Error> {
        validate_tags(&self.tags, self.ring_program_id.as_deref())?;
        validate_limit(self.limit)
    }
}

impl GetShieldedTransactionsByNullifiers {
    /// Checks the documented page and nullifier bounds.
    ///
    /// # Errors
    /// Rejects an empty list, more than 1,000 nullifiers, or a limit outside 1..=1,000.
    pub fn validate(&self) -> Result<(), Error> {
        if self.nullifiers.is_empty() || self.nullifiers.len() > 1_000 {
            return Err(Error::Limit("nullifier list"));
        }
        validate_limit(self.limit)
    }
}

impl GetMerkleProofs {
    /// Checks the documented leaf bound.
    ///
    /// # Errors
    /// Rejects an empty list or more than 1,000 leaves.
    pub fn validate(&self) -> Result<(), Error> {
        validate_leaves(&self.leaves)
    }
}

impl GetNonInclusionProofs {
    /// Checks the documented leaf bound.
    ///
    /// # Errors
    /// Rejects an empty list or more than 1,000 leaves.
    pub fn validate(&self) -> Result<(), Error> {
        validate_leaves(&self.leaves)
    }
}

impl GetUserRecords {
    /// Checks the documented owner bound.
    ///
    /// # Errors
    /// Rejects an empty list or more than 100 owners.
    pub fn validate(&self) -> Result<(), Error> {
        if self.owners.is_empty() || self.owners.len() > 100 {
            return Err(Error::Limit("user record owners"));
        }
        Ok(())
    }
}

fn validate_tags(tags: &[String], ring: Option<&str>) -> Result<(), Error> {
    if tags.len() > 1_000 || (tags.is_empty() && ring.unwrap_or("").is_empty()) {
        return Err(Error::Limit("view tags"));
    }
    Ok(())
}

fn validate_limit(limit: Option<u64>) -> Result<(), Error> {
    if let Some(limit) = limit
        && !(1..=1_000).contains(&limit)
    {
        return Err(Error::Limit("page limit"));
    }
    Ok(())
}

fn validate_leaves(leaves: &[String]) -> Result<(), Error> {
    if leaves.is_empty() || leaves.len() > 1_000 {
        return Err(Error::Limit("proof leaves"));
    }
    Ok(())
}

/// Decodes a standard-base64 indexer field (`payload`, `txViewingPk`, `salt`, `viewingPubkey`).
///
/// # Errors
/// Rejects a string that is not the standard base64 alphabet.
pub fn decode_base64(text: &str) -> Result<Vec<u8>, Error> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(text.trim())
        .map_err(|_| Error::Base64)
}

fn decode_object<T: for<'de> Deserialize<'de>>(
    result: &str,
) -> Result<T, solana_kit_client::Error> {
    if !result.trim_start().starts_with('{') {
        return Err(solana_kit_client::Error::Schema(
            <serde_json::Error as serde::de::Error>::custom("expected a privacy result object"),
        ));
    }
    serde_json::from_str(result).map_err(solana_kit_client::Error::Schema)
}

macro_rules! method {
    ($ty:ty, $name:literal, $response:ty) => {
        impl Method for $ty {
            type Response = $response;
            const NAME: &'static str = $name;

            fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
                decode_object(result)
            }
        }
    };
}

method!(GetEncryptedUtxosByTags, "getEncryptedUtxosByTags", UtxoPage);
method!(
    GetShieldedTransactionsByTags,
    "getShieldedTransactionsByTags",
    ShieldedPage
);
method!(
    GetShieldedTransactionsBySignature,
    "getShieldedTransactionsBySignature",
    ShieldedSignaturePage
);
method!(
    GetShieldedTransactionsByNullifiers,
    "getShieldedTransactionsByNullifiers",
    ShieldedPage
);
method!(GetMerkleProofs, "getMerkleProofs", MerkleProofs);
method!(
    GetNonInclusionProofs,
    "getNonInclusionProofs",
    NonInclusionProofs
);
method!(GetUserRecords, "getUserRecords", UserRecords);
