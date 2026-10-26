//! Account queries select one address or hash before reaching the RPC boundary.

use serde::{Deserialize, Serialize};

/// Identifies one compressed account by its assigned address or leaf hash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", try_from = "LocatorFields")]
pub enum AccountLocator {
    /// Query an account that has an assigned address.
    Address(String),
    /// Query a leaf by its hash, including accounts without an assigned address.
    Hash(String),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LocatorFields {
    address: Option<String>,
    hash: Option<String>,
}

impl TryFrom<LocatorFields> for AccountLocator {
    type Error = &'static str;

    fn try_from(value: LocatorFields) -> Result<Self, Self::Error> {
        match (value.address, value.hash) {
            (Some(address), None) => Ok(Self::Address(address)),
            (None, Some(hash)) => Ok(Self::Hash(hash)),
            _ => Err("provide exactly one compressed account address or hash"),
        }
    }
}

/// Batch account lookup uses addresses or hashes while preserving response positions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", try_from = "BatchLocatorFields")]
pub enum GetMultipleCompressedAccountsRequest {
    /// Look up each assigned address.
    Addresses(Vec<String>),
    /// Look up each leaf hash.
    Hashes(Vec<String>),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BatchLocatorFields {
    addresses: Option<Vec<String>>,
    hashes: Option<Vec<String>>,
}

impl TryFrom<BatchLocatorFields> for GetMultipleCompressedAccountsRequest {
    type Error = &'static str;

    fn try_from(value: BatchLocatorFields) -> Result<Self, Self::Error> {
        match (value.addresses, value.hashes) {
            (Some(addresses), None) => Ok(Self::Addresses(addresses)),
            (None, Some(hashes)) => Ok(Self::Hashes(hashes)),
            _ => Err("provide exactly one compressed account address list or hash list"),
        }
    }
}
