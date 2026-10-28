//! Canonical hexadecimal bytes used by Surfpool account and deployment calls.

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

/// Owned bytes serialized as a lowercase hexadecimal string without a prefix.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HexBytes(#[doc = "Owned unencoded bytes."] pub Vec<u8>);

impl Serialize for HexBytes {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for HexBytes {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        decode(&text).map(Self).map_err(D::Error::custom)
    }
}

/// Produces the exact lowercase byte representation used by the pinned source.
pub fn encode(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len().saturating_mul(2));

    for byte in bytes {
        text.push(char::from(DIGITS[usize::from(byte >> 4)]));
        text.push(char::from(DIGITS[usize::from(byte & 15)]));
    }

    text
}

/// Rejects odd-length or noncanonical hexadecimal rather than losing byte data.
pub fn decode(text: &str) -> Result<Vec<u8>, InvalidHex> {
    if !text.len().is_multiple_of(2) {
        return Err(InvalidHex);
    }

    text.as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = nibble(pair[0])?;
            let low = nibble(pair[1])?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn nibble(byte: u8) -> Result<u8, InvalidHex> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(InvalidHex),
    }
}

/// A hexadecimal string is odd-length, uppercase, prefixed, or contains nonhex bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidHex;

impl core::fmt::Display for InvalidHex {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("expected an even-length lowercase hexadecimal string")
    }
}

impl std::error::Error for InvalidHex {}
