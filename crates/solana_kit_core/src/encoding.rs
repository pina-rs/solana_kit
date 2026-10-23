//! Bounded base58 parsing shared by the fixed-byte public values.

use core::fmt;

/// Domain that owns a fixed-byte parse failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueKind {
    /// Account or program address.
    Address,
    /// Hash or blockhash.
    Hash,
    /// Ed25519 signature.
    Signature,
}

/// A fixed-byte value could not be decoded from its external representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The text contains a character outside the Bitcoin base58 alphabet.
    InvalidBase58 {
        /// Value boundary that rejected the alphabet.
        kind: ValueKind,
    },
    /// The decoded bytes do not have the required fixed length.
    WrongLength {
        /// Value boundary that rejected the decoded length.
        kind: ValueKind,
        /// Required number of bytes.
        expected: usize,
        /// Actual number of decoded bytes.
        actual: usize,
    },
    /// The text is longer than any encoding of this value can be.
    EncodedLengthExceeded {
        /// Value boundary that rejected the encoded length.
        kind: ValueKind,
        /// Maximum accepted number of UTF-8 bytes.
        maximum: usize,
        /// Actual number of UTF-8 bytes.
        actual: usize,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBase58 { .. } => formatter.write_str("invalid base58 character"),
            Self::WrongLength {
                expected, actual, ..
            } => {
                write!(formatter, "expected {expected} bytes, received {actual}")
            }
            Self::EncodedLengthExceeded {
                maximum, actual, ..
            } => {
                write!(
                    formatter,
                    "encoded length {actual} exceeds maximum {maximum}"
                )
            }
        }
    }
}

impl core::error::Error for ParseError {}

/// Bounds allocation before decoding untrusted base58 text.
pub(crate) fn decode<const N: usize>(
    text: &str,
    maximum: usize,
    kind: ValueKind,
) -> Result<[u8; N], ParseError> {
    if text.len() > maximum {
        return Err(ParseError::EncodedLengthExceeded {
            kind,
            maximum,
            actual: text.len(),
        });
    }

    let bytes = bs58::decode(text)
        .into_vec()
        .map_err(|_| ParseError::InvalidBase58 { kind })?;

    from_slice(&bytes, kind)
}

/// Rejects both truncated and trailing bytes at a fixed-value boundary.
pub(crate) fn from_slice<const N: usize>(
    bytes: &[u8],
    kind: ValueKind,
) -> Result<[u8; N], ParseError> {
    bytes.try_into().map_err(|_| ParseError::WrongLength {
        kind,
        expected: N,
        actual: bytes.len(),
    })
}
