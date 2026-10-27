//! Hermes response records for off-chain applications on every maintained host.
//!
//! Prices and confidence components remain arbitrary-precision integers. The
//! bounded entry points check JSON byte, nesting, item and string budgets before
//! typed allocation, reject duplicate keys and cap each integer at 8192 bytes.
//! Decoding establishes no price authenticity, freshness or account ownership.

mod client;
mod enums;
mod error;
mod integer;
mod models;
mod policy;
mod requests;

use alloc::vec::Vec;
use serde::de::DeserializeOwned;
use solana_kit_client::rest::{Budgets, JsonError};

pub use client::{Client, Service};
pub use enums::{AssetType, Encoding};
pub use error::Error;
pub use integer::MAX_INTEGER_BYTES;
pub use models::{BinaryUpdate, Price, PriceFeed, PriceFeedMetadata, PriceMetadata, PriceUpdate};
pub use policy::{ConfigError, DEFAULT_BASE_URL, MAX_HTTP_RETRIES, Policy};
pub use requests::UpdateOptions;

/// Decodes a price-update response after complete shared JSON admission.
///
/// # Errors
/// Rejects exceeded budgets, duplicate keys, malformed fields, unknown encoding
/// or an integer component exceeding `MAX_INTEGER_BYTES`.
pub fn decode_price_update(bytes: &[u8], budgets: Budgets) -> Result<PriceUpdate, JsonError> {
    decode(bytes, budgets)
}

/// Decodes the ordered price-feed listing after complete shared JSON admission.
///
/// # Errors
/// Rejects exceeded budgets, duplicate keys or malformed metadata fields.
pub fn decode_price_feeds(
    bytes: &[u8],
    budgets: Budgets,
) -> Result<Vec<PriceFeedMetadata>, JsonError> {
    decode(bytes, budgets)
}

/// Admission precedes serde allocation for both maintained response shapes.
fn decode<T: DeserializeOwned>(bytes: &[u8], budgets: Budgets) -> Result<T, JsonError> {
    budgets.validate_json(bytes)?;
    serde_json::from_slice(bytes).map_err(JsonError::Schema)
}
