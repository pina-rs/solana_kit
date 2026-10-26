// Generated from a validated Codama model. See README.md and upstream notices.
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Constant, Decoder, Encoder, Error, Limits, Number, Reader, Writer},
};
/// Selected trusted program identity.
pub const PROGRAM: Address = Address::from_bytes([
    15, 94, 158, 213, 55, 30, 44, 112, 137, 140, 169, 253, 14, 119, 192, 6, 92, 171, 93, 160, 46,
    86, 103, 139, 39, 19, 56, 42, 243, 116, 89, 183,
]);
const INSTRUCTION_TAG_0: u8 = 0;
const INSTRUCTION_TAG_1: u8 = 1;
const INSTRUCTION_TAG_2: u8 = 2;
const INSTRUCTION_TAG_3: u8 = 3;
const INSTRUCTION_TAG_4: u8 = 4;
const INSTRUCTION_TAG_5: u8 = 5;
const INSTRUCTION_TAG_6: u8 = 6;
const INSTRUCTION_TAG_7: u8 = 7;
const INSTRUCTION_TAG_9: u8 = 9;
const INSTRUCTION_TAG_10: u8 = 10;
const INSTRUCTION_TAG_11: u8 = 11;
const INSTRUCTION_TAG_228: u8 = 228;
const PROGRAM_ERROR_0: u32 = 0;
const PROGRAM_ERROR_1: u32 = 1;
const PROGRAM_ERROR_2: u32 = 2;
const PROGRAM_ERROR_3: u32 = 3;
const PROGRAM_ERROR_4: u32 = 4;
const PROGRAM_ERROR_5: u32 = 5;
const PROGRAM_ERROR_6: u32 = 6;
const PROGRAM_ERROR_7: u32 = 7;
const PROGRAM_ERROR_8: u32 = 8;
const PROGRAM_ERROR_9: u32 = 9;
const PROGRAM_ERROR_10: u32 = 10;
const PROGRAM_ERROR_11: u32 = 11;

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
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SchemaDataType {
    /// The `U8` variant.
    U8,
    /// The `U16` variant.
    U16,
    /// The `U32` variant.
    U32,
    /// The `U64` variant.
    U64,
    /// The `U128` variant.
    U128,
    /// The `I8` variant.
    I8,
    /// The `I16` variant.
    I16,
    /// The `I32` variant.
    I32,
    /// The `I64` variant.
    I64,
    /// The `I128` variant.
    I128,
    /// The `Bool` variant.
    Bool,
    /// The `Char` variant.
    Char,
    /// The `String` variant.
    String,
    /// The `VecU8` variant.
    VecU8,
    /// The `VecU16` variant.
    VecU16,
    /// The `VecU32` variant.
    VecU32,
    /// The `VecU64` variant.
    VecU64,
    /// The `VecU128` variant.
    VecU128,
    /// The `VecI8` variant.
    VecI8,
    /// The `VecI16` variant.
    VecI16,
    /// The `VecI32` variant.
    VecI32,
    /// The `VecI64` variant.
    VecI64,
    /// The `VecI128` variant.
    VecI128,
    /// The `VecBool` variant.
    VecBool,
    /// The `VecChar` variant.
    VecChar,
    /// The `VecString` variant.
    VecString,
}
/// Bounded codec for [`SchemaDataType`].
#[derive(Clone, Copy, Debug)]
pub struct SchemaDataTypeCodec;
const SCHEMADATATYPE_TAG_0: u8 = 0;
const SCHEMADATATYPE_TAG_1: u8 = 1;
const SCHEMADATATYPE_TAG_2: u8 = 2;
const SCHEMADATATYPE_TAG_3: u8 = 3;
const SCHEMADATATYPE_TAG_4: u8 = 4;
const SCHEMADATATYPE_TAG_5: u8 = 5;
const SCHEMADATATYPE_TAG_6: u8 = 6;
const SCHEMADATATYPE_TAG_7: u8 = 7;
const SCHEMADATATYPE_TAG_8: u8 = 8;
const SCHEMADATATYPE_TAG_9: u8 = 9;
const SCHEMADATATYPE_TAG_10: u8 = 10;
const SCHEMADATATYPE_TAG_11: u8 = 11;
const SCHEMADATATYPE_TAG_12: u8 = 12;
const SCHEMADATATYPE_TAG_13: u8 = 13;
const SCHEMADATATYPE_TAG_14: u8 = 14;
const SCHEMADATATYPE_TAG_15: u8 = 15;
const SCHEMADATATYPE_TAG_16: u8 = 16;
const SCHEMADATATYPE_TAG_17: u8 = 17;
const SCHEMADATATYPE_TAG_18: u8 = 18;
const SCHEMADATATYPE_TAG_19: u8 = 19;
const SCHEMADATATYPE_TAG_20: u8 = 20;
const SCHEMADATATYPE_TAG_21: u8 = 21;
const SCHEMADATATYPE_TAG_22: u8 = 22;
const SCHEMADATATYPE_TAG_23: u8 = 23;
const SCHEMADATATYPE_TAG_24: u8 = 24;
const SCHEMADATATYPE_TAG_25: u8 = 25;
impl Encoder<SchemaDataType> for SchemaDataTypeCodec {
    fn encoded_len(&self, value: &SchemaDataType) -> Result<usize, Error> {
        let size = match value {
            SchemaDataType::U8 => 0usize,
            SchemaDataType::U16 => 0usize,
            SchemaDataType::U32 => 0usize,
            SchemaDataType::U64 => 0usize,
            SchemaDataType::U128 => 0usize,
            SchemaDataType::I8 => 0usize,
            SchemaDataType::I16 => 0usize,
            SchemaDataType::I32 => 0usize,
            SchemaDataType::I64 => 0usize,
            SchemaDataType::I128 => 0usize,
            SchemaDataType::Bool => 0usize,
            SchemaDataType::Char => 0usize,
            SchemaDataType::String => 0usize,
            SchemaDataType::VecU8 => 0usize,
            SchemaDataType::VecU16 => 0usize,
            SchemaDataType::VecU32 => 0usize,
            SchemaDataType::VecU64 => 0usize,
            SchemaDataType::VecU128 => 0usize,
            SchemaDataType::VecI8 => 0usize,
            SchemaDataType::VecI16 => 0usize,
            SchemaDataType::VecI32 => 0usize,
            SchemaDataType::VecI64 => 0usize,
            SchemaDataType::VecI128 => 0usize,
            SchemaDataType::VecBool => 0usize,
            SchemaDataType::VecChar => 0usize,
            SchemaDataType::VecString => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &SchemaDataType, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            SchemaDataType::U8 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_0, writer)?;
                Ok(())
            }
            SchemaDataType::U16 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_1, writer)?;
                Ok(())
            }
            SchemaDataType::U32 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_2, writer)?;
                Ok(())
            }
            SchemaDataType::U64 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_3, writer)?;
                Ok(())
            }
            SchemaDataType::U128 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_4, writer)?;
                Ok(())
            }
            SchemaDataType::I8 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_5, writer)?;
                Ok(())
            }
            SchemaDataType::I16 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_6, writer)?;
                Ok(())
            }
            SchemaDataType::I32 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_7, writer)?;
                Ok(())
            }
            SchemaDataType::I64 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_8, writer)?;
                Ok(())
            }
            SchemaDataType::I128 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_9, writer)?;
                Ok(())
            }
            SchemaDataType::Bool => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_10, writer)?;
                Ok(())
            }
            SchemaDataType::Char => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_11, writer)?;
                Ok(())
            }
            SchemaDataType::String => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_12, writer)?;
                Ok(())
            }
            SchemaDataType::VecU8 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_13, writer)?;
                Ok(())
            }
            SchemaDataType::VecU16 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_14, writer)?;
                Ok(())
            }
            SchemaDataType::VecU32 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_15, writer)?;
                Ok(())
            }
            SchemaDataType::VecU64 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_16, writer)?;
                Ok(())
            }
            SchemaDataType::VecU128 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_17, writer)?;
                Ok(())
            }
            SchemaDataType::VecI8 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_18, writer)?;
                Ok(())
            }
            SchemaDataType::VecI16 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_19, writer)?;
                Ok(())
            }
            SchemaDataType::VecI32 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_20, writer)?;
                Ok(())
            }
            SchemaDataType::VecI64 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_21, writer)?;
                Ok(())
            }
            SchemaDataType::VecI128 => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_22, writer)?;
                Ok(())
            }
            SchemaDataType::VecBool => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_23, writer)?;
                Ok(())
            }
            SchemaDataType::VecChar => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_24, writer)?;
                Ok(())
            }
            SchemaDataType::VecString => {
                Number::<u8>::LE.write(&SCHEMADATATYPE_TAG_25, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<SchemaDataType> for SchemaDataTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SchemaDataType, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            SCHEMADATATYPE_TAG_0 => Ok(SchemaDataType::U8),
            SCHEMADATATYPE_TAG_1 => Ok(SchemaDataType::U16),
            SCHEMADATATYPE_TAG_2 => Ok(SchemaDataType::U32),
            SCHEMADATATYPE_TAG_3 => Ok(SchemaDataType::U64),
            SCHEMADATATYPE_TAG_4 => Ok(SchemaDataType::U128),
            SCHEMADATATYPE_TAG_5 => Ok(SchemaDataType::I8),
            SCHEMADATATYPE_TAG_6 => Ok(SchemaDataType::I16),
            SCHEMADATATYPE_TAG_7 => Ok(SchemaDataType::I32),
            SCHEMADATATYPE_TAG_8 => Ok(SchemaDataType::I64),
            SCHEMADATATYPE_TAG_9 => Ok(SchemaDataType::I128),
            SCHEMADATATYPE_TAG_10 => Ok(SchemaDataType::Bool),
            SCHEMADATATYPE_TAG_11 => Ok(SchemaDataType::Char),
            SCHEMADATATYPE_TAG_12 => Ok(SchemaDataType::String),
            SCHEMADATATYPE_TAG_13 => Ok(SchemaDataType::VecU8),
            SCHEMADATATYPE_TAG_14 => Ok(SchemaDataType::VecU16),
            SCHEMADATATYPE_TAG_15 => Ok(SchemaDataType::VecU32),
            SCHEMADATATYPE_TAG_16 => Ok(SchemaDataType::VecU64),
            SCHEMADATATYPE_TAG_17 => Ok(SchemaDataType::VecU128),
            SCHEMADATATYPE_TAG_18 => Ok(SchemaDataType::VecI8),
            SCHEMADATATYPE_TAG_19 => Ok(SchemaDataType::VecI16),
            SCHEMADATATYPE_TAG_20 => Ok(SchemaDataType::VecI32),
            SCHEMADATATYPE_TAG_21 => Ok(SchemaDataType::VecI64),
            SCHEMADATATYPE_TAG_22 => Ok(SchemaDataType::VecI128),
            SCHEMADATATYPE_TAG_23 => Ok(SchemaDataType::VecBool),
            SCHEMADATATYPE_TAG_24 => Ok(SchemaDataType::VecChar),
            SCHEMADATATYPE_TAG_25 => Ok(SchemaDataType::VecString),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `CloseAttestationEvent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseAttestationEvent {
    /// The `discriminator` wire field.
    pub discriminator: u8,
    /// The `schema` wire field.
    pub schema: Address,
    /// The `attestation_data` wire field.
    pub attestation_data: Vec<u8>,
}
/// Bounded codec for [`CloseAttestationEvent`].
#[derive(Clone, Copy, Debug)]
pub struct CloseAttestationEventCodec;
impl Encoder<CloseAttestationEvent> for CloseAttestationEventCodec {
    fn encoded_len(&self, value: &CloseAttestationEvent) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.discriminator)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.schema)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.attestation_data.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CloseAttestationEvent, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.discriminator, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.schema, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.attestation_data.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<CloseAttestationEvent> for CloseAttestationEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseAttestationEvent, Error> {
        Ok(CloseAttestationEvent {
            discriminator: Number::<u8>::LE.read(reader)?,
            schema: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            attestation_data: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `Credential`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Credential {
    /// The `authority` wire field.
    pub authority: Address,
    /// The `name` wire field.
    pub name: Vec<u8>,
    /// The `authorized_signers` wire field.
    pub authorized_signers: Vec<Address>,
}
/// Bounded codec for [`Credential`].
#[derive(Clone, Copy, Debug)]
pub struct CredentialCodec;
impl Encoder<Credential> for CredentialCodec {
    fn encoded_len(&self, value: &Credential) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[0],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.name.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    minimum: 32,
                }
                .encoded_len(value.authorized_signers.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Credential, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[0];
        let _record_start = writer.position();
        Affix {
            prefix: &[0],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.authority, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.name.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.authorized_signers.as_slice(), writer)?;
        let _record_bytes = writer
            .bytes()
            .get(_record_start..writer.position())
            .ok_or(Error::InvalidDiscriminator)?;
        if !_record_bytes.starts_with(_DISCRIMINATOR) {
            return Err(Error::InvalidDiscriminator);
        }
        Ok(())
    }
}
impl Decoder<Credential> for CredentialCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Credential, Error> {
        const _DISCRIMINATOR: &[u8] = &[0];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = Credential {
            authority: Affix {
                prefix: &[0],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            name: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            authorized_signers: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
        };
        let _record_bytes = reader
            .bytes()
            .get(_record_start..reader.position())
            .ok_or(Error::InvalidDiscriminator)?;
        if !_record_bytes.starts_with(_DISCRIMINATOR) {
            return Err(Error::InvalidDiscriminator);
        }
        Ok(_record_value)
    }
}
/// Validated data for `Schema`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schema {
    /// The `credential` wire field.
    pub credential: Address,
    /// The `name` wire field.
    pub name: Vec<u8>,
    /// The `description` wire field.
    pub description: Vec<u8>,
    /// The `layout` wire field.
    pub layout: Vec<u8>,
    /// The `field_names` wire field.
    pub field_names: Vec<u8>,
    /// The `is_paused` wire field.
    pub is_paused: bool,
    /// The `version` wire field.
    pub version: u8,
}
/// Bounded codec for [`Schema`].
#[derive(Clone, Copy, Debug)]
pub struct SchemaCodec;
impl Encoder<Schema> for SchemaCodec {
    fn encoded_len(&self, value: &Schema) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[1],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.credential)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.name.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.description.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.layout.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.field_names.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_paused)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.version)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Schema, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[1];
        let _record_start = writer.position();
        Affix {
            prefix: &[1],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.credential, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.name.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.description.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.layout.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.field_names.as_slice(), writer)?;
        Bool.write(&value.is_paused, writer)?;
        Number::<u8>::LE.write(&value.version, writer)?;
        let _record_bytes = writer
            .bytes()
            .get(_record_start..writer.position())
            .ok_or(Error::InvalidDiscriminator)?;
        if !_record_bytes.starts_with(_DISCRIMINATOR) {
            return Err(Error::InvalidDiscriminator);
        }
        Ok(())
    }
}
impl Decoder<Schema> for SchemaCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Schema, Error> {
        const _DISCRIMINATOR: &[u8] = &[1];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = Schema {
            credential: Affix {
                prefix: &[1],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            name: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            description: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            layout: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            field_names: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            is_paused: Bool.read(reader)?,
            version: Number::<u8>::LE.read(reader)?,
        };
        let _record_bytes = reader
            .bytes()
            .get(_record_start..reader.position())
            .ok_or(Error::InvalidDiscriminator)?;
        if !_record_bytes.starts_with(_DISCRIMINATOR) {
            return Err(Error::InvalidDiscriminator);
        }
        Ok(_record_value)
    }
}
/// Validated data for `Attestation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    /// The `nonce` wire field.
    pub nonce: Address,
    /// The `credential` wire field.
    pub credential: Address,
    /// The `schema` wire field.
    pub schema: Address,
    /// The `data` wire field.
    pub data: Vec<u8>,
    /// The `signer` wire field.
    pub signer: Address,
    /// The `expiry` wire field.
    pub expiry: i64,
    /// The `token_account` wire field.
    pub token_account: Address,
}
/// Bounded codec for [`Attestation`].
#[derive(Clone, Copy, Debug)]
pub struct AttestationCodec;
impl Encoder<Attestation> for AttestationCodec {
    fn encoded_len(&self, value: &Attestation) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[2],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.nonce)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.credential)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.schema)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.data.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.signer)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expiry)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.token_account)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Attestation, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[2];
        let _record_start = writer.position();
        Affix {
            prefix: &[2],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.nonce, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.credential, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.schema, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.data.as_slice(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.signer, writer)?;
        Number::<i64>::LE.write(&value.expiry, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.token_account, writer)?;
        let _record_bytes = writer
            .bytes()
            .get(_record_start..writer.position())
            .ok_or(Error::InvalidDiscriminator)?;
        if !_record_bytes.starts_with(_DISCRIMINATOR) {
            return Err(Error::InvalidDiscriminator);
        }
        Ok(())
    }
}
impl Decoder<Attestation> for AttestationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Attestation, Error> {
        const _DISCRIMINATOR: &[u8] = &[2];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = Attestation {
            nonce: Affix {
                prefix: &[2],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            credential: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            schema: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            data: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            signer: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            expiry: Number::<i64>::LE.read(reader)?,
            token_account: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        };
        let _record_bytes = reader
            .bytes()
            .get(_record_start..reader.position())
            .ok_or(Error::InvalidDiscriminator)?;
        if !_record_bytes.starts_with(_DISCRIMINATOR) {
            return Err(Error::InvalidDiscriminator);
        }
        Ok(_record_value)
    }
}
/// Validated data for `CreateCredential`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateCredential {
    /// The `name` wire field.
    pub name: String,
    /// The `signers` wire field.
    pub signers: Vec<Address>,
}
/// Bounded codec for [`CreateCredential`].
#[derive(Clone, Copy, Debug)]
pub struct CreateCredentialCodec;
impl Encoder<CreateCredential> for CreateCredentialCodec {
    fn encoded_len(&self, value: &CreateCredential) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.name.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    minimum: 32,
                }
                .encoded_len(value.signers.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateCredential, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_0]).write(&(), writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.name.as_str(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.signers.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<CreateCredential> for CreateCredentialCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateCredential, Error> {
        Constant(&[INSTRUCTION_TAG_0]).read(reader)?;
        Ok(CreateCredential {
            name: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            signers: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
        })
    }
}
impl CreateCredential {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* payer */ (true, Some(true), false),
                /* credential */ (true, Some(false), false),
                /* authority */ (false, Some(true), false),
                /* system_program */ (false, Some(false), false),
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
        let data = CreateCredentialCodec.encode(self, limits.max_bytes)?;
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
        CreateCredentialCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateSchema`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateSchema {
    /// The `name` wire field.
    pub name: String,
    /// The `description` wire field.
    pub description: String,
    /// The `layout` wire field.
    pub layout: Vec<SchemaDataType>,
    /// The `field_names` wire field.
    pub field_names: Vec<String>,
}
/// Bounded codec for [`CreateSchema`].
#[derive(Clone, Copy, Debug)]
pub struct CreateSchemaCodec;
impl Encoder<CreateSchema> for CreateSchemaCodec {
    fn encoded_len(&self, value: &CreateSchema) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.name.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.description.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: SchemaDataTypeCodec,
                    minimum: 1,
                }
                .encoded_len(value.layout.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: OwnedString(ByteSizePrefix {
                        item: Utf8,
                        prefix: Mapped::new(
                            Number::<u32>::LE,
                            |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                            |value: u32| Ok::<_, Error>(u64::from(value)),
                        ),
                        consumption: FrameConsumption::Exact,
                    }),
                    minimum: 4,
                }
                .encoded_len(value.field_names.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateSchema, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_1]).write(&(), writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.name.as_str(), writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.description.as_str(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: SchemaDataTypeCodec,
            minimum: 1,
        }
        .write(value.layout.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }),
            minimum: 4,
        }
        .write(value.field_names.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<CreateSchema> for CreateSchemaCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateSchema, Error> {
        Constant(&[INSTRUCTION_TAG_1]).read(reader)?;
        Ok(CreateSchema {
            name: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            description: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            layout: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: SchemaDataTypeCodec,
                minimum: 1,
            }
            .read(reader)?,
            field_names: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }),
                minimum: 4,
            }
            .read(reader)?,
        })
    }
}
impl CreateSchema {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 5 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), false),
                /* credential */ (false, Some(false), false),
                /* schema */ (true, Some(false), false),
                /* system_program */ (false, Some(false), false),
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
        let data = CreateSchemaCodec.encode(self, limits.max_bytes)?;
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
        CreateSchemaCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ChangeSchemaStatus`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeSchemaStatus {
    /// The `is_paused` wire field.
    pub is_paused: bool,
}
/// Bounded codec for [`ChangeSchemaStatus`].
#[derive(Clone, Copy, Debug)]
pub struct ChangeSchemaStatusCodec;
impl Encoder<ChangeSchemaStatus> for ChangeSchemaStatusCodec {
    fn encoded_len(&self, value: &ChangeSchemaStatus) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_paused)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ChangeSchemaStatus, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_2]).write(&(), writer)?;
        Bool.write(&value.is_paused, writer)?;
        Ok(())
    }
}
impl Decoder<ChangeSchemaStatus> for ChangeSchemaStatusCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ChangeSchemaStatus, Error> {
        Constant(&[INSTRUCTION_TAG_2]).read(reader)?;
        Ok(ChangeSchemaStatus {
            is_paused: Bool.read(reader)?,
        })
    }
}
impl ChangeSchemaStatus {
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
                /* authority */ (false, Some(true), false),
                /* credential */ (false, Some(false), false),
                /* schema */ (true, Some(false), false),
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
        let data = ChangeSchemaStatusCodec.encode(self, limits.max_bytes)?;
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
        ChangeSchemaStatusCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ChangeAuthorizedSigners`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeAuthorizedSigners {
    /// The `signers` wire field.
    pub signers: Vec<Address>,
}
/// Bounded codec for [`ChangeAuthorizedSigners`].
#[derive(Clone, Copy, Debug)]
pub struct ChangeAuthorizedSignersCodec;
impl Encoder<ChangeAuthorizedSigners> for ChangeAuthorizedSignersCodec {
    fn encoded_len(&self, value: &ChangeAuthorizedSigners) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    minimum: 32,
                }
                .encoded_len(value.signers.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ChangeAuthorizedSigners, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_3]).write(&(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.signers.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<ChangeAuthorizedSigners> for ChangeAuthorizedSignersCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ChangeAuthorizedSigners, Error> {
        Constant(&[INSTRUCTION_TAG_3]).read(reader)?;
        Ok(ChangeAuthorizedSigners {
            signers: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
        })
    }
}
impl ChangeAuthorizedSigners {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), false),
                /* credential */ (true, Some(false), false),
                /* system_program */ (false, Some(false), false),
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
        let data = ChangeAuthorizedSignersCodec.encode(self, limits.max_bytes)?;
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
        ChangeAuthorizedSignersCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ChangeSchemaDescription`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeSchemaDescription {
    /// The `description` wire field.
    pub description: String,
}
/// Bounded codec for [`ChangeSchemaDescription`].
#[derive(Clone, Copy, Debug)]
pub struct ChangeSchemaDescriptionCodec;
impl Encoder<ChangeSchemaDescription> for ChangeSchemaDescriptionCodec {
    fn encoded_len(&self, value: &ChangeSchemaDescription) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.description.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ChangeSchemaDescription, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_4]).write(&(), writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.description.as_str(), writer)?;
        Ok(())
    }
}
impl Decoder<ChangeSchemaDescription> for ChangeSchemaDescriptionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ChangeSchemaDescription, Error> {
        Constant(&[INSTRUCTION_TAG_4]).read(reader)?;
        Ok(ChangeSchemaDescription {
            description: ByteSizePrefix {
                item: Utf8,
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
impl ChangeSchemaDescription {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 5 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), false),
                /* credential */ (false, Some(false), false),
                /* schema */ (true, Some(false), false),
                /* system_program */ (false, Some(false), false),
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
        let data = ChangeSchemaDescriptionCodec.encode(self, limits.max_bytes)?;
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
        ChangeSchemaDescriptionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ChangeSchemaVersion`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeSchemaVersion {
    /// The `layout` wire field.
    pub layout: Vec<SchemaDataType>,
    /// The `field_names` wire field.
    pub field_names: Vec<String>,
}
/// Bounded codec for [`ChangeSchemaVersion`].
#[derive(Clone, Copy, Debug)]
pub struct ChangeSchemaVersionCodec;
impl Encoder<ChangeSchemaVersion> for ChangeSchemaVersionCodec {
    fn encoded_len(&self, value: &ChangeSchemaVersion) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: SchemaDataTypeCodec,
                    minimum: 1,
                }
                .encoded_len(value.layout.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: OwnedString(ByteSizePrefix {
                        item: Utf8,
                        prefix: Mapped::new(
                            Number::<u32>::LE,
                            |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                            |value: u32| Ok::<_, Error>(u64::from(value)),
                        ),
                        consumption: FrameConsumption::Exact,
                    }),
                    minimum: 4,
                }
                .encoded_len(value.field_names.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ChangeSchemaVersion, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_5]).write(&(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: SchemaDataTypeCodec,
            minimum: 1,
        }
        .write(value.layout.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }),
            minimum: 4,
        }
        .write(value.field_names.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<ChangeSchemaVersion> for ChangeSchemaVersionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ChangeSchemaVersion, Error> {
        Constant(&[INSTRUCTION_TAG_5]).read(reader)?;
        Ok(ChangeSchemaVersion {
            layout: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: SchemaDataTypeCodec,
                minimum: 1,
            }
            .read(reader)?,
            field_names: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }),
                minimum: 4,
            }
            .read(reader)?,
        })
    }
}
impl ChangeSchemaVersion {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 6 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), false),
                /* credential */ (false, Some(false), false),
                /* existing_schema */ (false, Some(false), false),
                /* new_schema */ (true, Some(false), false),
                /* system_program */ (false, Some(false), false),
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
        let data = ChangeSchemaVersionCodec.encode(self, limits.max_bytes)?;
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
        ChangeSchemaVersionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateAttestation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateAttestation {
    /// The `nonce` wire field.
    pub nonce: Address,
    /// The `data` wire field.
    pub data: Vec<u8>,
    /// The `expiry` wire field.
    pub expiry: i64,
}
/// Bounded codec for [`CreateAttestation`].
#[derive(Clone, Copy, Debug)]
pub struct CreateAttestationCodec;
impl Encoder<CreateAttestation> for CreateAttestationCodec {
    fn encoded_len(&self, value: &CreateAttestation) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.nonce)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.data.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expiry)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateAttestation, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_6]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.nonce, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.data.as_slice(), writer)?;
        Number::<i64>::LE.write(&value.expiry, writer)?;
        Ok(())
    }
}
impl Decoder<CreateAttestation> for CreateAttestationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateAttestation, Error> {
        Constant(&[INSTRUCTION_TAG_6]).read(reader)?;
        Ok(CreateAttestation {
            nonce: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            data: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            expiry: Number::<i64>::LE.read(reader)?,
        })
    }
}
impl CreateAttestation {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 6 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), false),
                /* credential */ (false, Some(false), false),
                /* schema */ (false, Some(false), false),
                /* attestation */ (true, Some(false), false),
                /* system_program */ (false, Some(false), false),
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
        let data = CreateAttestationCodec.encode(self, limits.max_bytes)?;
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
        CreateAttestationCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CloseAttestation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseAttestation {}
/// Bounded codec for [`CloseAttestation`].
#[derive(Clone, Copy, Debug)]
pub struct CloseAttestationCodec;
impl Encoder<CloseAttestation> for CloseAttestationCodec {
    fn encoded_len(&self, value: &CloseAttestation) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CloseAttestation, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_7]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CloseAttestation> for CloseAttestationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseAttestation, Error> {
        Constant(&[INSTRUCTION_TAG_7]).read(reader)?;
        let _ = reader;
        Ok(CloseAttestation {})
    }
}
impl CloseAttestation {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), false),
                /* credential */ (false, Some(false), false),
                /* attestation */ (true, Some(false), false),
                /* event_authority */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* attestation_program */ (false, Some(false), false),
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
        let data = CloseAttestationCodec.encode(self, limits.max_bytes)?;
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
        CloseAttestationCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TokenizeSchema`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenizeSchema {
    /// The `max_size` wire field.
    pub max_size: u64,
}
/// Bounded codec for [`TokenizeSchema`].
#[derive(Clone, Copy, Debug)]
pub struct TokenizeSchemaCodec;
impl Encoder<TokenizeSchema> for TokenizeSchemaCodec {
    fn encoded_len(&self, value: &TokenizeSchema) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.max_size)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TokenizeSchema, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_9]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.max_size, writer)?;
        Ok(())
    }
}
impl Decoder<TokenizeSchema> for TokenizeSchemaCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TokenizeSchema, Error> {
        Constant(&[INSTRUCTION_TAG_9]).read(reader)?;
        Ok(TokenizeSchema {
            max_size: Number::<u64>::LE.read(reader)?,
        })
    }
}
impl TokenizeSchema {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 8 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), false),
                /* credential */ (false, Some(false), false),
                /* schema */ (false, Some(false), false),
                /* mint */ (true, Some(false), false),
                /* sas_pda */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
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
        let data = TokenizeSchemaCodec.encode(self, limits.max_bytes)?;
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
        TokenizeSchemaCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateTokenizedAttestation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateTokenizedAttestation {
    /// The `nonce` wire field.
    pub nonce: Address,
    /// The `data` wire field.
    pub data: Vec<u8>,
    /// The `expiry` wire field.
    pub expiry: i64,
    /// The `name` wire field.
    pub name: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `symbol` wire field.
    pub symbol: String,
    /// The `mint_account_space` wire field.
    pub mint_account_space: u16,
}
/// Bounded codec for [`CreateTokenizedAttestation`].
#[derive(Clone, Copy, Debug)]
pub struct CreateTokenizedAttestationCodec;
impl Encoder<CreateTokenizedAttestation> for CreateTokenizedAttestationCodec {
    fn encoded_len(&self, value: &CreateTokenizedAttestation) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.nonce)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.data.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expiry)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.name.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.uri.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.symbol.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.mint_account_space)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &CreateTokenizedAttestation,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_10]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.nonce, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.data.as_slice(), writer)?;
        Number::<i64>::LE.write(&value.expiry, writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.name.as_str(), writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.uri.as_str(), writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.symbol.as_str(), writer)?;
        Number::<u16>::LE.write(&value.mint_account_space, writer)?;
        Ok(())
    }
}
impl Decoder<CreateTokenizedAttestation> for CreateTokenizedAttestationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateTokenizedAttestation, Error> {
        Constant(&[INSTRUCTION_TAG_10]).read(reader)?;
        Ok(CreateTokenizedAttestation {
            nonce: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            data: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            expiry: Number::<i64>::LE.read(reader)?,
            name: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            uri: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            symbol: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            mint_account_space: Number::<u16>::LE.read(reader)?,
        })
    }
}
impl CreateTokenizedAttestation {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 13 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), false),
                /* credential */ (false, Some(false), false),
                /* schema */ (false, Some(false), false),
                /* attestation */ (true, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* schema_mint */ (true, Some(false), false),
                /* attestation_mint */ (true, Some(false), false),
                /* sas_pda */ (false, Some(false), false),
                /* recipient_token_account */ (true, Some(false), false),
                /* recipient */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* associated_token_program */ (false, Some(false), false),
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
        let data = CreateTokenizedAttestationCodec.encode(self, limits.max_bytes)?;
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
        CreateTokenizedAttestationCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CloseTokenizedAttestation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseTokenizedAttestation {}
/// Bounded codec for [`CloseTokenizedAttestation`].
#[derive(Clone, Copy, Debug)]
pub struct CloseTokenizedAttestationCodec;
impl Encoder<CloseTokenizedAttestation> for CloseTokenizedAttestationCodec {
    fn encoded_len(&self, value: &CloseTokenizedAttestation) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &CloseTokenizedAttestation,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_11]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CloseTokenizedAttestation> for CloseTokenizedAttestationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseTokenizedAttestation, Error> {
        Constant(&[INSTRUCTION_TAG_11]).read(reader)?;
        let _ = reader;
        Ok(CloseTokenizedAttestation {})
    }
}
impl CloseTokenizedAttestation {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 11 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), false),
                /* credential */ (false, Some(false), false),
                /* attestation */ (true, Some(false), false),
                /* event_authority */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* attestation_program */ (false, Some(false), false),
                /* attestation_mint */ (true, Some(false), false),
                /* sas_pda */ (false, Some(false), false),
                /* attestation_token_account */ (true, Some(false), false),
                /* token_program */ (false, Some(false), false),
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
        let data = CloseTokenizedAttestationCodec.encode(self, limits.max_bytes)?;
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
        CloseTokenizedAttestationCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `EmitEvent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmitEvent {}
/// Bounded codec for [`EmitEvent`].
#[derive(Clone, Copy, Debug)]
pub struct EmitEventCodec;
impl Encoder<EmitEvent> for EmitEventCodec {
    fn encoded_len(&self, value: &EmitEvent) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &EmitEvent, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_228]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<EmitEvent> for EmitEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EmitEvent, Error> {
        Constant(&[INSTRUCTION_TAG_228]).read(reader)?;
        let _ = reader;
        Ok(EmitEvent {})
    }
}
impl EmitEvent {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 1 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([/* event_authority */ (false, Some(true), false)])
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
        let data = EmitEventCodec.encode(self, limits.max_bytes)?;
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
        EmitEventCodec.decode(&instruction.data, limits)
    }
}
/// Program error codes. Unknown numeric codes remain lossless.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttestationServiceError {
    /// The `InvalidCredential` program error.
    InvalidCredential,
    /// The `InvalidSchema` program error.
    InvalidSchema,
    /// The `InvalidAttestation` program error.
    InvalidAttestation,
    /// The `InvalidAuthority` program error.
    InvalidAuthority,
    /// The `InvalidSchemaDataType` program error.
    InvalidSchemaDataType,
    /// The `SignerNotAuthorized` program error.
    SignerNotAuthorized,
    /// The `InvalidAttestationData` program error.
    InvalidAttestationData,
    /// The `InvalidEventAuthority` program error.
    InvalidEventAuthority,
    /// The `InvalidMint` program error.
    InvalidMint,
    /// The `InvalidProgramSigner` program error.
    InvalidProgramSigner,
    /// The `InvalidTokenAccount` program error.
    InvalidTokenAccount,
    /// The `SchemaPaused` program error.
    SchemaPaused,
    /// An unrecognized program error code.
    Unknown(u32),
}
impl AttestationServiceError {
    /// Decodes known or unknown numeric program error codes.
    pub const fn from_code(code: u32) -> Self {
        match code {
            PROGRAM_ERROR_0 => Self::InvalidCredential,
            PROGRAM_ERROR_1 => Self::InvalidSchema,
            PROGRAM_ERROR_2 => Self::InvalidAttestation,
            PROGRAM_ERROR_3 => Self::InvalidAuthority,
            PROGRAM_ERROR_4 => Self::InvalidSchemaDataType,
            PROGRAM_ERROR_5 => Self::SignerNotAuthorized,
            PROGRAM_ERROR_6 => Self::InvalidAttestationData,
            PROGRAM_ERROR_7 => Self::InvalidEventAuthority,
            PROGRAM_ERROR_8 => Self::InvalidMint,
            PROGRAM_ERROR_9 => Self::InvalidProgramSigner,
            PROGRAM_ERROR_10 => Self::InvalidTokenAccount,
            PROGRAM_ERROR_11 => Self::SchemaPaused,
            code => Self::Unknown(code),
        }
    }
    /// Returns the exact numeric code.
    pub const fn code(self) -> u32 {
        match self {
            Self::InvalidCredential => PROGRAM_ERROR_0,
            Self::InvalidSchema => PROGRAM_ERROR_1,
            Self::InvalidAttestation => PROGRAM_ERROR_2,
            Self::InvalidAuthority => PROGRAM_ERROR_3,
            Self::InvalidSchemaDataType => PROGRAM_ERROR_4,
            Self::SignerNotAuthorized => PROGRAM_ERROR_5,
            Self::InvalidAttestationData => PROGRAM_ERROR_6,
            Self::InvalidEventAuthority => PROGRAM_ERROR_7,
            Self::InvalidMint => PROGRAM_ERROR_8,
            Self::InvalidProgramSigner => PROGRAM_ERROR_9,
            Self::InvalidTokenAccount => PROGRAM_ERROR_10,
            Self::SchemaPaused => PROGRAM_ERROR_11,
            Self::Unknown(code) => code,
        }
    }
    /// Returns the source message for known errors.
    pub const fn message(self) -> Option<&'static str> {
        match self {
            Self::InvalidCredential => Some("InvalidCredential"),
            Self::InvalidSchema => Some("InvalidSchema"),
            Self::InvalidAttestation => Some("InvalidAttestation"),
            Self::InvalidAuthority => Some("InvalidAuthority"),
            Self::InvalidSchemaDataType => Some("InvalidSchemaDataType"),
            Self::SignerNotAuthorized => Some("SignerNotAuthorized"),
            Self::InvalidAttestationData => Some("InvalidAttestationData"),
            Self::InvalidEventAuthority => Some("InvalidEventAuthority"),
            Self::InvalidMint => Some("InvalidMint"),
            Self::InvalidProgramSigner => Some("InvalidProgramSigner"),
            Self::InvalidTokenAccount => Some("InvalidTokenAccount"),
            Self::SchemaPaused => Some("SchemaPaused"),
            Self::Unknown(_) => None,
        }
    }
}
/// All selected instruction data variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttestationServiceInstructionData {
    /// Data for tag INSTRUCTION_TAG_0.
    CreateCredential(CreateCredential),
    /// Data for tag INSTRUCTION_TAG_1.
    CreateSchema(CreateSchema),
    /// Data for tag INSTRUCTION_TAG_2.
    ChangeSchemaStatus(ChangeSchemaStatus),
    /// Data for tag INSTRUCTION_TAG_3.
    ChangeAuthorizedSigners(ChangeAuthorizedSigners),
    /// Data for tag INSTRUCTION_TAG_4.
    ChangeSchemaDescription(ChangeSchemaDescription),
    /// Data for tag INSTRUCTION_TAG_5.
    ChangeSchemaVersion(ChangeSchemaVersion),
    /// Data for tag INSTRUCTION_TAG_6.
    CreateAttestation(CreateAttestation),
    /// Data for tag INSTRUCTION_TAG_7.
    CloseAttestation(CloseAttestation),
    /// Data for tag INSTRUCTION_TAG_9.
    TokenizeSchema(TokenizeSchema),
    /// Data for tag INSTRUCTION_TAG_10.
    CreateTokenizedAttestation(CreateTokenizedAttestation),
    /// Data for tag INSTRUCTION_TAG_11.
    CloseTokenizedAttestation(CloseTokenizedAttestation),
    /// Data for tag INSTRUCTION_TAG_228.
    EmitEvent(EmitEvent),
}
/// Strict bounded instruction dispatch codec.
#[derive(Clone, Copy, Debug)]
pub struct AttestationServiceInstructionDataCodec;
impl Encoder<AttestationServiceInstructionData> for AttestationServiceInstructionDataCodec {
    fn encoded_len(&self, value: &AttestationServiceInstructionData) -> Result<usize, Error> {
        match value {
            AttestationServiceInstructionData::CreateCredential(value) => {
                CreateCredentialCodec.encoded_len(value)
            }
            AttestationServiceInstructionData::CreateSchema(value) => {
                CreateSchemaCodec.encoded_len(value)
            }
            AttestationServiceInstructionData::ChangeSchemaStatus(value) => {
                ChangeSchemaStatusCodec.encoded_len(value)
            }
            AttestationServiceInstructionData::ChangeAuthorizedSigners(value) => {
                ChangeAuthorizedSignersCodec.encoded_len(value)
            }
            AttestationServiceInstructionData::ChangeSchemaDescription(value) => {
                ChangeSchemaDescriptionCodec.encoded_len(value)
            }
            AttestationServiceInstructionData::ChangeSchemaVersion(value) => {
                ChangeSchemaVersionCodec.encoded_len(value)
            }
            AttestationServiceInstructionData::CreateAttestation(value) => {
                CreateAttestationCodec.encoded_len(value)
            }
            AttestationServiceInstructionData::CloseAttestation(value) => {
                CloseAttestationCodec.encoded_len(value)
            }
            AttestationServiceInstructionData::TokenizeSchema(value) => {
                TokenizeSchemaCodec.encoded_len(value)
            }
            AttestationServiceInstructionData::CreateTokenizedAttestation(value) => {
                CreateTokenizedAttestationCodec.encoded_len(value)
            }
            AttestationServiceInstructionData::CloseTokenizedAttestation(value) => {
                CloseTokenizedAttestationCodec.encoded_len(value)
            }
            AttestationServiceInstructionData::EmitEvent(value) => {
                EmitEventCodec.encoded_len(value)
            }
        }
    }
    fn write(
        &self,
        value: &AttestationServiceInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            AttestationServiceInstructionData::CreateCredential(value) => {
                CreateCredentialCodec.write(value, writer)
            }
            AttestationServiceInstructionData::CreateSchema(value) => {
                CreateSchemaCodec.write(value, writer)
            }
            AttestationServiceInstructionData::ChangeSchemaStatus(value) => {
                ChangeSchemaStatusCodec.write(value, writer)
            }
            AttestationServiceInstructionData::ChangeAuthorizedSigners(value) => {
                ChangeAuthorizedSignersCodec.write(value, writer)
            }
            AttestationServiceInstructionData::ChangeSchemaDescription(value) => {
                ChangeSchemaDescriptionCodec.write(value, writer)
            }
            AttestationServiceInstructionData::ChangeSchemaVersion(value) => {
                ChangeSchemaVersionCodec.write(value, writer)
            }
            AttestationServiceInstructionData::CreateAttestation(value) => {
                CreateAttestationCodec.write(value, writer)
            }
            AttestationServiceInstructionData::CloseAttestation(value) => {
                CloseAttestationCodec.write(value, writer)
            }
            AttestationServiceInstructionData::TokenizeSchema(value) => {
                TokenizeSchemaCodec.write(value, writer)
            }
            AttestationServiceInstructionData::CreateTokenizedAttestation(value) => {
                CreateTokenizedAttestationCodec.write(value, writer)
            }
            AttestationServiceInstructionData::CloseTokenizedAttestation(value) => {
                CloseTokenizedAttestationCodec.write(value, writer)
            }
            AttestationServiceInstructionData::EmitEvent(value) => {
                EmitEventCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<AttestationServiceInstructionData> for AttestationServiceInstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AttestationServiceInstructionData, Error> {
        let tag = *reader.remaining().first().ok_or(Error::Truncated)?;

        match tag {
            INSTRUCTION_TAG_0 => Ok(AttestationServiceInstructionData::CreateCredential(
                CreateCredentialCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_1 => Ok(AttestationServiceInstructionData::CreateSchema(
                CreateSchemaCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_2 => Ok(AttestationServiceInstructionData::ChangeSchemaStatus(
                ChangeSchemaStatusCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_3 => Ok(AttestationServiceInstructionData::ChangeAuthorizedSigners(
                ChangeAuthorizedSignersCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_4 => Ok(AttestationServiceInstructionData::ChangeSchemaDescription(
                ChangeSchemaDescriptionCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_5 => Ok(AttestationServiceInstructionData::ChangeSchemaVersion(
                ChangeSchemaVersionCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_6 => Ok(AttestationServiceInstructionData::CreateAttestation(
                CreateAttestationCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_7 => Ok(AttestationServiceInstructionData::CloseAttestation(
                CloseAttestationCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_9 => Ok(AttestationServiceInstructionData::TokenizeSchema(
                TokenizeSchemaCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_10 => Ok(
                AttestationServiceInstructionData::CreateTokenizedAttestation(
                    CreateTokenizedAttestationCodec.read(reader)?,
                ),
            ),
            INSTRUCTION_TAG_11 => Ok(
                AttestationServiceInstructionData::CloseTokenizedAttestation(
                    CloseTokenizedAttestationCodec.read(reader)?,
                ),
            ),
            INSTRUCTION_TAG_228 => Ok(AttestationServiceInstructionData::EmitEvent(
                EmitEventCodec.read(reader)?,
            )),
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
        INSTRUCTION_TAG_5 => Ok(tag),
        INSTRUCTION_TAG_6 => Ok(tag),
        INSTRUCTION_TAG_7 => Ok(tag),
        INSTRUCTION_TAG_9 => Ok(tag),
        INSTRUCTION_TAG_10 => Ok(tag),
        INSTRUCTION_TAG_11 => Ok(tag),
        INSTRUCTION_TAG_228 => Ok(tag),
        _ => Err(Error::InvalidDiscriminator),
    }
}
/// Strictly parses a canonical program invocation.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<AttestationServiceInstructionData, Error> {
    parse_instruction_with_program(PROGRAM, instruction, limits)
}
/// Strictly parses an explicit expected program identity.
pub fn parse_instruction_with_program(
    program: Address,
    instruction: &Instruction,
    limits: Limits,
) -> Result<AttestationServiceInstructionData, Error> {
    match identify_instruction(&instruction.data)? {
        INSTRUCTION_TAG_0 => Ok(AttestationServiceInstructionData::CreateCredential(
            CreateCredential::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_1 => Ok(AttestationServiceInstructionData::CreateSchema(
            CreateSchema::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_2 => Ok(AttestationServiceInstructionData::ChangeSchemaStatus(
            ChangeSchemaStatus::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_3 => Ok(AttestationServiceInstructionData::ChangeAuthorizedSigners(
            ChangeAuthorizedSigners::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_4 => Ok(AttestationServiceInstructionData::ChangeSchemaDescription(
            ChangeSchemaDescription::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_5 => Ok(AttestationServiceInstructionData::ChangeSchemaVersion(
            ChangeSchemaVersion::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_6 => Ok(AttestationServiceInstructionData::CreateAttestation(
            CreateAttestation::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_7 => Ok(AttestationServiceInstructionData::CloseAttestation(
            CloseAttestation::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_9 => Ok(AttestationServiceInstructionData::TokenizeSchema(
            TokenizeSchema::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_10 => Ok(
            AttestationServiceInstructionData::CreateTokenizedAttestation(
                CreateTokenizedAttestation::parse_with_program(program, instruction, limits)?,
            ),
        ),
        INSTRUCTION_TAG_11 => Ok(
            AttestationServiceInstructionData::CloseTokenizedAttestation(
                CloseTokenizedAttestation::parse_with_program(program, instruction, limits)?,
            ),
        ),
        INSTRUCTION_TAG_228 => Ok(AttestationServiceInstructionData::EmitEvent(
            EmitEvent::parse_with_program(program, instruction, limits)?,
        )),
        _ => Err(Error::InvalidDiscriminator),
    }
}
struct OwnedString<C>(C);
impl<C: Encoder<str>> Encoder<String> for OwnedString<C> {
    fn encoded_len(&self, value: &String) -> Result<usize, Error> {
        self.0.encoded_len(value.as_str())
    }
    fn write(&self, value: &String, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write(value.as_str(), writer)
    }
}
impl<C: Decoder<String>> Decoder<String> for OwnedString<C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<String, Error> {
        self.0.read(reader)
    }
}
use alloc::string::String;
use solana_kit_core::AccountMeta;
use solana_kit_core::AccountRole;
use solana_kit_core::codecs::Affix;
use solana_kit_core::codecs::Bool;
use solana_kit_core::codecs::Counted;
use solana_kit_core::codecs::FixedBytes;
use solana_kit_core::codecs::Mapped;
use solana_kit_core::codecs::Utf8;
use solana_kit_core::codecs::{ByteSizePrefix, FrameConsumption};
impl core::hash::Hash for SchemaDataType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::U8 => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::U16 => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::U32 => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::U64 => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::U128 => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::I8 => {
                core::hash::Hash::hash(&5usize, state);
            }
            Self::I16 => {
                core::hash::Hash::hash(&6usize, state);
            }
            Self::I32 => {
                core::hash::Hash::hash(&7usize, state);
            }
            Self::I64 => {
                core::hash::Hash::hash(&8usize, state);
            }
            Self::I128 => {
                core::hash::Hash::hash(&9usize, state);
            }
            Self::Bool => {
                core::hash::Hash::hash(&10usize, state);
            }
            Self::Char => {
                core::hash::Hash::hash(&11usize, state);
            }
            Self::String => {
                core::hash::Hash::hash(&12usize, state);
            }
            Self::VecU8 => {
                core::hash::Hash::hash(&13usize, state);
            }
            Self::VecU16 => {
                core::hash::Hash::hash(&14usize, state);
            }
            Self::VecU32 => {
                core::hash::Hash::hash(&15usize, state);
            }
            Self::VecU64 => {
                core::hash::Hash::hash(&16usize, state);
            }
            Self::VecU128 => {
                core::hash::Hash::hash(&17usize, state);
            }
            Self::VecI8 => {
                core::hash::Hash::hash(&18usize, state);
            }
            Self::VecI16 => {
                core::hash::Hash::hash(&19usize, state);
            }
            Self::VecI32 => {
                core::hash::Hash::hash(&20usize, state);
            }
            Self::VecI64 => {
                core::hash::Hash::hash(&21usize, state);
            }
            Self::VecI128 => {
                core::hash::Hash::hash(&22usize, state);
            }
            Self::VecBool => {
                core::hash::Hash::hash(&23usize, state);
            }
            Self::VecChar => {
                core::hash::Hash::hash(&24usize, state);
            }
            Self::VecString => {
                core::hash::Hash::hash(&25usize, state);
            }
        }
    }
}
impl core::fmt::Display for SchemaDataType {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CloseAttestationEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.discriminator, state);
        core::hash::Hash::hash(&self.schema, state);
        core::hash::Hash::hash(&self.attestation_data, state);
    }
}
impl core::fmt::Display for CloseAttestationEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Credential {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authority, state);
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.authorized_signers, state);
    }
}
impl core::fmt::Display for Credential {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Schema {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.credential, state);
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.description, state);
        core::hash::Hash::hash(&self.layout, state);
        core::hash::Hash::hash(&self.field_names, state);
        core::hash::Hash::hash(&self.is_paused, state);
        core::hash::Hash::hash(&self.version, state);
    }
}
impl core::fmt::Display for Schema {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Attestation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.credential, state);
        core::hash::Hash::hash(&self.schema, state);
        core::hash::Hash::hash(&self.data, state);
        core::hash::Hash::hash(&self.signer, state);
        core::hash::Hash::hash(&self.expiry, state);
        core::hash::Hash::hash(&self.token_account, state);
    }
}
impl core::fmt::Display for Attestation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateCredential {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.signers, state);
    }
}
impl core::fmt::Display for CreateCredential {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateSchema {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.description, state);
        core::hash::Hash::hash(&self.layout, state);
        core::hash::Hash::hash(&self.field_names, state);
    }
}
impl core::fmt::Display for CreateSchema {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ChangeSchemaStatus {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.is_paused, state);
    }
}
impl core::fmt::Display for ChangeSchemaStatus {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ChangeAuthorizedSigners {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.signers, state);
    }
}
impl core::fmt::Display for ChangeAuthorizedSigners {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ChangeSchemaDescription {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.description, state);
    }
}
impl core::fmt::Display for ChangeSchemaDescription {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ChangeSchemaVersion {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.layout, state);
        core::hash::Hash::hash(&self.field_names, state);
    }
}
impl core::fmt::Display for ChangeSchemaVersion {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateAttestation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.data, state);
        core::hash::Hash::hash(&self.expiry, state);
    }
}
impl core::fmt::Display for CreateAttestation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CloseAttestation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CloseAttestation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TokenizeSchema {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.max_size, state);
    }
}
impl core::fmt::Display for TokenizeSchema {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateTokenizedAttestation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.data, state);
        core::hash::Hash::hash(&self.expiry, state);
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.symbol, state);
        core::hash::Hash::hash(&self.mint_account_space, state);
    }
}
impl core::fmt::Display for CreateTokenizedAttestation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CloseTokenizedAttestation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CloseTokenizedAttestation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for EmitEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for EmitEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AttestationServiceInstructionData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::CreateCredential(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateSchema(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ChangeSchemaStatus(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ChangeAuthorizedSigners(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ChangeSchemaDescription(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ChangeSchemaVersion(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateAttestation(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CloseAttestation(value) => {
                core::hash::Hash::hash(&7usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TokenizeSchema(value) => {
                core::hash::Hash::hash(&8usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateTokenizedAttestation(value) => {
                core::hash::Hash::hash(&9usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CloseTokenizedAttestation(value) => {
                core::hash::Hash::hash(&10usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::EmitEvent(value) => {
                core::hash::Hash::hash(&11usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for AttestationServiceInstructionData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AttestationServiceError {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.code(), state)
    }
}
impl core::fmt::Display for AttestationServiceError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.message() {
            Some(message) => formatter.write_str(message),
            None => write!(formatter, "unknown program error {}", self.code()),
        }
    }
}
impl core::error::Error for AttestationServiceError {}
