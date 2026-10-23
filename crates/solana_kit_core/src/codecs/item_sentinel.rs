//! Collection termination checked only between decoded items.

use alloc::vec::Vec;

use super::{Count, Decoder, Encoder, Error, Reader, Sequence, Writer};

/// Whether a collection writes and requires its terminating marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Termination {
    /// Write the marker and require it when reading.
    Required,
    /// Write the marker, but also accept the exact end of input.
    Optional,
    /// Omit the marker when writing; consume it when encountered on input.
    Omitted,
}

/// A homogeneous collection whose marker is compared only at item boundaries.
/// Items must consume bytes and must not start with the marker. Optional and
/// omitted markers must be no wider than the smallest item. Unlike the pinned
/// source, a partial final marker is rejected rather than silently skipped.
#[derive(Clone, Copy, Debug)]
pub struct ItemSentinel<'a, C> {
    item: C,
    marker: &'a [u8],
    termination: Termination,
    minimum_item_size: usize,
}

impl<'a, C> ItemSentinel<'a, C> {
    /// Rejects an empty marker, which could never distinguish items from an end.
    pub fn new(
        item: C,
        marker: &'a [u8],
        termination: Termination,
        minimum_item_size: usize,
    ) -> Result<Self, Error> {
        if marker.is_empty()
            || minimum_item_size == 0
            || (termination != Termination::Required && marker.len() > minimum_item_size)
        {
            return Err(Error::InvalidValue);
        }

        Ok(Self {
            item,
            marker,
            termination,
            minimum_item_size,
        })
    }

    fn suffix(&self) -> &[u8] {
        match self.termination {
            Termination::Omitted => &[],
            _ => self.marker,
        }
    }

    fn allow_exhaustion(&self) -> Result<(), Error> {
        if self.termination == Termination::Required {
            return Err(Error::Truncated);
        }

        Ok(())
    }
}

impl<C: Encoder<T>, T> Encoder<[T]> for ItemSentinel<'_, C> {
    fn encoded_len(&self, value: &[T]) -> Result<usize, Error> {
        for value in value {
            if self.item.encoded_len(value)? < self.minimum_item_size {
                return Err(Error::InvalidValue);
            }
        }

        let items = Sequence {
            item: &self.item,
            count: Count::Remainder,
        };
        items
            .encoded_len(value)?
            .checked_add(self.suffix().len())
            .ok_or(Error::Overflow)
    }

    fn write(&self, value: &[T], writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;

        for value in value {
            self.item.write(value, writer)?;
        }

        writer.put(self.suffix())
    }
}

impl<C: Decoder<T>, T> Decoder<Vec<T>> for ItemSentinel<'_, C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
        let mut values = Vec::new();

        loop {
            if reader.remaining().starts_with(self.marker) {
                reader.take(self.marker.len())?;

                return Ok(values);
            }

            if reader.remaining().is_empty() {
                self.allow_exhaustion()?;

                return Ok(values);
            }

            if reader.remaining().len() < self.minimum_item_size {
                return Err(Error::Truncated);
            }

            reader.claim(1)?;
            let before = reader.position();
            let value = self.item.read(reader)?;

            if reader
                .position()
                .checked_sub(before)
                .is_none_or(|size| size < self.minimum_item_size)
            {
                return Err(Error::InvalidValue);
            }

            values.try_reserve(1).map_err(|_| Error::AllocationFailed)?;
            values.push(value);
        }
    }
}
