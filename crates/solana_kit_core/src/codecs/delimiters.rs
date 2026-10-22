//! Bounded byte delimiters and deterministic padding.

use super::{Decoder, Encoder, Error, Reader, Writer};

const MAX_SENTINEL_BYTES: usize = 256;

/// Zero-filled padding around a self-delimiting inner codec.
/// Decoding skips padding without validating its contents, matching Dart.
/// Writing always zeroes padding, including caller-owned nonzero buffers.
#[derive(Clone, Copy, Debug)]
pub struct Padded<C> {
    /// Number of bytes before the inner value.
    pub left: usize,
    /// Inner codec.
    pub item: C,
    /// Number of bytes after the inner value.
    pub right: usize,
}

impl<C: Encoder<T>, T: ?Sized> Encoder<T> for Padded<C> {
    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        self.left
            .checked_add(self.item.encoded_len(value)?)
            .and_then(|size| size.checked_add(self.right))
            .ok_or(Error::Overflow)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;
        writer.fill(self.left, 0)?;
        self.item.write(value, writer)?;
        writer.fill(self.right, 0)
    }
}

impl<C: Decoder<T>, T> Decoder<T> for Padded<C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        reader.take(self.left)?;
        let value = self.item.read(reader)?;
        reader.take(self.right)?;
        Ok(value)
    }
}

/// Sentinel-delimited exact frames with an explicit maximum payload size.
/// A sentinel has 1..=256 bytes. Encoding uses at most max_payload scratch bytes.
/// Collisions, including a match overlapping the payload/sentinel boundary, fail.
/// This stricter overlap rule makes accepted writes unambiguous on read.
#[derive(Clone, Copy, Debug)]
pub struct Sentinel<'a, C> {
    item: C,
    sentinel: &'a [u8],
    max_payload: usize,
}

impl<'a, C> Sentinel<'a, C> {
    /// Validates the delimiter and checks the maximum framed length for overflow.
    pub fn new(item: C, sentinel: &'a [u8], max_payload: usize) -> Result<Self, Error> {
        if sentinel.is_empty() || sentinel.len() > MAX_SENTINEL_BYTES {
            return Err(Error::InvalidValue);
        }

        max_payload
            .checked_add(sentinel.len())
            .ok_or(Error::Overflow)?;
        Ok(Self {
            item,
            sentinel,
            max_payload,
        })
    }

    fn collision(&self, bytes: &[u8]) -> bool {
        (0..bytes.len()).any(|start| {
            bytes[start..]
                .iter()
                .chain(self.sentinel)
                .take(self.sentinel.len())
                .eq(self.sentinel)
        })
    }
}

impl<C: Encoder<T>, T: ?Sized> Encoder<T> for Sentinel<'_, C> {
    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        let length = self.item.encoded_len(value)?;

        if length > self.max_payload {
            return Err(Error::LimitExceeded);
        }

        length
            .checked_add(self.sentinel.len())
            .ok_or(Error::Overflow)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;
        let bytes = self.item.encode(value, self.max_payload)?;

        if self.collision(&bytes) {
            return Err(Error::InvalidValue);
        }

        writer.put(&bytes)?;
        writer.put(self.sentinel)
    }
}

impl<C: Decoder<T>, T> Decoder<T> for Sentinel<'_, C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        let maximum = self.max_payload + self.sentinel.len();
        let remaining = reader.remaining();
        let search = &remaining[..remaining.len().min(maximum)];
        let position = search
            .windows(self.sentinel.len())
            .position(|window| window == self.sentinel)
            .ok_or(Error::InvalidDiscriminator)?;
        let value = reader.frame(position, |inner| self.item.read(inner))?;
        reader.take(self.sentinel.len())?;
        Ok(value)
    }
}
