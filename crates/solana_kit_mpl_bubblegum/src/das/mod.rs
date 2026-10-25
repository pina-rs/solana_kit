//! Provider-neutral asset/proof binding. Transport and JSON budgets remain caller owned.
mod helius;
mod response;
pub use helius::HeliusDasClient;

use crate::{AssetProof, MetadataArgs, MetadataArgsV2, Node, RoyaltyRawFields, metadata_to_v2};
use alloc::{string::String, vec::Vec};
use core::{fmt, future::Future};
use solana_kit_core::{
    Address,
    codecs::{Error, Limits},
};

const MAX_PROOF_DEPTH: u32 = u32::BITS;
const NODE_BYTES: usize = 32;

/// Normalized typed DAS asset. Original display royalties remain separate from raw companions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DasAsset {
    /// Provider asset identity.
    pub id: Address,
    /// Committed leaf owner.
    pub owner: Address,
    /// Explicit delegated authority, absent when the owner is also the delegate.
    pub delegate: Option<Address>,
    /// Provider frozen state.
    pub frozen: bool,
    /// Provider transfer prohibition.
    pub non_transferable: bool,
    /// Whether the provider identifies a compressed asset.
    pub compressed: bool,
    /// Tree whose root must match the proof response.
    pub tree: Address,
    /// Full-width Bubblegum nonce.
    pub leaf_id: u64,
    /// Provider update sequence retained separately from the nonce/index.
    pub sequence: u64,
    /// Data digest.
    pub data_hash: Node,
    /// Creators digest.
    pub creator_hash: Node,
    /// Complete leaf digest.
    pub asset_hash: Node,
    /// Optional V2 collection commitment.
    pub collection_hash: Option<Node>,
    /// Optional V2 asset-data commitment.
    pub asset_data_hash: Option<Node>,
    /// Complete optional flags byte.
    pub flags: Option<u8>,
    /// Display-facing V1 metadata representation.
    pub metadata: MetadataArgs,
    /// Raw leaf royalty companions, without changing display metadata.
    pub raw_royalties: RoyaltyRawFields,
    /// Optional original content fields, retained independently of leaf metadata defaults.
    pub content: Option<DasContent>,
    /// All provider grouping entries, including entries unrelated to collections.
    pub grouping: Vec<DasGrouping>,
    /// Original optional royalty fields and resolved inheritance marker.
    pub royalty: Option<DasRoyalty>,
}

/// Optional provider content representation.
#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub struct DasContent {
    /// Exact optional off-chain metadata URI.
    pub json_uri: Option<String>,
    /// Exact optional display metadata.
    pub metadata: Option<DasContentMetadata>,
}

/// Optional display-only content fields.
#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub struct DasContentMetadata {
    /// Display name, absent when the provider omits it.
    pub name: Option<String>,
    /// Display symbol.
    pub symbol: Option<String>,
    /// Original optional description.
    pub description: Option<String>,
    /// Original optional image URI.
    pub image: Option<String>,
}

/// One retained DAS grouping entry.
#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub struct DasGrouping {
    /// Provider group name.
    pub group_key: String,
    /// Provider group value.
    pub group_value: String,
    /// Explicit provider verification status, default false when omitted.
    #[serde(default)]
    pub verified: bool,
}

/// Display royalty data remains separate from raw leaf companions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DasRoyalty {
    /// Original optional display royalty rate.
    pub basis_points: Option<u16>,
    /// Original optional raw leaf royalty rate.
    pub basis_points_raw: Option<u16>,
    /// Original optional sale marker.
    pub primary_sale_happened: Option<bool>,
    /// Source-defined inheritance resolution.
    pub inherited: bool,
}

/// Exact typed getAssetProof fields before tree/index consistency checks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DasAssetProof {
    /// Committed root.
    pub root: Node,
    /// Sibling path from leaf to root.
    pub proof: Vec<Node>,
    /// Binary-heap node index, not the zero-based leaf index.
    pub node_index: u64,
    /// Provider leaf digest.
    pub leaf: Node,
    /// Tree identity.
    pub tree: Address,
}

