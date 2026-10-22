const DECIMAL_BINARY_FACTOR: u8 = 5;

use alloc::{format, string::String};
use core::fmt;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

use super::{
    Error, FixedPoint, MAX_INPUT_BYTES, MAX_SCALE, Rounding, Shape, Signedness, shape::divide,
    value::bounded_operand,
};

/// Decimal formatting options, applied exactly before rendering.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FormatOptions {
    /// Target decimal places; absent means the exact native decimal scale.
    pub decimals: Option<u32>,
    /// Retain fractional trailing zeros at the selected scale.
    pub pad_trailing_zeros: bool,
    /// Rounding when reducing decimal precision.
    pub rounding: Rounding,
}

/// Exact base-10 scaled integer representation, including binary conversions.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Base10 {
    raw: BigInt,
    decimals: u32,
}

impl Base10 {
    /// Validates a bounded base-10 integer and decimal scale.
    pub fn new(raw: impl Into<BigInt>, decimals: u32) -> Result<Self, Error> {
        let raw = raw.into();
        bounded_operand(&raw)?;

        if decimals > MAX_SCALE {
            return Err(Error::LimitExceeded);
        }

        Ok(Self { raw, decimals })
    }
    /// Borrows the base-10 scaled integer.
    pub fn raw(&self) -> &BigInt {
        &self.raw
    }
    /// Decimal fractional digit count.
    pub const fn decimals(&self) -> u32 {
        self.decimals
    }
    /// Applies decimal precision and rounding, without storage-width constraints.
    pub fn with_options(&self, options: FormatOptions) -> Result<Self, Error> {
        let decimals = options.decimals.unwrap_or(self.decimals);

        if decimals > MAX_SCALE {
            return Err(Error::LimitExceeded);
        }

        let raw = divide(
            &self.raw * BigInt::from(10).pow(decimals),
            &BigInt::from(10).pow(self.decimals),
            options.rounding,
        )?;
        Self::new(raw, decimals)
    }
    /// Formats exactly, optionally retaining fractional zeros.
    pub fn format(&self, options: FormatOptions) -> Result<String, Error> {
        let value = self.with_options(options)?;
        Ok(format_scaled(
            &value.raw,
            value.decimals,
            options.pad_trailing_zeros,
        ))
    }
    /// Precision-preserving input for an application-provided number formatter.
    pub fn scientific_notation(&self) -> String {
        format!("{}E-{}", self.raw, self.decimals)
    }
}

impl<const RADIX: u8> Shape<RADIX> {
    /// Parses ASCII decimal text with optional minus and decimal point.
    /// Whitespace, exponents and plus signs are rejected. Decimal strict parsing
    /// rejects excess fractional digits even if all excess digits are zero.
    pub fn parse(self, input: &str, rounding: Rounding) -> Result<FixedPoint<RADIX>, Error> {
        if input.len() > MAX_INPUT_BYTES {
            return Err(Error::LimitExceeded);
        }

        let negative = input.starts_with('-');
        let unsigned = input.strip_prefix('-').unwrap_or(input);
        let (whole, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
        let digits = whole.len() + fraction.len();

        if digits == 0
            || !whole
                .bytes()
                .chain(fraction.bytes())
                .all(|byte| byte.is_ascii_digit())
            || (negative && self.signedness() == Signedness::Unsigned)
        {
            return Err(Error::InvalidSyntax);
        }

        let decimals = u32::try_from(fraction.len()).map_err(|_| Error::LimitExceeded)?;

        if RADIX == 10 && decimals > self.scale() && rounding == Rounding::Strict {
            return Err(Error::PrecisionLoss);
        }

        let mut text = String::with_capacity(digits);
        text.push_str(whole);
        text.push_str(fraction);
        let magnitude = BigInt::parse_bytes(text.as_bytes(), 10).ok_or(Error::InvalidSyntax)?;
        let numerator = if negative { -magnitude } else { magnitude };
        self.ratio(numerator, BigInt::from(10).pow(decimals), rounding)
    }
}

impl<const RADIX: u8> FixedPoint<RADIX> {
    /// Exact base-10 representation; binary scales multiply raw by `5^scale`.
    pub fn to_base10(&self) -> Base10 {
        let decimals = self.shape().scale();
        let raw = if RADIX == 2 {
            self.raw() * BigInt::from(DECIMAL_BINARY_FACTOR).pow(decimals)
        } else {
            self.raw().clone()
        };

        Base10 { raw, decimals }
    }
    /// Formats without floating-point conversion.
    pub fn format(&self, options: FormatOptions) -> Result<String, Error> {
        self.to_base10().format(options)
    }
    /// Calls an application formatter with exact scientific notation.
    pub fn format_with<T>(&self, formatter: impl FnOnce(&str) -> T) -> T {
        formatter(&self.to_base10().scientific_notation())
    }
    /// Explicitly lossy floating-point conversion for presentation only.
    /// Exact arithmetic, parsing, formatting and codecs never call this method.
    pub fn to_number(&self) -> Result<f64, Error> {
        let scale = BigInt::from(RADIX).pow(self.shape().scale());
        let denominator = scale.to_f64().ok_or(Error::Overflow)?;

        if RADIX == 2 {
            let integer = (self.raw() / &scale).to_f64().ok_or(Error::Overflow)?;
            let fraction = (self.raw() % scale).to_f64().ok_or(Error::Overflow)?;

            return Ok(integer + fraction / denominator);
        }

        Ok(self.raw().to_f64().ok_or(Error::Overflow)? / denominator)
    }
}

impl<const RADIX: u8> fmt::Display for FixedPoint<RADIX> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.to_base10();
        formatter.write_str(&format_scaled(&value.raw, value.decimals, false))
    }
}

/// Preserve the raw sign while padding and trimming only fractional zeros.
fn format_scaled(raw: &BigInt, decimals: u32, pad: bool) -> String {
    let digits = raw.magnitude().to_str_radix(10);
    let sign = if raw.sign() == num_bigint::Sign::Minus {
        "-"
    } else {
        ""
    };

    if decimals == 0 {
        return format!("{sign}{digits}");
    }

    let decimals = usize::try_from(decimals).expect("validated scale fits every supported target");
    let padded = format!("{:0>width$}", digits, width = decimals + 1);
    let split = padded.len() - decimals;
    let (whole, fraction) = padded.split_at(split);
    let fraction = if pad {
        fraction
    } else {
        fraction.trim_end_matches('0')
    };

    if fraction.is_empty() {
        return format!("{sign}{whole}");
    }

    format!("{sign}{whole}.{fraction}")
}
