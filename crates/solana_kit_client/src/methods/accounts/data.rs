//! The account encoding boundary never claims compressed bytes are decoded.
use super::{AccountConfig, DataSlice};
use crate::Error;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use solana_kit_core::{
    Address,
    accounts::{Account, MaybeAccount},
};

/// Node account representations, including the legacy base58 string format.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AccountEncoding {
    /// Legacy base58 string, rather than a tagged pair.
    Binary,
    /// Tagged base58 bytes.
    Base58,
    /// Canonical padded base64 bytes.
    Base64,
    /// Program-defined parsed JSON, with base64 fallback for unknown programs.
    JsonParsed,
    /// Base64-wrapped Zstandard bytes; decompression is a separate capability.
    #[serde(rename = "base64+zstd")]
    Base64Zstd,
}

/// Typed parsed-account metadata with an exact bounded program-defined payload.
#[derive(Debug, Deserialize)]
pub struct ParsedAccount {
    /// Node parser's program name.
    pub program: String,
    /// Program-defined structure retained without numeric normalization.
    pub parsed: Box<RawValue>,
    /// Complete account allocation, independent of encoding size.
    pub space: u64,
}

/// Account data with its representation and slice boundary explicit.
#[derive(Debug)]
pub enum AccountData {
    /// Decoded binary bytes and the requested range; allocation stays in metadata.
    Bytes {
        /// Exact bytes returned by the node.
        bytes: Vec<u8>,
        /// Requested range, absent for a full snapshot.
        slice: Option<DataSlice>,
    },
    /// Encoded compressed bytes. No decompression or length assertion is implied.
    Zstd {
        /// Exact decoded base64 compressed stream.
        compressed: Vec<u8>,
        /// Range applied by the node before compression.
        slice: Option<DataSlice>,
    },
    /// Program-defined parsed data with typed metadata.
    Parsed(ParsedAccount),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WireAccount {
    executable: bool,
    lamports: u64,
    #[serde(deserialize_with = "address")]
    owner: Address,
    rent_epoch: u64,
    space: u64,
    data: Box<RawValue>,
}

pub(crate) fn address<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Address, D::Error> {
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}

impl WireAccount {
    pub(crate) fn owned(
        self,
        address: Address,
        config: AccountConfig,
    ) -> Result<Account<AccountData>, Error> {
        let data = decode(self.data.get(), config, self.space)?;
        Ok(Account {
            address,
            owner: self.owner,
            lamports: self.lamports,
            executable: self.executable,
            space: self.space,
            rent_epoch: Some(self.rent_epoch),
            data,
        })
    }
}

fn decode(raw: &str, config: AccountConfig, space: u64) -> Result<AccountData, Error> {
    let expected = config.encoding.unwrap_or(AccountEncoding::Binary);

    match raw.trim_start().as_bytes().first() {
        Some(b'{') => parsed(raw, config, space),

        Some(b'"') if expected == AccountEncoding::Binary => {
            let text: String = serde_json::from_str(raw).map_err(Error::Schema)?;
            binary(&text, AccountEncoding::Base58, config, space)
        }

        Some(b'[') => tagged(raw, config, space),
        _ => Err(Error::AccountData),
    }
}

fn parsed(raw: &str, config: AccountConfig, space: u64) -> Result<AccountData, Error> {
    if config.encoding != Some(AccountEncoding::JsonParsed) || config.data_slice.is_some() {
        return Err(Error::AccountData);
    }

    let parsed: ParsedAccount = serde_json::from_str(raw).map_err(Error::Schema)?;

    if parsed.space != space {
        return Err(Error::AccountData);
    }

    Ok(AccountData::Parsed(parsed))
}

fn tagged(raw: &str, config: AccountConfig, space: u64) -> Result<AccountData, Error> {
    let expected = config.encoding.unwrap_or(AccountEncoding::Binary);
    let (text, encoding): (String, AccountEncoding) =
        serde_json::from_str(raw).map_err(Error::Schema)?;
    let parsed_fallback =
        expected == AccountEncoding::JsonParsed && encoding == AccountEncoding::Base64;

    if encoding != expected && !parsed_fallback {
        return Err(Error::AccountData);
    }

    binary(&text, encoding, config, space)
}

fn binary(
    text: &str,
    encoding: AccountEncoding,
    config: AccountConfig,
    space: u64,
) -> Result<AccountData, Error> {
    let bytes = match encoding {
        AccountEncoding::Base58 => bs58::decode(text)
            .into_vec()
            .map_err(|_| Error::AccountData)?,
        AccountEncoding::Base64 | AccountEncoding::Base64Zstd => {
            STANDARD.decode(text).map_err(|_| Error::AccountData)?
        }
        _ => return Err(Error::AccountData),
    };

    if encoding == AccountEncoding::Base64Zstd {
        return Ok(AccountData::Zstd {
            compressed: bytes,
            slice: config.data_slice,
        });
    }

    let expected = config.data_slice.map_or(space, |slice| {
        space.saturating_sub(slice.offset).min(slice.length)
    });

    if u64::try_from(bytes.len()).ok() != Some(expected) {
        return Err(Error::AccountData);
    }

    Ok(AccountData::Bytes {
        bytes,
        slice: config.data_slice,
    })
}

pub(super) fn maybe(
    wire: Option<WireAccount>,
    address: Address,
    config: AccountConfig,
) -> Result<MaybeAccount<AccountData>, Error> {
    match wire {
        Some(wire) => Ok(MaybeAccount::Present(wire.owned(address, config)?)),
        None => Ok(MaybeAccount::Missing(address)),
    }
}
