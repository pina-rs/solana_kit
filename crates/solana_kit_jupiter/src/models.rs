//! Typed nullable records from the pinned Price v3 and Tokens v2 contracts.

use crate::scalar;
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// A Price API v3 record, with exact decimal values and an opaque block identifier.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    /// USD price as an exact JSON number; absent when the provider has none.
    #[serde(default, deserialize_with = "scalar::number")]
    pub usd_price: Option<Number>,
    /// Provider block identifier, accepting the reference string or integer encoding.
    #[serde(default, deserialize_with = "scalar::text_or_integer")]
    pub block_id: Option<String>,
    /// Token decimal count; rejects values outside the unsigned eight-bit domain.
    #[serde(default, deserialize_with = "scalar::integer")]
    pub decimals: Option<u8>,
    /// Exact 24-hour price change, when available.
    #[serde(default, deserialize_with = "scalar::number")]
    pub price_change24h: Option<Number>,
}

/// A Token API v2 search, tag, category or recent-list record.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenItem {
    /// Provider mint text; callers may explicitly parse it as a Kit address.
    pub id: Option<String>,
    /// Display name.
    pub name: Option<String>,
    /// Display symbol.
    pub symbol: Option<String>,
    /// Icon reference; decoding performs no fetch.
    pub icon: Option<String>,
    /// Token decimal count, accepting an integer or decimal integer string.
    #[serde(default, deserialize_with = "scalar::integer")]
    pub decimals: Option<u8>,
    /// Provider tags in their original order.
    pub tags: Option<Vec<String>>,
    /// Exact organic score, when available.
    #[serde(default, deserialize_with = "scalar::number")]
    pub organic_score: Option<Number>,
    /// Provider score label.
    pub organic_score_label: Option<String>,
    /// Provider verification claim; this library does not authenticate that claim.
    pub is_verified: Option<bool>,
    /// Holder count, accepting an unsigned integer or decimal integer string.
    #[serde(default, deserialize_with = "scalar::integer")]
    pub holder_count: Option<u64>,
    /// Exact USD price, accepting number and numeric-string wire variants.
    #[serde(default, deserialize_with = "scalar::number_or_string")]
    pub usd_price: Option<Number>,
    /// Exact market capitalization, accepting number and numeric-string wire variants.
    #[serde(default, deserialize_with = "scalar::number_or_string")]
    pub mcap: Option<Number>,
    /// Exact liquidity, accepting number and numeric-string wire variants.
    #[serde(default, deserialize_with = "scalar::number_or_string")]
    pub liquidity: Option<Number>,
}

impl std::fmt::Debug for Price {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Price { .. }")
    }
}

impl std::fmt::Debug for TokenItem {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("TokenItem { .. }")
    }
}
