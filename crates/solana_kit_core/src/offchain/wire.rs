//! Source-backed V0 and V1 message wire layouts.

use alloc::{string::String, vec::Vec};

use crate::{
    Address,
    codecs::{Error as CodecError, Reader},
};

use super::{ContentFormat, Error, Format, Message};

const ADDRESS_BYTES: usize = 32;
const SHORT_CONTENT_MAX: usize = 1232;
const LONG_CONTENT_MAX: usize = 65535;
const V0_PREAMBLE_BYTES: usize = 53;
const V1_PREAMBLE_BYTES: usize = 18;
const SIGNING_DOMAIN: &[u8; 16] = b"\xffsolana offchain";

/// V0 limits the UTF-8 byte count and optionally restricts printable bytes.
pub(super) fn content_valid(format: Format, content: &str) -> Result<(), Error> {
    if content.is_empty() {
        return Err(Error::InvalidContent);
    }

    let Format::V0 { content_format, .. } = format else {
        return Ok(());
    };

    let maximum = match content_format {
        ContentFormat::Utf8Long => LONG_CONTENT_MAX,
        _ => SHORT_CONTENT_MAX,
    };

    if content.len() > maximum {
        return Err(Error::InvalidContent);
    }

    if content_format == ContentFormat::RestrictedAscii
        && !content.bytes().all(|byte| (0x20..=0x7e).contains(&byte))
    {
        return Err(Error::InvalidContent);
    }

    Ok(())
}

pub(super) fn signatories_valid(signatories: &[Address]) -> Result<u8, Error> {
    let count = u8::try_from(signatories.len()).map_err(|_| Error::InvalidSignatories)?;

    if count == 0 {
        return Err(Error::InvalidSignatories);
    }

    Ok(count)
}

fn ordered(signatories: &[Address]) -> Result<(), Error> {
    if signatories.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(Error::InvalidSignatories);
    }

    Ok(())
}

pub(super) fn copy_bytes(bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let mut result = Vec::new();
    result
        .try_reserve_exact(bytes.len())
        .map_err(|_| CodecError::AllocationFailed)?;
    result.extend_from_slice(bytes);
    Ok(result)
}

/// Construction validates before allocating and only V1 canonicalizes address order.
pub(super) fn build(
    format: Format,
    content: &str,
    signatories: &[Address],
    max_bytes: usize,
) -> Result<Message, Error> {
    let count = signatories_valid(signatories)?;

    let preamble = match format {
        Format::V0 { .. } => V0_PREAMBLE_BYTES,
        Format::V1 => V1_PREAMBLE_BYTES,
    };

    let size = content
        .len()
        .checked_add(preamble + signatories.len() * ADDRESS_BYTES)
        .ok_or(CodecError::Overflow)?;
    content_valid(format, content)?;

    if size > max_bytes {
        return Err(CodecError::LimitExceeded.into());
    }

    let mut addresses = Vec::new();
    addresses
        .try_reserve_exact(signatories.len())
        .map_err(|_| CodecError::AllocationFailed)?;
    addresses.extend_from_slice(signatories);

    if format == Format::V1 {
        addresses.sort_unstable();
        ordered(&addresses)?;
    }

    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(size)
        .map_err(|_| CodecError::AllocationFailed)?;
    write_header(&mut bytes, format, count);

    for address in &addresses {
        bytes.extend_from_slice(address.as_bytes());
    }

    if matches!(format, Format::V0 { .. }) {
        let length = u16::try_from(content.len()).map_err(|_| CodecError::Overflow)?;
        bytes.extend_from_slice(&length.to_le_bytes());
    }

    bytes.extend_from_slice(content.as_bytes());
    let text = copy_text(content)?;
    Ok(Message {
        bytes,
        format,
        signatories: addresses,
        content: text,
    })
}

fn write_header(bytes: &mut Vec<u8>, format: Format, count: u8) {
    bytes.extend_from_slice(SIGNING_DOMAIN);

    match format {
        Format::V0 {
            application_domain,
            content_format,
        } => {
            bytes.push(0);
            bytes.extend_from_slice(application_domain.as_bytes());
            bytes.push(content_format as u8);
        }
        Format::V1 => bytes.push(1),
    }

    bytes.push(count);
}

fn copy_text(content: &str) -> Result<String, Error> {
    let mut text = String::new();
    text.try_reserve_exact(content.len())
        .map_err(|_| CodecError::AllocationFailed)?;
    text.push_str(content);
    Ok(text)
}

fn byte(reader: &mut Reader<'_>) -> Result<u8, Error> {
    Ok(reader.take(1)?[0])
}

fn format(reader: &mut Reader<'_>) -> Result<Format, Error> {
    if reader.take(SIGNING_DOMAIN.len())? != SIGNING_DOMAIN {
        return Err(Error::InvalidDomainOrVersion);
    }

    match byte(reader)? {
        0 => {
            let domain = reader
                .take(ADDRESS_BYTES)?
                .try_into()
                .map_err(|_| CodecError::Truncated)?;
            let content_format = match byte(reader)? {
                0 => ContentFormat::RestrictedAscii,
                1 => ContentFormat::Utf8,
                2 => ContentFormat::Utf8Long,
                _ => return Err(CodecError::InvalidDiscriminator.into()),
            };

            Ok(Format::V0 {
                application_domain: Address::from_bytes(domain),
                content_format,
            })
        }
        1 => Ok(Format::V1),
        _ => Err(Error::InvalidDomainOrVersion),
    }
}

pub(super) fn read(reader: &mut Reader<'_>) -> Result<Message, Error> {
    let original = reader.remaining();
    let format = format(reader)?;
    let count = usize::from(byte(reader)?);
    let address_bytes = reader.take(count * ADDRESS_BYTES)?;
    reader.claim(count)?;

    if count == 0 {
        return Err(Error::InvalidSignatories);
    }

    let mut signatories = Vec::new();
    signatories
        .try_reserve_exact(count)
        .map_err(|_| CodecError::AllocationFailed)?;

    for bytes in address_bytes.as_chunks::<ADDRESS_BYTES>().0 {
        signatories.push(Address::from_bytes(*bytes));
    }

    if format == Format::V1 {
        ordered(&signatories)?;
    }

    let content_bytes = match format {
        Format::V0 { .. } => {
            let length = reader
                .take(2)?
                .try_into()
                .map_err(|_| CodecError::Truncated)?;
            reader.take(usize::from(u16::from_le_bytes(length)))?
        }
        Format::V1 => reader.take(reader.remaining().len())?,
    };

    let content = core::str::from_utf8(content_bytes).map_err(|_| CodecError::InvalidUtf8)?;
    content_valid(format, content)?;
    let consumed = original.len() - reader.remaining().len();
    reader.claim(consumed)?;
    reader.claim(content.len())?;
    let bytes = copy_bytes(&original[..consumed])?;
    let content = copy_text(content)?;

    Ok(Message {
        bytes,
        format,
        signatories,
        content,
    })
}
