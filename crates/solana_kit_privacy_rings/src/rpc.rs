//! Typed Privacy Rings indexer methods.
//!
//! Names and fields follow the Helius API reference, OpenAPI 0.51.2. Each
//! method is a JSON-RPC 2.0 object whose `method` equals the last segment of
//! [`crate::Cluster::indexer_url`]. The Kit client sends the body; the
//! transport chooses that URL and adds the API key.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use solana_kit_client::methods::Method;

use crate::Error;
use crate::history::Page;

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
    /// Continuation token. The producer serializes the key on every page:
    /// explicit `null` ends the history, so a missing key is a schema error.
    #[serde(deserialize_with = "required_nullable")]
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
    /// Continuation token. The producer serializes the key on every page:
    /// explicit `null` ends the history, so a missing key is a schema error.
    #[serde(deserialize_with = "required_nullable")]
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

/// One nullifier still sitting in a tree's on-chain input queue.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct NullifierQueueElement {
    /// Queue sequence. `next_start_seq` maps a full page to the next one.
    pub seq: u64,
    /// Nullifier hash, base58, exactly 32 decoded bytes.
    pub value: String,
}

/// `getNullifierQueueElements` result, in input-queue order.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct NullifierQueue {
    /// Indexer context.
    pub context: IndexerContext,
    /// Elements with `seq >= start_seq`, oldest first. Queue pruning may
    /// leave gaps between sequences.
    pub elements: Vec<NullifierQueueElement>,
}

/// `getNullifierQueueElements`.
///
/// The indexer returns queued nullifiers for one tree. `start_seq` is
/// inclusive. `limit` is the page size, from 1 to 1,000.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNullifierQueueElements {
    /// State or nullifier tree account, base58.
    pub tree_account: String,
    /// First sequence to return. Zero starts at the head of the queue.
    pub start_seq: u64,
    /// Maximum number of elements.
    pub limit: u64,
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

impl GetNullifierQueueElements {
    /// Checks the documented page bound.
    ///
    /// # Errors
    /// Rejects an empty tree account or a limit outside 1..=1,000.
    pub fn validate(&self) -> Result<(), Error> {
        if self.tree_account.is_empty() || !(1..=1_000).contains(&self.limit) {
            return Err(Error::Limit("nullifier queue"));
        }

        Ok(())
    }

    /// Admits one decoded page against this request.
    ///
    /// The page holds at most `limit` elements, every sequence sits at or
    /// after the inclusive `start_seq`, sequences are strictly increasing
    /// oldest first with no duplicates, and every value decodes as base58 to
    /// exactly the 32 bytes of a nullifier hash. Pruned queues legitimately
    /// leave sequence gaps. The result names the queue element the response
    /// carries: the queue output has no tree identity of its own, so this
    /// binds the page to the request, not to any tree.
    ///
    /// # Errors
    /// Returns the name of the violated queue rule.
    fn check_queue_page(&self, page: &NullifierQueue) -> Result<(), &'static str> {
        self.validate().map_err(|_| "queue request")?;

        if page.elements.len() as u64 > self.limit {
            return Err("queue page bound");
        }

        let mut previous: Option<u64> = None;

        for element in &page.elements {
            if element.seq < self.start_seq {
                return Err("queue sequence bound");
            }

            if previous.is_some_and(|previous| element.seq <= previous) {
                return Err("queue order");
            }

            previous = Some(element.seq);

            if crate::history::decode_nullifier(&element.value).is_err() {
                return Err("queue value");
            }
        }

