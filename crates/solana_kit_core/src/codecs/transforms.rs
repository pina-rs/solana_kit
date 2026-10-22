//! Exact constants and bounded byte-length framing.

use super::{Decoder, Encoder, Error, Number, Reader, Writer};

/// Exact bytes carrying no value; decoding validates every byte.
#[derive(Clone, Copy, Debug)]
pub struct Constant<'a>(pub &'a [u8]);

impl Encoder<()> for Constant<'_> {
    fn encoded_len(&self, _: &()) -> Result<usize, Error> {
        Ok(self.0.len())
    }

    fn write(&self, _: &(), writer: &mut Writer<'_>) -> Result<(), Error> {
        writer.put(self.0)
    }
}

impl Decoder<()> for Constant<'_> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<(), Error> {
        if reader.take(self.0.len())? != self.0 {
            return Err(Error::InvalidDiscriminator);
        }

        Ok(())
    }
}

/// Validates exact prefix and suffix constants around a self-delimiting codec.
/// The suffix begins immediately after the inner codec's consumed bytes.
#[derive(Clone, Copy, Debug)]
pub struct Affix<'a, C> {
    /// Bytes before the value.
    pub prefix: &'a [u8],
    /// Codec for the value.
    pub item: C,
    /// Bytes after the value.
    pub suffix: &'a [u8],
}

impl<C, T: ?Sized> Encoder<T> for Affix<'_, C>
where
    C: Encoder<T>,
{
    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        self.prefix
            .len()
            .checked_add(self.item.encoded_len(value)?)
            .and_then(|size| size.checked_add(self.suffix.len()))
            .ok_or(Error::Overflow)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        writer.put(self.prefix)?;
        self.item.write(value, writer)?;
        writer.put(self.suffix)
    }
}

impl<C, T> Decoder<T> for Affix<'_, C>
where
    C: Decoder<T>,
{
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        Constant(self.prefix).read(reader)?;
        let value = self.item.read(reader)?;
        Constant(self.suffix).read(reader)?;
        Ok(value)
    }
}

/// Little-endian `u32` byte-length framing. The inner decoder must consume its
/// entire declared frame; it cannot read into following fields. Nested frames
/// share the same allocation budget. Oversized or truncated lengths fail.
#[derive(Clone, Copy, Debug)]
pub struct SizePrefix<C>(pub C);

/// Little-endian `u64` byte-length framing, as used by bincode strings.
/// This is distinct from Borsh's [`SizePrefix`] and has the same exact frame
/// consumption and shared allocation budget guarantees.
#[derive(Clone, Copy, Debug)]
pub struct SizePrefix64<C>(pub C);

/// Implements the same checked frame contract for each explicit prefix width.
macro_rules! size_prefix {
    ($name:ident, $width:ty) => {
        impl<C, T: ?Sized> Encoder<T> for $name<C>
        where
            C: Encoder<T>,
        {
            fn encoded_len(&self, value: &T) -> Result<usize, Error> {
                let size = self.0.encoded_len(value)?;
                <$width>::try_from(size).map_err(|_| Error::Overflow)?;
                size.checked_add(core::mem::size_of::<$width>())
                    .ok_or(Error::Overflow)
            }

            fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
                let size =
                    <$width>::try_from(self.0.encoded_len(value)?).map_err(|_| Error::Overflow)?;
                Number::<$width>::LE.write(&size, writer)?;
                writer.frame(
                    usize::try_from(size).map_err(|_| Error::Overflow)?,
                    |inner| self.0.write(value, inner),
                )
            }
        }

        impl<C, T> Decoder<T> for $name<C>
        where
            C: Decoder<T>,
        {
            fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
                let size = usize::try_from(Number::<$width>::LE.read(reader)?)
                    .map_err(|_| Error::Overflow)?;
                reader.frame(size, |inner| self.0.read(inner))
            }
        }
    };
}

size_prefix!(SizePrefix, u32);
size_prefix!(SizePrefix64, u64);
