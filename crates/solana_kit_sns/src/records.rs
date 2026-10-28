//! SNS record derivation, headers, and content.

use crate::{
    CENTRAL_STATE_SNS_RECORDS, DataError, DomainError, SnsRecordVersion, find_domain_key,
    find_name_account_key, get_hashed_name,
};

const SOLANA_IDENTIFIER_LENGTH: usize = 32;
const EVM_IDENTIFIER_LENGTH: usize = 20;
const EVM_TEXT_LENGTH: usize = 2 + EVM_IDENTIFIER_LENGTH * 2;
const HEX_RADIX: u8 = 16;
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use solana_kit_core::Address;

/// A supported SNS record label.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnsRecord {
    /// IPFS content identifier.
    Ipfs,
    /// Arweave identifier.
    Arwv,
    /// Solana address.
    Sol,
    /// Ethereum address.
    Eth,
    /// Bitcoin address.
    Btc,
    /// Litecoin address.
    Ltc,
    /// Dogecoin address.
    Doge,
    /// Email address.
    Email,
    /// URL.
    Url,
    /// Discord handle.
    Discord,
    /// GitHub handle.
    Github,
    /// Reddit handle.
    Reddit,
    /// Twitter handle.
    Twitter,
    /// Telegram handle.
    Telegram,
    /// Picture URL.
    Pic,
    /// Shadow Drive identifier.
    Shdw,
    /// POINT identifier.
    Point,
    /// BNB Smart Chain address.
    Bsc,
    /// Injective address.
    Injective,
    /// Backpack identifier.
    Backpack,
    /// IPv4 address.
    Ipv4,
    /// IPv6 address.
    Aaaa,
    /// DNS canonical name.
    Cname,
    /// DNS text.
    Txt,
    /// Avatar background.
    Background,
    /// Base address.
    Base,
    /// IPNS identifier.
    Ipns,
    /// Biography.
    Bio,
}
impl SnsRecord {
    /// Returns the exact on-chain label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ipfs => "IPFS",
            Self::Arwv => "ARWV",
            Self::Sol => "SOL",
            Self::Eth => "ETH",
            Self::Btc => "BTC",
            Self::Ltc => "LTC",
            Self::Doge => "DOGE",
            Self::Email => "email",
            Self::Url => "url",
            Self::Discord => "discord",
            Self::Github => "github",
            Self::Reddit => "reddit",
            Self::Twitter => "twitter",
            Self::Telegram => "telegram",
            Self::Pic => "pic",
            Self::Shdw => "SHDW",
            Self::Point => "POINT",
            Self::Bsc => "BSC",
            Self::Injective => "INJ",
            Self::Backpack => "backpack",
            Self::Ipv4 => "A",
            Self::Aaaa => "AAAA",
            Self::Cname => "CNAME",
            Self::Txt => "TXT",
            Self::Background => "background",
            Self::Base => "BASE",
            Self::Ipns => "IPNS",
            Self::Bio => "bio",
        }
    }
    /// Whether the source client treats this record as UTF-8.
    fn is_text(self) -> bool {
        matches!(
            self,
            Self::Arwv
                | Self::Backpack
                | Self::Btc
                | Self::Cname
                | Self::Discord
                | Self::Doge
                | Self::Email
                | Self::Github
                | Self::Ipfs
                | Self::Ipns
                | Self::Ltc
                | Self::Pic
                | Self::Point
                | Self::Reddit
                | Self::Shdw
                | Self::Telegram
                | Self::Twitter
                | Self::Txt
                | Self::Url
                | Self::Bio
        )
    }
    /// Whether this is a hex-encoded EVM address.
    fn is_evm(self) -> bool {
        matches!(self, Self::Base | Self::Bsc | Self::Eth)
    }
}

/// Derives a V1 record address under a TLD-trimmed domain.
///
/// # Errors
/// Rejects unsupported names and PDA failures.
pub fn find_record_v1_address(domain: &str, record: SnsRecord) -> Result<Address, DomainError> {
    Ok(find_domain_key(
        &format!("{}.{domain}", record.label()),
        Some(SnsRecordVersion::V1),
    )?
    .address)
}

