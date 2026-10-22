//! Typed filters make ambiguous maps and invalid encoded filter bytes unnecessary.
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Serialize, Serializer};

/// Representation used for a validated byte comparison.
#[derive(Debug, Clone, Copy)]
pub enum FilterEncoding {
    /// Base58-encoded comparison.
    Base58,
    /// Base64-encoded comparison.
    Base64,
    /// Literal byte array.
    Bytes,
}
/// Validated comparison of at most128 bytes at a byte offset.
#[derive(Debug, Clone)]
pub struct Memcmp {
    offset: u64,
    bytes: Vec<u8>,
    encoding: FilterEncoding,
}
impl Memcmp {
    /// Rejects oversized filters before a request can reach a transport.
    pub fn new(
        offset: u64,
        bytes: Vec<u8>,
        encoding: FilterEncoding,
    ) -> Result<Self, crate::Error> {
        if bytes.len() > 128 || offset.checked_add(bytes.len() as u64).is_none() {
            return Err(crate::Error::AccountData);
        }

        Ok(Self {
            offset,
            bytes,
            encoding,
        })
    }
}
impl Serialize for Memcmp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Encoded<'a> {
            offset: u64,
            encoding: &'a str,
            bytes: String,
        }
        #[derive(Serialize)]
        struct Raw<'a> {
            offset: u64,
            encoding: &'a str,
            bytes: &'a [u8],
        }

        match self.encoding {
            FilterEncoding::Base58 => Encoded {
                offset: self.offset,
                encoding: "base58",
                bytes: bs58::encode(&self.bytes).into_string(),
            }
            .serialize(serializer),
            FilterEncoding::Base64 => Encoded {
                offset: self.offset,
                encoding: "base64",
                bytes: STANDARD.encode(&self.bytes),
            }
            .serialize(serializer),
            FilterEncoding::Bytes => Raw {
                offset: self.offset,
                encoding: "bytes",
                bytes: &self.bytes,
            }
            .serialize(serializer),
        }
    }
}
/// Conjunctive program account filter.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Filter {
    /// Complete allocation must match this size.
    DataSize(u64),
    /// Validated offset and byte comparison.
    Memcmp(Memcmp),
    /// Restrict to initialized token accounts recognized by the node.
    TokenAccountState,
}
