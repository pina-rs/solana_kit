//! Closed wire names shared by Hermes requests and returned binary records.
use serde::{Deserialize, Serialize};
use solana_kit_core::codecs::Error;

/// Encoding recorded on the returned binary update chunks.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Encoding {
    /// Hexadecimal text; also the default request encoding.
    #[default]
    Hex,
    /// Standard Base64 text.
    Base64,
}

impl Encoding {
    /// Exact query parameter and response wire name.
    #[must_use]
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::Hex => "hex",
            Self::Base64 => "base64",
        }
    }

    /// Resolves only the names supported by the pinned Hermes contract.
    ///
    /// # Errors
    /// Returns an invalid discriminator for every unknown or differently cased name.
    pub fn from_name(name: &str) -> Result<Self, Error> {
        match name {
            "hex" => Ok(Self::Hex),
            "base64" => Ok(Self::Base64),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
}

/// Asset categories accepted by the feed-listing filter.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    /// Listed company shares.
    Equity,
    /// Foreign exchange rates.
    Fx,
    /// Cryptocurrencies.
    Crypto,
    /// Precious metals.
    Metal,
    /// Interest rates.
    Rates,
    /// Crypto redemption rates.
    CryptoRedemptionRate,
    /// Commodities.
    Commodities,
    /// Crypto index prices.
    CryptoIndex,
    /// Crypto net asset value prices.
    CryptoNav,
    /// Ecology and weather observations.
    Eco,
    /// Kalshi event feeds.
    Kalshi,
}

impl AssetType {
    /// Exact `asset_type` query parameter value.
    #[must_use]
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::Equity => "equity",
            Self::Fx => "fx",
            Self::Crypto => "crypto",
            Self::Metal => "metal",
            Self::Rates => "rates",
            Self::CryptoRedemptionRate => "crypto_redemption_rate",
            Self::Commodities => "commodities",
            Self::CryptoIndex => "crypto_index",
            Self::CryptoNav => "crypto_nav",
            Self::Eco => "eco",
            Self::Kalshi => "kalshi",
        }
    }
}
