//! Byte, text, and sequential composition contracts.

use alloc::{string::String, vec::Vec};

use super::{Decoder, Encoder, Error, Number, Reader, Size, Writer};

/// A strict one-byte boolean: only zero and one are accepted.
#[derive(Clone, Copy, Debug, Default)]
pub struct Bool;

impl Encoder<bool> for Bool {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(1))
    }

    fn encoded_len(&self, _: &bool) -> Result<usize, Error> {
        Ok(1)
    }

    fn write(&self, value: &bool, writer: &mut Writer<'_>) -> Result<(), Error> {
        writer.put(&[u8::from(*value)])
    }
}

impl Decoder<bool> for Bool {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(1))
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<bool, Error> {
        match Number::<u8>::LE.read(reader)? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(Error::InvalidBoolean { value }),
        }
    }
}

/// Exactly `N` bytes, without a length prefix or allocation.
#[derive(Clone, Copy, Debug, Default)]
pub struct FixedBytes<const N: usize>;

impl<const N: usize> Encoder<[u8; N]> for FixedBytes<N> {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(N))
    }

    fn encoded_len(&self, _: &[u8; N]) -> Result<usize, Error> {
        Ok(N)
    }

    fn write(&self, value: &[u8; N], writer: &mut Writer<'_>) -> Result<(), Error> {
        writer.put(value)
    }
}

impl<const N: usize> Decoder<[u8; N]> for FixedBytes<N> {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(N))
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<[u8; N], Error> {
        reader.take(N)?.try_into().map_err(|_| Error::Truncated)
    }
}

/// Raw bytes consuming the enclosing reader's remainder, with no size prefix.
/// Wrap in [`super::SizePrefix`] for a bounded length-prefixed byte string.
#[derive(Clone, Copy, Debug, Default)]
pub struct Bytes;

impl Encoder<[u8]> for Bytes {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Variable { max: None })
    }

    fn encoded_len(&self, value: &[u8]) -> Result<usize, Error> {
        Ok(value.len())
    }

    fn write(&self, value: &[u8], writer: &mut Writer<'_>) -> Result<(), Error> {
        writer.put(value)
    }
}

impl Decoder<Vec<u8>> for Bytes {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Variable { max: None })
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<u8>, Error> {
        let length = reader.remaining().len();
        reader.claim(length)?;
        let mut output = Vec::new();
        output
            .try_reserve_exact(length)
            .map_err(|_| Error::AllocationFailed)?;
        output.extend_from_slice(reader.take(length)?);
        Ok(output)
    }
}

/// Strict UTF-8 consuming the enclosing reader's remainder, without a prefix.
/// [`SizePrefix(Utf8)`](super::SizePrefix) is a Borsh `u32` byte-length string.
#[derive(Clone, Copy, Debug, Default)]
pub struct Utf8;

impl Encoder<str> for Utf8 {
    fn encoded_len(&self, value: &str) -> Result<usize, Error> {
        Ok(value.len())
    }

    fn write(&self, value: &str, writer: &mut Writer<'_>) -> Result<(), Error> {
        writer.put(value.as_bytes())
    }
}

impl Decoder<String> for Utf8 {
    fn read(&self, reader: &mut Reader<'_>) -> Result<String, Error> {
        // Validate before reserving memory for untrusted text.
        core::str::from_utf8(reader.remaining()).map_err(|_| Error::InvalidUtf8)?;
        String::from_utf8(Bytes.read(reader)?).map_err(|_| Error::InvalidUtf8)
    }
}

/// How a sequence obtains its element count.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Count {
    /// A little-endian `u32` prefix, as in Borsh vectors.
    U32,
    /// Exactly this many elements, with no prefix.
    Fixed(usize),
    /// Consume elements to the enclosing reader's end; each must consume bytes.
    Remainder,
}

/// Homogeneous sequential elements. All allocations share the reader's budget.
#[derive(Clone, Copy, Debug)]
pub struct Sequence<C> {
    /// Codec applied to each element.
    pub item: C,
    /// Count contract used for both writing and reading.
    pub count: Count,
}

impl<C, T> Encoder<[T]> for Sequence<C>
where
    C: Encoder<T>,
{
    fn encoded_len(&self, value: &[T]) -> Result<usize, Error> {
        let prefix = self.count.encoded_len(value.len())?;
        value.iter().try_fold(prefix, |size, item| {
            let item_size = self.item.encoded_len(item)?;

            if self.count == Count::Remainder && item_size == 0 {
                return Err(Error::InvalidValue);
            }

            size.checked_add(item_size).ok_or(Error::Overflow)
        })
    }

    fn write(&self, value: &[T], writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;

        if self.count == Count::U32 {
            Number::<u32>::LE.write(
                &u32::try_from(value.len()).map_err(|_| Error::Overflow)?,
                writer,
            )?;
        }

        for item in value {
            self.item.write(item, writer)?;
        }

        Ok(())
    }
}

impl Count {
    /// Validates the element count and returns only its prefix size.
    fn encoded_len(self, length: usize) -> Result<usize, Error> {
        match self {
            Self::U32 => u32::try_from(length)
                .map(|_| 4)
                .map_err(|_| Error::Overflow),
            Self::Fixed(expected) if length != expected => Err(Error::InvalidValue),
            _ => Ok(0),
        }
    }
}

impl<C, T> Decoder<Vec<T>> for Sequence<C>
where
    C: Decoder<T>,
{
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
        let count = match self.count {
            Count::U32 => Some(
                usize::try_from(Number::<u32>::LE.read(reader)?).map_err(|_| Error::Overflow)?,
            ),
            Count::Fixed(count) => Some(count),
            Count::Remainder => None,
        };
        let mut output = Vec::new();

        if let Some(count) = count {
            reader.claim(count)?;
            output
                .try_reserve_exact(count)
                .map_err(|_| Error::AllocationFailed)?;

            for _ in 0..count {
                output.push(self.item.read(reader)?);
            }

            return Ok(output);
        }

        while !reader.remaining().is_empty() {
            reader.claim(1)?;
            let before = reader.position();
            let value = self.item.read(reader)?;

            if reader.position() == before {
                return Err(Error::InvalidValue);
            }

            output.try_reserve(1).map_err(|_| Error::AllocationFailed)?;
            output.push(value);
        }

        Ok(output)
    }
}

/// Two heterogeneous values encoded in order; nest tuples for larger records.
#[derive(Clone, Copy, Debug)]
pub struct Tuple<A, B>(pub A, pub B);

impl<A, B, T, U> Encoder<(T, U)> for Tuple<A, B>
where
    A: Encoder<T>,
    B: Encoder<U>,
{
    fn encoded_len(&self, value: &(T, U)) -> Result<usize, Error> {
        self.0
            .encoded_len(&value.0)?
            .checked_add(self.1.encoded_len(&value.1)?)
            .ok_or(Error::Overflow)
    }

    fn write(&self, value: &(T, U), writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write(&value.0, writer)?;
        self.1.write(&value.1, writer)
    }
}

impl<A, B, T, U> Decoder<(T, U)> for Tuple<A, B>
where
    A: Decoder<T>,
    B: Decoder<U>,
{
    fn read(&self, reader: &mut Reader<'_>) -> Result<(T, U), Error> {
        Ok((self.0.read(reader)?, self.1.read(reader)?))
    }
}
