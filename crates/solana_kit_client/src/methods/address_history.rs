//! Address transaction pages use the shared transaction decoder, not wire verification.
mod filters;
pub use filters::*;

use super::{
    Method,
    history::{HistoricalCommitment, SignatureHistory},
    transaction_results::{
        TransactionEncoding, TransactionResult, VersionRequirement, WireTransactionResult, parse,
        schema,
    },
};
use crate::{Error, Presence};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};
use serde_json::value::RawValue;
use solana_kit_core::Address;

/// Full transaction options, unavailable in signatures-only mode.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FullOptions {
    /// Omission selects JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<TransactionEncoding>,
    /// Highest numbered transaction version accepted, zero or one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_supported_transaction_version: Option<u8>,
}
/// Request shape prevents full-only options in signatures mode.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(tag = "transactionDetails", rename_all = "lowercase")]
pub enum AddressDetails {
    /// Complete owned transaction views.
    Full(FullOptions),
    /// Signatures and historical metadata only.
    Signatures,
}
/// Complete optional address-history configuration.
#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddressHistoryConfig {
    /// Historical commitment, excluding processed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<HistoricalCommitment>,
    /// AND-combined server filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AddressFilters>,
    /// One through 100 full entries or 1000 signatures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /// Minimum evaluation slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    /// Continuation supplied by a previous response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<PaginationToken>,
    /// Default is descending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    /// Omission preserves the signatures-only server default.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub details: Option<AddressDetails>,
}
impl AddressHistoryConfig {
    fn options(&self) -> Option<FullOptions> {
        match self.details {
            None | Some(AddressDetails::Signatures) => None,
            Some(AddressDetails::Full(options)) => Some(options),
        }
    }
    fn cap(&self) -> usize {
        if self.options().is_some() { 100 } else { 1000 }
    }
    fn validate(&self) -> Result<(), Error> {
        if self
            .limit
            .is_some_and(|limit| limit == 0 || usize::from(limit) > self.cap())
        {
            return Err(schema("address history limit exceeds selected detail cap"));
        }

        if self
            .options()
            .is_some_and(|o| o.max_supported_transaction_version.is_some_and(|v| v > 1))
        {
            return Err(schema("supported transaction version exceeds one"));
        }

        Ok(())
    }
}
/// One page; provider support is required and no automatic pagination occurs.
#[derive(Debug)]
pub struct TransactionsForAddress(pub Address, pub Option<AddressHistoryConfig>);
impl Serialize for TransactionsForAddress {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(None)?;
        seq.serialize_element(&self.0.to_string())?;

        if let Some(config) = &self.1 {
            config.validate().map_err(serde::ser::Error::custom)?;
            seq.serialize_element(config)?;
        }

        seq.end()
    }
}
/// Full address-history entry with mandatory block position.
#[derive(Debug)]
pub struct AddressTransaction {
    /// Containing slot.
    pub slot: u64,
    /// Nullable estimated Unix time.
    pub block_time: Option<i64>,
    /// Required position within this block.
    pub transaction_index: u32,
    /// Shared owned transaction graph.
    pub details: TransactionResult,
}
/// Explicit page detail variant selected by the request.
#[derive(Debug)]
pub enum AddressTransactions {
    /// Full transactions.
    Full(Vec<AddressTransaction>),
    /// Historical signatures; every position is required here.
    Signatures(Vec<SignatureHistory>),
}
/// Bounded page with a validated optional continuation cursor.
#[derive(Debug)]
pub struct AddressPage {
    /// Request-selected entries.
    pub data: AddressTransactions,
    /// Null signals no further results.
    pub pagination_token: Option<PaginationToken>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WirePage {
    data: Vec<Box<RawValue>>,
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pagination_token: Option<PaginationToken>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Position {
    slot: u64,
    #[serde(deserialize_with = "crate::json::required_nullable")]
    block_time: Option<i64>,
    transaction_index: u32,
}
impl Method for TransactionsForAddress {
    type Response = AddressPage;
    const NAME: &'static str = "getTransactionsForAddress";
    fn decode(&self, json: &str) -> Result<Self::Response, Error> {
        let default = AddressHistoryConfig::default();
        let config = self.1.as_ref().unwrap_or(&default);
        config.validate()?;
        let page: WirePage = parse(json)?;
        let limit = config.limit.map_or(config.cap(), usize::from);

        if page.data.len() > limit {
            return Err(schema("address history response exceeds requested limit"));
        }

        let data = match config.options() {
            Some(options) => AddressTransactions::Full(
                page.data
                    .into_iter()
                    .map(|raw| decode_full(raw.get(), options))
                    .collect::<Result<_, Error>>()?,
            ),
            None => AddressTransactions::Signatures(
                page.data
                    .into_iter()
                    .map(|raw| decode_signature(raw.get()))
                    .collect::<Result<_, Error>>()?,
            ),
        };

        Ok(AddressPage {
            data,
            pagination_token: page.pagination_token,
        })
    }
}
fn decode_full(raw: &str, options: FullOptions) -> Result<AddressTransaction, Error> {
    let position: Position = parse(raw)?;
    let details = parse::<WireTransactionResult>(raw)?.decode(
        options.encoding.unwrap_or_default(),
        options.max_supported_transaction_version,
        false,
        VersionRequirement::RequiredWhenRequested,
    )?;

    Ok(AddressTransaction {
        slot: position.slot,
        block_time: position.block_time,
        transaction_index: position.transaction_index,
        details,
    })
}
fn decode_signature(raw: &str) -> Result<SignatureHistory, Error> {
    let entry: SignatureHistory = parse(raw)?;

    if matches!(entry.transaction_index, Presence::Missing) {
        return Err(schema("address history requires transactionIndex"));
    }

    Ok(entry)
}
