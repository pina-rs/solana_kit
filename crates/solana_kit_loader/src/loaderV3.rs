// Generated from the exact pinned Codama input; see upstream notices.
use alloc::vec::Vec;
use solana_kit_core::codecs::Constant;
use solana_kit_core::codecs::Number;
use solana_kit_core::codecs::{Decoder, Encoder, Error, Reader, Writer};
/// Every pinned instruction data variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoaderV3InstructionData {
    /// Data for `InitializeBuffer`.
    InitializeBuffer(InitializeBuffer),
    /// Data for `Write`.
    Write(Write),
    /// Data for `DeployWithMaxDataLen`.
    DeployWithMaxDataLen(DeployWithMaxDataLen),
    /// Data for `Upgrade`.
    Upgrade(Upgrade),
    /// Data for `SetAuthority`.
    SetAuthority(SetAuthority),
    /// Data for `Close`.
    Close(Close),
    /// Data for `ExtendProgram`.
    ExtendProgram(ExtendProgram),
    /// Data for `SetAuthorityChecked`.
    SetAuthorityChecked(SetAuthorityChecked),
}
/// Prefix-safe multi-byte instruction dispatch.
#[derive(Clone, Copy, Debug)]
pub struct LoaderV3InstructionDataCodec;
impl Encoder<LoaderV3InstructionData> for LoaderV3InstructionDataCodec {
    fn encoded_len(&self, value: &LoaderV3InstructionData) -> Result<usize, Error> {
        match value {
            LoaderV3InstructionData::InitializeBuffer(value) => {
                InitializeBufferCodec.encoded_len(value)
            }
            LoaderV3InstructionData::Write(value) => WriteCodec.encoded_len(value),
            LoaderV3InstructionData::DeployWithMaxDataLen(value) => {
                DeployWithMaxDataLenCodec.encoded_len(value)
            }
            LoaderV3InstructionData::Upgrade(value) => UpgradeCodec.encoded_len(value),
            LoaderV3InstructionData::SetAuthority(value) => SetAuthorityCodec.encoded_len(value),
            LoaderV3InstructionData::Close(value) => CloseCodec.encoded_len(value),
            LoaderV3InstructionData::ExtendProgram(value) => ExtendProgramCodec.encoded_len(value),
            LoaderV3InstructionData::SetAuthorityChecked(value) => {
                SetAuthorityCheckedCodec.encoded_len(value)
            }
        }
    }
    fn write(&self, value: &LoaderV3InstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            LoaderV3InstructionData::InitializeBuffer(value) => {
                InitializeBufferCodec.write(value, writer)
            }
            LoaderV3InstructionData::Write(value) => WriteCodec.write(value, writer),
            LoaderV3InstructionData::DeployWithMaxDataLen(value) => {
                DeployWithMaxDataLenCodec.write(value, writer)
            }
            LoaderV3InstructionData::Upgrade(value) => UpgradeCodec.write(value, writer),
            LoaderV3InstructionData::SetAuthority(value) => SetAuthorityCodec.write(value, writer),
            LoaderV3InstructionData::Close(value) => CloseCodec.write(value, writer),
            LoaderV3InstructionData::ExtendProgram(value) => {
                ExtendProgramCodec.write(value, writer)
            }
            LoaderV3InstructionData::SetAuthorityChecked(value) => {
                SetAuthorityCheckedCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<LoaderV3InstructionData> for LoaderV3InstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LoaderV3InstructionData, Error> {
        reader.transaction(|reader| match reader.remaining() {
            [0, 0, 0, 0, ..] => Ok(LoaderV3InstructionData::InitializeBuffer(
                InitializeBufferCodec.read(reader)?,
            )),
            [1, 0, 0, 0, ..] => Ok(LoaderV3InstructionData::Write(WriteCodec.read(reader)?)),
            [2, 0, 0, 0, ..] => Ok(LoaderV3InstructionData::DeployWithMaxDataLen(
                DeployWithMaxDataLenCodec.read(reader)?,
            )),
            [3, 0, 0, 0, ..] => Ok(LoaderV3InstructionData::Upgrade(UpgradeCodec.read(reader)?)),
            [4, 0, 0, 0, ..] => Ok(LoaderV3InstructionData::SetAuthority(
                SetAuthorityCodec.read(reader)?,
            )),
            [5, 0, 0, 0, ..] => Ok(LoaderV3InstructionData::Close(CloseCodec.read(reader)?)),
            [6, 0, 0, 0, ..] => Ok(LoaderV3InstructionData::ExtendProgram(
                ExtendProgramCodec.read(reader)?,
            )),
            [7, 0, 0, 0, ..] => Ok(LoaderV3InstructionData::SetAuthorityChecked(
                SetAuthorityCheckedCodec.read(reader)?,
            )),
            [] => Err(Error::Truncated),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `InitializeBuffer`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeBuffer {}
/// Bounded codec for [`InitializeBuffer`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeBufferCodec;
impl Encoder<InitializeBuffer> for InitializeBufferCodec {
    fn encoded_len(&self, value: &InitializeBuffer) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &InitializeBuffer, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[0, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<InitializeBuffer> for InitializeBufferCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeBuffer, Error> {
        Constant(&[0, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(InitializeBuffer {})
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
        let size = 4usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Bytes,
                    prefix: Number::<u64>::LE,
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.bytes.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Write, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[1, 0, 0, 0]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.offset, writer)?;
        ByteSizePrefix {
            item: Bytes,
            prefix: Number::<u64>::LE,
            consumption: FrameConsumption::Exact,
        }
        .write(value.bytes.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<Write> for WriteCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Write, Error> {
        Constant(&[1, 0, 0, 0]).read(reader)?;
        Ok(Write {
            offset: Number::<u32>::LE.read(reader)?,
            bytes: ByteSizePrefix {
                item: Bytes,
                prefix: Number::<u64>::LE,
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `DeployWithMaxDataLen`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeployWithMaxDataLen {
    /// The `max_data_len` wire field.
    pub max_data_len: u64,
}
/// Bounded codec for [`DeployWithMaxDataLen`].
#[derive(Clone, Copy, Debug)]
pub struct DeployWithMaxDataLenCodec;
impl Encoder<DeployWithMaxDataLen> for DeployWithMaxDataLenCodec {
    fn encoded_len(&self, value: &DeployWithMaxDataLen) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.max_data_len)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &DeployWithMaxDataLen, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[2, 0, 0, 0]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.max_data_len, writer)?;
        Ok(())
    }
}
impl Decoder<DeployWithMaxDataLen> for DeployWithMaxDataLenCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DeployWithMaxDataLen, Error> {
        Constant(&[2, 0, 0, 0]).read(reader)?;
        Ok(DeployWithMaxDataLen {
            max_data_len: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `Upgrade`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Upgrade {}
/// Bounded codec for [`Upgrade`].
#[derive(Clone, Copy, Debug)]
pub struct UpgradeCodec;
impl Encoder<Upgrade> for UpgradeCodec {
    fn encoded_len(&self, value: &Upgrade) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Upgrade, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[3, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Upgrade> for UpgradeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Upgrade, Error> {
        Constant(&[3, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(Upgrade {})
    }
}
/// Validated data for `SetAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetAuthority {}
/// Bounded codec for [`SetAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct SetAuthorityCodec;
impl Encoder<SetAuthority> for SetAuthorityCodec {
    fn encoded_len(&self, value: &SetAuthority) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &SetAuthority, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[4, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<SetAuthority> for SetAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetAuthority, Error> {
        Constant(&[4, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(SetAuthority {})
    }
}
/// Validated data for `Close`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Close {}
/// Bounded codec for [`Close`].
#[derive(Clone, Copy, Debug)]
pub struct CloseCodec;
impl Encoder<Close> for CloseCodec {
    fn encoded_len(&self, value: &Close) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Close, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[5, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Close> for CloseCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Close, Error> {
        Constant(&[5, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(Close {})
    }
}
/// Validated data for `ExtendProgram`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtendProgram {
    /// The `additional_bytes` wire field.
    pub additional_bytes: u32,
}
/// Bounded codec for [`ExtendProgram`].
#[derive(Clone, Copy, Debug)]
pub struct ExtendProgramCodec;
impl Encoder<ExtendProgram> for ExtendProgramCodec {
    fn encoded_len(&self, value: &ExtendProgram) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.additional_bytes)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ExtendProgram, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[6, 0, 0, 0]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.additional_bytes, writer)?;
        Ok(())
    }
}
impl Decoder<ExtendProgram> for ExtendProgramCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtendProgram, Error> {
        Constant(&[6, 0, 0, 0]).read(reader)?;
        Ok(ExtendProgram {
            additional_bytes: Number::<u32>::LE.read(reader)?,
        })
    }
}
/// Validated data for `SetAuthorityChecked`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetAuthorityChecked {}
/// Bounded codec for [`SetAuthorityChecked`].
#[derive(Clone, Copy, Debug)]
pub struct SetAuthorityCheckedCodec;
impl Encoder<SetAuthorityChecked> for SetAuthorityCheckedCodec {
    fn encoded_len(&self, value: &SetAuthorityChecked) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &SetAuthorityChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[7, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<SetAuthorityChecked> for SetAuthorityCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetAuthorityChecked, Error> {
        Constant(&[7, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(SetAuthorityChecked {})
    }
}
use solana_kit_core::codecs::Bytes;
use solana_kit_core::codecs::{ByteSizePrefix, FrameConsumption};
