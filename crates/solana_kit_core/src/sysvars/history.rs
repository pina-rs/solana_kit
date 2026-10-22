//! Counted historical layouts with shared allocation limits.

use super::{HashCodec, SysvarCodec, layout};

const LENGTH_PREFIX_SIZE: usize = 8;
const HASH_ENTRY_SIZE: usize = 40;
const STAKE_ENTRY_SIZE: usize = 32;
use crate::{
    Hash,
    codecs::{Decoder, Encoder, Error, Number, Reader, Writer},
};
use alloc::vec::Vec;

/// Required bit-vector discriminator.
pub const BITVEC_DISCRIMINATOR: u8 = 1;
/// Number of tracked slot bits.
pub const BITVEC_NUM_BITS: usize = 1024 * 1024;
/// Number of u64 bit-vector blocks.
pub const BITVEC_LENGTH: usize = BITVEC_NUM_BITS / 64;
/// Fixed slot history encoded size.
pub const SLOT_HISTORY_SIZE: usize = 1 + 8 + BITVEC_LENGTH * 8 + 8 + 8;

/// A parent bank hash at a slot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SlotHashEntry {
    /// Slot number.
    pub slot: u64,
    /// Parent bank hash.
    pub hash: Hash,
}
layout!(
    SlotHashEntry,
    40,
    slot: Number::<u64>::LE,
    hash: HashCodec,
);

/// Historical signature fee calculation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FeeCalculator {
    /// Lamports charged per signature.
    pub lamports_per_signature: u64,
}
layout!(
    FeeCalculator,
    8,
    lamports_per_signature: Number::<u64>::LE,
);

/// Historical blockhash and signature fee, retained for old snapshots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecentBlockhashEntry {
    /// Recent block hash.
    pub blockhash: Hash,
    /// Historical signature fee.
    pub fee_calculator: FeeCalculator,
}
layout!(
    RecentBlockhashEntry,
    40,
    blockhash: HashCodec,
    fee_calculator: SysvarCodec::<FeeCalculator>::NEW,
);

/// Stake activation amounts for an epoch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StakeHistoryData {
    /// Effective stake in lamports.
    pub effective: u64,
    /// Stake warming up in lamports.
    pub activating: u64,
    /// Stake cooling down in lamports.
    pub deactivating: u64,
}
layout!(
    StakeHistoryData,
    24,
    effective: Number::<u64>::LE,
    activating: Number::<u64>::LE,
    deactivating: Number::<u64>::LE,
);

/// One epoch's stake activation history.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StakeHistoryEntry {
    /// Epoch number.
    pub epoch: u64,
    /// Stake activation values.
    pub stake_history: StakeHistoryData,
}
layout!(
    StakeHistoryEntry,
    32,
    epoch: Number::<u64>::LE,
    stake_history: SysvarCodec::<StakeHistoryData>::NEW,
);

/// A u64-prefixed vector of parent bank hashes.
pub type SlotHashes = Vec<SlotHashEntry>;
/// A historical u64-prefixed blockhash/fee vector.
pub type RecentBlockhashes = Vec<RecentBlockhashEntry>;
/// A u64-prefixed vector of epoch stake history.
pub type StakeHistory = Vec<StakeHistoryEntry>;

/// Reads a known-width vector only after validating its count and input size.
fn read_entries<T>(reader: &mut Reader<'_>, count: usize, size: usize) -> Result<Vec<T>, Error>
where
    SysvarCodec<T>: Decoder<T>,
{
    let bytes = count.checked_mul(size).ok_or(Error::Overflow)?;

    if bytes > reader.remaining().len() {
        return Err(Error::Truncated);
    }

    reader.claim(count)?;
    let mut entries = Vec::new();
    entries
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;

    for _ in 0..count {
        entries.push(SysvarCodec::<T>::NEW.read(reader)?);
    }

    Ok(entries)
}

/// Generates concrete vector implementations without exposing an unbounded parser.
macro_rules! vector_layout {
    ($ty:ty, $size:expr) => {
        impl Encoder<Vec<$ty>> for SysvarCodec<Vec<$ty>> {
            fn encoded_len(&self, value: &Vec<$ty>) -> Result<usize, Error> {
                value
                    .len()
                    .checked_mul($size)
                    .and_then(|size| size.checked_add(LENGTH_PREFIX_SIZE))
                    .ok_or(Error::Overflow)
            }
            fn write(&self, value: &Vec<$ty>, writer: &mut Writer<'_>) -> Result<(), Error> {
                Number::<u64>::LE.write(
                    &u64::try_from(value.len()).map_err(|_| Error::Overflow)?,
                    writer,
                )?;

                write_entries(value, writer)?;

                Ok(())
            }
        }
        impl Decoder<Vec<$ty>> for SysvarCodec<Vec<$ty>> {
            fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<$ty>, Error> {
                let count = usize::try_from(Number::<u64>::LE.read(reader)?)
                    .map_err(|_| Error::Overflow)?;
                read_entries(reader, count, $size)
            }
        }
    };
}
vector_layout!(SlotHashEntry, HASH_ENTRY_SIZE);
vector_layout!(RecentBlockhashEntry, HASH_ENTRY_SIZE);
vector_layout!(StakeHistoryEntry, STAKE_ENTRY_SIZE);

/// Writes typed entries through their composed layouts.
fn write_entries<T>(entries: &[T], writer: &mut Writer<'_>) -> Result<(), Error>
where
    SysvarCodec<T>: Encoder<T>,
{
    for entry in entries {
        SysvarCodec::<T>::NEW.write(entry, writer)?;
    }

    Ok(())
}

/// A fixed million-bit history and the next newer slot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlotHistory {
    /// Exactly BITVEC_LENGTH little-endian u64 blocks.
    pub bits: Vec<u64>,
    /// Slot immediately newer than the tracked history.
    pub next_slot: u64,
}

impl Encoder<SlotHistory> for SysvarCodec<SlotHistory> {
    fn encoded_len(&self, value: &SlotHistory) -> Result<usize, Error> {
        if value.bits.len() != BITVEC_LENGTH {
            return Err(Error::InvalidValue);
        }

        Ok(SLOT_HISTORY_SIZE)
    }
    fn write(&self, value: &SlotHistory, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;
        Number::<u8>::LE.write(&BITVEC_DISCRIMINATOR, writer)?;
        Number::<u64>::LE.write(&16384, writer)?;

        for bits in &value.bits {
            Number::<u64>::LE.write(bits, writer)?;
        }

        Number::<u64>::LE.write(&1048576, writer)?;
        Number::<u64>::LE.write(&value.next_slot, writer)
    }
}

impl Decoder<SlotHistory> for SysvarCodec<SlotHistory> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SlotHistory, Error> {
        if Number::<u8>::LE.read(reader)? != BITVEC_DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }

        if Number::<u64>::LE.read(reader)? != 16384 {
            return Err(Error::InvalidValue);
        }

        reader.claim(BITVEC_LENGTH)?;
        let mut bits = Vec::new();
        bits.try_reserve_exact(BITVEC_LENGTH)
            .map_err(|_| Error::AllocationFailed)?;

        for _ in 0..BITVEC_LENGTH {
            bits.push(Number::<u64>::LE.read(reader)?);
        }

        if Number::<u64>::LE.read(reader)? != 1048576 {
            return Err(Error::InvalidValue);
        }

        Ok(SlotHistory {
            bits,
            next_slot: Number::<u64>::LE.read(reader)?,
        })
    }
}
