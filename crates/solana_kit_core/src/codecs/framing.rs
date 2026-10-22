//! Explicit byte frames, allocation extents and fixed-width transforms.

use alloc::vec::Vec;

use super::{Decoder, Encoder, Error, Reader, Writer};

/// Determines whether unused bytes in a bounded payload are accepted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameConsumption {
    /// Require the nested decoder to consume every payload byte.
    Exact,
    /// Advance past the entire payload, accepting unread nested bytes as Dart does.
    Skip,
}

impl FrameConsumption {
    pub(super) fn read<C: Decoder<T>, T>(
        self,
        item: &C,
        reader: &mut Reader<'_>,
        size: usize,
    ) -> Result<T, Error> {
        if self == Self::Exact {
            return reader.frame(size, |inner| item.read(inner));
        }

        let bytes = reader.take(size)?;
        reader.scoped(bytes, |inner| item.read(inner))
    }
}

/// A caller-selected u64-valued byte-length prefix. Use Mapped for narrow,
/// compact, signed or floating prefix codecs, validating their numeric domain.
/// Prefix units are bytes, never sequence entries or decoded value sizes.
#[derive(Clone, Copy, Debug)]
pub struct ByteSizePrefix<C, P> {
    /// Nested payload codec.
    pub item: C,
    /// Codec for the payload byte count.
    pub prefix: P,
    /// Whether unread payload bytes fail decoding.
    pub consumption: FrameConsumption,
}

impl<C: Encoder<T>, P: Encoder<u64>, T: ?Sized> Encoder<T> for ByteSizePrefix<C, P> {
    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        let size = self.item.encoded_len(value)?;
        let count = u64::try_from(size).map_err(|_| Error::Overflow)?;
        size.checked_add(self.prefix.encoded_len(&count)?)
            .ok_or(Error::Overflow)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        let size = self.item.encoded_len(value)?;
        let count = u64::try_from(size).map_err(|_| Error::Overflow)?;
        self.prefix.write(&count, writer)?;
        writer.frame(size, |inner| self.item.write(value, inner))
    }
}

impl<C: Decoder<T>, P: Decoder<u64>, T> Decoder<T> for ByteSizePrefix<C, P> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        let size = usize::try_from(self.prefix.read(reader)?).map_err(|_| Error::Overflow)?;
        self.consumption.read(&self.item, reader, size)
    }
}

/// A fixed-capacity field. Writes truncate or reject oversized encodings and
/// zero-fill unused storage. Unlike Dart, dirty destination padding is cleared.
/// `decoded_size` declares a fixed nested decoder width, allowing zero extension
/// or truncation before reading. None passes exactly the field to the decoder.
/// Reads consume the entire field while allowing unread nested bytes.
#[derive(Clone, Copy, Debug)]
pub struct FixedSize<C> {
    /// Nested codec.
    pub item: C,
    /// Field width in bytes.
    pub size: usize,
    /// Optional fixed nested decoder width, independent of field width.
    pub decoded_size: Option<usize>,
    /// Whether oversized encoded values may be truncated.
    pub allow_truncation: bool,
    /// Maximum scratch encoding size before truncation.
    pub max_encoded_size: usize,
}

impl<C: Encoder<T>, T: ?Sized> Encoder<T> for FixedSize<C> {
    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        let size = self.item.encoded_len(value)?;

        if size > self.max_encoded_size {
            return Err(Error::LimitExceeded);
        }

        if size > self.size && !self.allow_truncation {
            return Err(Error::InvalidValue);
        }

        Ok(self.size)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;
        let bytes = self.item.encode(value, self.max_encoded_size)?;
        let used = bytes.len().min(self.size);
        writer.put(&bytes[..used])?;
        writer.fill(self.size - used, 0)
    }
}

impl<C: Decoder<T>, T> Decoder<T> for FixedSize<C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        let bytes = reader.take(self.size)?;
        let Some(width) = self.decoded_size else {
            return reader.scoped(bytes, |inner| self.item.read(inner));
        };

        if width <= bytes.len() {
            return reader.scoped(&bytes[..width], |inner| self.item.read(inner));
        }

        reader.claim(width)?;
        let mut padded = allocate(width, width)?;
        padded[..bytes.len()].copy_from_slice(bytes);
        reader.scoped(&padded, |inner| self.item.read(inner))
    }
}

/// Changes the allocation extent while leaving the nested write/read cursor
/// movement unchanged. The resize callback returns a checked nonnegative size.
/// Owned encoding accepts unused zero-filled storage, matching Dart resize.
/// Direct writes still require enough storage for the original nested codec.
#[derive(Clone, Copy, Debug)]
pub struct Resized<C, F> {
    /// Nested codec.
    pub item: C,
    /// Computes the new allocation extent from the original encoded size.
    pub resize: F,
}

impl<C: Encoder<T>, F: Fn(usize) -> Result<usize, Error>, T: ?Sized> Encoder<T> for Resized<C, F> {
    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        (self.resize)(self.item.encoded_len(value)?)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.item.write(value, writer)
    }

    fn encode(&self, value: &T, max_bytes: usize) -> Result<Vec<u8>, Error> {
        let mut bytes = allocate(self.encoded_len(value)?, max_bytes)?;
        self.write(value, &mut Writer::new(&mut bytes))?;
        Ok(bytes)
    }
}

impl<C: Decoder<T>, F, T> Decoder<T> for Resized<C, F> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        self.item.read(reader)
    }
}

/// Reverses one explicitly declared fixed-width field. Encoding checks that
/// the nested encoded width matches. Scratch decoding claims its width from
/// the shared element budget before allocating, then preserves nested claims.
#[derive(Clone, Copy, Debug)]
pub struct Reverse<C> {
    /// Nested fixed-width codec.
    pub item: C,
    /// Number of bytes to reverse.
    pub size: usize,
}

impl<C: Encoder<T>, T: ?Sized> Encoder<T> for Reverse<C> {
    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        if self.item.encoded_len(value)? != self.size {
            return Err(Error::InvalidValue);
        }

        Ok(self.size)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;
        let mut bytes = self.item.encode(value, self.size)?;
        bytes.reverse();
        writer.put(&bytes)
    }
}

impl<C: Decoder<T>, T> Decoder<T> for Reverse<C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        let bytes = reader.take(self.size)?;
        reader.claim(self.size)?;
        let mut reversed = allocate(self.size, self.size)?;
        reversed.copy_from_slice(bytes);
        reversed.reverse();
        reader.scoped(&reversed, |inner| self.item.read(inner))
    }
}

pub(super) fn allocate(size: usize, maximum: usize) -> Result<Vec<u8>, Error> {
    if size > maximum {
        return Err(Error::LimitExceeded);
    }

    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(size)
        .map_err(|_| Error::AllocationFailed)?;
    bytes.resize(size, 0);
    Ok(bytes)
}
