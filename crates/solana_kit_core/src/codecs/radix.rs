//! Integer-radix and bit-reslicing text codecs with explicit work limits.
use super::{Decoder, Encoder, Error, Reader, Writer, text::output};
use alloc::{string::String, vec::Vec};

const BYTE_BITS: u32 = u8::BITS;
const BYTE_BITS_U8: u8 = 8;
const BYTE_RADIX: u32 = 256;
const MAX_ALPHABET: usize = 65536;
const MAX_SYMBOL_BITS: u8 = 16;
const MAX_DIGITS: usize = 4096;
const SCRATCH: usize = MAX_DIGITS * 2;
const BASE58: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Alphabet text to minimal big-endian bytes, retaining one zero byte per leading zero symbol.
/// Alphabets contain 2..=MAX_ALPHABET BMP scalar symbols; duplicate symbols use their first index.
/// Non-BMP alphabets reject because Dart indexes UTF-16 code units while Rust strings require scalars.
/// Encoding accepts at most 4096 symbols; decoding is bounded by the reader's shared budget.
#[derive(Clone, Copy, Debug)]
pub struct BaseX<'a> {
    alphabet: &'a str,
    base: u32,
}

impl<'a> BaseX<'a> {
    /// Validates an alphabet before any conversion. No global alphabet cache is retained.
    pub fn new(alphabet: &'a str) -> Result<Self, Error> {
        let count = alphabet.chars().count();

        if !(2..=MAX_ALPHABET).contains(&count) || alphabet.chars().any(|c| u32::from(c) > 0xffff) {
            return Err(Error::InvalidValue);
        }

        Ok(Self {
            alphabet,
            base: u32::try_from(count).map_err(|_| Error::Overflow)?,
        })
    }

    /// Decimal text uses integer radix conversion, not ASCII bytes or floating point.
    #[must_use]
    pub const fn base10() -> Self {
        Self {
            alphabet: "0123456789",
            base: 10,
        }
    }

    /// Bitcoin/Solana Base58 alphabet. Writes reuse the audited bs58 implementation.
    #[must_use]
    pub const fn base58() -> Self {
        Self {
            alphabet: BASE58,
            base: 58,
        }
    }

    /// Rejects a symbol outside this alphabet, without allocating.
    pub fn validate(&self, value: &str) -> Result<(), Error> {
        for symbol in value.chars() {
            self.index(symbol)?;
        }

        Ok(())
    }

    /// First occurrence matches the reference's alphabet lookup.
    fn index(&self, symbol: char) -> Result<u32, Error> {
        let index = self
            .alphabet
            .chars()
            .position(|candidate| candidate == symbol)
            .ok_or(Error::InvalidValue)?;
        u32::try_from(index).map_err(|_| Error::Overflow)
    }

    /// Converts into fixed bounded scratch storage, so encoded_len never allocates a temporary.
    fn convert(&self, value: &str, buffer: &mut [u8; SCRATCH]) -> Result<(usize, usize), Error> {
        if value.chars().count() > MAX_DIGITS {
            return Err(Error::LimitExceeded);
        }

        self.validate(value)?;
        let zero = self.alphabet.chars().next().ok_or(Error::InvalidValue)?;
        let leading = value.chars().take_while(|c| *c == zero).count();
        let mut length = 0;

        for symbol in value.chars().skip(leading) {
            let mut carry = self.index(symbol)?;

            for byte in &mut buffer[..length] {
                carry += self.base * u32::from(*byte);
                *byte = u8::try_from(carry & 255).map_err(|_| Error::Overflow)?;
                carry >>= BYTE_BITS;
            }

            while carry != 0 {
                let byte = buffer.get_mut(length).ok_or(Error::LimitExceeded)?;
                *byte = u8::try_from(carry & 255).map_err(|_| Error::Overflow)?;
                length += 1;
                carry >>= BYTE_BITS;
            }
        }

        Ok((leading, length))
    }

    /// Converts a bounded input slice into least-significant-first digit indexes.
    fn digits(&self, bytes: &[u8], reader: &mut Reader<'_>) -> Result<Vec<u32>, Error> {
        let bits = (u32::BITS - 1) - self.base.leading_zeros();
        let capacity = bytes
            .len()
            .checked_mul(usize::from(BYTE_BITS_U8))
            .and_then(|n| n.checked_add(usize::try_from(bits).ok()? - 1))
            .ok_or(Error::Overflow)?
            / usize::try_from(bits).map_err(|_| Error::Overflow)?;
        reader.claim(capacity)?;
        let mut digits = Vec::<u32>::new();
        digits
            .try_reserve_exact(capacity)
            .map_err(|_| Error::AllocationFailed)?;

        for byte in bytes {
            let mut carry = u32::from(*byte);

            for digit in &mut digits {
                carry += BYTE_RADIX * *digit;
                *digit = carry % self.base;
                carry /= self.base;
            }

            while carry != 0 {
                digits.push(carry % self.base);
                carry /= self.base;
            }
        }

        Ok(digits)
    }

