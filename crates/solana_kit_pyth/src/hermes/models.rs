//! Exact response data; parsing neither authenticates nor refreshes a price.
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use num_traits::{Signed, ToPrimitive};
use serde::{Deserialize, Deserializer, Serialize};
use solana_kit_core::fixed_points::BigInt;

use super::{Encoding, integer};

/// Signed component, confidence, decimal exponent and publication time.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Price {
    /// Exact signed component, serialized as a decimal string.
    #[serde(with = "integer")]
    pub price: BigInt,
    /// Exact recorded confidence, including signed values accepted by the reference.
    #[serde(rename = "conf", with = "integer")]
    pub confidence: BigInt,
    /// Power of ten shared by the price and confidence components.
    #[serde(rename = "expo")]
    pub exponent: i32,
    /// Unix publication time in seconds, retaining the full signed range.
    pub publish_time: i64,
}

impl Price {
    /// Returns the explicitly lossy component multiplied by `10^exponent`.
    /// Large components or exponents can yield infinity, zero or NaN. The exact
    /// `price` field remains unchanged; no work loops over the supplied exponent.
    #[must_use]
    pub fn approximate_f64(&self) -> f64 {
        let component = match self.price.to_f64() {
            Some(value) => value,
            None if self.price.is_negative() => f64::NEG_INFINITY,
            None => f64::INFINITY,
        };

        component * 10_f64.powi(self.exponent)
    }
}

/// Optional metadata attached to a parsed price update.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PriceMetadata {
    /// Pythnet slot, preserved as an exact unsigned integer.
    pub slot: Option<u64>,
    /// Previous price publication time in Unix seconds.
    pub prev_publish_time: Option<i64>,
    /// Earliest proof availability time in Unix seconds.
    pub proof_available_time: Option<i64>,
}

/// One parsed feed and its latest ordinary and exponential-moving-average prices.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PriceFeed {
    /// Provider-returned feed identifier, retained verbatim.
    pub id: String,
    /// Latest aggregate price and confidence components.
    pub price: Price,
    /// Latest exponential moving average components.
    pub ema_price: Price,
    /// Optional publication metadata; missing and null remain absent.
    pub metadata: Option<PriceMetadata>,
}

/// Binary chunks retain the encoding stated by the response.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BinaryUpdate {
    /// Text representation of each update chunk.
    pub encoding: Encoding,
    /// Ordered chunks; a missing or null list is empty, as in the reference.
    #[serde(default, deserialize_with = "null_default")]
    pub data: Vec<String>,
}

/// Complete price-update response, including optional parsed feed records.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PriceUpdate {
    /// Required binary envelope, independent of whether parsed feeds were requested.
    pub binary: BinaryUpdate,
    /// Returned parsed feeds; an empty list remains distinct from absence.
    pub parsed: Option<Vec<PriceFeed>>,
}

/// Metadata from the available price-feed listing.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PriceFeedMetadata {
    /// Provider-returned feed identifier.
    pub id: String,
    /// Extra string attributes. Equality and hashing include every entry.
    #[serde(default, deserialize_with = "null_default")]
    pub attributes: BTreeMap<String, String>,
    /// Optional provider asset category.
    pub asset_type: Option<String>,
    /// Base asset or token.
    pub base: Option<String>,
    /// Human-readable feed description.
    pub description: Option<String>,
    /// Display symbol.
    pub display_symbol: Option<String>,
    /// Generic symbol.
    pub generic_symbol: Option<String>,
    /// Quote asset or token.
    pub quote: Option<String>,
    /// Combined asset symbol.
    pub symbol: Option<String>,
}

/// Only missing or null collection fields receive their specified empty value.
fn null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}
