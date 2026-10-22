const BITS_PER_BYTE: u32 = 8;

use super::{Error, FixedPoint, Shape, Signedness};
use crate::codecs::{self, Decoder, Encoder, Endian, Reader, Writer};
use num_bigint::BigInt;

/// Byte-aligned fixed-point codec sharing the bounded core cursor contracts.
#[derive(Clone, Copy, Debug)]
pub struct FixedPointCodec<const RADIX: u8> {
    shape: Shape<RADIX>,
    endian: Endian,
    size: usize,
}

impl<const RADIX: u8> Shape<RADIX> {
    /// Selects little- or big-endian two's-complement wire representation.
    pub fn codec(self, endian: Endian) -> Result<FixedPointCodec<RADIX>, Error> {
        if !self.total_bits().is_multiple_of(BITS_PER_BYTE) {
            return Err(Error::InvalidShape);
        }

        let size =
            usize::try_from(self.total_bits() / BITS_PER_BYTE).map_err(|_| Error::LimitExceeded)?;
        Ok(FixedPointCodec {
            shape: self,
            endian,
            size,
        })
    }
}

impl<const RADIX: u8> FixedPointCodec<RADIX> {
    /// Exact encoded width in bytes.
    pub const fn fixed_size(&self) -> usize {
        self.size
    }
    /// Expected validated shape.
    pub const fn shape(&self) -> Shape<RADIX> {
        self.shape
    }
}

impl<const RADIX: u8> Encoder<FixedPoint<RADIX>> for FixedPointCodec<RADIX> {
    fn encoded_len(&self, value: &FixedPoint<RADIX>) -> Result<usize, codecs::Error> {
        if value.shape() != self.shape {
            return Err(codecs::Error::InvalidValue);
        }

        Ok(self.size)
    }

    /// Encode only validated raw ranges; pad with the sign byte without truncating.
    fn write(
        &self,
        value: &FixedPoint<RADIX>,
        writer: &mut Writer<'_>,
    ) -> Result<(), codecs::Error> {
        self.encoded_len(value)?;
        let mut bytes = if self.shape.signedness() == Signedness::Signed {
            value.raw().to_signed_bytes_le()
        } else {
            value.raw().to_bytes_le().1
        };

        if bytes.len() > self.size {
            return Err(codecs::Error::Overflow);
        }

        let fill = if value.raw().sign() == num_bigint::Sign::Minus {
            255
        } else {
            0
        };

        bytes.resize(self.size, fill);

        if self.endian == Endian::Big {
            bytes.reverse();
        }

        writer.put(&bytes)
    }
}

impl<const RADIX: u8> Decoder<FixedPoint<RADIX>> for FixedPointCodec<RADIX> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<FixedPoint<RADIX>, codecs::Error> {
        reader.claim(self.size)?;
        let bytes = reader.take(self.size)?;

        let raw = match (self.shape.signedness(), self.endian) {
            (Signedness::Signed, Endian::Little) => BigInt::from_signed_bytes_le(bytes),
            (Signedness::Signed, Endian::Big) => BigInt::from_signed_bytes_be(bytes),
            (Signedness::Unsigned, Endian::Little) => {
                BigInt::from_bytes_le(num_bigint::Sign::Plus, bytes)
            }
            (Signedness::Unsigned, Endian::Big) => {
                BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes)
            }
        };

        self.shape.raw(raw).map_err(|_| codecs::Error::InvalidValue)
    }
}
