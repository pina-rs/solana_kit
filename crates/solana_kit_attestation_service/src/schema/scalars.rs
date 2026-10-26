//! Source-defined Unicode and binary-string presentation rules.
use alloc::string::String;
use solana_kit_core::codecs::{Decoder, Encoder, Error, Number, Reader, SizePrefix, Utf8, Writer};

pub(super) struct ScalarChar;

impl Encoder<char> for ScalarChar {
    fn encoded_len(&self, _: &char) -> Result<usize, Error> {
        Ok(4)
    }

    fn write(&self, value: &char, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u32>::LE.write(&u32::from(*value), writer)
    }
}

impl Decoder<char> for ScalarChar {
    fn read(&self, reader: &mut Reader<'_>) -> Result<char, Error> {
        char::from_u32(Number::<u32>::LE.read(reader)?).ok_or(Error::InvalidValue)
    }
}

pub(super) struct FieldString;

impl Encoder<String> for FieldString {
    fn encoded_len(&self, value: &String) -> Result<usize, Error> {
        SizePrefix(Utf8).encoded_len(value.as_str())
    }

    fn write(&self, value: &String, writer: &mut Writer<'_>) -> Result<(), Error> {
        SizePrefix(Utf8).write(value.as_str(), writer)
    }
}

impl Decoder<String> for FieldString {
    fn read(&self, reader: &mut Reader<'_>) -> Result<String, Error> {
        let size = usize::try_from(Number::<u32>::LE.read(reader)?).map_err(|_| Error::Overflow)?;
        let bytes = reader.take(size)?;

        match core::str::from_utf8(bytes) {
            Ok(text) => {
                reader.claim(size)?;
                let mut output = String::new();
                output
                    .try_reserve_exact(size)
                    .map_err(|_| Error::AllocationFailed)?;
                output.push_str(text);
                Ok(output)
            }
            Err(_) => hex(bytes, reader),
        }
    }
}

fn hex(bytes: &[u8], reader: &mut Reader<'_>) -> Result<String, Error> {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let size = bytes
        .len()
        .checked_mul(2)
        .and_then(|n| n.checked_add(2))
        .ok_or(Error::Overflow)?;
    reader.claim(size)?;
    let mut output = String::new();
    output
        .try_reserve_exact(size)
        .map_err(|_| Error::AllocationFailed)?;
    output.push_str("0x");

    for byte in bytes {
        output.push(char::from(DIGITS[usize::from(byte >> 4)]));
        output.push(char::from(DIGITS[usize::from(byte & 15)]));
    }

    Ok(output)
}
