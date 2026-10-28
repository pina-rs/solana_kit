//! Kit cursor codecs for SNS account and value layouts.

use crate::{
    NAME_REGISTRY_HEADER_LENGTH, NameRegistryState, RECORD_HEADER_LENGTH, SnsRecordHeader,
};
use alloc::{string::String, vec::Vec};
use solana_kit_core::codecs::{Decoder, Encoder, Error, Reader, Size, Writer};

/// Codec for a complete registry header followed by all remaining account data.
#[derive(Clone, Copy, Debug, Default)]
pub struct NameRegistryCodec;

impl Encoder<NameRegistryState> for NameRegistryCodec {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Variable { max: None })
    }
    fn encoded_len(&self, value: &NameRegistryState) -> Result<usize, Error> {
        NAME_REGISTRY_HEADER_LENGTH
            .checked_add(value.data.len())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &NameRegistryState, writer: &mut Writer<'_>) -> Result<(), Error> {
        writer.put(value.parent_name.as_bytes())?;
        writer.put(value.owner.as_bytes())?;
        writer.put(value.registry_class.as_bytes())?;
        writer.put(&value.data)
    }
}
impl Decoder<NameRegistryState> for NameRegistryCodec {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Variable { max: None })
    }
    fn read(&self, reader: &mut Reader<'_>) -> Result<NameRegistryState, Error> {
        let header = reader.take(NAME_REGISTRY_HEADER_LENGTH)?;
        let mut value = NameRegistryState::decode(header).map_err(|_| Error::Truncated)?;
        let length = reader.remaining().len();
        reader.claim(length)?;
        let mut data = Vec::new();
        data.try_reserve_exact(length)
            .map_err(|_| Error::AllocationFailed)?;
        data.extend_from_slice(reader.take(length)?);
        value.data = data;

        Ok(value)
    }
}

/// Codec for the fixed eight-byte V2 record header.
#[derive(Clone, Copy, Debug, Default)]
pub struct SnsRecordHeaderCodec;
impl Encoder<SnsRecordHeader> for SnsRecordHeaderCodec {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(RECORD_HEADER_LENGTH))
    }
    fn encoded_len(&self, _: &SnsRecordHeader) -> Result<usize, Error> {
        Ok(RECORD_HEADER_LENGTH)
    }
    fn write(&self, value: &SnsRecordHeader, writer: &mut Writer<'_>) -> Result<(), Error> {
        writer.put(&value.encode())
    }
}
impl Decoder<SnsRecordHeader> for SnsRecordHeaderCodec {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(RECORD_HEADER_LENGTH))
    }
    fn read(&self, reader: &mut Reader<'_>) -> Result<SnsRecordHeader, Error> {
        let bytes = reader.take(RECORD_HEADER_LENGTH)?;
        SnsRecordHeader::decode_prefix(bytes)
            .map(|result| result.0)
            .map_err(|_| Error::InvalidValue)
    }
}

/// Codec for a u32-length-prefixed UTF-8 name with source-compatible BOM handling.
#[derive(Clone, Copy, Debug, Default)]
pub struct NameValueCodec;
impl Encoder<str> for NameValueCodec {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Variable { max: None })
    }
    fn encoded_len(&self, value: &str) -> Result<usize, Error> {
        u32::try_from(value.len()).map_err(|_| Error::Overflow)?;
        4_usize.checked_add(value.len()).ok_or(Error::Overflow)
    }
    fn write(&self, value: &str, writer: &mut Writer<'_>) -> Result<(), Error> {
        let length = u32::try_from(value.len()).map_err(|_| Error::Overflow)?;
        writer.put(&length.to_le_bytes())?;
        writer.put(value.as_bytes())
    }
}
impl Decoder<String> for NameValueCodec {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Variable { max: None })
    }
    fn read(&self, reader: &mut Reader<'_>) -> Result<String, Error> {
        let length: [u8; 4] = reader.take(4)?.try_into().map_err(|_| Error::Truncated)?;
        let length = u32::from_le_bytes(length) as usize;
        let bytes = reader.take(length)?;
        let value = core::str::from_utf8(bytes).map_err(|_| Error::InvalidUtf8)?;
        let value = value.strip_prefix('\u{feff}').unwrap_or(value);
        reader.claim(value.len())?;
        let mut result = String::new();
        result
            .try_reserve_exact(value.len())
            .map_err(|_| Error::AllocationFailed)?;

        result.push_str(value);
        Ok(result)
    }
}
