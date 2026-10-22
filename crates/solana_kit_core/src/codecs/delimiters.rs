//! Bounded byte delimiters and deterministic padding.

use super::{Decoder, Encoder, Error, FrameConsumption, Reader, Writer};

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
    dart: bool,
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
            dart: false,
        })
    }

    /// Selects Dart delimiter behavior with an explicit scratch/payload bound.
    /// Empty delimiters reject nonempty writes, accept empty writes, and read
    /// an empty payload only when input is nonempty. Complete payload matches
    /// reject; append-boundary overlap is accepted. Reads skip unused payload.
    /// No delimiter-width cap applies, unlike the exact constructor.
    pub fn dart(item: C, sentinel: &'a [u8], max_payload: usize) -> Result<Self, Error> {
        max_payload
            .checked_add(sentinel.len())
            .ok_or(Error::Overflow)?;
        Ok(Self {
            item,
            sentinel,
            max_payload,
            dart: true,
        })
    }

    fn collision(&self, bytes: &[u8]) -> bool {
        if self.dart {
            return self.dart_collision(bytes);
        }

        (0..bytes.len()).any(|start| {
            bytes[start..]
                .iter()
                .chain(self.sentinel)
                .take(self.sentinel.len())
                .eq(self.sentinel)
        })
    }

    fn dart_collision(&self, bytes: &[u8]) -> bool {
        if self.sentinel.is_empty() {
            return !bytes.is_empty();
        }

        bytes
            .windows(self.sentinel.len())
            .any(|window| window == self.sentinel)
    }

    fn position(&self, remaining: &[u8]) -> Result<usize, Error> {
        if self.sentinel.is_empty() && remaining.is_empty() {
            return Err(Error::InvalidDiscriminator);
        }

        if self.sentinel.is_empty() {
            return Ok(0);
        }

        let maximum = self.max_payload + self.sentinel.len();
        let search = &remaining[..remaining.len().min(maximum)];
        search
            .windows(self.sentinel.len())
            .position(|window| window == self.sentinel)
            .ok_or(Error::InvalidDiscriminator)
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
        let position = self.position(reader.remaining())?;
        let consumption = if self.dart {
            FrameConsumption::Skip
        } else {
            FrameConsumption::Exact
        };

        let value = consumption.read(&self.item, reader, position)?;
        reader.take(self.sentinel.len())?;
        Ok(value)
    }
}
