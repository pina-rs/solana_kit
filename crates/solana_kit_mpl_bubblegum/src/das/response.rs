//! MIT Dart DAS schema. Required identity/proof fields fail instead of becoming empty values.
use super::{
    DasAsset, DasAssetProof, DasContent, DasGrouping, DasRoyalty, MAX_PROOF_DEPTH, NODE_BYTES,
};
use crate::{Collection, Creator, MetadataArgs, RoyaltyRawFields, SELLER_FEE_BASIS_POINTS_INHERIT};
use alloc::{string::String, vec::Vec};
use serde::Deserialize;
use solana_kit_core::{
    Address, Hash,
    codecs::{Error, Limits},
};

#[derive(Deserialize)]
pub(super) struct RawAsset {
    id: String,
    ownership: Ownership,
    compression: Compression,
    content: Option<DasContent>,
    #[serde(default)]
    creators: Vec<RawCreator>,
    creators_raw: Option<Vec<RawCreator>>,
    #[serde(default)]
    grouping: Vec<DasGrouping>,
    royalty: Option<Royalty>,
    #[serde(default = "yes")]
    mutable: bool,
    supply: Option<Supply>,
}

#[derive(Deserialize)]
struct Ownership {
    owner: String,
    delegate: Option<String>,
    frozen: bool,
    #[serde(default)]
    non_transferable: bool,
}

#[derive(Deserialize)]
struct Compression {
    compressed: bool,
    data_hash: String,
    creator_hash: String,
    asset_hash: String,
    tree: String,
    seq: u64,
    leaf_id: u64,
    collection_hash: Option<String>,
    asset_data_hash: Option<String>,
    flags: Option<u8>,
}

#[derive(Deserialize)]
struct RawCreator {
    address: String,
    share: u8,
    verified: bool,
}
#[derive(Deserialize)]
struct Supply {
    edition_nonce: Option<u8>,
}
#[derive(Deserialize)]
struct Royalty {
    basis_points: Option<u16>,
    basis_points_raw: Option<u16>,
    primary_sale_happened: Option<bool>,
    sfbp_inherited: Option<bool>,
    inherited: Option<bool>,
}

fn yes() -> bool {
    true
}

fn address(value: &str) -> Result<Address, Error> {
    value.parse().map_err(|_| Error::InvalidValue)
}

fn node(value: &str) -> Result<[u8; 32], Error> {
    value
        .parse::<Hash>()
        .map(|hash| hash.to_bytes())
        .map_err(|_| Error::InvalidValue)
}

fn creators(values: Vec<RawCreator>) -> Result<Vec<Creator>, Error> {
    let mut output = Vec::new();
    output
        .try_reserve_exact(values.len())
        .map_err(|_| Error::AllocationFailed)?;

    for value in values {
        output.push(Creator {
            address: address(&value.address)?,
            share: value.share,
            verified: value.verified,
        });
    }

    Ok(output)
}

impl RawAsset {
    pub(super) fn normalize(self, limits: Limits) -> Result<DasAsset, Error> {
        self.check_item_budget(limits)?;
        let collection = collection(&self.grouping)?;

        let mut metadata =
            MetadataArgs::nft(String::new(), String::new(), creators(self.creators)?);

        apply_content(&mut metadata, self.content.as_ref());

        metadata.collection = collection;
        metadata.is_mutable = self.mutable;
        metadata.edition_nonce = self.supply.and_then(|supply| supply.edition_nonce);
        let (mut raw, inherited, original_royalty) = apply_royalty(&mut metadata, self.royalty);

        raw.creators = self
            .creators_raw
            .map(creators)
            .transpose()?
            .or_else(|| inherited.then(Vec::new));
        let compression = self.compression;
        Ok(DasAsset {
            id: address(&self.id)?,
            owner: address(&self.ownership.owner)?,
            delegate: self
                .ownership
                .delegate
                .as_deref()
                .map(address)
                .transpose()?,
            frozen: self.ownership.frozen,
            non_transferable: self.ownership.non_transferable,
            compressed: compression.compressed,
            tree: address(&compression.tree)?,
            leaf_id: compression.leaf_id,
            sequence: compression.seq,
            data_hash: node(&compression.data_hash)?,
            creator_hash: node(&compression.creator_hash)?,
            asset_hash: node(&compression.asset_hash)?,
            collection_hash: compression
                .collection_hash
                .as_deref()
                .map(node)
                .transpose()?,
            asset_data_hash: compression
                .asset_data_hash
                .as_deref()
                .map(node)
                .transpose()?,
            flags: compression.flags,
            metadata,
            raw_royalties: raw,
            content: self.content,
            grouping: self.grouping,
            royalty: original_royalty,
        })
    }

