//! Text encodings consume the remainder of their enclosing frame.

use super::{Decoder, Encoder, Error, Reader, Utf8, Writer};
use alloc::string::String;

/// Allocates text only after the caller's shared byte-element claim succeeds.
pub(super) fn output(reader: &mut Reader<'_>, size: usize) -> Result<String, Error> {
    reader.claim(size)?;
    let mut text = String::new();
    text.try_reserve_exact(size)
        .map_err(|_| Error::AllocationFailed)?;
    Ok(text)
}

/// Hex text to bytes; a trailing lone digit is a low nibble. Decode emits lowercase pairs.
#[derive(Clone, Copy, Debug, Default)]
pub struct Base16;

impl Encoder<str> for Base16 {
    fn encoded_len(&self, value: &str) -> Result<usize, Error> {
        for byte in value.bytes() {
            hex_digit(byte)?;
        }

        value
            .len()
            .checked_add(1)
            .map(|n| n / 2)
            .ok_or(Error::Overflow)
    }

    fn write(&self, value: &str, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;

        for pair in value.as_bytes().chunks(2) {
            let high = hex_digit(pair[0])?;

            let byte = match pair.get(1) {
                Some(low) => (high << 4) | hex_digit(*low)?,
                None => high,
            };

            writer.put(&[byte])?;
        }

        Ok(())
    }
}

/// Validates one ASCII hexadecimal digit without lossy character conversion.
fn hex_digit(byte: u8) -> Result<u8, Error> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(Error::InvalidValue),
    }
}

impl Decoder<String> for Base16 {
    fn read(&self, reader: &mut Reader<'_>) -> Result<String, Error> {
        let bytes = reader.remaining();
        let mut text = output(reader, bytes.len().checked_mul(2).ok_or(Error::Overflow)?)?;
        const HEX: &[u8] = b"0123456789abcdef";

        for byte in reader.take(bytes.len())? {
            text.push(char::from(HEX[usize::from(byte >> 4)]));
            text.push(char::from(HEX[usize::from(byte & 15)]));
        }

        Ok(text)
    }
}

/// Configured UTF-8. Default matches pinned Dart: strict, strip leading BOM, preserve nulls.
/// Rust `str` cannot contain unpaired surrogates, so encoding is always well formed.
/// The existing unit `Utf8` retains its strict, BOM-preserving contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Utf8Config {
    /// Reject malformed bytes instead of replacing each malformed sequence with U+FFFD.
    pub fatal: bool,
    /// Preserve a leading BOM when true (the reference's `ignoreBOM` option).
    pub ignore_bom: bool,
    /// Strip decoded U+0000 characters, including embedded nulls.
    pub remove_null_characters: bool,
}

impl Default for Utf8Config {
    fn default() -> Self {
        Self {
            fatal: true,
            ignore_bom: false,
            remove_null_characters: false,
        }
    }
}

impl Encoder<str> for Utf8Config {
    fn encoded_len(&self, value: &str) -> Result<usize, Error> {
        Utf8.encoded_len(value)
    }
    fn write(&self, value: &str, writer: &mut Writer<'_>) -> Result<(), Error> {
        Utf8.write(value, writer)
    }
}

impl Utf8Config {
    /// Visits validated/replacement chunks without allocating an intermediate string.
    fn visit(
        &self,
        mut bytes: &[u8],
        mut emit: impl FnMut(&str) -> Result<(), Error>,
    ) -> Result<(), Error> {
        // Dart strips the first BOM before restoring it, and skips restoration
        // when the decoded remainder already begins with another BOM.
        let bom = &[0xef, 0xbb, 0xbf];

        if bytes.starts_with(bom) && (!self.ignore_bom || bytes[3..].starts_with(bom)) {
            bytes = &bytes[3..];
        }

        while !bytes.is_empty() {
            match core::str::from_utf8(bytes) {
                Ok(text) => {
                    self.emit(text, &mut emit)?;
                    break;
                }

                Err(error) => {
                    bytes = self.malformed(bytes, error, &mut emit)?;
                }
            }
        }

        Ok(())
    }

    /// Recovers exactly one invalid UTF-8 sequence, retaining valid prefix bytes.
    fn malformed<'a>(
        &self,
        bytes: &'a [u8],
        error: core::str::Utf8Error,
        emit: &mut impl FnMut(&str) -> Result<(), Error>,
    ) -> Result<&'a [u8], Error> {
        if self.fatal {
            return Err(Error::InvalidUtf8);
        }

        let valid = error.valid_up_to();
        let text = core::str::from_utf8(&bytes[..valid]).map_err(|_| Error::InvalidUtf8)?;
        self.emit(text, emit)?;
        emit("\u{fffd}")?;
        let skip = error.error_len().unwrap_or(bytes.len() - valid);
        Ok(&bytes[valid + skip..])
    }

    /// Null filtering is applied to decoded characters, never to raw malformed bytes.
    fn emit(
        &self,
        text: &str,
        emit: &mut impl FnMut(&str) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if self.remove_null_characters {
            for part in text.split('\0') {
                emit(part)?;
            }

            return Ok(());
        }

        emit(text)
    }
}

impl Decoder<String> for Utf8Config {
    fn read(&self, reader: &mut Reader<'_>) -> Result<String, Error> {
        let bytes = reader.remaining();
        let mut size = 0usize;
        self.visit(bytes, |part| {
            size = size.checked_add(part.len()).ok_or(Error::Overflow)?;
            Ok(())
        })?;
        let mut text = output(reader, size)?;
        self.visit(reader.take(bytes.len())?, |part| {
            text.push_str(part);
            Ok(())
        })?;
        Ok(text)
    }
}

/// Removes only U+0000, preserving all other characters, including every BOM.
/// Allocation is bounded by an explicit output-byte limit.
pub fn remove_null_characters(value: &str, max_bytes: usize) -> Result<String, Error> {
    let size = value.split('\0').try_fold(0usize, |size, part| {
        size.checked_add(part.len()).ok_or(Error::Overflow)
    })?;

    if size > max_bytes {
        return Err(Error::LimitExceeded);
    }

    let mut text = String::new();
    text.try_reserve_exact(size)
        .map_err(|_| Error::AllocationFailed)?;

    for part in value.split('\0') {
        text.push_str(part);
    }

    Ok(text)
}

/// Pads to a UTF-16 code-unit count, matching Dart's length convention, without truncation.
pub fn pad_null_characters(value: &str, units: usize, max_bytes: usize) -> Result<String, Error> {
    let padding = units.saturating_sub(value.encode_utf16().count());
    let size = value.len().checked_add(padding).ok_or(Error::Overflow)?;
    let mut reader = Reader::new(&[], super::Limits::new(0, max_bytes))?;
    let mut text = output(&mut reader, size)?;
    text.push_str(value);

    for _ in 0..padding {
        text.push('\0');
    }

    Ok(text)
}

#[cfg(feature = "text-codecs")]
mod base64_codec;
#[cfg(feature = "text-codecs")]
pub use base64_codec::Base64;
