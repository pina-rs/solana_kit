//! Leaf-canonical royalties preserve the display value and explicit raw precedence.
use crate::{Creator, MetadataArgs, MetadataArgsV2};
use alloc::{string::String, vec::Vec};
use solana_kit_core::codecs::Error;

/// A leaf whose royalty rate is inherited from its Core collection.
pub const SELLER_FEE_BASIS_POINTS_INHERIT: u16 = u16::MAX;

impl MetadataArgs {
    /// Constructs ordinary V1 NFT metadata with the source's explicit defaults.
    #[must_use]
    pub fn nft(name: String, uri: String, creators: Vec<Creator>) -> Self {
        Self {
            name,
            symbol: String::new(),
            uri,
            seller_fee_basis_points: 0,
            primary_sale_happened: false,
            is_mutable: true,
            edition_nonce: None,
            token_standard: Some(crate::TokenStandard::NonFungible),
            collection: None,
            uses: None,
            token_program_version: crate::TokenProgramVersion::Original,
            creators,
        }
    }
}

/// DAS raw companions used for leaf hashing rather than the displayed royalties.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RoyaltyRawFields {
    /// Explicit leaf rate. None preserves the metadata rate.
    pub seller_fee_basis_points: Option<u16>,
    /// Explicit leaf creators. Some empty is distinct from None.
    pub creators: Option<Vec<Creator>>,
}

/// Returns an owned leaf value without mutating display-facing metadata.
#[must_use]
pub fn to_leaf_metadata(metadata: &MetadataArgs, raw: &RoyaltyRawFields) -> MetadataArgs {
    let mut leaf = metadata.clone();
    leaf.seller_fee_basis_points = raw
        .seller_fee_basis_points
        .unwrap_or(metadata.seller_fee_basis_points);
    leaf.creators = raw
        .creators
        .clone()
        .unwrap_or_else(|| metadata.creators.clone());
    leaf
}

/// Resolves the raw royalty companions on V2 metadata.
#[must_use]
pub fn to_leaf_metadata_v2(metadata: &MetadataArgsV2, raw: &RoyaltyRawFields) -> MetadataArgsV2 {
    let mut leaf = metadata.clone();
    leaf.seller_fee_basis_points = raw
        .seller_fee_basis_points
        .unwrap_or(metadata.seller_fee_basis_points);
    leaf.creators = raw
        .creators
        .clone()
        .unwrap_or_else(|| metadata.creators.clone());
    leaf
}

/// Converts V1 to the exact V2 wire shape, retaining the collection address.
#[must_use]
pub fn metadata_to_v2(metadata: &MetadataArgs, raw: &RoyaltyRawFields) -> MetadataArgsV2 {
    let leaf = to_leaf_metadata(metadata, raw);
    MetadataArgsV2 {
        name: leaf.name,
        symbol: leaf.symbol,
        uri: leaf.uri,
        seller_fee_basis_points: leaf.seller_fee_basis_points,
        primary_sale_happened: leaf.primary_sale_happened,
        is_mutable: leaf.is_mutable,
        token_standard: leaf.token_standard,
        creators: leaf.creators,
        collection: leaf.collection.map(|collection| collection.key),
    }
}

/// Checks the byte-sized metadata constraints before instruction construction.
/// # Errors
/// Rejects oversized UTF-8, invalid rates, creator shares and duplicate addresses.
pub fn validate_metadata(
    name: &str,
    symbol: &str,
    uri: &str,
    fee: u16,
    creators: &[Creator],
) -> Result<(), Error> {
    if name.len() > 32 || symbol.len() > 10 || uri.len() > 200 || creators.len() > 5 {
        return Err(Error::LimitExceeded);
    }

    if fee > 10_000 {
        return Err(Error::InvalidValue);
    }

    let mut shares = 0u16;

    for (index, creator) in creators.iter().enumerate() {
        if creators[..index]
            .iter()
            .any(|other| other.address == creator.address)
        {
            return Err(Error::InvalidValue);
        }

        shares = shares
            .checked_add(u16::from(creator.share))
            .ok_or(Error::Overflow)?;
    }

    if !creators.is_empty() && shares != 100 {
        return Err(Error::InvalidValue);
    }

    Ok(())
}

/// Checks V2 metadata, including its collection-inherited royalty sentinel.
/// # Errors
/// Rejects invalid byte limits, rates, duplicate creators, shares or inherited leaf creators.
pub fn validate_metadata_v2(
    name: &str,
    symbol: &str,
    uri: &str,
    fee: u16,
    creators: &[Creator],
) -> Result<(), Error> {
    if fee == SELLER_FEE_BASIS_POINTS_INHERIT {
        if !creators.is_empty() {
            return Err(Error::InvalidValue);
        }

        return validate_metadata(name, symbol, uri, 0, creators);
    }

    validate_metadata(name, symbol, uri, fee, creators)
}
