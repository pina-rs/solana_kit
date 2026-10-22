//! Exact 256-bit numeric text conversions; no floating point or heap scratch.
use super::{Error, Int256, Uint256};
const MAX_RADIX: u32 = 36;
const MAX_TEXT_BYTES: usize = 4096;
const SIGN_INDEX: usize = 31;
const SIGN_BIT: u8 = 128;
const BYTE_RADIX: u16 = 256;
use core::{fmt, str::FromStr};

/// Parses a nonnegative magnitude, with checked multiply/add for all 32 bytes.
fn magnitude(value: &str, radix: u32) -> Result<[u8; 32], Error> {
    if !(2..=MAX_RADIX).contains(&radix) || value.is_empty() {
        return Err(Error::InvalidValue);
    }

    if value.len() > MAX_TEXT_BYTES {
        return Err(Error::LimitExceeded);
    }

    let mut bytes = [0u8; 32];

    for symbol in value.chars() {
        let mut carry = symbol.to_digit(radix).ok_or(Error::InvalidValue)?;

        for byte in &mut bytes {
            carry += u32::from(*byte) * radix;
            *byte = u8::try_from(carry & 255).map_err(|_| Error::Overflow)?;
            carry >>= u8::BITS;
        }

        if carry != 0 {
            return Err(Error::Overflow);
        }
    }

    Ok(bytes)
}

/// Negates in two's complement, including the minimum negative value.
fn negate(bytes: &mut [u8; 32]) {
    let mut carry = true;

    for byte in bytes {
        let (next, overflow) = (!*byte).overflowing_add(u8::from(carry));
        *byte = next;
        carry = overflow;
    }
}

impl Uint256 {
    /// Smallest unsigned 256-bit value.
    pub const MIN: Self = Self::from_le_bytes([0; 32]);
    /// Largest unsigned 256-bit value.
    pub const MAX: Self = Self::from_le_bytes([255; 32]);
    /// Parses unsigned ASCII digits in radix 2..=MAX_RADIX; '+' is accepted, '-' rejects.
    pub fn from_str_radix(value: &str, radix: u32) -> Result<Self, Error> {
        Ok(Self::from_le_bytes(magnitude(
            value.strip_prefix('+').unwrap_or(value),
            radix,
        )?))
    }
    /// Constructs the exact big-endian representation.
    #[must_use]
    pub fn from_be_bytes(mut bytes: [u8; 32]) -> Self {
        bytes.reverse();
        Self::from_le_bytes(bytes)
    }
    /// Returns the exact big-endian representation.
    #[must_use]
    pub fn to_be_bytes(self) -> [u8; 32] {
        let mut bytes = self.to_le_bytes();
        bytes.reverse();
        bytes
    }
}

impl Int256 {
    /// Smallest signed 256-bit value.
    pub const MIN: Self = {
        let mut bytes = [0; 32];
        bytes[SIGN_INDEX] = 128;
        Self::from_le_bytes(bytes)
    };
    /// Largest signed 256-bit value.
    pub const MAX: Self = {
        let mut bytes = [255; 32];
        bytes[SIGN_INDEX] = 127;
        Self::from_le_bytes(bytes)
    };
    /// Parses signed ASCII digits in radix 2..=MAX_RADIX, rejecting values outside [-2^255, 2^255-1].
    pub fn from_str_radix(value: &str, radix: u32) -> Result<Self, Error> {
        let negative = value.starts_with('-');
        let digits = value.strip_prefix(['-', '+']).unwrap_or(value);
        let mut bytes = magnitude(digits, radix)?;
        let minimum =
            bytes[SIGN_INDEX] == SIGN_BIT && bytes[..SIGN_INDEX].iter().all(|byte| *byte == 0);

        if bytes[SIGN_INDEX] >= SIGN_BIT && !(negative && minimum) {
            return Err(Error::Overflow);
        }

        if negative {
            negate(&mut bytes);
        }

        Ok(Self::from_le_bytes(bytes))
    }
    /// Constructs the exact big-endian two's-complement representation.
    #[must_use]
    pub fn from_be_bytes(mut bytes: [u8; 32]) -> Self {
        bytes.reverse();
        Self::from_le_bytes(bytes)
    }
    /// Returns the exact big-endian two's-complement representation.
    #[must_use]
    pub fn to_be_bytes(self) -> [u8; 32] {
        let mut bytes = self.to_le_bytes();
        bytes.reverse();
        bytes
    }
}

/// Writes decimal digits using bounded repeated long division on integer bytes.
fn decimal(mut bytes: [u8; 32], formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut digits = [b'0'; 78];
    let mut start = digits.len();

    loop {
        let mut remainder = 0u16;

        for byte in bytes.iter_mut().rev() {
            let value = remainder * BYTE_RADIX + u16::from(*byte);
            *byte = u8::try_from(value / 10).map_err(|_| fmt::Error)?;
            remainder = value % 10;
        }

        start -= 1;

        digits[start] = b'0' + u8::try_from(remainder).map_err(|_| fmt::Error)?;

        if bytes.iter().all(|byte| *byte == 0) {
            break;
        }
    }

    formatter.write_str(core::str::from_utf8(&digits[start..]).map_err(|_| fmt::Error)?)
}

impl fmt::Display for Uint256 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        decimal(self.to_le_bytes(), formatter)
    }
}
impl fmt::Display for Int256 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes = self.to_le_bytes();

        if bytes[SIGN_INDEX] >= SIGN_BIT {
            formatter.write_str("-")?;
            negate(&mut bytes);
        }

        decimal(bytes, formatter)
    }
}
impl FromStr for Uint256 {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        Self::from_str_radix(value, 10)
    }
}
impl FromStr for Int256 {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        Self::from_str_radix(value, 10)
    }
}
