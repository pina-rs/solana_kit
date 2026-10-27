//! Anchor/Borsh price-update accounts, independently of Solana's Rust crates.
use alloc::string::String;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Endian, Error, FixedBytes, Limits, Number, Reader},
};

use crate::{PythPriceFeedMessage, messages::read_price_message};

/// Anchor account discriminator of `PriceUpdateV2`.
pub const PRICE_UPDATE_V2_DISCRIMINATOR: [u8; 8] = [0x22, 0xf1, 0x23, 0x63, 0x9d, 0x7e, 0xf4, 0xcd];
/// Serialized prefix when the verification level is `Full`.
pub const PRICE_UPDATE_V2_FULL_SIZE: usize = 133;
/// Serialized prefix for `Partial`; also the program's maximum account allocation.
pub const PRICE_UPDATE_V2_PARTIAL_SIZE: usize = 134;

/// Recorded guardian verification level. Parsing does not verify signatures.
/// `Full` orders above every partial level, regardless of its signature count.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PythVerificationLevel {
    /// Only the recorded number of guardian signatures was checked.
    Partial {
        /// Recorded guardian signature count.
        num_signatures: u8,
    },
    /// The account records verification against the guardian quorum.
    Full,
}

impl PythVerificationLevel {
    /// Whether the recorded level is full verification.
    #[must_use]
    pub const fn is_full(self) -> bool {
        matches!(self, Self::Full)
    }

    /// Partial signature count; full verification has no numeric count.
    #[must_use]
    pub const fn num_signatures(self) -> Option<u8> {
        match self {
            Self::Partial { num_signatures } => Some(num_signatures),
            Self::Full => None,
        }
    }
}

/// Decoded receiver account. The price fields share the exact wire-message type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PriceUpdateV2Account<'a> {
    /// Authority recorded as able to rewrite or close this account.
    pub write_authority: Address,
    /// Verification level recorded by the receiver.
    pub verification_level: PythVerificationLevel,
    /// Feed identifier, price, confidence, exponent, timestamps and EMA values.
    pub price_message: PythPriceFeedMessage,
    /// Slot at which the update was posted.
    pub posted_slot: u64,
    /// Remaining account bytes, including the full-variant allocation padding.
    pub extension: &'a [u8],
}

impl PriceUpdateV2Account<'_> {
    /// Returns the feed identifier as 64 lowercase hexadecimal digits.
    /// Use this textual form when matching an account to a Hermes feed.
    ///
    /// # Errors
    /// Returns a codec allocation error if the string cannot be allocated.
    pub fn feed_id_hex(&self) -> Result<String, Error> {
        self.price_message.feed_id_hex()
    }
}

/// Decodes the variant's actual Borsh prefix and borrows any account tail.
/// Full verification needs 133 bytes, while partial verification needs 134.
/// This function allocates nothing and requires no item budget.
///
/// # Errors
/// Rejects a bad discriminator, unknown verification variant, truncated fields
/// or an exceeded byte limit. Account ownership, freshness and authenticity
/// remain the caller's responsibility.
pub fn decode_price_update_v2_account(
    bytes: &[u8],
    limits: Limits,
) -> Result<PriceUpdateV2Account<'_>, Error> {
    let mut reader = Reader::new(bytes, limits)?;

    if reader.take(PRICE_UPDATE_V2_DISCRIMINATOR.len())? != PRICE_UPDATE_V2_DISCRIMINATOR {
        return Err(Error::InvalidConstant {
            offset: 0,
            length: PRICE_UPDATE_V2_DISCRIMINATOR.len(),
        });
    }

    let write_authority = Address::from_bytes(FixedBytes::<32>.read(&mut reader)?);

    let verification_level = match Number::<u8>::LE.read(&mut reader)? {
        0 => PythVerificationLevel::Partial {
            num_signatures: Number::<u8>::LE.read(&mut reader)?,
        },
        1 => PythVerificationLevel::Full,
        _ => return Err(Error::InvalidDiscriminator),
    };

    let price_message = read_price_message(&mut reader, Endian::Little)?;
    let posted_slot = Number::<u64>::LE.read(&mut reader)?;

    Ok(PriceUpdateV2Account {
        write_authority,
        verification_level,
        price_message,
        posted_slot,
        extension: reader.remaining(),
    })
}
