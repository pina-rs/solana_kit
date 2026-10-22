use core::fmt;
use num_bigint::BigInt;

/// Maximum validated raw storage width.
pub const MAX_TOTAL_BITS: u32 = 4096;
/// Maximum decimal places or binary fractional bits.
pub const MAX_SCALE: u32 = 4096;
/// Maximum UTF-8 input bytes accepted by decimal parsers.
pub const MAX_INPUT_BYTES: usize = 8192;

/// Whether negative raw values are representable.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Signedness {
    /// Zero and positive values.
    Unsigned,
    /// Two's-complement signed range.
    Signed,
}

/// Rounding applied to an exact rational result.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Rounding {
    /// Reject precision loss.
    #[default]
    Strict,
    /// Toward negative infinity.
    Floor,
    /// Toward positive infinity.
    Ceil,
    /// Toward zero.
    Trunc,
    /// Nearest, with ties away from zero.
    Nearest,
}

/// An invalid fixed-point operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Width, scale, radix, or byte alignment is invalid.
    InvalidShape,
    /// A caller-controlled input exceeded an explicit resource bound.
    LimitExceeded,
    /// Text does not match the decimal grammar.
    InvalidSyntax,
    /// A result is outside the destination raw range.
    Overflow,
    /// Operands do not have the required matching shape or signedness.
    ShapeMismatch,
    /// A denominator was zero.
    DivisionByZero,
    /// Strict rounding would discard precision.
    PrecisionLoss,
    /// Negation requires a signed shape.
    UnsignedNegation,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "fixed-point error: {self:?}")
    }
}

impl core::error::Error for Error {}

/// Immutable validated fixed-point storage metadata; radix is 2 or 10.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Shape<const RADIX: u8> {
    signedness: Signedness,
    total_bits: u32,
    scale: u32,
}

impl<const RADIX: u8> Shape<RADIX> {
    /// Validates radix, width, and scale before any proportional allocation.
    pub fn new(signedness: Signedness, total_bits: u32, scale: u32) -> Result<Self, Error> {
        if !matches!(RADIX, 2 | 10) || total_bits == 0 {
            return Err(Error::InvalidShape);
        }

        if total_bits > MAX_TOTAL_BITS || scale > MAX_SCALE {
            return Err(Error::LimitExceeded);
        }

        Ok(Self {
            signedness,
            total_bits,
            scale,
        })
    }

    /// Signedness of the raw integer.
    pub const fn signedness(self) -> Signedness {
        self.signedness
    }
    /// Raw storage width in bits.
    pub const fn total_bits(self) -> u32 {
        self.total_bits
    }
    /// Decimal places or binary fractional bits.
    pub const fn scale(self) -> u32 {
        self.scale
    }
    /// Inclusive raw integer range.
    pub fn raw_range(self) -> (BigInt, BigInt) {
        let signed = self.signedness == Signedness::Signed;
        let high = BigInt::from(1) << (self.total_bits - u32::from(signed));
        let min = if signed { -&high } else { BigInt::from(0) };

        (min, high - 1)
    }

    pub(super) fn factor(self) -> BigInt {
        BigInt::from(RADIX).pow(self.scale)
    }

    pub(super) fn validate(self, raw: &BigInt) -> Result<(), Error> {
        let (min, max) = self.raw_range();

        if raw < &min || raw > &max {
            return Err(Error::Overflow);
        }

        Ok(())
    }
}

pub(super) fn divide(
    numerator: BigInt,
    denominator: &BigInt,
    rounding: Rounding,
) -> Result<BigInt, Error> {
    if denominator == &BigInt::from(0) {
        return Err(Error::DivisionByZero);
    }

    let quotient = &numerator / denominator;
    let remainder = &numerator % denominator;

    if remainder == BigInt::from(0) {
        return Ok(quotient);
    }

    let positive = numerator.sign() == denominator.sign();
    let direction = if positive { 1 } else { -1 };
    let increment = match rounding {
        Rounding::Strict => return Err(Error::PrecisionLoss),
        Rounding::Trunc => false,
        Rounding::Floor => !positive,
        Rounding::Ceil => positive,
        Rounding::Nearest => remainder.magnitude() * 2u8 >= *denominator.magnitude(),
    };

    Ok(if increment {
        quotient + direction
    } else {
        quotient
    })
}
