//! Provider token amounts retain both their wire representation and decimal precision.

use crate::Decimal;
use serde::{Deserialize, Deserializer, Serialize};

/// A provider token amount, preserving whether the wire used a number or string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum TokenAmount {
    /// An exact JSON number, including fractional UI quantities.
    Number(Decimal),
    /// The original string, without an implicit numeric conversion.
    String(String),
}

impl<'de> Deserialize<'de> for TokenAmount {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Inspect raw JSON before Serde's untagged buffering can erase numeric spelling
        // or interpret a user object as arbitrary-precision number internals.
        let raw = Box::<serde_json::value::RawValue>::deserialize(deserializer)?;

        match raw.get().as_bytes().first() {
            Some(b'"') => serde_json::from_str(raw.get()).map(Self::String),
            Some(b'-' | b'0'..=b'9') => serde_json::from_str(raw.get()).map(Self::Number),
            _ => {
                return Err(serde::de::Error::custom(
                    "expected a number or string token amount",
                ));
            }
        }
        .map_err(serde::de::Error::custom)
    }
}
