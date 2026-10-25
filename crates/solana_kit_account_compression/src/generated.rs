// Generated from a validated Codama model. See README.md and upstream notices.
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Constant, Decoder, Encoder, Error, Limits, Number, Reader, Writer},
};
/// Selected trusted program identity.
pub const PROGRAM: Address = Address::from_bytes([
    9, 42, 19, 238, 149, 196, 28, 186, 8, 166, 127, 90, 198, 126, 141, 247, 225, 218, 17, 98, 94,
    29, 100, 19, 127, 143, 79, 35, 131, 3, 127, 20,
]);
const PROGRAM_ERROR_6000: u32 = 6000;
const PROGRAM_ERROR_6001: u32 = 6001;
const PROGRAM_ERROR_6002: u32 = 6002;
const PROGRAM_ERROR_6003: u32 = 6003;
const PROGRAM_ERROR_6004: u32 = 6004;
const PROGRAM_ERROR_6005: u32 = 6005;
const PROGRAM_ERROR_6006: u32 = 6006;
const PROGRAM_ERROR_6007: u32 = 6007;
const PROGRAM_ERROR_6008: u32 = 6008;
const FIXED_COUNT_6: usize = 6;
const FIXED_COUNT_32: usize = 32;

struct CheckedCounted<P, C> {
    prefix: P,
    item: C,
    minimum: usize,
}
impl<P: Encoder<u32>, C: Encoder<T>, T> Encoder<[T]> for CheckedCounted<P, C> {
    fn encoded_len(&self, value: &[T]) -> Result<usize, Error> {
        Counted {
            prefix: &self.prefix,
            item: &self.item,
        }
        .encoded_len(value)
    }
    fn write(&self, value: &[T], writer: &mut Writer<'_>) -> Result<(), Error> {
        Counted {
            prefix: &self.prefix,
            item: &self.item,
        }
        .write(value, writer)
    }
}
impl<P: Decoder<u32>, C: Decoder<T>, T> Decoder<Vec<T>> for CheckedCounted<P, C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
        let count = usize::try_from(self.prefix.read(reader)?).map_err(|_| Error::Overflow)?;
        let minimum = count.checked_mul(self.minimum).ok_or(Error::Overflow)?;

        if minimum > reader.remaining().len() {
            return Err(Error::Truncated);
        }

        solana_kit_core::codecs::Sequence {
            item: &self.item,
            count: solana_kit_core::codecs::Count::Fixed(count),
        }
        .read(reader)
    }
}
fn role_matches(
    program: Address,
    meta: &AccountMeta,
    expected: &(bool, Option<bool>, bool),
) -> bool {
    let (writable, signer, optional) = *expected;

    if optional && meta.address == program {
        return meta.role == AccountRole::Readonly;
    }

    meta.role.is_writable() == writable
        && signer.is_none_or(|signer| meta.role.is_signer() == signer)
}
/// Validated data for `ApplicationDataEventV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationDataEventV1 {
    /// The `application_data` wire field.
    pub application_data: Vec<u8>,
}
/// Bounded codec for [`ApplicationDataEventV1`].
#[derive(Clone, Copy, Debug)]
pub struct ApplicationDataEventV1Codec;
impl Encoder<ApplicationDataEventV1> for ApplicationDataEventV1Codec {
    fn encoded_len(&self, value: &ApplicationDataEventV1) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.application_data.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ApplicationDataEventV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.application_data.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<ApplicationDataEventV1> for ApplicationDataEventV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ApplicationDataEventV1, Error> {
        Ok(ApplicationDataEventV1 {
            application_data: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `ChangeLogEventV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeLogEventV1 {
    /// The `id` wire field.
    pub id: Address,
    /// The `path` wire field.
    pub path: Vec<PathNode>,
    /// The `seq` wire field.
    pub seq: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`ChangeLogEventV1`].
#[derive(Clone, Copy, Debug)]
pub struct ChangeLogEventV1Codec;
impl Encoder<ChangeLogEventV1> for ChangeLogEventV1Codec {
    fn encoded_len(&self, value: &ChangeLogEventV1) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.id)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: PathNodeCodec,
                    minimum: 36,
                }
                .encoded_len(value.path.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.seq)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ChangeLogEventV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.id, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: PathNodeCodec,
            minimum: 36,
        }
        .write(value.path.as_slice(), writer)?;
        Number::<u64>::LE.write(&value.seq, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<ChangeLogEventV1> for ChangeLogEventV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ChangeLogEventV1, Error> {
        Ok(ChangeLogEventV1 {
            id: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            path: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: PathNodeCodec,
                minimum: 36,
            }
            .read(reader)?,
            seq: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ConcurrentMerkleTreeHeader`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConcurrentMerkleTreeHeader {
    /// The `account_type` wire field.
    pub account_type: CompressionAccountType,
    /// The `header` wire field.
    pub header: ConcurrentMerkleTreeHeaderData,
}
/// Bounded codec for [`ConcurrentMerkleTreeHeader`].
#[derive(Clone, Copy, Debug)]
pub struct ConcurrentMerkleTreeHeaderCodec;
impl Encoder<ConcurrentMerkleTreeHeader> for ConcurrentMerkleTreeHeaderCodec {
    fn encoded_len(&self, value: &ConcurrentMerkleTreeHeader) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(CompressionAccountTypeCodec.encoded_len(&value.account_type)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ConcurrentMerkleTreeHeaderDataCodec.encoded_len(&value.header)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConcurrentMerkleTreeHeader,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        CompressionAccountTypeCodec.write(&value.account_type, writer)?;
        ConcurrentMerkleTreeHeaderDataCodec.write(&value.header, writer)?;
        Ok(())
    }
}
impl Decoder<ConcurrentMerkleTreeHeader> for ConcurrentMerkleTreeHeaderCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConcurrentMerkleTreeHeader, Error> {
        Ok(ConcurrentMerkleTreeHeader {
            account_type: CompressionAccountTypeCodec.read(reader)?,
            header: ConcurrentMerkleTreeHeaderDataCodec.read(reader)?,
        })
    }
}
/// Validated data for `ConcurrentMerkleTreeHeaderDataV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConcurrentMerkleTreeHeaderDataV1 {
    /// The `max_buffer_size` wire field.
    pub max_buffer_size: u32,
    /// The `max_depth` wire field.
    pub max_depth: u32,
    /// The `authority` wire field.
    pub authority: Address,
    /// The `creation_slot` wire field.
    pub creation_slot: u64,
    /// The `padding` wire field.
    pub padding: Vec<u8>,
}
/// Bounded codec for [`ConcurrentMerkleTreeHeaderDataV1`].
#[derive(Clone, Copy, Debug)]
pub struct ConcurrentMerkleTreeHeaderDataV1Codec;
impl Encoder<ConcurrentMerkleTreeHeaderDataV1> for ConcurrentMerkleTreeHeaderDataV1Codec {
    fn encoded_len(&self, value: &ConcurrentMerkleTreeHeaderDataV1) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.max_buffer_size)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.max_depth)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.creation_slot)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Number::<u8>::LE,
                    count: Count::Fixed(FIXED_COUNT_6),
                })
                .encoded_len(&value.padding)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConcurrentMerkleTreeHeaderDataV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u32>::LE.write(&value.max_buffer_size, writer)?;
        Number::<u32>::LE.write(&value.max_depth, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.authority, writer)?;
        Number::<u64>::LE.write(&value.creation_slot, writer)?;
        OwnedSequence(Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(FIXED_COUNT_6),
        })
        .write(&value.padding, writer)?;
        Ok(())
    }
}
impl Decoder<ConcurrentMerkleTreeHeaderDataV1> for ConcurrentMerkleTreeHeaderDataV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConcurrentMerkleTreeHeaderDataV1, Error> {
        Ok(ConcurrentMerkleTreeHeaderDataV1 {
            max_buffer_size: Number::<u32>::LE.read(reader)?,
            max_depth: Number::<u32>::LE.read(reader)?,
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            creation_slot: Number::<u64>::LE.read(reader)?,
            padding: OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::Fixed(FIXED_COUNT_6),
            })
            .read(reader)?,
        })
    }
}
/// Validated data for `PathNode`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathNode {
    /// The `node` wire field.
    pub node: Vec<u8>,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`PathNode`].
