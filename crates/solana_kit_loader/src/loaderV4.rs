// Generated from the exact pinned Codama input; see upstream notices.
use alloc::vec::Vec;
use solana_kit_core::codecs::Constant;
use solana_kit_core::codecs::Number;
use solana_kit_core::codecs::{Decoder, Encoder, Error, Reader, Writer};
/// Every pinned instruction data variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoaderV4InstructionData {
    /// Data for `Write`.
    Write(Write),
    /// Data for `Truncate`.
    Truncate(Truncate),
    /// Data for `Deploy`.
    Deploy(Deploy),
    /// Data for `Retract`.
    Retract(Retract),
    /// Data for `TransferAuthority`.
    TransferAuthority(TransferAuthority),
    /// Data for `Finalize`.
    Finalize(Finalize),
}
/// Prefix-safe multi-byte instruction dispatch.
#[derive(Clone, Copy, Debug)]
pub struct LoaderV4InstructionDataCodec;
impl Encoder<LoaderV4InstructionData> for LoaderV4InstructionDataCodec {
    fn encoded_len(&self, value: &LoaderV4InstructionData) -> Result<usize, Error> {
        match value {
            LoaderV4InstructionData::Write(value) => WriteCodec.encoded_len(value),
            LoaderV4InstructionData::Truncate(value) => TruncateCodec.encoded_len(value),
            LoaderV4InstructionData::Deploy(value) => DeployCodec.encoded_len(value),
            LoaderV4InstructionData::Retract(value) => RetractCodec.encoded_len(value),
            LoaderV4InstructionData::TransferAuthority(value) => {
                TransferAuthorityCodec.encoded_len(value)
            }
            LoaderV4InstructionData::Finalize(value) => FinalizeCodec.encoded_len(value),
        }
    }
    fn write(&self, value: &LoaderV4InstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            LoaderV4InstructionData::Write(value) => WriteCodec.write(value, writer),
            LoaderV4InstructionData::Truncate(value) => TruncateCodec.write(value, writer),
            LoaderV4InstructionData::Deploy(value) => DeployCodec.write(value, writer),
            LoaderV4InstructionData::Retract(value) => RetractCodec.write(value, writer),
            LoaderV4InstructionData::TransferAuthority(value) => {
                TransferAuthorityCodec.write(value, writer)
            }
            LoaderV4InstructionData::Finalize(value) => FinalizeCodec.write(value, writer),
        }
    }
}
impl Decoder<LoaderV4InstructionData> for LoaderV4InstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LoaderV4InstructionData, Error> {
        reader.transaction(|reader| match reader.remaining() {
            [0, ..] => Ok(LoaderV4InstructionData::Write(WriteCodec.read(reader)?)),
            [1, ..] => Ok(LoaderV4InstructionData::Truncate(
                TruncateCodec.read(reader)?,
            )),
            [2, ..] => Ok(LoaderV4InstructionData::Deploy(DeployCodec.read(reader)?)),
            [3, ..] => Ok(LoaderV4InstructionData::Retract(RetractCodec.read(reader)?)),
            [4, ..] => Ok(LoaderV4InstructionData::TransferAuthority(
                TransferAuthorityCodec.read(reader)?,
            )),
            [5, ..] => Ok(LoaderV4InstructionData::Finalize(
                FinalizeCodec.read(reader)?,
            )),
            [] => Err(Error::Truncated),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `Write`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Write {
    /// The `offset` wire field.
    pub offset: u32,
    /// The `bytes` wire field.
    pub bytes: Vec<u8>,
}
/// Bounded codec for [`Write`].
#[derive(Clone, Copy, Debug)]
pub struct WriteCodec;
impl Encoder<Write> for WriteCodec {
    fn encoded_len(&self, value: &Write) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Bytes,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.bytes.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Write, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[0]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.offset, writer)?;
        ByteSizePrefix {
            item: Bytes,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.bytes.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<Write> for WriteCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Write, Error> {
        Constant(&[0]).read(reader)?;
        Ok(Write {
            offset: Number::<u32>::LE.read(reader)?,
            bytes: ByteSizePrefix {
                item: Bytes,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `Truncate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Truncate {
    /// The `new_size` wire field.
    pub new_size: u32,
}
/// Bounded codec for [`Truncate`].
#[derive(Clone, Copy, Debug)]
pub struct TruncateCodec;
impl Encoder<Truncate> for TruncateCodec {
    fn encoded_len(&self, value: &Truncate) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.new_size)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Truncate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[1]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.new_size, writer)?;
        Ok(())
    }
}
impl Decoder<Truncate> for TruncateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Truncate, Error> {
        Constant(&[1]).read(reader)?;
        Ok(Truncate {
            new_size: Number::<u32>::LE.read(reader)?,
        })
    }
}
/// Validated data for `Deploy`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deploy {}
/// Bounded codec for [`Deploy`].
#[derive(Clone, Copy, Debug)]
pub struct DeployCodec;
impl Encoder<Deploy> for DeployCodec {
    fn encoded_len(&self, value: &Deploy) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Deploy, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[2]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Deploy> for DeployCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Deploy, Error> {
        Constant(&[2]).read(reader)?;
        let _ = reader;
        Ok(Deploy {})
    }
}
/// Validated data for `Retract`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Retract {}
/// Bounded codec for [`Retract`].
#[derive(Clone, Copy, Debug)]
pub struct RetractCodec;
impl Encoder<Retract> for RetractCodec {
    fn encoded_len(&self, value: &Retract) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Retract, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[3]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Retract> for RetractCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Retract, Error> {
        Constant(&[3]).read(reader)?;
        let _ = reader;
        Ok(Retract {})
    }
}
/// Validated data for `TransferAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferAuthority {}
/// Bounded codec for [`TransferAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct TransferAuthorityCodec;
impl Encoder<TransferAuthority> for TransferAuthorityCodec {
    fn encoded_len(&self, value: &TransferAuthority) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &TransferAuthority, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[4]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<TransferAuthority> for TransferAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferAuthority, Error> {
        Constant(&[4]).read(reader)?;
        let _ = reader;
        Ok(TransferAuthority {})
    }
}
/// Validated data for `Finalize`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Finalize {}
/// Bounded codec for [`Finalize`].
#[derive(Clone, Copy, Debug)]
pub struct FinalizeCodec;
impl Encoder<Finalize> for FinalizeCodec {
    fn encoded_len(&self, value: &Finalize) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Finalize, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[5]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Finalize> for FinalizeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Finalize, Error> {
        Constant(&[5]).read(reader)?;
        let _ = reader;
        Ok(Finalize {})
    }
}
use solana_kit_core::codecs::Bytes;
use solana_kit_core::codecs::Mapped;
use solana_kit_core::codecs::{ByteSizePrefix, FrameConsumption};
