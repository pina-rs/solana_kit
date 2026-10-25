//! Arbitrary positive byte widths using optional, host-independent big integers.

use alloc::vec::Vec;
use core::{marker::PhantomData, num::NonZeroUsize};
use num_bigint::{BigInt, BigUint, Sign};

use super::{Decoder, Encoder, Endian, Error, Reader, Size, Writer};

const BITS_PER_BYTE: usize = 8;

/// An explicitly sized signed or unsigned integer codec.
///
/// Use `BigNumber<BigUint>` for unsigned values and `BigNumber<BigInt>` for
/// signed two's-complement values. Zero-width formats are rejected. Encoding
/// uses temporary bytes proportional to the supplied value; decoding claims
/// the complete wire width before the big integer allocates its limbs.
#[derive(Clone, Debug)]
pub struct BigNumber<T> {
    bytes: NonZeroUsize,
    endian: Endian,
    value: PhantomData<fn() -> T>,
}

impl<T> BigNumber<T> {
    /// Chooses a nonzero width and byte order, rejecting bit-width overflow.
    pub fn new(bytes: usize, endian: Endian) -> Result<Self, Error> {
        bytes.checked_mul(BITS_PER_BYTE).ok_or(Error::Overflow)?;
        let bytes = NonZeroUsize::new(bytes).ok_or(Error::InvalidValue)?;

        Ok(Self {
            bytes,
            endian,
            value: PhantomData,
        })
    }

    /// Returns the complete wire width in bytes.
    #[must_use]
    pub const fn width(&self) -> usize {
        self.bytes.get()
    }

    fn write_bytes(
        &self,
        mut bytes: Vec<u8>,
        fill: u8,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        let padding = self
            .width()
            .checked_sub(bytes.len())
            .ok_or(Error::NumberOutOfRange)?;

        if self.endian == Endian::Big {
            bytes.reverse();
            writer.fill(padding, fill)?;

            return writer.put(&bytes);
        }

        writer.put(&bytes)?;
        writer.fill(padding, fill)
    }
}

impl Encoder<BigUint> for BigNumber<BigUint> {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(self.width()))
    }

    fn encoded_len(&self, value: &BigUint) -> Result<usize, Error> {
        let bits = u64::try_from(self.width())
            .map_err(|_| Error::Overflow)?
            .checked_mul(u64::from(u8::BITS))
            .ok_or(Error::Overflow)?;

        if value.bits() > bits {
            return Err(Error::NumberOutOfRange);
        }

        Ok(self.width())
    }

    fn write(&self, value: &BigUint, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;
        self.write_bytes(value.to_bytes_le(), 0, writer)
    }
}

impl Decoder<BigUint> for BigNumber<BigUint> {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(self.width()))
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<BigUint, Error> {
        reader.claim(self.width())?;
        let bytes = reader.take(self.width())?;

        Ok(match self.endian {
            Endian::Little => BigUint::from_bytes_le(bytes),
            Endian::Big => BigUint::from_bytes_be(bytes),
        })
    }
}

impl Encoder<BigInt> for BigNumber<BigInt> {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(self.width()))
    }

    fn encoded_len(&self, value: &BigInt) -> Result<usize, Error> {
        if value.to_signed_bytes_le().len() > self.width() {
            return Err(Error::NumberOutOfRange);
        }

        Ok(self.width())
    }

    fn write(&self, value: &BigInt, writer: &mut Writer<'_>) -> Result<(), Error> {
        let fill = if value.sign() == Sign::Minus {
            u8::MAX
        } else {
            0
        };

        self.write_bytes(value.to_signed_bytes_le(), fill, writer)
    }
}

impl Decoder<BigInt> for BigNumber<BigInt> {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(self.width()))
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<BigInt, Error> {
        reader.claim(self.width())?;
        let bytes = reader.take(self.width())?;

        Ok(match self.endian {
            Endian::Little => BigInt::from_signed_bytes_le(bytes),
            Endian::Big => BigInt::from_signed_bytes_be(bytes),
        })
    }
}
