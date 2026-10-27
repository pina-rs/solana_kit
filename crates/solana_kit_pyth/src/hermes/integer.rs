//! Decimal-string serde preserves exact integers instead of routing through f64.
use alloc::string::ToString;
use serde::{Deserialize, Deserializer, Serializer, de::Error as _};
use solana_kit_core::fixed_points::BigInt;

/// Maximum text bytes admitted before parsing a Hermes integer component.
pub const MAX_INTEGER_BYTES: usize = 8192;

/// Canonical response serialization always emits a decimal string.
pub(super) fn serialize<S: Serializer>(value: &BigInt, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}

/// Accepts exact integer JSON tokens and strings with bounded conversion work.
pub(super) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<BigInt, D::Error> {
    let text = match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::String(text) => text,
        serde_json::Value::Number(number) => number.to_string(),
        _ => return Err(D::Error::custom("Expected an integer component")),
    };

    if text.len() > MAX_INTEGER_BYTES {
        return Err(D::Error::custom("Integer component byte budget exceeded"));
    }

    let digits = match text.as_bytes() {
        [b'-' | b'+', digits @ ..] => digits,
        digits => digits,
    };

    if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) {
        return Err(D::Error::custom("Invalid decimal integer component"));
    }

    text.parse()
        .map_err(|_| D::Error::custom("Invalid decimal integer component"))
}