/// Derives a V2 record address under a TLD-trimmed domain or subdomain.
///
/// # Errors
/// Rejects unsupported names and PDA failures.
pub fn find_record_v2_address(domain: &str, record: SnsRecord) -> Result<Address, DomainError> {
    let parent = find_domain_key(domain, None)?.address;
    let hash = get_hashed_name(&format!("\u{2}{}", record.label()));
    Ok(find_name_account_key(
        &hash,
        Some(CENTRAL_STATE_SNS_RECORDS),
        Some(parent),
    )?)
}

/// Validation identifier format, without any claim that it is verified.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum SnsValidation {
    /// No identifier.
    None = 0,
    /// A 32-byte Solana identifier.
    Solana = 1,
    /// A 20-byte Ethereum identifier.
    Ethereum = 2,
    /// A 32-byte self-reported Solana identifier.
    UnverifiedSolana = 3,
}
impl SnsValidation {
    /// Identifier byte count for this mode.
    #[must_use]
    pub const fn identifier_length(self) -> usize {
        match self {
            Self::None => 0,
            Self::Ethereum => EVM_IDENTIFIER_LENGTH,
            Self::Solana | Self::UnverifiedSolana => SOLANA_IDENTIFIER_LENGTH,
        }
    }
}
impl TryFrom<u16> for SnsValidation {
    type Error = DataError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Solana),
            2 => Ok(Self::Ethereum),
            3 => Ok(Self::UnverifiedSolana),
            _ => Err(DataError::InvalidValidation),
        }
    }
}

/// V2 header length in bytes.
pub const RECORD_HEADER_LENGTH: usize = 8;
/// Offset of the V2 header in a full account.
pub const RECORD_DATA_OFFSET: usize = 96;

/// V2 record header containing two identifier formats and a content length.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SnsRecordHeader {
    /// Staleness identifier format.
    pub staleness_validation: SnsValidation,
    /// Association identifier format.
    pub right_of_association_validation: SnsValidation,
    /// Number of content bytes following identifiers.
    pub content_length: u32,
}
impl SnsRecordHeader {
    /// Reads the first header and returns the consumed byte count.
    ///
    /// # Errors
    /// Rejects truncated headers and unknown validation values.
    pub fn decode_prefix(bytes: &[u8]) -> Result<(Self, usize), DataError> {
        let bytes = bytes
            .get(..RECORD_HEADER_LENGTH)
            .ok_or(DataError::Truncated)?;
        let header = Self {
            staleness_validation: u16::from_le_bytes([bytes[0], bytes[1]]).try_into()?,
            right_of_association_validation: u16::from_le_bytes([bytes[2], bytes[3]]).try_into()?,
            content_length: u32::from_le_bytes(
                bytes[4..].try_into().map_err(|_| DataError::Truncated)?,
            ),
        };
        Ok((header, RECORD_HEADER_LENGTH))
    }
    /// Encodes the fixed-size header in little-endian order.
    #[must_use]
    pub fn encode(self) -> [u8; RECORD_HEADER_LENGTH] {
        let mut bytes = [0; RECORD_HEADER_LENGTH];
        bytes[..2].copy_from_slice(&(self.staleness_validation as u16).to_le_bytes());
        bytes[2..4].copy_from_slice(&(self.right_of_association_validation as u16).to_le_bytes());
        bytes[4..].copy_from_slice(&self.content_length.to_le_bytes());
        bytes
    }
}

