//! Exact-width numbers and canonical Solana short-u16.

use core::marker::PhantomData;

use super::{Decoder, Encoder, Error, Reader, Writer};

/// Checks an inclusive caller-selected range without converting the number.
/// Like the reference assertions, unordered values such as NaN are not outside
/// either bound. Use integer types for integer codecs and explicit finite checks
/// when a caller's floating-point contract requires them.
pub fn ensure_number_range<T: PartialOrd>(value: &T, min: &T, max: &T) -> Result<(), Error> {
    if value < min || value > max {
        return Err(Error::InvalidValue);
    }

    Ok(())
}

/// Byte order, independent of the host architecture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endian {
    /// Least significant byte first (Solana's usual order).
    Little,
    /// Most significant byte first.
    Big,
}

impl Endian {
    /// Selects the wire order without branching inside numeric implementations.
    fn choose<T>(self, little: T, big: T) -> T {
        match self {
            Self::Little => little,
            Self::Big => big,
        }
    }

    /// Reorders a little-endian byte array for the selected wire order.
    fn order<const N: usize>(self, mut bytes: [u8; N]) -> [u8; N] {
        if self == Self::Big {
            bytes.reverse();
        }

        bytes
    }
}

/// Exact-width numeric codec. Floats preserve all IEEE 754 bit patterns,
/// including infinities, signed zero, and NaN payloads, without normalization.
#[derive(Clone, Copy, Debug)]
pub struct Number<T> {
    endian: Endian,
    marker: PhantomData<T>,
}

impl<T> Number<T> {
    /// Selects a wire byte order explicitly.
    #[must_use]
    pub const fn new(endian: Endian) -> Self {
        Self {
            endian,
            marker: PhantomData,
        }
    }

    /// Returns the selected wire byte order.
    #[must_use]
    pub const fn endian(&self) -> Endian {
        self.endian
    }

    /// Little-endian numeric codec.
    pub const LE: Self = Self {
        endian: Endian::Little,
        marker: PhantomData,
    };
    /// Big-endian numeric codec.
    pub const BE: Self = Self {
        endian: Endian::Big,
        marker: PhantomData,
    };
}

impl<T> Default for Number<T> {
    fn default() -> Self {
        Self::LE
    }
}

/// Implements native primitive byte contracts with no numeric casts.
macro_rules! number {
    ($($ty:ty),+ $(,)?) => {$(
        impl Encoder<$ty> for Number<$ty> {
            fn encoded_len(&self, _: &$ty) -> Result<usize, Error> {
                Ok(core::mem::size_of::<$ty>())
            }

            fn write(&self, value: &$ty, writer: &mut Writer<'_>) -> Result<(), Error> {
                let bytes = self.endian.choose(value.to_le_bytes(), value.to_be_bytes());
                writer.put(&bytes)
            }
        }

        impl Decoder<$ty> for Number<$ty> {
            fn read(&self, reader: &mut Reader<'_>) -> Result<$ty, Error> {
                let bytes = reader.take(core::mem::size_of::<$ty>())?;
                let array = bytes.try_into().map_err(|_| Error::Truncated)?;

                Ok(self.endian.choose(<$ty>::from_le_bytes(array), <$ty>::from_be_bytes(array)))
            }
        }
    )+};
}

number!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64,);

/// Defines a Kit-owned 256-bit wire value without exposing another SDK's types.
macro_rules! wide {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub struct $name([u8; 32]);

        impl $name {
            /// Constructs the exact little-endian representation.
            #[must_use]
            pub const fn from_le_bytes(bytes: [u8; 32]) -> Self {
                Self(bytes)
            }

            /// Returns the exact little-endian representation.
            #[must_use]
            pub const fn to_le_bytes(self) -> [u8; 32] {
                self.0
            }
        }

        impl Encoder<$name> for Number<$name> {
            fn encoded_len(&self, _: &$name) -> Result<usize, Error> {
                Ok(32)
            }

            fn write(&self, value: &$name, writer: &mut Writer<'_>) -> Result<(), Error> {
                let bytes = self.endian.order(value.0);
                writer.put(&bytes)
            }
        }

        impl Decoder<$name> for Number<$name> {
            fn read(&self, reader: &mut Reader<'_>) -> Result<$name, Error> {
                let bytes: [u8; 32] = reader.take(32)?.try_into().map_err(|_| Error::Truncated)?;

                Ok($name(self.endian.order(bytes)))
            }
        }
    };
}

wide!(
    Uint256,
    "An unsigned 256-bit integer represented by its exact bytes; arithmetic is separate."
);
wide!(
    Int256,
    "A signed two's-complement 256-bit integer represented by its exact bytes."
);

/// Canonical short-u16: one to three little-endian groups of seven bits.
#[derive(Clone, Copy, Debug, Default)]
pub struct ShortU16;

const GROUP_BITS: u32 = 7;
const LAST_SHIFT: u32 = GROUP_BITS * 2;
const LAST_MAX: u8 = 3;
const CONTINUATION: u8 = 0x80;
const PAYLOAD_MASK: u8 = 0x7f;

impl Encoder<u16> for ShortU16 {
    fn encoded_len(&self, value: &u16) -> Result<usize, Error> {
        Ok(if *value < u16::from(CONTINUATION) {
            1
        } else if *value < (1 << LAST_SHIFT) {
            2
        } else {
            3
        })
    }

    fn write(&self, value: &u16, writer: &mut Writer<'_>) -> Result<(), Error> {
        let mut remaining = *value;

        while remaining >= u16::from(CONTINUATION) {
            let byte = u8::try_from(remaining & u16::from(PAYLOAD_MASK))
                .map_err(|_| Error::Overflow)?
                | CONTINUATION;
            writer.put(&[byte])?;
            remaining >>= GROUP_BITS;
        }

        writer.put(&[u8::try_from(remaining).map_err(|_| Error::Overflow)?])
    }
}

impl Decoder<u16> for ShortU16 {
    fn read(&self, reader: &mut Reader<'_>) -> Result<u16, Error> {
        let mut value = 0u16;

        for shift in [0, GROUP_BITS, LAST_SHIFT] {
            let byte = Number::<u8>::LE.read(reader)?;

            if shift == LAST_SHIFT && byte > LAST_MAX {
                return Err(Error::Overflow);
            }

            if shift > 0 && byte == 0 {
                return Err(Error::NonCanonical);
            }

            value |= u16::from(byte & PAYLOAD_MASK) << shift;

            if byte & CONTINUATION == 0 {
                return Ok(value);
            }
        }

        Err(Error::Overflow)
    }
}
