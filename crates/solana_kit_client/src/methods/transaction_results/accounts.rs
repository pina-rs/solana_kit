//! Transaction account annotations and lookup references.
use super::{address, addresses};
use crate::Presence;
use serde::Deserialize;
use solana_kit_core::Address;

/// Origin of an account resolved by the node.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AccountSource {
    /// Static transaction account.
    Transaction,
    /// Account loaded from an address lookup table.
    LookupTable,
}
/// Account privileges in a parsed or accounts-only transaction result.
#[derive(Debug, Deserialize)]
pub struct ParsedAccount {
    /// Validated account address.
    #[serde(deserialize_with = "address")]
    pub pubkey: Address,
    /// Whether this transaction may modify the account.
    pub writable: bool,
    /// Whether this account signed the transaction.
    pub signer: bool,
    /// Source annotation is versioned and can be absent or null on older nodes.
    #[serde(default)]
    pub source: Presence<Option<AccountSource>>,
}
/// Indices into one address lookup table.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressTableLookup {
    /// Address lookup table account.
    #[serde(deserialize_with = "address")]
    pub account_key: Address,
    /// Writable table entry positions.
    pub writable_indexes: Vec<u8>,
    /// Readonly table entry positions.
    pub readonly_indexes: Vec<u8>,
}
/// Addresses resolved from lookup tables in execution order.
#[derive(Debug, Deserialize)]
pub struct LoadedAddresses {
    /// Writable loaded accounts.
    #[serde(deserialize_with = "addresses")]
    pub writable: Vec<Address>,
    /// Readonly loaded accounts.
    #[serde(deserialize_with = "addresses")]
    pub readonly: Vec<Address>,
}