        Ok(())
    }

    /// The `start_seq` that fetches past this page.
    ///
    /// A page shorter than the request's `limit` — including an empty page —
    /// ends the queue, and a full page advances to one past its last
    /// sequence, skipping pruned gaps. A full page whose last sequence is
    /// `u64::MAX` cannot advance and is an error, never a wrapped or
    /// restarted cursor.
    ///
    /// # Errors
    /// Rejects a page that violates the queue contract, and a cursor overflow.
    pub fn next_start_seq(&self, page: &NullifierQueue) -> Result<Option<u64>, Error> {
        self.check_queue_page(page).map_err(Error::Limit)?;

        if page.elements.is_empty() || (page.elements.len() as u64) < self.limit {
            return Ok(None);
        }

        page.elements
            .last()
            .and_then(|last| last.seq.checked_add(1))
            .map(Some)
            .ok_or(Error::Limit("queue sequence bound"))
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

/// A schema failure with the violated contract in the message.
fn schema(message: impl fmt::Display) -> solana_kit_client::Error {
    solana_kit_client::Error::Schema(<serde_json::Error as serde::de::Error>::custom(message))
}

fn decode_object<T: for<'de> Deserialize<'de>>(
    result: &str,
) -> Result<T, solana_kit_client::Error> {
    if !result.trim_start().starts_with('{') {
        return Err(schema("expected a privacy result object"));
    }

    serde_json::from_str(result).map_err(solana_kit_client::Error::Schema)
}

/// Required nullable field: serde's ordinary `Option` also accepts a missing
/// field, and the pinned producer serializes `paginationToken` on every page,
/// `null` when the history ends. A page from before the field existed is
/// refused instead of silently completing a walk.
fn required_nullable<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::deserialize(deserializer)
}

/// The request-relative controls every paginated page satisfies: the page
/// snapshot is at least as fresh as the requested minimum slot, and the page
/// carries no more transaction groups than the requested size. Slot filters
/// describe which rows the indexer matched, not snapshot freshness, so they
/// play no part in the fence.
fn check_page_bounds(
    context_slot: u64,
    groups: usize,
    min_context_slot: Option<u64>,
    limit: Option<u64>,
) -> Result<(), solana_kit_client::Error> {
    if let Some(floor) = min_context_slot
        && context_slot < floor
    {
        return Err(schema(format_args!(
            "page context slot {context_slot} is below the requested minContextSlot {floor}"
        )));
    }

    if let Some(limit) = limit
        && groups as u64 > limit
    {
        return Err(schema(format_args!(
            "page holds {groups} transaction groups beyond the requested limit {limit}"
        )));
    }

    Ok(())
}

/// Binds positional proofs to their request: one proof per requested leaf,
/// every proof inside the requested tree, and each proof leaf equal to the
/// leaf asked for at its position. Repeated requested leaves keep repeated
/// proofs.
fn check_positional_proofs<'a>(
    tree_account: &str,
    leaves: &[String],
    proofs: impl IntoIterator<Item = (&'a str, &'a MerkleContext)>,
) -> Result<(), solana_kit_client::Error> {
    let mut count = 0usize;

    for (position, (leaf, context)) in proofs.into_iter().enumerate() {
        if position >= leaves.len() {
            return Err(schema(format_args!(
                "{} proofs for {} requested leaves",
                position + 1,
                leaves.len()
            )));
        }

        if context.tree != tree_account {
            return Err(schema(format_args!(
                "proof {position} names tree {} but the request asked for {tree_account}",
                context.tree
            )));
        }

        if leaf != leaves[position] {
            return Err(schema(format_args!(
                "proof {position} names leaf {leaf} but the request asked for {}",
                leaves[position]
            )));
        }

        count = position + 1;
    }

    if count != leaves.len() {
        return Err(schema(format_args!(
            "{count} proofs for {} requested leaves",
            leaves.len()
        )));
    }

    Ok(())
}

/// Checks that outputs retain their enclosing transaction's identity.
fn check_utxo_groups(groups: &[UtxoTransaction]) -> Result<(), solana_kit_client::Error> {
    for group in groups {
        if group
            .outputs
            .iter()
            .any(|output| output.tx_signature != group.signature || output.slot != group.slot)
        {
            return Err(schema("UTXO output disagrees with its transaction group"));
        }
    }

    Ok(())
}

/// Checks that events retain their enclosing transaction's identity.
fn check_shielded_groups(
    groups: &[ShieldedTransactionGroup],
) -> Result<(), solana_kit_client::Error> {
    for group in groups {
        if group
            .events
            .iter()
            .any(|event| event.tx_signature != group.signature || event.slot != group.slot)
        {
            return Err(schema(
                "shielded event disagrees with its transaction group",
            ));
        }
    }

    Ok(())
}

impl Method for GetEncryptedUtxosByTags {
    type Response = UtxoPage;
    const NAME: &'static str = "getEncryptedUtxosByTags";

    fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
        let page: UtxoPage = decode_object(result)?;
        check_page_bounds(
            page.context.slot,
            page.data.len(),
            self.min_context_slot,
            self.limit,
        )?;
        check_utxo_groups(&page.data)?;