    fn check_item_budget(&self, limits: Limits) -> Result<(), Error> {
        let count = self
            .creators
            .len()
            .checked_add(self.creators_raw.as_ref().map_or(0, Vec::len))
            .and_then(|count| count.checked_add(self.grouping.len()))
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        Ok(())
    }
}

/// Preserve the provider's single nonempty collection entry, rejecting duplicates.
fn collection(groups: &[DasGrouping]) -> Result<Option<Collection>, Error> {
    let mut collection = None;

    for group in groups {
        if group.group_key != "collection" || group.group_value.is_empty() {
            continue;
        }

        if collection.is_some() {
            return Err(Error::InvalidValue);
        }

        collection = Some(Collection {
            key: address(&group.group_value)?,
            verified: group.verified,
        });
    }

    Ok(collection)
}

fn apply_content(metadata: &mut MetadataArgs, content: Option<&DasContent>) {
    let Some(content) = content else {
        return;
    };
    metadata.uri = content.json_uri.clone().unwrap_or_default();

    if let Some(fields) = &content.metadata {
        metadata.name = fields.name.clone().unwrap_or_default();
        metadata.symbol = fields.symbol.clone().unwrap_or_default();
    }
}

/// Display royalties and raw inherited companions remain independent representations.
fn apply_royalty(
    metadata: &mut MetadataArgs,
    royalty: Option<Royalty>,
) -> (RoyaltyRawFields, bool, Option<DasRoyalty>) {
    let Some(royalty) = royalty else {
        return (RoyaltyRawFields::default(), false, None);
    };
    metadata.seller_fee_basis_points = royalty.basis_points.unwrap_or(0);
    metadata.primary_sale_happened = royalty.primary_sale_happened.unwrap_or(false);
    let inherited = royalty
        .sfbp_inherited
        .or(royalty.inherited)
        .unwrap_or(royalty.basis_points == Some(SELLER_FEE_BASIS_POINTS_INHERIT));
    let raw = RoyaltyRawFields {
        seller_fee_basis_points: royalty
            .basis_points_raw
            .or(inherited.then_some(SELLER_FEE_BASIS_POINTS_INHERIT)),
        ..RoyaltyRawFields::default()
    };
    let original = DasRoyalty {
        basis_points: royalty.basis_points,
        basis_points_raw: royalty.basis_points_raw,
        primary_sale_happened: royalty.primary_sale_happened,
        inherited,
    };

    (raw, inherited, Some(original))
}

#[derive(Deserialize)]
pub(super) struct RawProof {
    root: String,
    proof: Vec<String>,
    node_index: u64,
    leaf: String,
    tree_id: String,
}

impl RawProof {
    pub(super) fn normalize(self, limits: Limits) -> Result<DasAssetProof, Error> {
        if self.proof.len() > MAX_PROOF_DEPTH as usize
            || self.proof.len() > limits.max_items
            || self
                .proof
                .len()
                .checked_mul(NODE_BYTES)
                .ok_or(Error::Overflow)?
                > limits.max_bytes
        {
            return Err(Error::LimitExceeded);
        }

        let mut proof = Vec::new();
        proof
            .try_reserve_exact(self.proof.len())
            .map_err(|_| Error::AllocationFailed)?;

        for value in self.proof {
            proof.push(node(&value)?);
        }

        Ok(DasAssetProof {
            root: node(&self.root)?,
            proof,
            node_index: self.node_index,
            leaf: node(&self.leaf)?,
            tree: address(&self.tree_id)?,
        })
    }
}
