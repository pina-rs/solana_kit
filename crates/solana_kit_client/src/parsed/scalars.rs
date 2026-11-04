//! Exact scalar admission shared by the parsed account families.

pub(super) use crate::methods::accounts::data::address as required_address;

use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;
use solana_kit_core::Address;

/// Requires unsigned base-10 text and parses it without a floating-point path.
pub(super) fn unsigned_decimal<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    let text = String::deserialize(d)?;

    parse_unsigned(&text).map_err(serde::de::Error::custom)
}

/// Accepts an absent or null optional amount of unsigned base-10 text.
pub(super) fn optional_unsigned_decimal<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<u64>, D::Error> {
    Option::<String>::deserialize(d)?
        .map(|text| parse_unsigned(&text).map_err(serde::de::Error::custom))
        .transpose()
}

/// Accepts an absent or null optional address.
pub(super) fn optional_address<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<Address>, D::Error> {
    Option::<String>::deserialize(d)?
        .map(|text| text.parse().map_err(serde::de::Error::custom))
        .transpose()
}

/// Accepts an absent or null extension list while requiring an array shape.
pub(super) fn optional_raw_array<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<Box<RawValue>>, D::Error> {
    match Option::<Box<RawValue>>::deserialize(d)? {
        Some(raw) if raw.get().trim_start().starts_with('[') => Ok(Some(raw)),
        Some(_) => Err(serde::de::Error::custom("extension array required")),
        None => Ok(None),
    }
}

/// Requires every member of an address array to be a complete address.
pub(super) fn addresses<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Address>, D::Error> {
    Vec::<String>::deserialize(d)?
        .into_iter()
        .map(|text| text.parse().map_err(serde::de::Error::custom))
        .collect()
}

fn parse_unsigned(text: &str) -> Result<u64, &'static str> {
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("unsigned decimal integer required");
    }

    text.parse()
        .map_err(|_| "unsigned decimal integer exceeds u64")
}
