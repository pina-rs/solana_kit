//! Exact UTF-16 code-unit alphabets, including unpaired surrogate symbols.

use alloc::vec::Vec;

use super::radix::{
    MAX_DIGITS, SCRATCH, expand_digits, fold_digits, visit_resliced, write_resliced,
};
use super::{Decoder, Encoder, Error, Reader, Writer};

/// Base-X conversion whose symbols are UTF-16 code units rather than scalars.
///
/// Dart indexes its alphabets by code unit. A supplementary character contributes
/// two alphabet positions, and either surrogate may appear alone in decoded text.
/// This codec therefore uses `[u16]` and `Vec<u16>` instead of imposing Rust's
/// scalar-string invariant. Duplicate symbols select their first index; decoding
/// a later duplicate position need not reproduce the original bytes on encoding.
/// At most 4096 input units are encoded. Decoding shares the reader's item budget.
#[derive(Clone, Copy, Debug)]
pub struct Utf16BaseX<'a> {
    alphabet: &'a [u16],
    base: u32,
}

impl<'a> Utf16BaseX<'a> {
    /// Validates an alphabet of 2..=65536 code units without allocating a cache.
    pub fn new(alphabet: &'a [u16]) -> Result<Self, Error> {
        if !(2..=65536).contains(&alphabet.len()) {
            return Err(Error::InvalidValue);
        }

        Ok(Self {
            alphabet,
            base: u32::try_from(alphabet.len()).map_err(|_| Error::Overflow)?,
        })
    }

    /// Checks every code unit against the declared alphabet without allocation.
    pub fn validate(&self, value: &[u16]) -> Result<(), Error> {
        for unit in value {
            self.index(*unit)?;
        }

        Ok(())
    }

    fn index(&self, unit: u16) -> Result<u32, Error> {
        let index = self
            .alphabet
            .iter()
            .position(|item| *item == unit)
            .ok_or(Error::InvalidValue)?;
        u32::try_from(index).map_err(|_| Error::Overflow)
    }

    fn convert(&self, value: &[u16], scratch: &mut [u8; SCRATCH]) -> Result<(usize, usize), Error> {
        if value.len() > MAX_DIGITS {
            return Err(Error::LimitExceeded);
        }

        self.validate(value)?;
        let leading = value
            .iter()
            .take_while(|unit| **unit == self.alphabet[0])
            .count();
        let digits = value[leading..].iter().map(|unit| self.index(*unit));
        let length = fold_digits(digits, self.base, scratch)?;

        Ok((leading, length))
    }
}

impl Encoder<[u16]> for Utf16BaseX<'_> {
    fn encoded_len(&self, value: &[u16]) -> Result<usize, Error> {
        let (leading, length) = self.convert(value, &mut [0; SCRATCH])?;
        leading.checked_add(length).ok_or(Error::Overflow)
    }

    fn write(&self, value: &[u16], writer: &mut Writer<'_>) -> Result<(), Error> {
        let mut scratch = [0; SCRATCH];
        let (leading, length) = self.convert(value, &mut scratch)?;
        scratch[..length].reverse();
        writer.fill(leading, 0)?;
        writer.put(&scratch[..length])
    }
}

impl Decoder<Vec<u16>> for Utf16BaseX<'_> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<u16>, Error> {
        let bytes = reader.remaining();
        let leading = bytes.iter().take_while(|byte| **byte == 0).count();
        let digits = expand_digits(&bytes[leading..], self.base, reader)?;
        let length = leading.checked_add(digits.len()).ok_or(Error::Overflow)?;
        reader.claim(length)?;
        let mut units = Vec::new();
        units
            .try_reserve_exact(length)
            .map_err(|_| Error::AllocationFailed)?;
        units.resize(leading, self.alphabet[0]);

        for digit in digits.into_iter().rev() {
            let index = usize::try_from(digit).map_err(|_| Error::Overflow)?;
            units.push(*self.alphabet.get(index).ok_or(Error::InvalidValue)?);
        }

        reader.take(bytes.len())?;
        Ok(units)
    }
}

/// Bit-reslicing with exact UTF-16 symbols and no host string conversion.
/// Encoding discards incomplete trailing bytes; decoding zero-pads the final
/// symbol, matching `BaseXReslice`. Work is linear and output claims are shared.
#[derive(Clone, Copy, Debug)]
pub struct Utf16BaseXReslice<'a> {
    alphabet: Utf16BaseX<'a>,
    bits: u8,
}

impl<'a> Utf16BaseXReslice<'a> {
    /// Requires 1..=16 bits and an alphabet with exactly 2^bits positions.
    pub fn new(alphabet: &'a [u16], bits: u8) -> Result<Self, Error> {
        if !(1..=16).contains(&bits) {
            return Err(Error::InvalidValue);
        }

        let alphabet = Utf16BaseX::new(alphabet)?;

        if alphabet.base != 1u32 << bits {
            return Err(Error::InvalidValue);
        }

        Ok(Self { alphabet, bits })
    }
}

impl Encoder<[u16]> for Utf16BaseXReslice<'_> {
    fn encoded_len(&self, value: &[u16]) -> Result<usize, Error> {
        self.alphabet.validate(value)?;
        value
            .len()
            .checked_mul(usize::from(self.bits))
            .map(|bits| bits / 8)
            .ok_or(Error::Overflow)
    }

    fn write(&self, value: &[u16], writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;
        let digits = value.iter().map(|unit| self.alphabet.index(*unit));
        write_resliced(digits, self.bits, writer)
    }
}

impl Decoder<Vec<u16>> for Utf16BaseXReslice<'_> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<u16>, Error> {
        let bytes = reader.remaining();
        let bits = usize::from(self.bits);
        let length = bytes
            .len()
            .checked_mul(8)
            .and_then(|size| size.checked_add(bits - 1))
            .ok_or(Error::Overflow)?
            / bits;
        reader.claim(length)?;
        let mut units = Vec::new();
        units
            .try_reserve_exact(length)
            .map_err(|_| Error::AllocationFailed)?;
        visit_resliced(bytes, self.bits, |index| {
            let index = usize::try_from(index).map_err(|_| Error::Overflow)?;
            units.push(
                *self
                    .alphabet
                    .alphabet
                    .get(index)
                    .ok_or(Error::InvalidValue)?,
            );
            Ok(())
        })?;
        reader.take(bytes.len())?;
        Ok(units)
    }
}
