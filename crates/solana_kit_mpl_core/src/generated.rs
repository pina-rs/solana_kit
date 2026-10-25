// Generated from a validated Codama model. See README.md and upstream notices.
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Constant, Decoder, Encoder, Error, Limits, Number, Reader, Writer},
};
/// Selected trusted program identity.
pub const PROGRAM: Address = Address::from_bytes([
    175, 84, 171, 16, 189, 151, 165, 66, 160, 158, 247, 179, 152, 137, 221, 12, 211, 148, 164, 204,
    233, 223, 166, 205, 201, 126, 190, 45, 35, 91, 167, 72,
]);
const INSTRUCTION_TAG_0: u8 = 0;
const INSTRUCTION_TAG_1: u8 = 1;
const INSTRUCTION_TAG_2: u8 = 2;
const INSTRUCTION_TAG_3: u8 = 3;
const INSTRUCTION_TAG_4: u8 = 4;
const INSTRUCTION_TAG_5: u8 = 5;
const INSTRUCTION_TAG_6: u8 = 6;
const INSTRUCTION_TAG_7: u8 = 7;
const INSTRUCTION_TAG_8: u8 = 8;
const INSTRUCTION_TAG_9: u8 = 9;
const INSTRUCTION_TAG_10: u8 = 10;
const INSTRUCTION_TAG_11: u8 = 11;
const INSTRUCTION_TAG_12: u8 = 12;
const INSTRUCTION_TAG_13: u8 = 13;
const INSTRUCTION_TAG_14: u8 = 14;
const INSTRUCTION_TAG_15: u8 = 15;
const INSTRUCTION_TAG_16: u8 = 16;
const INSTRUCTION_TAG_17: u8 = 17;
const INSTRUCTION_TAG_18: u8 = 18;
const INSTRUCTION_TAG_19: u8 = 19;
const INSTRUCTION_TAG_20: u8 = 20;
const INSTRUCTION_TAG_21: u8 = 21;
const INSTRUCTION_TAG_22: u8 = 22;
const INSTRUCTION_TAG_23: u8 = 23;
const INSTRUCTION_TAG_24: u8 = 24;
const INSTRUCTION_TAG_25: u8 = 25;
const INSTRUCTION_TAG_26: u8 = 26;
const INSTRUCTION_TAG_27: u8 = 27;
const INSTRUCTION_TAG_28: u8 = 28;
const INSTRUCTION_TAG_29: u8 = 29;
const INSTRUCTION_TAG_30: u8 = 30;
const INSTRUCTION_TAG_31: u8 = 31;
const INSTRUCTION_TAG_32: u8 = 32;
const INSTRUCTION_TAG_33: u8 = 33;
const INSTRUCTION_TAG_34: u8 = 34;
const INSTRUCTION_TAG_35: u8 = 35;
const INSTRUCTION_TAG_36: u8 = 36;
const INSTRUCTION_TAG_37: u8 = 37;
const INSTRUCTION_TAG_38: u8 = 38;
const INSTRUCTION_TAG_39: u8 = 39;
const INSTRUCTION_TAG_40: u8 = 40;
const INSTRUCTION_TAG_41: u8 = 41;
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
const PROGRAM_ERROR_12: u32 = 12;
const PROGRAM_ERROR_13: u32 = 13;
const PROGRAM_ERROR_14: u32 = 14;
const PROGRAM_ERROR_15: u32 = 15;
const PROGRAM_ERROR_16: u32 = 16;
const PROGRAM_ERROR_17: u32 = 17;
const PROGRAM_ERROR_18: u32 = 18;
const PROGRAM_ERROR_19: u32 = 19;
const PROGRAM_ERROR_20: u32 = 20;
const PROGRAM_ERROR_21: u32 = 21;
const PROGRAM_ERROR_22: u32 = 22;
const PROGRAM_ERROR_23: u32 = 23;
const PROGRAM_ERROR_24: u32 = 24;
const PROGRAM_ERROR_25: u32 = 25;
const PROGRAM_ERROR_26: u32 = 26;
const PROGRAM_ERROR_27: u32 = 27;
const PROGRAM_ERROR_28: u32 = 28;
const PROGRAM_ERROR_29: u32 = 29;
const PROGRAM_ERROR_30: u32 = 30;
const PROGRAM_ERROR_31: u32 = 31;
const PROGRAM_ERROR_32: u32 = 32;
const PROGRAM_ERROR_33: u32 = 33;
const PROGRAM_ERROR_34: u32 = 34;
const PROGRAM_ERROR_35: u32 = 35;
const PROGRAM_ERROR_36: u32 = 36;
const PROGRAM_ERROR_37: u32 = 37;
const PROGRAM_ERROR_38: u32 = 38;
const PROGRAM_ERROR_39: u32 = 39;
const PROGRAM_ERROR_40: u32 = 40;
const PROGRAM_ERROR_41: u32 = 41;
const PROGRAM_ERROR_42: u32 = 42;
const PROGRAM_ERROR_43: u32 = 43;
const PROGRAM_ERROR_44: u32 = 44;
const PROGRAM_ERROR_45: u32 = 45;
const PROGRAM_ERROR_46: u32 = 46;
const PROGRAM_ERROR_47: u32 = 47;
const PROGRAM_ERROR_48: u32 = 48;
const PROGRAM_ERROR_49: u32 = 49;
const PROGRAM_ERROR_50: u32 = 50;
const PROGRAM_ERROR_51: u32 = 51;
const PROGRAM_ERROR_52: u32 = 52;
const PROGRAM_ERROR_53: u32 = 53;
const PROGRAM_ERROR_54: u32 = 54;
const PROGRAM_ERROR_55: u32 = 55;
const PROGRAM_ERROR_56: u32 = 56;

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
/// Validated data for `AddBlocker`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddBlocker {}
/// Bounded codec for [`AddBlocker`].
#[derive(Clone, Copy, Debug)]
pub struct AddBlockerCodec;
impl Encoder<AddBlocker> for AddBlockerCodec {
    fn encoded_len(&self, value: &AddBlocker) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &AddBlocker, writer: &mut Writer<'_>) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<AddBlocker> for AddBlockerCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AddBlocker, Error> {
        let _ = reader;
        Ok(AddBlocker {})
    }
}
/// Validated data for `BubblegumV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BubblegumV2 {}
/// Bounded codec for [`BubblegumV2`].
#[derive(Clone, Copy, Debug)]
pub struct BubblegumV2Codec;
impl Encoder<BubblegumV2> for BubblegumV2Codec {
    fn encoded_len(&self, value: &BubblegumV2) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &BubblegumV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<BubblegumV2> for BubblegumV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BubblegumV2, Error> {
        let _ = reader;
        Ok(BubblegumV2 {})
    }
}
/// Validated data for `BurnDelegate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnDelegate {}
/// Bounded codec for [`BurnDelegate`].
#[derive(Clone, Copy, Debug)]
pub struct BurnDelegateCodec;
impl Encoder<BurnDelegate> for BurnDelegateCodec {
    fn encoded_len(&self, value: &BurnDelegate) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &BurnDelegate, writer: &mut Writer<'_>) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<BurnDelegate> for BurnDelegateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BurnDelegate, Error> {
        let _ = reader;
        Ok(BurnDelegate {})
    }
}
/// Validated data for `TransferDelegate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferDelegate {}
/// Bounded codec for [`TransferDelegate`].
#[derive(Clone, Copy, Debug)]
pub struct TransferDelegateCodec;
impl Encoder<TransferDelegate> for TransferDelegateCodec {
    fn encoded_len(&self, value: &TransferDelegate) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &TransferDelegate, writer: &mut Writer<'_>) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<TransferDelegate> for TransferDelegateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferDelegate, Error> {
        let _ = reader;
        Ok(TransferDelegate {})
    }
}
/// Validated data for `PermanentTransferDelegate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermanentTransferDelegate {}
/// Bounded codec for [`PermanentTransferDelegate`].
#[derive(Clone, Copy, Debug)]
pub struct PermanentTransferDelegateCodec;
impl Encoder<PermanentTransferDelegate> for PermanentTransferDelegateCodec {
    fn encoded_len(&self, value: &PermanentTransferDelegate) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &PermanentTransferDelegate,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<PermanentTransferDelegate> for PermanentTransferDelegateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PermanentTransferDelegate, Error> {
        let _ = reader;
        Ok(PermanentTransferDelegate {})
    }
}
/// Validated data for `PermanentBurnDelegate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermanentBurnDelegate {}
/// Bounded codec for [`PermanentBurnDelegate`].
#[derive(Clone, Copy, Debug)]
pub struct PermanentBurnDelegateCodec;
impl Encoder<PermanentBurnDelegate> for PermanentBurnDelegateCodec {
    fn encoded_len(&self, value: &PermanentBurnDelegate) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &PermanentBurnDelegate, writer: &mut Writer<'_>) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<PermanentBurnDelegate> for PermanentBurnDelegateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PermanentBurnDelegate, Error> {
        let _ = reader;
        Ok(PermanentBurnDelegate {})
    }
}
/// Validated data for `ImmutableMetadata`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImmutableMetadata {}
/// Bounded codec for [`ImmutableMetadata`].
#[derive(Clone, Copy, Debug)]
pub struct ImmutableMetadataCodec;
impl Encoder<ImmutableMetadata> for ImmutableMetadataCodec {
    fn encoded_len(&self, value: &ImmutableMetadata) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &ImmutableMetadata, writer: &mut Writer<'_>) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ImmutableMetadata> for ImmutableMetadataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ImmutableMetadata, Error> {
        let _ = reader;
        Ok(ImmutableMetadata {})
    }
}
/// Validated data for `DataSectionUpdateInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataSectionUpdateInfo {}
/// Bounded codec for [`DataSectionUpdateInfo`].
#[derive(Clone, Copy, Debug)]
pub struct DataSectionUpdateInfoCodec;
impl Encoder<DataSectionUpdateInfo> for DataSectionUpdateInfoCodec {
    fn encoded_len(&self, value: &DataSectionUpdateInfo) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &DataSectionUpdateInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DataSectionUpdateInfo> for DataSectionUpdateInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DataSectionUpdateInfo, Error> {
        let _ = reader;
        Ok(DataSectionUpdateInfo {})
    }
}
/// Validated data for `AgentIdentity`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentIdentity {
    /// The `uri` wire field.
    pub uri: String,
}
/// Bounded codec for [`AgentIdentity`].
#[derive(Clone, Copy, Debug)]
pub struct AgentIdentityCodec;
impl Encoder<AgentIdentity> for AgentIdentityCodec {
    fn encoded_len(&self, value: &AgentIdentity) -> Result<usize, Error> {
        let size = 0usize;
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
        Ok(size)
    }
    fn write(&self, value: &AgentIdentity, writer: &mut Writer<'_>) -> Result<(), Error> {
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
        Ok(())
    }
}
impl Decoder<AgentIdentity> for AgentIdentityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AgentIdentity, Error> {
        Ok(AgentIdentity {
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
        })
    }
}
/// Validated data for `AgentIdentityInitInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentIdentityInitInfo {
    /// The `uri` wire field.
    pub uri: String,
    /// The `init_plugin_authority` wire field.
    pub init_plugin_authority: Option<Authority>,
    /// The `lifecycle_checks` wire field.
    pub lifecycle_checks: Vec<LifecycleCheck>,
}
/// Bounded codec for [`AgentIdentityInitInfo`].
#[derive(Clone, Copy, Debug)]
pub struct AgentIdentityInitInfoCodec;
impl Encoder<AgentIdentityInitInfo> for AgentIdentityInitInfoCodec {
    fn encoded_len(&self, value: &AgentIdentityInitInfo) -> Result<usize, Error> {
        let size = 0usize;
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
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.init_plugin_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: LifecycleCheckCodec,
                    minimum: 5,
                }
                .encoded_len(value.lifecycle_checks.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AgentIdentityInitInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
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
        OptionCodec::u8(AuthorityCodec).write(&value.init_plugin_authority, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: LifecycleCheckCodec,
            minimum: 5,
        }
        .write(value.lifecycle_checks.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<AgentIdentityInitInfo> for AgentIdentityInitInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AgentIdentityInitInfo, Error> {
        Ok(AgentIdentityInitInfo {
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
            init_plugin_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
            lifecycle_checks: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: LifecycleCheckCodec,
                minimum: 5,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `AgentIdentityUpdateInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentIdentityUpdateInfo {
    /// The `uri` wire field.
    pub uri: Option<String>,
    /// The `lifecycle_checks` wire field.
    pub lifecycle_checks: Option<Vec<LifecycleCheck>>,
}
/// Bounded codec for [`AgentIdentityUpdateInfo`].
#[derive(Clone, Copy, Debug)]
pub struct AgentIdentityUpdateInfoCodec;
impl Encoder<AgentIdentityUpdateInfo> for AgentIdentityUpdateInfoCodec {
    fn encoded_len(&self, value: &AgentIdentityUpdateInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.uri)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: LifecycleCheckCodec,
                    minimum: 5,
                }))
                .encoded_len(&value.lifecycle_checks)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AgentIdentityUpdateInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.uri, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: LifecycleCheckCodec,
            minimum: 5,
        }))
        .write(&value.lifecycle_checks, writer)?;
        Ok(())
    }
}
impl Decoder<AgentIdentityUpdateInfo> for AgentIdentityUpdateInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AgentIdentityUpdateInfo, Error> {
        Ok(AgentIdentityUpdateInfo {
            uri: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
            lifecycle_checks: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: LifecycleCheckCodec,
                minimum: 5,
            }))
            .read(reader)?,
        })
    }
}
/// Validated data for `AppData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppData {
    /// The `data_authority` wire field.
    pub data_authority: Authority,
    /// The `schema` wire field.
    pub schema: ExternalPluginAdapterSchema,
}
/// Bounded codec for [`AppData`].
#[derive(Clone, Copy, Debug)]
pub struct AppDataCodec;
impl Encoder<AppData> for AppDataCodec {
    fn encoded_len(&self, value: &AppData) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(AuthorityCodec.encoded_len(&value.data_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalPluginAdapterSchemaCodec.encoded_len(&value.schema)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AppData, writer: &mut Writer<'_>) -> Result<(), Error> {
        AuthorityCodec.write(&value.data_authority, writer)?;
        ExternalPluginAdapterSchemaCodec.write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<AppData> for AppDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AppData, Error> {
        Ok(AppData {
            data_authority: AuthorityCodec.read(reader)?,
            schema: ExternalPluginAdapterSchemaCodec.read(reader)?,
        })
    }
}
/// Validated data for `AppDataInitInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppDataInitInfo {
    /// The `data_authority` wire field.
    pub data_authority: Authority,
    /// The `init_plugin_authority` wire field.
    pub init_plugin_authority: Option<Authority>,
    /// The `schema` wire field.
    pub schema: Option<ExternalPluginAdapterSchema>,
}
/// Bounded codec for [`AppDataInitInfo`].
#[derive(Clone, Copy, Debug)]
pub struct AppDataInitInfoCodec;
impl Encoder<AppDataInitInfo> for AppDataInitInfoCodec {
    fn encoded_len(&self, value: &AppDataInitInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(AuthorityCodec.encoded_len(&value.data_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.init_plugin_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ExternalPluginAdapterSchemaCodec).encoded_len(&value.schema)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AppDataInitInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        AuthorityCodec.write(&value.data_authority, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.init_plugin_authority, writer)?;
        OptionCodec::u8(ExternalPluginAdapterSchemaCodec).write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<AppDataInitInfo> for AppDataInitInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AppDataInitInfo, Error> {
        Ok(AppDataInitInfo {
            data_authority: AuthorityCodec.read(reader)?,
            init_plugin_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
            schema: OptionCodec::u8(ExternalPluginAdapterSchemaCodec).read(reader)?,
        })
    }
}
/// Validated data for `AppDataUpdateInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppDataUpdateInfo {
    /// The `schema` wire field.
    pub schema: Option<ExternalPluginAdapterSchema>,
}
/// Bounded codec for [`AppDataUpdateInfo`].
#[derive(Clone, Copy, Debug)]
pub struct AppDataUpdateInfoCodec;
impl Encoder<AppDataUpdateInfo> for AppDataUpdateInfoCodec {
    fn encoded_len(&self, value: &AppDataUpdateInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(ExternalPluginAdapterSchemaCodec).encoded_len(&value.schema)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AppDataUpdateInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(ExternalPluginAdapterSchemaCodec).write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<AppDataUpdateInfo> for AppDataUpdateInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AppDataUpdateInfo, Error> {
        Ok(AppDataUpdateInfo {
            schema: OptionCodec::u8(ExternalPluginAdapterSchemaCodec).read(reader)?,
        })
    }
}
/// Validated data for `Attribute`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attribute {
    /// The `key` wire field.
    pub key: String,
    /// The `value` wire field.
    pub value: String,
}
/// Bounded codec for [`Attribute`].
#[derive(Clone, Copy, Debug)]
pub struct AttributeCodec;
impl Encoder<Attribute> for AttributeCodec {
    fn encoded_len(&self, value: &Attribute) -> Result<usize, Error> {
        let size = 0usize;
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
                .encoded_len(value.key.as_str())?,
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
                .encoded_len(value.value.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Attribute, writer: &mut Writer<'_>) -> Result<(), Error> {
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.key.as_str(), writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.value.as_str(), writer)?;
        Ok(())
    }
}
impl Decoder<Attribute> for AttributeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Attribute, Error> {
        Ok(Attribute {
            key: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            value: ByteSizePrefix {
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
/// Validated data for `Attributes`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attributes {
    /// The `attribute_list` wire field.
    pub attribute_list: Vec<Attribute>,
}
/// Bounded codec for [`Attributes`].
#[derive(Clone, Copy, Debug)]
pub struct AttributesCodec;
impl Encoder<Attributes> for AttributesCodec {
    fn encoded_len(&self, value: &Attributes) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: AttributeCodec,
                    minimum: 8,
                }
                .encoded_len(value.attribute_list.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Attributes, writer: &mut Writer<'_>) -> Result<(), Error> {
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: AttributeCodec,
            minimum: 8,
        }
        .write(value.attribute_list.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<Attributes> for AttributesCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Attributes, Error> {
        Ok(Attributes {
            attribute_list: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: AttributeCodec,
                minimum: 8,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `Autograph`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Autograph {
    /// The `signatures` wire field.
    pub signatures: Vec<AutographSignature>,
}
/// Bounded codec for [`Autograph`].
#[derive(Clone, Copy, Debug)]
pub struct AutographCodec;
impl Encoder<Autograph> for AutographCodec {
    fn encoded_len(&self, value: &Autograph) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: AutographSignatureCodec,
                    minimum: 36,
                }
                .encoded_len(value.signatures.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Autograph, writer: &mut Writer<'_>) -> Result<(), Error> {
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: AutographSignatureCodec,
            minimum: 36,
        }
        .write(value.signatures.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<Autograph> for AutographCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Autograph, Error> {
        Ok(Autograph {
            signatures: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: AutographSignatureCodec,
                minimum: 36,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `AutographSignature`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutographSignature {
    /// The `address` wire field.
    pub address: Address,
    /// The `message` wire field.
    pub message: String,
}
/// Bounded codec for [`AutographSignature`].
#[derive(Clone, Copy, Debug)]
pub struct AutographSignatureCodec;
impl Encoder<AutographSignature> for AutographSignatureCodec {
    fn encoded_len(&self, value: &AutographSignature) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.address)?,
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
                .encoded_len(value.message.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AutographSignature, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.address, writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.message.as_str(), writer)?;
        Ok(())
    }
}
impl Decoder<AutographSignature> for AutographSignatureCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AutographSignature, Error> {
        Ok(AutographSignature {
            address: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            message: ByteSizePrefix {
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
/// Validated data for `CompressionProof`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompressionProof {
    /// The `owner` wire field.
    pub owner: Address,
    /// The `update_authority` wire field.
    pub update_authority: UpdateAuthority,
    /// The `name` wire field.
    pub name: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `seq` wire field.
    pub seq: u64,
    /// The `plugins` wire field.
    pub plugins: Vec<HashablePluginSchema>,
}
/// Bounded codec for [`CompressionProof`].
#[derive(Clone, Copy, Debug)]
pub struct CompressionProofCodec;
impl Encoder<CompressionProof> for CompressionProofCodec {
    fn encoded_len(&self, value: &CompressionProof) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.owner)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(UpdateAuthorityCodec.encoded_len(&value.update_authority)?)
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
            .checked_add(Number::<u64>::LE.encoded_len(&value.seq)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: HashablePluginSchemaCodec,
                    minimum: 10,
                }
                .encoded_len(value.plugins.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CompressionProof, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.owner, writer)?;
        UpdateAuthorityCodec.write(&value.update_authority, writer)?;
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
        Number::<u64>::LE.write(&value.seq, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: HashablePluginSchemaCodec,
            minimum: 10,
        }
        .write(value.plugins.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<CompressionProof> for CompressionProofCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CompressionProof, Error> {
        Ok(CompressionProof {
            owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            update_authority: UpdateAuthorityCodec.read(reader)?,
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
            seq: Number::<u64>::LE.read(reader)?,
            plugins: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: HashablePluginSchemaCodec,
                minimum: 10,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `Creator`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Creator {
    /// The `address` wire field.
    pub address: Address,
    /// The `percentage` wire field.
    pub percentage: u8,
}
/// Bounded codec for [`Creator`].
#[derive(Clone, Copy, Debug)]
pub struct CreatorCodec;
impl Encoder<Creator> for CreatorCodec {
    fn encoded_len(&self, value: &Creator) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.address)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.percentage)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Creator, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.address, writer)?;
        Number::<u8>::LE.write(&value.percentage, writer)?;
        Ok(())
    }
}
impl Decoder<Creator> for CreatorCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Creator, Error> {
        Ok(Creator {
            address: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            percentage: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `DataSection`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataSection {
    /// The `parent_key` wire field.
    pub parent_key: LinkedDataKey,
    /// The `schema` wire field.
    pub schema: ExternalPluginAdapterSchema,
}
/// Bounded codec for [`DataSection`].
#[derive(Clone, Copy, Debug)]
pub struct DataSectionCodec;
impl Encoder<DataSection> for DataSectionCodec {
    fn encoded_len(&self, value: &DataSection) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(LinkedDataKeyCodec.encoded_len(&value.parent_key)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalPluginAdapterSchemaCodec.encoded_len(&value.schema)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &DataSection, writer: &mut Writer<'_>) -> Result<(), Error> {
        LinkedDataKeyCodec.write(&value.parent_key, writer)?;
        ExternalPluginAdapterSchemaCodec.write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<DataSection> for DataSectionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DataSection, Error> {
        Ok(DataSection {
            parent_key: LinkedDataKeyCodec.read(reader)?,
            schema: ExternalPluginAdapterSchemaCodec.read(reader)?,
        })
    }
}
/// Validated data for `DataSectionInitInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataSectionInitInfo {
    /// The `parent_key` wire field.
    pub parent_key: LinkedDataKey,
    /// The `schema` wire field.
    pub schema: ExternalPluginAdapterSchema,
}
/// Bounded codec for [`DataSectionInitInfo`].
#[derive(Clone, Copy, Debug)]
pub struct DataSectionInitInfoCodec;
impl Encoder<DataSectionInitInfo> for DataSectionInitInfoCodec {
    fn encoded_len(&self, value: &DataSectionInitInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(LinkedDataKeyCodec.encoded_len(&value.parent_key)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalPluginAdapterSchemaCodec.encoded_len(&value.schema)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &DataSectionInitInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        LinkedDataKeyCodec.write(&value.parent_key, writer)?;
        ExternalPluginAdapterSchemaCodec.write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<DataSectionInitInfo> for DataSectionInitInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DataSectionInitInfo, Error> {
        Ok(DataSectionInitInfo {
            parent_key: LinkedDataKeyCodec.read(reader)?,
            schema: ExternalPluginAdapterSchemaCodec.read(reader)?,
        })
    }
}
/// Validated data for `Edition`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edition {
    /// The `number` wire field.
    pub number: u32,
}
/// Bounded codec for [`Edition`].
#[derive(Clone, Copy, Debug)]
pub struct EditionCodec;
impl Encoder<Edition> for EditionCodec {
    fn encoded_len(&self, value: &Edition) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.number)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Edition, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u32>::LE.write(&value.number, writer)?;
        Ok(())
    }
}
impl Decoder<Edition> for EditionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Edition, Error> {
        Ok(Edition {
            number: Number::<u32>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ExternalCheckResult`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalCheckResult {
    /// The `flags` wire field.
    pub flags: u32,
}
/// Bounded codec for [`ExternalCheckResult`].
#[derive(Clone, Copy, Debug)]
pub struct ExternalCheckResultCodec;
impl Encoder<ExternalCheckResult> for ExternalCheckResultCodec {
    fn encoded_len(&self, value: &ExternalCheckResult) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ExternalCheckResult, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u32>::LE.write(&value.flags, writer)?;
        Ok(())
    }
}
impl Decoder<ExternalCheckResult> for ExternalCheckResultCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExternalCheckResult, Error> {
        Ok(ExternalCheckResult {
            flags: Number::<u32>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ExternalRegistryRecord`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalRegistryRecord {
    /// The `plugin_type` wire field.
    pub plugin_type: ExternalPluginAdapterType,
    /// The `authority` wire field.
    pub authority: Authority,
    /// The `lifecycle_checks` wire field.
    pub lifecycle_checks: Option<Vec<LifecycleCheck>>,
    /// The `offset` wire field.
    pub offset: u64,
    /// The `data_offset` wire field.
    pub data_offset: Option<u64>,
    /// The `data_len` wire field.
    pub data_len: Option<u64>,
}
/// Bounded codec for [`ExternalRegistryRecord`].
#[derive(Clone, Copy, Debug)]
pub struct ExternalRegistryRecordCodec;
impl Encoder<ExternalRegistryRecord> for ExternalRegistryRecordCodec {
    fn encoded_len(&self, value: &ExternalRegistryRecord) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(ExternalPluginAdapterTypeCodec.encoded_len(&value.plugin_type)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(AuthorityCodec.encoded_len(&value.authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: LifecycleCheckCodec,
                    minimum: 5,
                }))
                .encoded_len(&value.lifecycle_checks)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.data_offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.data_len)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ExternalRegistryRecord, writer: &mut Writer<'_>) -> Result<(), Error> {
        ExternalPluginAdapterTypeCodec.write(&value.plugin_type, writer)?;
        AuthorityCodec.write(&value.authority, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: LifecycleCheckCodec,
            minimum: 5,
        }))
        .write(&value.lifecycle_checks, writer)?;
        Number::<u64>::LE.write(&value.offset, writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.data_offset, writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.data_len, writer)?;
        Ok(())
    }
}
impl Decoder<ExternalRegistryRecord> for ExternalRegistryRecordCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExternalRegistryRecord, Error> {
        Ok(ExternalRegistryRecord {
            plugin_type: ExternalPluginAdapterTypeCodec.read(reader)?,
            authority: AuthorityCodec.read(reader)?,
            lifecycle_checks: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: LifecycleCheckCodec,
                minimum: 5,
            }))
            .read(reader)?,
            offset: Number::<u64>::LE.read(reader)?,
            data_offset: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
            data_len: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
        })
    }
}
/// Validated data for `FreezeDelegate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreezeDelegate {
    /// The `frozen` wire field.
    pub frozen: bool,
}
/// Bounded codec for [`FreezeDelegate`].
#[derive(Clone, Copy, Debug)]
pub struct FreezeDelegateCodec;
impl Encoder<FreezeDelegate> for FreezeDelegateCodec {
    fn encoded_len(&self, value: &FreezeDelegate) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.frozen)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &FreezeDelegate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Bool.write(&value.frozen, writer)?;
        Ok(())
    }
}
impl Decoder<FreezeDelegate> for FreezeDelegateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<FreezeDelegate, Error> {
        Ok(FreezeDelegate {
            frozen: Bool.read(reader)?,
        })
    }
}
/// Validated data for `FreezeExecute`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreezeExecute {
    /// The `frozen` wire field.
    pub frozen: bool,
}
/// Bounded codec for [`FreezeExecute`].
#[derive(Clone, Copy, Debug)]
pub struct FreezeExecuteCodec;
impl Encoder<FreezeExecute> for FreezeExecuteCodec {
    fn encoded_len(&self, value: &FreezeExecute) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.frozen)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &FreezeExecute, writer: &mut Writer<'_>) -> Result<(), Error> {
        Bool.write(&value.frozen, writer)?;
        Ok(())
    }
}
impl Decoder<FreezeExecute> for FreezeExecuteCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<FreezeExecute, Error> {
        Ok(FreezeExecute {
            frozen: Bool.read(reader)?,
        })
    }
}
/// Validated data for `Groups`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Groups {
    /// The `groups` wire field.
    pub groups: Vec<Address>,
}
/// Bounded codec for [`Groups`].
#[derive(Clone, Copy, Debug)]
pub struct GroupsCodec;
impl Encoder<Groups> for GroupsCodec {
    fn encoded_len(&self, value: &Groups) -> Result<usize, Error> {
        let size = 0usize;
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
                .encoded_len(value.groups.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Groups, writer: &mut Writer<'_>) -> Result<(), Error> {
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.groups.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<Groups> for GroupsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Groups, Error> {
        Ok(Groups {
            groups: CheckedCounted {
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
/// Validated data for `HashablePluginSchema`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HashablePluginSchema {
    /// The `index` wire field.
    pub index: u64,
    /// The `authority` wire field.
    pub authority: Authority,
    /// The `plugin` wire field.
    pub plugin: Plugin,
}
/// Bounded codec for [`HashablePluginSchema`].
#[derive(Clone, Copy, Debug)]
pub struct HashablePluginSchemaCodec;
impl Encoder<HashablePluginSchema> for HashablePluginSchemaCodec {
    fn encoded_len(&self, value: &HashablePluginSchema) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(AuthorityCodec.encoded_len(&value.authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(PluginCodec.encoded_len(&value.plugin)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &HashablePluginSchema, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.index, writer)?;
        AuthorityCodec.write(&value.authority, writer)?;
        PluginCodec.write(&value.plugin, writer)?;
        Ok(())
    }
}
impl Decoder<HashablePluginSchema> for HashablePluginSchemaCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<HashablePluginSchema, Error> {
        Ok(HashablePluginSchema {
            index: Number::<u64>::LE.read(reader)?,
            authority: AuthorityCodec.read(reader)?,
            plugin: PluginCodec.read(reader)?,
        })
    }
}
/// Validated data for `HashedAssetSchema`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HashedAssetSchema {
    /// The `asset_hash` wire field.
    pub asset_hash: [u8; 32],
    /// The `plugin_hashes` wire field.
    pub plugin_hashes: Vec<[u8; 32]>,
}
/// Bounded codec for [`HashedAssetSchema`].
#[derive(Clone, Copy, Debug)]
pub struct HashedAssetSchemaCodec;
impl Encoder<HashedAssetSchema> for HashedAssetSchemaCodec {
    fn encoded_len(&self, value: &HashedAssetSchema) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.asset_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: FixedBytes::<32>,
                    minimum: 32,
                }
                .encoded_len(value.plugin_hashes.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &HashedAssetSchema, writer: &mut Writer<'_>) -> Result<(), Error> {
        FixedBytes::<32>.write(&value.asset_hash, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: FixedBytes::<32>,
            minimum: 32,
        }
        .write(value.plugin_hashes.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<HashedAssetSchema> for HashedAssetSchemaCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<HashedAssetSchema, Error> {
        Ok(HashedAssetSchema {
            asset_hash: FixedBytes::<32>.read(reader)?,
            plugin_hashes: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: FixedBytes::<32>,
                minimum: 32,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `LifecycleCheck`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleCheck {
    /// The `event` wire field.
    pub event: HookableLifecycleEvent,
    /// The `result` wire field.
    pub result: ExternalCheckResult,
}
/// Bounded codec for [`LifecycleCheck`].
#[derive(Clone, Copy, Debug)]
pub struct LifecycleCheckCodec;
impl Encoder<LifecycleCheck> for LifecycleCheckCodec {
    fn encoded_len(&self, value: &LifecycleCheck) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(HookableLifecycleEventCodec.encoded_len(&value.event)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalCheckResultCodec.encoded_len(&value.result)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LifecycleCheck, writer: &mut Writer<'_>) -> Result<(), Error> {
        HookableLifecycleEventCodec.write(&value.event, writer)?;
        ExternalCheckResultCodec.write(&value.result, writer)?;
        Ok(())
    }
}
impl Decoder<LifecycleCheck> for LifecycleCheckCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LifecycleCheck, Error> {
        Ok(LifecycleCheck {
            event: HookableLifecycleEventCodec.read(reader)?,
            result: ExternalCheckResultCodec.read(reader)?,
        })
    }
}
/// Validated data for `LifecycleHook`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleHook {
    /// The `hooked_program` wire field.
    pub hooked_program: Address,
    /// The `extra_accounts` wire field.
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    /// The `data_authority` wire field.
    pub data_authority: Option<Authority>,
    /// The `schema` wire field.
    pub schema: ExternalPluginAdapterSchema,
}
/// Bounded codec for [`LifecycleHook`].
#[derive(Clone, Copy, Debug)]
pub struct LifecycleHookCodec;
impl Encoder<LifecycleHook> for LifecycleHookCodec {
    fn encoded_len(&self, value: &LifecycleHook) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.hooked_program)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ExtraAccountCodec,
                    minimum: 3,
                }))
                .encoded_len(&value.extra_accounts)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.data_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalPluginAdapterSchemaCodec.encoded_len(&value.schema)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LifecycleHook, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.hooked_program, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ExtraAccountCodec,
            minimum: 3,
        }))
        .write(&value.extra_accounts, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.data_authority, writer)?;
        ExternalPluginAdapterSchemaCodec.write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<LifecycleHook> for LifecycleHookCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LifecycleHook, Error> {
        Ok(LifecycleHook {
            hooked_program: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            extra_accounts: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ExtraAccountCodec,
                minimum: 3,
            }))
            .read(reader)?,
            data_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
            schema: ExternalPluginAdapterSchemaCodec.read(reader)?,
        })
    }
}
/// Validated data for `LifecycleHookInitInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleHookInitInfo {
    /// The `hooked_program` wire field.
    pub hooked_program: Address,
    /// The `init_plugin_authority` wire field.
    pub init_plugin_authority: Option<Authority>,
    /// The `lifecycle_checks` wire field.
    pub lifecycle_checks: Vec<LifecycleCheck>,
    /// The `extra_accounts` wire field.
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    /// The `data_authority` wire field.
    pub data_authority: Option<Authority>,
    /// The `schema` wire field.
    pub schema: Option<ExternalPluginAdapterSchema>,
}
/// Bounded codec for [`LifecycleHookInitInfo`].
#[derive(Clone, Copy, Debug)]
pub struct LifecycleHookInitInfoCodec;
impl Encoder<LifecycleHookInitInfo> for LifecycleHookInitInfoCodec {
    fn encoded_len(&self, value: &LifecycleHookInitInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.hooked_program)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.init_plugin_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: LifecycleCheckCodec,
                    minimum: 5,
                }
                .encoded_len(value.lifecycle_checks.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ExtraAccountCodec,
                    minimum: 3,
                }))
                .encoded_len(&value.extra_accounts)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.data_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ExternalPluginAdapterSchemaCodec).encoded_len(&value.schema)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LifecycleHookInitInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.hooked_program, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.init_plugin_authority, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: LifecycleCheckCodec,
            minimum: 5,
        }
        .write(value.lifecycle_checks.as_slice(), writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ExtraAccountCodec,
            minimum: 3,
        }))
        .write(&value.extra_accounts, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.data_authority, writer)?;
        OptionCodec::u8(ExternalPluginAdapterSchemaCodec).write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<LifecycleHookInitInfo> for LifecycleHookInitInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LifecycleHookInitInfo, Error> {
        Ok(LifecycleHookInitInfo {
            hooked_program: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            init_plugin_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
            lifecycle_checks: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: LifecycleCheckCodec,
                minimum: 5,
            }
            .read(reader)?,
            extra_accounts: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ExtraAccountCodec,
                minimum: 3,
            }))
            .read(reader)?,
            data_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
            schema: OptionCodec::u8(ExternalPluginAdapterSchemaCodec).read(reader)?,
        })
    }
}
/// Validated data for `LifecycleHookUpdateInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleHookUpdateInfo {
    /// The `lifecycle_checks` wire field.
    pub lifecycle_checks: Option<Vec<LifecycleCheck>>,
    /// The `extra_accounts` wire field.
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    /// The `schema` wire field.
    pub schema: Option<ExternalPluginAdapterSchema>,
}
/// Bounded codec for [`LifecycleHookUpdateInfo`].
#[derive(Clone, Copy, Debug)]
pub struct LifecycleHookUpdateInfoCodec;
impl Encoder<LifecycleHookUpdateInfo> for LifecycleHookUpdateInfoCodec {
    fn encoded_len(&self, value: &LifecycleHookUpdateInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: LifecycleCheckCodec,
                    minimum: 5,
                }))
                .encoded_len(&value.lifecycle_checks)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ExtraAccountCodec,
                    minimum: 3,
                }))
                .encoded_len(&value.extra_accounts)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ExternalPluginAdapterSchemaCodec).encoded_len(&value.schema)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LifecycleHookUpdateInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: LifecycleCheckCodec,
            minimum: 5,
        }))
        .write(&value.lifecycle_checks, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ExtraAccountCodec,
            minimum: 3,
        }))
        .write(&value.extra_accounts, writer)?;
        OptionCodec::u8(ExternalPluginAdapterSchemaCodec).write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<LifecycleHookUpdateInfo> for LifecycleHookUpdateInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LifecycleHookUpdateInfo, Error> {
        Ok(LifecycleHookUpdateInfo {
            lifecycle_checks: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: LifecycleCheckCodec,
                minimum: 5,
            }))
            .read(reader)?,
            extra_accounts: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ExtraAccountCodec,
                minimum: 3,
            }))
            .read(reader)?,
            schema: OptionCodec::u8(ExternalPluginAdapterSchemaCodec).read(reader)?,
        })
    }
}
/// Validated data for `LinkedAppData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkedAppData {
    /// The `data_authority` wire field.
    pub data_authority: Authority,
    /// The `schema` wire field.
    pub schema: ExternalPluginAdapterSchema,
}
/// Bounded codec for [`LinkedAppData`].
#[derive(Clone, Copy, Debug)]
pub struct LinkedAppDataCodec;
impl Encoder<LinkedAppData> for LinkedAppDataCodec {
    fn encoded_len(&self, value: &LinkedAppData) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(AuthorityCodec.encoded_len(&value.data_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalPluginAdapterSchemaCodec.encoded_len(&value.schema)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LinkedAppData, writer: &mut Writer<'_>) -> Result<(), Error> {
        AuthorityCodec.write(&value.data_authority, writer)?;
        ExternalPluginAdapterSchemaCodec.write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<LinkedAppData> for LinkedAppDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LinkedAppData, Error> {
        Ok(LinkedAppData {
            data_authority: AuthorityCodec.read(reader)?,
            schema: ExternalPluginAdapterSchemaCodec.read(reader)?,
        })
    }
}
/// Validated data for `LinkedAppDataInitInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkedAppDataInitInfo {
    /// The `data_authority` wire field.
    pub data_authority: Authority,
    /// The `init_plugin_authority` wire field.
    pub init_plugin_authority: Option<Authority>,
    /// The `schema` wire field.
    pub schema: Option<ExternalPluginAdapterSchema>,
}
/// Bounded codec for [`LinkedAppDataInitInfo`].
#[derive(Clone, Copy, Debug)]
pub struct LinkedAppDataInitInfoCodec;
impl Encoder<LinkedAppDataInitInfo> for LinkedAppDataInitInfoCodec {
    fn encoded_len(&self, value: &LinkedAppDataInitInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(AuthorityCodec.encoded_len(&value.data_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.init_plugin_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ExternalPluginAdapterSchemaCodec).encoded_len(&value.schema)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LinkedAppDataInitInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        AuthorityCodec.write(&value.data_authority, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.init_plugin_authority, writer)?;
        OptionCodec::u8(ExternalPluginAdapterSchemaCodec).write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<LinkedAppDataInitInfo> for LinkedAppDataInitInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LinkedAppDataInitInfo, Error> {
        Ok(LinkedAppDataInitInfo {
            data_authority: AuthorityCodec.read(reader)?,
            init_plugin_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
            schema: OptionCodec::u8(ExternalPluginAdapterSchemaCodec).read(reader)?,
        })
    }
}
/// Validated data for `LinkedAppDataUpdateInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkedAppDataUpdateInfo {
    /// The `schema` wire field.
    pub schema: Option<ExternalPluginAdapterSchema>,
}
/// Bounded codec for [`LinkedAppDataUpdateInfo`].
#[derive(Clone, Copy, Debug)]
pub struct LinkedAppDataUpdateInfoCodec;
impl Encoder<LinkedAppDataUpdateInfo> for LinkedAppDataUpdateInfoCodec {
    fn encoded_len(&self, value: &LinkedAppDataUpdateInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(ExternalPluginAdapterSchemaCodec).encoded_len(&value.schema)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LinkedAppDataUpdateInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(ExternalPluginAdapterSchemaCodec).write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<LinkedAppDataUpdateInfo> for LinkedAppDataUpdateInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LinkedAppDataUpdateInfo, Error> {
        Ok(LinkedAppDataUpdateInfo {
            schema: OptionCodec::u8(ExternalPluginAdapterSchemaCodec).read(reader)?,
        })
    }
}
/// Validated data for `LinkedLifecycleHook`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkedLifecycleHook {
    /// The `hooked_program` wire field.
    pub hooked_program: Address,
    /// The `extra_accounts` wire field.
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    /// The `data_authority` wire field.
    pub data_authority: Option<Authority>,
    /// The `schema` wire field.
    pub schema: ExternalPluginAdapterSchema,
}
/// Bounded codec for [`LinkedLifecycleHook`].
#[derive(Clone, Copy, Debug)]
pub struct LinkedLifecycleHookCodec;
impl Encoder<LinkedLifecycleHook> for LinkedLifecycleHookCodec {
    fn encoded_len(&self, value: &LinkedLifecycleHook) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.hooked_program)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ExtraAccountCodec,
                    minimum: 3,
                }))
                .encoded_len(&value.extra_accounts)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.data_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalPluginAdapterSchemaCodec.encoded_len(&value.schema)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LinkedLifecycleHook, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.hooked_program, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ExtraAccountCodec,
            minimum: 3,
        }))
        .write(&value.extra_accounts, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.data_authority, writer)?;
        ExternalPluginAdapterSchemaCodec.write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<LinkedLifecycleHook> for LinkedLifecycleHookCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LinkedLifecycleHook, Error> {
        Ok(LinkedLifecycleHook {
            hooked_program: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            extra_accounts: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ExtraAccountCodec,
                minimum: 3,
            }))
            .read(reader)?,
            data_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
            schema: ExternalPluginAdapterSchemaCodec.read(reader)?,
        })
    }
}
/// Validated data for `LinkedLifecycleHookInitInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkedLifecycleHookInitInfo {
    /// The `hooked_program` wire field.
    pub hooked_program: Address,
    /// The `init_plugin_authority` wire field.
    pub init_plugin_authority: Option<Authority>,
    /// The `lifecycle_checks` wire field.
    pub lifecycle_checks: Vec<LifecycleCheck>,
    /// The `extra_accounts` wire field.
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    /// The `data_authority` wire field.
    pub data_authority: Option<Authority>,
    /// The `schema` wire field.
    pub schema: Option<ExternalPluginAdapterSchema>,
}
/// Bounded codec for [`LinkedLifecycleHookInitInfo`].
#[derive(Clone, Copy, Debug)]
pub struct LinkedLifecycleHookInitInfoCodec;
impl Encoder<LinkedLifecycleHookInitInfo> for LinkedLifecycleHookInitInfoCodec {
    fn encoded_len(&self, value: &LinkedLifecycleHookInitInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.hooked_program)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.init_plugin_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: LifecycleCheckCodec,
                    minimum: 5,
                }
                .encoded_len(value.lifecycle_checks.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ExtraAccountCodec,
                    minimum: 3,
                }))
                .encoded_len(&value.extra_accounts)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.data_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ExternalPluginAdapterSchemaCodec).encoded_len(&value.schema)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &LinkedLifecycleHookInitInfo,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.hooked_program, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.init_plugin_authority, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: LifecycleCheckCodec,
            minimum: 5,
        }
        .write(value.lifecycle_checks.as_slice(), writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ExtraAccountCodec,
            minimum: 3,
        }))
        .write(&value.extra_accounts, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.data_authority, writer)?;
        OptionCodec::u8(ExternalPluginAdapterSchemaCodec).write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<LinkedLifecycleHookInitInfo> for LinkedLifecycleHookInitInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LinkedLifecycleHookInitInfo, Error> {
        Ok(LinkedLifecycleHookInitInfo {
            hooked_program: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            init_plugin_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
            lifecycle_checks: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: LifecycleCheckCodec,
                minimum: 5,
            }
            .read(reader)?,
            extra_accounts: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ExtraAccountCodec,
                minimum: 3,
            }))
            .read(reader)?,
            data_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
            schema: OptionCodec::u8(ExternalPluginAdapterSchemaCodec).read(reader)?,
        })
    }
}
/// Validated data for `LinkedLifecycleHookUpdateInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkedLifecycleHookUpdateInfo {
    /// The `lifecycle_checks` wire field.
    pub lifecycle_checks: Option<Vec<LifecycleCheck>>,
    /// The `extra_accounts` wire field.
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    /// The `schema` wire field.
    pub schema: Option<ExternalPluginAdapterSchema>,
}
/// Bounded codec for [`LinkedLifecycleHookUpdateInfo`].
#[derive(Clone, Copy, Debug)]
pub struct LinkedLifecycleHookUpdateInfoCodec;
impl Encoder<LinkedLifecycleHookUpdateInfo> for LinkedLifecycleHookUpdateInfoCodec {
    fn encoded_len(&self, value: &LinkedLifecycleHookUpdateInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: LifecycleCheckCodec,
                    minimum: 5,
                }))
                .encoded_len(&value.lifecycle_checks)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ExtraAccountCodec,
                    minimum: 3,
                }))
                .encoded_len(&value.extra_accounts)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ExternalPluginAdapterSchemaCodec).encoded_len(&value.schema)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &LinkedLifecycleHookUpdateInfo,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: LifecycleCheckCodec,
            minimum: 5,
        }))
        .write(&value.lifecycle_checks, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ExtraAccountCodec,
            minimum: 3,
        }))
        .write(&value.extra_accounts, writer)?;
        OptionCodec::u8(ExternalPluginAdapterSchemaCodec).write(&value.schema, writer)?;
        Ok(())
    }
}
impl Decoder<LinkedLifecycleHookUpdateInfo> for LinkedLifecycleHookUpdateInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LinkedLifecycleHookUpdateInfo, Error> {
        Ok(LinkedLifecycleHookUpdateInfo {
            lifecycle_checks: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: LifecycleCheckCodec,
                minimum: 5,
            }))
            .read(reader)?,
            extra_accounts: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ExtraAccountCodec,
                minimum: 3,
            }))
            .read(reader)?,
            schema: OptionCodec::u8(ExternalPluginAdapterSchemaCodec).read(reader)?,
        })
    }
}
/// Validated data for `MasterEdition`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MasterEdition {
    /// The `max_supply` wire field.
    pub max_supply: Option<u32>,
    /// The `name` wire field.
    pub name: Option<String>,
    /// The `uri` wire field.
    pub uri: Option<String>,
}
/// Bounded codec for [`MasterEdition`].
#[derive(Clone, Copy, Debug)]
pub struct MasterEditionCodec;
impl Encoder<MasterEdition> for MasterEditionCodec {
    fn encoded_len(&self, value: &MasterEdition) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u32>::LE).encoded_len(&value.max_supply)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.name)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.uri)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MasterEdition, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(Number::<u32>::LE).write(&value.max_supply, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.name, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.uri, writer)?;
        Ok(())
    }
}
impl Decoder<MasterEdition> for MasterEditionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MasterEdition, Error> {
        Ok(MasterEdition {
            max_supply: OptionCodec::u8(Number::<u32>::LE).read(reader)?,
            name: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
            uri: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
        })
    }
}
/// Validated data for `Oracle`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Oracle {
    /// The `base_address` wire field.
    pub base_address: Address,
    /// The `base_address_config` wire field.
    pub base_address_config: Option<ExtraAccount>,
    /// The `results_offset` wire field.
    pub results_offset: ValidationResultsOffset,
}
/// Bounded codec for [`Oracle`].
#[derive(Clone, Copy, Debug)]
pub struct OracleCodec;
impl Encoder<Oracle> for OracleCodec {
    fn encoded_len(&self, value: &Oracle) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.base_address)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ExtraAccountCodec).encoded_len(&value.base_address_config)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ValidationResultsOffsetCodec.encoded_len(&value.results_offset)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Oracle, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.base_address, writer)?;
        OptionCodec::u8(ExtraAccountCodec).write(&value.base_address_config, writer)?;
        ValidationResultsOffsetCodec.write(&value.results_offset, writer)?;
        Ok(())
    }
}
impl Decoder<Oracle> for OracleCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Oracle, Error> {
        Ok(Oracle {
            base_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            base_address_config: OptionCodec::u8(ExtraAccountCodec).read(reader)?,
            results_offset: ValidationResultsOffsetCodec.read(reader)?,
        })
    }
}
/// Validated data for `OracleInitInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OracleInitInfo {
    /// The `base_address` wire field.
    pub base_address: Address,
    /// The `init_plugin_authority` wire field.
    pub init_plugin_authority: Option<Authority>,
    /// The `lifecycle_checks` wire field.
    pub lifecycle_checks: Vec<LifecycleCheck>,
    /// The `base_address_config` wire field.
    pub base_address_config: Option<ExtraAccount>,
    /// The `results_offset` wire field.
    pub results_offset: Option<ValidationResultsOffset>,
}
/// Bounded codec for [`OracleInitInfo`].
#[derive(Clone, Copy, Debug)]
pub struct OracleInitInfoCodec;
impl Encoder<OracleInitInfo> for OracleInitInfoCodec {
    fn encoded_len(&self, value: &OracleInitInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.base_address)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.init_plugin_authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: LifecycleCheckCodec,
                    minimum: 5,
                }
                .encoded_len(value.lifecycle_checks.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ExtraAccountCodec).encoded_len(&value.base_address_config)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ValidationResultsOffsetCodec).encoded_len(&value.results_offset)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &OracleInitInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.base_address, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.init_plugin_authority, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: LifecycleCheckCodec,
            minimum: 5,
        }
        .write(value.lifecycle_checks.as_slice(), writer)?;
        OptionCodec::u8(ExtraAccountCodec).write(&value.base_address_config, writer)?;
        OptionCodec::u8(ValidationResultsOffsetCodec).write(&value.results_offset, writer)?;
        Ok(())
    }
}
impl Decoder<OracleInitInfo> for OracleInitInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<OracleInitInfo, Error> {
        Ok(OracleInitInfo {
            base_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            init_plugin_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
            lifecycle_checks: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: LifecycleCheckCodec,
                minimum: 5,
            }
            .read(reader)?,
            base_address_config: OptionCodec::u8(ExtraAccountCodec).read(reader)?,
            results_offset: OptionCodec::u8(ValidationResultsOffsetCodec).read(reader)?,
        })
    }
}
/// Validated data for `OracleUpdateInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OracleUpdateInfo {
    /// The `lifecycle_checks` wire field.
    pub lifecycle_checks: Option<Vec<LifecycleCheck>>,
    /// The `base_address_config` wire field.
    pub base_address_config: Option<ExtraAccount>,
    /// The `results_offset` wire field.
    pub results_offset: Option<ValidationResultsOffset>,
}
/// Bounded codec for [`OracleUpdateInfo`].
#[derive(Clone, Copy, Debug)]
pub struct OracleUpdateInfoCodec;
impl Encoder<OracleUpdateInfo> for OracleUpdateInfoCodec {
    fn encoded_len(&self, value: &OracleUpdateInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: LifecycleCheckCodec,
                    minimum: 5,
                }))
                .encoded_len(&value.lifecycle_checks)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ExtraAccountCodec).encoded_len(&value.base_address_config)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ValidationResultsOffsetCodec).encoded_len(&value.results_offset)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &OracleUpdateInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: LifecycleCheckCodec,
            minimum: 5,
        }))
        .write(&value.lifecycle_checks, writer)?;
        OptionCodec::u8(ExtraAccountCodec).write(&value.base_address_config, writer)?;
        OptionCodec::u8(ValidationResultsOffsetCodec).write(&value.results_offset, writer)?;
        Ok(())
    }
}
impl Decoder<OracleUpdateInfo> for OracleUpdateInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<OracleUpdateInfo, Error> {
        Ok(OracleUpdateInfo {
            lifecycle_checks: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: LifecycleCheckCodec,
                minimum: 5,
            }))
            .read(reader)?,
            base_address_config: OptionCodec::u8(ExtraAccountCodec).read(reader)?,
            results_offset: OptionCodec::u8(ValidationResultsOffsetCodec).read(reader)?,
        })
    }
}
/// Validated data for `PermanentFreezeDelegate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermanentFreezeDelegate {
    /// The `frozen` wire field.
    pub frozen: bool,
}
/// Bounded codec for [`PermanentFreezeDelegate`].
#[derive(Clone, Copy, Debug)]
pub struct PermanentFreezeDelegateCodec;
impl Encoder<PermanentFreezeDelegate> for PermanentFreezeDelegateCodec {
    fn encoded_len(&self, value: &PermanentFreezeDelegate) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.frozen)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PermanentFreezeDelegate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Bool.write(&value.frozen, writer)?;
        Ok(())
    }
}
impl Decoder<PermanentFreezeDelegate> for PermanentFreezeDelegateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PermanentFreezeDelegate, Error> {
        Ok(PermanentFreezeDelegate {
            frozen: Bool.read(reader)?,
        })
    }
}
/// Validated data for `PermanentFreezeExecute`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermanentFreezeExecute {
    /// The `frozen` wire field.
    pub frozen: bool,
}
/// Bounded codec for [`PermanentFreezeExecute`].
#[derive(Clone, Copy, Debug)]
pub struct PermanentFreezeExecuteCodec;
impl Encoder<PermanentFreezeExecute> for PermanentFreezeExecuteCodec {
    fn encoded_len(&self, value: &PermanentFreezeExecute) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.frozen)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PermanentFreezeExecute, writer: &mut Writer<'_>) -> Result<(), Error> {
        Bool.write(&value.frozen, writer)?;
        Ok(())
    }
}
impl Decoder<PermanentFreezeExecute> for PermanentFreezeExecuteCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PermanentFreezeExecute, Error> {
        Ok(PermanentFreezeExecute {
            frozen: Bool.read(reader)?,
        })
    }
}
/// Validated data for `PluginAuthorityPair`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginAuthorityPair {
    /// The `plugin` wire field.
    pub plugin: Plugin,
    /// The `authority` wire field.
    pub authority: Option<Authority>,
}
/// Bounded codec for [`PluginAuthorityPair`].
#[derive(Clone, Copy, Debug)]
pub struct PluginAuthorityPairCodec;
impl Encoder<PluginAuthorityPair> for PluginAuthorityPairCodec {
    fn encoded_len(&self, value: &PluginAuthorityPair) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(PluginCodec.encoded_len(&value.plugin)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.authority)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PluginAuthorityPair, writer: &mut Writer<'_>) -> Result<(), Error> {
        PluginCodec.write(&value.plugin, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.authority, writer)?;
        Ok(())
    }
}
impl Decoder<PluginAuthorityPair> for PluginAuthorityPairCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PluginAuthorityPair, Error> {
        Ok(PluginAuthorityPair {
            plugin: PluginCodec.read(reader)?,
            authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
        })
    }
}
/// Validated data for `RegistryRecord`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryRecord {
    /// The `plugin_type` wire field.
    pub plugin_type: PluginType,
    /// The `authority` wire field.
    pub authority: Authority,
    /// The `offset` wire field.
    pub offset: u64,
}
/// Bounded codec for [`RegistryRecord`].
#[derive(Clone, Copy, Debug)]
pub struct RegistryRecordCodec;
impl Encoder<RegistryRecord> for RegistryRecordCodec {
    fn encoded_len(&self, value: &RegistryRecord) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(PluginTypeCodec.encoded_len(&value.plugin_type)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(AuthorityCodec.encoded_len(&value.authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.offset)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &RegistryRecord, writer: &mut Writer<'_>) -> Result<(), Error> {
        PluginTypeCodec.write(&value.plugin_type, writer)?;
        AuthorityCodec.write(&value.authority, writer)?;
        Number::<u64>::LE.write(&value.offset, writer)?;
        Ok(())
    }
}
impl Decoder<RegistryRecord> for RegistryRecordCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RegistryRecord, Error> {
        Ok(RegistryRecord {
            plugin_type: PluginTypeCodec.read(reader)?,
            authority: AuthorityCodec.read(reader)?,
            offset: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `RelationshipEntry`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationshipEntry {
    /// The `kind` wire field.
    pub kind: RelationshipKind,
    /// The `key` wire field.
    pub key: Address,
}
/// Bounded codec for [`RelationshipEntry`].
#[derive(Clone, Copy, Debug)]
pub struct RelationshipEntryCodec;
impl Encoder<RelationshipEntry> for RelationshipEntryCodec {
    fn encoded_len(&self, value: &RelationshipEntry) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(RelationshipKindCodec.encoded_len(&value.kind)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.key)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &RelationshipEntry, writer: &mut Writer<'_>) -> Result<(), Error> {
        RelationshipKindCodec.write(&value.kind, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.key, writer)?;
        Ok(())
    }
}
impl Decoder<RelationshipEntry> for RelationshipEntryCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RelationshipEntry, Error> {
        Ok(RelationshipEntry {
            kind: RelationshipKindCodec.read(reader)?,
            key: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `Royalties`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Royalties {
    /// The `basis_points` wire field.
    pub basis_points: u16,
    /// The `creators` wire field.
    pub creators: Vec<Creator>,
    /// The `rule_set` wire field.
    pub rule_set: RuleSet,
}
/// Bounded codec for [`Royalties`].
#[derive(Clone, Copy, Debug)]
pub struct RoyaltiesCodec;
impl Encoder<Royalties> for RoyaltiesCodec {
    fn encoded_len(&self, value: &Royalties) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.basis_points)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: CreatorCodec,
                    minimum: 33,
                }
                .encoded_len(value.creators.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(RuleSetCodec.encoded_len(&value.rule_set)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Royalties, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u16>::LE.write(&value.basis_points, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: CreatorCodec,
            minimum: 33,
        }
        .write(value.creators.as_slice(), writer)?;
        RuleSetCodec.write(&value.rule_set, writer)?;
        Ok(())
    }
}
impl Decoder<Royalties> for RoyaltiesCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Royalties, Error> {
        Ok(Royalties {
            basis_points: Number::<u16>::LE.read(reader)?,
            creators: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: CreatorCodec,
                minimum: 33,
            }
            .read(reader)?,
            rule_set: RuleSetCodec.read(reader)?,
        })
    }
}
/// Validated data for `UpdateDelegate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateDelegate {
    /// The `additional_delegates` wire field.
    pub additional_delegates: Vec<Address>,
}
/// Bounded codec for [`UpdateDelegate`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateDelegateCodec;
impl Encoder<UpdateDelegate> for UpdateDelegateCodec {
    fn encoded_len(&self, value: &UpdateDelegate) -> Result<usize, Error> {
        let size = 0usize;
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
                .encoded_len(value.additional_delegates.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateDelegate, writer: &mut Writer<'_>) -> Result<(), Error> {
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.additional_delegates.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<UpdateDelegate> for UpdateDelegateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateDelegate, Error> {
        Ok(UpdateDelegate {
            additional_delegates: CheckedCounted {
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
/// Validated data for `VerifiedCreators`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCreators {
    /// The `signatures` wire field.
    pub signatures: Vec<VerifiedCreatorsSignature>,
}
/// Bounded codec for [`VerifiedCreators`].
#[derive(Clone, Copy, Debug)]
pub struct VerifiedCreatorsCodec;
impl Encoder<VerifiedCreators> for VerifiedCreatorsCodec {
    fn encoded_len(&self, value: &VerifiedCreators) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: VerifiedCreatorsSignatureCodec,
                    minimum: 33,
                }
                .encoded_len(value.signatures.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &VerifiedCreators, writer: &mut Writer<'_>) -> Result<(), Error> {
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: VerifiedCreatorsSignatureCodec,
            minimum: 33,
        }
        .write(value.signatures.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<VerifiedCreators> for VerifiedCreatorsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VerifiedCreators, Error> {
        Ok(VerifiedCreators {
            signatures: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: VerifiedCreatorsSignatureCodec,
                minimum: 33,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `VerifiedCreatorsSignature`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCreatorsSignature {
    /// The `address` wire field.
    pub address: Address,
    /// The `verified` wire field.
    pub verified: bool,
}
/// Bounded codec for [`VerifiedCreatorsSignature`].
#[derive(Clone, Copy, Debug)]
pub struct VerifiedCreatorsSignatureCodec;
impl Encoder<VerifiedCreatorsSignature> for VerifiedCreatorsSignatureCodec {
    fn encoded_len(&self, value: &VerifiedCreatorsSignature) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.address)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.verified)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &VerifiedCreatorsSignature,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.address, writer)?;
        Bool.write(&value.verified, writer)?;
        Ok(())
    }
}
impl Decoder<VerifiedCreatorsSignature> for VerifiedCreatorsSignatureCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VerifiedCreatorsSignature, Error> {
        Ok(VerifiedCreatorsSignature {
            address: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            verified: Bool.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataState {
    /// The `AccountState` variant.
    AccountState,
    /// The `LedgerState` variant.
    LedgerState,
}
/// Bounded codec for [`DataState`].
#[derive(Clone, Copy, Debug)]
pub struct DataStateCodec;
const DATASTATE_TAG_0: u8 = 0;
const DATASTATE_TAG_1: u8 = 1;
impl Encoder<DataState> for DataStateCodec {
    fn encoded_len(&self, value: &DataState) -> Result<usize, Error> {
        let size = match value {
            DataState::AccountState => 0usize,
            DataState::LedgerState => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &DataState, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            DataState::AccountState => {
                Number::<u8>::LE.write(&DATASTATE_TAG_0, writer)?;
                Ok(())
            }
            DataState::LedgerState => {
                Number::<u8>::LE.write(&DATASTATE_TAG_1, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<DataState> for DataStateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DataState, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            DATASTATE_TAG_0 => Ok(DataState::AccountState),
            DATASTATE_TAG_1 => Ok(DataState::LedgerState),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExternalPluginAdapterSchema {
    /// The `Binary` variant.
    Binary,
    /// The `Json` variant.
    Json,
    /// The `MsgPack` variant.
    MsgPack,
}
/// Bounded codec for [`ExternalPluginAdapterSchema`].
#[derive(Clone, Copy, Debug)]
pub struct ExternalPluginAdapterSchemaCodec;
const EXTERNALPLUGINADAPTERSCHEMA_TAG_0: u8 = 0;
const EXTERNALPLUGINADAPTERSCHEMA_TAG_1: u8 = 1;
const EXTERNALPLUGINADAPTERSCHEMA_TAG_2: u8 = 2;
impl Encoder<ExternalPluginAdapterSchema> for ExternalPluginAdapterSchemaCodec {
    fn encoded_len(&self, value: &ExternalPluginAdapterSchema) -> Result<usize, Error> {
        let size = match value {
            ExternalPluginAdapterSchema::Binary => 0usize,
            ExternalPluginAdapterSchema::Json => 0usize,
            ExternalPluginAdapterSchema::MsgPack => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(
        &self,
        value: &ExternalPluginAdapterSchema,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            ExternalPluginAdapterSchema::Binary => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERSCHEMA_TAG_0, writer)?;
                Ok(())
            }
            ExternalPluginAdapterSchema::Json => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERSCHEMA_TAG_1, writer)?;
                Ok(())
            }
            ExternalPluginAdapterSchema::MsgPack => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERSCHEMA_TAG_2, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<ExternalPluginAdapterSchema> for ExternalPluginAdapterSchemaCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExternalPluginAdapterSchema, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            EXTERNALPLUGINADAPTERSCHEMA_TAG_0 => Ok(ExternalPluginAdapterSchema::Binary),
            EXTERNALPLUGINADAPTERSCHEMA_TAG_1 => Ok(ExternalPluginAdapterSchema::Json),
            EXTERNALPLUGINADAPTERSCHEMA_TAG_2 => Ok(ExternalPluginAdapterSchema::MsgPack),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExternalPluginAdapterType {
    /// The `LifecycleHook` variant.
    LifecycleHook,
    /// The `Oracle` variant.
    Oracle,
    /// The `AppData` variant.
    AppData,
    /// The `LinkedLifecycleHook` variant.
    LinkedLifecycleHook,
    /// The `LinkedAppData` variant.
    LinkedAppData,
    /// The `DataSection` variant.
    DataSection,
    /// The `AgentIdentity` variant.
    AgentIdentity,
}
/// Bounded codec for [`ExternalPluginAdapterType`].
#[derive(Clone, Copy, Debug)]
pub struct ExternalPluginAdapterTypeCodec;
const EXTERNALPLUGINADAPTERTYPE_TAG_0: u8 = 0;
const EXTERNALPLUGINADAPTERTYPE_TAG_1: u8 = 1;
const EXTERNALPLUGINADAPTERTYPE_TAG_2: u8 = 2;
const EXTERNALPLUGINADAPTERTYPE_TAG_3: u8 = 3;
const EXTERNALPLUGINADAPTERTYPE_TAG_4: u8 = 4;
const EXTERNALPLUGINADAPTERTYPE_TAG_5: u8 = 5;
const EXTERNALPLUGINADAPTERTYPE_TAG_6: u8 = 6;
impl Encoder<ExternalPluginAdapterType> for ExternalPluginAdapterTypeCodec {
    fn encoded_len(&self, value: &ExternalPluginAdapterType) -> Result<usize, Error> {
        let size = match value {
            ExternalPluginAdapterType::LifecycleHook => 0usize,
            ExternalPluginAdapterType::Oracle => 0usize,
            ExternalPluginAdapterType::AppData => 0usize,
            ExternalPluginAdapterType::LinkedLifecycleHook => 0usize,
            ExternalPluginAdapterType::LinkedAppData => 0usize,
            ExternalPluginAdapterType::DataSection => 0usize,
            ExternalPluginAdapterType::AgentIdentity => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(
        &self,
        value: &ExternalPluginAdapterType,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            ExternalPluginAdapterType::LifecycleHook => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERTYPE_TAG_0, writer)?;
                Ok(())
            }
            ExternalPluginAdapterType::Oracle => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERTYPE_TAG_1, writer)?;
                Ok(())
            }
            ExternalPluginAdapterType::AppData => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERTYPE_TAG_2, writer)?;
                Ok(())
            }
            ExternalPluginAdapterType::LinkedLifecycleHook => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERTYPE_TAG_3, writer)?;
                Ok(())
            }
            ExternalPluginAdapterType::LinkedAppData => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERTYPE_TAG_4, writer)?;
                Ok(())
            }
            ExternalPluginAdapterType::DataSection => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERTYPE_TAG_5, writer)?;
                Ok(())
            }
            ExternalPluginAdapterType::AgentIdentity => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERTYPE_TAG_6, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<ExternalPluginAdapterType> for ExternalPluginAdapterTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExternalPluginAdapterType, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            EXTERNALPLUGINADAPTERTYPE_TAG_0 => Ok(ExternalPluginAdapterType::LifecycleHook),
            EXTERNALPLUGINADAPTERTYPE_TAG_1 => Ok(ExternalPluginAdapterType::Oracle),
            EXTERNALPLUGINADAPTERTYPE_TAG_2 => Ok(ExternalPluginAdapterType::AppData),
            EXTERNALPLUGINADAPTERTYPE_TAG_3 => Ok(ExternalPluginAdapterType::LinkedLifecycleHook),
            EXTERNALPLUGINADAPTERTYPE_TAG_4 => Ok(ExternalPluginAdapterType::LinkedAppData),
            EXTERNALPLUGINADAPTERTYPE_TAG_5 => Ok(ExternalPluginAdapterType::DataSection),
            EXTERNALPLUGINADAPTERTYPE_TAG_6 => Ok(ExternalPluginAdapterType::AgentIdentity),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExternalValidationResult {
    /// The `Approved` variant.
    Approved,
    /// The `Rejected` variant.
    Rejected,
    /// The `Pass` variant.
    Pass,
}
/// Bounded codec for [`ExternalValidationResult`].
#[derive(Clone, Copy, Debug)]
pub struct ExternalValidationResultCodec;
const EXTERNALVALIDATIONRESULT_TAG_0: u8 = 0;
const EXTERNALVALIDATIONRESULT_TAG_1: u8 = 1;
const EXTERNALVALIDATIONRESULT_TAG_2: u8 = 2;
impl Encoder<ExternalValidationResult> for ExternalValidationResultCodec {
    fn encoded_len(&self, value: &ExternalValidationResult) -> Result<usize, Error> {
        let size = match value {
            ExternalValidationResult::Approved => 0usize,
            ExternalValidationResult::Rejected => 0usize,
            ExternalValidationResult::Pass => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(
        &self,
        value: &ExternalValidationResult,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            ExternalValidationResult::Approved => {
                Number::<u8>::LE.write(&EXTERNALVALIDATIONRESULT_TAG_0, writer)?;
                Ok(())
            }
            ExternalValidationResult::Rejected => {
                Number::<u8>::LE.write(&EXTERNALVALIDATIONRESULT_TAG_1, writer)?;
                Ok(())
            }
            ExternalValidationResult::Pass => {
                Number::<u8>::LE.write(&EXTERNALVALIDATIONRESULT_TAG_2, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<ExternalValidationResult> for ExternalValidationResultCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExternalValidationResult, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            EXTERNALVALIDATIONRESULT_TAG_0 => Ok(ExternalValidationResult::Approved),
            EXTERNALVALIDATIONRESULT_TAG_1 => Ok(ExternalValidationResult::Rejected),
            EXTERNALVALIDATIONRESULT_TAG_2 => Ok(ExternalValidationResult::Pass),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HookableLifecycleEvent {
    /// The `Create` variant.
    Create,
    /// The `Transfer` variant.
    Transfer,
    /// The `Burn` variant.
    Burn,
    /// The `Update` variant.
    Update,
    /// The `Execute` variant.
    Execute,
}
/// Bounded codec for [`HookableLifecycleEvent`].
#[derive(Clone, Copy, Debug)]
pub struct HookableLifecycleEventCodec;
const HOOKABLELIFECYCLEEVENT_TAG_0: u8 = 0;
const HOOKABLELIFECYCLEEVENT_TAG_1: u8 = 1;
const HOOKABLELIFECYCLEEVENT_TAG_2: u8 = 2;
const HOOKABLELIFECYCLEEVENT_TAG_3: u8 = 3;
const HOOKABLELIFECYCLEEVENT_TAG_4: u8 = 4;
impl Encoder<HookableLifecycleEvent> for HookableLifecycleEventCodec {
    fn encoded_len(&self, value: &HookableLifecycleEvent) -> Result<usize, Error> {
        let size = match value {
            HookableLifecycleEvent::Create => 0usize,
            HookableLifecycleEvent::Transfer => 0usize,
            HookableLifecycleEvent::Burn => 0usize,
            HookableLifecycleEvent::Update => 0usize,
            HookableLifecycleEvent::Execute => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &HookableLifecycleEvent, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            HookableLifecycleEvent::Create => {
                Number::<u8>::LE.write(&HOOKABLELIFECYCLEEVENT_TAG_0, writer)?;
                Ok(())
            }
            HookableLifecycleEvent::Transfer => {
                Number::<u8>::LE.write(&HOOKABLELIFECYCLEEVENT_TAG_1, writer)?;
                Ok(())
            }
            HookableLifecycleEvent::Burn => {
                Number::<u8>::LE.write(&HOOKABLELIFECYCLEEVENT_TAG_2, writer)?;
                Ok(())
            }
            HookableLifecycleEvent::Update => {
                Number::<u8>::LE.write(&HOOKABLELIFECYCLEEVENT_TAG_3, writer)?;
                Ok(())
            }
            HookableLifecycleEvent::Execute => {
                Number::<u8>::LE.write(&HOOKABLELIFECYCLEEVENT_TAG_4, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<HookableLifecycleEvent> for HookableLifecycleEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<HookableLifecycleEvent, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            HOOKABLELIFECYCLEEVENT_TAG_0 => Ok(HookableLifecycleEvent::Create),
            HOOKABLELIFECYCLEEVENT_TAG_1 => Ok(HookableLifecycleEvent::Transfer),
            HOOKABLELIFECYCLEEVENT_TAG_2 => Ok(HookableLifecycleEvent::Burn),
            HOOKABLELIFECYCLEEVENT_TAG_3 => Ok(HookableLifecycleEvent::Update),
            HOOKABLELIFECYCLEEVENT_TAG_4 => Ok(HookableLifecycleEvent::Execute),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Key {
    /// The `Uninitialized` variant.
    Uninitialized,
    /// The `AssetV1` variant.
    AssetV1,
    /// The `HashedAssetV1` variant.
    HashedAssetV1,
    /// The `PluginHeaderV1` variant.
    PluginHeaderV1,
    /// The `PluginRegistryV1` variant.
    PluginRegistryV1,
    /// The `CollectionV1` variant.
    CollectionV1,
    /// The `GroupV1` variant.
    GroupV1,
}
/// Bounded codec for [`Key`].
#[derive(Clone, Copy, Debug)]
pub struct KeyCodec;
const KEY_TAG_0: u8 = 0;
const KEY_TAG_1: u8 = 1;
const KEY_TAG_2: u8 = 2;
const KEY_TAG_3: u8 = 3;
const KEY_TAG_4: u8 = 4;
const KEY_TAG_5: u8 = 5;
const KEY_TAG_6: u8 = 6;
impl Encoder<Key> for KeyCodec {
    fn encoded_len(&self, value: &Key) -> Result<usize, Error> {
        let size = match value {
            Key::Uninitialized => 0usize,
            Key::AssetV1 => 0usize,
            Key::HashedAssetV1 => 0usize,
            Key::PluginHeaderV1 => 0usize,
            Key::PluginRegistryV1 => 0usize,
            Key::CollectionV1 => 0usize,
            Key::GroupV1 => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &Key, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            Key::Uninitialized => {
                Number::<u8>::LE.write(&KEY_TAG_0, writer)?;
                Ok(())
            }
            Key::AssetV1 => {
                Number::<u8>::LE.write(&KEY_TAG_1, writer)?;
                Ok(())
            }
            Key::HashedAssetV1 => {
                Number::<u8>::LE.write(&KEY_TAG_2, writer)?;
                Ok(())
            }
            Key::PluginHeaderV1 => {
                Number::<u8>::LE.write(&KEY_TAG_3, writer)?;
                Ok(())
            }
            Key::PluginRegistryV1 => {
                Number::<u8>::LE.write(&KEY_TAG_4, writer)?;
                Ok(())
            }
            Key::CollectionV1 => {
                Number::<u8>::LE.write(&KEY_TAG_5, writer)?;
                Ok(())
            }
            Key::GroupV1 => {
                Number::<u8>::LE.write(&KEY_TAG_6, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<Key> for KeyCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Key, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            KEY_TAG_0 => Ok(Key::Uninitialized),
            KEY_TAG_1 => Ok(Key::AssetV1),
            KEY_TAG_2 => Ok(Key::HashedAssetV1),
            KEY_TAG_3 => Ok(Key::PluginHeaderV1),
            KEY_TAG_4 => Ok(Key::PluginRegistryV1),
            KEY_TAG_5 => Ok(Key::CollectionV1),
            KEY_TAG_6 => Ok(Key::GroupV1),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PluginType {
    /// The `Royalties` variant.
    Royalties,
    /// The `FreezeDelegate` variant.
    FreezeDelegate,
    /// The `BurnDelegate` variant.
    BurnDelegate,
    /// The `TransferDelegate` variant.
    TransferDelegate,
    /// The `UpdateDelegate` variant.
    UpdateDelegate,
    /// The `PermanentFreezeDelegate` variant.
    PermanentFreezeDelegate,
    /// The `Attributes` variant.
    Attributes,
    /// The `PermanentTransferDelegate` variant.
    PermanentTransferDelegate,
    /// The `PermanentBurnDelegate` variant.
    PermanentBurnDelegate,
    /// The `Edition` variant.
    Edition,
    /// The `MasterEdition` variant.
    MasterEdition,
    /// The `AddBlocker` variant.
    AddBlocker,
    /// The `ImmutableMetadata` variant.
    ImmutableMetadata,
    /// The `VerifiedCreators` variant.
    VerifiedCreators,
    /// The `Autograph` variant.
    Autograph,
    /// The `BubblegumV2` variant.
    BubblegumV2,
    /// The `FreezeExecute` variant.
    FreezeExecute,
    /// The `PermanentFreezeExecute` variant.
    PermanentFreezeExecute,
    /// The `Groups` variant.
    Groups,
}
/// Bounded codec for [`PluginType`].
#[derive(Clone, Copy, Debug)]
pub struct PluginTypeCodec;
const PLUGINTYPE_TAG_0: u8 = 0;
const PLUGINTYPE_TAG_1: u8 = 1;
const PLUGINTYPE_TAG_2: u8 = 2;
const PLUGINTYPE_TAG_3: u8 = 3;
const PLUGINTYPE_TAG_4: u8 = 4;
const PLUGINTYPE_TAG_5: u8 = 5;
const PLUGINTYPE_TAG_6: u8 = 6;
const PLUGINTYPE_TAG_7: u8 = 7;
const PLUGINTYPE_TAG_8: u8 = 8;
const PLUGINTYPE_TAG_9: u8 = 9;
const PLUGINTYPE_TAG_10: u8 = 10;
const PLUGINTYPE_TAG_11: u8 = 11;
const PLUGINTYPE_TAG_12: u8 = 12;
const PLUGINTYPE_TAG_13: u8 = 13;
const PLUGINTYPE_TAG_14: u8 = 14;
const PLUGINTYPE_TAG_15: u8 = 15;
const PLUGINTYPE_TAG_16: u8 = 16;
const PLUGINTYPE_TAG_17: u8 = 17;
const PLUGINTYPE_TAG_18: u8 = 18;
impl Encoder<PluginType> for PluginTypeCodec {
    fn encoded_len(&self, value: &PluginType) -> Result<usize, Error> {
        let size = match value {
            PluginType::Royalties => 0usize,
            PluginType::FreezeDelegate => 0usize,
            PluginType::BurnDelegate => 0usize,
            PluginType::TransferDelegate => 0usize,
            PluginType::UpdateDelegate => 0usize,
            PluginType::PermanentFreezeDelegate => 0usize,
            PluginType::Attributes => 0usize,
            PluginType::PermanentTransferDelegate => 0usize,
            PluginType::PermanentBurnDelegate => 0usize,
            PluginType::Edition => 0usize,
            PluginType::MasterEdition => 0usize,
            PluginType::AddBlocker => 0usize,
            PluginType::ImmutableMetadata => 0usize,
            PluginType::VerifiedCreators => 0usize,
            PluginType::Autograph => 0usize,
            PluginType::BubblegumV2 => 0usize,
            PluginType::FreezeExecute => 0usize,
            PluginType::PermanentFreezeExecute => 0usize,
            PluginType::Groups => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &PluginType, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            PluginType::Royalties => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_0, writer)?;
                Ok(())
            }
            PluginType::FreezeDelegate => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_1, writer)?;
                Ok(())
            }
            PluginType::BurnDelegate => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_2, writer)?;
                Ok(())
            }
            PluginType::TransferDelegate => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_3, writer)?;
                Ok(())
            }
            PluginType::UpdateDelegate => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_4, writer)?;
                Ok(())
            }
            PluginType::PermanentFreezeDelegate => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_5, writer)?;
                Ok(())
            }
            PluginType::Attributes => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_6, writer)?;
                Ok(())
            }
            PluginType::PermanentTransferDelegate => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_7, writer)?;
                Ok(())
            }
            PluginType::PermanentBurnDelegate => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_8, writer)?;
                Ok(())
            }
            PluginType::Edition => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_9, writer)?;
                Ok(())
            }
            PluginType::MasterEdition => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_10, writer)?;
                Ok(())
            }
            PluginType::AddBlocker => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_11, writer)?;
                Ok(())
            }
            PluginType::ImmutableMetadata => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_12, writer)?;
                Ok(())
            }
            PluginType::VerifiedCreators => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_13, writer)?;
                Ok(())
            }
            PluginType::Autograph => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_14, writer)?;
                Ok(())
            }
            PluginType::BubblegumV2 => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_15, writer)?;
                Ok(())
            }
            PluginType::FreezeExecute => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_16, writer)?;
                Ok(())
            }
            PluginType::PermanentFreezeExecute => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_17, writer)?;
                Ok(())
            }
            PluginType::Groups => {
                Number::<u8>::LE.write(&PLUGINTYPE_TAG_18, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<PluginType> for PluginTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PluginType, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            PLUGINTYPE_TAG_0 => Ok(PluginType::Royalties),
            PLUGINTYPE_TAG_1 => Ok(PluginType::FreezeDelegate),
            PLUGINTYPE_TAG_2 => Ok(PluginType::BurnDelegate),
            PLUGINTYPE_TAG_3 => Ok(PluginType::TransferDelegate),
            PLUGINTYPE_TAG_4 => Ok(PluginType::UpdateDelegate),
            PLUGINTYPE_TAG_5 => Ok(PluginType::PermanentFreezeDelegate),
            PLUGINTYPE_TAG_6 => Ok(PluginType::Attributes),
            PLUGINTYPE_TAG_7 => Ok(PluginType::PermanentTransferDelegate),
            PLUGINTYPE_TAG_8 => Ok(PluginType::PermanentBurnDelegate),
            PLUGINTYPE_TAG_9 => Ok(PluginType::Edition),
            PLUGINTYPE_TAG_10 => Ok(PluginType::MasterEdition),
            PLUGINTYPE_TAG_11 => Ok(PluginType::AddBlocker),
            PLUGINTYPE_TAG_12 => Ok(PluginType::ImmutableMetadata),
            PLUGINTYPE_TAG_13 => Ok(PluginType::VerifiedCreators),
            PLUGINTYPE_TAG_14 => Ok(PluginType::Autograph),
            PLUGINTYPE_TAG_15 => Ok(PluginType::BubblegumV2),
            PLUGINTYPE_TAG_16 => Ok(PluginType::FreezeExecute),
            PLUGINTYPE_TAG_17 => Ok(PluginType::PermanentFreezeExecute),
            PLUGINTYPE_TAG_18 => Ok(PluginType::Groups),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationshipKind {
    /// The `Collection` variant.
    Collection,
    /// The `ChildGroup` variant.
    ChildGroup,
    /// The `ParentGroup` variant.
    ParentGroup,
    /// The `Asset` variant.
    Asset,
}
/// Bounded codec for [`RelationshipKind`].
#[derive(Clone, Copy, Debug)]
pub struct RelationshipKindCodec;
const RELATIONSHIPKIND_TAG_0: u8 = 0;
const RELATIONSHIPKIND_TAG_1: u8 = 1;
const RELATIONSHIPKIND_TAG_2: u8 = 2;
const RELATIONSHIPKIND_TAG_3: u8 = 3;
impl Encoder<RelationshipKind> for RelationshipKindCodec {
    fn encoded_len(&self, value: &RelationshipKind) -> Result<usize, Error> {
        let size = match value {
            RelationshipKind::Collection => 0usize,
            RelationshipKind::ChildGroup => 0usize,
            RelationshipKind::ParentGroup => 0usize,
            RelationshipKind::Asset => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &RelationshipKind, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            RelationshipKind::Collection => {
                Number::<u8>::LE.write(&RELATIONSHIPKIND_TAG_0, writer)?;
                Ok(())
            }
            RelationshipKind::ChildGroup => {
                Number::<u8>::LE.write(&RELATIONSHIPKIND_TAG_1, writer)?;
                Ok(())
            }
            RelationshipKind::ParentGroup => {
                Number::<u8>::LE.write(&RELATIONSHIPKIND_TAG_2, writer)?;
                Ok(())
            }
            RelationshipKind::Asset => {
                Number::<u8>::LE.write(&RELATIONSHIPKIND_TAG_3, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<RelationshipKind> for RelationshipKindCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RelationshipKind, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            RELATIONSHIPKIND_TAG_0 => Ok(RelationshipKind::Collection),
            RELATIONSHIPKIND_TAG_1 => Ok(RelationshipKind::ChildGroup),
            RELATIONSHIPKIND_TAG_2 => Ok(RelationshipKind::ParentGroup),
            RELATIONSHIPKIND_TAG_3 => Ok(RelationshipKind::Asset),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpdateType {
    /// The `Mint` variant.
    Mint,
    /// The `Add` variant.
    Add,
    /// The `Remove` variant.
    Remove,
}
/// Bounded codec for [`UpdateType`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateTypeCodec;
const UPDATETYPE_TAG_0: u8 = 0;
const UPDATETYPE_TAG_1: u8 = 1;
const UPDATETYPE_TAG_2: u8 = 2;
impl Encoder<UpdateType> for UpdateTypeCodec {
    fn encoded_len(&self, value: &UpdateType) -> Result<usize, Error> {
        let size = match value {
            UpdateType::Mint => 0usize,
            UpdateType::Add => 0usize,
            UpdateType::Remove => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &UpdateType, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            UpdateType::Mint => {
                Number::<u8>::LE.write(&UPDATETYPE_TAG_0, writer)?;
                Ok(())
            }
            UpdateType::Add => {
                Number::<u8>::LE.write(&UPDATETYPE_TAG_1, writer)?;
                Ok(())
            }
            UpdateType::Remove => {
                Number::<u8>::LE.write(&UPDATETYPE_TAG_2, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<UpdateType> for UpdateTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateType, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            UPDATETYPE_TAG_0 => Ok(UpdateType::Mint),
            UPDATETYPE_TAG_1 => Ok(UpdateType::Add),
            UPDATETYPE_TAG_2 => Ok(UpdateType::Remove),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationResult {
    /// The `Approved` variant.
    Approved,
    /// The `Rejected` variant.
    Rejected,
    /// The `Pass` variant.
    Pass,
    /// The `ForceApproved` variant.
    ForceApproved,
}
/// Bounded codec for [`ValidationResult`].
#[derive(Clone, Copy, Debug)]
pub struct ValidationResultCodec;
const VALIDATIONRESULT_TAG_0: u8 = 0;
const VALIDATIONRESULT_TAG_1: u8 = 1;
const VALIDATIONRESULT_TAG_2: u8 = 2;
const VALIDATIONRESULT_TAG_3: u8 = 3;
impl Encoder<ValidationResult> for ValidationResultCodec {
    fn encoded_len(&self, value: &ValidationResult) -> Result<usize, Error> {
        let size = match value {
            ValidationResult::Approved => 0usize,
            ValidationResult::Rejected => 0usize,
            ValidationResult::Pass => 0usize,
            ValidationResult::ForceApproved => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &ValidationResult, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            ValidationResult::Approved => {
                Number::<u8>::LE.write(&VALIDATIONRESULT_TAG_0, writer)?;
                Ok(())
            }
            ValidationResult::Rejected => {
                Number::<u8>::LE.write(&VALIDATIONRESULT_TAG_1, writer)?;
                Ok(())
            }
            ValidationResult::Pass => {
                Number::<u8>::LE.write(&VALIDATIONRESULT_TAG_2, writer)?;
                Ok(())
            }
            ValidationResult::ForceApproved => {
                Number::<u8>::LE.write(&VALIDATIONRESULT_TAG_3, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<ValidationResult> for ValidationResultCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ValidationResult, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            VALIDATIONRESULT_TAG_0 => Ok(ValidationResult::Approved),
            VALIDATIONRESULT_TAG_1 => Ok(ValidationResult::Rejected),
            VALIDATIONRESULT_TAG_2 => Ok(ValidationResult::Pass),
            VALIDATIONRESULT_TAG_3 => Ok(ValidationResult::ForceApproved),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `AuthorityAddressPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityAddressPayload {
    /// The `address` wire field.
    pub address: Address,
}
/// Bounded codec for [`AuthorityAddressPayload`].
#[derive(Clone, Copy, Debug)]
pub struct AuthorityAddressPayloadCodec;
impl Encoder<AuthorityAddressPayload> for AuthorityAddressPayloadCodec {
    fn encoded_len(&self, value: &AuthorityAddressPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.address)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AuthorityAddressPayload, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.address, writer)?;
        Ok(())
    }
}
impl Decoder<AuthorityAddressPayload> for AuthorityAddressPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AuthorityAddressPayload, Error> {
        Ok(AuthorityAddressPayload {
            address: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Authority {
    /// The `None` variant.
    None,
    /// The `Owner` variant.
    Owner,
    /// The `UpdateAuthority` variant.
    UpdateAuthority,
    /// The `Address` variant.
    Address(AuthorityAddressPayload),
}
/// Bounded codec for [`Authority`].
#[derive(Clone, Copy, Debug)]
pub struct AuthorityCodec;
const AUTHORITY_TAG_0: u8 = 0;
const AUTHORITY_TAG_1: u8 = 1;
const AUTHORITY_TAG_2: u8 = 2;
const AUTHORITY_TAG_3: u8 = 3;
impl Encoder<Authority> for AuthorityCodec {
    fn encoded_len(&self, value: &Authority) -> Result<usize, Error> {
        let size = match value {
            Authority::None => 0usize,
            Authority::Owner => 0usize,
            Authority::UpdateAuthority => 0usize,
            Authority::Address(value) => AuthorityAddressPayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &Authority, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            Authority::None => {
                Number::<u8>::LE.write(&AUTHORITY_TAG_0, writer)?;
                Ok(())
            }
            Authority::Owner => {
                Number::<u8>::LE.write(&AUTHORITY_TAG_1, writer)?;
                Ok(())
            }
            Authority::UpdateAuthority => {
                Number::<u8>::LE.write(&AUTHORITY_TAG_2, writer)?;
                Ok(())
            }
            Authority::Address(value) => {
                Number::<u8>::LE.write(&AUTHORITY_TAG_3, writer)?;
                AuthorityAddressPayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<Authority> for AuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Authority, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            AUTHORITY_TAG_0 => Ok(Authority::None),
            AUTHORITY_TAG_1 => Ok(Authority::Owner),
            AUTHORITY_TAG_2 => Ok(Authority::UpdateAuthority),
            AUTHORITY_TAG_3 => Ok(Authority::Address(
                AuthorityAddressPayloadCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpdateAuthority {
    /// The `None` variant.
    None,
    /// The `Address` variant.
    Address(Address),
    /// The `Collection` variant.
    Collection(Address),
}
/// Bounded codec for [`UpdateAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateAuthorityCodec;
const UPDATEAUTHORITY_TAG_0: u8 = 0;
const UPDATEAUTHORITY_TAG_1: u8 = 1;
const UPDATEAUTHORITY_TAG_2: u8 = 2;
impl Encoder<UpdateAuthority> for UpdateAuthorityCodec {
    fn encoded_len(&self, value: &UpdateAuthority) -> Result<usize, Error> {
        let size = match value {
            UpdateAuthority::None => 0usize,
            UpdateAuthority::Address(value) => Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .encoded_len(value)?,
            UpdateAuthority::Collection(value) => Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &UpdateAuthority, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            UpdateAuthority::None => {
                Number::<u8>::LE.write(&UPDATEAUTHORITY_TAG_0, writer)?;
                Ok(())
            }
            UpdateAuthority::Address(value) => {
                Number::<u8>::LE.write(&UPDATEAUTHORITY_TAG_1, writer)?;
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .write(value, writer)
            }
            UpdateAuthority::Collection(value) => {
                Number::<u8>::LE.write(&UPDATEAUTHORITY_TAG_2, writer)?;
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .write(value, writer)
            }
        }
    }
}
impl Decoder<UpdateAuthority> for UpdateAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateAuthority, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            UPDATEAUTHORITY_TAG_0 => Ok(UpdateAuthority::None),
            UPDATEAUTHORITY_TAG_1 => Ok(UpdateAuthority::Address(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .read(reader)?,
            )),
            UPDATEAUTHORITY_TAG_2 => Ok(UpdateAuthority::Collection(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleSet {
    /// The `None` variant.
    None,
    /// The `ProgramAllowList` variant.
    ProgramAllowList(Vec<Address>),
    /// The `ProgramDenyList` variant.
    ProgramDenyList(Vec<Address>),
}
/// Bounded codec for [`RuleSet`].
#[derive(Clone, Copy, Debug)]
pub struct RuleSetCodec;
const RULESET_TAG_0: u8 = 0;
const RULESET_TAG_1: u8 = 1;
const RULESET_TAG_2: u8 = 2;
impl Encoder<RuleSet> for RuleSetCodec {
    fn encoded_len(&self, value: &RuleSet) -> Result<usize, Error> {
        let size = match value {
            RuleSet::None => 0usize,
            RuleSet::ProgramAllowList(value) => CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .encoded_len(value.as_slice())?,
            RuleSet::ProgramDenyList(value) => CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .encoded_len(value.as_slice())?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &RuleSet, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            RuleSet::None => {
                Number::<u8>::LE.write(&RULESET_TAG_0, writer)?;
                Ok(())
            }
            RuleSet::ProgramAllowList(value) => {
                Number::<u8>::LE.write(&RULESET_TAG_1, writer)?;
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    minimum: 32,
                }
                .write(value.as_slice(), writer)
            }
            RuleSet::ProgramDenyList(value) => {
                Number::<u8>::LE.write(&RULESET_TAG_2, writer)?;
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    minimum: 32,
                }
                .write(value.as_slice(), writer)
            }
        }
    }
}
impl Decoder<RuleSet> for RuleSetCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RuleSet, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            RULESET_TAG_0 => Ok(RuleSet::None),
            RULESET_TAG_1 => Ok(RuleSet::ProgramAllowList(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    minimum: 32,
                }
                .read(reader)?,
            )),
            RULESET_TAG_2 => Ok(RuleSet::ProgramDenyList(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    minimum: 32,
                }
                .read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Seed {
    /// The `Collection` variant.
    Collection,
    /// The `Owner` variant.
    Owner,
    /// The `Recipient` variant.
    Recipient,
    /// The `Asset` variant.
    Asset,
    /// The `Address` variant.
    Address(Address),
    /// The `Bytes` variant.
    Bytes(Vec<u8>),
}
/// Bounded codec for [`Seed`].
#[derive(Clone, Copy, Debug)]
pub struct SeedCodec;
const SEED_TAG_0: u8 = 0;
const SEED_TAG_1: u8 = 1;
const SEED_TAG_2: u8 = 2;
const SEED_TAG_3: u8 = 3;
const SEED_TAG_4: u8 = 4;
const SEED_TAG_5: u8 = 5;
impl Encoder<Seed> for SeedCodec {
    fn encoded_len(&self, value: &Seed) -> Result<usize, Error> {
        let size = match value {
            Seed::Collection => 0usize,
            Seed::Owner => 0usize,
            Seed::Recipient => 0usize,
            Seed::Asset => 0usize,
            Seed::Address(value) => Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .encoded_len(value)?,
            Seed::Bytes(value) => CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .encoded_len(value.as_slice())?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &Seed, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            Seed::Collection => {
                Number::<u8>::LE.write(&SEED_TAG_0, writer)?;
                Ok(())
            }
            Seed::Owner => {
                Number::<u8>::LE.write(&SEED_TAG_1, writer)?;
                Ok(())
            }
            Seed::Recipient => {
                Number::<u8>::LE.write(&SEED_TAG_2, writer)?;
                Ok(())
            }
            Seed::Asset => {
                Number::<u8>::LE.write(&SEED_TAG_3, writer)?;
                Ok(())
            }
            Seed::Address(value) => {
                Number::<u8>::LE.write(&SEED_TAG_4, writer)?;
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .write(value, writer)
            }
            Seed::Bytes(value) => {
                Number::<u8>::LE.write(&SEED_TAG_5, writer)?;
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .write(value.as_slice(), writer)
            }
        }
    }
}
impl Decoder<Seed> for SeedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Seed, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            SEED_TAG_0 => Ok(Seed::Collection),
            SEED_TAG_1 => Ok(Seed::Owner),
            SEED_TAG_2 => Ok(Seed::Recipient),
            SEED_TAG_3 => Ok(Seed::Asset),
            SEED_TAG_4 => Ok(Seed::Address(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .read(reader)?,
            )),
            SEED_TAG_5 => Ok(Seed::Bytes(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationResultsOffset {
    /// The `NoOffset` variant.
    NoOffset,
    /// The `Anchor` variant.
    Anchor,
    /// The `Custom` variant.
    Custom(u64),
}
/// Bounded codec for [`ValidationResultsOffset`].
#[derive(Clone, Copy, Debug)]
pub struct ValidationResultsOffsetCodec;
const VALIDATIONRESULTSOFFSET_TAG_0: u8 = 0;
const VALIDATIONRESULTSOFFSET_TAG_1: u8 = 1;
const VALIDATIONRESULTSOFFSET_TAG_2: u8 = 2;
impl Encoder<ValidationResultsOffset> for ValidationResultsOffsetCodec {
    fn encoded_len(&self, value: &ValidationResultsOffset) -> Result<usize, Error> {
        let size = match value {
            ValidationResultsOffset::NoOffset => 0usize,
            ValidationResultsOffset::Anchor => 0usize,
            ValidationResultsOffset::Custom(value) => Number::<u64>::LE.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &ValidationResultsOffset, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            ValidationResultsOffset::NoOffset => {
                Number::<u8>::LE.write(&VALIDATIONRESULTSOFFSET_TAG_0, writer)?;
                Ok(())
            }
            ValidationResultsOffset::Anchor => {
                Number::<u8>::LE.write(&VALIDATIONRESULTSOFFSET_TAG_1, writer)?;
                Ok(())
            }
            ValidationResultsOffset::Custom(value) => {
                Number::<u8>::LE.write(&VALIDATIONRESULTSOFFSET_TAG_2, writer)?;
                Number::<u64>::LE.write(value, writer)
            }
        }
    }
}
impl Decoder<ValidationResultsOffset> for ValidationResultsOffsetCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ValidationResultsOffset, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            VALIDATIONRESULTSOFFSET_TAG_0 => Ok(ValidationResultsOffset::NoOffset),
            VALIDATIONRESULTSOFFSET_TAG_1 => Ok(ValidationResultsOffset::Anchor),
            VALIDATIONRESULTSOFFSET_TAG_2 => Ok(ValidationResultsOffset::Custom(
                Number::<u64>::LE.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinkedDataKey {
    /// The `LinkedLifecycleHook` variant.
    LinkedLifecycleHook(Address),
    /// The `LinkedAppData` variant.
    LinkedAppData(Authority),
}
/// Bounded codec for [`LinkedDataKey`].
#[derive(Clone, Copy, Debug)]
pub struct LinkedDataKeyCodec;
const LINKEDDATAKEY_TAG_0: u8 = 0;
const LINKEDDATAKEY_TAG_1: u8 = 1;
impl Encoder<LinkedDataKey> for LinkedDataKeyCodec {
    fn encoded_len(&self, value: &LinkedDataKey) -> Result<usize, Error> {
        let size = match value {
            LinkedDataKey::LinkedLifecycleHook(value) => Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .encoded_len(value)?,
            LinkedDataKey::LinkedAppData(value) => AuthorityCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &LinkedDataKey, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            LinkedDataKey::LinkedLifecycleHook(value) => {
                Number::<u8>::LE.write(&LINKEDDATAKEY_TAG_0, writer)?;
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .write(value, writer)
            }
            LinkedDataKey::LinkedAppData(value) => {
                Number::<u8>::LE.write(&LINKEDDATAKEY_TAG_1, writer)?;
                AuthorityCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<LinkedDataKey> for LinkedDataKeyCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LinkedDataKey, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            LINKEDDATAKEY_TAG_0 => Ok(LinkedDataKey::LinkedLifecycleHook(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .read(reader)?,
            )),
            LINKEDDATAKEY_TAG_1 => Ok(LinkedDataKey::LinkedAppData(AuthorityCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `OracleValidationV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OracleValidationV1Payload {
    /// The `create` wire field.
    pub create: ExternalValidationResult,
    /// The `transfer` wire field.
    pub transfer: ExternalValidationResult,
    /// The `burn` wire field.
    pub burn: ExternalValidationResult,
    /// The `update` wire field.
    pub update: ExternalValidationResult,
}
/// Bounded codec for [`OracleValidationV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct OracleValidationV1PayloadCodec;
impl Encoder<OracleValidationV1Payload> for OracleValidationV1PayloadCodec {
    fn encoded_len(&self, value: &OracleValidationV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(ExternalValidationResultCodec.encoded_len(&value.create)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalValidationResultCodec.encoded_len(&value.transfer)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalValidationResultCodec.encoded_len(&value.burn)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalValidationResultCodec.encoded_len(&value.update)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &OracleValidationV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        ExternalValidationResultCodec.write(&value.create, writer)?;
        ExternalValidationResultCodec.write(&value.transfer, writer)?;
        ExternalValidationResultCodec.write(&value.burn, writer)?;
        ExternalValidationResultCodec.write(&value.update, writer)?;
        Ok(())
    }
}
impl Decoder<OracleValidationV1Payload> for OracleValidationV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<OracleValidationV1Payload, Error> {
        Ok(OracleValidationV1Payload {
            create: ExternalValidationResultCodec.read(reader)?,
            transfer: ExternalValidationResultCodec.read(reader)?,
            burn: ExternalValidationResultCodec.read(reader)?,
            update: ExternalValidationResultCodec.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OracleValidation {
    /// The `Uninitialized` variant.
    Uninitialized,
    /// The `V1` variant.
    V1(OracleValidationV1Payload),
}
/// Bounded codec for [`OracleValidation`].
#[derive(Clone, Copy, Debug)]
pub struct OracleValidationCodec;
const ORACLEVALIDATION_TAG_0: u8 = 0;
const ORACLEVALIDATION_TAG_1: u8 = 1;
impl Encoder<OracleValidation> for OracleValidationCodec {
    fn encoded_len(&self, value: &OracleValidation) -> Result<usize, Error> {
        let size = match value {
            OracleValidation::Uninitialized => 0usize,
            OracleValidation::V1(value) => OracleValidationV1PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &OracleValidation, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            OracleValidation::Uninitialized => {
                Number::<u8>::LE.write(&ORACLEVALIDATION_TAG_0, writer)?;
                Ok(())
            }
            OracleValidation::V1(value) => {
                Number::<u8>::LE.write(&ORACLEVALIDATION_TAG_1, writer)?;
                OracleValidationV1PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<OracleValidation> for OracleValidationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<OracleValidation, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            ORACLEVALIDATION_TAG_0 => Ok(OracleValidation::Uninitialized),
            ORACLEVALIDATION_TAG_1 => Ok(OracleValidation::V1(
                OracleValidationV1PayloadCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `ExtraAccountPreconfiguredProgramPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtraAccountPreconfiguredProgramPayload {
    /// The `is_signer` wire field.
    pub is_signer: bool,
    /// The `is_writable` wire field.
    pub is_writable: bool,
}
/// Bounded codec for [`ExtraAccountPreconfiguredProgramPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtraAccountPreconfiguredProgramPayloadCodec;
impl Encoder<ExtraAccountPreconfiguredProgramPayload>
    for ExtraAccountPreconfiguredProgramPayloadCodec
{
    fn encoded_len(&self, value: &ExtraAccountPreconfiguredProgramPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_signer)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_writable)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtraAccountPreconfiguredProgramPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Bool.write(&value.is_signer, writer)?;
        Bool.write(&value.is_writable, writer)?;
        Ok(())
    }
}
impl Decoder<ExtraAccountPreconfiguredProgramPayload>
    for ExtraAccountPreconfiguredProgramPayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ExtraAccountPreconfiguredProgramPayload, Error> {
        Ok(ExtraAccountPreconfiguredProgramPayload {
            is_signer: Bool.read(reader)?,
            is_writable: Bool.read(reader)?,
        })
    }
}
/// Validated data for `ExtraAccountPreconfiguredCollectionPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtraAccountPreconfiguredCollectionPayload {
    /// The `is_signer` wire field.
    pub is_signer: bool,
    /// The `is_writable` wire field.
    pub is_writable: bool,
}
/// Bounded codec for [`ExtraAccountPreconfiguredCollectionPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtraAccountPreconfiguredCollectionPayloadCodec;
impl Encoder<ExtraAccountPreconfiguredCollectionPayload>
    for ExtraAccountPreconfiguredCollectionPayloadCodec
{
    fn encoded_len(
        &self,
        value: &ExtraAccountPreconfiguredCollectionPayload,
    ) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_signer)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_writable)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtraAccountPreconfiguredCollectionPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Bool.write(&value.is_signer, writer)?;
        Bool.write(&value.is_writable, writer)?;
        Ok(())
    }
}
impl Decoder<ExtraAccountPreconfiguredCollectionPayload>
    for ExtraAccountPreconfiguredCollectionPayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ExtraAccountPreconfiguredCollectionPayload, Error> {
        Ok(ExtraAccountPreconfiguredCollectionPayload {
            is_signer: Bool.read(reader)?,
            is_writable: Bool.read(reader)?,
        })
    }
}
/// Validated data for `ExtraAccountPreconfiguredOwnerPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtraAccountPreconfiguredOwnerPayload {
    /// The `is_signer` wire field.
    pub is_signer: bool,
    /// The `is_writable` wire field.
    pub is_writable: bool,
}
/// Bounded codec for [`ExtraAccountPreconfiguredOwnerPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtraAccountPreconfiguredOwnerPayloadCodec;
impl Encoder<ExtraAccountPreconfiguredOwnerPayload> for ExtraAccountPreconfiguredOwnerPayloadCodec {
    fn encoded_len(&self, value: &ExtraAccountPreconfiguredOwnerPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_signer)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_writable)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtraAccountPreconfiguredOwnerPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Bool.write(&value.is_signer, writer)?;
        Bool.write(&value.is_writable, writer)?;
        Ok(())
    }
}
impl Decoder<ExtraAccountPreconfiguredOwnerPayload> for ExtraAccountPreconfiguredOwnerPayloadCodec {
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ExtraAccountPreconfiguredOwnerPayload, Error> {
        Ok(ExtraAccountPreconfiguredOwnerPayload {
            is_signer: Bool.read(reader)?,
            is_writable: Bool.read(reader)?,
        })
    }
}
/// Validated data for `ExtraAccountPreconfiguredRecipientPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtraAccountPreconfiguredRecipientPayload {
    /// The `is_signer` wire field.
    pub is_signer: bool,
    /// The `is_writable` wire field.
    pub is_writable: bool,
}
/// Bounded codec for [`ExtraAccountPreconfiguredRecipientPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtraAccountPreconfiguredRecipientPayloadCodec;
impl Encoder<ExtraAccountPreconfiguredRecipientPayload>
    for ExtraAccountPreconfiguredRecipientPayloadCodec
{
    fn encoded_len(
        &self,
        value: &ExtraAccountPreconfiguredRecipientPayload,
    ) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_signer)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_writable)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtraAccountPreconfiguredRecipientPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Bool.write(&value.is_signer, writer)?;
        Bool.write(&value.is_writable, writer)?;
        Ok(())
    }
}
impl Decoder<ExtraAccountPreconfiguredRecipientPayload>
    for ExtraAccountPreconfiguredRecipientPayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ExtraAccountPreconfiguredRecipientPayload, Error> {
        Ok(ExtraAccountPreconfiguredRecipientPayload {
            is_signer: Bool.read(reader)?,
            is_writable: Bool.read(reader)?,
        })
    }
}
/// Validated data for `ExtraAccountPreconfiguredAssetPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtraAccountPreconfiguredAssetPayload {
    /// The `is_signer` wire field.
    pub is_signer: bool,
    /// The `is_writable` wire field.
    pub is_writable: bool,
}
/// Bounded codec for [`ExtraAccountPreconfiguredAssetPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtraAccountPreconfiguredAssetPayloadCodec;
impl Encoder<ExtraAccountPreconfiguredAssetPayload> for ExtraAccountPreconfiguredAssetPayloadCodec {
    fn encoded_len(&self, value: &ExtraAccountPreconfiguredAssetPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_signer)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_writable)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtraAccountPreconfiguredAssetPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Bool.write(&value.is_signer, writer)?;
        Bool.write(&value.is_writable, writer)?;
        Ok(())
    }
}
impl Decoder<ExtraAccountPreconfiguredAssetPayload> for ExtraAccountPreconfiguredAssetPayloadCodec {
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ExtraAccountPreconfiguredAssetPayload, Error> {
        Ok(ExtraAccountPreconfiguredAssetPayload {
            is_signer: Bool.read(reader)?,
            is_writable: Bool.read(reader)?,
        })
    }
}
/// Validated data for `ExtraAccountCustomPdaPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtraAccountCustomPdaPayload {
    /// The `seeds` wire field.
    pub seeds: Vec<Seed>,
    /// The `custom_program_id` wire field.
    pub custom_program_id: Option<Address>,
    /// The `is_signer` wire field.
    pub is_signer: bool,
    /// The `is_writable` wire field.
    pub is_writable: bool,
}
/// Bounded codec for [`ExtraAccountCustomPdaPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtraAccountCustomPdaPayloadCodec;
impl Encoder<ExtraAccountCustomPdaPayload> for ExtraAccountCustomPdaPayloadCodec {
    fn encoded_len(&self, value: &ExtraAccountCustomPdaPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: SeedCodec,
                    minimum: 1,
                }
                .encoded_len(value.seeds.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.custom_program_id)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_signer)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_writable)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtraAccountCustomPdaPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: SeedCodec,
            minimum: 1,
        }
        .write(value.seeds.as_slice(), writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.custom_program_id, writer)?;
        Bool.write(&value.is_signer, writer)?;
        Bool.write(&value.is_writable, writer)?;
        Ok(())
    }
}
impl Decoder<ExtraAccountCustomPdaPayload> for ExtraAccountCustomPdaPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtraAccountCustomPdaPayload, Error> {
        Ok(ExtraAccountCustomPdaPayload {
            seeds: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: SeedCodec,
                minimum: 1,
            }
            .read(reader)?,
            custom_program_id: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            is_signer: Bool.read(reader)?,
            is_writable: Bool.read(reader)?,
        })
    }
}
/// Validated data for `ExtraAccountAddressPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtraAccountAddressPayload {
    /// The `address` wire field.
    pub address: Address,
    /// The `is_signer` wire field.
    pub is_signer: bool,
    /// The `is_writable` wire field.
    pub is_writable: bool,
}
/// Bounded codec for [`ExtraAccountAddressPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtraAccountAddressPayloadCodec;
impl Encoder<ExtraAccountAddressPayload> for ExtraAccountAddressPayloadCodec {
    fn encoded_len(&self, value: &ExtraAccountAddressPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.address)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_signer)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_writable)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtraAccountAddressPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.address, writer)?;
        Bool.write(&value.is_signer, writer)?;
        Bool.write(&value.is_writable, writer)?;
        Ok(())
    }
}
impl Decoder<ExtraAccountAddressPayload> for ExtraAccountAddressPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtraAccountAddressPayload, Error> {
        Ok(ExtraAccountAddressPayload {
            address: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            is_signer: Bool.read(reader)?,
            is_writable: Bool.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExtraAccount {
    /// The `PreconfiguredProgram` variant.
    PreconfiguredProgram(ExtraAccountPreconfiguredProgramPayload),
    /// The `PreconfiguredCollection` variant.
    PreconfiguredCollection(ExtraAccountPreconfiguredCollectionPayload),
    /// The `PreconfiguredOwner` variant.
    PreconfiguredOwner(ExtraAccountPreconfiguredOwnerPayload),
    /// The `PreconfiguredRecipient` variant.
    PreconfiguredRecipient(ExtraAccountPreconfiguredRecipientPayload),
    /// The `PreconfiguredAsset` variant.
    PreconfiguredAsset(ExtraAccountPreconfiguredAssetPayload),
    /// The `CustomPda` variant.
    CustomPda(ExtraAccountCustomPdaPayload),
    /// The `Address` variant.
    Address(ExtraAccountAddressPayload),
}
/// Bounded codec for [`ExtraAccount`].
#[derive(Clone, Copy, Debug)]
pub struct ExtraAccountCodec;
const EXTRAACCOUNT_TAG_0: u8 = 0;
const EXTRAACCOUNT_TAG_1: u8 = 1;
const EXTRAACCOUNT_TAG_2: u8 = 2;
const EXTRAACCOUNT_TAG_3: u8 = 3;
const EXTRAACCOUNT_TAG_4: u8 = 4;
const EXTRAACCOUNT_TAG_5: u8 = 5;
const EXTRAACCOUNT_TAG_6: u8 = 6;
impl Encoder<ExtraAccount> for ExtraAccountCodec {
    fn encoded_len(&self, value: &ExtraAccount) -> Result<usize, Error> {
        let size = match value {
            ExtraAccount::PreconfiguredProgram(value) => {
                ExtraAccountPreconfiguredProgramPayloadCodec.encoded_len(value)?
            }
            ExtraAccount::PreconfiguredCollection(value) => {
                ExtraAccountPreconfiguredCollectionPayloadCodec.encoded_len(value)?
            }
            ExtraAccount::PreconfiguredOwner(value) => {
                ExtraAccountPreconfiguredOwnerPayloadCodec.encoded_len(value)?
            }
            ExtraAccount::PreconfiguredRecipient(value) => {
                ExtraAccountPreconfiguredRecipientPayloadCodec.encoded_len(value)?
            }
            ExtraAccount::PreconfiguredAsset(value) => {
                ExtraAccountPreconfiguredAssetPayloadCodec.encoded_len(value)?
            }
            ExtraAccount::CustomPda(value) => {
                ExtraAccountCustomPdaPayloadCodec.encoded_len(value)?
            }
            ExtraAccount::Address(value) => ExtraAccountAddressPayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &ExtraAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            ExtraAccount::PreconfiguredProgram(value) => {
                Number::<u8>::LE.write(&EXTRAACCOUNT_TAG_0, writer)?;
                ExtraAccountPreconfiguredProgramPayloadCodec.write(value, writer)
            }
            ExtraAccount::PreconfiguredCollection(value) => {
                Number::<u8>::LE.write(&EXTRAACCOUNT_TAG_1, writer)?;
                ExtraAccountPreconfiguredCollectionPayloadCodec.write(value, writer)
            }
            ExtraAccount::PreconfiguredOwner(value) => {
                Number::<u8>::LE.write(&EXTRAACCOUNT_TAG_2, writer)?;
                ExtraAccountPreconfiguredOwnerPayloadCodec.write(value, writer)
            }
            ExtraAccount::PreconfiguredRecipient(value) => {
                Number::<u8>::LE.write(&EXTRAACCOUNT_TAG_3, writer)?;
                ExtraAccountPreconfiguredRecipientPayloadCodec.write(value, writer)
            }
            ExtraAccount::PreconfiguredAsset(value) => {
                Number::<u8>::LE.write(&EXTRAACCOUNT_TAG_4, writer)?;
                ExtraAccountPreconfiguredAssetPayloadCodec.write(value, writer)
            }
            ExtraAccount::CustomPda(value) => {
                Number::<u8>::LE.write(&EXTRAACCOUNT_TAG_5, writer)?;
                ExtraAccountCustomPdaPayloadCodec.write(value, writer)
            }
            ExtraAccount::Address(value) => {
                Number::<u8>::LE.write(&EXTRAACCOUNT_TAG_6, writer)?;
                ExtraAccountAddressPayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<ExtraAccount> for ExtraAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtraAccount, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            EXTRAACCOUNT_TAG_0 => Ok(ExtraAccount::PreconfiguredProgram(
                ExtraAccountPreconfiguredProgramPayloadCodec.read(reader)?,
            )),
            EXTRAACCOUNT_TAG_1 => Ok(ExtraAccount::PreconfiguredCollection(
                ExtraAccountPreconfiguredCollectionPayloadCodec.read(reader)?,
            )),
            EXTRAACCOUNT_TAG_2 => Ok(ExtraAccount::PreconfiguredOwner(
                ExtraAccountPreconfiguredOwnerPayloadCodec.read(reader)?,
            )),
            EXTRAACCOUNT_TAG_3 => Ok(ExtraAccount::PreconfiguredRecipient(
                ExtraAccountPreconfiguredRecipientPayloadCodec.read(reader)?,
            )),
            EXTRAACCOUNT_TAG_4 => Ok(ExtraAccount::PreconfiguredAsset(
                ExtraAccountPreconfiguredAssetPayloadCodec.read(reader)?,
            )),
            EXTRAACCOUNT_TAG_5 => Ok(ExtraAccount::CustomPda(
                ExtraAccountCustomPdaPayloadCodec.read(reader)?,
            )),
            EXTRAACCOUNT_TAG_6 => Ok(ExtraAccount::Address(
                ExtraAccountAddressPayloadCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Plugin {
    /// The `Royalties` variant.
    Royalties(Royalties),
    /// The `FreezeDelegate` variant.
    FreezeDelegate(FreezeDelegate),
    /// The `BurnDelegate` variant.
    BurnDelegate(BurnDelegate),
    /// The `TransferDelegate` variant.
    TransferDelegate(TransferDelegate),
    /// The `UpdateDelegate` variant.
    UpdateDelegate(UpdateDelegate),
    /// The `PermanentFreezeDelegate` variant.
    PermanentFreezeDelegate(PermanentFreezeDelegate),
    /// The `Attributes` variant.
    Attributes(Attributes),
    /// The `PermanentTransferDelegate` variant.
    PermanentTransferDelegate(PermanentTransferDelegate),
    /// The `PermanentBurnDelegate` variant.
    PermanentBurnDelegate(PermanentBurnDelegate),
    /// The `Edition` variant.
    Edition(Edition),
    /// The `MasterEdition` variant.
    MasterEdition(MasterEdition),
    /// The `AddBlocker` variant.
    AddBlocker(AddBlocker),
    /// The `ImmutableMetadata` variant.
    ImmutableMetadata(ImmutableMetadata),
    /// The `VerifiedCreators` variant.
    VerifiedCreators(VerifiedCreators),
    /// The `Autograph` variant.
    Autograph(Autograph),
    /// The `BubblegumV2` variant.
    BubblegumV2(BubblegumV2),
    /// The `FreezeExecute` variant.
    FreezeExecute(FreezeExecute),
    /// The `PermanentFreezeExecute` variant.
    PermanentFreezeExecute(PermanentFreezeExecute),
    /// The `Groups` variant.
    Groups(Groups),
}
/// Bounded codec for [`Plugin`].
#[derive(Clone, Copy, Debug)]
pub struct PluginCodec;
const PLUGIN_TAG_0: u8 = 0;
const PLUGIN_TAG_1: u8 = 1;
const PLUGIN_TAG_2: u8 = 2;
const PLUGIN_TAG_3: u8 = 3;
const PLUGIN_TAG_4: u8 = 4;
const PLUGIN_TAG_5: u8 = 5;
const PLUGIN_TAG_6: u8 = 6;
const PLUGIN_TAG_7: u8 = 7;
const PLUGIN_TAG_8: u8 = 8;
const PLUGIN_TAG_9: u8 = 9;
const PLUGIN_TAG_10: u8 = 10;
const PLUGIN_TAG_11: u8 = 11;
const PLUGIN_TAG_12: u8 = 12;
const PLUGIN_TAG_13: u8 = 13;
const PLUGIN_TAG_14: u8 = 14;
const PLUGIN_TAG_15: u8 = 15;
const PLUGIN_TAG_16: u8 = 16;
const PLUGIN_TAG_17: u8 = 17;
const PLUGIN_TAG_18: u8 = 18;
impl Encoder<Plugin> for PluginCodec {
    fn encoded_len(&self, value: &Plugin) -> Result<usize, Error> {
        let size = match value {
            Plugin::Royalties(value) => RoyaltiesCodec.encoded_len(value)?,
            Plugin::FreezeDelegate(value) => FreezeDelegateCodec.encoded_len(value)?,
            Plugin::BurnDelegate(value) => BurnDelegateCodec.encoded_len(value)?,
            Plugin::TransferDelegate(value) => TransferDelegateCodec.encoded_len(value)?,
            Plugin::UpdateDelegate(value) => UpdateDelegateCodec.encoded_len(value)?,
            Plugin::PermanentFreezeDelegate(value) => {
                PermanentFreezeDelegateCodec.encoded_len(value)?
            }
            Plugin::Attributes(value) => AttributesCodec.encoded_len(value)?,
            Plugin::PermanentTransferDelegate(value) => {
                PermanentTransferDelegateCodec.encoded_len(value)?
            }
            Plugin::PermanentBurnDelegate(value) => {
                PermanentBurnDelegateCodec.encoded_len(value)?
            }
            Plugin::Edition(value) => EditionCodec.encoded_len(value)?,
            Plugin::MasterEdition(value) => MasterEditionCodec.encoded_len(value)?,
            Plugin::AddBlocker(value) => AddBlockerCodec.encoded_len(value)?,
            Plugin::ImmutableMetadata(value) => ImmutableMetadataCodec.encoded_len(value)?,
            Plugin::VerifiedCreators(value) => VerifiedCreatorsCodec.encoded_len(value)?,
            Plugin::Autograph(value) => AutographCodec.encoded_len(value)?,
            Plugin::BubblegumV2(value) => BubblegumV2Codec.encoded_len(value)?,
            Plugin::FreezeExecute(value) => FreezeExecuteCodec.encoded_len(value)?,
            Plugin::PermanentFreezeExecute(value) => {
                PermanentFreezeExecuteCodec.encoded_len(value)?
            }
            Plugin::Groups(value) => GroupsCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &Plugin, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            Plugin::Royalties(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_0, writer)?;
                RoyaltiesCodec.write(value, writer)
            }
            Plugin::FreezeDelegate(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_1, writer)?;
                FreezeDelegateCodec.write(value, writer)
            }
            Plugin::BurnDelegate(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_2, writer)?;
                BurnDelegateCodec.write(value, writer)
            }
            Plugin::TransferDelegate(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_3, writer)?;
                TransferDelegateCodec.write(value, writer)
            }
            Plugin::UpdateDelegate(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_4, writer)?;
                UpdateDelegateCodec.write(value, writer)
            }
            Plugin::PermanentFreezeDelegate(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_5, writer)?;
                PermanentFreezeDelegateCodec.write(value, writer)
            }
            Plugin::Attributes(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_6, writer)?;
                AttributesCodec.write(value, writer)
            }
            Plugin::PermanentTransferDelegate(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_7, writer)?;
                PermanentTransferDelegateCodec.write(value, writer)
            }
            Plugin::PermanentBurnDelegate(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_8, writer)?;
                PermanentBurnDelegateCodec.write(value, writer)
            }
            Plugin::Edition(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_9, writer)?;
                EditionCodec.write(value, writer)
            }
            Plugin::MasterEdition(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_10, writer)?;
                MasterEditionCodec.write(value, writer)
            }
            Plugin::AddBlocker(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_11, writer)?;
                AddBlockerCodec.write(value, writer)
            }
            Plugin::ImmutableMetadata(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_12, writer)?;
                ImmutableMetadataCodec.write(value, writer)
            }
            Plugin::VerifiedCreators(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_13, writer)?;
                VerifiedCreatorsCodec.write(value, writer)
            }
            Plugin::Autograph(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_14, writer)?;
                AutographCodec.write(value, writer)
            }
            Plugin::BubblegumV2(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_15, writer)?;
                BubblegumV2Codec.write(value, writer)
            }
            Plugin::FreezeExecute(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_16, writer)?;
                FreezeExecuteCodec.write(value, writer)
            }
            Plugin::PermanentFreezeExecute(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_17, writer)?;
                PermanentFreezeExecuteCodec.write(value, writer)
            }
            Plugin::Groups(value) => {
                Number::<u8>::LE.write(&PLUGIN_TAG_18, writer)?;
                GroupsCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<Plugin> for PluginCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Plugin, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            PLUGIN_TAG_0 => Ok(Plugin::Royalties(RoyaltiesCodec.read(reader)?)),
            PLUGIN_TAG_1 => Ok(Plugin::FreezeDelegate(FreezeDelegateCodec.read(reader)?)),
            PLUGIN_TAG_2 => Ok(Plugin::BurnDelegate(BurnDelegateCodec.read(reader)?)),
            PLUGIN_TAG_3 => Ok(Plugin::TransferDelegate(
                TransferDelegateCodec.read(reader)?,
            )),
            PLUGIN_TAG_4 => Ok(Plugin::UpdateDelegate(UpdateDelegateCodec.read(reader)?)),
            PLUGIN_TAG_5 => Ok(Plugin::PermanentFreezeDelegate(
                PermanentFreezeDelegateCodec.read(reader)?,
            )),
            PLUGIN_TAG_6 => Ok(Plugin::Attributes(AttributesCodec.read(reader)?)),
            PLUGIN_TAG_7 => Ok(Plugin::PermanentTransferDelegate(
                PermanentTransferDelegateCodec.read(reader)?,
            )),
            PLUGIN_TAG_8 => Ok(Plugin::PermanentBurnDelegate(
                PermanentBurnDelegateCodec.read(reader)?,
            )),
            PLUGIN_TAG_9 => Ok(Plugin::Edition(EditionCodec.read(reader)?)),
            PLUGIN_TAG_10 => Ok(Plugin::MasterEdition(MasterEditionCodec.read(reader)?)),
            PLUGIN_TAG_11 => Ok(Plugin::AddBlocker(AddBlockerCodec.read(reader)?)),
            PLUGIN_TAG_12 => Ok(Plugin::ImmutableMetadata(
                ImmutableMetadataCodec.read(reader)?,
            )),
            PLUGIN_TAG_13 => Ok(Plugin::VerifiedCreators(
                VerifiedCreatorsCodec.read(reader)?,
            )),
            PLUGIN_TAG_14 => Ok(Plugin::Autograph(AutographCodec.read(reader)?)),
            PLUGIN_TAG_15 => Ok(Plugin::BubblegumV2(BubblegumV2Codec.read(reader)?)),
            PLUGIN_TAG_16 => Ok(Plugin::FreezeExecute(FreezeExecuteCodec.read(reader)?)),
            PLUGIN_TAG_17 => Ok(Plugin::PermanentFreezeExecute(
                PermanentFreezeExecuteCodec.read(reader)?,
            )),
            PLUGIN_TAG_18 => Ok(Plugin::Groups(GroupsCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExternalPluginAdapter {
    /// The `LifecycleHook` variant.
    LifecycleHook(LifecycleHook),
    /// The `Oracle` variant.
    Oracle(Oracle),
    /// The `AppData` variant.
    AppData(AppData),
    /// The `LinkedLifecycleHook` variant.
    LinkedLifecycleHook(LinkedLifecycleHook),
    /// The `LinkedAppData` variant.
    LinkedAppData(LinkedAppData),
    /// The `DataSection` variant.
    DataSection(DataSection),
    /// The `AgentIdentity` variant.
    AgentIdentity(AgentIdentity),
}
/// Bounded codec for [`ExternalPluginAdapter`].
#[derive(Clone, Copy, Debug)]
pub struct ExternalPluginAdapterCodec;
const EXTERNALPLUGINADAPTER_TAG_0: u8 = 0;
const EXTERNALPLUGINADAPTER_TAG_1: u8 = 1;
const EXTERNALPLUGINADAPTER_TAG_2: u8 = 2;
const EXTERNALPLUGINADAPTER_TAG_3: u8 = 3;
const EXTERNALPLUGINADAPTER_TAG_4: u8 = 4;
const EXTERNALPLUGINADAPTER_TAG_5: u8 = 5;
const EXTERNALPLUGINADAPTER_TAG_6: u8 = 6;
impl Encoder<ExternalPluginAdapter> for ExternalPluginAdapterCodec {
    fn encoded_len(&self, value: &ExternalPluginAdapter) -> Result<usize, Error> {
        let size = match value {
            ExternalPluginAdapter::LifecycleHook(value) => LifecycleHookCodec.encoded_len(value)?,
            ExternalPluginAdapter::Oracle(value) => OracleCodec.encoded_len(value)?,
            ExternalPluginAdapter::AppData(value) => AppDataCodec.encoded_len(value)?,
            ExternalPluginAdapter::LinkedLifecycleHook(value) => {
                LinkedLifecycleHookCodec.encoded_len(value)?
            }
            ExternalPluginAdapter::LinkedAppData(value) => LinkedAppDataCodec.encoded_len(value)?,
            ExternalPluginAdapter::DataSection(value) => DataSectionCodec.encoded_len(value)?,
            ExternalPluginAdapter::AgentIdentity(value) => AgentIdentityCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &ExternalPluginAdapter, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            ExternalPluginAdapter::LifecycleHook(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTER_TAG_0, writer)?;
                LifecycleHookCodec.write(value, writer)
            }
            ExternalPluginAdapter::Oracle(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTER_TAG_1, writer)?;
                OracleCodec.write(value, writer)
            }
            ExternalPluginAdapter::AppData(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTER_TAG_2, writer)?;
                AppDataCodec.write(value, writer)
            }
            ExternalPluginAdapter::LinkedLifecycleHook(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTER_TAG_3, writer)?;
                LinkedLifecycleHookCodec.write(value, writer)
            }
            ExternalPluginAdapter::LinkedAppData(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTER_TAG_4, writer)?;
                LinkedAppDataCodec.write(value, writer)
            }
            ExternalPluginAdapter::DataSection(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTER_TAG_5, writer)?;
                DataSectionCodec.write(value, writer)
            }
            ExternalPluginAdapter::AgentIdentity(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTER_TAG_6, writer)?;
                AgentIdentityCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<ExternalPluginAdapter> for ExternalPluginAdapterCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExternalPluginAdapter, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            EXTERNALPLUGINADAPTER_TAG_0 => Ok(ExternalPluginAdapter::LifecycleHook(
                LifecycleHookCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTER_TAG_1 => {
                Ok(ExternalPluginAdapter::Oracle(OracleCodec.read(reader)?))
            }
            EXTERNALPLUGINADAPTER_TAG_2 => {
                Ok(ExternalPluginAdapter::AppData(AppDataCodec.read(reader)?))
            }
            EXTERNALPLUGINADAPTER_TAG_3 => Ok(ExternalPluginAdapter::LinkedLifecycleHook(
                LinkedLifecycleHookCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTER_TAG_4 => Ok(ExternalPluginAdapter::LinkedAppData(
                LinkedAppDataCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTER_TAG_5 => Ok(ExternalPluginAdapter::DataSection(
                DataSectionCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTER_TAG_6 => Ok(ExternalPluginAdapter::AgentIdentity(
                AgentIdentityCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExternalPluginAdapterInitInfo {
    /// The `LifecycleHook` variant.
    LifecycleHook(LifecycleHookInitInfo),
    /// The `Oracle` variant.
    Oracle(OracleInitInfo),
    /// The `AppData` variant.
    AppData(AppDataInitInfo),
    /// The `LinkedLifecycleHook` variant.
    LinkedLifecycleHook(LinkedLifecycleHookInitInfo),
    /// The `LinkedAppData` variant.
    LinkedAppData(LinkedAppDataInitInfo),
    /// The `DataSection` variant.
    DataSection(DataSectionInitInfo),
    /// The `AgentIdentity` variant.
    AgentIdentity(AgentIdentityInitInfo),
}
/// Bounded codec for [`ExternalPluginAdapterInitInfo`].
#[derive(Clone, Copy, Debug)]
pub struct ExternalPluginAdapterInitInfoCodec;
const EXTERNALPLUGINADAPTERINITINFO_TAG_0: u8 = 0;
const EXTERNALPLUGINADAPTERINITINFO_TAG_1: u8 = 1;
const EXTERNALPLUGINADAPTERINITINFO_TAG_2: u8 = 2;
const EXTERNALPLUGINADAPTERINITINFO_TAG_3: u8 = 3;
const EXTERNALPLUGINADAPTERINITINFO_TAG_4: u8 = 4;
const EXTERNALPLUGINADAPTERINITINFO_TAG_5: u8 = 5;
const EXTERNALPLUGINADAPTERINITINFO_TAG_6: u8 = 6;
impl Encoder<ExternalPluginAdapterInitInfo> for ExternalPluginAdapterInitInfoCodec {
    fn encoded_len(&self, value: &ExternalPluginAdapterInitInfo) -> Result<usize, Error> {
        let size = match value {
            ExternalPluginAdapterInitInfo::LifecycleHook(value) => {
                LifecycleHookInitInfoCodec.encoded_len(value)?
            }
            ExternalPluginAdapterInitInfo::Oracle(value) => {
                OracleInitInfoCodec.encoded_len(value)?
            }
            ExternalPluginAdapterInitInfo::AppData(value) => {
                AppDataInitInfoCodec.encoded_len(value)?
            }
            ExternalPluginAdapterInitInfo::LinkedLifecycleHook(value) => {
                LinkedLifecycleHookInitInfoCodec.encoded_len(value)?
            }
            ExternalPluginAdapterInitInfo::LinkedAppData(value) => {
                LinkedAppDataInitInfoCodec.encoded_len(value)?
            }
            ExternalPluginAdapterInitInfo::DataSection(value) => {
                DataSectionInitInfoCodec.encoded_len(value)?
            }
            ExternalPluginAdapterInitInfo::AgentIdentity(value) => {
                AgentIdentityInitInfoCodec.encoded_len(value)?
            }
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(
        &self,
        value: &ExternalPluginAdapterInitInfo,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            ExternalPluginAdapterInitInfo::LifecycleHook(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERINITINFO_TAG_0, writer)?;
                LifecycleHookInitInfoCodec.write(value, writer)
            }
            ExternalPluginAdapterInitInfo::Oracle(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERINITINFO_TAG_1, writer)?;
                OracleInitInfoCodec.write(value, writer)
            }
            ExternalPluginAdapterInitInfo::AppData(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERINITINFO_TAG_2, writer)?;
                AppDataInitInfoCodec.write(value, writer)
            }
            ExternalPluginAdapterInitInfo::LinkedLifecycleHook(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERINITINFO_TAG_3, writer)?;
                LinkedLifecycleHookInitInfoCodec.write(value, writer)
            }
            ExternalPluginAdapterInitInfo::LinkedAppData(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERINITINFO_TAG_4, writer)?;
                LinkedAppDataInitInfoCodec.write(value, writer)
            }
            ExternalPluginAdapterInitInfo::DataSection(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERINITINFO_TAG_5, writer)?;
                DataSectionInitInfoCodec.write(value, writer)
            }
            ExternalPluginAdapterInitInfo::AgentIdentity(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERINITINFO_TAG_6, writer)?;
                AgentIdentityInitInfoCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<ExternalPluginAdapterInitInfo> for ExternalPluginAdapterInitInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExternalPluginAdapterInitInfo, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            EXTERNALPLUGINADAPTERINITINFO_TAG_0 => {
                Ok(ExternalPluginAdapterInitInfo::LifecycleHook(
                    LifecycleHookInitInfoCodec.read(reader)?,
                ))
            }
            EXTERNALPLUGINADAPTERINITINFO_TAG_1 => Ok(ExternalPluginAdapterInitInfo::Oracle(
                OracleInitInfoCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTERINITINFO_TAG_2 => Ok(ExternalPluginAdapterInitInfo::AppData(
                AppDataInitInfoCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTERINITINFO_TAG_3 => {
                Ok(ExternalPluginAdapterInitInfo::LinkedLifecycleHook(
                    LinkedLifecycleHookInitInfoCodec.read(reader)?,
                ))
            }
            EXTERNALPLUGINADAPTERINITINFO_TAG_4 => {
                Ok(ExternalPluginAdapterInitInfo::LinkedAppData(
                    LinkedAppDataInitInfoCodec.read(reader)?,
                ))
            }
            EXTERNALPLUGINADAPTERINITINFO_TAG_5 => Ok(ExternalPluginAdapterInitInfo::DataSection(
                DataSectionInitInfoCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTERINITINFO_TAG_6 => {
                Ok(ExternalPluginAdapterInitInfo::AgentIdentity(
                    AgentIdentityInitInfoCodec.read(reader)?,
                ))
            }
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExternalPluginAdapterUpdateInfo {
    /// The `LifecycleHook` variant.
    LifecycleHook(LifecycleHookUpdateInfo),
    /// The `Oracle` variant.
    Oracle(OracleUpdateInfo),
    /// The `AppData` variant.
    AppData(AppDataUpdateInfo),
    /// The `LinkedLifecycleHook` variant.
    LinkedLifecycleHook(LinkedLifecycleHookUpdateInfo),
    /// The `LinkedAppData` variant.
    LinkedAppData(LinkedAppDataUpdateInfo),
    /// The `AgentIdentity` variant.
    AgentIdentity(AgentIdentityUpdateInfo),
}
/// Bounded codec for [`ExternalPluginAdapterUpdateInfo`].
#[derive(Clone, Copy, Debug)]
pub struct ExternalPluginAdapterUpdateInfoCodec;
const EXTERNALPLUGINADAPTERUPDATEINFO_TAG_0: u8 = 0;
const EXTERNALPLUGINADAPTERUPDATEINFO_TAG_1: u8 = 1;
const EXTERNALPLUGINADAPTERUPDATEINFO_TAG_2: u8 = 2;
const EXTERNALPLUGINADAPTERUPDATEINFO_TAG_3: u8 = 3;
const EXTERNALPLUGINADAPTERUPDATEINFO_TAG_4: u8 = 4;
const EXTERNALPLUGINADAPTERUPDATEINFO_TAG_5: u8 = 5;
impl Encoder<ExternalPluginAdapterUpdateInfo> for ExternalPluginAdapterUpdateInfoCodec {
    fn encoded_len(&self, value: &ExternalPluginAdapterUpdateInfo) -> Result<usize, Error> {
        let size = match value {
            ExternalPluginAdapterUpdateInfo::LifecycleHook(value) => {
                LifecycleHookUpdateInfoCodec.encoded_len(value)?
            }
            ExternalPluginAdapterUpdateInfo::Oracle(value) => {
                OracleUpdateInfoCodec.encoded_len(value)?
            }
            ExternalPluginAdapterUpdateInfo::AppData(value) => {
                AppDataUpdateInfoCodec.encoded_len(value)?
            }
            ExternalPluginAdapterUpdateInfo::LinkedLifecycleHook(value) => {
                LinkedLifecycleHookUpdateInfoCodec.encoded_len(value)?
            }
            ExternalPluginAdapterUpdateInfo::LinkedAppData(value) => {
                LinkedAppDataUpdateInfoCodec.encoded_len(value)?
            }
            ExternalPluginAdapterUpdateInfo::AgentIdentity(value) => {
                AgentIdentityUpdateInfoCodec.encoded_len(value)?
            }
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(
        &self,
        value: &ExternalPluginAdapterUpdateInfo,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            ExternalPluginAdapterUpdateInfo::LifecycleHook(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERUPDATEINFO_TAG_0, writer)?;
                LifecycleHookUpdateInfoCodec.write(value, writer)
            }
            ExternalPluginAdapterUpdateInfo::Oracle(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERUPDATEINFO_TAG_1, writer)?;
                OracleUpdateInfoCodec.write(value, writer)
            }
            ExternalPluginAdapterUpdateInfo::AppData(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERUPDATEINFO_TAG_2, writer)?;
                AppDataUpdateInfoCodec.write(value, writer)
            }
            ExternalPluginAdapterUpdateInfo::LinkedLifecycleHook(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERUPDATEINFO_TAG_3, writer)?;
                LinkedLifecycleHookUpdateInfoCodec.write(value, writer)
            }
            ExternalPluginAdapterUpdateInfo::LinkedAppData(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERUPDATEINFO_TAG_4, writer)?;
                LinkedAppDataUpdateInfoCodec.write(value, writer)
            }
            ExternalPluginAdapterUpdateInfo::AgentIdentity(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERUPDATEINFO_TAG_5, writer)?;
                AgentIdentityUpdateInfoCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<ExternalPluginAdapterUpdateInfo> for ExternalPluginAdapterUpdateInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExternalPluginAdapterUpdateInfo, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            EXTERNALPLUGINADAPTERUPDATEINFO_TAG_0 => {
                Ok(ExternalPluginAdapterUpdateInfo::LifecycleHook(
                    LifecycleHookUpdateInfoCodec.read(reader)?,
                ))
            }
            EXTERNALPLUGINADAPTERUPDATEINFO_TAG_1 => Ok(ExternalPluginAdapterUpdateInfo::Oracle(
                OracleUpdateInfoCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTERUPDATEINFO_TAG_2 => Ok(ExternalPluginAdapterUpdateInfo::AppData(
                AppDataUpdateInfoCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTERUPDATEINFO_TAG_3 => {
                Ok(ExternalPluginAdapterUpdateInfo::LinkedLifecycleHook(
                    LinkedLifecycleHookUpdateInfoCodec.read(reader)?,
                ))
            }
            EXTERNALPLUGINADAPTERUPDATEINFO_TAG_4 => {
                Ok(ExternalPluginAdapterUpdateInfo::LinkedAppData(
                    LinkedAppDataUpdateInfoCodec.read(reader)?,
                ))
            }
            EXTERNALPLUGINADAPTERUPDATEINFO_TAG_5 => {
                Ok(ExternalPluginAdapterUpdateInfo::AgentIdentity(
                    AgentIdentityUpdateInfoCodec.read(reader)?,
                ))
            }
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExternalPluginAdapterKey {
    /// The `LifecycleHook` variant.
    LifecycleHook(Address),
    /// The `Oracle` variant.
    Oracle(Address),
    /// The `AppData` variant.
    AppData(Authority),
    /// The `LinkedLifecycleHook` variant.
    LinkedLifecycleHook(Address),
    /// The `LinkedAppData` variant.
    LinkedAppData(Authority),
    /// The `DataSection` variant.
    DataSection(LinkedDataKey),
    /// The `AgentIdentity` variant.
    AgentIdentity,
}
/// Bounded codec for [`ExternalPluginAdapterKey`].
#[derive(Clone, Copy, Debug)]
pub struct ExternalPluginAdapterKeyCodec;
const EXTERNALPLUGINADAPTERKEY_TAG_0: u8 = 0;
const EXTERNALPLUGINADAPTERKEY_TAG_1: u8 = 1;
const EXTERNALPLUGINADAPTERKEY_TAG_2: u8 = 2;
const EXTERNALPLUGINADAPTERKEY_TAG_3: u8 = 3;
const EXTERNALPLUGINADAPTERKEY_TAG_4: u8 = 4;
const EXTERNALPLUGINADAPTERKEY_TAG_5: u8 = 5;
const EXTERNALPLUGINADAPTERKEY_TAG_6: u8 = 6;
impl Encoder<ExternalPluginAdapterKey> for ExternalPluginAdapterKeyCodec {
    fn encoded_len(&self, value: &ExternalPluginAdapterKey) -> Result<usize, Error> {
        let size = match value {
            ExternalPluginAdapterKey::LifecycleHook(value) => Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .encoded_len(value)?,
            ExternalPluginAdapterKey::Oracle(value) => Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .encoded_len(value)?,
            ExternalPluginAdapterKey::AppData(value) => AuthorityCodec.encoded_len(value)?,
            ExternalPluginAdapterKey::LinkedLifecycleHook(value) => Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .encoded_len(value)?,
            ExternalPluginAdapterKey::LinkedAppData(value) => AuthorityCodec.encoded_len(value)?,
            ExternalPluginAdapterKey::DataSection(value) => {
                LinkedDataKeyCodec.encoded_len(value)?
            }
            ExternalPluginAdapterKey::AgentIdentity => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(
        &self,
        value: &ExternalPluginAdapterKey,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            ExternalPluginAdapterKey::LifecycleHook(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERKEY_TAG_0, writer)?;
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .write(value, writer)
            }
            ExternalPluginAdapterKey::Oracle(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERKEY_TAG_1, writer)?;
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .write(value, writer)
            }
            ExternalPluginAdapterKey::AppData(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERKEY_TAG_2, writer)?;
                AuthorityCodec.write(value, writer)
            }
            ExternalPluginAdapterKey::LinkedLifecycleHook(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERKEY_TAG_3, writer)?;
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .write(value, writer)
            }
            ExternalPluginAdapterKey::LinkedAppData(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERKEY_TAG_4, writer)?;
                AuthorityCodec.write(value, writer)
            }
            ExternalPluginAdapterKey::DataSection(value) => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERKEY_TAG_5, writer)?;
                LinkedDataKeyCodec.write(value, writer)
            }
            ExternalPluginAdapterKey::AgentIdentity => {
                Number::<u8>::LE.write(&EXTERNALPLUGINADAPTERKEY_TAG_6, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<ExternalPluginAdapterKey> for ExternalPluginAdapterKeyCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExternalPluginAdapterKey, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            EXTERNALPLUGINADAPTERKEY_TAG_0 => Ok(ExternalPluginAdapterKey::LifecycleHook(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .read(reader)?,
            )),
            EXTERNALPLUGINADAPTERKEY_TAG_1 => Ok(ExternalPluginAdapterKey::Oracle(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .read(reader)?,
            )),
            EXTERNALPLUGINADAPTERKEY_TAG_2 => Ok(ExternalPluginAdapterKey::AppData(
                AuthorityCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTERKEY_TAG_3 => Ok(ExternalPluginAdapterKey::LinkedLifecycleHook(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .read(reader)?,
            )),
            EXTERNALPLUGINADAPTERKEY_TAG_4 => Ok(ExternalPluginAdapterKey::LinkedAppData(
                AuthorityCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTERKEY_TAG_5 => Ok(ExternalPluginAdapterKey::DataSection(
                LinkedDataKeyCodec.read(reader)?,
            )),
            EXTERNALPLUGINADAPTERKEY_TAG_6 => Ok(ExternalPluginAdapterKey::AgentIdentity),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `AssetV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetV1 {
    /// The `owner` wire field.
    pub owner: Address,
    /// The `update_authority` wire field.
    pub update_authority: UpdateAuthority,
    /// The `name` wire field.
    pub name: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `seq` wire field.
    pub seq: Option<u64>,
}
/// Bounded codec for [`AssetV1`].
#[derive(Clone, Copy, Debug)]
pub struct AssetV1Codec;
impl Encoder<AssetV1> for AssetV1Codec {
    fn encoded_len(&self, value: &AssetV1) -> Result<usize, Error> {
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
                .encoded_len(&value.owner)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(UpdateAuthorityCodec.encoded_len(&value.update_authority)?)
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
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.seq)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AssetV1, writer: &mut Writer<'_>) -> Result<(), Error> {
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
        .write(&value.owner, writer)?;
        UpdateAuthorityCodec.write(&value.update_authority, writer)?;
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
        OptionCodec::u8(Number::<u64>::LE).write(&value.seq, writer)?;
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
impl Decoder<AssetV1> for AssetV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AssetV1, Error> {
        const _DISCRIMINATOR: &[u8] = &[1];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = AssetV1 {
            owner: Affix {
                prefix: &[1],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            update_authority: UpdateAuthorityCodec.read(reader)?,
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
            seq: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
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
/// Validated data for `HashedAssetV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HashedAssetV1 {
    /// The `hash` wire field.
    pub hash: [u8; 32],
}
/// Bounded codec for [`HashedAssetV1`].
#[derive(Clone, Copy, Debug)]
pub struct HashedAssetV1Codec;
impl Encoder<HashedAssetV1> for HashedAssetV1Codec {
    fn encoded_len(&self, value: &HashedAssetV1) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[2],
                    item: FixedBytes::<32>,
                    suffix: &[],
                }
                .encoded_len(&value.hash)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &HashedAssetV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[2];
        let _record_start = writer.position();
        Affix {
            prefix: &[2],
            item: FixedBytes::<32>,
            suffix: &[],
        }
        .write(&value.hash, writer)?;
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
impl Decoder<HashedAssetV1> for HashedAssetV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<HashedAssetV1, Error> {
        const _DISCRIMINATOR: &[u8] = &[2];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = HashedAssetV1 {
            hash: Affix {
                prefix: &[2],
                item: FixedBytes::<32>,
                suffix: &[],
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
/// Validated data for `PluginHeaderV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginHeaderV1 {
    /// The `plugin_registry_offset` wire field.
    pub plugin_registry_offset: u64,
}
/// Bounded codec for [`PluginHeaderV1`].
#[derive(Clone, Copy, Debug)]
pub struct PluginHeaderV1Codec;
impl Encoder<PluginHeaderV1> for PluginHeaderV1Codec {
    fn encoded_len(&self, value: &PluginHeaderV1) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[3],
                    item: Number::<u64>::LE,
                    suffix: &[],
                }
                .encoded_len(&value.plugin_registry_offset)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PluginHeaderV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[3];
        let _record_start = writer.position();
        Affix {
            prefix: &[3],
            item: Number::<u64>::LE,
            suffix: &[],
        }
        .write(&value.plugin_registry_offset, writer)?;
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
impl Decoder<PluginHeaderV1> for PluginHeaderV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PluginHeaderV1, Error> {
        const _DISCRIMINATOR: &[u8] = &[3];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = PluginHeaderV1 {
            plugin_registry_offset: Affix {
                prefix: &[3],
                item: Number::<u64>::LE,
                suffix: &[],
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
/// Validated data for `PluginRegistryV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginRegistryV1 {
    /// The `registry` wire field.
    pub registry: Vec<RegistryRecord>,
    /// The `external_registry` wire field.
    pub external_registry: Vec<ExternalRegistryRecord>,
}
/// Bounded codec for [`PluginRegistryV1`].
#[derive(Clone, Copy, Debug)]
pub struct PluginRegistryV1Codec;
impl Encoder<PluginRegistryV1> for PluginRegistryV1Codec {
    fn encoded_len(&self, value: &PluginRegistryV1) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[4],
                    item: CheckedCounted {
                        prefix: Number::<u32>::LE,
                        item: RegistryRecordCodec,
                        minimum: 10,
                    },
                    suffix: &[],
                }
                .encoded_len(value.registry.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ExternalRegistryRecordCodec,
                    minimum: 13,
                }
                .encoded_len(value.external_registry.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PluginRegistryV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[4];
        let _record_start = writer.position();
        Affix {
            prefix: &[4],
            item: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: RegistryRecordCodec,
                minimum: 10,
            },
            suffix: &[],
        }
        .write(value.registry.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ExternalRegistryRecordCodec,
            minimum: 13,
        }
        .write(value.external_registry.as_slice(), writer)?;
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
impl Decoder<PluginRegistryV1> for PluginRegistryV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PluginRegistryV1, Error> {
        const _DISCRIMINATOR: &[u8] = &[4];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = PluginRegistryV1 {
            registry: Affix {
                prefix: &[4],
                item: CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: RegistryRecordCodec,
                    minimum: 10,
                },
                suffix: &[],
            }
            .read(reader)?,
            external_registry: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ExternalRegistryRecordCodec,
                minimum: 13,
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
/// Validated data for `CollectionV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionV1 {
    /// The `update_authority` wire field.
    pub update_authority: Address,
    /// The `name` wire field.
    pub name: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `num_minted` wire field.
    pub num_minted: u32,
    /// The `current_size` wire field.
    pub current_size: u32,
}
/// Bounded codec for [`CollectionV1`].
#[derive(Clone, Copy, Debug)]
pub struct CollectionV1Codec;
impl Encoder<CollectionV1> for CollectionV1Codec {
    fn encoded_len(&self, value: &CollectionV1) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[5],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.update_authority)?,
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
            .checked_add(Number::<u32>::LE.encoded_len(&value.num_minted)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.current_size)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CollectionV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[5];
        let _record_start = writer.position();
        Affix {
            prefix: &[5],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.update_authority, writer)?;
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
        Number::<u32>::LE.write(&value.num_minted, writer)?;
        Number::<u32>::LE.write(&value.current_size, writer)?;
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
impl Decoder<CollectionV1> for CollectionV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CollectionV1, Error> {
        const _DISCRIMINATOR: &[u8] = &[5];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = CollectionV1 {
            update_authority: Affix {
                prefix: &[5],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
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
            num_minted: Number::<u32>::LE.read(reader)?,
            current_size: Number::<u32>::LE.read(reader)?,
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
/// Validated data for `GroupV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupV1 {
    /// The `update_authority` wire field.
    pub update_authority: Address,
    /// The `name` wire field.
    pub name: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `collections` wire field.
    pub collections: Vec<Address>,
    /// The `groups` wire field.
    pub groups: Vec<Address>,
    /// The `parent_groups` wire field.
    pub parent_groups: Vec<Address>,
    /// The `assets` wire field.
    pub assets: Vec<Address>,
}
/// Bounded codec for [`GroupV1`].
#[derive(Clone, Copy, Debug)]
pub struct GroupV1Codec;
impl Encoder<GroupV1> for GroupV1Codec {
    fn encoded_len(&self, value: &GroupV1) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[6],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.update_authority)?,
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
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    minimum: 32,
                }
                .encoded_len(value.collections.as_slice())?,
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
                .encoded_len(value.groups.as_slice())?,
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
                .encoded_len(value.parent_groups.as_slice())?,
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
                .encoded_len(value.assets.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &GroupV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[6];
        let _record_start = writer.position();
        Affix {
            prefix: &[6],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.update_authority, writer)?;
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
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.collections.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.groups.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.parent_groups.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.assets.as_slice(), writer)?;
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
impl Decoder<GroupV1> for GroupV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<GroupV1, Error> {
        const _DISCRIMINATOR: &[u8] = &[6];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = GroupV1 {
            update_authority: Affix {
                prefix: &[6],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
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
            collections: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
            groups: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
            parent_groups: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
            assets: CheckedCounted {
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
/// Validated data for `CreateV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateV1 {
    /// The `data_state` wire field.
    pub data_state: DataState,
    /// The `name` wire field.
    pub name: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `plugins` wire field.
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}
/// Bounded codec for [`CreateV1`].
#[derive(Clone, Copy, Debug)]
pub struct CreateV1Codec;
impl Encoder<CreateV1> for CreateV1Codec {
    fn encoded_len(&self, value: &CreateV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(DataStateCodec.encoded_len(&value.data_state)?)
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
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: PluginAuthorityPairCodec,
                    minimum: 2,
                }))
                .encoded_len(&value.plugins)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_0]).write(&(), writer)?;
        DataStateCodec.write(&value.data_state, writer)?;
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
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: PluginAuthorityPairCodec,
            minimum: 2,
        }))
        .write(&value.plugins, writer)?;
        Ok(())
    }
}
impl Decoder<CreateV1> for CreateV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateV1, Error> {
        Constant(&[INSTRUCTION_TAG_0]).read(reader)?;
        Ok(CreateV1 {
            data_state: DataStateCodec.read(reader)?,
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
            plugins: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: PluginAuthorityPairCodec,
                minimum: 2,
            }))
            .read(reader)?,
        })
    }
}
impl CreateV1 {
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
                /* asset */ (true, Some(true), false),
                /* collection */ (true, Some(false), true),
                /* authority */ (false, Some(true), true),
                /* payer */ (true, Some(true), false),
                /* owner */ (false, Some(false), true),
                /* update_authority */ (false, Some(false), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = CreateV1Codec.encode(self, limits.max_bytes)?;
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
        CreateV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateCollectionV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateCollectionV1 {
    /// The `name` wire field.
    pub name: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `plugins` wire field.
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}
/// Bounded codec for [`CreateCollectionV1`].
#[derive(Clone, Copy, Debug)]
pub struct CreateCollectionV1Codec;
impl Encoder<CreateCollectionV1> for CreateCollectionV1Codec {
    fn encoded_len(&self, value: &CreateCollectionV1) -> Result<usize, Error> {
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
                .encoded_len(value.uri.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: PluginAuthorityPairCodec,
                    minimum: 2,
                }))
                .encoded_len(&value.plugins)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateCollectionV1, writer: &mut Writer<'_>) -> Result<(), Error> {
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
        .write(value.uri.as_str(), writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: PluginAuthorityPairCodec,
            minimum: 2,
        }))
        .write(&value.plugins, writer)?;
        Ok(())
    }
}
impl Decoder<CreateCollectionV1> for CreateCollectionV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateCollectionV1, Error> {
        Constant(&[INSTRUCTION_TAG_1]).read(reader)?;
        Ok(CreateCollectionV1 {
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
            plugins: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: PluginAuthorityPairCodec,
                minimum: 2,
            }))
            .read(reader)?,
        })
    }
}
impl CreateCollectionV1 {
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
                /* collection */ (true, Some(true), false),
                /* update_authority */ (false, Some(false), true),
                /* payer */ (true, Some(true), false),
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
        let data = CreateCollectionV1Codec.encode(self, limits.max_bytes)?;
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
        CreateCollectionV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `AddPluginV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddPluginV1 {
    /// The `plugin` wire field.
    pub plugin: Plugin,
    /// The `init_authority` wire field.
    pub init_authority: Option<Authority>,
}
/// Bounded codec for [`AddPluginV1`].
#[derive(Clone, Copy, Debug)]
pub struct AddPluginV1Codec;
impl Encoder<AddPluginV1> for AddPluginV1Codec {
    fn encoded_len(&self, value: &AddPluginV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PluginCodec.encoded_len(&value.plugin)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.init_authority)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AddPluginV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_2]).write(&(), writer)?;
        PluginCodec.write(&value.plugin, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.init_authority, writer)?;
        Ok(())
    }
}
impl Decoder<AddPluginV1> for AddPluginV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AddPluginV1, Error> {
        Constant(&[INSTRUCTION_TAG_2]).read(reader)?;
        Ok(AddPluginV1 {
            plugin: PluginCodec.read(reader)?,
            init_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
        })
    }
}
impl AddPluginV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = AddPluginV1Codec.encode(self, limits.max_bytes)?;
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
        AddPluginV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `AddCollectionPluginV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddCollectionPluginV1 {
    /// The `plugin` wire field.
    pub plugin: Plugin,
    /// The `init_authority` wire field.
    pub init_authority: Option<Authority>,
}
/// Bounded codec for [`AddCollectionPluginV1`].
#[derive(Clone, Copy, Debug)]
pub struct AddCollectionPluginV1Codec;
impl Encoder<AddCollectionPluginV1> for AddCollectionPluginV1Codec {
    fn encoded_len(&self, value: &AddCollectionPluginV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PluginCodec.encoded_len(&value.plugin)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(AuthorityCodec).encoded_len(&value.init_authority)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AddCollectionPluginV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_3]).write(&(), writer)?;
        PluginCodec.write(&value.plugin, writer)?;
        OptionCodec::u8(AuthorityCodec).write(&value.init_authority, writer)?;
        Ok(())
    }
}
impl Decoder<AddCollectionPluginV1> for AddCollectionPluginV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AddCollectionPluginV1, Error> {
        Constant(&[INSTRUCTION_TAG_3]).read(reader)?;
        Ok(AddCollectionPluginV1 {
            plugin: PluginCodec.read(reader)?,
            init_authority: OptionCodec::u8(AuthorityCodec).read(reader)?,
        })
    }
}
impl AddCollectionPluginV1 {
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
                /* collection */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = AddCollectionPluginV1Codec.encode(self, limits.max_bytes)?;
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
        AddCollectionPluginV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RemovePluginV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemovePluginV1 {
    /// The `plugin_type` wire field.
    pub plugin_type: PluginType,
}
/// Bounded codec for [`RemovePluginV1`].
#[derive(Clone, Copy, Debug)]
pub struct RemovePluginV1Codec;
impl Encoder<RemovePluginV1> for RemovePluginV1Codec {
    fn encoded_len(&self, value: &RemovePluginV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PluginTypeCodec.encoded_len(&value.plugin_type)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &RemovePluginV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_4]).write(&(), writer)?;
        PluginTypeCodec.write(&value.plugin_type, writer)?;
        Ok(())
    }
}
impl Decoder<RemovePluginV1> for RemovePluginV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RemovePluginV1, Error> {
        Constant(&[INSTRUCTION_TAG_4]).read(reader)?;
        Ok(RemovePluginV1 {
            plugin_type: PluginTypeCodec.read(reader)?,
        })
    }
}
impl RemovePluginV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = RemovePluginV1Codec.encode(self, limits.max_bytes)?;
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
        RemovePluginV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RemoveCollectionPluginV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoveCollectionPluginV1 {
    /// The `plugin_type` wire field.
    pub plugin_type: PluginType,
}
/// Bounded codec for [`RemoveCollectionPluginV1`].
#[derive(Clone, Copy, Debug)]
pub struct RemoveCollectionPluginV1Codec;
impl Encoder<RemoveCollectionPluginV1> for RemoveCollectionPluginV1Codec {
    fn encoded_len(&self, value: &RemoveCollectionPluginV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PluginTypeCodec.encoded_len(&value.plugin_type)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &RemoveCollectionPluginV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_5]).write(&(), writer)?;
        PluginTypeCodec.write(&value.plugin_type, writer)?;
        Ok(())
    }
}
impl Decoder<RemoveCollectionPluginV1> for RemoveCollectionPluginV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RemoveCollectionPluginV1, Error> {
        Constant(&[INSTRUCTION_TAG_5]).read(reader)?;
        Ok(RemoveCollectionPluginV1 {
            plugin_type: PluginTypeCodec.read(reader)?,
        })
    }
}
impl RemoveCollectionPluginV1 {
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
                /* collection */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = RemoveCollectionPluginV1Codec.encode(self, limits.max_bytes)?;
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
        RemoveCollectionPluginV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdatePluginV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdatePluginV1 {
    /// The `plugin` wire field.
    pub plugin: Plugin,
}
/// Bounded codec for [`UpdatePluginV1`].
#[derive(Clone, Copy, Debug)]
pub struct UpdatePluginV1Codec;
impl Encoder<UpdatePluginV1> for UpdatePluginV1Codec {
    fn encoded_len(&self, value: &UpdatePluginV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PluginCodec.encoded_len(&value.plugin)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdatePluginV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_6]).write(&(), writer)?;
        PluginCodec.write(&value.plugin, writer)?;
        Ok(())
    }
}
impl Decoder<UpdatePluginV1> for UpdatePluginV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdatePluginV1, Error> {
        Constant(&[INSTRUCTION_TAG_6]).read(reader)?;
        Ok(UpdatePluginV1 {
            plugin: PluginCodec.read(reader)?,
        })
    }
}
impl UpdatePluginV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = UpdatePluginV1Codec.encode(self, limits.max_bytes)?;
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
        UpdatePluginV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateCollectionPluginV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateCollectionPluginV1 {
    /// The `plugin` wire field.
    pub plugin: Plugin,
}
/// Bounded codec for [`UpdateCollectionPluginV1`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateCollectionPluginV1Codec;
impl Encoder<UpdateCollectionPluginV1> for UpdateCollectionPluginV1Codec {
    fn encoded_len(&self, value: &UpdateCollectionPluginV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PluginCodec.encoded_len(&value.plugin)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateCollectionPluginV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_7]).write(&(), writer)?;
        PluginCodec.write(&value.plugin, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateCollectionPluginV1> for UpdateCollectionPluginV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateCollectionPluginV1, Error> {
        Constant(&[INSTRUCTION_TAG_7]).read(reader)?;
        Ok(UpdateCollectionPluginV1 {
            plugin: PluginCodec.read(reader)?,
        })
    }
}
impl UpdateCollectionPluginV1 {
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
                /* collection */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = UpdateCollectionPluginV1Codec.encode(self, limits.max_bytes)?;
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
        UpdateCollectionPluginV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ApprovePluginAuthorityV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApprovePluginAuthorityV1 {
    /// The `plugin_type` wire field.
    pub plugin_type: PluginType,
    /// The `new_authority` wire field.
    pub new_authority: Authority,
}
/// Bounded codec for [`ApprovePluginAuthorityV1`].
#[derive(Clone, Copy, Debug)]
pub struct ApprovePluginAuthorityV1Codec;
impl Encoder<ApprovePluginAuthorityV1> for ApprovePluginAuthorityV1Codec {
    fn encoded_len(&self, value: &ApprovePluginAuthorityV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PluginTypeCodec.encoded_len(&value.plugin_type)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(AuthorityCodec.encoded_len(&value.new_authority)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ApprovePluginAuthorityV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_8]).write(&(), writer)?;
        PluginTypeCodec.write(&value.plugin_type, writer)?;
        AuthorityCodec.write(&value.new_authority, writer)?;
        Ok(())
    }
}
impl Decoder<ApprovePluginAuthorityV1> for ApprovePluginAuthorityV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ApprovePluginAuthorityV1, Error> {
        Constant(&[INSTRUCTION_TAG_8]).read(reader)?;
        Ok(ApprovePluginAuthorityV1 {
            plugin_type: PluginTypeCodec.read(reader)?,
            new_authority: AuthorityCodec.read(reader)?,
        })
    }
}
impl ApprovePluginAuthorityV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = ApprovePluginAuthorityV1Codec.encode(self, limits.max_bytes)?;
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
        ApprovePluginAuthorityV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ApproveCollectionPluginAuthorityV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApproveCollectionPluginAuthorityV1 {
    /// The `plugin_type` wire field.
    pub plugin_type: PluginType,
    /// The `new_authority` wire field.
    pub new_authority: Authority,
}
/// Bounded codec for [`ApproveCollectionPluginAuthorityV1`].
#[derive(Clone, Copy, Debug)]
pub struct ApproveCollectionPluginAuthorityV1Codec;
impl Encoder<ApproveCollectionPluginAuthorityV1> for ApproveCollectionPluginAuthorityV1Codec {
    fn encoded_len(&self, value: &ApproveCollectionPluginAuthorityV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PluginTypeCodec.encoded_len(&value.plugin_type)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(AuthorityCodec.encoded_len(&value.new_authority)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ApproveCollectionPluginAuthorityV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_9]).write(&(), writer)?;
        PluginTypeCodec.write(&value.plugin_type, writer)?;
        AuthorityCodec.write(&value.new_authority, writer)?;
        Ok(())
    }
}
impl Decoder<ApproveCollectionPluginAuthorityV1> for ApproveCollectionPluginAuthorityV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ApproveCollectionPluginAuthorityV1, Error> {
        Constant(&[INSTRUCTION_TAG_9]).read(reader)?;
        Ok(ApproveCollectionPluginAuthorityV1 {
            plugin_type: PluginTypeCodec.read(reader)?,
            new_authority: AuthorityCodec.read(reader)?,
        })
    }
}
impl ApproveCollectionPluginAuthorityV1 {
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
                /* collection */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = ApproveCollectionPluginAuthorityV1Codec.encode(self, limits.max_bytes)?;
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
        ApproveCollectionPluginAuthorityV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RevokePluginAuthorityV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevokePluginAuthorityV1 {
    /// The `plugin_type` wire field.
    pub plugin_type: PluginType,
}
/// Bounded codec for [`RevokePluginAuthorityV1`].
#[derive(Clone, Copy, Debug)]
pub struct RevokePluginAuthorityV1Codec;
impl Encoder<RevokePluginAuthorityV1> for RevokePluginAuthorityV1Codec {
    fn encoded_len(&self, value: &RevokePluginAuthorityV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PluginTypeCodec.encoded_len(&value.plugin_type)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &RevokePluginAuthorityV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_10]).write(&(), writer)?;
        PluginTypeCodec.write(&value.plugin_type, writer)?;
        Ok(())
    }
}
impl Decoder<RevokePluginAuthorityV1> for RevokePluginAuthorityV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RevokePluginAuthorityV1, Error> {
        Constant(&[INSTRUCTION_TAG_10]).read(reader)?;
        Ok(RevokePluginAuthorityV1 {
            plugin_type: PluginTypeCodec.read(reader)?,
        })
    }
}
impl RevokePluginAuthorityV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = RevokePluginAuthorityV1Codec.encode(self, limits.max_bytes)?;
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
        RevokePluginAuthorityV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RevokeCollectionPluginAuthorityV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevokeCollectionPluginAuthorityV1 {
    /// The `plugin_type` wire field.
    pub plugin_type: PluginType,
}
/// Bounded codec for [`RevokeCollectionPluginAuthorityV1`].
#[derive(Clone, Copy, Debug)]
pub struct RevokeCollectionPluginAuthorityV1Codec;
impl Encoder<RevokeCollectionPluginAuthorityV1> for RevokeCollectionPluginAuthorityV1Codec {
    fn encoded_len(&self, value: &RevokeCollectionPluginAuthorityV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PluginTypeCodec.encoded_len(&value.plugin_type)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &RevokeCollectionPluginAuthorityV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_11]).write(&(), writer)?;
        PluginTypeCodec.write(&value.plugin_type, writer)?;
        Ok(())
    }
}
impl Decoder<RevokeCollectionPluginAuthorityV1> for RevokeCollectionPluginAuthorityV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RevokeCollectionPluginAuthorityV1, Error> {
        Constant(&[INSTRUCTION_TAG_11]).read(reader)?;
        Ok(RevokeCollectionPluginAuthorityV1 {
            plugin_type: PluginTypeCodec.read(reader)?,
        })
    }
}
impl RevokeCollectionPluginAuthorityV1 {
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
                /* collection */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = RevokeCollectionPluginAuthorityV1Codec.encode(self, limits.max_bytes)?;
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
        RevokeCollectionPluginAuthorityV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `BurnV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnV1 {
    /// The `compression_proof` wire field.
    pub compression_proof: Option<CompressionProof>,
}
/// Bounded codec for [`BurnV1`].
#[derive(Clone, Copy, Debug)]
pub struct BurnV1Codec;
impl Encoder<BurnV1> for BurnV1Codec {
    fn encoded_len(&self, value: &BurnV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                OptionCodec::u8(CompressionProofCodec).encoded_len(&value.compression_proof)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &BurnV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_12]).write(&(), writer)?;
        OptionCodec::u8(CompressionProofCodec).write(&value.compression_proof, writer)?;
        Ok(())
    }
}
impl Decoder<BurnV1> for BurnV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BurnV1, Error> {
        Constant(&[INSTRUCTION_TAG_12]).read(reader)?;
        Ok(BurnV1 {
            compression_proof: OptionCodec::u8(CompressionProofCodec).read(reader)?,
        })
    }
}
impl BurnV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), true),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = BurnV1Codec.encode(self, limits.max_bytes)?;
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
        BurnV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `BurnCollectionV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnCollectionV1 {
    /// The `compression_proof` wire field.
    pub compression_proof: Option<CompressionProof>,
}
/// Bounded codec for [`BurnCollectionV1`].
#[derive(Clone, Copy, Debug)]
pub struct BurnCollectionV1Codec;
impl Encoder<BurnCollectionV1> for BurnCollectionV1Codec {
    fn encoded_len(&self, value: &BurnCollectionV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                OptionCodec::u8(CompressionProofCodec).encoded_len(&value.compression_proof)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &BurnCollectionV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_13]).write(&(), writer)?;
        OptionCodec::u8(CompressionProofCodec).write(&value.compression_proof, writer)?;
        Ok(())
    }
}
impl Decoder<BurnCollectionV1> for BurnCollectionV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BurnCollectionV1, Error> {
        Constant(&[INSTRUCTION_TAG_13]).read(reader)?;
        Ok(BurnCollectionV1 {
            compression_proof: OptionCodec::u8(CompressionProofCodec).read(reader)?,
        })
    }
}
impl BurnCollectionV1 {
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
                /* collection */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (true, Some(true), true),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = BurnCollectionV1Codec.encode(self, limits.max_bytes)?;
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
        BurnCollectionV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TransferV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferV1 {
    /// The `compression_proof` wire field.
    pub compression_proof: Option<CompressionProof>,
}
/// Bounded codec for [`TransferV1`].
#[derive(Clone, Copy, Debug)]
pub struct TransferV1Codec;
impl Encoder<TransferV1> for TransferV1Codec {
    fn encoded_len(&self, value: &TransferV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                OptionCodec::u8(CompressionProofCodec).encoded_len(&value.compression_proof)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_14]).write(&(), writer)?;
        OptionCodec::u8(CompressionProofCodec).write(&value.compression_proof, writer)?;
        Ok(())
    }
}
impl Decoder<TransferV1> for TransferV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferV1, Error> {
        Constant(&[INSTRUCTION_TAG_14]).read(reader)?;
        Ok(TransferV1 {
            compression_proof: OptionCodec::u8(CompressionProofCodec).read(reader)?,
        })
    }
}
impl TransferV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (false, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* new_owner */ (false, Some(false), false),
                /* system_program */ (false, Some(false), true),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = TransferV1Codec.encode(self, limits.max_bytes)?;
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
        TransferV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateV1 {
    /// The `new_name` wire field.
    pub new_name: Option<String>,
    /// The `new_uri` wire field.
    pub new_uri: Option<String>,
    /// The `new_update_authority` wire field.
    pub new_update_authority: Option<UpdateAuthority>,
}
/// Bounded codec for [`UpdateV1`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateV1Codec;
impl Encoder<UpdateV1> for UpdateV1Codec {
    fn encoded_len(&self, value: &UpdateV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.new_name)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.new_uri)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(UpdateAuthorityCodec).encoded_len(&value.new_update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_15]).write(&(), writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.new_name, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.new_uri, writer)?;
        OptionCodec::u8(UpdateAuthorityCodec).write(&value.new_update_authority, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateV1> for UpdateV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateV1, Error> {
        Constant(&[INSTRUCTION_TAG_15]).read(reader)?;
        Ok(UpdateV1 {
            new_name: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
            new_uri: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
            new_update_authority: OptionCodec::u8(UpdateAuthorityCodec).read(reader)?,
        })
    }
}
impl UpdateV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (false, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = UpdateV1Codec.encode(self, limits.max_bytes)?;
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
        UpdateV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateCollectionV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateCollectionV1 {
    /// The `new_name` wire field.
    pub new_name: Option<String>,
    /// The `new_uri` wire field.
    pub new_uri: Option<String>,
}
/// Bounded codec for [`UpdateCollectionV1`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateCollectionV1Codec;
impl Encoder<UpdateCollectionV1> for UpdateCollectionV1Codec {
    fn encoded_len(&self, value: &UpdateCollectionV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.new_name)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.new_uri)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateCollectionV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_16]).write(&(), writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.new_name, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.new_uri, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateCollectionV1> for UpdateCollectionV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateCollectionV1, Error> {
        Constant(&[INSTRUCTION_TAG_16]).read(reader)?;
        Ok(UpdateCollectionV1 {
            new_name: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
            new_uri: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
        })
    }
}
impl UpdateCollectionV1 {
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
                /* collection */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* new_update_authority */ (false, Some(false), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = UpdateCollectionV1Codec.encode(self, limits.max_bytes)?;
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
        UpdateCollectionV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CompressV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompressV1 {}
/// Bounded codec for [`CompressV1`].
#[derive(Clone, Copy, Debug)]
pub struct CompressV1Codec;
impl Encoder<CompressV1> for CompressV1Codec {
    fn encoded_len(&self, value: &CompressV1) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CompressV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_17]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CompressV1> for CompressV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CompressV1, Error> {
        Constant(&[INSTRUCTION_TAG_17]).read(reader)?;
        let _ = reader;
        Ok(CompressV1 {})
    }
}
impl CompressV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (false, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = CompressV1Codec.encode(self, limits.max_bytes)?;
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
        CompressV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `DecompressV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecompressV1 {
    /// The `compression_proof` wire field.
    pub compression_proof: CompressionProof,
}
/// Bounded codec for [`DecompressV1`].
#[derive(Clone, Copy, Debug)]
pub struct DecompressV1Codec;
impl Encoder<DecompressV1> for DecompressV1Codec {
    fn encoded_len(&self, value: &DecompressV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(CompressionProofCodec.encoded_len(&value.compression_proof)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &DecompressV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_18]).write(&(), writer)?;
        CompressionProofCodec.write(&value.compression_proof, writer)?;
        Ok(())
    }
}
impl Decoder<DecompressV1> for DecompressV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DecompressV1, Error> {
        Constant(&[INSTRUCTION_TAG_18]).read(reader)?;
        Ok(DecompressV1 {
            compression_proof: CompressionProofCodec.read(reader)?,
        })
    }
}
impl DecompressV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (false, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = DecompressV1Codec.encode(self, limits.max_bytes)?;
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
        DecompressV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Collect`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Collect {}
/// Bounded codec for [`Collect`].
#[derive(Clone, Copy, Debug)]
pub struct CollectCodec;
impl Encoder<Collect> for CollectCodec {
    fn encoded_len(&self, value: &Collect) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Collect, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_19]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Collect> for CollectCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Collect, Error> {
        Constant(&[INSTRUCTION_TAG_19]).read(reader)?;
        let _ = reader;
        Ok(Collect {})
    }
}
impl Collect {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 2 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* recipient1 */ (true, Some(false), false),
                /* recipient2 */ (true, Some(false), false),
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
        let data = CollectCodec.encode(self, limits.max_bytes)?;
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
        CollectCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateV2 {
    /// The `data_state` wire field.
    pub data_state: DataState,
    /// The `name` wire field.
    pub name: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `plugins` wire field.
    pub plugins: Option<Vec<PluginAuthorityPair>>,
    /// The `external_plugin_adapters` wire field.
    pub external_plugin_adapters: Option<Vec<ExternalPluginAdapterInitInfo>>,
}
/// Bounded codec for [`CreateV2`].
#[derive(Clone, Copy, Debug)]
pub struct CreateV2Codec;
impl Encoder<CreateV2> for CreateV2Codec {
    fn encoded_len(&self, value: &CreateV2) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(DataStateCodec.encoded_len(&value.data_state)?)
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
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: PluginAuthorityPairCodec,
                    minimum: 2,
                }))
                .encoded_len(&value.plugins)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ExternalPluginAdapterInitInfoCodec,
                    minimum: 4,
                }))
                .encoded_len(&value.external_plugin_adapters)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_20]).write(&(), writer)?;
        DataStateCodec.write(&value.data_state, writer)?;
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
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: PluginAuthorityPairCodec,
            minimum: 2,
        }))
        .write(&value.plugins, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ExternalPluginAdapterInitInfoCodec,
            minimum: 4,
        }))
        .write(&value.external_plugin_adapters, writer)?;
        Ok(())
    }
}
impl Decoder<CreateV2> for CreateV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateV2, Error> {
        Constant(&[INSTRUCTION_TAG_20]).read(reader)?;
        Ok(CreateV2 {
            data_state: DataStateCodec.read(reader)?,
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
            plugins: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: PluginAuthorityPairCodec,
                minimum: 2,
            }))
            .read(reader)?,
            external_plugin_adapters: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ExternalPluginAdapterInitInfoCodec,
                minimum: 4,
            }))
            .read(reader)?,
        })
    }
}
impl CreateV2 {
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
                /* asset */ (true, Some(true), false),
                /* collection */ (true, Some(false), true),
                /* authority */ (false, Some(true), true),
                /* payer */ (true, Some(true), false),
                /* owner */ (false, Some(false), true),
                /* update_authority */ (false, Some(false), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = CreateV2Codec.encode(self, limits.max_bytes)?;
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
        CreateV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateCollectionV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateCollectionV2 {
    /// The `name` wire field.
    pub name: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `plugins` wire field.
    pub plugins: Option<Vec<PluginAuthorityPair>>,
    /// The `external_plugin_adapters` wire field.
    pub external_plugin_adapters: Option<Vec<ExternalPluginAdapterInitInfo>>,
}
/// Bounded codec for [`CreateCollectionV2`].
#[derive(Clone, Copy, Debug)]
pub struct CreateCollectionV2Codec;
impl Encoder<CreateCollectionV2> for CreateCollectionV2Codec {
    fn encoded_len(&self, value: &CreateCollectionV2) -> Result<usize, Error> {
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
                .encoded_len(value.uri.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: PluginAuthorityPairCodec,
                    minimum: 2,
                }))
                .encoded_len(&value.plugins)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ExternalPluginAdapterInitInfoCodec,
                    minimum: 4,
                }))
                .encoded_len(&value.external_plugin_adapters)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateCollectionV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_21]).write(&(), writer)?;
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
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: PluginAuthorityPairCodec,
            minimum: 2,
        }))
        .write(&value.plugins, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ExternalPluginAdapterInitInfoCodec,
            minimum: 4,
        }))
        .write(&value.external_plugin_adapters, writer)?;
        Ok(())
    }
}
impl Decoder<CreateCollectionV2> for CreateCollectionV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateCollectionV2, Error> {
        Constant(&[INSTRUCTION_TAG_21]).read(reader)?;
        Ok(CreateCollectionV2 {
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
            plugins: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: PluginAuthorityPairCodec,
                minimum: 2,
            }))
            .read(reader)?,
            external_plugin_adapters: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ExternalPluginAdapterInitInfoCodec,
                minimum: 4,
            }))
            .read(reader)?,
        })
    }
}
impl CreateCollectionV2 {
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
                /* collection */ (true, Some(true), false),
                /* update_authority */ (false, Some(false), true),
                /* payer */ (true, Some(true), false),
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
        let data = CreateCollectionV2Codec.encode(self, limits.max_bytes)?;
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
        CreateCollectionV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `AddExternalPluginAdapterV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddExternalPluginAdapterV1 {
    /// The `init_info` wire field.
    pub init_info: ExternalPluginAdapterInitInfo,
}
/// Bounded codec for [`AddExternalPluginAdapterV1`].
#[derive(Clone, Copy, Debug)]
pub struct AddExternalPluginAdapterV1Codec;
impl Encoder<AddExternalPluginAdapterV1> for AddExternalPluginAdapterV1Codec {
    fn encoded_len(&self, value: &AddExternalPluginAdapterV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(ExternalPluginAdapterInitInfoCodec.encoded_len(&value.init_info)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &AddExternalPluginAdapterV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_22]).write(&(), writer)?;
        ExternalPluginAdapterInitInfoCodec.write(&value.init_info, writer)?;
        Ok(())
    }
}
impl Decoder<AddExternalPluginAdapterV1> for AddExternalPluginAdapterV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AddExternalPluginAdapterV1, Error> {
        Constant(&[INSTRUCTION_TAG_22]).read(reader)?;
        Ok(AddExternalPluginAdapterV1 {
            init_info: ExternalPluginAdapterInitInfoCodec.read(reader)?,
        })
    }
}
impl AddExternalPluginAdapterV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = AddExternalPluginAdapterV1Codec.encode(self, limits.max_bytes)?;
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
        AddExternalPluginAdapterV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `AddCollectionExternalPluginAdapterV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddCollectionExternalPluginAdapterV1 {
    /// The `init_info` wire field.
    pub init_info: ExternalPluginAdapterInitInfo,
}
/// Bounded codec for [`AddCollectionExternalPluginAdapterV1`].
#[derive(Clone, Copy, Debug)]
pub struct AddCollectionExternalPluginAdapterV1Codec;
impl Encoder<AddCollectionExternalPluginAdapterV1> for AddCollectionExternalPluginAdapterV1Codec {
    fn encoded_len(&self, value: &AddCollectionExternalPluginAdapterV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(ExternalPluginAdapterInitInfoCodec.encoded_len(&value.init_info)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &AddCollectionExternalPluginAdapterV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_23]).write(&(), writer)?;
        ExternalPluginAdapterInitInfoCodec.write(&value.init_info, writer)?;
        Ok(())
    }
}
impl Decoder<AddCollectionExternalPluginAdapterV1> for AddCollectionExternalPluginAdapterV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AddCollectionExternalPluginAdapterV1, Error> {
        Constant(&[INSTRUCTION_TAG_23]).read(reader)?;
        Ok(AddCollectionExternalPluginAdapterV1 {
            init_info: ExternalPluginAdapterInitInfoCodec.read(reader)?,
        })
    }
}
impl AddCollectionExternalPluginAdapterV1 {
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
                /* collection */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = AddCollectionExternalPluginAdapterV1Codec.encode(self, limits.max_bytes)?;
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
        AddCollectionExternalPluginAdapterV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RemoveExternalPluginAdapterV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoveExternalPluginAdapterV1 {
    /// The `key` wire field.
    pub key: ExternalPluginAdapterKey,
}
/// Bounded codec for [`RemoveExternalPluginAdapterV1`].
#[derive(Clone, Copy, Debug)]
pub struct RemoveExternalPluginAdapterV1Codec;
impl Encoder<RemoveExternalPluginAdapterV1> for RemoveExternalPluginAdapterV1Codec {
    fn encoded_len(&self, value: &RemoveExternalPluginAdapterV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(ExternalPluginAdapterKeyCodec.encoded_len(&value.key)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &RemoveExternalPluginAdapterV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_24]).write(&(), writer)?;
        ExternalPluginAdapterKeyCodec.write(&value.key, writer)?;
        Ok(())
    }
}
impl Decoder<RemoveExternalPluginAdapterV1> for RemoveExternalPluginAdapterV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RemoveExternalPluginAdapterV1, Error> {
        Constant(&[INSTRUCTION_TAG_24]).read(reader)?;
        Ok(RemoveExternalPluginAdapterV1 {
            key: ExternalPluginAdapterKeyCodec.read(reader)?,
        })
    }
}
impl RemoveExternalPluginAdapterV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = RemoveExternalPluginAdapterV1Codec.encode(self, limits.max_bytes)?;
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
        RemoveExternalPluginAdapterV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RemoveCollectionExternalPluginAdapterV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoveCollectionExternalPluginAdapterV1 {
    /// The `key` wire field.
    pub key: ExternalPluginAdapterKey,
}
/// Bounded codec for [`RemoveCollectionExternalPluginAdapterV1`].
#[derive(Clone, Copy, Debug)]
pub struct RemoveCollectionExternalPluginAdapterV1Codec;
impl Encoder<RemoveCollectionExternalPluginAdapterV1>
    for RemoveCollectionExternalPluginAdapterV1Codec
{
    fn encoded_len(&self, value: &RemoveCollectionExternalPluginAdapterV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(ExternalPluginAdapterKeyCodec.encoded_len(&value.key)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &RemoveCollectionExternalPluginAdapterV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_25]).write(&(), writer)?;
        ExternalPluginAdapterKeyCodec.write(&value.key, writer)?;
        Ok(())
    }
}
impl Decoder<RemoveCollectionExternalPluginAdapterV1>
    for RemoveCollectionExternalPluginAdapterV1Codec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<RemoveCollectionExternalPluginAdapterV1, Error> {
        Constant(&[INSTRUCTION_TAG_25]).read(reader)?;
        Ok(RemoveCollectionExternalPluginAdapterV1 {
            key: ExternalPluginAdapterKeyCodec.read(reader)?,
        })
    }
}
impl RemoveCollectionExternalPluginAdapterV1 {
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
                /* collection */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = RemoveCollectionExternalPluginAdapterV1Codec.encode(self, limits.max_bytes)?;
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
        RemoveCollectionExternalPluginAdapterV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateExternalPluginAdapterV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateExternalPluginAdapterV1 {
    /// The `key` wire field.
    pub key: ExternalPluginAdapterKey,
    /// The `update_info` wire field.
    pub update_info: ExternalPluginAdapterUpdateInfo,
}
/// Bounded codec for [`UpdateExternalPluginAdapterV1`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateExternalPluginAdapterV1Codec;
impl Encoder<UpdateExternalPluginAdapterV1> for UpdateExternalPluginAdapterV1Codec {
    fn encoded_len(&self, value: &UpdateExternalPluginAdapterV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(ExternalPluginAdapterKeyCodec.encoded_len(&value.key)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalPluginAdapterUpdateInfoCodec.encoded_len(&value.update_info)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateExternalPluginAdapterV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_26]).write(&(), writer)?;
        ExternalPluginAdapterKeyCodec.write(&value.key, writer)?;
        ExternalPluginAdapterUpdateInfoCodec.write(&value.update_info, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateExternalPluginAdapterV1> for UpdateExternalPluginAdapterV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateExternalPluginAdapterV1, Error> {
        Constant(&[INSTRUCTION_TAG_26]).read(reader)?;
        Ok(UpdateExternalPluginAdapterV1 {
            key: ExternalPluginAdapterKeyCodec.read(reader)?,
            update_info: ExternalPluginAdapterUpdateInfoCodec.read(reader)?,
        })
    }
}
impl UpdateExternalPluginAdapterV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = UpdateExternalPluginAdapterV1Codec.encode(self, limits.max_bytes)?;
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
        UpdateExternalPluginAdapterV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateCollectionExternalPluginAdapterV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateCollectionExternalPluginAdapterV1 {
    /// The `key` wire field.
    pub key: ExternalPluginAdapterKey,
    /// The `update_info` wire field.
    pub update_info: ExternalPluginAdapterUpdateInfo,
}
/// Bounded codec for [`UpdateCollectionExternalPluginAdapterV1`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateCollectionExternalPluginAdapterV1Codec;
impl Encoder<UpdateCollectionExternalPluginAdapterV1>
    for UpdateCollectionExternalPluginAdapterV1Codec
{
    fn encoded_len(&self, value: &UpdateCollectionExternalPluginAdapterV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(ExternalPluginAdapterKeyCodec.encoded_len(&value.key)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ExternalPluginAdapterUpdateInfoCodec.encoded_len(&value.update_info)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateCollectionExternalPluginAdapterV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_27]).write(&(), writer)?;
        ExternalPluginAdapterKeyCodec.write(&value.key, writer)?;
        ExternalPluginAdapterUpdateInfoCodec.write(&value.update_info, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateCollectionExternalPluginAdapterV1>
    for UpdateCollectionExternalPluginAdapterV1Codec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<UpdateCollectionExternalPluginAdapterV1, Error> {
        Constant(&[INSTRUCTION_TAG_27]).read(reader)?;
        Ok(UpdateCollectionExternalPluginAdapterV1 {
            key: ExternalPluginAdapterKeyCodec.read(reader)?,
            update_info: ExternalPluginAdapterUpdateInfoCodec.read(reader)?,
        })
    }
}
impl UpdateCollectionExternalPluginAdapterV1 {
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
                /* collection */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = UpdateCollectionExternalPluginAdapterV1Codec.encode(self, limits.max_bytes)?;
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
        UpdateCollectionExternalPluginAdapterV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `WriteExternalPluginAdapterDataV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WriteExternalPluginAdapterDataV1 {
    /// The `key` wire field.
    pub key: ExternalPluginAdapterKey,
    /// The `data` wire field.
    pub data: Option<Vec<u8>>,
}
/// Bounded codec for [`WriteExternalPluginAdapterDataV1`].
#[derive(Clone, Copy, Debug)]
pub struct WriteExternalPluginAdapterDataV1Codec;
impl Encoder<WriteExternalPluginAdapterDataV1> for WriteExternalPluginAdapterDataV1Codec {
    fn encoded_len(&self, value: &WriteExternalPluginAdapterDataV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(ExternalPluginAdapterKeyCodec.encoded_len(&value.key)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }))
                .encoded_len(&value.data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &WriteExternalPluginAdapterDataV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_28]).write(&(), writer)?;
        ExternalPluginAdapterKeyCodec.write(&value.key, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }))
        .write(&value.data, writer)?;
        Ok(())
    }
}
impl Decoder<WriteExternalPluginAdapterDataV1> for WriteExternalPluginAdapterDataV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<WriteExternalPluginAdapterDataV1, Error> {
        Constant(&[INSTRUCTION_TAG_28]).read(reader)?;
        Ok(WriteExternalPluginAdapterDataV1 {
            key: ExternalPluginAdapterKeyCodec.read(reader)?,
            data: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }))
            .read(reader)?,
        })
    }
}
impl WriteExternalPluginAdapterDataV1 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* buffer */ (false, Some(false), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = WriteExternalPluginAdapterDataV1Codec.encode(self, limits.max_bytes)?;
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
        WriteExternalPluginAdapterDataV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `WriteCollectionExternalPluginAdapterDataV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WriteCollectionExternalPluginAdapterDataV1 {
    /// The `key` wire field.
    pub key: ExternalPluginAdapterKey,
    /// The `data` wire field.
    pub data: Option<Vec<u8>>,
}
/// Bounded codec for [`WriteCollectionExternalPluginAdapterDataV1`].
#[derive(Clone, Copy, Debug)]
pub struct WriteCollectionExternalPluginAdapterDataV1Codec;
impl Encoder<WriteCollectionExternalPluginAdapterDataV1>
    for WriteCollectionExternalPluginAdapterDataV1Codec
{
    fn encoded_len(
        &self,
        value: &WriteCollectionExternalPluginAdapterDataV1,
    ) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(ExternalPluginAdapterKeyCodec.encoded_len(&value.key)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }))
                .encoded_len(&value.data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &WriteCollectionExternalPluginAdapterDataV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_29]).write(&(), writer)?;
        ExternalPluginAdapterKeyCodec.write(&value.key, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }))
        .write(&value.data, writer)?;
        Ok(())
    }
}
impl Decoder<WriteCollectionExternalPluginAdapterDataV1>
    for WriteCollectionExternalPluginAdapterDataV1Codec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<WriteCollectionExternalPluginAdapterDataV1, Error> {
        Constant(&[INSTRUCTION_TAG_29]).read(reader)?;
        Ok(WriteCollectionExternalPluginAdapterDataV1 {
            key: ExternalPluginAdapterKeyCodec.read(reader)?,
            data: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }))
            .read(reader)?,
        })
    }
}
impl WriteCollectionExternalPluginAdapterDataV1 {
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
                /* collection */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* buffer */ (false, Some(false), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data =
            WriteCollectionExternalPluginAdapterDataV1Codec.encode(self, limits.max_bytes)?;
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
        WriteCollectionExternalPluginAdapterDataV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateV2 {
    /// The `new_name` wire field.
    pub new_name: Option<String>,
    /// The `new_uri` wire field.
    pub new_uri: Option<String>,
    /// The `new_update_authority` wire field.
    pub new_update_authority: Option<UpdateAuthority>,
}
/// Bounded codec for [`UpdateV2`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateV2Codec;
impl Encoder<UpdateV2> for UpdateV2Codec {
    fn encoded_len(&self, value: &UpdateV2) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.new_name)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.new_uri)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(UpdateAuthorityCodec).encoded_len(&value.new_update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_30]).write(&(), writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.new_name, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.new_uri, writer)?;
        OptionCodec::u8(UpdateAuthorityCodec).write(&value.new_update_authority, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateV2> for UpdateV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateV2, Error> {
        Constant(&[INSTRUCTION_TAG_30]).read(reader)?;
        Ok(UpdateV2 {
            new_name: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
            new_uri: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
            new_update_authority: OptionCodec::u8(UpdateAuthorityCodec).read(reader)?,
        })
    }
}
impl UpdateV2 {
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
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* new_collection */ (true, Some(false), true),
                /* system_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), true),
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
        let data = UpdateV2Codec.encode(self, limits.max_bytes)?;
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
        UpdateV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ExecuteV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecuteV1 {
    /// The `instruction_data` wire field.
    pub instruction_data: Vec<u8>,
}
/// Bounded codec for [`ExecuteV1`].
#[derive(Clone, Copy, Debug)]
pub struct ExecuteV1Codec;
impl Encoder<ExecuteV1> for ExecuteV1Codec {
    fn encoded_len(&self, value: &ExecuteV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.instruction_data.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ExecuteV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_31]).write(&(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.instruction_data.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<ExecuteV1> for ExecuteV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExecuteV1, Error> {
        Constant(&[INSTRUCTION_TAG_31]).read(reader)?;
        Ok(ExecuteV1 {
            instruction_data: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
        })
    }
}
impl ExecuteV1 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* asset */ (true, Some(false), false),
                /* collection */ (true, Some(false), true),
                /* asset_signer */ (false, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* system_program */ (false, Some(false), false),
                /* program_id */ (false, Some(false), false),
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
        let data = ExecuteV1Codec.encode(self, limits.max_bytes)?;
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
        ExecuteV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateCollectionInfoV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateCollectionInfoV1 {
    /// The `update_type` wire field.
    pub update_type: UpdateType,
    /// The `amount` wire field.
    pub amount: u32,
}
/// Bounded codec for [`UpdateCollectionInfoV1`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateCollectionInfoV1Codec;
impl Encoder<UpdateCollectionInfoV1> for UpdateCollectionInfoV1Codec {
    fn encoded_len(&self, value: &UpdateCollectionInfoV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(UpdateTypeCodec.encoded_len(&value.update_type)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateCollectionInfoV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_32]).write(&(), writer)?;
        UpdateTypeCodec.write(&value.update_type, writer)?;
        Number::<u32>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateCollectionInfoV1> for UpdateCollectionInfoV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateCollectionInfoV1, Error> {
        Constant(&[INSTRUCTION_TAG_32]).read(reader)?;
        Ok(UpdateCollectionInfoV1 {
            update_type: UpdateTypeCodec.read(reader)?,
            amount: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl UpdateCollectionInfoV1 {
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
                /* collection */ (true, Some(false), false),
                /* bubblegum_signer */ (false, Some(true), false),
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
        let data = UpdateCollectionInfoV1Codec.encode(self, limits.max_bytes)?;
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
        UpdateCollectionInfoV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `AddCollectionsToGroupV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddCollectionsToGroupV1 {}
/// Bounded codec for [`AddCollectionsToGroupV1`].
#[derive(Clone, Copy, Debug)]
pub struct AddCollectionsToGroupV1Codec;
impl Encoder<AddCollectionsToGroupV1> for AddCollectionsToGroupV1Codec {
    fn encoded_len(&self, value: &AddCollectionsToGroupV1) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &AddCollectionsToGroupV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_33]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<AddCollectionsToGroupV1> for AddCollectionsToGroupV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AddCollectionsToGroupV1, Error> {
        Constant(&[INSTRUCTION_TAG_33]).read(reader)?;
        let _ = reader;
        Ok(AddCollectionsToGroupV1 {})
    }
}
impl AddCollectionsToGroupV1 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* group */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
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
        let data = AddCollectionsToGroupV1Codec.encode(self, limits.max_bytes)?;
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
        AddCollectionsToGroupV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RemoveCollectionsFromGroupV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoveCollectionsFromGroupV1 {
    /// The `collections` wire field.
    pub collections: Vec<Address>,
}
/// Bounded codec for [`RemoveCollectionsFromGroupV1`].
#[derive(Clone, Copy, Debug)]
pub struct RemoveCollectionsFromGroupV1Codec;
impl Encoder<RemoveCollectionsFromGroupV1> for RemoveCollectionsFromGroupV1Codec {
    fn encoded_len(&self, value: &RemoveCollectionsFromGroupV1) -> Result<usize, Error> {
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
                .encoded_len(value.collections.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &RemoveCollectionsFromGroupV1,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_34]).write(&(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.collections.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<RemoveCollectionsFromGroupV1> for RemoveCollectionsFromGroupV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RemoveCollectionsFromGroupV1, Error> {
        Constant(&[INSTRUCTION_TAG_34]).read(reader)?;
        Ok(RemoveCollectionsFromGroupV1 {
            collections: CheckedCounted {
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
impl RemoveCollectionsFromGroupV1 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* group */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
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
        let data = RemoveCollectionsFromGroupV1Codec.encode(self, limits.max_bytes)?;
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
        RemoveCollectionsFromGroupV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `AddAssetsToGroupV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddAssetsToGroupV1 {}
/// Bounded codec for [`AddAssetsToGroupV1`].
#[derive(Clone, Copy, Debug)]
pub struct AddAssetsToGroupV1Codec;
impl Encoder<AddAssetsToGroupV1> for AddAssetsToGroupV1Codec {
    fn encoded_len(&self, value: &AddAssetsToGroupV1) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &AddAssetsToGroupV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_35]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<AddAssetsToGroupV1> for AddAssetsToGroupV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AddAssetsToGroupV1, Error> {
        Constant(&[INSTRUCTION_TAG_35]).read(reader)?;
        let _ = reader;
        Ok(AddAssetsToGroupV1 {})
    }
}
impl AddAssetsToGroupV1 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* group */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
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
        let data = AddAssetsToGroupV1Codec.encode(self, limits.max_bytes)?;
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
        AddAssetsToGroupV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RemoveAssetsFromGroupV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoveAssetsFromGroupV1 {
    /// The `assets` wire field.
    pub assets: Vec<Address>,
}
/// Bounded codec for [`RemoveAssetsFromGroupV1`].
#[derive(Clone, Copy, Debug)]
pub struct RemoveAssetsFromGroupV1Codec;
impl Encoder<RemoveAssetsFromGroupV1> for RemoveAssetsFromGroupV1Codec {
    fn encoded_len(&self, value: &RemoveAssetsFromGroupV1) -> Result<usize, Error> {
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
                .encoded_len(value.assets.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &RemoveAssetsFromGroupV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_36]).write(&(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.assets.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<RemoveAssetsFromGroupV1> for RemoveAssetsFromGroupV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RemoveAssetsFromGroupV1, Error> {
        Constant(&[INSTRUCTION_TAG_36]).read(reader)?;
        Ok(RemoveAssetsFromGroupV1 {
            assets: CheckedCounted {
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
impl RemoveAssetsFromGroupV1 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* group */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
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
        let data = RemoveAssetsFromGroupV1Codec.encode(self, limits.max_bytes)?;
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
        RemoveAssetsFromGroupV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `AddGroupsToGroupV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddGroupsToGroupV1 {
    /// The `groups` wire field.
    pub groups: Vec<Address>,
}
/// Bounded codec for [`AddGroupsToGroupV1`].
#[derive(Clone, Copy, Debug)]
pub struct AddGroupsToGroupV1Codec;
impl Encoder<AddGroupsToGroupV1> for AddGroupsToGroupV1Codec {
    fn encoded_len(&self, value: &AddGroupsToGroupV1) -> Result<usize, Error> {
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
                .encoded_len(value.groups.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AddGroupsToGroupV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_37]).write(&(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.groups.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<AddGroupsToGroupV1> for AddGroupsToGroupV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AddGroupsToGroupV1, Error> {
        Constant(&[INSTRUCTION_TAG_37]).read(reader)?;
        Ok(AddGroupsToGroupV1 {
            groups: CheckedCounted {
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
impl AddGroupsToGroupV1 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* parent_group */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
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
        let data = AddGroupsToGroupV1Codec.encode(self, limits.max_bytes)?;
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
        AddGroupsToGroupV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RemoveGroupsFromGroupV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoveGroupsFromGroupV1 {
    /// The `groups` wire field.
    pub groups: Vec<Address>,
}
/// Bounded codec for [`RemoveGroupsFromGroupV1`].
#[derive(Clone, Copy, Debug)]
pub struct RemoveGroupsFromGroupV1Codec;
impl Encoder<RemoveGroupsFromGroupV1> for RemoveGroupsFromGroupV1Codec {
    fn encoded_len(&self, value: &RemoveGroupsFromGroupV1) -> Result<usize, Error> {
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
                .encoded_len(value.groups.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &RemoveGroupsFromGroupV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_38]).write(&(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.groups.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<RemoveGroupsFromGroupV1> for RemoveGroupsFromGroupV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RemoveGroupsFromGroupV1, Error> {
        Constant(&[INSTRUCTION_TAG_38]).read(reader)?;
        Ok(RemoveGroupsFromGroupV1 {
            groups: CheckedCounted {
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
impl RemoveGroupsFromGroupV1 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* parent_group */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
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
        let data = RemoveGroupsFromGroupV1Codec.encode(self, limits.max_bytes)?;
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
        RemoveGroupsFromGroupV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateGroupV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateGroupV1 {
    /// The `name` wire field.
    pub name: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `relationships` wire field.
    pub relationships: Vec<RelationshipEntry>,
}
/// Bounded codec for [`CreateGroupV1`].
#[derive(Clone, Copy, Debug)]
pub struct CreateGroupV1Codec;
impl Encoder<CreateGroupV1> for CreateGroupV1Codec {
    fn encoded_len(&self, value: &CreateGroupV1) -> Result<usize, Error> {
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
                .encoded_len(value.uri.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: RelationshipEntryCodec,
                    minimum: 33,
                }
                .encoded_len(value.relationships.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateGroupV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_39]).write(&(), writer)?;
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
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: RelationshipEntryCodec,
            minimum: 33,
        }
        .write(value.relationships.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<CreateGroupV1> for CreateGroupV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateGroupV1, Error> {
        Constant(&[INSTRUCTION_TAG_39]).read(reader)?;
        Ok(CreateGroupV1 {
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
            relationships: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: RelationshipEntryCodec,
                minimum: 33,
            }
            .read(reader)?,
        })
    }
}
impl CreateGroupV1 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* group */ (true, Some(true), false),
                /* update_authority */ (false, Some(true), true),
                /* payer */ (true, Some(true), false),
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
        let data = CreateGroupV1Codec.encode(self, limits.max_bytes)?;
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
        CreateGroupV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CloseGroupV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseGroupV1 {}
/// Bounded codec for [`CloseGroupV1`].
#[derive(Clone, Copy, Debug)]
pub struct CloseGroupV1Codec;
impl Encoder<CloseGroupV1> for CloseGroupV1Codec {
    fn encoded_len(&self, value: &CloseGroupV1) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CloseGroupV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_40]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CloseGroupV1> for CloseGroupV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseGroupV1, Error> {
        Constant(&[INSTRUCTION_TAG_40]).read(reader)?;
        let _ = reader;
        Ok(CloseGroupV1 {})
    }
}
impl CloseGroupV1 {
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
                /* group */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
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
        let data = CloseGroupV1Codec.encode(self, limits.max_bytes)?;
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
        CloseGroupV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateGroupV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateGroupV1 {
    /// The `new_name` wire field.
    pub new_name: Option<String>,
    /// The `new_uri` wire field.
    pub new_uri: Option<String>,
}
/// Bounded codec for [`UpdateGroupV1`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateGroupV1Codec;
impl Encoder<UpdateGroupV1> for UpdateGroupV1Codec {
    fn encoded_len(&self, value: &UpdateGroupV1) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.new_name)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.new_uri)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateGroupV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_41]).write(&(), writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.new_name, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.new_uri, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateGroupV1> for UpdateGroupV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateGroupV1, Error> {
        Constant(&[INSTRUCTION_TAG_41]).read(reader)?;
        Ok(UpdateGroupV1 {
            new_name: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
            new_uri: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
        })
    }
}
impl UpdateGroupV1 {
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
                /* group */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* new_update_authority */ (false, Some(false), true),
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
        let data = UpdateGroupV1Codec.encode(self, limits.max_bytes)?;
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
        UpdateGroupV1Codec.decode(&instruction.data, limits)
    }
}
/// Program error codes. Unknown numeric codes remain lossless.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MplCoreError {
    /// The `InvalidSystemProgram` program error.
    InvalidSystemProgram,
    /// The `DeserializationError` program error.
    DeserializationError,
    /// The `SerializationError` program error.
    SerializationError,
    /// The `PluginsNotInitialized` program error.
    PluginsNotInitialized,
    /// The `PluginNotFound` program error.
    PluginNotFound,
    /// The `NumericalOverflow` program error.
    NumericalOverflow,
    /// The `IncorrectAccount` program error.
    IncorrectAccount,
    /// The `IncorrectAssetHash` program error.
    IncorrectAssetHash,
    /// The `InvalidPlugin` program error.
    InvalidPlugin,
    /// The `InvalidAuthority` program error.
    InvalidAuthority,
    /// The `AssetIsFrozen` program error.
    AssetIsFrozen,
    /// The `MissingCompressionProof` program error.
    MissingCompressionProof,
    /// The `CannotMigrateMasterWithSupply` program error.
    CannotMigrateMasterWithSupply,
    /// The `CannotMigratePrints` program error.
    CannotMigratePrints,
    /// The `CannotBurnCollection` program error.
    CannotBurnCollection,
    /// The `PluginAlreadyExists` program error.
    PluginAlreadyExists,
    /// The `NumericalOverflowError` program error.
    NumericalOverflowError,
    /// The `AlreadyCompressed` program error.
    AlreadyCompressed,
    /// The `AlreadyDecompressed` program error.
    AlreadyDecompressed,
    /// The `InvalidCollection` program error.
    InvalidCollection,
    /// The `MissingUpdateAuthority` program error.
    MissingUpdateAuthority,
    /// The `MissingNewOwner` program error.
    MissingNewOwner,
    /// The `MissingSystemProgram` program error.
    MissingSystemProgram,
    /// The `NotAvailable` program error.
    NotAvailable,
    /// The `InvalidAsset` program error.
    InvalidAsset,
    /// The `MissingCollection` program error.
    MissingCollection,
    /// The `NoApprovals` program error.
    NoApprovals,
    /// The `CannotRedelegate` program error.
    CannotRedelegate,
    /// The `InvalidPluginSetting` program error.
    InvalidPluginSetting,
    /// The `ConflictingAuthority` program error.
    ConflictingAuthority,
    /// The `InvalidLogWrapperProgram` program error.
    InvalidLogWrapperProgram,
    /// The `ExternalPluginAdapterNotFound` program error.
    ExternalPluginAdapterNotFound,
    /// The `ExternalPluginAdapterAlreadyExists` program error.
    ExternalPluginAdapterAlreadyExists,
    /// The `MissingAsset` program error.
    MissingAsset,
    /// The `MissingExternalPluginAdapterAccount` program error.
    MissingExternalPluginAdapterAccount,
    /// The `OracleCanRejectOnly` program error.
    OracleCanRejectOnly,
    /// The `RequiresLifecycleCheck` program error.
    RequiresLifecycleCheck,
    /// The `DuplicateLifecycleChecks` program error.
    DuplicateLifecycleChecks,
    /// The `InvalidOracleAccountData` program error.
    InvalidOracleAccountData,
    /// The `UninitializedOracleAccount` program error.
    UninitializedOracleAccount,
    /// The `MissingSigner` program error.
    MissingSigner,
    /// The `InvalidPluginOperation` program error.
    InvalidPluginOperation,
    /// The `CollectionMustBeEmpty` program error.
    CollectionMustBeEmpty,
    /// The `TwoDataSources` program error.
    TwoDataSources,
    /// The `UnsupportedOperation` program error.
    UnsupportedOperation,
    /// The `NoDataSources` program error.
    NoDataSources,
    /// The `InvalidPluginAdapterTarget` program error.
    InvalidPluginAdapterTarget,
    /// The `CannotAddDataSection` program error.
    CannotAddDataSection,
    /// The `PermanentDelegatesPreventMove` program error.
    PermanentDelegatesPreventMove,
    /// The `InvalidExecutePda` program error.
    InvalidExecutePda,
    /// The `BlockedByBubblegumV2` program error.
    BlockedByBubblegumV2,
    /// The `AgentIdentityMustSign` program error.
    AgentIdentityMustSign,
    /// The `GroupMustBeEmpty` program error.
    GroupMustBeEmpty,
    /// The `DuplicateEntry` program error.
    DuplicateEntry,
    /// The `GroupVectorFull` program error.
    GroupVectorFull,
    /// The `GroupNestingDepthExceeded` program error.
    GroupNestingDepthExceeded,
    /// The `InconsistentGroupRelationship` program error.
    InconsistentGroupRelationship,
    /// An unrecognized program error code.
    Unknown(u32),
}
impl MplCoreError {
    /// Decodes known or unknown numeric program error codes.
    pub const fn from_code(code: u32) -> Self {
        match code {
            PROGRAM_ERROR_0 => Self::InvalidSystemProgram,
            PROGRAM_ERROR_1 => Self::DeserializationError,
            PROGRAM_ERROR_2 => Self::SerializationError,
            PROGRAM_ERROR_3 => Self::PluginsNotInitialized,
            PROGRAM_ERROR_4 => Self::PluginNotFound,
            PROGRAM_ERROR_5 => Self::NumericalOverflow,
            PROGRAM_ERROR_6 => Self::IncorrectAccount,
            PROGRAM_ERROR_7 => Self::IncorrectAssetHash,
            PROGRAM_ERROR_8 => Self::InvalidPlugin,
            PROGRAM_ERROR_9 => Self::InvalidAuthority,
            PROGRAM_ERROR_10 => Self::AssetIsFrozen,
            PROGRAM_ERROR_11 => Self::MissingCompressionProof,
            PROGRAM_ERROR_12 => Self::CannotMigrateMasterWithSupply,
            PROGRAM_ERROR_13 => Self::CannotMigratePrints,
            PROGRAM_ERROR_14 => Self::CannotBurnCollection,
            PROGRAM_ERROR_15 => Self::PluginAlreadyExists,
            PROGRAM_ERROR_16 => Self::NumericalOverflowError,
            PROGRAM_ERROR_17 => Self::AlreadyCompressed,
            PROGRAM_ERROR_18 => Self::AlreadyDecompressed,
            PROGRAM_ERROR_19 => Self::InvalidCollection,
            PROGRAM_ERROR_20 => Self::MissingUpdateAuthority,
            PROGRAM_ERROR_21 => Self::MissingNewOwner,
            PROGRAM_ERROR_22 => Self::MissingSystemProgram,
            PROGRAM_ERROR_23 => Self::NotAvailable,
            PROGRAM_ERROR_24 => Self::InvalidAsset,
            PROGRAM_ERROR_25 => Self::MissingCollection,
            PROGRAM_ERROR_26 => Self::NoApprovals,
            PROGRAM_ERROR_27 => Self::CannotRedelegate,
            PROGRAM_ERROR_28 => Self::InvalidPluginSetting,
            PROGRAM_ERROR_29 => Self::ConflictingAuthority,
            PROGRAM_ERROR_30 => Self::InvalidLogWrapperProgram,
            PROGRAM_ERROR_31 => Self::ExternalPluginAdapterNotFound,
            PROGRAM_ERROR_32 => Self::ExternalPluginAdapterAlreadyExists,
            PROGRAM_ERROR_33 => Self::MissingAsset,
            PROGRAM_ERROR_34 => Self::MissingExternalPluginAdapterAccount,
            PROGRAM_ERROR_35 => Self::OracleCanRejectOnly,
            PROGRAM_ERROR_36 => Self::RequiresLifecycleCheck,
            PROGRAM_ERROR_37 => Self::DuplicateLifecycleChecks,
            PROGRAM_ERROR_38 => Self::InvalidOracleAccountData,
            PROGRAM_ERROR_39 => Self::UninitializedOracleAccount,
            PROGRAM_ERROR_40 => Self::MissingSigner,
            PROGRAM_ERROR_41 => Self::InvalidPluginOperation,
            PROGRAM_ERROR_42 => Self::CollectionMustBeEmpty,
            PROGRAM_ERROR_43 => Self::TwoDataSources,
            PROGRAM_ERROR_44 => Self::UnsupportedOperation,
            PROGRAM_ERROR_45 => Self::NoDataSources,
            PROGRAM_ERROR_46 => Self::InvalidPluginAdapterTarget,
            PROGRAM_ERROR_47 => Self::CannotAddDataSection,
            PROGRAM_ERROR_48 => Self::PermanentDelegatesPreventMove,
            PROGRAM_ERROR_49 => Self::InvalidExecutePda,
            PROGRAM_ERROR_50 => Self::BlockedByBubblegumV2,
            PROGRAM_ERROR_51 => Self::AgentIdentityMustSign,
            PROGRAM_ERROR_52 => Self::GroupMustBeEmpty,
            PROGRAM_ERROR_53 => Self::DuplicateEntry,
            PROGRAM_ERROR_54 => Self::GroupVectorFull,
            PROGRAM_ERROR_55 => Self::GroupNestingDepthExceeded,
            PROGRAM_ERROR_56 => Self::InconsistentGroupRelationship,
            code => Self::Unknown(code),
        }
    }
    /// Returns the exact numeric code.
    pub const fn code(self) -> u32 {
        match self {
            Self::InvalidSystemProgram => PROGRAM_ERROR_0,
            Self::DeserializationError => PROGRAM_ERROR_1,
            Self::SerializationError => PROGRAM_ERROR_2,
            Self::PluginsNotInitialized => PROGRAM_ERROR_3,
            Self::PluginNotFound => PROGRAM_ERROR_4,
            Self::NumericalOverflow => PROGRAM_ERROR_5,
            Self::IncorrectAccount => PROGRAM_ERROR_6,
            Self::IncorrectAssetHash => PROGRAM_ERROR_7,
            Self::InvalidPlugin => PROGRAM_ERROR_8,
            Self::InvalidAuthority => PROGRAM_ERROR_9,
            Self::AssetIsFrozen => PROGRAM_ERROR_10,
            Self::MissingCompressionProof => PROGRAM_ERROR_11,
            Self::CannotMigrateMasterWithSupply => PROGRAM_ERROR_12,
            Self::CannotMigratePrints => PROGRAM_ERROR_13,
            Self::CannotBurnCollection => PROGRAM_ERROR_14,
            Self::PluginAlreadyExists => PROGRAM_ERROR_15,
            Self::NumericalOverflowError => PROGRAM_ERROR_16,
            Self::AlreadyCompressed => PROGRAM_ERROR_17,
            Self::AlreadyDecompressed => PROGRAM_ERROR_18,
            Self::InvalidCollection => PROGRAM_ERROR_19,
            Self::MissingUpdateAuthority => PROGRAM_ERROR_20,
            Self::MissingNewOwner => PROGRAM_ERROR_21,
            Self::MissingSystemProgram => PROGRAM_ERROR_22,
            Self::NotAvailable => PROGRAM_ERROR_23,
            Self::InvalidAsset => PROGRAM_ERROR_24,
            Self::MissingCollection => PROGRAM_ERROR_25,
            Self::NoApprovals => PROGRAM_ERROR_26,
            Self::CannotRedelegate => PROGRAM_ERROR_27,
            Self::InvalidPluginSetting => PROGRAM_ERROR_28,
            Self::ConflictingAuthority => PROGRAM_ERROR_29,
            Self::InvalidLogWrapperProgram => PROGRAM_ERROR_30,
            Self::ExternalPluginAdapterNotFound => PROGRAM_ERROR_31,
            Self::ExternalPluginAdapterAlreadyExists => PROGRAM_ERROR_32,
            Self::MissingAsset => PROGRAM_ERROR_33,
            Self::MissingExternalPluginAdapterAccount => PROGRAM_ERROR_34,
            Self::OracleCanRejectOnly => PROGRAM_ERROR_35,
            Self::RequiresLifecycleCheck => PROGRAM_ERROR_36,
            Self::DuplicateLifecycleChecks => PROGRAM_ERROR_37,
            Self::InvalidOracleAccountData => PROGRAM_ERROR_38,
            Self::UninitializedOracleAccount => PROGRAM_ERROR_39,
            Self::MissingSigner => PROGRAM_ERROR_40,
            Self::InvalidPluginOperation => PROGRAM_ERROR_41,
            Self::CollectionMustBeEmpty => PROGRAM_ERROR_42,
            Self::TwoDataSources => PROGRAM_ERROR_43,
            Self::UnsupportedOperation => PROGRAM_ERROR_44,
            Self::NoDataSources => PROGRAM_ERROR_45,
            Self::InvalidPluginAdapterTarget => PROGRAM_ERROR_46,
            Self::CannotAddDataSection => PROGRAM_ERROR_47,
            Self::PermanentDelegatesPreventMove => PROGRAM_ERROR_48,
            Self::InvalidExecutePda => PROGRAM_ERROR_49,
            Self::BlockedByBubblegumV2 => PROGRAM_ERROR_50,
            Self::AgentIdentityMustSign => PROGRAM_ERROR_51,
            Self::GroupMustBeEmpty => PROGRAM_ERROR_52,
            Self::DuplicateEntry => PROGRAM_ERROR_53,
            Self::GroupVectorFull => PROGRAM_ERROR_54,
            Self::GroupNestingDepthExceeded => PROGRAM_ERROR_55,
            Self::InconsistentGroupRelationship => PROGRAM_ERROR_56,
            Self::Unknown(code) => code,
        }
    }
    /// Returns the source message for known errors.
    pub const fn message(self) -> Option<&'static str> {
        match self {
            Self::InvalidSystemProgram => Some("Invalid System Program"),
            Self::DeserializationError => Some("Error deserializing account"),
            Self::SerializationError => Some("Error serializing account"),
            Self::PluginsNotInitialized => Some("Plugins not initialized"),
            Self::PluginNotFound => Some("Plugin not found"),
            Self::NumericalOverflow => Some("Numerical Overflow"),
            Self::IncorrectAccount => Some("Incorrect account"),
            Self::IncorrectAssetHash => Some("Incorrect asset hash"),
            Self::InvalidPlugin => Some("Invalid Plugin"),
            Self::InvalidAuthority => Some("Invalid Authority"),
            Self::AssetIsFrozen => Some("Cannot transfer a frozen asset"),
            Self::MissingCompressionProof => Some("Missing compression proof"),
            Self::CannotMigrateMasterWithSupply => {
                Some("Cannot migrate a master edition used for prints")
            }
            Self::CannotMigratePrints => Some("Cannot migrate a print edition"),
            Self::CannotBurnCollection => Some("Cannot burn a collection NFT"),
            Self::PluginAlreadyExists => Some("Plugin already exists"),
            Self::NumericalOverflowError => Some("Numerical overflow"),
            Self::AlreadyCompressed => Some("Already compressed account"),
            Self::AlreadyDecompressed => Some("Already decompressed account"),
            Self::InvalidCollection => Some("Invalid Collection passed in"),
            Self::MissingUpdateAuthority => Some("Missing update authority"),
            Self::MissingNewOwner => Some("Missing new owner"),
            Self::MissingSystemProgram => Some("Missing system program"),
            Self::NotAvailable => Some("Feature not available"),
            Self::InvalidAsset => Some("Invalid Asset passed in"),
            Self::MissingCollection => Some("Missing collection"),
            Self::NoApprovals => {
                Some("Neither the asset or any plugins have approved this operation")
            }
            Self::CannotRedelegate => {
                Some("Plugin Manager cannot redelegate a delegated plugin without revoking first")
            }
            Self::InvalidPluginSetting => Some("Invalid setting for plugin"),
            Self::ConflictingAuthority => {
                Some("Cannot specify both an update authority and collection on an asset")
            }
            Self::InvalidLogWrapperProgram => Some("Invalid Log Wrapper Program"),
            Self::ExternalPluginAdapterNotFound => Some("External Plugin Adapter not found"),
            Self::ExternalPluginAdapterAlreadyExists => {
                Some("External Plugin Adapter already exists")
            }
            Self::MissingAsset => Some("Missing asset needed for extra account PDA derivation"),
            Self::MissingExternalPluginAdapterAccount => {
                Some("Missing account needed for external plugin adapter")
            }
            Self::OracleCanRejectOnly => {
                Some("Oracle external plugin adapter can only be configured to reject")
            }
            Self::RequiresLifecycleCheck => {
                Some("External plugin adapter must have at least one lifecycle check")
            }
            Self::DuplicateLifecycleChecks => {
                Some("Duplicate lifecycle checks were provided for external plugin adapter ")
            }
            Self::InvalidOracleAccountData => Some("Could not read from oracle account"),
            Self::UninitializedOracleAccount => Some("Oracle account is uninitialized"),
            Self::MissingSigner => Some("Missing required signer for operation"),
            Self::InvalidPluginOperation => Some("Invalid plugin operation"),
            Self::CollectionMustBeEmpty => Some("Collection must be empty to be burned"),
            Self::TwoDataSources => Some("Two data sources provided, only one is allowed"),
            Self::UnsupportedOperation => Some("External Plugin does not support this operation"),
            Self::NoDataSources => Some("No data sources provided, one is required"),
            Self::InvalidPluginAdapterTarget => {
                Some("This plugin adapter cannot be added to an Asset")
            }
            Self::CannotAddDataSection => {
                Some("Cannot add a Data Section without a linked external plugin")
            }
            Self::PermanentDelegatesPreventMove => {
                Some("Cannot move asset to collection with permanent delegates")
            }
            Self::InvalidExecutePda => Some("Invalid Signing PDA for Asset or Collection Execute"),
            Self::BlockedByBubblegumV2 => Some("Bubblegum V2 Plugin limits other plugins"),
            Self::AgentIdentityMustSign => Some("Agent Identity Program must sign"),
            Self::GroupMustBeEmpty => Some("Group must be empty to be closed"),
            Self::DuplicateEntry => {
                Some("Duplicate entry provided when adding relationships to a group")
            }
            Self::GroupVectorFull => Some("Group vector is at maximum capacity"),
            Self::GroupNestingDepthExceeded => Some("Group nesting depth exceeded"),
            Self::InconsistentGroupRelationship => {
                Some("Bidirectional group relationship is inconsistent")
            }
            Self::Unknown(_) => None,
        }
    }
}
/// All selected instruction data variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MplCoreInstructionData {
    /// Data for tag 0.
    CreateV1(CreateV1),
    /// Data for tag 1.
    CreateCollectionV1(CreateCollectionV1),
    /// Data for tag 2.
    AddPluginV1(AddPluginV1),
    /// Data for tag 3.
    AddCollectionPluginV1(AddCollectionPluginV1),
    /// Data for tag 4.
    RemovePluginV1(RemovePluginV1),
    /// Data for tag 5.
    RemoveCollectionPluginV1(RemoveCollectionPluginV1),
    /// Data for tag 6.
    UpdatePluginV1(UpdatePluginV1),
    /// Data for tag 7.
    UpdateCollectionPluginV1(UpdateCollectionPluginV1),
    /// Data for tag 8.
    ApprovePluginAuthorityV1(ApprovePluginAuthorityV1),
    /// Data for tag 9.
    ApproveCollectionPluginAuthorityV1(ApproveCollectionPluginAuthorityV1),
    /// Data for tag 10.
    RevokePluginAuthorityV1(RevokePluginAuthorityV1),
    /// Data for tag 11.
    RevokeCollectionPluginAuthorityV1(RevokeCollectionPluginAuthorityV1),
    /// Data for tag 12.
    BurnV1(BurnV1),
    /// Data for tag 13.
    BurnCollectionV1(BurnCollectionV1),
    /// Data for tag 14.
    TransferV1(TransferV1),
    /// Data for tag 15.
    UpdateV1(UpdateV1),
    /// Data for tag 16.
    UpdateCollectionV1(UpdateCollectionV1),
    /// Data for tag 17.
    CompressV1(CompressV1),
    /// Data for tag 18.
    DecompressV1(DecompressV1),
    /// Data for tag 19.
    Collect(Collect),
    /// Data for tag 20.
    CreateV2(CreateV2),
    /// Data for tag 21.
    CreateCollectionV2(CreateCollectionV2),
    /// Data for tag 22.
    AddExternalPluginAdapterV1(AddExternalPluginAdapterV1),
    /// Data for tag 23.
    AddCollectionExternalPluginAdapterV1(AddCollectionExternalPluginAdapterV1),
    /// Data for tag 24.
    RemoveExternalPluginAdapterV1(RemoveExternalPluginAdapterV1),
    /// Data for tag 25.
    RemoveCollectionExternalPluginAdapterV1(RemoveCollectionExternalPluginAdapterV1),
    /// Data for tag 26.
    UpdateExternalPluginAdapterV1(UpdateExternalPluginAdapterV1),
    /// Data for tag 27.
    UpdateCollectionExternalPluginAdapterV1(UpdateCollectionExternalPluginAdapterV1),
    /// Data for tag 28.
    WriteExternalPluginAdapterDataV1(WriteExternalPluginAdapterDataV1),
    /// Data for tag 29.
    WriteCollectionExternalPluginAdapterDataV1(WriteCollectionExternalPluginAdapterDataV1),
    /// Data for tag 30.
    UpdateV2(UpdateV2),
    /// Data for tag 31.
    ExecuteV1(ExecuteV1),
    /// Data for tag 32.
    UpdateCollectionInfoV1(UpdateCollectionInfoV1),
    /// Data for tag 33.
    AddCollectionsToGroupV1(AddCollectionsToGroupV1),
    /// Data for tag 34.
    RemoveCollectionsFromGroupV1(RemoveCollectionsFromGroupV1),
    /// Data for tag 35.
    AddAssetsToGroupV1(AddAssetsToGroupV1),
    /// Data for tag 36.
    RemoveAssetsFromGroupV1(RemoveAssetsFromGroupV1),
    /// Data for tag 37.
    AddGroupsToGroupV1(AddGroupsToGroupV1),
    /// Data for tag 38.
    RemoveGroupsFromGroupV1(RemoveGroupsFromGroupV1),
    /// Data for tag 39.
    CreateGroupV1(CreateGroupV1),
    /// Data for tag 40.
    CloseGroupV1(CloseGroupV1),
    /// Data for tag 41.
    UpdateGroupV1(UpdateGroupV1),
}
/// Strict bounded instruction dispatch codec.
#[derive(Clone, Copy, Debug)]
pub struct MplCoreInstructionDataCodec;
impl Encoder<MplCoreInstructionData> for MplCoreInstructionDataCodec {
    fn encoded_len(&self, value: &MplCoreInstructionData) -> Result<usize, Error> {
        Self::encoded_len_0(value)
    }
    fn write(&self, value: &MplCoreInstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        Self::write_0(value, writer)
    }
}
impl Decoder<MplCoreInstructionData> for MplCoreInstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MplCoreInstructionData, Error> {
        let tag = *reader.remaining().first().ok_or(Error::Truncated)?;
        Self::read_0(tag, reader)
    }
}
impl MplCoreInstructionDataCodec {
    fn encoded_len_0(value: &MplCoreInstructionData) -> Result<usize, Error> {
        match value {
            MplCoreInstructionData::CreateV1(value) => CreateV1Codec.encoded_len(value),
            MplCoreInstructionData::CreateCollectionV1(value) => {
                CreateCollectionV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::AddPluginV1(value) => AddPluginV1Codec.encoded_len(value),
            MplCoreInstructionData::AddCollectionPluginV1(value) => {
                AddCollectionPluginV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::RemovePluginV1(value) => RemovePluginV1Codec.encoded_len(value),
            MplCoreInstructionData::RemoveCollectionPluginV1(value) => {
                RemoveCollectionPluginV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::UpdatePluginV1(value) => UpdatePluginV1Codec.encoded_len(value),
            MplCoreInstructionData::UpdateCollectionPluginV1(value) => {
                UpdateCollectionPluginV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::ApprovePluginAuthorityV1(value) => {
                ApprovePluginAuthorityV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::ApproveCollectionPluginAuthorityV1(value) => {
                ApproveCollectionPluginAuthorityV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::RevokePluginAuthorityV1(value) => {
                RevokePluginAuthorityV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::RevokeCollectionPluginAuthorityV1(value) => {
                RevokeCollectionPluginAuthorityV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::BurnV1(value) => BurnV1Codec.encoded_len(value),
            MplCoreInstructionData::BurnCollectionV1(value) => {
                BurnCollectionV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::TransferV1(value) => TransferV1Codec.encoded_len(value),
            MplCoreInstructionData::UpdateV1(value) => UpdateV1Codec.encoded_len(value),
            _ => Self::encoded_len_1(value),
        }
    }
    fn encoded_len_1(value: &MplCoreInstructionData) -> Result<usize, Error> {
        match value {
            MplCoreInstructionData::UpdateCollectionV1(value) => {
                UpdateCollectionV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::CompressV1(value) => CompressV1Codec.encoded_len(value),
            MplCoreInstructionData::DecompressV1(value) => DecompressV1Codec.encoded_len(value),
            MplCoreInstructionData::Collect(value) => CollectCodec.encoded_len(value),
            MplCoreInstructionData::CreateV2(value) => CreateV2Codec.encoded_len(value),
            MplCoreInstructionData::CreateCollectionV2(value) => {
                CreateCollectionV2Codec.encoded_len(value)
            }
            MplCoreInstructionData::AddExternalPluginAdapterV1(value) => {
                AddExternalPluginAdapterV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::AddCollectionExternalPluginAdapterV1(value) => {
                AddCollectionExternalPluginAdapterV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::RemoveExternalPluginAdapterV1(value) => {
                RemoveExternalPluginAdapterV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::RemoveCollectionExternalPluginAdapterV1(value) => {
                RemoveCollectionExternalPluginAdapterV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::UpdateExternalPluginAdapterV1(value) => {
                UpdateExternalPluginAdapterV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::UpdateCollectionExternalPluginAdapterV1(value) => {
                UpdateCollectionExternalPluginAdapterV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::WriteExternalPluginAdapterDataV1(value) => {
                WriteExternalPluginAdapterDataV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::WriteCollectionExternalPluginAdapterDataV1(value) => {
                WriteCollectionExternalPluginAdapterDataV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::UpdateV2(value) => UpdateV2Codec.encoded_len(value),
            MplCoreInstructionData::ExecuteV1(value) => ExecuteV1Codec.encoded_len(value),
            _ => Self::encoded_len_2(value),
        }
    }
    fn encoded_len_2(value: &MplCoreInstructionData) -> Result<usize, Error> {
        match value {
            MplCoreInstructionData::UpdateCollectionInfoV1(value) => {
                UpdateCollectionInfoV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::AddCollectionsToGroupV1(value) => {
                AddCollectionsToGroupV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::RemoveCollectionsFromGroupV1(value) => {
                RemoveCollectionsFromGroupV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::AddAssetsToGroupV1(value) => {
                AddAssetsToGroupV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::RemoveAssetsFromGroupV1(value) => {
                RemoveAssetsFromGroupV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::AddGroupsToGroupV1(value) => {
                AddGroupsToGroupV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::RemoveGroupsFromGroupV1(value) => {
                RemoveGroupsFromGroupV1Codec.encoded_len(value)
            }
            MplCoreInstructionData::CreateGroupV1(value) => CreateGroupV1Codec.encoded_len(value),
            MplCoreInstructionData::CloseGroupV1(value) => CloseGroupV1Codec.encoded_len(value),
            MplCoreInstructionData::UpdateGroupV1(value) => UpdateGroupV1Codec.encoded_len(value),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
    fn write_0(value: &MplCoreInstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            MplCoreInstructionData::CreateV1(value) => CreateV1Codec.write(value, writer),
            MplCoreInstructionData::CreateCollectionV1(value) => {
                CreateCollectionV1Codec.write(value, writer)
            }
            MplCoreInstructionData::AddPluginV1(value) => AddPluginV1Codec.write(value, writer),
            MplCoreInstructionData::AddCollectionPluginV1(value) => {
                AddCollectionPluginV1Codec.write(value, writer)
            }
            MplCoreInstructionData::RemovePluginV1(value) => {
                RemovePluginV1Codec.write(value, writer)
            }
            MplCoreInstructionData::RemoveCollectionPluginV1(value) => {
                RemoveCollectionPluginV1Codec.write(value, writer)
            }
            MplCoreInstructionData::UpdatePluginV1(value) => {
                UpdatePluginV1Codec.write(value, writer)
            }
            MplCoreInstructionData::UpdateCollectionPluginV1(value) => {
                UpdateCollectionPluginV1Codec.write(value, writer)
            }
            MplCoreInstructionData::ApprovePluginAuthorityV1(value) => {
                ApprovePluginAuthorityV1Codec.write(value, writer)
            }
            MplCoreInstructionData::ApproveCollectionPluginAuthorityV1(value) => {
                ApproveCollectionPluginAuthorityV1Codec.write(value, writer)
            }
            MplCoreInstructionData::RevokePluginAuthorityV1(value) => {
                RevokePluginAuthorityV1Codec.write(value, writer)
            }
            MplCoreInstructionData::RevokeCollectionPluginAuthorityV1(value) => {
                RevokeCollectionPluginAuthorityV1Codec.write(value, writer)
            }
            MplCoreInstructionData::BurnV1(value) => BurnV1Codec.write(value, writer),
            MplCoreInstructionData::BurnCollectionV1(value) => {
                BurnCollectionV1Codec.write(value, writer)
            }
            MplCoreInstructionData::TransferV1(value) => TransferV1Codec.write(value, writer),
            MplCoreInstructionData::UpdateV1(value) => UpdateV1Codec.write(value, writer),
            _ => Self::write_1(value, writer),
        }
    }
    fn write_1(value: &MplCoreInstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            MplCoreInstructionData::UpdateCollectionV1(value) => {
                UpdateCollectionV1Codec.write(value, writer)
            }
            MplCoreInstructionData::CompressV1(value) => CompressV1Codec.write(value, writer),
            MplCoreInstructionData::DecompressV1(value) => DecompressV1Codec.write(value, writer),
            MplCoreInstructionData::Collect(value) => CollectCodec.write(value, writer),
            MplCoreInstructionData::CreateV2(value) => CreateV2Codec.write(value, writer),
            MplCoreInstructionData::CreateCollectionV2(value) => {
                CreateCollectionV2Codec.write(value, writer)
            }
            MplCoreInstructionData::AddExternalPluginAdapterV1(value) => {
                AddExternalPluginAdapterV1Codec.write(value, writer)
            }
            MplCoreInstructionData::AddCollectionExternalPluginAdapterV1(value) => {
                AddCollectionExternalPluginAdapterV1Codec.write(value, writer)
            }
            MplCoreInstructionData::RemoveExternalPluginAdapterV1(value) => {
                RemoveExternalPluginAdapterV1Codec.write(value, writer)
            }
            MplCoreInstructionData::RemoveCollectionExternalPluginAdapterV1(value) => {
                RemoveCollectionExternalPluginAdapterV1Codec.write(value, writer)
            }
            MplCoreInstructionData::UpdateExternalPluginAdapterV1(value) => {
                UpdateExternalPluginAdapterV1Codec.write(value, writer)
            }
            MplCoreInstructionData::UpdateCollectionExternalPluginAdapterV1(value) => {
                UpdateCollectionExternalPluginAdapterV1Codec.write(value, writer)
            }
            MplCoreInstructionData::WriteExternalPluginAdapterDataV1(value) => {
                WriteExternalPluginAdapterDataV1Codec.write(value, writer)
            }
            MplCoreInstructionData::WriteCollectionExternalPluginAdapterDataV1(value) => {
                WriteCollectionExternalPluginAdapterDataV1Codec.write(value, writer)
            }
            MplCoreInstructionData::UpdateV2(value) => UpdateV2Codec.write(value, writer),
            MplCoreInstructionData::ExecuteV1(value) => ExecuteV1Codec.write(value, writer),
            _ => Self::write_2(value, writer),
        }
    }
    fn write_2(value: &MplCoreInstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            MplCoreInstructionData::UpdateCollectionInfoV1(value) => {
                UpdateCollectionInfoV1Codec.write(value, writer)
            }
            MplCoreInstructionData::AddCollectionsToGroupV1(value) => {
                AddCollectionsToGroupV1Codec.write(value, writer)
            }
            MplCoreInstructionData::RemoveCollectionsFromGroupV1(value) => {
                RemoveCollectionsFromGroupV1Codec.write(value, writer)
            }
            MplCoreInstructionData::AddAssetsToGroupV1(value) => {
                AddAssetsToGroupV1Codec.write(value, writer)
            }
            MplCoreInstructionData::RemoveAssetsFromGroupV1(value) => {
                RemoveAssetsFromGroupV1Codec.write(value, writer)
            }
            MplCoreInstructionData::AddGroupsToGroupV1(value) => {
                AddGroupsToGroupV1Codec.write(value, writer)
            }
            MplCoreInstructionData::RemoveGroupsFromGroupV1(value) => {
                RemoveGroupsFromGroupV1Codec.write(value, writer)
            }
            MplCoreInstructionData::CreateGroupV1(value) => CreateGroupV1Codec.write(value, writer),
            MplCoreInstructionData::CloseGroupV1(value) => CloseGroupV1Codec.write(value, writer),
            MplCoreInstructionData::UpdateGroupV1(value) => UpdateGroupV1Codec.write(value, writer),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
    fn read_0(tag: u8, reader: &mut Reader<'_>) -> Result<MplCoreInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_0 => Ok(MplCoreInstructionData::CreateV1(
                CreateV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_1 => Ok(MplCoreInstructionData::CreateCollectionV1(
                CreateCollectionV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_2 => Ok(MplCoreInstructionData::AddPluginV1(
                AddPluginV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_3 => Ok(MplCoreInstructionData::AddCollectionPluginV1(
                AddCollectionPluginV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_4 => Ok(MplCoreInstructionData::RemovePluginV1(
                RemovePluginV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_5 => Ok(MplCoreInstructionData::RemoveCollectionPluginV1(
                RemoveCollectionPluginV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_6 => Ok(MplCoreInstructionData::UpdatePluginV1(
                UpdatePluginV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_7 => Ok(MplCoreInstructionData::UpdateCollectionPluginV1(
                UpdateCollectionPluginV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_8 => Ok(MplCoreInstructionData::ApprovePluginAuthorityV1(
                ApprovePluginAuthorityV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_9 => Ok(MplCoreInstructionData::ApproveCollectionPluginAuthorityV1(
                ApproveCollectionPluginAuthorityV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_10 => Ok(MplCoreInstructionData::RevokePluginAuthorityV1(
                RevokePluginAuthorityV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_11 => Ok(MplCoreInstructionData::RevokeCollectionPluginAuthorityV1(
                RevokeCollectionPluginAuthorityV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_12 => Ok(MplCoreInstructionData::BurnV1(BurnV1Codec.read(reader)?)),
            INSTRUCTION_TAG_13 => Ok(MplCoreInstructionData::BurnCollectionV1(
                BurnCollectionV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_14 => Ok(MplCoreInstructionData::TransferV1(
                TransferV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_15 => Ok(MplCoreInstructionData::UpdateV1(
                UpdateV1Codec.read(reader)?,
            )),
            _ => Self::read_1(tag, reader),
        }
    }
    fn read_1(tag: u8, reader: &mut Reader<'_>) -> Result<MplCoreInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_16 => Ok(MplCoreInstructionData::UpdateCollectionV1(
                UpdateCollectionV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_17 => Ok(MplCoreInstructionData::CompressV1(
                CompressV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_18 => Ok(MplCoreInstructionData::DecompressV1(
                DecompressV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_19 => Ok(MplCoreInstructionData::Collect(CollectCodec.read(reader)?)),
            INSTRUCTION_TAG_20 => Ok(MplCoreInstructionData::CreateV2(
                CreateV2Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_21 => Ok(MplCoreInstructionData::CreateCollectionV2(
                CreateCollectionV2Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_22 => Ok(MplCoreInstructionData::AddExternalPluginAdapterV1(
                AddExternalPluginAdapterV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_23 => Ok(
                MplCoreInstructionData::AddCollectionExternalPluginAdapterV1(
                    AddCollectionExternalPluginAdapterV1Codec.read(reader)?,
                ),
            ),
            INSTRUCTION_TAG_24 => Ok(MplCoreInstructionData::RemoveExternalPluginAdapterV1(
                RemoveExternalPluginAdapterV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_25 => Ok(
                MplCoreInstructionData::RemoveCollectionExternalPluginAdapterV1(
                    RemoveCollectionExternalPluginAdapterV1Codec.read(reader)?,
                ),
            ),
            INSTRUCTION_TAG_26 => Ok(MplCoreInstructionData::UpdateExternalPluginAdapterV1(
                UpdateExternalPluginAdapterV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_27 => Ok(
                MplCoreInstructionData::UpdateCollectionExternalPluginAdapterV1(
                    UpdateCollectionExternalPluginAdapterV1Codec.read(reader)?,
                ),
            ),
            INSTRUCTION_TAG_28 => Ok(MplCoreInstructionData::WriteExternalPluginAdapterDataV1(
                WriteExternalPluginAdapterDataV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_29 => Ok(
                MplCoreInstructionData::WriteCollectionExternalPluginAdapterDataV1(
                    WriteCollectionExternalPluginAdapterDataV1Codec.read(reader)?,
                ),
            ),
            INSTRUCTION_TAG_30 => Ok(MplCoreInstructionData::UpdateV2(
                UpdateV2Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_31 => Ok(MplCoreInstructionData::ExecuteV1(
                ExecuteV1Codec.read(reader)?,
            )),
            _ => Self::read_2(tag, reader),
        }
    }
    fn read_2(tag: u8, reader: &mut Reader<'_>) -> Result<MplCoreInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_32 => Ok(MplCoreInstructionData::UpdateCollectionInfoV1(
                UpdateCollectionInfoV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_33 => Ok(MplCoreInstructionData::AddCollectionsToGroupV1(
                AddCollectionsToGroupV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_34 => Ok(MplCoreInstructionData::RemoveCollectionsFromGroupV1(
                RemoveCollectionsFromGroupV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_35 => Ok(MplCoreInstructionData::AddAssetsToGroupV1(
                AddAssetsToGroupV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_36 => Ok(MplCoreInstructionData::RemoveAssetsFromGroupV1(
                RemoveAssetsFromGroupV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_37 => Ok(MplCoreInstructionData::AddGroupsToGroupV1(
                AddGroupsToGroupV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_38 => Ok(MplCoreInstructionData::RemoveGroupsFromGroupV1(
                RemoveGroupsFromGroupV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_39 => Ok(MplCoreInstructionData::CreateGroupV1(
                CreateGroupV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_40 => Ok(MplCoreInstructionData::CloseGroupV1(
                CloseGroupV1Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_41 => Ok(MplCoreInstructionData::UpdateGroupV1(
                UpdateGroupV1Codec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
    fn parse_0(
        tag: u8,
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<MplCoreInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_0 => Ok(MplCoreInstructionData::CreateV1(
                CreateV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_1 => Ok(MplCoreInstructionData::CreateCollectionV1(
                CreateCollectionV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_2 => Ok(MplCoreInstructionData::AddPluginV1(
                AddPluginV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_3 => Ok(MplCoreInstructionData::AddCollectionPluginV1(
                AddCollectionPluginV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_4 => Ok(MplCoreInstructionData::RemovePluginV1(
                RemovePluginV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_5 => Ok(MplCoreInstructionData::RemoveCollectionPluginV1(
                RemoveCollectionPluginV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_6 => Ok(MplCoreInstructionData::UpdatePluginV1(
                UpdatePluginV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_7 => Ok(MplCoreInstructionData::UpdateCollectionPluginV1(
                UpdateCollectionPluginV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_8 => Ok(MplCoreInstructionData::ApprovePluginAuthorityV1(
                ApprovePluginAuthorityV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_9 => Ok(MplCoreInstructionData::ApproveCollectionPluginAuthorityV1(
                ApproveCollectionPluginAuthorityV1::parse_with_program(
                    program,
                    instruction,
                    limits,
                )?,
            )),
            INSTRUCTION_TAG_10 => Ok(MplCoreInstructionData::RevokePluginAuthorityV1(
                RevokePluginAuthorityV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_11 => Ok(MplCoreInstructionData::RevokeCollectionPluginAuthorityV1(
                RevokeCollectionPluginAuthorityV1::parse_with_program(
                    program,
                    instruction,
                    limits,
                )?,
            )),
            INSTRUCTION_TAG_12 => Ok(MplCoreInstructionData::BurnV1(BurnV1::parse_with_program(
                program,
                instruction,
                limits,
            )?)),
            INSTRUCTION_TAG_13 => Ok(MplCoreInstructionData::BurnCollectionV1(
                BurnCollectionV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_14 => Ok(MplCoreInstructionData::TransferV1(
                TransferV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_15 => Ok(MplCoreInstructionData::UpdateV1(
                UpdateV1::parse_with_program(program, instruction, limits)?,
            )),
            _ => Self::parse_1(tag, program, instruction, limits),
        }
    }
    fn parse_1(
        tag: u8,
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<MplCoreInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_16 => Ok(MplCoreInstructionData::UpdateCollectionV1(
                UpdateCollectionV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_17 => Ok(MplCoreInstructionData::CompressV1(
                CompressV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_18 => Ok(MplCoreInstructionData::DecompressV1(
                DecompressV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_19 => Ok(MplCoreInstructionData::Collect(
                Collect::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_20 => Ok(MplCoreInstructionData::CreateV2(
                CreateV2::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_21 => Ok(MplCoreInstructionData::CreateCollectionV2(
                CreateCollectionV2::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_22 => Ok(MplCoreInstructionData::AddExternalPluginAdapterV1(
                AddExternalPluginAdapterV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_23 => Ok(
                MplCoreInstructionData::AddCollectionExternalPluginAdapterV1(
                    AddCollectionExternalPluginAdapterV1::parse_with_program(
                        program,
                        instruction,
                        limits,
                    )?,
                ),
            ),
            INSTRUCTION_TAG_24 => Ok(MplCoreInstructionData::RemoveExternalPluginAdapterV1(
                RemoveExternalPluginAdapterV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_25 => Ok(
                MplCoreInstructionData::RemoveCollectionExternalPluginAdapterV1(
                    RemoveCollectionExternalPluginAdapterV1::parse_with_program(
                        program,
                        instruction,
                        limits,
                    )?,
                ),
            ),
            INSTRUCTION_TAG_26 => Ok(MplCoreInstructionData::UpdateExternalPluginAdapterV1(
                UpdateExternalPluginAdapterV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_27 => Ok(
                MplCoreInstructionData::UpdateCollectionExternalPluginAdapterV1(
                    UpdateCollectionExternalPluginAdapterV1::parse_with_program(
                        program,
                        instruction,
                        limits,
                    )?,
                ),
            ),
            INSTRUCTION_TAG_28 => Ok(MplCoreInstructionData::WriteExternalPluginAdapterDataV1(
                WriteExternalPluginAdapterDataV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_29 => Ok(
                MplCoreInstructionData::WriteCollectionExternalPluginAdapterDataV1(
                    WriteCollectionExternalPluginAdapterDataV1::parse_with_program(
                        program,
                        instruction,
                        limits,
                    )?,
                ),
            ),
            INSTRUCTION_TAG_30 => Ok(MplCoreInstructionData::UpdateV2(
                UpdateV2::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_31 => Ok(MplCoreInstructionData::ExecuteV1(
                ExecuteV1::parse_with_program(program, instruction, limits)?,
            )),
            _ => Self::parse_2(tag, program, instruction, limits),
        }
    }
    fn parse_2(
        tag: u8,
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<MplCoreInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_32 => Ok(MplCoreInstructionData::UpdateCollectionInfoV1(
                UpdateCollectionInfoV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_33 => Ok(MplCoreInstructionData::AddCollectionsToGroupV1(
                AddCollectionsToGroupV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_34 => Ok(MplCoreInstructionData::RemoveCollectionsFromGroupV1(
                RemoveCollectionsFromGroupV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_35 => Ok(MplCoreInstructionData::AddAssetsToGroupV1(
                AddAssetsToGroupV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_36 => Ok(MplCoreInstructionData::RemoveAssetsFromGroupV1(
                RemoveAssetsFromGroupV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_37 => Ok(MplCoreInstructionData::AddGroupsToGroupV1(
                AddGroupsToGroupV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_38 => Ok(MplCoreInstructionData::RemoveGroupsFromGroupV1(
                RemoveGroupsFromGroupV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_39 => Ok(MplCoreInstructionData::CreateGroupV1(
                CreateGroupV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_40 => Ok(MplCoreInstructionData::CloseGroupV1(
                CloseGroupV1::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_41 => Ok(MplCoreInstructionData::UpdateGroupV1(
                UpdateGroupV1::parse_with_program(program, instruction, limits)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
}
/// Identifies a tag without asserting the full payload is valid.
pub fn identify_instruction(data: &[u8]) -> Result<u8, Error> {
    let tag = *data.first().ok_or(Error::Truncated)?;
    const TAGS: &[u8] = &[
        INSTRUCTION_TAG_0,
        INSTRUCTION_TAG_1,
        INSTRUCTION_TAG_2,
        INSTRUCTION_TAG_3,
        INSTRUCTION_TAG_4,
        INSTRUCTION_TAG_5,
        INSTRUCTION_TAG_6,
        INSTRUCTION_TAG_7,
        INSTRUCTION_TAG_8,
        INSTRUCTION_TAG_9,
        INSTRUCTION_TAG_10,
        INSTRUCTION_TAG_11,
        INSTRUCTION_TAG_12,
        INSTRUCTION_TAG_13,
        INSTRUCTION_TAG_14,
        INSTRUCTION_TAG_15,
        INSTRUCTION_TAG_16,
        INSTRUCTION_TAG_17,
        INSTRUCTION_TAG_18,
        INSTRUCTION_TAG_19,
        INSTRUCTION_TAG_20,
        INSTRUCTION_TAG_21,
        INSTRUCTION_TAG_22,
        INSTRUCTION_TAG_23,
        INSTRUCTION_TAG_24,
        INSTRUCTION_TAG_25,
        INSTRUCTION_TAG_26,
        INSTRUCTION_TAG_27,
        INSTRUCTION_TAG_28,
        INSTRUCTION_TAG_29,
        INSTRUCTION_TAG_30,
        INSTRUCTION_TAG_31,
        INSTRUCTION_TAG_32,
        INSTRUCTION_TAG_33,
        INSTRUCTION_TAG_34,
        INSTRUCTION_TAG_35,
        INSTRUCTION_TAG_36,
        INSTRUCTION_TAG_37,
        INSTRUCTION_TAG_38,
        INSTRUCTION_TAG_39,
        INSTRUCTION_TAG_40,
        INSTRUCTION_TAG_41,
    ];

    if !TAGS.contains(&tag) {
        return Err(Error::InvalidDiscriminator);
    }
    Ok(tag)
}
/// Strictly parses a canonical program invocation.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<MplCoreInstructionData, Error> {
    parse_instruction_with_program(PROGRAM, instruction, limits)
}
/// Strictly parses an explicit expected program identity.
pub fn parse_instruction_with_program(
    program: Address,
    instruction: &Instruction,
    limits: Limits,
) -> Result<MplCoreInstructionData, Error> {
    MplCoreInstructionDataCodec::parse_0(
        identify_instruction(&instruction.data)?,
        program,
        instruction,
        limits,
    )
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

struct OwnedVector<C>(C);
impl<T, C: Encoder<[T]>> Encoder<Vec<T>> for OwnedVector<C> {
    fn encoded_len(&self, value: &Vec<T>) -> Result<usize, Error> {
        self.0.encoded_len(value.as_slice())
    }
    fn write(&self, value: &Vec<T>, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write(value.as_slice(), writer)
    }
}
impl<T, C: Decoder<Vec<T>>> Decoder<Vec<T>> for OwnedVector<C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
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
use solana_kit_core::codecs::OptionCodec;
use solana_kit_core::codecs::Utf8;
use solana_kit_core::codecs::{ByteSizePrefix, FrameConsumption};
impl core::hash::Hash for AddBlocker {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for AddBlocker {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BubblegumV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for BubblegumV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BurnDelegate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for BurnDelegate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransferDelegate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for TransferDelegate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PermanentTransferDelegate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for PermanentTransferDelegate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PermanentBurnDelegate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for PermanentBurnDelegate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ImmutableMetadata {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for ImmutableMetadata {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DataSectionUpdateInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for DataSectionUpdateInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AgentIdentity {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.uri, state);
    }
}
impl core::fmt::Display for AgentIdentity {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AgentIdentityInitInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.init_plugin_authority, state);
        core::hash::Hash::hash(&self.lifecycle_checks, state);
    }
}
impl core::fmt::Display for AgentIdentityInitInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AgentIdentityUpdateInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.lifecycle_checks, state);
    }
}
impl core::fmt::Display for AgentIdentityUpdateInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AppData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.data_authority, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for AppData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AppDataInitInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.data_authority, state);
        core::hash::Hash::hash(&self.init_plugin_authority, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for AppDataInitInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AppDataUpdateInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for AppDataUpdateInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Attribute {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.key, state);
        core::hash::Hash::hash(&self.value, state);
    }
}
impl core::fmt::Display for Attribute {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Attributes {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.attribute_list, state);
    }
}
impl core::fmt::Display for Attributes {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Autograph {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.signatures, state);
    }
}
impl core::fmt::Display for Autograph {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AutographSignature {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.address, state);
        core::hash::Hash::hash(&self.message, state);
    }
}
impl core::fmt::Display for AutographSignature {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CompressionProof {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.owner, state);
        core::hash::Hash::hash(&self.update_authority, state);
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.seq, state);
        core::hash::Hash::hash(&self.plugins, state);
    }
}
impl core::fmt::Display for CompressionProof {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Creator {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.address, state);
        core::hash::Hash::hash(&self.percentage, state);
    }
}
impl core::fmt::Display for Creator {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DataSection {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.parent_key, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for DataSection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DataSectionInitInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.parent_key, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for DataSectionInitInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Edition {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.number, state);
    }
}
impl core::fmt::Display for Edition {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExternalCheckResult {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.flags, state);
    }
}
impl core::fmt::Display for ExternalCheckResult {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExternalRegistryRecord {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin_type, state);
        core::hash::Hash::hash(&self.authority, state);
        core::hash::Hash::hash(&self.lifecycle_checks, state);
        core::hash::Hash::hash(&self.offset, state);
        core::hash::Hash::hash(&self.data_offset, state);
        core::hash::Hash::hash(&self.data_len, state);
    }
}
impl core::fmt::Display for ExternalRegistryRecord {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for FreezeDelegate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.frozen, state);
    }
}
impl core::fmt::Display for FreezeDelegate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for FreezeExecute {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.frozen, state);
    }
}
impl core::fmt::Display for FreezeExecute {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Groups {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.groups, state);
    }
}
impl core::fmt::Display for Groups {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for HashablePluginSchema {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.authority, state);
        core::hash::Hash::hash(&self.plugin, state);
    }
}
impl core::fmt::Display for HashablePluginSchema {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for HashedAssetSchema {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.asset_hash, state);
        core::hash::Hash::hash(&self.plugin_hashes, state);
    }
}
impl core::fmt::Display for HashedAssetSchema {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LifecycleCheck {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.event, state);
        core::hash::Hash::hash(&self.result, state);
    }
}
impl core::fmt::Display for LifecycleCheck {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LifecycleHook {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.hooked_program, state);
        core::hash::Hash::hash(&self.extra_accounts, state);
        core::hash::Hash::hash(&self.data_authority, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for LifecycleHook {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LifecycleHookInitInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.hooked_program, state);
        core::hash::Hash::hash(&self.init_plugin_authority, state);
        core::hash::Hash::hash(&self.lifecycle_checks, state);
        core::hash::Hash::hash(&self.extra_accounts, state);
        core::hash::Hash::hash(&self.data_authority, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for LifecycleHookInitInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LifecycleHookUpdateInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.lifecycle_checks, state);
        core::hash::Hash::hash(&self.extra_accounts, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for LifecycleHookUpdateInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LinkedAppData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.data_authority, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for LinkedAppData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LinkedAppDataInitInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.data_authority, state);
        core::hash::Hash::hash(&self.init_plugin_authority, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for LinkedAppDataInitInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LinkedAppDataUpdateInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for LinkedAppDataUpdateInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LinkedLifecycleHook {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.hooked_program, state);
        core::hash::Hash::hash(&self.extra_accounts, state);
        core::hash::Hash::hash(&self.data_authority, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for LinkedLifecycleHook {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LinkedLifecycleHookInitInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.hooked_program, state);
        core::hash::Hash::hash(&self.init_plugin_authority, state);
        core::hash::Hash::hash(&self.lifecycle_checks, state);
        core::hash::Hash::hash(&self.extra_accounts, state);
        core::hash::Hash::hash(&self.data_authority, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for LinkedLifecycleHookInitInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LinkedLifecycleHookUpdateInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.lifecycle_checks, state);
        core::hash::Hash::hash(&self.extra_accounts, state);
        core::hash::Hash::hash(&self.schema, state);
    }
}
impl core::fmt::Display for LinkedLifecycleHookUpdateInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MasterEdition {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.max_supply, state);
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.uri, state);
    }
}
impl core::fmt::Display for MasterEdition {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Oracle {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.base_address, state);
        core::hash::Hash::hash(&self.base_address_config, state);
        core::hash::Hash::hash(&self.results_offset, state);
    }
}
impl core::fmt::Display for Oracle {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for OracleInitInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.base_address, state);
        core::hash::Hash::hash(&self.init_plugin_authority, state);
        core::hash::Hash::hash(&self.lifecycle_checks, state);
        core::hash::Hash::hash(&self.base_address_config, state);
        core::hash::Hash::hash(&self.results_offset, state);
    }
}
impl core::fmt::Display for OracleInitInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for OracleUpdateInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.lifecycle_checks, state);
        core::hash::Hash::hash(&self.base_address_config, state);
        core::hash::Hash::hash(&self.results_offset, state);
    }
}
impl core::fmt::Display for OracleUpdateInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PermanentFreezeDelegate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.frozen, state);
    }
}
impl core::fmt::Display for PermanentFreezeDelegate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PermanentFreezeExecute {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.frozen, state);
    }
}
impl core::fmt::Display for PermanentFreezeExecute {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PluginAuthorityPair {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin, state);
        core::hash::Hash::hash(&self.authority, state);
    }
}
impl core::fmt::Display for PluginAuthorityPair {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RegistryRecord {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin_type, state);
        core::hash::Hash::hash(&self.authority, state);
        core::hash::Hash::hash(&self.offset, state);
    }
}
impl core::fmt::Display for RegistryRecord {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RelationshipEntry {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.kind, state);
        core::hash::Hash::hash(&self.key, state);
    }
}
impl core::fmt::Display for RelationshipEntry {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Royalties {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.basis_points, state);
        core::hash::Hash::hash(&self.creators, state);
        core::hash::Hash::hash(&self.rule_set, state);
    }
}
impl core::fmt::Display for Royalties {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateDelegate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.additional_delegates, state);
    }
}
impl core::fmt::Display for UpdateDelegate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VerifiedCreators {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.signatures, state);
    }
}
impl core::fmt::Display for VerifiedCreators {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VerifiedCreatorsSignature {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.address, state);
        core::hash::Hash::hash(&self.verified, state);
    }
}
impl core::fmt::Display for VerifiedCreatorsSignature {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DataState {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::AccountState => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::LedgerState => {
                core::hash::Hash::hash(&1usize, state);
            }
        }
    }
}
impl core::fmt::Display for DataState {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExternalPluginAdapterSchema {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Binary => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Json => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::MsgPack => {
                core::hash::Hash::hash(&2usize, state);
            }
        }
    }
}
impl core::fmt::Display for ExternalPluginAdapterSchema {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExternalPluginAdapterType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::LifecycleHook => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Oracle => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::AppData => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::LinkedLifecycleHook => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::LinkedAppData => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::DataSection => {
                core::hash::Hash::hash(&5usize, state);
            }
            Self::AgentIdentity => {
                core::hash::Hash::hash(&6usize, state);
            }
        }
    }
}
impl core::fmt::Display for ExternalPluginAdapterType {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExternalValidationResult {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Approved => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Rejected => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Pass => {
                core::hash::Hash::hash(&2usize, state);
            }
        }
    }
}
impl core::fmt::Display for ExternalValidationResult {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for HookableLifecycleEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Create => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Transfer => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Burn => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::Update => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::Execute => {
                core::hash::Hash::hash(&4usize, state);
            }
        }
    }
}
impl core::fmt::Display for HookableLifecycleEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Key {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Uninitialized => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::AssetV1 => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::HashedAssetV1 => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::PluginHeaderV1 => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::PluginRegistryV1 => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::CollectionV1 => {
                core::hash::Hash::hash(&5usize, state);
            }
            Self::GroupV1 => {
                core::hash::Hash::hash(&6usize, state);
            }
        }
    }
}
impl core::fmt::Display for Key {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PluginType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Royalties => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::FreezeDelegate => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::BurnDelegate => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::TransferDelegate => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::UpdateDelegate => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::PermanentFreezeDelegate => {
                core::hash::Hash::hash(&5usize, state);
            }
            Self::Attributes => {
                core::hash::Hash::hash(&6usize, state);
            }
            Self::PermanentTransferDelegate => {
                core::hash::Hash::hash(&7usize, state);
            }
            Self::PermanentBurnDelegate => {
                core::hash::Hash::hash(&8usize, state);
            }
            Self::Edition => {
                core::hash::Hash::hash(&9usize, state);
            }
            Self::MasterEdition => {
                core::hash::Hash::hash(&10usize, state);
            }
            Self::AddBlocker => {
                core::hash::Hash::hash(&11usize, state);
            }
            Self::ImmutableMetadata => {
                core::hash::Hash::hash(&12usize, state);
            }
            Self::VerifiedCreators => {
                core::hash::Hash::hash(&13usize, state);
            }
            Self::Autograph => {
                core::hash::Hash::hash(&14usize, state);
            }
            Self::BubblegumV2 => {
                core::hash::Hash::hash(&15usize, state);
            }
            Self::FreezeExecute => {
                core::hash::Hash::hash(&16usize, state);
            }
            Self::PermanentFreezeExecute => {
                core::hash::Hash::hash(&17usize, state);
            }
            Self::Groups => {
                core::hash::Hash::hash(&18usize, state);
            }
        }
    }
}
impl core::fmt::Display for PluginType {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RelationshipKind {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Collection => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::ChildGroup => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::ParentGroup => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::Asset => {
                core::hash::Hash::hash(&3usize, state);
            }
        }
    }
}
impl core::fmt::Display for RelationshipKind {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Mint => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Add => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Remove => {
                core::hash::Hash::hash(&2usize, state);
            }
        }
    }
}
impl core::fmt::Display for UpdateType {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ValidationResult {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Approved => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Rejected => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Pass => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::ForceApproved => {
                core::hash::Hash::hash(&3usize, state);
            }
        }
    }
}
impl core::fmt::Display for ValidationResult {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AuthorityAddressPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.address, state);
    }
}
impl core::fmt::Display for AuthorityAddressPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Authority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::None => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Owner => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::UpdateAuthority => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::Address(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for Authority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::None => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Address(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Collection(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for UpdateAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RuleSet {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::None => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::ProgramAllowList(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProgramDenyList(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for RuleSet {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Seed {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Collection => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Owner => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Recipient => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::Asset => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::Address(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Bytes(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for Seed {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ValidationResultsOffset {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::NoOffset => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Anchor => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Custom(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for ValidationResultsOffset {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LinkedDataKey {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::LinkedLifecycleHook(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::LinkedAppData(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for LinkedDataKey {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for OracleValidationV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.create, state);
        core::hash::Hash::hash(&self.transfer, state);
        core::hash::Hash::hash(&self.burn, state);
        core::hash::Hash::hash(&self.update, state);
    }
}
impl core::fmt::Display for OracleValidationV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for OracleValidation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Uninitialized => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::V1(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for OracleValidation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExtraAccountPreconfiguredProgramPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.is_signer, state);
        core::hash::Hash::hash(&self.is_writable, state);
    }
}
impl core::fmt::Display for ExtraAccountPreconfiguredProgramPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExtraAccountPreconfiguredCollectionPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.is_signer, state);
        core::hash::Hash::hash(&self.is_writable, state);
    }
}
impl core::fmt::Display for ExtraAccountPreconfiguredCollectionPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExtraAccountPreconfiguredOwnerPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.is_signer, state);
        core::hash::Hash::hash(&self.is_writable, state);
    }
}
impl core::fmt::Display for ExtraAccountPreconfiguredOwnerPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExtraAccountPreconfiguredRecipientPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.is_signer, state);
        core::hash::Hash::hash(&self.is_writable, state);
    }
}
impl core::fmt::Display for ExtraAccountPreconfiguredRecipientPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExtraAccountPreconfiguredAssetPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.is_signer, state);
        core::hash::Hash::hash(&self.is_writable, state);
    }
}
impl core::fmt::Display for ExtraAccountPreconfiguredAssetPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExtraAccountCustomPdaPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.seeds, state);
        core::hash::Hash::hash(&self.custom_program_id, state);
        core::hash::Hash::hash(&self.is_signer, state);
        core::hash::Hash::hash(&self.is_writable, state);
    }
}
impl core::fmt::Display for ExtraAccountCustomPdaPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExtraAccountAddressPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.address, state);
        core::hash::Hash::hash(&self.is_signer, state);
        core::hash::Hash::hash(&self.is_writable, state);
    }
}
impl core::fmt::Display for ExtraAccountAddressPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExtraAccount {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::PreconfiguredProgram(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::PreconfiguredCollection(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::PreconfiguredOwner(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::PreconfiguredRecipient(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::PreconfiguredAsset(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CustomPda(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Address(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for ExtraAccount {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Plugin {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Royalties(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::FreezeDelegate(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BurnDelegate(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransferDelegate(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateDelegate(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::PermanentFreezeDelegate(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Attributes(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::PermanentTransferDelegate(value) => {
                core::hash::Hash::hash(&7usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::PermanentBurnDelegate(value) => {
                core::hash::Hash::hash(&8usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Edition(value) => {
                core::hash::Hash::hash(&9usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MasterEdition(value) => {
                core::hash::Hash::hash(&10usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AddBlocker(value) => {
                core::hash::Hash::hash(&11usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ImmutableMetadata(value) => {
                core::hash::Hash::hash(&12usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VerifiedCreators(value) => {
                core::hash::Hash::hash(&13usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Autograph(value) => {
                core::hash::Hash::hash(&14usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BubblegumV2(value) => {
                core::hash::Hash::hash(&15usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::FreezeExecute(value) => {
                core::hash::Hash::hash(&16usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::PermanentFreezeExecute(value) => {
                core::hash::Hash::hash(&17usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Groups(value) => {
                core::hash::Hash::hash(&18usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for Plugin {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExternalPluginAdapter {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::LifecycleHook(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Oracle(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AppData(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::LinkedLifecycleHook(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::LinkedAppData(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DataSection(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AgentIdentity(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for ExternalPluginAdapter {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExternalPluginAdapterInitInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::LifecycleHook(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Oracle(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AppData(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::LinkedLifecycleHook(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::LinkedAppData(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DataSection(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AgentIdentity(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for ExternalPluginAdapterInitInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExternalPluginAdapterUpdateInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::LifecycleHook(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Oracle(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AppData(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::LinkedLifecycleHook(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::LinkedAppData(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AgentIdentity(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for ExternalPluginAdapterUpdateInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExternalPluginAdapterKey {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::LifecycleHook(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Oracle(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AppData(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::LinkedLifecycleHook(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::LinkedAppData(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DataSection(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AgentIdentity => {
                core::hash::Hash::hash(&6usize, state);
            }
        }
    }
}
impl core::fmt::Display for ExternalPluginAdapterKey {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AssetV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.owner, state);
        core::hash::Hash::hash(&self.update_authority, state);
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.seq, state);
    }
}
impl core::fmt::Display for AssetV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for HashedAssetV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.hash, state);
    }
}
impl core::fmt::Display for HashedAssetV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PluginHeaderV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin_registry_offset, state);
    }
}
impl core::fmt::Display for PluginHeaderV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PluginRegistryV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.registry, state);
        core::hash::Hash::hash(&self.external_registry, state);
    }
}
impl core::fmt::Display for PluginRegistryV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CollectionV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.update_authority, state);
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.num_minted, state);
        core::hash::Hash::hash(&self.current_size, state);
    }
}
impl core::fmt::Display for CollectionV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for GroupV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.update_authority, state);
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.collections, state);
        core::hash::Hash::hash(&self.groups, state);
        core::hash::Hash::hash(&self.parent_groups, state);
        core::hash::Hash::hash(&self.assets, state);
    }
}
impl core::fmt::Display for GroupV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.data_state, state);
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.plugins, state);
    }
}
impl core::fmt::Display for CreateV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateCollectionV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.plugins, state);
    }
}
impl core::fmt::Display for CreateCollectionV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AddPluginV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin, state);
        core::hash::Hash::hash(&self.init_authority, state);
    }
}
impl core::fmt::Display for AddPluginV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AddCollectionPluginV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin, state);
        core::hash::Hash::hash(&self.init_authority, state);
    }
}
impl core::fmt::Display for AddCollectionPluginV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RemovePluginV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin_type, state);
    }
}
impl core::fmt::Display for RemovePluginV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RemoveCollectionPluginV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin_type, state);
    }
}
impl core::fmt::Display for RemoveCollectionPluginV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdatePluginV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin, state);
    }
}
impl core::fmt::Display for UpdatePluginV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateCollectionPluginV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin, state);
    }
}
impl core::fmt::Display for UpdateCollectionPluginV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ApprovePluginAuthorityV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin_type, state);
        core::hash::Hash::hash(&self.new_authority, state);
    }
}
impl core::fmt::Display for ApprovePluginAuthorityV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ApproveCollectionPluginAuthorityV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin_type, state);
        core::hash::Hash::hash(&self.new_authority, state);
    }
}
impl core::fmt::Display for ApproveCollectionPluginAuthorityV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RevokePluginAuthorityV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin_type, state);
    }
}
impl core::fmt::Display for RevokePluginAuthorityV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RevokeCollectionPluginAuthorityV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plugin_type, state);
    }
}
impl core::fmt::Display for RevokeCollectionPluginAuthorityV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BurnV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.compression_proof, state);
    }
}
impl core::fmt::Display for BurnV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BurnCollectionV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.compression_proof, state);
    }
}
impl core::fmt::Display for BurnCollectionV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransferV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.compression_proof, state);
    }
}
impl core::fmt::Display for TransferV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_name, state);
        core::hash::Hash::hash(&self.new_uri, state);
        core::hash::Hash::hash(&self.new_update_authority, state);
    }
}
impl core::fmt::Display for UpdateV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateCollectionV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_name, state);
        core::hash::Hash::hash(&self.new_uri, state);
    }
}
impl core::fmt::Display for UpdateCollectionV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CompressV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CompressV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DecompressV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.compression_proof, state);
    }
}
impl core::fmt::Display for DecompressV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Collect {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for Collect {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.data_state, state);
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.plugins, state);
        core::hash::Hash::hash(&self.external_plugin_adapters, state);
    }
}
impl core::fmt::Display for CreateV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateCollectionV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.plugins, state);
        core::hash::Hash::hash(&self.external_plugin_adapters, state);
    }
}
impl core::fmt::Display for CreateCollectionV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AddExternalPluginAdapterV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.init_info, state);
    }
}
impl core::fmt::Display for AddExternalPluginAdapterV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AddCollectionExternalPluginAdapterV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.init_info, state);
    }
}
impl core::fmt::Display for AddCollectionExternalPluginAdapterV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RemoveExternalPluginAdapterV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.key, state);
    }
}
impl core::fmt::Display for RemoveExternalPluginAdapterV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RemoveCollectionExternalPluginAdapterV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.key, state);
    }
}
impl core::fmt::Display for RemoveCollectionExternalPluginAdapterV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateExternalPluginAdapterV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.key, state);
        core::hash::Hash::hash(&self.update_info, state);
    }
}
impl core::fmt::Display for UpdateExternalPluginAdapterV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateCollectionExternalPluginAdapterV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.key, state);
        core::hash::Hash::hash(&self.update_info, state);
    }
}
impl core::fmt::Display for UpdateCollectionExternalPluginAdapterV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for WriteExternalPluginAdapterDataV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.key, state);
        core::hash::Hash::hash(&self.data, state);
    }
}
impl core::fmt::Display for WriteExternalPluginAdapterDataV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for WriteCollectionExternalPluginAdapterDataV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.key, state);
        core::hash::Hash::hash(&self.data, state);
    }
}
impl core::fmt::Display for WriteCollectionExternalPluginAdapterDataV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_name, state);
        core::hash::Hash::hash(&self.new_uri, state);
        core::hash::Hash::hash(&self.new_update_authority, state);
    }
}
impl core::fmt::Display for UpdateV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ExecuteV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.instruction_data, state);
    }
}
impl core::fmt::Display for ExecuteV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateCollectionInfoV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.update_type, state);
        core::hash::Hash::hash(&self.amount, state);
    }
}
impl core::fmt::Display for UpdateCollectionInfoV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AddCollectionsToGroupV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for AddCollectionsToGroupV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RemoveCollectionsFromGroupV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.collections, state);
    }
}
impl core::fmt::Display for RemoveCollectionsFromGroupV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AddAssetsToGroupV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for AddAssetsToGroupV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RemoveAssetsFromGroupV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.assets, state);
    }
}
impl core::fmt::Display for RemoveAssetsFromGroupV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AddGroupsToGroupV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.groups, state);
    }
}
impl core::fmt::Display for AddGroupsToGroupV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RemoveGroupsFromGroupV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.groups, state);
    }
}
impl core::fmt::Display for RemoveGroupsFromGroupV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateGroupV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.relationships, state);
    }
}
impl core::fmt::Display for CreateGroupV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CloseGroupV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CloseGroupV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateGroupV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_name, state);
        core::hash::Hash::hash(&self.new_uri, state);
    }
}
impl core::fmt::Display for UpdateGroupV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MplCoreInstructionData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::CreateV1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateCollectionV1(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AddPluginV1(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AddCollectionPluginV1(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RemovePluginV1(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RemoveCollectionPluginV1(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdatePluginV1(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateCollectionPluginV1(value) => {
                core::hash::Hash::hash(&7usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ApprovePluginAuthorityV1(value) => {
                core::hash::Hash::hash(&8usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ApproveCollectionPluginAuthorityV1(value) => {
                core::hash::Hash::hash(&9usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RevokePluginAuthorityV1(value) => {
                core::hash::Hash::hash(&10usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RevokeCollectionPluginAuthorityV1(value) => {
                core::hash::Hash::hash(&11usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BurnV1(value) => {
                core::hash::Hash::hash(&12usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BurnCollectionV1(value) => {
                core::hash::Hash::hash(&13usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransferV1(value) => {
                core::hash::Hash::hash(&14usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateV1(value) => {
                core::hash::Hash::hash(&15usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateCollectionV1(value) => {
                core::hash::Hash::hash(&16usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CompressV1(value) => {
                core::hash::Hash::hash(&17usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DecompressV1(value) => {
                core::hash::Hash::hash(&18usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Collect(value) => {
                core::hash::Hash::hash(&19usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateV2(value) => {
                core::hash::Hash::hash(&20usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateCollectionV2(value) => {
                core::hash::Hash::hash(&21usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AddExternalPluginAdapterV1(value) => {
                core::hash::Hash::hash(&22usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AddCollectionExternalPluginAdapterV1(value) => {
                core::hash::Hash::hash(&23usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RemoveExternalPluginAdapterV1(value) => {
                core::hash::Hash::hash(&24usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RemoveCollectionExternalPluginAdapterV1(value) => {
                core::hash::Hash::hash(&25usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateExternalPluginAdapterV1(value) => {
                core::hash::Hash::hash(&26usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateCollectionExternalPluginAdapterV1(value) => {
                core::hash::Hash::hash(&27usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::WriteExternalPluginAdapterDataV1(value) => {
                core::hash::Hash::hash(&28usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::WriteCollectionExternalPluginAdapterDataV1(value) => {
                core::hash::Hash::hash(&29usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateV2(value) => {
                core::hash::Hash::hash(&30usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ExecuteV1(value) => {
                core::hash::Hash::hash(&31usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateCollectionInfoV1(value) => {
                core::hash::Hash::hash(&32usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AddCollectionsToGroupV1(value) => {
                core::hash::Hash::hash(&33usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RemoveCollectionsFromGroupV1(value) => {
                core::hash::Hash::hash(&34usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AddAssetsToGroupV1(value) => {
                core::hash::Hash::hash(&35usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RemoveAssetsFromGroupV1(value) => {
                core::hash::Hash::hash(&36usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AddGroupsToGroupV1(value) => {
                core::hash::Hash::hash(&37usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RemoveGroupsFromGroupV1(value) => {
                core::hash::Hash::hash(&38usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateGroupV1(value) => {
                core::hash::Hash::hash(&39usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CloseGroupV1(value) => {
                core::hash::Hash::hash(&40usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateGroupV1(value) => {
                core::hash::Hash::hash(&41usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for MplCoreInstructionData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MplCoreError {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.code(), state)
    }
}
impl core::fmt::Display for MplCoreError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.message() {
            Some(message) => formatter.write_str(message),
            None => write!(formatter, "unknown program error {}", self.code()),
        }
    }
}
impl core::error::Error for MplCoreError {}
