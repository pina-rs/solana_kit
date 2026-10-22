//! Exact80-byte nonce account snapshots, including uninitialized storage.

use crate::SYSTEM_PROGRAM;
use solana_kit_core::{
    Address, Hash,
    accounts::{Account, AccountError, EncodedAccount},
    codecs::{Decoder, Encoder, Error, FixedBytes, Limits, Number, Reader, Writer},
};

/// Allocated nonce account size, including version and state tags.
pub const NONCE_SIZE: usize = 80;

/// Historical and domain-separated nonce versions use identical wire layouts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum NonceVersion {
    /// Historical blockhash-domain nonce; not usable for current durable signing.
    Legacy = 0,
    /// Domain-separated durable nonce.
    Current = 1,
}

/// Initialized nonce fields; balances remain exact integer wire units.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NonceData {
    /// Authority required to advance or authorize this nonce.
    pub authority: Address,
    /// Stored nonce hash, distinct from a live recent blockhash.
    pub blockhash: Hash,
    /// Full-width fee per signature in lamports.
    pub lamports_per_signature: u64,
}

/// Initialization controls whether stored payload bytes have semantic meaning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NonceState {
    /// Unused allocated storage retained byte-for-byte; no authority or hash exists.
    Uninitialized([u8; 72]),
    /// Initialized authority, nonce hash and fee.
    Initialized(NonceData),
}

/// A full allocated nonce account snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Nonce {
    /// Version remains explicit; decoding never upgrades historical nonce hashes.
    pub version: NonceVersion,
    /// Initialization state and its corresponding payload.
    pub state: NonceState,
}

impl Nonce {
    /// Returns usable nonce fields only for initialized Current snapshots.
    #[must_use]
    pub const fn durable_data(&self) -> Option<&NonceData> {
        match (&self.version, &self.state) {
            (NonceVersion::Current, NonceState::Initialized(data)) => Some(data),
            _ => None,
        }
    }
}

/// Bounded exact allocated-account codec; unlike bincode prefixes it requires80bytes.
#[derive(Clone, Copy, Debug)]
pub struct NonceCodec;

impl Encoder<Nonce> for NonceCodec {
    fn encoded_len(&self, _: &Nonce) -> Result<usize, Error> {
        Ok(NONCE_SIZE)
    }

    fn write(&self, value: &Nonce, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u32>::LE.write(&(value.version as u32), writer)?;

        match &value.state {
            NonceState::Uninitialized(storage) => {
                Number::<u32>::LE.write(&0, writer)?;
                FixedBytes::<72>.write(storage, writer)
            }
            NonceState::Initialized(data) => {
                Number::<u32>::LE.write(&1, writer)?;
                FixedBytes::<32>.write(data.authority.as_bytes(), writer)?;
                FixedBytes::<32>.write(data.blockhash.as_bytes(), writer)?;
                Number::<u64>::LE.write(&data.lamports_per_signature, writer)
            }
        }
    }
}

impl Decoder<Nonce> for NonceCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Nonce, Error> {
        let version = match Number::<u32>::LE.read(reader)? {
            0 => NonceVersion::Legacy,
            1 => NonceVersion::Current,
            _ => return Err(Error::InvalidDiscriminator),
        };

        let state = match Number::<u32>::LE.read(reader)? {
            0 => NonceState::Uninitialized(FixedBytes::<72>.read(reader)?),
            1 => NonceState::Initialized(NonceData {
                authority: Address::from_bytes(FixedBytes::<32>.read(reader)?),
                blockhash: Hash::from_bytes(FixedBytes::<32>.read(reader)?),
                lamports_per_signature: Number::<u64>::LE.read(reader)?,
            }),
            _ => return Err(Error::InvalidDiscriminator),
        };

        Ok(Nonce { version, state })
    }
}

/// Decodes a System-owned80-byte account while preserving all snapshot metadata.
pub fn decode_nonce(
    account: EncodedAccount,
    limits: Limits,
) -> Result<Account<Nonce>, AccountError> {
    account.decode(&NonceCodec, SYSTEM_PROGRAM, limits)
}
