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
    #[serde(default, deserialize_with = "optional_program")]
    pub program: String,
    /// Program-defined structure retained without numeric normalization.
    pub parsed: Box<RawValue>,
    /// Complete account allocation, independent of encoding size.
    #[serde(default)]
    pub space: Option<u64>,
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
    #[serde(default)]
    rent_epoch: crate::Presence<u64>,
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
        if matches!(self.rent_epoch, crate::Presence::Missing) {
            return Err(Error::AccountData);
        }

        let data = decode(self.data.get(), config, Some(self.space))?;

        Ok(self.snapshot(address, data))
    }

    fn snapshot(self, address: Address, data: AccountData) -> Account<AccountData> {
        Account {
            address,
            owner: self.owner,
            lamports: self.lamports,
            executable: self.executable,
            space: self.space,
            rent_epoch: match self.rent_epoch {
                crate::Presence::Missing => None,
                crate::Presence::Present(epoch) => Some(epoch),
            },
            data,
        }
    }
}

fn decode(raw: &str, config: AccountConfig, space: Option<u64>) -> Result<AccountData, Error> {
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

fn parsed(raw: &str, config: AccountConfig, space: Option<u64>) -> Result<AccountData, Error> {
    if config.encoding != Some(AccountEncoding::JsonParsed) || config.data_slice.is_some() {
        return Err(Error::AccountData);
    }

    let parsed: ParsedAccount = serde_json::from_str(raw).map_err(Error::Schema)?;

    if let Some(space) = space {
        #[derive(Deserialize)]
        struct RequiredMetadata {
            program: String,
            space: u64,
        }

        let required: RequiredMetadata = serde_json::from_str(raw).map_err(Error::Schema)?;

        if required.space != space || required.program != parsed.program {
            return Err(Error::AccountData);
        }
    }

    Ok(AccountData::Parsed(parsed))
}

fn tagged(raw: &str, config: AccountConfig, space: Option<u64>) -> Result<AccountData, Error> {
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
    space: Option<u64>,
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

    let expected = space.map(|space| {
        config.data_slice.map_or(space, |slice| {
            space.saturating_sub(slice.offset).min(slice.length)
        })
    });

    if expected.is_some_and(|expected| u64::try_from(bytes.len()).ok() != Some(expected)) {
        return Err(Error::AccountData);
    }

    Ok(AccountData::Bytes {
        bytes,
        slice: config.data_slice,
    })
}

pub(crate) fn parse_account(
    address: Address,
    raw: &str,
    encoding: AccountEncoding,
) -> Result<MaybeAccount<AccountData>, Error> {
    #[derive(Deserialize)]
    struct Snapshot {
        #[serde(flatten)]
        header: crate::accounts::BaseAccount,
        data: Box<RawValue>,
    }

    let wire: Option<Snapshot> = serde_json::from_str(raw).map_err(Error::Schema)?;
    let Some(wire) = wire else {
        return Ok(MaybeAccount::Missing(address));
    };
    let config = AccountConfig {
        encoding: Some(encoding),
        ..AccountConfig::default()
    };
    let raw = wire.data.get();
    let data = if raw.trim_start().starts_with('"') {
        let text: String = serde_json::from_str(raw).map_err(Error::Schema)?;
        binary(&text, encoding, config, None)?
    } else {
        decode(raw, config, None)?
    };

    Ok(MaybeAccount::Present(Account {
        address,
        owner: wire.header.owner,
        lamports: wire.header.lamports,
        executable: wire.header.executable,
        space: wire.header.space,
        rent_epoch: match wire.header.rent_epoch {
            crate::Presence::Missing => None,
            crate::Presence::Present(epoch) => Some(epoch),
        },
        data,
    }))
}

pub(crate) fn metadata_number<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Number {
        Integer(u64),
        Text(String),
    }

    match Number::deserialize(d)? {
        Number::Integer(value) => Ok(value),
        Number::Text(text) => parse_metadata_number(&text).ok_or_else(|| {
            serde::de::Error::custom("account metadata must be an unsigned 64-bit integer")
        }),
    }
}

fn parse_metadata_number(text: &str) -> Option<u64> {
    let text = text.trim_matches(metadata_whitespace);
    let text = text.strip_prefix('+').unwrap_or(text);
    let (radix, digits) = text
        .strip_prefix("0x")
        .or_else(|| text.strip_prefix("0X"))
        .map_or((10, text), |digits| (16, digits));

    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }

    u64::from_str_radix(digits, radix).ok()
}

fn metadata_whitespace(value: char) -> bool {
    matches!(value, '\u{0009}'..='\u{000d}' | ' ' | '\u{00a0}' | '\u{1680}'
        | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}'
        | '\u{205f}' | '\u{3000}' | '\u{feff}')
}

fn optional_program<'de, D: serde::Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    Ok(Option::<String>::deserialize(d)?.unwrap_or_default())
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