/// Borrowed V2 account view. Payload ranges are checked only when requested.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SnsRecordV2<'a> {
    /// Decoded header.
    pub header: SnsRecordHeader,
    /// Identifier and content bytes after the header.
    pub data: &'a [u8],
}
impl<'a> SnsRecordV2<'a> {
    /// Reads a full account's header and retains its remaining payload.
    ///
    /// # Errors
    /// Rejects inputs shorter than 104 bytes and invalid header modes.
    pub fn deserialize(account: &'a [u8]) -> Result<Self, DataError> {
        let record = account
            .get(RECORD_DATA_OFFSET..)
            .ok_or(DataError::Truncated)?;
        let (header, end) = SnsRecordHeader::decode_prefix(record)?;
        Ok(Self {
            header,
            data: &record[end..],
        })
    }
    /// Reads the staleness identifier bytes without validating their meaning.
    ///
    /// # Errors
    /// Returns [`DataError::Truncated`] if the identifier is incomplete.
    pub fn staleness_id(&self) -> Result<&'a [u8], DataError> {
        self.data
            .get(..self.header.staleness_validation.identifier_length())
            .ok_or(DataError::Truncated)
    }
    /// Reads the association identifier bytes without verifying association.
    ///
    /// # Errors
    /// Returns [`DataError::Truncated`] if either identifier range is incomplete.
    pub fn roa_id(&self) -> Result<&'a [u8], DataError> {
        let start = self.header.staleness_validation.identifier_length();
        let end = start
            + self
                .header
                .right_of_association_validation
                .identifier_length();
        self.data.get(start..end).ok_or(DataError::Truncated)
    }
    /// Reads the declared content, ignoring trailing account allocation.
    ///
    /// # Errors
    /// Returns [`DataError::Truncated`] if the content range is incomplete.
    pub fn content(&self) -> Result<&'a [u8], DataError> {
        let start = self.header.staleness_validation.identifier_length()
            + self
                .header
                .right_of_association_validation
                .identifier_length();
        let end = start
            .checked_add(self.header.content_length as usize)
            .ok_or(DataError::LengthOverflow)?;
        self.data.get(start..end).ok_or(DataError::Truncated)
    }
}

/// Decodes supported V2 record content into text.
///
/// # Errors
/// Rejects malformed content and record types without a source-client decoder.
pub fn decode_record_content(content: &[u8], record: SnsRecord) -> Result<String, DataError> {
    if record.is_text() {
        let value = core::str::from_utf8(content).map_err(|_| DataError::InvalidUtf8)?;

        return Ok(value.strip_prefix('\u{feff}').unwrap_or(value).to_string());
    }

    if record == SnsRecord::Sol {
        let bytes = content
            .get(..SOLANA_IDENTIFIER_LENGTH)
            .ok_or(DataError::Truncated)?;
        let address = Address::try_from(bytes).map_err(|_| DataError::InvalidContent)?;

        return Ok(address.to_string());
    }

    if record.is_evm() {
        if content.len() != EVM_IDENTIFIER_LENGTH {
            return Err(DataError::InvalidContent);
        }

        let mut value = String::from("0x");
        use core::fmt::Write;

        for byte in content {
            write!(&mut value, "{byte:02x}").map_err(|_| DataError::InvalidContent)?;
        }

        return Ok(value);
    }

    Err(DataError::UnsupportedRecord)
}

/// Encodes text into supported V2 record content.
///
/// # Errors
/// Rejects malformed addresses and unsupported record types.
pub fn encode_record_content(content: &str, record: SnsRecord) -> Result<Vec<u8>, DataError> {
    if record.is_text() {
        return Ok(content.as_bytes().to_vec());
    }

    if record == SnsRecord::Sol {
        let address: Address = content.parse().map_err(|_| DataError::InvalidContent)?;

        return Ok(address.as_bytes().to_vec());
    }

    if record.is_evm() {
        let bytes = content.as_bytes();

        if bytes.len() != EVM_TEXT_LENGTH || !bytes.starts_with(b"0x") {
            return Err(DataError::InvalidContent);
        }

        let mut result = Vec::with_capacity(EVM_IDENTIFIER_LENGTH);

        for pair in bytes[2..].as_chunks::<2>().0 {
            let high = hex_digit(pair[0]).ok_or(DataError::InvalidContent)?;
            let low = hex_digit(pair[1]).ok_or(DataError::InvalidContent)?;
            result.push(high * HEX_RADIX + low);
        }

        return Ok(result);
    }

    Err(DataError::UnsupportedRecord)
}

/// Decodes a hexadecimal digit from an ASCII byte.
fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
