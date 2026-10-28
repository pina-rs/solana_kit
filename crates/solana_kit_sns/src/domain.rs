//! Offline name, record, and reverse address derivations.

use crate::{CENTRAL_STATE_SNS_RECORDS, NAME_PROGRAM, REVERSE_LOOKUP_CLASS, SNS_ROOT_DOMAIN};
use alloc::{format, string::ToString};
use core::fmt;
use solana_kit_core::{
    Address, Hash,
    crypto::{self, PdaError},
};

/// Prefix hashed before every SNS name.
pub const SNS_HASH_PREFIX: &str = "SPL Name Service";

/// Hashes arbitrary bytes with SHA-256.
#[must_use]
pub fn sha256(bytes: &[u8]) -> Hash {
    crypto::hash(&[bytes])
}

/// Hashes a UTF-8 name using the SNS prefix.
#[must_use]
pub fn get_hashed_name(name: &str) -> Hash {
    crypto::hash(&[SNS_HASH_PREFIX.as_bytes(), name.as_bytes()])
}

/// Record label prefix and class selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnsRecordVersion {
    /// V1 record.
    V1,
    /// V2 record.
    V2,
}

impl SnsRecordVersion {
    /// Byte preceding a record label before hashing.
    #[must_use]
    pub const fn prefix(self) -> u8 {
        match self {
            Self::V1 => 1,
            Self::V2 => 2,
        }
    }
}

/// A name's address, hash, and parent context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SnsDomainKey {
    /// Name account address.
    pub address: Address,
    /// Hash used in derivation.
    pub hash: Hash,
    /// Whether the input has multiple labels.
    pub is_sub: bool,
    /// Whether this is a record of a subdomain.
    pub is_sub_record: bool,
    /// Root-level parent domain for multi-label inputs.
    pub parent_address: Option<Address>,
}

/// Invalid input or PDA derivation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DomainError {
    /// Names must be nonempty, TLD-trimmed, and have a supported label count.
    InvalidDomain,
    /// The core PDA operation failed.
    Pda(PdaError),
}
impl From<PdaError> for DomainError {
    fn from(error: PdaError) -> Self {
        Self::Pda(error)
    }
}
impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDomain => f.write_str("invalid TLD-trimmed SNS domain"),
            Self::Pda(error) => error.fmt(f),
        }
    }
}
impl core::error::Error for DomainError {}

/// Finds the name PDA from a hash and optional class and parent.
///
/// # Errors
/// Returns the core PDA error if derivation fails.
pub fn find_name_account_key(
    hash: &Hash,
    class: Option<Address>,
    parent: Option<Address>,
) -> Result<Address, PdaError> {
    let class = class.unwrap_or(Address::from_bytes([0; 32]));
    let parent = parent.unwrap_or(Address::from_bytes([0; 32]));
    Ok(crypto::find_program_address(
        &[hash.as_bytes(), class.as_bytes(), parent.as_bytes()],
        &NAME_PROGRAM,
    )?
    .address())
}

/// Hashes a name and derives its PDA with the supplied class and parent.
///
/// # Errors
/// Returns the core PDA error if derivation fails.
pub fn derive_name_address(
    name: &str,
    parent: Option<Address>,
    class: Option<Address>,
) -> Result<(Address, Hash), PdaError> {
    let hash = get_hashed_name(name);
    Ok((find_name_account_key(&hash, class, parent)?, hash))
}

/// Derives a TLD-trimmed domain, subdomain, or record name.
///
/// # Errors
/// Rejects empty labels, explicit `.sol` or `.sns` suffixes, unsupported
/// nesting, and PDA derivation errors.
pub fn find_domain_key(
    domain: &str,
    record: Option<SnsRecordVersion>,
) -> Result<SnsDomainKey, DomainError> {
    let labels: alloc::vec::Vec<_> = domain.split('.').collect();

    validate_labels(&labels, record)?;

    let (parent, parent_hash) =
        derive_name_address(labels[labels.len() - 1], Some(SNS_ROOT_DOMAIN), None)?;

    if labels.len() == 1 {
        return Ok(SnsDomainKey {
            address: parent,
            hash: parent_hash,
            is_sub: false,
            is_sub_record: false,
            parent_address: None,
        });
    }
    let direct_parent = if labels.len() == 3 {
        derive_name_address(&format!("\0{}", labels[1]), Some(parent), None)?.0
    } else {
        parent
    };

    let prefix = record.map_or(0, SnsRecordVersion::prefix) as char;
    let class = (record == Some(SnsRecordVersion::V2)).then_some(CENTRAL_STATE_SNS_RECORDS);
    let (address, hash) = derive_name_address(
        &format!("{prefix}{}", labels[0]),
        Some(direct_parent),
        class,
    )?;
    Ok(SnsDomainKey {
        address,
        hash,
        is_sub: true,
        is_sub_record: labels.len() == 3,
        parent_address: Some(parent),
    })
}

/// Rejects unsupported label shapes before address derivation.
fn validate_labels(labels: &[&str], record: Option<SnsRecordVersion>) -> Result<(), DomainError> {
    if labels.iter().any(|label| label.is_empty())
        || (labels.len() > 1 && matches!(labels.last(), Some(&"sol" | &"sns")))
        || labels.len() > 3
        || (labels.len() == 3 && record.is_none())
    {
        return Err(DomainError::InvalidDomain);
    }

    Ok(())
}

/// Derives the reverse account from a domain address and optional parent.
///
/// # Errors
/// Returns the core PDA error if derivation fails.
pub fn find_reverse_address(domain: Address, parent: Option<Address>) -> Result<Address, PdaError> {
    find_name_account_key(
        &get_hashed_name(&domain.to_string()),
        Some(REVERSE_LOOKUP_CLASS),
        parent,
    )
}

/// Derives the reverse account of a TLD-trimmed domain.
///
/// # Errors
/// Returns an invalid-domain or PDA derivation error.
pub fn find_reverse_address_for_domain(domain: &str) -> Result<Address, DomainError> {
    let key = find_domain_key(domain, None)?;
    Ok(find_reverse_address(key.address, key.parent_address)?)
}
