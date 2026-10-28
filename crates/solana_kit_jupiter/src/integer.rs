//! Arbitrary precision decimal integers without arithmetic or binary floating point.

use crate::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use serde_json::value::RawValue;
use std::{fmt, str::FromStr};

/// An exact signed decimal integer, independent of the host's numeric width.
///
/// Parsing accepts an optional minus sign and decimal digits. Leading zeroes
/// normalize away; fractional, exponent, whitespace and hexadecimal forms reject.
/// Serialization uses a string, as required for execution expiry heights.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Integer(String);

impl Integer {
    /// Borrows the canonical decimal representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Integer {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let digits = text.strip_prefix('-').unwrap_or(text);

        if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Error::InvalidInteger);
        }

        let significant = digits.trim_start_matches('0');

        if significant.is_empty() {
            return Ok(Self("0".into()));
        }

        let sign = if text.starts_with('-') { "-" } else { "" };
        Ok(Self(format!("{sign}{significant}")))
    }
}

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Integer { .. }")
    }
}

impl Serialize for Integer {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Integer {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(decoder)?;
        let text = raw.get();

        if text.starts_with('"') {
            let value: String = serde_json::from_str(text).map_err(D::Error::custom)?;

            return value.parse().map_err(D::Error::custom);
        }

        text.parse().map_err(D::Error::custom)
    }
}
