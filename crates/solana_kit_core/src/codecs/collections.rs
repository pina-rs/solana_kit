//! Ordered domain values reuse the selected sequence's framing and budget.

use alloc::vec::Vec;

use crate::collections::{OrderedMap, OrderedSet};

use super::{Decoder, Encoder, Error, Limits, Reader, Size, Writer};

/// Converts a sequence of key/value pairs to an insertion-ordered map.
///
/// Use [`super::Sequence`] with [`super::Tuple`] for ordinary `u32`, fixed or
/// remainder counts, [`super::Counted`] for another prefix, or
/// [`super::ItemSentinel`] for item-boundary termination. Repeated decoded keys
/// replace values without relocating the first key. All original wire entries
/// are decoded and claimed before in-place normalization.
#[derive(Clone, Copy, Debug)]
pub struct OrderedMapCodec<C>(pub C);

impl<C: Encoder<[(K, V)]>, K, V> Encoder<OrderedMap<K, V>> for OrderedMapCodec<C> {
    fn encode_size(&self) -> Result<Size, Error> {
        self.0.encode_size()
    }

    fn encoded_len(&self, value: &OrderedMap<K, V>) -> Result<usize, Error> {
        self.0.encoded_len(value.as_slice())
    }

    fn write(&self, value: &OrderedMap<K, V>, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write(value.as_slice(), writer)
    }

    fn encode(&self, value: &OrderedMap<K, V>, max_bytes: usize) -> Result<Vec<u8>, Error> {
        self.0.encode(value.as_slice(), max_bytes)
    }
}

impl<C: Decoder<Vec<(K, V)>>, K: Eq, V> Decoder<OrderedMap<K, V>> for OrderedMapCodec<C> {
    fn decode_size(&self) -> Result<Size, Error> {
        self.0.decode_size()
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<OrderedMap<K, V>, Error> {
        self.0.read(reader).map(OrderedMap::from_entries)
    }

    fn decode(&self, bytes: &[u8], limits: Limits) -> Result<OrderedMap<K, V>, Error> {
        self.0.decode(bytes, limits).map(OrderedMap::from_entries)
    }

    fn decode_prefix(
        &self,
        bytes: &[u8],
        offset: usize,
        limits: Limits,
    ) -> Result<(OrderedMap<K, V>, usize), Error> {
        self.0
            .decode_prefix(bytes, offset, limits)
            .map(|(entries, next)| (OrderedMap::from_entries(entries), next))
    }
}

/// Converts a selected sequence to first-seen, distinct set values.
///
/// The sequence controls count, framing, errors, allocation claims and metadata.
/// Duplicate wire items still consume their original bytes and budget. Only the
/// returned domain value removes later equal values, without another allocation.
#[derive(Clone, Copy, Debug)]
pub struct OrderedSetCodec<C>(pub C);

impl<C: Encoder<[T]>, T> Encoder<OrderedSet<T>> for OrderedSetCodec<C> {
    fn encode_size(&self) -> Result<Size, Error> {
        self.0.encode_size()
    }

    fn encoded_len(&self, value: &OrderedSet<T>) -> Result<usize, Error> {
        self.0.encoded_len(value.as_slice())
    }

    fn write(&self, value: &OrderedSet<T>, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write(value.as_slice(), writer)
    }

    fn encode(&self, value: &OrderedSet<T>, max_bytes: usize) -> Result<Vec<u8>, Error> {
        self.0.encode(value.as_slice(), max_bytes)
    }
}

impl<C: Decoder<Vec<T>>, T: Eq> Decoder<OrderedSet<T>> for OrderedSetCodec<C> {
    fn decode_size(&self) -> Result<Size, Error> {
        self.0.decode_size()
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<OrderedSet<T>, Error> {
        self.0.read(reader).map(OrderedSet::from_values)
    }

    fn decode(&self, bytes: &[u8], limits: Limits) -> Result<OrderedSet<T>, Error> {
        self.0.decode(bytes, limits).map(OrderedSet::from_values)
    }

    fn decode_prefix(
        &self,
        bytes: &[u8],
        offset: usize,
        limits: Limits,
    ) -> Result<(OrderedSet<T>, usize), Error> {
        self.0
            .decode_prefix(bytes, offset, limits)
            .map(|(values, next)| (OrderedSet::from_values(values), next))
    }
}
