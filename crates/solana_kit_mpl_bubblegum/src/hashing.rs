//! Keccak-256 domains follow the leaf format, separate from account Borsh enum tags.
use crate::{Creator, MetadataArgs, MetadataArgsCodec, MetadataArgsV2, MetadataArgsV2Codec, Node};
use sha3::{Digest, Keccak256};
use solana_kit_core::{
    Address,
    codecs::{Encoder, Error, Limits},
};

/// Hashes the concatenation without an intermediate concatenated allocation.
#[must_use]
pub fn bubblegum_hash(slices: &[&[u8]]) -> Node {
    let mut hasher = Keccak256::new();

    for slice in slices {
        hasher.update(slice);
    }

    hasher.finalize().into()
}

/// Hashes creator addresses, verification flags and shares without a vector prefix.
#[must_use]
pub fn hash_metadata_creators(creators: &[Creator]) -> Node {
    let mut hasher = Keccak256::new();

    for creator in creators {
        hasher.update(creator.address.as_bytes());
        hasher.update([u8::from(creator.verified), creator.share]);
    }

    hasher.finalize().into()
}

/// Hashes V1 Borsh metadata, then the exact two-byte royalty rate.
/// Resolve DAS raw companions before calling this function.
/// # Errors
/// Preserves encoder limits, integer overflow and allocation failures.
pub fn hash_metadata_data(metadata: &MetadataArgs, limits: Limits) -> Result<Node, Error> {
    let bytes = MetadataArgsCodec.encode(metadata, limits.max_bytes)?;
    Ok(bubblegum_hash(&[
        &bubblegum_hash(&[&bytes]),
        &metadata.seller_fee_basis_points.to_le_bytes(),
    ]))
}

/// Hashes the distinct V2 metadata layout and royalty rate.
/// # Errors
/// Preserves encoder limits, integer overflow and allocation failures.
pub fn hash_metadata_data_v2(metadata: &MetadataArgsV2, limits: Limits) -> Result<Node, Error> {
    let bytes = MetadataArgsV2Codec.encode(metadata, limits.max_bytes)?;
    Ok(bubblegum_hash(&[
        &bubblegum_hash(&[&bytes]),
        &metadata.seller_fee_basis_points.to_le_bytes(),
    ]))
}

/// Commits the V1 data digest together with its independently encoded creators.
/// # Errors
/// Preserves the metadata encoder failure.
pub fn hash_metadata(metadata: &MetadataArgs, limits: Limits) -> Result<Node, Error> {
    Ok(bubblegum_hash(&[
        &hash_metadata_data(metadata, limits)?,
        &hash_metadata_creators(&metadata.creators),
    ]))
}

/// Commits the V2 data digest together with its independently encoded creators.
/// # Errors
/// Preserves the metadata encoder failure.
pub fn hash_metadata_v2(metadata: &MetadataArgsV2, limits: Limits) -> Result<Node, Error> {
    Ok(bubblegum_hash(&[
        &hash_metadata_data_v2(metadata, limits)?,
        &hash_metadata_creators(&metadata.creators),
    ]))
}

/// Hashes a collection address, including the zero address for absence.
#[must_use]
pub fn hash_collection(collection: Option<Address>) -> Node {
    bubblegum_hash(&[collection
        .unwrap_or(Address::from_bytes([0; 32]))
        .as_bytes()])
}

/// None and an empty asset body share Keccak's empty-input digest.
#[must_use]
pub fn hash_asset_data(data: Option<&[u8]>) -> Node {
    bubblegum_hash(&[data.unwrap_or_default()])
}

/// Fixed-width fields common to both leaf hash versions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeafHashInput {
    /// Caller-derived asset identity, never the tree address.
    pub asset_id: Address,
    /// Owner committed to the leaf.
    pub owner: Address,
    /// None resolves to the owner.
    pub delegate: Option<Address>,
    /// Full-width nonce, encoded as eight little-endian bytes.
    pub nonce: u64,
    /// Metadata data digest, retained separately from the creator digest.
    pub data_hash: Node,
    /// Creator digest. A combined metadata hash cannot replace these two fields.
    pub creator_hash: Node,
}

/// Hashes a V1 leaf with the hash domain byte1, distinct from Borsh's V1 tag0.
#[must_use]
pub fn hash_leaf_v1(leaf: &LeafHashInput) -> Node {
    bubblegum_hash(&[
        &[1],
        leaf.asset_id.as_bytes(),
        leaf.owner.as_bytes(),
        leaf.delegate.unwrap_or(leaf.owner).as_bytes(),
        &leaf.nonce.to_le_bytes(),
        &leaf.data_hash,
        &leaf.creator_hash,
    ])
}

/// V2 leaf-only commitments. Flags are raw program bytes, not a presence-bit guess.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeafHashV2 {
    /// Common identity, authority and metadata fields.
    pub leaf: LeafHashInput,
    /// Collection digest supplied or computed by the caller.
    pub collection_hash: Node,
    /// Asset-data digest supplied or computed by the caller.
    pub asset_data_hash: Node,
    /// Complete flags byte preserved without silently masking unknown bits.
    pub flags: u8,
}

/// Hashes the complete V2 commitments with domain byte2.
#[must_use]
pub fn hash_leaf_v2(value: &LeafHashV2) -> Node {
    let leaf = &value.leaf;
    bubblegum_hash(&[
        &[2],
        leaf.asset_id.as_bytes(),
        leaf.owner.as_bytes(),
        leaf.delegate.unwrap_or(leaf.owner).as_bytes(),
        &leaf.nonce.to_le_bytes(),
        &leaf.data_hash,
        &leaf.creator_hash,
        &value.collection_hash,
        &value.asset_data_hash,
        &[value.flags],
    ])
}
