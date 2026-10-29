//! Typed conversions without erased values or hidden allocation.

use alloc::vec::Vec;
use core::{fmt, marker::PhantomData};

use super::{Decoder, Encoder, Error, Limits, PairError, Reader, Size, Writer};

/// Pairs independent operations after checking their selected size declarations.
///
/// Agreement does not prove inverses or equal cursor movement. The selected input
/// and output types can differ. Each complete operation delegates to its owner,
/// preserving custom owned-operation overrides and their budgets and claims.
pub struct Paired<E, D, From: ?Sized, To> {
    encoder: E,
    decoder: D,
    values: PhantomData<fn(&From) -> To>,
}

impl<E: Encoder<From>, D: Decoder<To>, From: ?Sized, To> Paired<E, D, From, To> {
    /// Checks both declarations once without sizing values, reading or writing.
    ///
    /// # Errors
    /// Returns the original metadata failure, or rejects unknown classes, mixed
    /// fixed and variable classes, unequal widths or unequal optional maxima.
    pub fn try_new(encoder: E, decoder: D) -> Result<Self, PairError> {
        PairError::check(encoder.encode_size()?, decoder.decode_size()?)?;

        Ok(Self {
            encoder,
            decoder,
            values: PhantomData,
        })
    }

    /// Consumes the checked identity and returns its independent parts.
    #[must_use]
    pub fn into_parts(self) -> (E, D) {
        (self.encoder, self.decoder)
    }
}

impl<E: Clone, D: Clone, From: ?Sized, To> Clone for Paired<E, D, From, To> {
    fn clone(&self) -> Self {
        Self {
            encoder: self.encoder.clone(),
            decoder: self.decoder.clone(),
            values: PhantomData,
        }
    }
}

impl<E: Copy, D: Copy, From: ?Sized, To> Copy for Paired<E, D, From, To> {}

impl<E: fmt::Debug, D: fmt::Debug, From: ?Sized, To> fmt::Debug for Paired<E, D, From, To> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Paired")
            .field("encoder", &self.encoder)
            .field("decoder", &self.decoder)
            .finish()
    }
}

impl<E: Encoder<T>, D, T: ?Sized, To> Encoder<T> for Paired<E, D, T, To> {
    fn encode_size(&self) -> Result<Size, Error> {
        self.encoder.encode_size()
    }

    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        self.encoder.encoded_len(value)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoder.write(value, writer)
    }

    fn encode(&self, value: &T, max_bytes: usize) -> Result<Vec<u8>, Error> {
        self.encoder.encode(value, max_bytes)
    }
}

impl<E, D: Decoder<T>, From: ?Sized, T> Decoder<T> for Paired<E, D, From, T> {
    fn decode_size(&self) -> Result<Size, Error> {
        self.decoder.decode_size()
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        self.decoder.read(reader)
    }

    fn decode(&self, bytes: &[u8], limits: Limits) -> Result<T, Error> {
        self.decoder.decode(bytes, limits)
    }

    fn decode_prefix(
        &self,
        bytes: &[u8],
        offset: usize,
        limits: Limits,
    ) -> Result<(T, usize), Error> {
        self.decoder.decode_prefix(bytes, offset, limits)
    }
}

/// Fallible value conversion around a concrete codec.
/// Conversion callbacks run during sizing and writing and must be deterministic.
/// Output conversion runs after reading; failures retain consumed bytes and claims.
/// Callbacks own any allocations they make and must bound them independently.
#[derive(Clone, Copy, Debug)]
pub struct Mapped<C, E, D, T> {
    item: C,
    encode: E,
    decode: D,
    source: PhantomData<fn() -> T>,
}

impl<C, E, D, T> Mapped<C, E, D, T> {
    /// Converts incoming values to `T` and decoded `T` values to the output type.
    pub const fn new(item: C, encode: E, decode: D) -> Self {
        Self {
            item,
            encode,
            decode,
            source: PhantomData,
        }
    }
}

impl<C, E, D, T, U: ?Sized> Encoder<U> for Mapped<C, E, D, T>
where
    C: Encoder<T>,
    E: Fn(&U) -> Result<T, Error>,
{
    fn encode_size(&self) -> Result<Size, Error> {
        self.item.encode_size()
    }

    fn encoded_len(&self, value: &U) -> Result<usize, Error> {
        self.item.encoded_len(&(self.encode)(value)?)
    }

    fn write(&self, value: &U, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.item.write(&(self.encode)(value)?, writer)
    }
}

impl<C, E, D, T, U> Decoder<U> for Mapped<C, E, D, T>
where
    C: Decoder<T>,
    D: Fn(T) -> Result<U, Error>,
{
    fn decode_size(&self) -> Result<Size, Error> {
        self.item.decode_size()
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<U, Error> {
        (self.decode)(self.item.read(reader)?)
    }
}

/// Observes or rejects values before writing and after reading.
/// Preserves the inner codec's directional size declarations.
/// Size calculation does not invoke the observer. Callbacks must not allocate
/// without their own limits. Byte-buffer observation is intentionally separate.
#[derive(Clone, Copy, Debug)]
pub struct Checked<C, E, D> {
    /// Codec whose values are observed.
    pub item: C,
    /// Runs before writing any inner bytes.
    pub before_write: E,
    /// Runs after the inner value has been read.
    pub after_read: D,
}

impl<C, E, D, T: ?Sized> Encoder<T> for Checked<C, E, D>
where
    C: Encoder<T>,
    E: Fn(&T) -> Result<(), Error>,
{
    fn encode_size(&self) -> Result<Size, Error> {
        self.item.encode_size()
    }

    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        self.item.encoded_len(value)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        (self.before_write)(value)?;
        self.item.write(value, writer)
    }
}

impl<C, E, D, T> Decoder<T> for Checked<C, E, D>
where
    C: Decoder<T>,
    D: Fn(&T) -> Result<(), Error>,
{
    fn decode_size(&self) -> Result<Size, Error> {
        self.item.decode_size()
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        let value = self.item.read(reader)?;
        (self.after_read)(&value)?;
        Ok(value)
    }
}
