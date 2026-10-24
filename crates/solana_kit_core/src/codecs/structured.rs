//! Concrete choices, bit fields and dependent records.

use alloc::vec::Vec;

use super::{Count, Decoder, Encoder, Error, Limits, Number, Reader, Sequence, Size, Writer};

const BITS_PER_BYTE: usize = 8;

/// An empty value occupying no wire bytes. Use only counted zero-width sequences.
#[derive(Clone, Copy, Debug, Default)]
pub struct Unit;

impl Encoder<()> for Unit {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(0))
    }

    fn encoded_len(&self, _: &()) -> Result<usize, Error> {
        Ok(0)
    }
    fn write(&self, _: &(), _: &mut Writer<'_>) -> Result<(), Error> {
        Ok(())
    }
}

impl Decoder<()> for Unit {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(0))
    }

    fn read(&self, _: &mut Reader<'_>) -> Result<(), Error> {
        Ok(())
    }
}

/// A fixed number of packed bytes. Missing bits are false and excess bits ignored.
/// Backward mode reverses both byte order and bit order, matching Dart.
#[derive(Clone, Copy, Debug)]
pub struct BitArray {
    /// Number of wire bytes, not the number of boolean values.
    pub bytes: usize,
    /// Whether the first boolean occupies the low bit of the last byte.
    pub backward: bool,
}

impl BitArray {
    fn wire_group(&self, group: usize) -> usize {
        if self.backward {
            return self.bytes - group - 1;
        }

        group
    }

    fn shift(&self, bit: usize) -> usize {
        if self.backward {
            return bit;
        }

        BITS_PER_BYTE - 1 - bit
    }

    fn pack(&self, values: &[bool], group: usize) -> u8 {
        let start = group * BITS_PER_BYTE;
        (0..BITS_PER_BYTE).fold(0, |byte, bit| {
            byte | (u8::from(values.get(start + bit).copied().unwrap_or(false)) << self.shift(bit))
        })
    }
}

impl Encoder<[bool]> for BitArray {
    fn encoded_len(&self, _: &[bool]) -> Result<usize, Error> {
        self.bytes
            .checked_mul(BITS_PER_BYTE)
            .ok_or(Error::Overflow)?;
        Ok(self.bytes)
    }

    fn write(&self, value: &[bool], writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;

        for output in 0..self.bytes {
            writer.put(&[self.pack(value, self.wire_group(output))])?;
        }

        Ok(())
    }
}

impl Decoder<Vec<bool>> for BitArray {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<bool>, Error> {
        let count = self
            .bytes
            .checked_mul(BITS_PER_BYTE)
            .ok_or(Error::Overflow)?;
        reader.claim(count)?;
        let bytes = reader.take(self.bytes)?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;

        for group in 0..self.bytes {
            let byte = bytes[self.wire_group(group)];
            values.extend((0..BITS_PER_BYTE).map(|bit| byte & (1 << self.shift(bit)) != 0));
        }

        Ok(values)
    }
}

/// A literal table encoded by a caller-selected u32 index codec.
/// Values are Copy so decoding cannot hide unbudgeted cloning allocations.
/// Duplicate values encode their first index, matching Dart list lookup.
#[derive(Clone, Copy, Debug)]
pub struct Literal<'a, T, P> {
    /// Ordered allowed values.
    pub variants: &'a [T],
    /// Index encoding, commonly a mapped u8 codec or `Number<u32>`.
    pub index: P,
}

impl<T: PartialEq, P: Encoder<u32>> Encoder<T> for Literal<'_, T, P> {
    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        self.index.encoded_len(&self.position(value)?)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.index.write(&self.position(value)?, writer)
    }
}

impl<T: PartialEq, P> Literal<'_, T, P> {
    fn position(&self, value: &T) -> Result<u32, Error> {
        let index = self
            .variants
            .iter()
            .position(|item| item == value)
            .ok_or(Error::InvalidDiscriminator)?;
        u32::try_from(index).map_err(|_| Error::Overflow)
    }
}

impl<T: Copy, P: Decoder<u32>> Decoder<T> for Literal<'_, T, P> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        let index = usize::try_from(self.index.read(reader)?).map_err(|_| Error::Overflow)?;
        self.variants
            .get(index)
            .copied()
            .ok_or(Error::InvalidDiscriminator)
    }
}

/// Two typed alternatives with a checked one-byte zero/one tag.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Either<A, B> {
    /// Tag zero followed by the first payload.
    First(A),
    /// Tag one followed by the second payload.
    Second(B),
}

/// Codecs for the two alternatives of Either, without trial decoding.
#[derive(Clone, Copy, Debug)]
pub struct Choice<A, B>(pub A, pub B);

impl<A: Encoder<T>, B: Encoder<U>, T, U> Encoder<Either<T, U>> for Choice<A, B> {
    fn encoded_len(&self, value: &Either<T, U>) -> Result<usize, Error> {
        let size = match value {
            Either::First(value) => self.0.encoded_len(value)?,
            Either::Second(value) => self.1.encoded_len(value)?,
        };

        size.checked_add(1).ok_or(Error::Overflow)
    }

