//! Typed conversions without erased values or hidden allocation.

use core::marker::PhantomData;

use super::{Decoder, Encoder, Error, Reader, Writer};

/// Pairs an independent encoder and decoder without changing their contracts.
/// Unlike Dart, size metadata is not inferred or checked at construction.
/// Each operation still enforces its exact size/full-consumption contract.
#[derive(Clone, Copy, Debug)]
pub struct Paired<E, D>(pub E, pub D);

impl<E: Encoder<T>, D, T: ?Sized> Encoder<T> for Paired<E, D> {
    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        self.0.encoded_len(value)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write(value, writer)
    }
}

impl<E, D: Decoder<T>, T> Decoder<T> for Paired<E, D> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        self.1.read(reader)
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
    fn read(&self, reader: &mut Reader<'_>) -> Result<U, Error> {
        (self.decode)(self.item.read(reader)?)
    }
}

/// Observes or rejects values before writing and after reading.
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
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        let value = self.item.read(reader)?;
        (self.after_read)(&value)?;
        Ok(value)
    }
}
