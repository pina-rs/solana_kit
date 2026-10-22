use core::cmp::Ordering;
use num_bigint::BigInt;

use super::{Error, Rounding, Shape, Signedness, shape::divide};

/// An exact scaled integer whose immutable shape and raw range are validated.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FixedPoint<const RADIX: u8> {
    raw: BigInt,
    shape: Shape<RADIX>,
}

impl<const RADIX: u8> Shape<RADIX> {
    /// Wraps a raw scaled integer, rejecting out-of-range values.
    pub fn raw(self, raw: impl Into<BigInt>) -> Result<FixedPoint<RADIX>, Error> {
        let raw = raw.into();
        self.validate(&raw)?;

        Ok(FixedPoint { raw, shape: self })
    }

    /// Converts a ratio exactly, applying explicit rounding and range checking.
    /// Numerator and denominator are limited to 32768 magnitude bits each.
    pub fn ratio(
        self,
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
        rounding: Rounding,
    ) -> Result<FixedPoint<RADIX>, Error> {
        let numerator = numerator.into();
        let denominator = denominator.into();
        bounded_operand(&numerator)?;
        bounded_operand(&denominator)?;
        self.raw(divide(numerator * self.factor(), &denominator, rounding)?)
    }
}

impl<const RADIX: u8> FixedPoint<RADIX> {
    /// Borrows the exact raw scaled integer.
    pub fn raw(&self) -> &BigInt {
        &self.raw
    }
    /// Returns the validated storage shape.
    pub const fn shape(&self) -> Shape<RADIX> {
        self.shape
    }
    /// Adds values with exactly matching shapes.
    pub fn checked_add(&self, other: &Self) -> Result<Self, Error> {
        self.same_shape(other)?;
        self.shape.raw(&self.raw + &other.raw)
    }
    /// Subtracts values with exactly matching shapes.
    pub fn checked_sub(&self, other: &Self) -> Result<Self, Error> {
        self.same_shape(other)?;
        self.shape.raw(&self.raw - &other.raw)
    }
    /// Compares matching scales, allowing different widths and signedness.
    pub fn compare(&self, other: &Self) -> Result<Ordering, Error> {
        if self.shape.scale() != other.shape.scale() {
            return Err(Error::ShapeMismatch);
        }

        Ok(self.raw.cmp(&other.raw))
    }
    /// Compares exactly matching shapes, like the reference's `compareTo`.
    pub fn compare_same_shape(&self, other: &Self) -> Result<Ordering, Error> {
        self.same_shape(other)?;

        Ok(self.raw.cmp(&other.raw))
    }
    /// Multiplies by an integer scalar with checked output range.
    pub fn checked_mul_scalar(&self, scalar: impl Into<BigInt>) -> Result<Self, Error> {
        let scalar = scalar.into();
        bounded_operand(&scalar)?;
        self.shape.raw(&self.raw * scalar)
    }
    /// Divides by an integer scalar with explicit rounding.
    pub fn checked_div_scalar(
        &self,
        scalar: impl Into<BigInt>,
        rounding: Rounding,
    ) -> Result<Self, Error> {
        let scalar = scalar.into();
        bounded_operand(&scalar)?;
        self.shape.raw(divide(self.raw.clone(), &scalar, rounding)?)
    }
    /// Multiplies matching signedness values; the result uses the left shape.
    pub fn checked_mul(&self, other: &Self, rounding: Rounding) -> Result<Self, Error> {
        self.same_signedness(other)?;
        self.shape.raw(divide(
            &self.raw * &other.raw,
            &other.shape.factor(),
            rounding,
        )?)
    }
    /// Divides matching signedness values; the result uses the left shape.
    pub fn checked_div(&self, other: &Self, rounding: Rounding) -> Result<Self, Error> {
        self.same_signedness(other)?;
        self.shape.raw(divide(
            &self.raw * other.shape.factor(),
            &other.raw,
            rounding,
        )?)
    }
    /// Negates signed values; signed minimum overflows rather than wrapping.
    pub fn checked_neg(&self) -> Result<Self, Error> {
        if self.shape.signedness() != Signedness::Signed {
            return Err(Error::UnsignedNegation);
        }

        self.shape.raw(-&self.raw)
    }
    /// Returns the absolute value, rejecting signed-minimum overflow.
    pub fn checked_abs(&self) -> Result<Self, Error> {
        self.shape.raw(BigInt::from(self.raw.magnitude().clone()))
    }
    /// Changes signedness, preserving width and scale and validating the range.
    pub fn with_signedness(&self, signedness: Signedness) -> Result<Self, Error> {
        Shape::new(signedness, self.shape.total_bits(), self.shape.scale())?.raw(self.raw.clone())
    }
    /// Changes width and scale, preserving signedness and applying rounding.
    pub fn rescale(&self, total_bits: u32, scale: u32, rounding: Rounding) -> Result<Self, Error> {
        let target = Shape::new(self.shape.signedness(), total_bits, scale)?;
        let raw = divide(&self.raw * target.factor(), &self.shape.factor(), rounding)?;
        target.raw(raw)
    }

    fn same_shape(&self, other: &Self) -> Result<(), Error> {
        if self.shape != other.shape {
            return Err(Error::ShapeMismatch);
        }

        Ok(())
    }

    fn same_signedness(&self, other: &Self) -> Result<(), Error> {
        if self.shape.signedness() != other.shape.signedness() {
            return Err(Error::ShapeMismatch);
        }

        Ok(())
    }
}

pub(super) fn bounded_operand(value: &BigInt) -> Result<(), Error> {
    if value.bits() > 32768 {
        return Err(Error::LimitExceeded);
    }

    Ok(())
}