#[derive(Clone, Copy, Debug)]
pub struct PathNodeCodec;
impl Encoder<PathNode> for PathNodeCodec {
    fn encoded_len(&self, value: &PathNode) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Number::<u8>::LE,
                    count: Count::Fixed(FIXED_COUNT_32),
                })
                .encoded_len(&value.node)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PathNode, writer: &mut Writer<'_>) -> Result<(), Error> {
        OwnedSequence(Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(FIXED_COUNT_32),
        })
        .write(&value.node, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<PathNode> for PathNodeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PathNode, Error> {
        Ok(PathNode {
            node: OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::Fixed(FIXED_COUNT_32),
            })
            .read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplicationDataEvent {
    /// The `V1` variant.
    V1(ApplicationDataEventV1),
}
/// Bounded codec for [`ApplicationDataEvent`].
#[derive(Clone, Copy, Debug)]
pub struct ApplicationDataEventCodec;
const APPLICATIONDATAEVENT_TAG_0: u8 = 0;
impl Encoder<ApplicationDataEvent> for ApplicationDataEventCodec {
    fn encoded_len(&self, value: &ApplicationDataEvent) -> Result<usize, Error> {
        let size = match value {
            ApplicationDataEvent::V1(value) => ApplicationDataEventV1Codec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &ApplicationDataEvent, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            ApplicationDataEvent::V1(value) => {
                Number::<u8>::LE.write(&APPLICATIONDATAEVENT_TAG_0, writer)?;
                ApplicationDataEventV1Codec.write(value, writer)
            }
        }
    }
}
impl Decoder<ApplicationDataEvent> for ApplicationDataEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ApplicationDataEvent, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            APPLICATIONDATAEVENT_TAG_0 => Ok(ApplicationDataEvent::V1(
                ApplicationDataEventV1Codec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChangeLogEvent {
    /// The `V1` variant.
    V1(ChangeLogEventV1),
}
/// Bounded codec for [`ChangeLogEvent`].
#[derive(Clone, Copy, Debug)]
pub struct ChangeLogEventCodec;
const CHANGELOGEVENT_TAG_0: u8 = 0;
impl Encoder<ChangeLogEvent> for ChangeLogEventCodec {
    fn encoded_len(&self, value: &ChangeLogEvent) -> Result<usize, Error> {
        let size = match value {
            ChangeLogEvent::V1(value) => ChangeLogEventV1Codec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &ChangeLogEvent, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            ChangeLogEvent::V1(value) => {
                Number::<u8>::LE.write(&CHANGELOGEVENT_TAG_0, writer)?;
                ChangeLogEventV1Codec.write(value, writer)
            }
        }
    }
}
impl Decoder<ChangeLogEvent> for ChangeLogEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ChangeLogEvent, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            CHANGELOGEVENT_TAG_0 => Ok(ChangeLogEvent::V1(ChangeLogEventV1Codec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccountCompressionEvent {
    /// The `ChangeLog` variant.
    ChangeLog(ChangeLogEvent),
    /// The `ApplicationData` variant.
    ApplicationData(ApplicationDataEvent),
}
/// Bounded codec for [`AccountCompressionEvent`].
#[derive(Clone, Copy, Debug)]
pub struct AccountCompressionEventCodec;
const ACCOUNTCOMPRESSIONEVENT_TAG_0: u8 = 0;
const ACCOUNTCOMPRESSIONEVENT_TAG_1: u8 = 1;
impl Encoder<AccountCompressionEvent> for AccountCompressionEventCodec {
    fn encoded_len(&self, value: &AccountCompressionEvent) -> Result<usize, Error> {
        let size = match value {
            AccountCompressionEvent::ChangeLog(value) => ChangeLogEventCodec.encoded_len(value)?,
            AccountCompressionEvent::ApplicationData(value) => {
                ApplicationDataEventCodec.encoded_len(value)?
            }
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &AccountCompressionEvent, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            AccountCompressionEvent::ChangeLog(value) => {
                Number::<u8>::LE.write(&ACCOUNTCOMPRESSIONEVENT_TAG_0, writer)?;
                ChangeLogEventCodec.write(value, writer)
            }
            AccountCompressionEvent::ApplicationData(value) => {
                Number::<u8>::LE.write(&ACCOUNTCOMPRESSIONEVENT_TAG_1, writer)?;
                ApplicationDataEventCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<AccountCompressionEvent> for AccountCompressionEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AccountCompressionEvent, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            ACCOUNTCOMPRESSIONEVENT_TAG_0 => Ok(AccountCompressionEvent::ChangeLog(
                ChangeLogEventCodec.read(reader)?,
            )),
            ACCOUNTCOMPRESSIONEVENT_TAG_1 => Ok(AccountCompressionEvent::ApplicationData(
                ApplicationDataEventCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Finite u8 tagged enum.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompressionAccountType {
    /// The `Uninitialized` variant.
    Uninitialized,
    /// The `ConcurrentMerkleTree` variant.
    ConcurrentMerkleTree,
}
/// Strict codec for [`CompressionAccountType`].
#[derive(Clone, Copy, Debug)]
pub struct CompressionAccountTypeCodec;
impl CompressionAccountTypeCodec {
    const VALUES: &'static [CompressionAccountType] = &[
        CompressionAccountType::Uninitialized,
        CompressionAccountType::ConcurrentMerkleTree,
    ];
}
impl Encoder<CompressionAccountType> for CompressionAccountTypeCodec {
    fn encoded_len(&self, _: &CompressionAccountType) -> Result<usize, Error> {
        Ok(1)
    }
    fn write(&self, value: &CompressionAccountType, writer: &mut Writer<'_>) -> Result<(), Error> {
        let index = Self::VALUES
            .iter()
            .position(|candidate| candidate == value)
            .ok_or(Error::InvalidDiscriminator)?;
        let tag = u8::try_from(index).map_err(|_| Error::Overflow)?;
        Number::<u8>::LE.write(&tag, writer)
    }
}
impl Decoder<CompressionAccountType> for CompressionAccountTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CompressionAccountType, Error> {
        Self::VALUES
            .get(usize::from(Number::<u8>::LE.read(reader)?))
            .cloned()
            .ok_or(Error::InvalidDiscriminator)
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConcurrentMerkleTreeHeaderData {
    /// The `V1` variant.
    V1(ConcurrentMerkleTreeHeaderDataV1),
}
/// Bounded codec for [`ConcurrentMerkleTreeHeaderData`].
#[derive(Clone, Copy, Debug)]
pub struct ConcurrentMerkleTreeHeaderDataCodec;
const CONCURRENTMERKLETREEHEADERDATA_TAG_0: u8 = 0;
impl Encoder<ConcurrentMerkleTreeHeaderData> for ConcurrentMerkleTreeHeaderDataCodec {
    fn encoded_len(&self, value: &ConcurrentMerkleTreeHeaderData) -> Result<usize, Error> {
        let size = match value {
            ConcurrentMerkleTreeHeaderData::V1(value) => {
                ConcurrentMerkleTreeHeaderDataV1Codec.encoded_len(value)?
            }
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(
        &self,
        value: &ConcurrentMerkleTreeHeaderData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            ConcurrentMerkleTreeHeaderData::V1(value) => {
                Number::<u8>::LE.write(&CONCURRENTMERKLETREEHEADERDATA_TAG_0, writer)?;
                ConcurrentMerkleTreeHeaderDataV1Codec.write(value, writer)
            }
        }
    }
}
impl Decoder<ConcurrentMerkleTreeHeaderData> for ConcurrentMerkleTreeHeaderDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConcurrentMerkleTreeHeaderData, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            CONCURRENTMERKLETREEHEADERDATA_TAG_0 => Ok(ConcurrentMerkleTreeHeaderData::V1(
                ConcurrentMerkleTreeHeaderDataV1Codec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `InitEmptyMerkleTree`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitEmptyMerkleTree {
    /// The `max_depth` wire field.
    pub max_depth: u32,
    /// The `max_buffer_size` wire field.
    pub max_buffer_size: u32,
}
/// Bounded codec for [`InitEmptyMerkleTree`].
#[derive(Clone, Copy, Debug)]
pub struct InitEmptyMerkleTreeCodec;
impl Encoder<InitEmptyMerkleTree> for InitEmptyMerkleTreeCodec {
    fn encoded_len(&self, value: &InitEmptyMerkleTree) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.max_depth)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.max_buffer_size)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitEmptyMerkleTree, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[191, 11, 119, 7, 180, 107, 220, 110]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.max_depth, writer)?;
        Number::<u32>::LE.write(&value.max_buffer_size, writer)?;
        Ok(())
    }
}
impl Decoder<InitEmptyMerkleTree> for InitEmptyMerkleTreeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitEmptyMerkleTree, Error> {
        Constant(&[191, 11, 119, 7, 180, 107, 220, 110]).read(reader)?;
        Ok(InitEmptyMerkleTree {
            max_depth: Number::<u32>::LE.read(reader)?,
            max_buffer_size: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl InitEmptyMerkleTree {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* merkle_tree */ (true, Some(false), false),
                /* authority */ (false, Some(true), false),
                /* noop */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = InitEmptyMerkleTreeCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        InitEmptyMerkleTreeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ReplaceLeaf`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplaceLeaf {
    /// The `root` wire field.
    pub root: Vec<u8>,
    /// The `previous_leaf` wire field.
    pub previous_leaf: Vec<u8>,
    /// The `new_leaf` wire field.
    pub new_leaf: Vec<u8>,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`ReplaceLeaf`].
#[derive(Clone, Copy, Debug)]
pub struct ReplaceLeafCodec;
impl Encoder<ReplaceLeaf> for ReplaceLeafCodec {
    fn encoded_len(&self, value: &ReplaceLeaf) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Number::<u8>::LE,
                    count: Count::Fixed(FIXED_COUNT_32),
                })
                .encoded_len(&value.root)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Number::<u8>::LE,
                    count: Count::Fixed(FIXED_COUNT_32),
                })
                .encoded_len(&value.previous_leaf)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Number::<u8>::LE,
                    count: Count::Fixed(FIXED_COUNT_32),
                })
                .encoded_len(&value.new_leaf)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ReplaceLeaf, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[204, 165, 76, 100, 73, 147, 0, 128]).write(&(), writer)?;
        OwnedSequence(Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(FIXED_COUNT_32),
        })
        .write(&value.root, writer)?;
        OwnedSequence(Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(FIXED_COUNT_32),
        })
        .write(&value.previous_leaf, writer)?;
        OwnedSequence(Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(FIXED_COUNT_32),
        })
        .write(&value.new_leaf, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<ReplaceLeaf> for ReplaceLeafCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ReplaceLeaf, Error> {
        Constant(&[204, 165, 76, 100, 73, 147, 0, 128]).read(reader)?;
        Ok(ReplaceLeaf {
            root: OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::Fixed(FIXED_COUNT_32),
            })
            .read(reader)?,
            previous_leaf: OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::Fixed(FIXED_COUNT_32),
            })
            .read(reader)?,
            new_leaf: OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::Fixed(FIXED_COUNT_32),
            })
            .read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl ReplaceLeaf {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* merkle_tree */ (true, Some(false), false),
                /* authority */ (false, Some(true), false),
                /* noop */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = ReplaceLeafCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        ReplaceLeafCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TransferAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferAuthority {
    /// The `new_authority` wire field.
    pub new_authority: Address,
}
/// Bounded codec for [`TransferAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct TransferAuthorityCodec;
impl Encoder<TransferAuthority> for TransferAuthorityCodec {
    fn encoded_len(&self, value: &TransferAuthority) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.new_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferAuthority, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[48, 169, 76, 72, 229, 180, 55, 161]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.new_authority, writer)?;
        Ok(())
    }
}
impl Decoder<TransferAuthority> for TransferAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferAuthority, Error> {
        Constant(&[48, 169, 76, 72, 229, 180, 55, 161]).read(reader)?;
        Ok(TransferAuthority {
            new_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
impl TransferAuthority {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 2 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* merkle_tree */ (true, Some(false), false),
                /* authority */ (false, Some(true), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = TransferAuthorityCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        TransferAuthorityCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `VerifyLeaf`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifyLeaf {
    /// The `root` wire field.
    pub root: Vec<u8>,
    /// The `leaf` wire field.
    pub leaf: Vec<u8>,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`VerifyLeaf`].
#[derive(Clone, Copy, Debug)]
pub struct VerifyLeafCodec;
impl Encoder<VerifyLeaf> for VerifyLeafCodec {
    fn encoded_len(&self, value: &VerifyLeaf) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Number::<u8>::LE,
                    count: Count::Fixed(FIXED_COUNT_32),
                })
                .encoded_len(&value.root)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Number::<u8>::LE,
                    count: Count::Fixed(FIXED_COUNT_32),
                })
                .encoded_len(&value.leaf)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &VerifyLeaf, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[124, 220, 22, 223, 104, 10, 250, 224]).write(&(), writer)?;
        OwnedSequence(Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(FIXED_COUNT_32),
        })
        .write(&value.root, writer)?;
        OwnedSequence(Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(FIXED_COUNT_32),
        })
        .write(&value.leaf, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<VerifyLeaf> for VerifyLeafCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VerifyLeaf, Error> {
        Constant(&[124, 220, 22, 223, 104, 10, 250, 224]).read(reader)?;
        Ok(VerifyLeaf {
            root: OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::Fixed(FIXED_COUNT_32),
            })
            .read(reader)?,
            leaf: OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::Fixed(FIXED_COUNT_32),
            })
            .read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl VerifyLeaf {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.is_empty() {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([/* merkle_tree */ (false, Some(false), false)])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[1..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = VerifyLeafCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        VerifyLeafCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Append`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Append {
    /// The `leaf` wire field.
    pub leaf: Vec<u8>,
}
/// Bounded codec for [`Append`].
#[derive(Clone, Copy, Debug)]
pub struct AppendCodec;
impl Encoder<Append> for AppendCodec {
    fn encoded_len(&self, value: &Append) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Number::<u8>::LE,
                    count: Count::Fixed(FIXED_COUNT_32),
                })
                .encoded_len(&value.leaf)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Append, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[149, 120, 18, 222, 236, 225, 88, 203]).write(&(), writer)?;
        OwnedSequence(Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(FIXED_COUNT_32),
        })
        .write(&value.leaf, writer)?;
        Ok(())
    }
}
impl Decoder<Append> for AppendCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Append, Error> {
        Constant(&[149, 120, 18, 222, 236, 225, 88, 203]).read(reader)?;
        Ok(Append {
            leaf: OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::Fixed(FIXED_COUNT_32),
            })
            .read(reader)?,
        })
    }
}
impl Append {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* merkle_tree */ (true, Some(false), false),
                /* authority */ (false, Some(true), false),
                /* noop */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = AppendCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        AppendCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `InsertOrAppend`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsertOrAppend {
    /// The `root` wire field.
    pub root: Vec<u8>,
    /// The `leaf` wire field.
    pub leaf: Vec<u8>,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`InsertOrAppend`].
#[derive(Clone, Copy, Debug)]
pub struct InsertOrAppendCodec;
impl Encoder<InsertOrAppend> for InsertOrAppendCodec {
    fn encoded_len(&self, value: &InsertOrAppend) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Number::<u8>::LE,
                    count: Count::Fixed(FIXED_COUNT_32),
                })
                .encoded_len(&value.root)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Number::<u8>::LE,
                    count: Count::Fixed(FIXED_COUNT_32),
                })
                .encoded_len(&value.leaf)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InsertOrAppend, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[6, 42, 50, 190, 51, 109, 178, 168]).write(&(), writer)?;
        OwnedSequence(Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(FIXED_COUNT_32),
        })
        .write(&value.root, writer)?;
        OwnedSequence(Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(FIXED_COUNT_32),
        })
        .write(&value.leaf, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<InsertOrAppend> for InsertOrAppendCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InsertOrAppend, Error> {
        Constant(&[6, 42, 50, 190, 51, 109, 178, 168]).read(reader)?;
        Ok(InsertOrAppend {
            root: OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::Fixed(FIXED_COUNT_32),
            })
            .read(reader)?,
            leaf: OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::Fixed(FIXED_COUNT_32),
            })
            .read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl InsertOrAppend {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* merkle_tree */ (true, Some(false), false),
                /* authority */ (false, Some(true), false),
                /* noop */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = InsertOrAppendCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        InsertOrAppendCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CloseEmptyTree`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseEmptyTree {}
/// Bounded codec for [`CloseEmptyTree`].
#[derive(Clone, Copy, Debug)]
pub struct CloseEmptyTreeCodec;
impl Encoder<CloseEmptyTree> for CloseEmptyTreeCodec {
    fn encoded_len(&self, value: &CloseEmptyTree) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CloseEmptyTree, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[50, 14, 219, 107, 78, 103, 16, 103]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CloseEmptyTree> for CloseEmptyTreeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseEmptyTree, Error> {
        Constant(&[50, 14, 219, 107, 78, 103, 16, 103]).read(reader)?;
        let _ = reader;
        Ok(CloseEmptyTree {})
    }
}
impl CloseEmptyTree {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* merkle_tree */ (true, Some(false), false),
                /* authority */ (false, Some(true), false),
                /* recipient */ (true, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = CloseEmptyTreeCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        CloseEmptyTreeCodec.decode(&instruction.data, limits)
    }
}
/// Program error codes. Unknown numeric codes remain lossless.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SplAccountCompressionError {
    /// The `IncorrectLeafLength` program error.
    IncorrectLeafLength,
    /// The `ConcurrentMerkleTreeError` program error.
    ConcurrentMerkleTreeError,
    /// The `ZeroCopyError` program error.
    ZeroCopyError,
    /// The `ConcurrentMerkleTreeConstantsError` program error.
    ConcurrentMerkleTreeConstantsError,
    /// The `CanopyLengthMismatch` program error.
    CanopyLengthMismatch,
    /// The `IncorrectAuthority` program error.
    IncorrectAuthority,
    /// The `IncorrectAccountOwner` program error.
    IncorrectAccountOwner,
    /// The `IncorrectAccountType` program error.
    IncorrectAccountType,
    /// The `LeafIndexOutOfBounds` program error.
    LeafIndexOutOfBounds,
    /// An unrecognized program error code.
    Unknown(u32),
}
impl SplAccountCompressionError {
    /// Decodes known or unknown numeric program error codes.
    pub const fn from_code(code: u32) -> Self {
        match code {
            PROGRAM_ERROR_6000 => Self::IncorrectLeafLength,
            PROGRAM_ERROR_6001 => Self::ConcurrentMerkleTreeError,
            PROGRAM_ERROR_6002 => Self::ZeroCopyError,
            PROGRAM_ERROR_6003 => Self::ConcurrentMerkleTreeConstantsError,
            PROGRAM_ERROR_6004 => Self::CanopyLengthMismatch,
            PROGRAM_ERROR_6005 => Self::IncorrectAuthority,
            PROGRAM_ERROR_6006 => Self::IncorrectAccountOwner,
            PROGRAM_ERROR_6007 => Self::IncorrectAccountType,
            PROGRAM_ERROR_6008 => Self::LeafIndexOutOfBounds,
            code => Self::Unknown(code),
        }
    }
    /// Returns the exact numeric code.
    pub const fn code(self) -> u32 {
        match self {
            Self::IncorrectLeafLength => PROGRAM_ERROR_6000,
            Self::ConcurrentMerkleTreeError => PROGRAM_ERROR_6001,
            Self::ZeroCopyError => PROGRAM_ERROR_6002,
            Self::ConcurrentMerkleTreeConstantsError => PROGRAM_ERROR_6003,
            Self::CanopyLengthMismatch => PROGRAM_ERROR_6004,
            Self::IncorrectAuthority => PROGRAM_ERROR_6005,
            Self::IncorrectAccountOwner => PROGRAM_ERROR_6006,
            Self::IncorrectAccountType => PROGRAM_ERROR_6007,
            Self::LeafIndexOutOfBounds => PROGRAM_ERROR_6008,
            Self::Unknown(code) => code,
        }
    }
    /// Returns the source message for known errors.
    pub const fn message(self) -> Option<&'static str> {
        match self {
            Self::IncorrectLeafLength => Some("Incorrect leaf length. Expected vec of 32 bytes"),
            Self::ConcurrentMerkleTreeError => Some("Concurrent merkle tree error"),
            Self::ZeroCopyError => Some("Issue zero copying concurrent merkle tree data"),
            Self::ConcurrentMerkleTreeConstantsError => {
                Some("An unsupported max depth or max buffer size constant was provided")
            }
            Self::CanopyLengthMismatch => {
                Some("Expected a different byte length for the merkle tree canopy")
            }
            Self::IncorrectAuthority => {
                Some("Provided authority does not match expected tree authority")
            }
            Self::IncorrectAccountOwner => Some(
                "Account is owned by a different program, expected it to be owned by this program",
            ),
            Self::IncorrectAccountType => Some("Account provided has incorrect account type"),
            Self::LeafIndexOutOfBounds => {
                Some("Leaf index of concurrent merkle tree is out of bounds")
            }
            Self::Unknown(_) => None,
        }
    }
}
/// All selected instruction data variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SplAccountCompressionInstructionData {
    /// Data for tag 191,11,119,7,180,107,220,110.
    InitEmptyMerkleTree(InitEmptyMerkleTree),
    /// Data for tag 204,165,76,100,73,147,0,128.
    ReplaceLeaf(ReplaceLeaf),
    /// Data for tag 48,169,76,72,229,180,55,161.
    TransferAuthority(TransferAuthority),
    /// Data for tag 124,220,22,223,104,10,250,224.
    VerifyLeaf(VerifyLeaf),
    /// Data for tag 149,120,18,222,236,225,88,203.
    Append(Append),
    /// Data for tag 6,42,50,190,51,109,178,168.
    InsertOrAppend(InsertOrAppend),
    /// Data for tag 50,14,219,107,78,103,16,103.
    CloseEmptyTree(CloseEmptyTree),
}
/// Strict bounded instruction dispatch codec.
#[derive(Clone, Copy, Debug)]
pub struct SplAccountCompressionInstructionDataCodec;
impl Encoder<SplAccountCompressionInstructionData> for SplAccountCompressionInstructionDataCodec {
    fn encoded_len(&self, value: &SplAccountCompressionInstructionData) -> Result<usize, Error> {
        match value {
            SplAccountCompressionInstructionData::InitEmptyMerkleTree(value) => {
                InitEmptyMerkleTreeCodec.encoded_len(value)
            }
            SplAccountCompressionInstructionData::ReplaceLeaf(value) => {
                ReplaceLeafCodec.encoded_len(value)
            }
            SplAccountCompressionInstructionData::TransferAuthority(value) => {
                TransferAuthorityCodec.encoded_len(value)
            }
            SplAccountCompressionInstructionData::VerifyLeaf(value) => {
                VerifyLeafCodec.encoded_len(value)
            }
            SplAccountCompressionInstructionData::Append(value) => AppendCodec.encoded_len(value),
            SplAccountCompressionInstructionData::InsertOrAppend(value) => {
                InsertOrAppendCodec.encoded_len(value)
            }
            SplAccountCompressionInstructionData::CloseEmptyTree(value) => {
                CloseEmptyTreeCodec.encoded_len(value)
            }
        }
    }
    fn write(
        &self,
        value: &SplAccountCompressionInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            SplAccountCompressionInstructionData::InitEmptyMerkleTree(value) => {
                InitEmptyMerkleTreeCodec.write(value, writer)
            }
            SplAccountCompressionInstructionData::ReplaceLeaf(value) => {
                ReplaceLeafCodec.write(value, writer)
            }
            SplAccountCompressionInstructionData::TransferAuthority(value) => {
                TransferAuthorityCodec.write(value, writer)
            }
            SplAccountCompressionInstructionData::VerifyLeaf(value) => {
                VerifyLeafCodec.write(value, writer)
            }
            SplAccountCompressionInstructionData::Append(value) => AppendCodec.write(value, writer),
            SplAccountCompressionInstructionData::InsertOrAppend(value) => {
                InsertOrAppendCodec.write(value, writer)
            }
            SplAccountCompressionInstructionData::CloseEmptyTree(value) => {
                CloseEmptyTreeCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<SplAccountCompressionInstructionData> for SplAccountCompressionInstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SplAccountCompressionInstructionData, Error> {
        let tag = reader.remaining();
        if tag.is_empty() {
            return Err(Error::Truncated);
        }

        match tag {
            [191, 11, 119, 7, 180, 107, 220, 110, ..] => {
                Ok(SplAccountCompressionInstructionData::InitEmptyMerkleTree(
                    InitEmptyMerkleTreeCodec.read(reader)?,
                ))
            }
            [204, 165, 76, 100, 73, 147, 0, 128, ..] => Ok(
                SplAccountCompressionInstructionData::ReplaceLeaf(ReplaceLeafCodec.read(reader)?),
            ),
            [48, 169, 76, 72, 229, 180, 55, 161, ..] => {
                Ok(SplAccountCompressionInstructionData::TransferAuthority(
                    TransferAuthorityCodec.read(reader)?,
                ))
            }
            [124, 220, 22, 223, 104, 10, 250, 224, ..] => Ok(
                SplAccountCompressionInstructionData::VerifyLeaf(VerifyLeafCodec.read(reader)?),
            ),
            [149, 120, 18, 222, 236, 225, 88, 203, ..] => Ok(
                SplAccountCompressionInstructionData::Append(AppendCodec.read(reader)?),
            ),
            [6, 42, 50, 190, 51, 109, 178, 168, ..] => {
                Ok(SplAccountCompressionInstructionData::InsertOrAppend(
                    InsertOrAppendCodec.read(reader)?,
                ))
            }
            [50, 14, 219, 107, 78, 103, 16, 103, ..] => {
                Ok(SplAccountCompressionInstructionData::CloseEmptyTree(
                    CloseEmptyTreeCodec.read(reader)?,
                ))
            }
            _ => Err(Error::InvalidDiscriminator),
        }
    }
}
/// Identifies a literal header without validating its payload.
pub fn identify_instruction(data: &[u8]) -> Result<&[u8], Error> {
    match data {
        [191, 11, 119, 7, 180, 107, 220, 110, ..] => Ok(&data[..8]),
        [204, 165, 76, 100, 73, 147, 0, 128, ..] => Ok(&data[..8]),
        [48, 169, 76, 72, 229, 180, 55, 161, ..] => Ok(&data[..8]),
        [124, 220, 22, 223, 104, 10, 250, 224, ..] => Ok(&data[..8]),
        [149, 120, 18, 222, 236, 225, 88, 203, ..] => Ok(&data[..8]),
        [6, 42, 50, 190, 51, 109, 178, 168, ..] => Ok(&data[..8]),
        [50, 14, 219, 107, 78, 103, 16, 103, ..] => Ok(&data[..8]),
        [] => Err(Error::Truncated),
        _ => Err(Error::InvalidDiscriminator),
    }
}
/// Strictly parses a canonical program invocation.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<SplAccountCompressionInstructionData, Error> {
    parse_instruction_with_program(PROGRAM, instruction, limits)
}
/// Strictly parses an explicit expected program identity.
pub fn parse_instruction_with_program(
    program: Address,
    instruction: &Instruction,
    limits: Limits,
) -> Result<SplAccountCompressionInstructionData, Error> {
    match identify_instruction(&instruction.data)? {
        [191, 11, 119, 7, 180, 107, 220, 110, ..] => {
            Ok(SplAccountCompressionInstructionData::InitEmptyMerkleTree(
                InitEmptyMerkleTree::parse_with_program(program, instruction, limits)?,
            ))
        }
        [204, 165, 76, 100, 73, 147, 0, 128, ..] => {
            Ok(SplAccountCompressionInstructionData::ReplaceLeaf(
                ReplaceLeaf::parse_with_program(program, instruction, limits)?,
            ))
        }
        [48, 169, 76, 72, 229, 180, 55, 161, ..] => {
            Ok(SplAccountCompressionInstructionData::TransferAuthority(
                TransferAuthority::parse_with_program(program, instruction, limits)?,
            ))
        }
        [124, 220, 22, 223, 104, 10, 250, 224, ..] => {
            Ok(SplAccountCompressionInstructionData::VerifyLeaf(
                VerifyLeaf::parse_with_program(program, instruction, limits)?,
            ))
        }
        [149, 120, 18, 222, 236, 225, 88, 203, ..] => {
            Ok(SplAccountCompressionInstructionData::Append(
                Append::parse_with_program(program, instruction, limits)?,
            ))
        }
        [6, 42, 50, 190, 51, 109, 178, 168, ..] => {
            Ok(SplAccountCompressionInstructionData::InsertOrAppend(
                InsertOrAppend::parse_with_program(program, instruction, limits)?,
            ))
        }
        [50, 14, 219, 107, 78, 103, 16, 103, ..] => {
            Ok(SplAccountCompressionInstructionData::CloseEmptyTree(
                CloseEmptyTree::parse_with_program(program, instruction, limits)?,
            ))
        }
        _ => Err(Error::InvalidDiscriminator),
    }
}
use solana_kit_core::AccountMeta;
use solana_kit_core::AccountRole;
use solana_kit_core::codecs::Counted;
use solana_kit_core::codecs::FixedBytes;
use solana_kit_core::codecs::Mapped;
use solana_kit_core::codecs::{Count, OwnedSequence, Sequence};
impl core::hash::Hash for ApplicationDataEventV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.application_data, state);
    }
}
impl core::fmt::Display for ApplicationDataEventV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ChangeLogEventV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.id, state);
        core::hash::Hash::hash(&self.path, state);
        core::hash::Hash::hash(&self.seq, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for ChangeLogEventV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConcurrentMerkleTreeHeader {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.account_type, state);
        core::hash::Hash::hash(&self.header, state);
    }
}
impl core::fmt::Display for ConcurrentMerkleTreeHeader {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConcurrentMerkleTreeHeaderDataV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.max_buffer_size, state);
        core::hash::Hash::hash(&self.max_depth, state);
        core::hash::Hash::hash(&self.authority, state);
        core::hash::Hash::hash(&self.creation_slot, state);
        core::hash::Hash::hash(&self.padding, state);
    }
}
impl core::fmt::Display for ConcurrentMerkleTreeHeaderDataV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PathNode {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.node, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for PathNode {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ApplicationDataEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for ApplicationDataEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ChangeLogEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for ChangeLogEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AccountCompressionEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::ChangeLog(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ApplicationData(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for AccountCompressionEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CompressionAccountType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Uninitialized => core::hash::Hash::hash(&0usize, state),
            Self::ConcurrentMerkleTree => core::hash::Hash::hash(&1usize, state),
        }
    }
}
impl core::fmt::Display for CompressionAccountType {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConcurrentMerkleTreeHeaderData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for ConcurrentMerkleTreeHeaderData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for InitEmptyMerkleTree {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.max_depth, state);
        core::hash::Hash::hash(&self.max_buffer_size, state);
    }
}
impl core::fmt::Display for InitEmptyMerkleTree {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ReplaceLeaf {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.previous_leaf, state);
        core::hash::Hash::hash(&self.new_leaf, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for ReplaceLeaf {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransferAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_authority, state);
    }
}
impl core::fmt::Display for TransferAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VerifyLeaf {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.leaf, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for VerifyLeaf {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Append {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.leaf, state);
    }
}
impl core::fmt::Display for Append {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for InsertOrAppend {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.leaf, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for InsertOrAppend {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CloseEmptyTree {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CloseEmptyTree {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SplAccountCompressionInstructionData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::InitEmptyMerkleTree(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ReplaceLeaf(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransferAuthority(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VerifyLeaf(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Append(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::InsertOrAppend(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CloseEmptyTree(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for SplAccountCompressionInstructionData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SplAccountCompressionError {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.code(), state)
    }
}
impl core::fmt::Display for SplAccountCompressionError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.message() {
            Some(message) => formatter.write_str(message),
            None => write!(formatter, "unknown program error {}", self.code()),
        }
    }
}
impl core::error::Error for SplAccountCompressionError {}
