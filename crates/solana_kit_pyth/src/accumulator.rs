//! Borrowed accumulator payloads with bounded update and proof allocations.
use alloc::vec::Vec;
use solana_kit_core::codecs::{Decoder, Error, FixedBytes, Limits, Number, Reader};

/// Accumulator transfer magic, distinct from the contained verification message.
pub const ACCUMULATOR_UPDATE_DATA_MAGIC: [u8; 4] = *b"PNAU";

/// One price message and its ordered sibling hashes. No proof is authenticated.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerklePriceUpdate<'a> {
    /// Caller-owned wire message, typically a price-feed message.
    pub message: &'a [u8],
    /// Keccak-160 sibling hashes in encoded order.
    pub proof: Vec<[u8; 20]>,
}

/// Current major-version-one accumulator update with preserved extension bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccumulatorUpdateData<'a> {
    /// Encoded compatible minor version.
    pub minor_version: u8,
    /// Header extension bytes, before the proof variant.
    pub trailing: &'a [u8],
    /// Caller-owned Wormhole VAA; parsing does not authenticate it.
    pub vaa: &'a [u8],
    /// Ordered messages and proof paths.
    pub updates: Vec<MerklePriceUpdate<'a>>,
    /// Append-only fields after the known proof structure.
    pub extension: &'a [u8],
}

/// Recognizes the major-one header, including forward-compatible minor versions.
/// This inspects six bytes and does not validate the proof or complete transfer.
#[must_use]
pub fn is_accumulator_update_data(bytes: &[u8]) -> bool {
    bytes.len() >= 6 && bytes[..4] == ACCUMULATOR_UPDATE_DATA_MAGIC && bytes[4] == 1
}

/// Parses variant-zero Wormhole/Merkle transfers using one shared item budget.
/// Header, VAA, message and extension bytes borrow the original input. Allocated
/// update entries and proof hashes each claim one item before allocation.
///
/// # Errors
/// Rejects incorrect magic, an unsupported major/proof variant, truncation,
/// exhausted byte/item limits or allocation failure. Minor-version and tail
/// extensions follow the upstream append-only wire contract.
pub fn parse_accumulator_update_data(
    bytes: &[u8],
    limits: Limits,
) -> Result<AccumulatorUpdateData<'_>, Error> {
    let mut reader = Reader::new(bytes, limits)?;
    let magic = reader.take(ACCUMULATOR_UPDATE_DATA_MAGIC.len())?;

    if magic != ACCUMULATOR_UPDATE_DATA_MAGIC {
        return Err(Error::InvalidConstant {
            offset: 0,
            length: ACCUMULATOR_UPDATE_DATA_MAGIC.len(),
        });
    }

    if Number::<u8>::BE.read(&mut reader)? != 1 {
        return Err(Error::InvalidDiscriminator);
    }

    let minor_version = Number::<u8>::BE.read(&mut reader)?;
    let trailing_size = usize::from(Number::<u8>::BE.read(&mut reader)?);
    let trailing = reader.take(trailing_size)?;

    if Number::<u8>::BE.read(&mut reader)? != 0 {
        return Err(Error::InvalidDiscriminator);
    }

    let vaa = prefixed_bytes(&mut reader)?;
    let count = usize::from(Number::<u8>::BE.read(&mut reader)?);
    reader.claim(count)?;
    let mut updates = Vec::new();
    updates
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;

    for _ in 0..count {
        updates.push(price_update(&mut reader)?);
    }

    Ok(AccumulatorUpdateData {
        minor_version,
        trailing,
        vaa,
        updates,
        extension: reader.remaining(),
    })
}

fn prefixed_bytes<'a>(reader: &mut Reader<'a>) -> Result<&'a [u8], Error> {
    let count = usize::from(Number::<u16>::BE.read(reader)?);
    reader.take(count)
}

fn price_update<'a>(reader: &mut Reader<'a>) -> Result<MerklePriceUpdate<'a>, Error> {
    let message = prefixed_bytes(reader)?;
    let count = usize::from(Number::<u8>::BE.read(reader)?);
    reader.claim(count)?;
    let mut proof = Vec::new();
    proof
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;

    for _ in 0..count {
        proof.push(FixedBytes::<20>.read(reader)?);
    }

    Ok(MerklePriceUpdate { message, proof })
}
