//! Registry accounts and length-prefixed name values.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;
use solana_kit_core::Address;

/// Size of the three-address registry header.
pub const NAME_REGISTRY_HEADER_LENGTH: usize = 96;

/// An account, content, or encoded value could not be decoded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataError {
    /// Input ends before the requested field.
    Truncated,
    /// Encoded text is invalid UTF-8.
    InvalidUtf8,
    /// A validation mode is not recognized.
    InvalidValidation,
    /// The requested record has no text encoding in the source client.
    UnsupportedRecord,
    /// An address or hexadecimal string is invalid.
    InvalidContent,
    /// The value is too large for its u32 length prefix.
    LengthOverflow,
}
impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SNS data error: {self:?}")
    }
}
impl core::error::Error for DataError {}

/// Registry header and the remaining application-defined bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NameRegistryState {
    /// Parent name account.
    pub parent_name: Address,
    /// Owner account.
    pub owner: Address,
    /// Name class.
    pub registry_class: Address,
    /// Bytes after the fixed header.
    pub data: Vec<u8>,
}
impl NameRegistryState {
    /// Reads a complete registry account, including all trailing data.
    ///
    /// # Errors
    /// Returns [`DataError::Truncated`] for a header shorter than 96 bytes.
    pub fn decode(bytes: &[u8]) -> Result<Self, DataError> {
        let header = bytes
            .get(..NAME_REGISTRY_HEADER_LENGTH)
            .ok_or(DataError::Truncated)?;
        let address = |start| {
            let mut array = [0; 32];
            array.copy_from_slice(&header[start..start + 32]);
            Address::from_bytes(array)
        };
        Ok(Self {
            parent_name: address(0),
            owner: address(32),
            registry_class: address(64),
            data: bytes[NAME_REGISTRY_HEADER_LENGTH..].to_vec(),
        })
    }
    /// Encodes the complete registry account.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(NAME_REGISTRY_HEADER_LENGTH + self.data.len());
        bytes.extend_from_slice(self.parent_name.as_bytes());
        bytes.extend_from_slice(self.owner.as_bytes());
        bytes.extend_from_slice(self.registry_class.as_bytes());
        bytes.extend_from_slice(&self.data);
        bytes
    }
    /// Whether the account has data following its header.
    #[must_use]
    pub fn has_data(&self) -> bool {
        !self.data.is_empty()
    }
}

/// Encodes a UTF-8 string preceded by its little-endian u32 byte length.
///
/// # Errors
/// Returns [`DataError::LengthOverflow`] when the length cannot fit in u32.
pub fn encode_name_value(value: &str) -> Result<Vec<u8>, DataError> {
    let len = u32::try_from(value.len()).map_err(|_| DataError::LengthOverflow)?;
    let size = 4_usize
        .checked_add(value.len())
        .ok_or(DataError::LengthOverflow)?;
    let mut bytes = Vec::with_capacity(size);
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.extend_from_slice(value.as_bytes());
    Ok(bytes)
}

/// Reads one length-prefixed UTF-8 value and returns the consumed byte count.
/// A leading UTF-8 BOM is removed, matching the source decoder.
///
/// # Errors
/// Returns a truncation or invalid UTF-8 error.
pub fn decode_name_value_prefix(bytes: &[u8]) -> Result<(String, usize), DataError> {
    let length = bytes.get(..4).ok_or(DataError::Truncated)?;
    let mut raw = [0; 4];
    raw.copy_from_slice(length);
    let end = 4_usize
        .checked_add(u32::from_le_bytes(raw) as usize)
        .ok_or(DataError::LengthOverflow)?;
    let value = bytes.get(4..end).ok_or(DataError::Truncated)?;
    let value = core::str::from_utf8(value).map_err(|_| DataError::InvalidUtf8)?;
    Ok((
        value.strip_prefix('\u{feff}').unwrap_or(value).to_string(),
        end,
    ))
}

/// Reads a name value while ignoring trailing account allocation bytes.
///
/// # Errors
/// Returns a truncation or invalid UTF-8 error.
pub fn decode_name_value(bytes: &[u8]) -> Result<String, DataError> {
    Ok(decode_name_value_prefix(bytes)?.0)
}

/// Encodes a reverse value, optionally marking a subdomain with a NUL.
///
/// # Errors
/// Returns [`DataError::LengthOverflow`] for oversized values.
pub fn encode_reverse_value(value: &str, is_subdomain: bool) -> Result<Vec<u8>, DataError> {
    if is_subdomain {
        return encode_name_value(&alloc::format!("\0{value}"));
    }

    encode_name_value(value)
}

/// Decodes a reverse value, optionally removing its first NUL marker.
///
/// # Errors
/// Returns a truncation or invalid UTF-8 error.
pub fn decode_reverse_value(bytes: &[u8], trim_leading_null: bool) -> Result<String, DataError> {
    let value = decode_name_value(bytes)?;

    if trim_leading_null {
        return Ok(value.strip_prefix('\0').unwrap_or(&value).to_string());
    }

    Ok(value)
}
