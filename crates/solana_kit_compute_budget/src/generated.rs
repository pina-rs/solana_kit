// Generated from a validated Codama model. See README.md and upstream notices.
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Constant, Decoder, Encoder, Error, Limits, Number, Reader, Writer},
};
/// Selected trusted program identity.
pub const PROGRAM: Address = Address::from_bytes([
    3, 6, 70, 111, 229, 33, 23, 50, 255, 236, 173, 186, 114, 195, 155, 231, 188, 140, 229, 187,
    197, 247, 18, 107, 44, 67, 155, 58, 64, 0, 0, 0,
]);
const INSTRUCTION_TAG_0: u8 = 0;
const INSTRUCTION_TAG_1: u8 = 1;
const INSTRUCTION_TAG_2: u8 = 2;
const INSTRUCTION_TAG_3: u8 = 3;
const INSTRUCTION_TAG_4: u8 = 4;
/// Validated data for `RequestUnits`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestUnits {
    /// The `units` wire field.
    pub units: u32,
    /// The `additional_fee` wire field.
    pub additional_fee: u32,
}
/// Bounded codec for [`RequestUnits`].
#[derive(Clone, Copy, Debug)]
pub struct RequestUnitsCodec;
impl Encoder<RequestUnits> for RequestUnitsCodec {
    fn encoded_len(&self, value: &RequestUnits) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.units)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.additional_fee)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &RequestUnits, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_0]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.units, writer)?;
        Number::<u32>::LE.write(&value.additional_fee, writer)?;
        Ok(())
    }
}
impl Decoder<RequestUnits> for RequestUnitsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RequestUnits, Error> {
        Constant(&[INSTRUCTION_TAG_0]).read(reader)?;
        Ok(RequestUnits {
            units: Number::<u32>::LE.read(reader)?,
            additional_fee: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl RequestUnits {
    /// Constructs an owned off-chain program invocation with no accounts.
    pub fn instruction(&self, limits: Limits) -> Result<Instruction, Error> {
        Ok(Instruction {
            program_address: PROGRAM,
            accounts: Vec::new(),
            data: RequestUnitsCodec.encode(self, limits.max_bytes)?,
        })
    }
    /// Parses a canonical invocation, rejecting foreign programs, accounts and trailing data.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        if instruction.program_address != PROGRAM || !instruction.accounts.is_empty() {
            return Err(Error::InvalidValue);
        }

        RequestUnitsCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RequestHeapFrame`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestHeapFrame {
    /// The `bytes` wire field.
    pub bytes: u32,
}
/// Bounded codec for [`RequestHeapFrame`].
#[derive(Clone, Copy, Debug)]
pub struct RequestHeapFrameCodec;
impl Encoder<RequestHeapFrame> for RequestHeapFrameCodec {
    fn encoded_len(&self, value: &RequestHeapFrame) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.bytes)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &RequestHeapFrame, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_1]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.bytes, writer)?;
        Ok(())
    }
}
impl Decoder<RequestHeapFrame> for RequestHeapFrameCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RequestHeapFrame, Error> {
        Constant(&[INSTRUCTION_TAG_1]).read(reader)?;
        Ok(RequestHeapFrame {
            bytes: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl RequestHeapFrame {
    /// Constructs an owned off-chain program invocation with no accounts.
    pub fn instruction(&self, limits: Limits) -> Result<Instruction, Error> {
        Ok(Instruction {
            program_address: PROGRAM,
            accounts: Vec::new(),
            data: RequestHeapFrameCodec.encode(self, limits.max_bytes)?,
        })
    }
    /// Parses a canonical invocation, rejecting foreign programs, accounts and trailing data.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        if instruction.program_address != PROGRAM || !instruction.accounts.is_empty() {
            return Err(Error::InvalidValue);
        }

        RequestHeapFrameCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetComputeUnitLimit`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetComputeUnitLimit {
    /// The `units` wire field.
    pub units: u32,
}
/// Bounded codec for [`SetComputeUnitLimit`].
#[derive(Clone, Copy, Debug)]
pub struct SetComputeUnitLimitCodec;
impl Encoder<SetComputeUnitLimit> for SetComputeUnitLimitCodec {
    fn encoded_len(&self, value: &SetComputeUnitLimit) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.units)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetComputeUnitLimit, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_2]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.units, writer)?;
        Ok(())
    }
}
impl Decoder<SetComputeUnitLimit> for SetComputeUnitLimitCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetComputeUnitLimit, Error> {
        Constant(&[INSTRUCTION_TAG_2]).read(reader)?;
        Ok(SetComputeUnitLimit {
            units: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl SetComputeUnitLimit {
    /// Constructs an owned off-chain program invocation with no accounts.
    pub fn instruction(&self, limits: Limits) -> Result<Instruction, Error> {
        Ok(Instruction {
            program_address: PROGRAM,
            accounts: Vec::new(),
            data: SetComputeUnitLimitCodec.encode(self, limits.max_bytes)?,
        })
    }
    /// Parses a canonical invocation, rejecting foreign programs, accounts and trailing data.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        if instruction.program_address != PROGRAM || !instruction.accounts.is_empty() {
            return Err(Error::InvalidValue);
        }

        SetComputeUnitLimitCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetComputeUnitPrice`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetComputeUnitPrice {
    /// The `micro_lamports` wire field.
    pub micro_lamports: u64,
}
/// Bounded codec for [`SetComputeUnitPrice`].
#[derive(Clone, Copy, Debug)]
pub struct SetComputeUnitPriceCodec;
impl Encoder<SetComputeUnitPrice> for SetComputeUnitPriceCodec {
    fn encoded_len(&self, value: &SetComputeUnitPrice) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.micro_lamports)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetComputeUnitPrice, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_3]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.micro_lamports, writer)?;
        Ok(())
    }
}
impl Decoder<SetComputeUnitPrice> for SetComputeUnitPriceCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetComputeUnitPrice, Error> {
        Constant(&[INSTRUCTION_TAG_3]).read(reader)?;
        Ok(SetComputeUnitPrice {
            micro_lamports: Number::<u64>::LE.read(reader)?,
        })
    }
}
impl SetComputeUnitPrice {
    /// Constructs an owned off-chain program invocation with no accounts.
    pub fn instruction(&self, limits: Limits) -> Result<Instruction, Error> {
        Ok(Instruction {
            program_address: PROGRAM,
            accounts: Vec::new(),
            data: SetComputeUnitPriceCodec.encode(self, limits.max_bytes)?,
        })
    }
    /// Parses a canonical invocation, rejecting foreign programs, accounts and trailing data.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        if instruction.program_address != PROGRAM || !instruction.accounts.is_empty() {
            return Err(Error::InvalidValue);
        }

        SetComputeUnitPriceCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetLoadedAccountsDataSizeLimit`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetLoadedAccountsDataSizeLimit {
    /// The `account_data_size_limit` wire field.
    pub account_data_size_limit: u32,
}
/// Bounded codec for [`SetLoadedAccountsDataSizeLimit`].
#[derive(Clone, Copy, Debug)]
pub struct SetLoadedAccountsDataSizeLimitCodec;
impl Encoder<SetLoadedAccountsDataSizeLimit> for SetLoadedAccountsDataSizeLimitCodec {
    fn encoded_len(&self, value: &SetLoadedAccountsDataSizeLimit) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.account_data_size_limit)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &SetLoadedAccountsDataSizeLimit,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_4]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.account_data_size_limit, writer)?;
        Ok(())
    }
}
impl Decoder<SetLoadedAccountsDataSizeLimit> for SetLoadedAccountsDataSizeLimitCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetLoadedAccountsDataSizeLimit, Error> {
        Constant(&[INSTRUCTION_TAG_4]).read(reader)?;
        Ok(SetLoadedAccountsDataSizeLimit {
            account_data_size_limit: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl SetLoadedAccountsDataSizeLimit {
    /// Constructs an owned off-chain program invocation with no accounts.
    pub fn instruction(&self, limits: Limits) -> Result<Instruction, Error> {
        Ok(Instruction {
            program_address: PROGRAM,
            accounts: Vec::new(),
            data: SetLoadedAccountsDataSizeLimitCodec.encode(self, limits.max_bytes)?,
        })
    }
    /// Parses a canonical invocation, rejecting foreign programs, accounts and trailing data.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        if instruction.program_address != PROGRAM || !instruction.accounts.is_empty() {
            return Err(Error::InvalidValue);
        }

        SetLoadedAccountsDataSizeLimitCodec.decode(&instruction.data, limits)
    }
}
/// All selected instruction data variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComputeBudgetInstructionData {
    /// Data for tag INSTRUCTION_TAG_0.
    RequestUnits(RequestUnits),
    /// Data for tag INSTRUCTION_TAG_1.
    RequestHeapFrame(RequestHeapFrame),
    /// Data for tag INSTRUCTION_TAG_2.
    SetComputeUnitLimit(SetComputeUnitLimit),
    /// Data for tag INSTRUCTION_TAG_3.
    SetComputeUnitPrice(SetComputeUnitPrice),
    /// Data for tag INSTRUCTION_TAG_4.
    SetLoadedAccountsDataSizeLimit(SetLoadedAccountsDataSizeLimit),
}
/// Strict bounded instruction dispatch codec.
#[derive(Clone, Copy, Debug)]
pub struct ComputeBudgetInstructionDataCodec;
impl Encoder<ComputeBudgetInstructionData> for ComputeBudgetInstructionDataCodec {
    fn encoded_len(&self, value: &ComputeBudgetInstructionData) -> Result<usize, Error> {
        match value {
            ComputeBudgetInstructionData::RequestUnits(value) => {
                RequestUnitsCodec.encoded_len(value)
            }
            ComputeBudgetInstructionData::RequestHeapFrame(value) => {
                RequestHeapFrameCodec.encoded_len(value)
            }
            ComputeBudgetInstructionData::SetComputeUnitLimit(value) => {
                SetComputeUnitLimitCodec.encoded_len(value)
            }
            ComputeBudgetInstructionData::SetComputeUnitPrice(value) => {
                SetComputeUnitPriceCodec.encoded_len(value)
            }
            ComputeBudgetInstructionData::SetLoadedAccountsDataSizeLimit(value) => {
                SetLoadedAccountsDataSizeLimitCodec.encoded_len(value)
            }
        }
    }
    fn write(
        &self,
        value: &ComputeBudgetInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            ComputeBudgetInstructionData::RequestUnits(value) => {
                RequestUnitsCodec.write(value, writer)
            }
            ComputeBudgetInstructionData::RequestHeapFrame(value) => {
                RequestHeapFrameCodec.write(value, writer)
            }
            ComputeBudgetInstructionData::SetComputeUnitLimit(value) => {
                SetComputeUnitLimitCodec.write(value, writer)
            }
            ComputeBudgetInstructionData::SetComputeUnitPrice(value) => {
                SetComputeUnitPriceCodec.write(value, writer)
            }
            ComputeBudgetInstructionData::SetLoadedAccountsDataSizeLimit(value) => {
                SetLoadedAccountsDataSizeLimitCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<ComputeBudgetInstructionData> for ComputeBudgetInstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ComputeBudgetInstructionData, Error> {
        let tag = *reader.remaining().first().ok_or(Error::Truncated)?;

        match tag {
            INSTRUCTION_TAG_0 => Ok(ComputeBudgetInstructionData::RequestUnits(
                RequestUnitsCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_1 => Ok(ComputeBudgetInstructionData::RequestHeapFrame(
                RequestHeapFrameCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_2 => Ok(ComputeBudgetInstructionData::SetComputeUnitLimit(
                SetComputeUnitLimitCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_3 => Ok(ComputeBudgetInstructionData::SetComputeUnitPrice(
                SetComputeUnitPriceCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_4 => Ok(
                ComputeBudgetInstructionData::SetLoadedAccountsDataSizeLimit(
                    SetLoadedAccountsDataSizeLimitCodec.read(reader)?,
                ),
            ),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
}
/// Identifies a tag without asserting the full payload is valid.
pub fn identify_instruction(data: &[u8]) -> Result<u8, Error> {
    let tag = *data.first().ok_or(Error::Truncated)?;

    match tag {
        INSTRUCTION_TAG_0 => Ok(tag),
        INSTRUCTION_TAG_1 => Ok(tag),
        INSTRUCTION_TAG_2 => Ok(tag),
        INSTRUCTION_TAG_3 => Ok(tag),
        INSTRUCTION_TAG_4 => Ok(tag),
        _ => Err(Error::InvalidDiscriminator),
    }
}
/// Fully parses a canonical invocation with the fixed selected program identity.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<ComputeBudgetInstructionData, Error> {
    if instruction.program_address != PROGRAM || !instruction.accounts.is_empty() {
        return Err(Error::InvalidValue);
    }

    ComputeBudgetInstructionDataCodec.decode(&instruction.data, limits)
}