    /// Reads an index proven by arithmetic to be below the alphabet base.
    fn symbol(&self, index: u32) -> Result<char, Error> {
        self.alphabet
            .chars()
            .nth(usize::try_from(index).map_err(|_| Error::Overflow)?)
            .ok_or(Error::InvalidValue)
    }
}

impl Encoder<str> for BaseX<'_> {
    fn encoded_len(&self, value: &str) -> Result<usize, Error> {
        let (leading, length) = self.convert(value, &mut [0; SCRATCH])?;
        leading.checked_add(length).ok_or(Error::Overflow)
    }

    fn write(&self, value: &str, writer: &mut Writer<'_>) -> Result<(), Error> {
        let mut buffer = [0; SCRATCH];
        let (leading, length) = self.convert(value, &mut buffer)?;

        if self.alphabet == BASE58 {
            let length = bs58::decode(value)
                .onto(&mut buffer[..])
                .map_err(|_| Error::InvalidValue)?;

            return writer.put(&buffer[..length]);
        }

        writer.fill(leading, 0)?;
        buffer[..length].reverse();
        writer.put(&buffer[..length])
    }
}

impl Decoder<String> for BaseX<'_> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<String, Error> {
        let bytes = reader.remaining();
        let leading = bytes.iter().take_while(|byte| **byte == 0).count();
        let digits = self.digits(&bytes[leading..], reader)?;
        let zero = self.symbol(0)?;
        let mut size = leading
            .checked_mul(zero.len_utf8())
            .ok_or(Error::Overflow)?;

        for digit in &digits {
            size = size
                .checked_add(self.symbol(*digit)?.len_utf8())
                .ok_or(Error::Overflow)?;
        }

        let mut text = output(reader, size)?;

        for _ in 0..leading {
            text.push(zero);
        }

        for digit in digits.into_iter().rev() {
            text.push(self.symbol(digit)?);
        }

        reader.take(bytes.len())?;
        Ok(text)
    }
}

/// Re-slices alphabet digits into bits; incomplete trailing input bits are discarded.
/// Decode pads the final output symbol with zero bits. Alphabet length must equal 2^bits.
#[derive(Clone, Copy, Debug)]
pub struct BaseXReslice<'a> {
    alphabet: BaseX<'a>,
    bits: u8,
}

impl<'a> BaseXReslice<'a> {
    /// Validated 1..=MAX_SYMBOL_BITS bit symbols prevent invalid shifts and alphabet indexing.
    pub fn new(alphabet: &'a str, bits: u8) -> Result<Self, Error> {
        if !(1..=MAX_SYMBOL_BITS).contains(&bits) {
            return Err(Error::InvalidValue);
        }

        let alphabet = BaseX::new(alphabet)?;

        if alphabet.base != 1u32 << bits {
            return Err(Error::InvalidValue);
        }

        Ok(Self { alphabet, bits })
    }
}

impl Encoder<str> for BaseXReslice<'_> {
    fn encoded_len(&self, value: &str) -> Result<usize, Error> {
        self.alphabet.validate(value)?;
        value
            .chars()
            .count()
            .checked_mul(usize::from(self.bits))
            .map(|n| n / usize::from(BYTE_BITS_U8))
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &str, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;
        let mut accumulator = 0u32;
        let mut bits = 0;

        for symbol in value.chars() {
            accumulator = (accumulator << self.bits) | self.alphabet.index(symbol)?;
            bits += self.bits;

            while u32::from(bits) >= BYTE_BITS {
                bits -= BYTE_BITS_U8;

                writer.put(&[
                    u8::try_from((accumulator >> bits) & 255).map_err(|_| Error::Overflow)?
                ])?;
            }

            accumulator &= (1 << bits) - 1;
        }

        Ok(())
    }
}

impl Decoder<String> for BaseXReslice<'_> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<String, Error> {
        let bytes = reader.remaining();
        let mut size = 0usize;
        self.visit(bytes, |symbol| {
            size = size.checked_add(symbol.len_utf8()).ok_or(Error::Overflow)?;
            Ok(())
        })?;
        let mut text = output(reader, size)?;
        self.visit(reader.take(bytes.len())?, |symbol| {
            text.push(symbol);
            Ok(())
        })?;
        Ok(text)
    }
}

impl BaseXReslice<'_> {
    /// Fixed accumulator storage is independent of input length.
    fn visit(
        &self,
        bytes: &[u8],
        mut emit: impl FnMut(char) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let mut accumulator = 0u32;
        let mut bits = 0;
        let mask = (1 << self.bits) - 1;

        for byte in bytes {
            accumulator = (accumulator << BYTE_BITS) | u32::from(*byte);
            bits += BYTE_BITS_U8;

            while bits >= self.bits {
                bits -= self.bits;
                emit(self.alphabet.symbol((accumulator >> bits) & mask)?)?;
            }

            accumulator &= (1 << bits) - 1;
        }

        if bits > 0 {
            emit(
                self.alphabet
                    .symbol((accumulator << (self.bits - bits)) & mask)?,
            )?;
        }

        Ok(())
    }
}
