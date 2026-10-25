//! Runtime-selected tuple schemas with explicit caller-owned value types.

use alloc::vec::Vec;

use super::size::sum_sizes;
use super::{Decoder, Encoder, Error, Reader, Size, Writer};

/// Sequential fields whose runtime-selected encoders share a Rust value domain.
/// A caller-defined enum can represent heterogeneous values without erased casts.
/// Size queries reserve one metadata entry per schema position.
pub struct TupleEncoder<'a, T> {
    /// One encoder for each tuple position, in wire order.
    pub items: &'a [&'a dyn Encoder<T>],
}

impl<T> Encoder<[T]> for TupleEncoder<'_, T> {
    fn encode_size(&self) -> Result<Size, Error> {
        sum_sizes(self.items.iter().map(|item| item.encode_size()))
    }

    fn encoded_len(&self, values: &[T]) -> Result<usize, Error> {
        if values.len() != self.items.len() {
            return Err(Error::InvalidValue);
        }

        self.items
            .iter()
            .zip(values)
            .try_fold(0usize, |size, (item, value)| {
                size.checked_add(item.encoded_len(value)?)
                    .ok_or(Error::Overflow)
            })
    }

    fn write(&self, values: &[T], writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(values)?;

        for (item, value) in self.items.iter().zip(values) {
            item.write(value, writer)?;
        }

        Ok(())
    }
}

/// Sequential fields whose runtime-selected decoders produce one Rust value domain.
/// Size queries reserve one metadata entry per schema position.
pub struct TupleDecoder<'a, T> {
    /// One decoder for each tuple position, in wire order.
    pub items: &'a [&'a dyn Decoder<T>],
}

impl<T> Decoder<Vec<T>> for TupleDecoder<'_, T> {
    fn decode_size(&self) -> Result<Size, Error> {
        sum_sizes(self.items.iter().map(|item| item.decode_size()))
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
        reader.claim(self.items.len())?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(self.items.len())
            .map_err(|_| Error::AllocationFailed)?;

        for item in self.items {
            values.push(item.read(reader)?);
        }

        Ok(values)
    }
}