        Ok(page)
    }
}

impl Method for GetShieldedTransactionsByTags {
    type Response = ShieldedPage;
    const NAME: &'static str = "getShieldedTransactionsByTags";

    fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
        let page: ShieldedPage = decode_object(result)?;
        check_page_bounds(
            page.context.slot,
            page.data.len(),
            self.min_context_slot,
            self.limit,
        )?;
        check_shielded_groups(&page.data)?;

        Ok(page)
    }
}

impl Method for GetShieldedTransactionsBySignature {
    type Response = ShieldedSignaturePage;
    const NAME: &'static str = "getShieldedTransactionsBySignature";

    fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
        let page: ShieldedSignaturePage = decode_object(result)?;
        let expected_slot = page
            .transactions
            .first()
            .map(|entry| entry.transaction.slot);

        for entry in &page.transactions {
            let found = entry.transaction.tx_signature.as_str();

            if found != self.tx_signature {
                return Err(schema(format_args!(
                    "event {} names signature {found} but the request asked for {}",
                    entry.event_index, self.tx_signature
                )));
            }

            if Some(entry.transaction.slot) != expected_slot
                || entry
                    .transaction
                    .event_index
                    .is_some_and(|index| index != entry.event_index)
            {
                return Err(schema(
                    "event index or slot disagrees with the signature result",
                ));
            }
        }

        Ok(page)
    }
}

impl Method for GetShieldedTransactionsByNullifiers {
    type Response = ShieldedPage;
    const NAME: &'static str = "getShieldedTransactionsByNullifiers";

    fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
        let page: ShieldedPage = decode_object(result)?;
        check_page_bounds(
            page.context.slot,
            page.data.len(),
            self.min_context_slot,
            self.limit,
        )?;
        check_shielded_groups(&page.data)?;

        Ok(page)
    }
}

impl Method for GetMerkleProofs {
    type Response = MerkleProofs;
    const NAME: &'static str = "getMerkleProofs";

    fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
        let proofs: MerkleProofs = decode_object(result)?;
        check_positional_proofs(
            &self.tree_account,
            &self.leaves,
            proofs
                .proofs
                .iter()
                .map(|proof| (proof.leaf.as_str(), &proof.merkle_context)),
        )?;

        Ok(proofs)
    }
}

impl Method for GetNonInclusionProofs {
    type Response = NonInclusionProofs;
    const NAME: &'static str = "getNonInclusionProofs";

    fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
        let proofs: NonInclusionProofs = decode_object(result)?;
        check_positional_proofs(
            &self.tree_account,
            &self.leaves,
            proofs
                .proofs
                .iter()
                .map(|proof| (proof.leaf.as_str(), &proof.merkle_context)),
        )?;

        Ok(proofs)
    }
}

impl Method for GetUserRecords {
    type Response = UserRecords;
    const NAME: &'static str = "getUserRecords";

    fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
        let records: UserRecords = decode_object(result)?;

        if records.records.len() != self.owners.len() {
            return Err(schema(format_args!(
                "{} records for {} requested owners",
                records.records.len(),
                self.owners.len()
            )));
        }

        for (position, (record, owner)) in
            records.records.iter().zip(self.owners.iter()).enumerate()
        {
            let Some(record) = record else {
                continue;
            };

            if record.owner != *owner {
                return Err(schema(format_args!(
                    "record {position} names owner {} but the request asked for {owner}",
                    record.owner
                )));
            }
        }

        Ok(records)
    }
}

impl Method for GetNullifierQueueElements {
    type Response = NullifierQueue;
    const NAME: &'static str = "getNullifierQueueElements";

    fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
        let page: NullifierQueue = decode_object(result)?;
        self.check_queue_page(&page)
            .map_err(|rule| schema(format_args!("{rule} rejected")))?;

        Ok(page)
    }
}

impl From<UtxoPage> for Page<UtxoTransaction> {
    fn from(page: UtxoPage) -> Self {
        Page {
            context_slot: page.context.slot,
            items: page.data,
            pagination_token: page.pagination_token,
        }
    }
}

impl From<ShieldedPage> for Page<ShieldedTransactionGroup> {
    fn from(page: ShieldedPage) -> Self {
        Page {
            context_slot: page.context.slot,
            items: page.data,
            pagination_token: page.pagination_token,
        }
    }
}