/// Provider-neutral contract preserving local futures and caller transport errors.
pub trait DasApi {
    /// Original transport/provider error type.
    type Error;
    /// Gets one complete normalized asset with no automatic retry.
    fn get_asset(&self, id: Address) -> impl Future<Output = Result<DasAsset, Self::Error>> + '_;
    /// Gets one complete proof with no automatic retry.
    fn get_asset_proof(
        &self,
        id: Address,
    ) -> impl Future<Output = Result<DasAssetProof, Self::Error>> + '_;
}

/// Provider failures remain distinct from invalid asset/proof data.
#[derive(Debug)]
pub enum DasError<E> {
    /// The caller-owned RPC/transport failed.
    Provider(E),
    /// Parsed values violate a Bubblegum binding or caller budget.
    Data(Error),
}

impl<E: fmt::Display> fmt::Display for DasError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Provider(error) => write!(formatter, "DAS provider failed: {error}"),
            Self::Data(error) => write!(formatter, "DAS asset data failed: {error}"),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for DasError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Provider(error) => Some(error),
            Self::Data(error) => Some(error),
        }
    }
}

/// A bound asset and proof with separately resolved V2 write metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetWithProof {
    /// Original normalized asset and display metadata.
    pub asset: DasAsset,
    /// Checked tree/root/index/path binding.
    pub proof: AssetProof,
    /// Raw-companion-aware metadata for V2 writes/hashing.
    pub current_metadata: MetadataArgsV2,
}

/// Fetches both requests concurrently and drops the losing work on provider failure/cancellation.
/// # Errors
/// Preserves provider errors and rejects asset/tree/leaf/index/budget mismatches.
pub async fn get_asset_with_proof<A: DasApi>(
    api: &A,
    id: Address,
    limits: Limits,
) -> Result<AssetWithProof, DasError<A::Error>> {
    let (asset, proof) = futures_util::future::try_join(api.get_asset(id), api.get_asset_proof(id))
        .await
        .map_err(DasError::Provider)?;
    bind_asset_proof(id, asset, proof, limits).map_err(DasError::Data)
}

/// Validates provider identity and converts the heap index without truncation or aliases.
/// With `hashing`, also verifies the complete supplied path against its root.
/// This does not reconstruct metadata commitments or authenticate a provider root against a bank.
/// # Errors
/// Rejects mismatched identities/digests, invalid indices and exhausted budgets.
pub fn bind_asset_proof(
    id: Address,
    asset: DasAsset,
    proof: DasAssetProof,
    limits: Limits,
) -> Result<AssetWithProof, Error> {
    if asset.id != id
        || !asset.compressed
        || asset.tree != proof.tree
        || asset.asset_hash != proof.leaf
    {
        return Err(Error::InvalidValue);
    }

    let index = proof_index(&proof, limits)?;

    #[cfg(feature = "hashing")]
    if !crate::verify_proof(proof.root, proof.leaf, index, &proof.proof) {
        return Err(Error::InvalidValue);
    }

    let current_metadata = metadata_to_v2(&asset.metadata, &asset.raw_royalties);
    Ok(AssetWithProof {
        asset,
        proof: AssetProof {
            tree: proof.tree,
            root: proof.root,
            leaf: proof.leaf,
            index,
            siblings: proof.proof,
        },
        current_metadata,
    })
}

/// Convert the bounded binary-heap index only after validating the supplied path budget.
fn proof_index(proof: &DasAssetProof, limits: Limits) -> Result<u32, Error> {
    let depth = u32::try_from(proof.proof.len()).map_err(|_| Error::Overflow)?;

    if depth > MAX_PROOF_DEPTH
        || proof.proof.len() > limits.max_items
        || proof
            .proof
            .len()
            .checked_mul(NODE_BYTES)
            .ok_or(Error::Overflow)?
            > limits.max_bytes
    {
        return Err(Error::LimitExceeded);
    }

    let first = 1u64 << depth;
    let index = proof
        .node_index
        .checked_sub(first)
        .ok_or(Error::InvalidValue)?;

    if index >= first {
        return Err(Error::InvalidValue);
    }

    u32::try_from(index).map_err(|_| Error::Overflow)
}
