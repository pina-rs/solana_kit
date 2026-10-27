//! One bounded Borsh encoder reuses the core writer and fallible owned allocation.
use crate::MerklePriceUpdate;
use alloc::vec::Vec;
use solana_kit_core::codecs::{Encoder, Error, Limits, Number, Writer};

const LENGTH_BYTES: usize = 4;
const PROOF_BYTES: usize = 20;

pub(super) struct Wire<'a> {
    pub vaa: Option<&'a [u8]>,
    pub update: &'a MerklePriceUpdate<'a>,
    pub treasury_id: u8,
    pub discriminator: Option<[u8; 8]>,
}

impl Wire<'_> {
    /// Item budget covers all proof hashes plus every allocated instruction account.
    pub fn encode(self, limits: Limits, accounts: usize) -> Result<Vec<u8>, Error> {
        let items = self
            .update
            .proof
            .len()
            .checked_add(accounts)
            .ok_or(Error::Overflow)?;

        if items > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        Codec.encode(&self, limits.max_bytes)
    }
}

struct Codec;

impl Encoder<Wire<'_>> for Codec {
    fn encoded_len(&self, value: &Wire<'_>) -> Result<usize, Error> {
        let message = vector_size(value.update.message.len(), 1)?;
        let proof = vector_size(value.update.proof.len(), PROOF_BYTES)?;

        let vaa = match value.vaa {
            Some(vaa) => vector_size(vaa.len(), 1)?,
            None => 0,
        };

        let prefix = value.discriminator.map_or(0, |bytes| bytes.len());

        [message, proof, vaa, prefix, 1]
            .into_iter()
            .try_fold(0_usize, |total, size| {
                total.checked_add(size).ok_or(Error::Overflow)
            })
    }

    fn write(&self, value: &Wire<'_>, writer: &mut Writer<'_>) -> Result<(), Error> {
        if let Some(discriminator) = value.discriminator {
            writer.put(&discriminator)?;
        }

        if let Some(vaa) = value.vaa {
            write_bytes(vaa, writer)?;
        }

        write_bytes(value.update.message, writer)?;
        Number::<u32>::LE.write(
            &u32::try_from(value.update.proof.len()).map_err(|_| Error::Overflow)?,
            writer,
        )?;

        for hash in &value.update.proof {
            writer.put(hash)?;
        }

        Number::<u8>::LE.write(&value.treasury_id, writer)
    }
}

/// A Borsh vector prefix must represent the element count before allocating bytes.
fn vector_size(count: usize, width: usize) -> Result<usize, Error> {
    u32::try_from(count).map_err(|_| Error::Overflow)?;
    count
        .checked_mul(width)
        .and_then(|size| size.checked_add(LENGTH_BYTES))
        .ok_or(Error::Overflow)
}

fn write_bytes(bytes: &[u8], writer: &mut Writer<'_>) -> Result<(), Error> {
    let count = u32::try_from(bytes.len()).map_err(|_| Error::Overflow)?;
    Number::<u32>::LE.write(&count, writer)?;
    writer.put(bytes)
}
