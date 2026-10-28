#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod build;
mod client;
mod convert;
mod error;
mod integer;
mod lookup;
mod models;
mod object;

mod order;
mod order_request;
mod requests;
mod scalar;
mod swap;

use solana_kit_client::rest::Budgets;
use solana_kit_core::Address;
use std::collections::BTreeMap;

pub use build::{AccountMetaPayload, BlockhashWithMetadata, BuildResponse, InstructionPayload};
pub use client::{Client, Service};
pub use convert::decode_base64_swap_transaction;
pub use error::Error;
pub use integer::Integer;
pub use lookup::{LookupTablePayload, LookupTables};
pub use models::{Price, TokenItem};
pub use order::{ExecutionResponse, OrderMode, OrderResponse};
pub use order_request::{OrderRequest, SwapMode};
pub use requests::{Category, Interval, TokenLimit};
pub use swap::{decode_build, decode_execution, decode_order};

/// The reference service root; the application adapter approves origins and authentication.
pub const DEFAULT_BASE_URL: &str = "https://api.jup.ag";
/// Maximum mint count in one Price API v3 request.
pub const MAX_PRICE_MINTS: usize = 50;

/// Decodes a Price API v3 object after complete shared JSON admission.
///
/// Missing prices remain omitted. Keys must be canonical Solana addresses.
/// No authenticity, freshness or suitability of a returned price is established.
pub fn decode_prices(bytes: &[u8], budgets: Budgets) -> Result<BTreeMap<Address, Price>, Error> {
    budgets.validate_json(bytes).map_err(Error::Json)?;
    let prices: BTreeMap<String, Price> = serde_json::from_slice(bytes).map_err(Error::Schema)?;
    prices
        .into_iter()
        .map(|(mint, price)| {
            mint.parse()
                .map(|address| (address, price))
                .map_err(|_| Error::InvalidMint)
        })
        .collect()
}

/// Decodes the ordered Token API v2 array after complete shared JSON admission.
///
/// Known malformed fields are rejected. Missing and null optional fields remain
/// unavailable; integers and prices never pass through a binary float.
pub fn decode_tokens(bytes: &[u8], budgets: Budgets) -> Result<Vec<TokenItem>, Error> {
    budgets.validate_json(bytes).map_err(Error::Json)?;
    serde_json::from_slice(bytes).map_err(Error::Schema)
}
