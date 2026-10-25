//! Pure metadata presentation, payload and edition-bit helpers.
use crate::{Data, EditionMarker, Payload, PayloadType};
use alloc::{string::String, vec::Vec};
use solana_kit_core::codecs::{Error, Limits};

/// Maximum UTF-8 byte length of a metadata name accepted by the program.
pub const MAX_NAME_LENGTH: usize = 32;
/// Maximum UTF-8 byte length of a metadata symbol accepted by the program.
pub const MAX_SYMBOL_LENGTH: usize = 10;
/// Maximum UTF-8 byte length of a metadata URI accepted by the program.
pub const MAX_URI_LENGTH: usize = 200;
/// Edition slots in the fixed legacy marker ledger.
pub const EDITION_MARKER_BIT_SIZE: u64 = 248;

/// Validates program string limits without changing the original on-wire strings.
/// # Errors
/// Rejects overlong UTF-8 bytes. Creator and program-state policy remains caller owned.
pub fn validate_metadata_strings(data: &Data) -> Result<(), Error> {
    if data.name.len() > MAX_NAME_LENGTH
        || data.symbol.len() > MAX_SYMBOL_LENGTH
        || data.uri.len() > MAX_URI_LENGTH
    {
        return Err(Error::LimitExceeded);
    }

    Ok(())
}

/// Presentation view of a puffed string, retaining embedded NULs and original storage.
pub fn unpuffed(value: &str) -> &str {
    value.trim_end_matches('\0')
}

/// Tests the most-significant-bit-first position inside the edition's marker.
pub fn edition_is_marked(marker: &EditionMarker, edition: u64) -> bool {
    let position = edition % EDITION_MARKER_BIT_SIZE;
    let byte = usize::try_from(position / 8).expect("marker byte is below31");
    let bit = u32::try_from(position % 8).expect("marker bit is below8");

    marker.ledger[byte] & (0x80_u8 >> bit) != 0
}

/// Constructs an insertion-ordered payload with unique keys and an explicit item budget.
/// # Errors
/// Rejects duplicate keys, excessive keys or excessive aggregate key bytes.
pub fn authorization_payload(
    entries: Vec<(String, PayloadType)>,
    limits: Limits,
) -> Result<Payload, Error> {
    if entries.len() > limits.max_items {
        return Err(Error::LimitExceeded);
    }

    let mut bytes = 0usize;

    for (index, (key, _)) in entries.iter().enumerate() {
        bytes = bytes.checked_add(key.len()).ok_or(Error::Overflow)?;

        if bytes > limits.max_bytes {
            return Err(Error::LimitExceeded);
        }

        if entries[..index].iter().any(|(prior, _)| prior == key) {
            return Err(Error::InvalidValue);
        }
    }

    Ok(Payload { map: entries })
}