    fn write(&self, value: &Either<T, U>, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            Either::First(value) => {
                writer.put(&[0])?;
                self.0.write(value, writer)
            }
            Either::Second(value) => {
                writer.put(&[1])?;
                self.1.write(value, writer)
            }
        }
    }
}

impl<A: Decoder<T>, B: Decoder<U>, T, U> Decoder<Either<T, U>> for Choice<A, B> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Either<T, U>, Error> {
        match Number::<u8>::LE.read(reader)? {
            0 => self.0.read(reader).map(Either::First),
            1 => self.1.read(reader).map(Either::Second),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
}

/// A count-prefixed sequence reusing Sequence's claim and allocation policy.
#[derive(Clone, Copy, Debug)]
pub struct Counted<P, C> {
    /// Count encoding. Conversion adapters permit narrow or compact prefixes.
    pub prefix: P,
    /// Codec for each element.
    pub item: C,
}

impl<P: Encoder<u32>, C: Encoder<T>, T> Encoder<[T]> for Counted<P, C> {
    fn encoded_len(&self, value: &[T]) -> Result<usize, Error> {
        let count = u32::try_from(value.len()).map_err(|_| Error::Overflow)?;
        let size = Sequence {
            item: &self.item,
            count: Count::Fixed(value.len()),
        };
        self.prefix
            .encoded_len(&count)?
            .checked_add(size.encoded_len(value)?)
            .ok_or(Error::Overflow)
    }

    fn write(&self, value: &[T], writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;
        self.prefix.write(
            &u32::try_from(value.len()).map_err(|_| Error::Overflow)?,
            writer,
        )?;

        for value in value {
            self.item.write(value, writer)?;
        }

        Ok(())
    }
}

impl<P: Decoder<u32>, C: Decoder<T>, T> Decoder<Vec<T>> for Counted<P, C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
        let count = usize::try_from(self.prefix.read(reader)?).map_err(|_| Error::Overflow)?;
        Sequence {
            item: &self.item,
            count: Count::Fixed(count),
        }
        .read(reader)
    }
}

/// A record whose second codec is selected from its first decoded value.
/// The selector receives a borrowed first value, returns a concrete codec and
/// must bound any allocations it performs. Nested decoders share Reader claims.
#[derive(Clone, Copy, Debug)]
pub struct Dependent<A, F>(pub A, pub F);

impl<A, F, B, T, U> Encoder<(T, U)> for Dependent<A, F>
where
    A: Encoder<T>,
    F: Fn(&T) -> Result<B, Error>,
    B: Encoder<U>,
{
    fn encoded_len(&self, value: &(T, U)) -> Result<usize, Error> {
        self.0
            .encoded_len(&value.0)?
            .checked_add((self.1)(&value.0)?.encoded_len(&value.1)?)
            .ok_or(Error::Overflow)
    }

    fn write(&self, value: &(T, U), writer: &mut Writer<'_>) -> Result<(), Error> {
        let second = (self.1)(&value.0)?;
        self.0.write(&value.0, writer)?;
        second.write(&value.1, writer)
    }
}

impl<A, F, B, T, U> Decoder<(T, U)> for Dependent<A, F>
where
    A: Decoder<T>,
    F: Fn(&T) -> Result<B, Error>,
    B: Decoder<U>,
{
    fn read(&self, reader: &mut Reader<'_>) -> Result<(T, U), Error> {
        let first = self.0.read(reader)?;
        let second = (self.1)(&first)?.read(reader)?;
        Ok((first, second))
    }
}

impl<Codec: Encoder<T>, T: ?Sized> Encoder<T> for &Codec {
    fn encode_size(&self) -> Result<Size, Error> {
        (*self).encode_size()
    }

    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        (*self).encoded_len(value)
    }
    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        (*self).write(value, writer)
    }

    fn encode(&self, value: &T, max_bytes: usize) -> Result<Vec<u8>, Error> {
        Codec::encode(*self, value, max_bytes)
    }
}

impl<Codec: Decoder<T>, T> Decoder<T> for &Codec {
    fn decode_size(&self) -> Result<Size, Error> {
        (*self).decode_size()
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        (*self).read(reader)
    }

    fn decode(&self, bytes: &[u8], limits: Limits) -> Result<T, Error> {
        Codec::decode(*self, bytes, limits)
    }

    fn decode_prefix(
        &self,
        bytes: &[u8],
        offset: usize,
        limits: Limits,
    ) -> Result<(T, usize), Error> {
        Codec::decode_prefix(*self, bytes, offset, limits)
    }
}

/// Adapts a sequence to owned Vec fields in typed records.
/// Keeping this wrapper separate preserves existing slice and array inference.
#[derive(Clone, Copy, Debug)]
pub struct OwnedSequence<C>(pub Sequence<C>);

impl<C: Encoder<T>, T> Encoder<Vec<T>> for OwnedSequence<C> {
    fn encoded_len(&self, value: &Vec<T>) -> Result<usize, Error> {
        self.0.encoded_len(value.as_slice())
    }

    fn write(&self, value: &Vec<T>, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write(value.as_slice(), writer)
    }
}

impl<C: Decoder<T>, T> Decoder<Vec<T>> for OwnedSequence<C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
        self.0.read(reader)
    }
}
