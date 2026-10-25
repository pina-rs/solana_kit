//! Runtime named schemas with borrowed names and Kit-owned ordered values.

use alloc::vec::Vec;

use crate::collections::OrderedMap;

use super::size::sum_sizes;
use super::{Decoder, Encoder, Error, Reader, Size, Writer};

/// Encodes declared fields in schema order. Unrelated input keys are preserved
/// by the caller and ignored; a missing declared field is an error.
/// Size queries reserve one metadata entry per declared field.
pub struct RecordEncoder<'a, T> {
    fields: &'a [(&'a str, &'a dyn Encoder<T>)],
}

impl<'a, T> RecordEncoder<'a, T> {
    /// Rejects ambiguous duplicate schema names before any value operation.
    pub fn try_new(fields: &'a [(&'a str, &'a dyn Encoder<T>)]) -> Result<Self, Error> {
        unique_names(fields.iter().map(|(name, _)| *name))?;
        Ok(Self { fields })
    }
}

impl<T> Encoder<OrderedMap<&str, T>> for RecordEncoder<'_, T> {
    fn encode_size(&self) -> Result<Size, Error> {
        sum_sizes(self.fields.iter().map(|(_, item)| item.encode_size()))
    }

    fn encoded_len(&self, values: &OrderedMap<&str, T>) -> Result<usize, Error> {
        self.fields.iter().try_fold(0usize, |size, (name, item)| {
            let value = values.get(*name).ok_or(Error::InvalidValue)?;
            size.checked_add(item.encoded_len(value)?)
                .ok_or(Error::Overflow)
        })
    }

    fn write(&self, values: &OrderedMap<&str, T>, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(values)?;

        for (name, item) in self.fields {
            item.write(values.get(*name).ok_or(Error::InvalidValue)?, writer)?;
        }

        Ok(())
    }
}

/// Decodes declared fields into an ordered map without allocating field names.
/// Size queries reserve one metadata entry per declared field.
pub struct RecordDecoder<'a, T> {
    fields: &'a [(&'a str, &'a dyn Decoder<T>)],
}

impl<'a, T> RecordDecoder<'a, T> {
    /// Rejects ambiguous duplicate schema names before any byte operation.
    pub fn try_new(fields: &'a [(&'a str, &'a dyn Decoder<T>)]) -> Result<Self, Error> {
        unique_names(fields.iter().map(|(name, _)| *name))?;
        Ok(Self { fields })
    }
}

impl<'a, T> Decoder<OrderedMap<&'a str, T>> for RecordDecoder<'a, T> {
    fn decode_size(&self) -> Result<Size, Error> {
        sum_sizes(self.fields.iter().map(|(_, item)| item.decode_size()))
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<OrderedMap<&'a str, T>, Error> {
        reader.claim(self.fields.len())?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(self.fields.len())
            .map_err(|_| Error::AllocationFailed)?;

        for (name, item) in self.fields {
            values.push((*name, item.read(reader)?));
        }

        Ok(OrderedMap::from_entries(values))
    }
}

fn unique_names<'a>(names: impl Iterator<Item = &'a str> + Clone) -> Result<(), Error> {
    for (index, name) in names.clone().enumerate() {
        if names.clone().take(index).any(|previous| previous == name) {
            return Err(Error::InvalidValue);
        }
    }

    Ok(())
}
