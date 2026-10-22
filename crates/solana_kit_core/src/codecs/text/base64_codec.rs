//! Pinned Dart's tolerant Base64 input with canonical padded output.
use super::{Decoder, Encoder, Error, Reader, Writer, output};
use alloc::string::String;
use base64::{Engine, engine::general_purpose::STANDARD};

const SYMBOL_BITS: usize = 6;
const BYTE_BITS: usize = 8;
const LOW_NIBBLE: u8 = 15;
const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Base64 text encodes to bytes. All '=' symbols are ignored; missing padding is accepted.
/// A lone trailing symbol is discarded. Invalid alphabet and nonzero unused bits reject.
#[derive(Clone, Copy, Debug, Default)]
pub struct Base64;

/// Validates without allocating, returning significant symbols and exact byte length.
fn dimensions(value: &str) -> Result<(usize, usize), Error> {
    let mut count = 0usize;
    let mut last = 0u8;

    for byte in value.bytes().filter(|byte| *byte != b'=') {
        last = digit(byte)?;
        count = count.checked_add(1).ok_or(Error::Overflow)?;
    }

    let effective = count - usize::from(count % 4 == 1);

    if count % 4 != 1
        && ((count % 4 == 2 && last & LOW_NIBBLE != 0) || (count % 4 == 3 && last & 3 != 0))
    {
        return Err(Error::InvalidValue);
    }

    Ok((
        effective,
        effective.checked_mul(SYMBOL_BITS).ok_or(Error::Overflow)? / BYTE_BITS,
    ))
}

/// Maps the standard alphabet only; URL-safe and whitespace input reject.
fn digit(byte: u8) -> Result<u8, Error> {
    let index = ALPHABET
        .iter()
        .position(|symbol| *symbol == byte)
        .ok_or(Error::InvalidValue)?;
    u8::try_from(index).map_err(|_| Error::Overflow)
}

impl Encoder<str> for Base64 {
    fn encoded_len(&self, value: &str) -> Result<usize, Error> {
        Ok(dimensions(value)?.1)
    }
    fn write(&self, value: &str, writer: &mut Writer<'_>) -> Result<(), Error> {
        let (effective, _) = dimensions(value)?;
        let mut chars = value.bytes().filter(|byte| *byte != b'=').take(effective);

        loop {
            let mut block = [b'='; 4];
            let mut count = 0;

            for (slot, byte) in block.iter_mut().zip(chars.by_ref().take(4)) {
                *slot = byte;
                count += 1;
            }

            if count == 0 {
                break;
            }

            let mut decoded = [0; 3];
            let written = STANDARD
                .decode_slice(block, &mut decoded)
                .map_err(|_| Error::InvalidValue)?;
            writer.put(&decoded[..written])?;
        }

        Ok(())
    }
}

impl Decoder<String> for Base64 {
    fn read(&self, reader: &mut Reader<'_>) -> Result<String, Error> {
        let bytes = reader.remaining();
        let size = bytes
            .len()
            .checked_add(2)
            .and_then(|n| (n / 3).checked_mul(4))
            .ok_or(Error::Overflow)?;
        let mut text = output(reader, size)?;

        for chunk in reader.take(bytes.len())?.chunks(3) {
            let mut block = [0; 4];
            let len = STANDARD
                .encode_slice(chunk, &mut block)
                .map_err(|_| Error::InvalidValue)?;
            text.push_str(core::str::from_utf8(&block[..len]).map_err(|_| Error::InvalidUtf8)?);
        }

        Ok(text)
    }
}
