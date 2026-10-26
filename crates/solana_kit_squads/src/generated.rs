// Generated from a validated Codama model. See README.md and upstream notices.
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Constant, Decoder, Encoder, Error, Limits, Number, Reader, Writer},
};
/// Selected trusted program identity.
pub const PROGRAM: Address = Address::from_bytes([
    6, 129, 196, 206, 71, 226, 35, 104, 184, 177, 85, 94, 200, 135, 175, 9, 46, 252, 126, 251, 182,
    108, 163, 245, 47, 191, 104, 212, 172, 156, 183, 168,
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
const PROGRAM_ERROR_6009: u32 = 6009;
const PROGRAM_ERROR_6010: u32 = 6010;
const PROGRAM_ERROR_6011: u32 = 6011;
const PROGRAM_ERROR_6012: u32 = 6012;
const PROGRAM_ERROR_6013: u32 = 6013;
const PROGRAM_ERROR_6014: u32 = 6014;
const PROGRAM_ERROR_6015: u32 = 6015;
const PROGRAM_ERROR_6016: u32 = 6016;
const PROGRAM_ERROR_6017: u32 = 6017;
const PROGRAM_ERROR_6018: u32 = 6018;
const PROGRAM_ERROR_6019: u32 = 6019;
const PROGRAM_ERROR_6020: u32 = 6020;
const PROGRAM_ERROR_6021: u32 = 6021;
const PROGRAM_ERROR_6022: u32 = 6022;
const PROGRAM_ERROR_6023: u32 = 6023;
const PROGRAM_ERROR_6024: u32 = 6024;
const PROGRAM_ERROR_6025: u32 = 6025;
const PROGRAM_ERROR_6026: u32 = 6026;
const PROGRAM_ERROR_6027: u32 = 6027;
const PROGRAM_ERROR_6028: u32 = 6028;
const PROGRAM_ERROR_6029: u32 = 6029;
const PROGRAM_ERROR_6030: u32 = 6030;
const PROGRAM_ERROR_6031: u32 = 6031;
const PROGRAM_ERROR_6032: u32 = 6032;
const PROGRAM_ERROR_6033: u32 = 6033;
const PROGRAM_ERROR_6034: u32 = 6034;
const PROGRAM_ERROR_6035: u32 = 6035;
const PROGRAM_ERROR_6036: u32 = 6036;
const PROGRAM_ERROR_6037: u32 = 6037;
const PROGRAM_ERROR_6038: u32 = 6038;
const PROGRAM_ERROR_6039: u32 = 6039;
const PROGRAM_ERROR_6040: u32 = 6040;
const PROGRAM_ERROR_6041: u32 = 6041;
const PROGRAM_ERROR_6042: u32 = 6042;
const PROGRAM_ERROR_6043: u32 = 6043;
const PROGRAM_ERROR_6044: u32 = 6044;

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
/// Validated data for `Member`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    /// The `key` wire field.
    pub key: Address,
    /// The `permissions` wire field.
    pub permissions: Permissions,
}
/// Bounded codec for [`Member`].
#[derive(Clone, Copy, Debug)]
pub struct MemberCodec;
impl Encoder<Member> for MemberCodec {
    fn encoded_len(&self, value: &Member) -> Result<usize, Error> {
        let size = 0usize;
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
        let size = size
            .checked_add(PermissionsCodec.encoded_len(&value.permissions)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Member, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.key, writer)?;
        PermissionsCodec.write(&value.permissions, writer)?;
        Ok(())
    }
}
impl Decoder<Member> for MemberCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Member, Error> {
        Ok(Member {
            key: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            permissions: PermissionsCodec.read(reader)?,
        })
    }
}
/// Validated data for `Permissions`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permissions {
    /// The `mask` wire field.
    pub mask: u8,
}
/// Bounded codec for [`Permissions`].
#[derive(Clone, Copy, Debug)]
pub struct PermissionsCodec;
impl Encoder<Permissions> for PermissionsCodec {
    fn encoded_len(&self, value: &Permissions) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.mask)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Permissions, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.mask, writer)?;
        Ok(())
    }
}
impl Decoder<Permissions> for PermissionsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Permissions, Error> {
        Ok(Permissions {
            mask: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `VaultTransactionMessage`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultTransactionMessage {
    /// The `num_signers` wire field.
    pub num_signers: u8,
    /// The `num_writable_signers` wire field.
    pub num_writable_signers: u8,
    /// The `num_writable_non_signers` wire field.
    pub num_writable_non_signers: u8,
    /// The `account_keys` wire field.
    pub account_keys: Vec<Address>,
    /// The `instructions` wire field.
    pub instructions: Vec<MultisigCompiledInstruction>,
    /// The `address_table_lookups` wire field.
    pub address_table_lookups: Vec<MultisigMessageAddressTableLookup>,
}
/// Bounded codec for [`VaultTransactionMessage`].
#[derive(Clone, Copy, Debug)]
pub struct VaultTransactionMessageCodec;
impl Encoder<VaultTransactionMessage> for VaultTransactionMessageCodec {
    fn encoded_len(&self, value: &VaultTransactionMessage) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.num_signers)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.num_writable_signers)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.num_writable_non_signers)?)
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
                .encoded_len(value.account_keys.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: MultisigCompiledInstructionCodec,
                    minimum: 9,
                }
                .encoded_len(value.instructions.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: MultisigMessageAddressTableLookupCodec,
                    minimum: 40,
                }
                .encoded_len(value.address_table_lookups.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &VaultTransactionMessage, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.num_signers, writer)?;
        Number::<u8>::LE.write(&value.num_writable_signers, writer)?;
        Number::<u8>::LE.write(&value.num_writable_non_signers, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.account_keys.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: MultisigCompiledInstructionCodec,
            minimum: 9,
        }
        .write(value.instructions.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: MultisigMessageAddressTableLookupCodec,
            minimum: 40,
        }
        .write(value.address_table_lookups.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<VaultTransactionMessage> for VaultTransactionMessageCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VaultTransactionMessage, Error> {
        Ok(VaultTransactionMessage {
            num_signers: Number::<u8>::LE.read(reader)?,
            num_writable_signers: Number::<u8>::LE.read(reader)?,
            num_writable_non_signers: Number::<u8>::LE.read(reader)?,
            account_keys: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
            instructions: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: MultisigCompiledInstructionCodec,
                minimum: 9,
            }
            .read(reader)?,
            address_table_lookups: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: MultisigMessageAddressTableLookupCodec,
                minimum: 40,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `MultisigCompiledInstruction`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigCompiledInstruction {
    /// The `program_id_index` wire field.
    pub program_id_index: u8,
    /// The `account_indexes` wire field.
    pub account_indexes: Vec<u8>,
    /// The `data` wire field.
    pub data: Vec<u8>,
}
/// Bounded codec for [`MultisigCompiledInstruction`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigCompiledInstructionCodec;
impl Encoder<MultisigCompiledInstruction> for MultisigCompiledInstructionCodec {
    fn encoded_len(&self, value: &MultisigCompiledInstruction) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.program_id_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.account_indexes.as_slice())?,
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
        Ok(size)
    }
    fn write(
        &self,
        value: &MultisigCompiledInstruction,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.program_id_index, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.account_indexes.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.data.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<MultisigCompiledInstruction> for MultisigCompiledInstructionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigCompiledInstruction, Error> {
        Ok(MultisigCompiledInstruction {
            program_id_index: Number::<u8>::LE.read(reader)?,
            account_indexes: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            data: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `MultisigMessageAddressTableLookup`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigMessageAddressTableLookup {
    /// The `account_key` wire field.
    pub account_key: Address,
    /// The `writable_indexes` wire field.
    pub writable_indexes: Vec<u8>,
    /// The `readonly_indexes` wire field.
    pub readonly_indexes: Vec<u8>,
}
/// Bounded codec for [`MultisigMessageAddressTableLookup`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigMessageAddressTableLookupCodec;
impl Encoder<MultisigMessageAddressTableLookup> for MultisigMessageAddressTableLookupCodec {
    fn encoded_len(&self, value: &MultisigMessageAddressTableLookup) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.account_key)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.writable_indexes.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.readonly_indexes.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &MultisigMessageAddressTableLookup,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.account_key, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.writable_indexes.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.readonly_indexes.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<MultisigMessageAddressTableLookup> for MultisigMessageAddressTableLookupCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigMessageAddressTableLookup, Error> {
        Ok(MultisigMessageAddressTableLookup {
            account_key: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            writable_indexes: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            readonly_indexes: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `ProposalVoteArgs`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalVoteArgs {
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`ProposalVoteArgs`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalVoteArgsCodec;
impl Encoder<ProposalVoteArgs> for ProposalVoteArgsCodec {
    fn encoded_len(&self, value: &ProposalVoteArgs) -> Result<usize, Error> {
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ProposalVoteArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalVoteArgs> for ProposalVoteArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalVoteArgs, Error> {
        Ok(ProposalVoteArgs {
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
/// Validated data for `VaultTransactionCreateArgs`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultTransactionCreateArgs {
    /// The `vault_index` wire field.
    pub vault_index: u8,
    /// The `ephemeral_signers` wire field.
    pub ephemeral_signers: u8,
    /// The `transaction_message` wire field.
    pub transaction_message: Vec<u8>,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`VaultTransactionCreateArgs`].
#[derive(Clone, Copy, Debug)]
pub struct VaultTransactionCreateArgsCodec;
impl Encoder<VaultTransactionCreateArgs> for VaultTransactionCreateArgsCodec {
    fn encoded_len(&self, value: &VaultTransactionCreateArgs) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.vault_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.ephemeral_signers)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.transaction_message.as_slice())?,
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &VaultTransactionCreateArgs,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.vault_index, writer)?;
        Number::<u8>::LE.write(&value.ephemeral_signers, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.transaction_message.as_slice(), writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<VaultTransactionCreateArgs> for VaultTransactionCreateArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VaultTransactionCreateArgs, Error> {
        Ok(VaultTransactionCreateArgs {
            vault_index: Number::<u8>::LE.read(reader)?,
            ephemeral_signers: Number::<u8>::LE.read(reader)?,
            transaction_message: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Vote {
    /// The `Approve` variant.
    Approve,
    /// The `Reject` variant.
    Reject,
    /// The `Cancel` variant.
    Cancel,
}
/// Bounded codec for [`Vote`].
#[derive(Clone, Copy, Debug)]
pub struct VoteCodec;
const VOTE_TAG_0: u8 = 0;
const VOTE_TAG_1: u8 = 1;
const VOTE_TAG_2: u8 = 2;
impl Encoder<Vote> for VoteCodec {
    fn encoded_len(&self, value: &Vote) -> Result<usize, Error> {
        let size = match value {
            Vote::Approve => 0usize,
            Vote::Reject => 0usize,
            Vote::Cancel => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &Vote, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            Vote::Approve => {
                Number::<u8>::LE.write(&VOTE_TAG_0, writer)?;
                Ok(())
            }
            Vote::Reject => {
                Number::<u8>::LE.write(&VOTE_TAG_1, writer)?;
                Ok(())
            }
            Vote::Cancel => {
                Number::<u8>::LE.write(&VOTE_TAG_2, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<Vote> for VoteCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vote, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            VOTE_TAG_0 => Ok(Vote::Approve),
            VOTE_TAG_1 => Ok(Vote::Reject),
            VOTE_TAG_2 => Ok(Vote::Cancel),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Period {
    /// The `OneTime` variant.
    OneTime,
    /// The `Day` variant.
    Day,
    /// The `Week` variant.
    Week,
    /// The `Month` variant.
    Month,
}
/// Bounded codec for [`Period`].
#[derive(Clone, Copy, Debug)]
pub struct PeriodCodec;
const PERIOD_TAG_0: u8 = 0;
const PERIOD_TAG_1: u8 = 1;
const PERIOD_TAG_2: u8 = 2;
const PERIOD_TAG_3: u8 = 3;
impl Encoder<Period> for PeriodCodec {
    fn encoded_len(&self, value: &Period) -> Result<usize, Error> {
        let size = match value {
            Period::OneTime => 0usize,
            Period::Day => 0usize,
            Period::Week => 0usize,
            Period::Month => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &Period, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            Period::OneTime => {
                Number::<u8>::LE.write(&PERIOD_TAG_0, writer)?;
                Ok(())
            }
            Period::Day => {
                Number::<u8>::LE.write(&PERIOD_TAG_1, writer)?;
                Ok(())
            }
            Period::Week => {
                Number::<u8>::LE.write(&PERIOD_TAG_2, writer)?;
                Ok(())
            }
            Period::Month => {
                Number::<u8>::LE.write(&PERIOD_TAG_3, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<Period> for PeriodCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Period, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            PERIOD_TAG_0 => Ok(Period::OneTime),
            PERIOD_TAG_1 => Ok(Period::Day),
            PERIOD_TAG_2 => Ok(Period::Week),
            PERIOD_TAG_3 => Ok(Period::Month),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `ProposalStatusDraftPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalStatusDraftPayload {
    /// The `timestamp` wire field.
    pub timestamp: i64,
}
/// Bounded codec for [`ProposalStatusDraftPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalStatusDraftPayloadCodec;
impl Encoder<ProposalStatusDraftPayload> for ProposalStatusDraftPayloadCodec {
    fn encoded_len(&self, value: &ProposalStatusDraftPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.timestamp)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ProposalStatusDraftPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<i64>::LE.write(&value.timestamp, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalStatusDraftPayload> for ProposalStatusDraftPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalStatusDraftPayload, Error> {
        Ok(ProposalStatusDraftPayload {
            timestamp: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ProposalStatusActivePayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalStatusActivePayload {
    /// The `timestamp` wire field.
    pub timestamp: i64,
}
/// Bounded codec for [`ProposalStatusActivePayload`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalStatusActivePayloadCodec;
impl Encoder<ProposalStatusActivePayload> for ProposalStatusActivePayloadCodec {
    fn encoded_len(&self, value: &ProposalStatusActivePayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.timestamp)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ProposalStatusActivePayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<i64>::LE.write(&value.timestamp, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalStatusActivePayload> for ProposalStatusActivePayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalStatusActivePayload, Error> {
        Ok(ProposalStatusActivePayload {
            timestamp: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ProposalStatusRejectedPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalStatusRejectedPayload {
    /// The `timestamp` wire field.
    pub timestamp: i64,
}
/// Bounded codec for [`ProposalStatusRejectedPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalStatusRejectedPayloadCodec;
impl Encoder<ProposalStatusRejectedPayload> for ProposalStatusRejectedPayloadCodec {
    fn encoded_len(&self, value: &ProposalStatusRejectedPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.timestamp)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ProposalStatusRejectedPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<i64>::LE.write(&value.timestamp, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalStatusRejectedPayload> for ProposalStatusRejectedPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalStatusRejectedPayload, Error> {
        Ok(ProposalStatusRejectedPayload {
            timestamp: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ProposalStatusApprovedPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalStatusApprovedPayload {
    /// The `timestamp` wire field.
    pub timestamp: i64,
}
/// Bounded codec for [`ProposalStatusApprovedPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalStatusApprovedPayloadCodec;
impl Encoder<ProposalStatusApprovedPayload> for ProposalStatusApprovedPayloadCodec {
    fn encoded_len(&self, value: &ProposalStatusApprovedPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.timestamp)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ProposalStatusApprovedPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<i64>::LE.write(&value.timestamp, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalStatusApprovedPayload> for ProposalStatusApprovedPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalStatusApprovedPayload, Error> {
        Ok(ProposalStatusApprovedPayload {
            timestamp: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ProposalStatusExecutedPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalStatusExecutedPayload {
    /// The `timestamp` wire field.
    pub timestamp: i64,
}
/// Bounded codec for [`ProposalStatusExecutedPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalStatusExecutedPayloadCodec;
impl Encoder<ProposalStatusExecutedPayload> for ProposalStatusExecutedPayloadCodec {
    fn encoded_len(&self, value: &ProposalStatusExecutedPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.timestamp)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ProposalStatusExecutedPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<i64>::LE.write(&value.timestamp, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalStatusExecutedPayload> for ProposalStatusExecutedPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalStatusExecutedPayload, Error> {
        Ok(ProposalStatusExecutedPayload {
            timestamp: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ProposalStatusCancelledPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalStatusCancelledPayload {
    /// The `timestamp` wire field.
    pub timestamp: i64,
}
/// Bounded codec for [`ProposalStatusCancelledPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalStatusCancelledPayloadCodec;
impl Encoder<ProposalStatusCancelledPayload> for ProposalStatusCancelledPayloadCodec {
    fn encoded_len(&self, value: &ProposalStatusCancelledPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.timestamp)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ProposalStatusCancelledPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<i64>::LE.write(&value.timestamp, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalStatusCancelledPayload> for ProposalStatusCancelledPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalStatusCancelledPayload, Error> {
        Ok(ProposalStatusCancelledPayload {
            timestamp: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    /// The `Draft` variant.
    Draft(ProposalStatusDraftPayload),
    /// The `Active` variant.
    Active(ProposalStatusActivePayload),
    /// The `Rejected` variant.
    Rejected(ProposalStatusRejectedPayload),
    /// The `Approved` variant.
    Approved(ProposalStatusApprovedPayload),
    /// The `Executing` variant.
    Executing,
    /// The `Executed` variant.
    Executed(ProposalStatusExecutedPayload),
    /// The `Cancelled` variant.
    Cancelled(ProposalStatusCancelledPayload),
}
/// Bounded codec for [`ProposalStatus`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalStatusCodec;
const PROPOSALSTATUS_TAG_0: u8 = 0;
const PROPOSALSTATUS_TAG_1: u8 = 1;
const PROPOSALSTATUS_TAG_2: u8 = 2;
const PROPOSALSTATUS_TAG_3: u8 = 3;
const PROPOSALSTATUS_TAG_4: u8 = 4;
const PROPOSALSTATUS_TAG_5: u8 = 5;
const PROPOSALSTATUS_TAG_6: u8 = 6;
impl Encoder<ProposalStatus> for ProposalStatusCodec {
    fn encoded_len(&self, value: &ProposalStatus) -> Result<usize, Error> {
        let size = match value {
            ProposalStatus::Draft(value) => ProposalStatusDraftPayloadCodec.encoded_len(value)?,
            ProposalStatus::Active(value) => ProposalStatusActivePayloadCodec.encoded_len(value)?,
            ProposalStatus::Rejected(value) => {
                ProposalStatusRejectedPayloadCodec.encoded_len(value)?
            }
            ProposalStatus::Approved(value) => {
                ProposalStatusApprovedPayloadCodec.encoded_len(value)?
            }
            ProposalStatus::Executing => 0usize,
            ProposalStatus::Executed(value) => {
                ProposalStatusExecutedPayloadCodec.encoded_len(value)?
            }
            ProposalStatus::Cancelled(value) => {
                ProposalStatusCancelledPayloadCodec.encoded_len(value)?
            }
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &ProposalStatus, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            ProposalStatus::Draft(value) => {
                Number::<u8>::LE.write(&PROPOSALSTATUS_TAG_0, writer)?;
                ProposalStatusDraftPayloadCodec.write(value, writer)
            }
            ProposalStatus::Active(value) => {
                Number::<u8>::LE.write(&PROPOSALSTATUS_TAG_1, writer)?;
                ProposalStatusActivePayloadCodec.write(value, writer)
            }
            ProposalStatus::Rejected(value) => {
                Number::<u8>::LE.write(&PROPOSALSTATUS_TAG_2, writer)?;
                ProposalStatusRejectedPayloadCodec.write(value, writer)
            }
            ProposalStatus::Approved(value) => {
                Number::<u8>::LE.write(&PROPOSALSTATUS_TAG_3, writer)?;
                ProposalStatusApprovedPayloadCodec.write(value, writer)
            }
            ProposalStatus::Executing => {
                Number::<u8>::LE.write(&PROPOSALSTATUS_TAG_4, writer)?;
                Ok(())
            }
            ProposalStatus::Executed(value) => {
                Number::<u8>::LE.write(&PROPOSALSTATUS_TAG_5, writer)?;
                ProposalStatusExecutedPayloadCodec.write(value, writer)
            }
            ProposalStatus::Cancelled(value) => {
                Number::<u8>::LE.write(&PROPOSALSTATUS_TAG_6, writer)?;
                ProposalStatusCancelledPayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<ProposalStatus> for ProposalStatusCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalStatus, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            PROPOSALSTATUS_TAG_0 => Ok(ProposalStatus::Draft(
                ProposalStatusDraftPayloadCodec.read(reader)?,
            )),
            PROPOSALSTATUS_TAG_1 => Ok(ProposalStatus::Active(
                ProposalStatusActivePayloadCodec.read(reader)?,
            )),
            PROPOSALSTATUS_TAG_2 => Ok(ProposalStatus::Rejected(
                ProposalStatusRejectedPayloadCodec.read(reader)?,
            )),
            PROPOSALSTATUS_TAG_3 => Ok(ProposalStatus::Approved(
                ProposalStatusApprovedPayloadCodec.read(reader)?,
            )),
            PROPOSALSTATUS_TAG_4 => Ok(ProposalStatus::Executing),
            PROPOSALSTATUS_TAG_5 => Ok(ProposalStatus::Executed(
                ProposalStatusExecutedPayloadCodec.read(reader)?,
            )),
            PROPOSALSTATUS_TAG_6 => Ok(ProposalStatus::Cancelled(
                ProposalStatusCancelledPayloadCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `ConfigActionAddMemberPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigActionAddMemberPayload {
    /// The `new_member` wire field.
    pub new_member: Member,
}
/// Bounded codec for [`ConfigActionAddMemberPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigActionAddMemberPayloadCodec;
impl Encoder<ConfigActionAddMemberPayload> for ConfigActionAddMemberPayloadCodec {
    fn encoded_len(&self, value: &ConfigActionAddMemberPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(MemberCodec.encoded_len(&value.new_member)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfigActionAddMemberPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        MemberCodec.write(&value.new_member, writer)?;
        Ok(())
    }
}
impl Decoder<ConfigActionAddMemberPayload> for ConfigActionAddMemberPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigActionAddMemberPayload, Error> {
        Ok(ConfigActionAddMemberPayload {
            new_member: MemberCodec.read(reader)?,
        })
    }
}
/// Validated data for `ConfigActionRemoveMemberPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigActionRemoveMemberPayload {
    /// The `old_member` wire field.
    pub old_member: Address,
}
/// Bounded codec for [`ConfigActionRemoveMemberPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigActionRemoveMemberPayloadCodec;
impl Encoder<ConfigActionRemoveMemberPayload> for ConfigActionRemoveMemberPayloadCodec {
    fn encoded_len(&self, value: &ConfigActionRemoveMemberPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.old_member)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfigActionRemoveMemberPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.old_member, writer)?;
        Ok(())
    }
}
impl Decoder<ConfigActionRemoveMemberPayload> for ConfigActionRemoveMemberPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigActionRemoveMemberPayload, Error> {
        Ok(ConfigActionRemoveMemberPayload {
            old_member: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `ConfigActionChangeThresholdPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigActionChangeThresholdPayload {
    /// The `new_threshold` wire field.
    pub new_threshold: u16,
}
/// Bounded codec for [`ConfigActionChangeThresholdPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigActionChangeThresholdPayloadCodec;
impl Encoder<ConfigActionChangeThresholdPayload> for ConfigActionChangeThresholdPayloadCodec {
    fn encoded_len(&self, value: &ConfigActionChangeThresholdPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.new_threshold)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfigActionChangeThresholdPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u16>::LE.write(&value.new_threshold, writer)?;
        Ok(())
    }
}
impl Decoder<ConfigActionChangeThresholdPayload> for ConfigActionChangeThresholdPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigActionChangeThresholdPayload, Error> {
        Ok(ConfigActionChangeThresholdPayload {
            new_threshold: Number::<u16>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ConfigActionSetTimeLockPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigActionSetTimeLockPayload {
    /// The `new_time_lock` wire field.
    pub new_time_lock: u32,
}
/// Bounded codec for [`ConfigActionSetTimeLockPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigActionSetTimeLockPayloadCodec;
impl Encoder<ConfigActionSetTimeLockPayload> for ConfigActionSetTimeLockPayloadCodec {
    fn encoded_len(&self, value: &ConfigActionSetTimeLockPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.new_time_lock)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfigActionSetTimeLockPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u32>::LE.write(&value.new_time_lock, writer)?;
        Ok(())
    }
}
impl Decoder<ConfigActionSetTimeLockPayload> for ConfigActionSetTimeLockPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigActionSetTimeLockPayload, Error> {
        Ok(ConfigActionSetTimeLockPayload {
            new_time_lock: Number::<u32>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ConfigActionAddSpendingLimitPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigActionAddSpendingLimitPayload {
    /// The `create_key` wire field.
    pub create_key: Address,
    /// The `vault_index` wire field.
    pub vault_index: u8,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `amount` wire field.
    pub amount: u64,
    /// The `period` wire field.
    pub period: Period,
    /// The `members` wire field.
    pub members: Vec<Address>,
    /// The `destinations` wire field.
    pub destinations: Vec<Address>,
}
/// Bounded codec for [`ConfigActionAddSpendingLimitPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigActionAddSpendingLimitPayloadCodec;
impl Encoder<ConfigActionAddSpendingLimitPayload> for ConfigActionAddSpendingLimitPayloadCodec {
    fn encoded_len(&self, value: &ConfigActionAddSpendingLimitPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.create_key)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.vault_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.mint)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(PeriodCodec.encoded_len(&value.period)?)
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
                .encoded_len(value.members.as_slice())?,
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
                .encoded_len(value.destinations.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfigActionAddSpendingLimitPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.create_key, writer)?;
        Number::<u8>::LE.write(&value.vault_index, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        PeriodCodec.write(&value.period, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.members.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.destinations.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<ConfigActionAddSpendingLimitPayload> for ConfigActionAddSpendingLimitPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigActionAddSpendingLimitPayload, Error> {
        Ok(ConfigActionAddSpendingLimitPayload {
            create_key: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            vault_index: Number::<u8>::LE.read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            amount: Number::<u64>::LE.read(reader)?,
            period: PeriodCodec.read(reader)?,
            members: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
            destinations: CheckedCounted {
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
/// Validated data for `ConfigActionRemoveSpendingLimitPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigActionRemoveSpendingLimitPayload {
    /// The `spending_limit` wire field.
    pub spending_limit: Address,
}
/// Bounded codec for [`ConfigActionRemoveSpendingLimitPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigActionRemoveSpendingLimitPayloadCodec;
impl Encoder<ConfigActionRemoveSpendingLimitPayload>
    for ConfigActionRemoveSpendingLimitPayloadCodec
{
    fn encoded_len(&self, value: &ConfigActionRemoveSpendingLimitPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.spending_limit)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfigActionRemoveSpendingLimitPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.spending_limit, writer)?;
        Ok(())
    }
}
impl Decoder<ConfigActionRemoveSpendingLimitPayload>
    for ConfigActionRemoveSpendingLimitPayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ConfigActionRemoveSpendingLimitPayload, Error> {
        Ok(ConfigActionRemoveSpendingLimitPayload {
            spending_limit: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `ConfigActionSetRentCollectorPayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigActionSetRentCollectorPayload {
    /// The `new_rent_collector` wire field.
    pub new_rent_collector: Option<Address>,
}
/// Bounded codec for [`ConfigActionSetRentCollectorPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigActionSetRentCollectorPayloadCodec;
impl Encoder<ConfigActionSetRentCollectorPayload> for ConfigActionSetRentCollectorPayloadCodec {
    fn encoded_len(&self, value: &ConfigActionSetRentCollectorPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.new_rent_collector)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfigActionSetRentCollectorPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.new_rent_collector, writer)?;
        Ok(())
    }
}
impl Decoder<ConfigActionSetRentCollectorPayload> for ConfigActionSetRentCollectorPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigActionSetRentCollectorPayload, Error> {
        Ok(ConfigActionSetRentCollectorPayload {
            new_rent_collector: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConfigAction {
    /// The `AddMember` variant.
    AddMember(ConfigActionAddMemberPayload),
    /// The `RemoveMember` variant.
    RemoveMember(ConfigActionRemoveMemberPayload),
    /// The `ChangeThreshold` variant.
    ChangeThreshold(ConfigActionChangeThresholdPayload),
    /// The `SetTimeLock` variant.
    SetTimeLock(ConfigActionSetTimeLockPayload),
    /// The `AddSpendingLimit` variant.
    AddSpendingLimit(ConfigActionAddSpendingLimitPayload),
    /// The `RemoveSpendingLimit` variant.
    RemoveSpendingLimit(ConfigActionRemoveSpendingLimitPayload),
    /// The `SetRentCollector` variant.
    SetRentCollector(ConfigActionSetRentCollectorPayload),
}
/// Bounded codec for [`ConfigAction`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigActionCodec;
const CONFIGACTION_TAG_0: u8 = 0;
const CONFIGACTION_TAG_1: u8 = 1;
const CONFIGACTION_TAG_2: u8 = 2;
const CONFIGACTION_TAG_3: u8 = 3;
const CONFIGACTION_TAG_4: u8 = 4;
const CONFIGACTION_TAG_5: u8 = 5;
const CONFIGACTION_TAG_6: u8 = 6;
impl Encoder<ConfigAction> for ConfigActionCodec {
    fn encoded_len(&self, value: &ConfigAction) -> Result<usize, Error> {
        let size = match value {
            ConfigAction::AddMember(value) => {
                ConfigActionAddMemberPayloadCodec.encoded_len(value)?
            }
            ConfigAction::RemoveMember(value) => {
                ConfigActionRemoveMemberPayloadCodec.encoded_len(value)?
            }
            ConfigAction::ChangeThreshold(value) => {
                ConfigActionChangeThresholdPayloadCodec.encoded_len(value)?
            }
            ConfigAction::SetTimeLock(value) => {
                ConfigActionSetTimeLockPayloadCodec.encoded_len(value)?
            }
            ConfigAction::AddSpendingLimit(value) => {
                ConfigActionAddSpendingLimitPayloadCodec.encoded_len(value)?
            }
            ConfigAction::RemoveSpendingLimit(value) => {
                ConfigActionRemoveSpendingLimitPayloadCodec.encoded_len(value)?
            }
            ConfigAction::SetRentCollector(value) => {
                ConfigActionSetRentCollectorPayloadCodec.encoded_len(value)?
            }
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &ConfigAction, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            ConfigAction::AddMember(value) => {
                Number::<u8>::LE.write(&CONFIGACTION_TAG_0, writer)?;
                ConfigActionAddMemberPayloadCodec.write(value, writer)
            }
            ConfigAction::RemoveMember(value) => {
                Number::<u8>::LE.write(&CONFIGACTION_TAG_1, writer)?;
                ConfigActionRemoveMemberPayloadCodec.write(value, writer)
            }
            ConfigAction::ChangeThreshold(value) => {
                Number::<u8>::LE.write(&CONFIGACTION_TAG_2, writer)?;
                ConfigActionChangeThresholdPayloadCodec.write(value, writer)
            }
            ConfigAction::SetTimeLock(value) => {
                Number::<u8>::LE.write(&CONFIGACTION_TAG_3, writer)?;
                ConfigActionSetTimeLockPayloadCodec.write(value, writer)
            }
            ConfigAction::AddSpendingLimit(value) => {
                Number::<u8>::LE.write(&CONFIGACTION_TAG_4, writer)?;
                ConfigActionAddSpendingLimitPayloadCodec.write(value, writer)
            }
            ConfigAction::RemoveSpendingLimit(value) => {
                Number::<u8>::LE.write(&CONFIGACTION_TAG_5, writer)?;
                ConfigActionRemoveSpendingLimitPayloadCodec.write(value, writer)
            }
            ConfigAction::SetRentCollector(value) => {
                Number::<u8>::LE.write(&CONFIGACTION_TAG_6, writer)?;
                ConfigActionSetRentCollectorPayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<ConfigAction> for ConfigActionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigAction, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            CONFIGACTION_TAG_0 => Ok(ConfigAction::AddMember(
                ConfigActionAddMemberPayloadCodec.read(reader)?,
            )),
            CONFIGACTION_TAG_1 => Ok(ConfigAction::RemoveMember(
                ConfigActionRemoveMemberPayloadCodec.read(reader)?,
            )),
            CONFIGACTION_TAG_2 => Ok(ConfigAction::ChangeThreshold(
                ConfigActionChangeThresholdPayloadCodec.read(reader)?,
            )),
            CONFIGACTION_TAG_3 => Ok(ConfigAction::SetTimeLock(
                ConfigActionSetTimeLockPayloadCodec.read(reader)?,
            )),
            CONFIGACTION_TAG_4 => Ok(ConfigAction::AddSpendingLimit(
                ConfigActionAddSpendingLimitPayloadCodec.read(reader)?,
            )),
            CONFIGACTION_TAG_5 => Ok(ConfigAction::RemoveSpendingLimit(
                ConfigActionRemoveSpendingLimitPayloadCodec.read(reader)?,
            )),
            CONFIGACTION_TAG_6 => Ok(ConfigAction::SetRentCollector(
                ConfigActionSetRentCollectorPayloadCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `Batch`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Batch {
    /// The `multisig` wire field.
    pub multisig: Address,
    /// The `creator` wire field.
    pub creator: Address,
    /// The `index` wire field.
    pub index: u64,
    /// The `bump` wire field.
    pub bump: u8,
    /// The `vault_index` wire field.
    pub vault_index: u8,
    /// The `vault_bump` wire field.
    pub vault_bump: u8,
    /// The `size` wire field.
    pub size: u32,
    /// The `executed_transaction_index` wire field.
    pub executed_transaction_index: u32,
}
/// Bounded codec for [`Batch`].
#[derive(Clone, Copy, Debug)]
pub struct BatchCodec;
impl Encoder<Batch> for BatchCodec {
    fn encoded_len(&self, value: &Batch) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[156, 194, 70, 44, 22, 88, 137, 44],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.multisig)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.creator)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.vault_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.vault_bump)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.size)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.executed_transaction_index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Batch, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[156, 194, 70, 44, 22, 88, 137, 44];
        let _record_start = writer.position();
        Affix {
            prefix: &[156, 194, 70, 44, 22, 88, 137, 44],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.multisig, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.creator, writer)?;
        Number::<u64>::LE.write(&value.index, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
        Number::<u8>::LE.write(&value.vault_index, writer)?;
        Number::<u8>::LE.write(&value.vault_bump, writer)?;
        Number::<u32>::LE.write(&value.size, writer)?;
        Number::<u32>::LE.write(&value.executed_transaction_index, writer)?;
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
impl Decoder<Batch> for BatchCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Batch, Error> {
        const _DISCRIMINATOR: &[u8] = &[156, 194, 70, 44, 22, 88, 137, 44];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = Batch {
            multisig: Affix {
                prefix: &[156, 194, 70, 44, 22, 88, 137, 44],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            creator: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            index: Number::<u64>::LE.read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
            vault_index: Number::<u8>::LE.read(reader)?,
            vault_bump: Number::<u8>::LE.read(reader)?,
            size: Number::<u32>::LE.read(reader)?,
            executed_transaction_index: Number::<u32>::LE.read(reader)?,
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
/// Validated data for `VaultBatchTransaction`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultBatchTransaction {
    /// The `bump` wire field.
    pub bump: u8,
    /// The `ephemeral_signer_bumps` wire field.
    pub ephemeral_signer_bumps: Vec<u8>,
    /// The `message` wire field.
    pub message: VaultTransactionMessage,
}
/// Bounded codec for [`VaultBatchTransaction`].
#[derive(Clone, Copy, Debug)]
pub struct VaultBatchTransactionCodec;
impl Encoder<VaultBatchTransaction> for VaultBatchTransactionCodec {
    fn encoded_len(&self, value: &VaultBatchTransaction) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[196, 121, 46, 36, 12, 19, 252, 7],
                    item: Number::<u8>::LE,
                    suffix: &[],
                }
                .encoded_len(&value.bump)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.ephemeral_signer_bumps.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(VaultTransactionMessageCodec.encoded_len(&value.message)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &VaultBatchTransaction, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[196, 121, 46, 36, 12, 19, 252, 7];
        let _record_start = writer.position();
        Affix {
            prefix: &[196, 121, 46, 36, 12, 19, 252, 7],
            item: Number::<u8>::LE,
            suffix: &[],
        }
        .write(&value.bump, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.ephemeral_signer_bumps.as_slice(), writer)?;
        VaultTransactionMessageCodec.write(&value.message, writer)?;
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
impl Decoder<VaultBatchTransaction> for VaultBatchTransactionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VaultBatchTransaction, Error> {
        const _DISCRIMINATOR: &[u8] = &[196, 121, 46, 36, 12, 19, 252, 7];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = VaultBatchTransaction {
            bump: Affix {
                prefix: &[196, 121, 46, 36, 12, 19, 252, 7],
                item: Number::<u8>::LE,
                suffix: &[],
            }
            .read(reader)?,
            ephemeral_signer_bumps: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            message: VaultTransactionMessageCodec.read(reader)?,
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
/// Validated data for `ConfigTransaction`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigTransaction {
    /// The `multisig` wire field.
    pub multisig: Address,
    /// The `creator` wire field.
    pub creator: Address,
    /// The `index` wire field.
    pub index: u64,
    /// The `bump` wire field.
    pub bump: u8,
    /// The `actions` wire field.
    pub actions: Vec<ConfigAction>,
}
/// Bounded codec for [`ConfigTransaction`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigTransactionCodec;
impl Encoder<ConfigTransaction> for ConfigTransactionCodec {
    fn encoded_len(&self, value: &ConfigTransaction) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[94, 8, 4, 35, 113, 139, 139, 112],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.multisig)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.creator)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ConfigActionCodec,
                    minimum: 2,
                }
                .encoded_len(value.actions.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ConfigTransaction, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[94, 8, 4, 35, 113, 139, 139, 112];
        let _record_start = writer.position();
        Affix {
            prefix: &[94, 8, 4, 35, 113, 139, 139, 112],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.multisig, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.creator, writer)?;
        Number::<u64>::LE.write(&value.index, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ConfigActionCodec,
            minimum: 2,
        }
        .write(value.actions.as_slice(), writer)?;
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
impl Decoder<ConfigTransaction> for ConfigTransactionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigTransaction, Error> {
        const _DISCRIMINATOR: &[u8] = &[94, 8, 4, 35, 113, 139, 139, 112];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = ConfigTransaction {
            multisig: Affix {
                prefix: &[94, 8, 4, 35, 113, 139, 139, 112],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            creator: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            index: Number::<u64>::LE.read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
            actions: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ConfigActionCodec,
                minimum: 2,
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
/// Validated data for `Multisig`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Multisig {
    /// The `create_key` wire field.
    pub create_key: Address,
    /// The `config_authority` wire field.
    pub config_authority: Address,
    /// The `threshold` wire field.
    pub threshold: u16,
    /// The `time_lock` wire field.
    pub time_lock: u32,
    /// The `transaction_index` wire field.
    pub transaction_index: u64,
    /// The `stale_transaction_index` wire field.
    pub stale_transaction_index: u64,
    /// The `rent_collector` wire field.
    pub rent_collector: Option<Address>,
    /// The `bump` wire field.
    pub bump: u8,
    /// The `members` wire field.
    pub members: Vec<Member>,
}
/// Bounded codec for [`Multisig`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigCodec;
impl Encoder<Multisig> for MultisigCodec {
    fn encoded_len(&self, value: &Multisig) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[224, 116, 121, 186, 68, 161, 79, 236],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.create_key)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.config_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.threshold)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.time_lock)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.transaction_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.stale_transaction_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.rent_collector)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: MemberCodec,
                    minimum: 33,
                }
                .encoded_len(value.members.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Multisig, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[224, 116, 121, 186, 68, 161, 79, 236];
        let _record_start = writer.position();
        Affix {
            prefix: &[224, 116, 121, 186, 68, 161, 79, 236],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.create_key, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.config_authority, writer)?;
        Number::<u16>::LE.write(&value.threshold, writer)?;
        Number::<u32>::LE.write(&value.time_lock, writer)?;
        Number::<u64>::LE.write(&value.transaction_index, writer)?;
        Number::<u64>::LE.write(&value.stale_transaction_index, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.rent_collector, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: MemberCodec,
            minimum: 33,
        }
        .write(value.members.as_slice(), writer)?;
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
impl Decoder<Multisig> for MultisigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Multisig, Error> {
        const _DISCRIMINATOR: &[u8] = &[224, 116, 121, 186, 68, 161, 79, 236];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = Multisig {
            create_key: Affix {
                prefix: &[224, 116, 121, 186, 68, 161, 79, 236],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            config_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            threshold: Number::<u16>::LE.read(reader)?,
            time_lock: Number::<u32>::LE.read(reader)?,
            transaction_index: Number::<u64>::LE.read(reader)?,
            stale_transaction_index: Number::<u64>::LE.read(reader)?,
            rent_collector: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
            members: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: MemberCodec,
                minimum: 33,
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
/// Validated data for `ProgramConfig`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramConfig {
    /// The `authority` wire field.
    pub authority: Address,
    /// The `multisig_creation_fee` wire field.
    pub multisig_creation_fee: u64,
    /// The `treasury` wire field.
    pub treasury: Address,
    /// The `reserved` wire field.
    pub reserved: [u8; 64],
}
/// Bounded codec for [`ProgramConfig`].
#[derive(Clone, Copy, Debug)]
pub struct ProgramConfigCodec;
impl Encoder<ProgramConfig> for ProgramConfigCodec {
    fn encoded_len(&self, value: &ProgramConfig) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[196, 210, 90, 231, 144, 149, 140, 63],
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
            .checked_add(Number::<u64>::LE.encoded_len(&value.multisig_creation_fee)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.treasury)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<64>.encoded_len(&value.reserved)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ProgramConfig, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[196, 210, 90, 231, 144, 149, 140, 63];
        let _record_start = writer.position();
        Affix {
            prefix: &[196, 210, 90, 231, 144, 149, 140, 63],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.authority, writer)?;
        Number::<u64>::LE.write(&value.multisig_creation_fee, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.treasury, writer)?;
        FixedBytes::<64>.write(&value.reserved, writer)?;
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
impl Decoder<ProgramConfig> for ProgramConfigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProgramConfig, Error> {
        const _DISCRIMINATOR: &[u8] = &[196, 210, 90, 231, 144, 149, 140, 63];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = ProgramConfig {
            authority: Affix {
                prefix: &[196, 210, 90, 231, 144, 149, 140, 63],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            multisig_creation_fee: Number::<u64>::LE.read(reader)?,
            treasury: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            reserved: FixedBytes::<64>.read(reader)?,
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
/// Validated data for `Proposal`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    /// The `multisig` wire field.
    pub multisig: Address,
    /// The `transaction_index` wire field.
    pub transaction_index: u64,
    /// The `status` wire field.
    pub status: ProposalStatus,
    /// The `bump` wire field.
    pub bump: u8,
    /// The `approved` wire field.
    pub approved: Vec<Address>,
    /// The `rejected` wire field.
    pub rejected: Vec<Address>,
    /// The `cancelled` wire field.
    pub cancelled: Vec<Address>,
}
/// Bounded codec for [`Proposal`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalCodec;
impl Encoder<Proposal> for ProposalCodec {
    fn encoded_len(&self, value: &Proposal) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[26, 94, 189, 187, 116, 136, 53, 33],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.multisig)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.transaction_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(ProposalStatusCodec.encoded_len(&value.status)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
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
                .encoded_len(value.approved.as_slice())?,
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
                .encoded_len(value.rejected.as_slice())?,
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
                .encoded_len(value.cancelled.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Proposal, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[26, 94, 189, 187, 116, 136, 53, 33];
        let _record_start = writer.position();
        Affix {
            prefix: &[26, 94, 189, 187, 116, 136, 53, 33],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.multisig, writer)?;
        Number::<u64>::LE.write(&value.transaction_index, writer)?;
        ProposalStatusCodec.write(&value.status, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.approved.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.rejected.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.cancelled.as_slice(), writer)?;
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
impl Decoder<Proposal> for ProposalCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Proposal, Error> {
        const _DISCRIMINATOR: &[u8] = &[26, 94, 189, 187, 116, 136, 53, 33];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = Proposal {
            multisig: Affix {
                prefix: &[26, 94, 189, 187, 116, 136, 53, 33],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            transaction_index: Number::<u64>::LE.read(reader)?,
            status: ProposalStatusCodec.read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
            approved: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
            rejected: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
            cancelled: CheckedCounted {
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
/// Validated data for `SpendingLimit`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpendingLimit {
    /// The `multisig` wire field.
    pub multisig: Address,
    /// The `create_key` wire field.
    pub create_key: Address,
    /// The `vault_index` wire field.
    pub vault_index: u8,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `amount` wire field.
    pub amount: u64,
    /// The `period` wire field.
    pub period: Period,
    /// The `remaining_amount` wire field.
    pub remaining_amount: u64,
    /// The `last_reset` wire field.
    pub last_reset: i64,
    /// The `bump` wire field.
    pub bump: u8,
    /// The `members` wire field.
    pub members: Vec<Address>,
    /// The `destinations` wire field.
    pub destinations: Vec<Address>,
}
/// Bounded codec for [`SpendingLimit`].
#[derive(Clone, Copy, Debug)]
pub struct SpendingLimitCodec;
impl Encoder<SpendingLimit> for SpendingLimitCodec {
    fn encoded_len(&self, value: &SpendingLimit) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[10, 201, 27, 160, 218, 195, 222, 152],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.multisig)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.create_key)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.vault_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.mint)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(PeriodCodec.encoded_len(&value.period)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.remaining_amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.last_reset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
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
                .encoded_len(value.members.as_slice())?,
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
                .encoded_len(value.destinations.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SpendingLimit, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[10, 201, 27, 160, 218, 195, 222, 152];
        let _record_start = writer.position();
        Affix {
            prefix: &[10, 201, 27, 160, 218, 195, 222, 152],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.multisig, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.create_key, writer)?;
        Number::<u8>::LE.write(&value.vault_index, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        PeriodCodec.write(&value.period, writer)?;
        Number::<u64>::LE.write(&value.remaining_amount, writer)?;
        Number::<i64>::LE.write(&value.last_reset, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.members.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.destinations.as_slice(), writer)?;
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
impl Decoder<SpendingLimit> for SpendingLimitCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SpendingLimit, Error> {
        const _DISCRIMINATOR: &[u8] = &[10, 201, 27, 160, 218, 195, 222, 152];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = SpendingLimit {
            multisig: Affix {
                prefix: &[10, 201, 27, 160, 218, 195, 222, 152],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            create_key: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            vault_index: Number::<u8>::LE.read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            amount: Number::<u64>::LE.read(reader)?,
            period: PeriodCodec.read(reader)?,
            remaining_amount: Number::<u64>::LE.read(reader)?,
            last_reset: Number::<i64>::LE.read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
            members: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
            destinations: CheckedCounted {
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
/// Validated data for `TransactionBuffer`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionBuffer {
    /// The `multisig` wire field.
    pub multisig: Address,
    /// The `creator` wire field.
    pub creator: Address,
    /// The `buffer_index` wire field.
    pub buffer_index: u8,
    /// The `vault_index` wire field.
    pub vault_index: u8,
    /// The `final_buffer_hash` wire field.
    pub final_buffer_hash: [u8; 32],
    /// The `final_buffer_size` wire field.
    pub final_buffer_size: u16,
    /// The `buffer` wire field.
    pub buffer: Vec<u8>,
}
/// Bounded codec for [`TransactionBuffer`].
#[derive(Clone, Copy, Debug)]
pub struct TransactionBufferCodec;
impl Encoder<TransactionBuffer> for TransactionBufferCodec {
    fn encoded_len(&self, value: &TransactionBuffer) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[90, 36, 35, 219, 93, 225, 110, 96],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.multisig)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.creator)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.buffer_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.vault_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.final_buffer_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.final_buffer_size)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.buffer.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransactionBuffer, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[90, 36, 35, 219, 93, 225, 110, 96];
        let _record_start = writer.position();
        Affix {
            prefix: &[90, 36, 35, 219, 93, 225, 110, 96],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.multisig, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.creator, writer)?;
        Number::<u8>::LE.write(&value.buffer_index, writer)?;
        Number::<u8>::LE.write(&value.vault_index, writer)?;
        FixedBytes::<32>.write(&value.final_buffer_hash, writer)?;
        Number::<u16>::LE.write(&value.final_buffer_size, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.buffer.as_slice(), writer)?;
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
impl Decoder<TransactionBuffer> for TransactionBufferCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransactionBuffer, Error> {
        const _DISCRIMINATOR: &[u8] = &[90, 36, 35, 219, 93, 225, 110, 96];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = TransactionBuffer {
            multisig: Affix {
                prefix: &[90, 36, 35, 219, 93, 225, 110, 96],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            creator: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            buffer_index: Number::<u8>::LE.read(reader)?,
            vault_index: Number::<u8>::LE.read(reader)?,
            final_buffer_hash: FixedBytes::<32>.read(reader)?,
            final_buffer_size: Number::<u16>::LE.read(reader)?,
            buffer: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
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
/// Validated data for `VaultTransaction`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultTransaction {
    /// The `multisig` wire field.
    pub multisig: Address,
    /// The `creator` wire field.
    pub creator: Address,
    /// The `index` wire field.
    pub index: u64,
    /// The `bump` wire field.
    pub bump: u8,
    /// The `vault_index` wire field.
    pub vault_index: u8,
    /// The `vault_bump` wire field.
    pub vault_bump: u8,
    /// The `ephemeral_signer_bumps` wire field.
    pub ephemeral_signer_bumps: Vec<u8>,
    /// The `message` wire field.
    pub message: VaultTransactionMessage,
}
/// Bounded codec for [`VaultTransaction`].
#[derive(Clone, Copy, Debug)]
pub struct VaultTransactionCodec;
impl Encoder<VaultTransaction> for VaultTransactionCodec {
    fn encoded_len(&self, value: &VaultTransaction) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[168, 250, 162, 100, 81, 14, 162, 207],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.multisig)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.creator)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.vault_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.vault_bump)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.ephemeral_signer_bumps.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(VaultTransactionMessageCodec.encoded_len(&value.message)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &VaultTransaction, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[168, 250, 162, 100, 81, 14, 162, 207];
        let _record_start = writer.position();
        Affix {
            prefix: &[168, 250, 162, 100, 81, 14, 162, 207],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.multisig, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.creator, writer)?;
        Number::<u64>::LE.write(&value.index, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
        Number::<u8>::LE.write(&value.vault_index, writer)?;
        Number::<u8>::LE.write(&value.vault_bump, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.ephemeral_signer_bumps.as_slice(), writer)?;
        VaultTransactionMessageCodec.write(&value.message, writer)?;
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
impl Decoder<VaultTransaction> for VaultTransactionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VaultTransaction, Error> {
        const _DISCRIMINATOR: &[u8] = &[168, 250, 162, 100, 81, 14, 162, 207];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = VaultTransaction {
            multisig: Affix {
                prefix: &[168, 250, 162, 100, 81, 14, 162, 207],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            creator: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            index: Number::<u64>::LE.read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
            vault_index: Number::<u8>::LE.read(reader)?,
            vault_bump: Number::<u8>::LE.read(reader)?,
            ephemeral_signer_bumps: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
            message: VaultTransactionMessageCodec.read(reader)?,
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
/// Validated data for `ProgramConfigInit`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramConfigInit {
    /// The `authority` wire field.
    pub authority: Address,
    /// The `multisig_creation_fee` wire field.
    pub multisig_creation_fee: u64,
    /// The `treasury` wire field.
    pub treasury: Address,
}
/// Bounded codec for [`ProgramConfigInit`].
#[derive(Clone, Copy, Debug)]
pub struct ProgramConfigInitCodec;
impl Encoder<ProgramConfigInit> for ProgramConfigInitCodec {
    fn encoded_len(&self, value: &ProgramConfigInit) -> Result<usize, Error> {
        let size = 8usize;
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
            .checked_add(Number::<u64>::LE.encoded_len(&value.multisig_creation_fee)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.treasury)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ProgramConfigInit, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[184, 188, 198, 195, 205, 124, 117, 216]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.authority, writer)?;
        Number::<u64>::LE.write(&value.multisig_creation_fee, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.treasury, writer)?;
        Ok(())
    }
}
impl Decoder<ProgramConfigInit> for ProgramConfigInitCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProgramConfigInit, Error> {
        Constant(&[184, 188, 198, 195, 205, 124, 117, 216]).read(reader)?;
        Ok(ProgramConfigInit {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            multisig_creation_fee: Number::<u64>::LE.read(reader)?,
            treasury: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
impl ProgramConfigInit {
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
                /* program_config */ (true, Some(false), false),
                /* initializer */ (true, Some(true), false),
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
        let data = ProgramConfigInitCodec.encode(self, limits.max_bytes)?;
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
        ProgramConfigInitCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ProgramConfigSetAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramConfigSetAuthority {
    /// The `new_authority` wire field.
    pub new_authority: Address,
}
/// Bounded codec for [`ProgramConfigSetAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct ProgramConfigSetAuthorityCodec;
impl Encoder<ProgramConfigSetAuthority> for ProgramConfigSetAuthorityCodec {
    fn encoded_len(&self, value: &ProgramConfigSetAuthority) -> Result<usize, Error> {
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
    fn write(
        &self,
        value: &ProgramConfigSetAuthority,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[238, 242, 36, 181, 32, 143, 216, 75]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.new_authority, writer)?;
        Ok(())
    }
}
impl Decoder<ProgramConfigSetAuthority> for ProgramConfigSetAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProgramConfigSetAuthority, Error> {
        Constant(&[238, 242, 36, 181, 32, 143, 216, 75]).read(reader)?;
        Ok(ProgramConfigSetAuthority {
            new_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
impl ProgramConfigSetAuthority {
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
                /* program_config */ (true, Some(false), false),
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
        let data = ProgramConfigSetAuthorityCodec.encode(self, limits.max_bytes)?;
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
        ProgramConfigSetAuthorityCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ProgramConfigSetMultisigCreationFee`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramConfigSetMultisigCreationFee {
    /// The `new_multisig_creation_fee` wire field.
    pub new_multisig_creation_fee: u64,
}
/// Bounded codec for [`ProgramConfigSetMultisigCreationFee`].
#[derive(Clone, Copy, Debug)]
pub struct ProgramConfigSetMultisigCreationFeeCodec;
impl Encoder<ProgramConfigSetMultisigCreationFee> for ProgramConfigSetMultisigCreationFeeCodec {
    fn encoded_len(&self, value: &ProgramConfigSetMultisigCreationFee) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.new_multisig_creation_fee)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ProgramConfigSetMultisigCreationFee,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[101, 160, 249, 63, 154, 215, 153, 13]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.new_multisig_creation_fee, writer)?;
        Ok(())
    }
}
impl Decoder<ProgramConfigSetMultisigCreationFee> for ProgramConfigSetMultisigCreationFeeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProgramConfigSetMultisigCreationFee, Error> {
        Constant(&[101, 160, 249, 63, 154, 215, 153, 13]).read(reader)?;
        Ok(ProgramConfigSetMultisigCreationFee {
            new_multisig_creation_fee: Number::<u64>::LE.read(reader)?,
        })
    }
}
impl ProgramConfigSetMultisigCreationFee {
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
                /* program_config */ (true, Some(false), false),
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
        let data = ProgramConfigSetMultisigCreationFeeCodec.encode(self, limits.max_bytes)?;
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
        ProgramConfigSetMultisigCreationFeeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ProgramConfigSetTreasury`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramConfigSetTreasury {
    /// The `new_treasury` wire field.
    pub new_treasury: Address,
}
/// Bounded codec for [`ProgramConfigSetTreasury`].
#[derive(Clone, Copy, Debug)]
pub struct ProgramConfigSetTreasuryCodec;
impl Encoder<ProgramConfigSetTreasury> for ProgramConfigSetTreasuryCodec {
    fn encoded_len(&self, value: &ProgramConfigSetTreasury) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.new_treasury)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ProgramConfigSetTreasury,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[111, 46, 243, 117, 144, 188, 162, 107]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.new_treasury, writer)?;
        Ok(())
    }
}
impl Decoder<ProgramConfigSetTreasury> for ProgramConfigSetTreasuryCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProgramConfigSetTreasury, Error> {
        Constant(&[111, 46, 243, 117, 144, 188, 162, 107]).read(reader)?;
        Ok(ProgramConfigSetTreasury {
            new_treasury: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
impl ProgramConfigSetTreasury {
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
                /* program_config */ (true, Some(false), false),
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
        let data = ProgramConfigSetTreasuryCodec.encode(self, limits.max_bytes)?;
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
        ProgramConfigSetTreasuryCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MultisigCreate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigCreate {}
/// Bounded codec for [`MultisigCreate`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigCreateCodec;
impl Encoder<MultisigCreate> for MultisigCreateCodec {
    fn encoded_len(&self, value: &MultisigCreate) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &MultisigCreate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[122, 77, 80, 159, 84, 88, 90, 197]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<MultisigCreate> for MultisigCreateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigCreate, Error> {
        Constant(&[122, 77, 80, 159, 84, 88, 90, 197]).read(reader)?;
        let _ = reader;
        Ok(MultisigCreate {})
    }
}
impl MultisigCreate {
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
            .zip([/* null */ (false, Some(false), false)])
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
        let data = MultisigCreateCodec.encode(self, limits.max_bytes)?;
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
        MultisigCreateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MultisigCreateV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigCreateV2 {
    /// The `config_authority` wire field.
    pub config_authority: Option<Address>,
    /// The `threshold` wire field.
    pub threshold: u16,
    /// The `members` wire field.
    pub members: Vec<Member>,
    /// The `time_lock` wire field.
    pub time_lock: u32,
    /// The `rent_collector` wire field.
    pub rent_collector: Option<Address>,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`MultisigCreateV2`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigCreateV2Codec;
impl Encoder<MultisigCreateV2> for MultisigCreateV2Codec {
    fn encoded_len(&self, value: &MultisigCreateV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.config_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.threshold)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: MemberCodec,
                    minimum: 33,
                }
                .encoded_len(value.members.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.time_lock)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.rent_collector)?,
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MultisigCreateV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[50, 221, 199, 93, 40, 245, 139, 233]).write(&(), writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.config_authority, writer)?;
        Number::<u16>::LE.write(&value.threshold, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: MemberCodec,
            minimum: 33,
        }
        .write(value.members.as_slice(), writer)?;
        Number::<u32>::LE.write(&value.time_lock, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.rent_collector, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<MultisigCreateV2> for MultisigCreateV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigCreateV2, Error> {
        Constant(&[50, 221, 199, 93, 40, 245, 139, 233]).read(reader)?;
        Ok(MultisigCreateV2 {
            config_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            threshold: Number::<u16>::LE.read(reader)?,
            members: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: MemberCodec,
                minimum: 33,
            }
            .read(reader)?,
            time_lock: Number::<u32>::LE.read(reader)?,
            rent_collector: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl MultisigCreateV2 {
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
                /* program_config */ (false, Some(false), false),
                /* treasury */ (true, Some(false), false),
                /* multisig */ (true, Some(false), false),
                /* create_key */ (false, Some(true), false),
                /* creator */ (true, Some(true), false),
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
        let data = MultisigCreateV2Codec.encode(self, limits.max_bytes)?;
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
        MultisigCreateV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MultisigAddMember`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigAddMember {
    /// The `new_member` wire field.
    pub new_member: Member,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`MultisigAddMember`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigAddMemberCodec;
impl Encoder<MultisigAddMember> for MultisigAddMemberCodec {
    fn encoded_len(&self, value: &MultisigAddMember) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(MemberCodec.encoded_len(&value.new_member)?)
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MultisigAddMember, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[1, 219, 215, 108, 184, 229, 214, 8]).write(&(), writer)?;
        MemberCodec.write(&value.new_member, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<MultisigAddMember> for MultisigAddMemberCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigAddMember, Error> {
        Constant(&[1, 219, 215, 108, 184, 229, 214, 8]).read(reader)?;
        Ok(MultisigAddMember {
            new_member: MemberCodec.read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl MultisigAddMember {
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
                /* multisig */ (true, Some(false), false),
                /* config_authority */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), true),
                /* system_program */ (false, Some(false), true),
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
        let data = MultisigAddMemberCodec.encode(self, limits.max_bytes)?;
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
        MultisigAddMemberCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MultisigRemoveMember`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigRemoveMember {
    /// The `old_member` wire field.
    pub old_member: Address,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`MultisigRemoveMember`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigRemoveMemberCodec;
impl Encoder<MultisigRemoveMember> for MultisigRemoveMemberCodec {
    fn encoded_len(&self, value: &MultisigRemoveMember) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.old_member)?,
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MultisigRemoveMember, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[217, 117, 177, 210, 182, 145, 218, 72]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.old_member, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<MultisigRemoveMember> for MultisigRemoveMemberCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigRemoveMember, Error> {
        Constant(&[217, 117, 177, 210, 182, 145, 218, 72]).read(reader)?;
        Ok(MultisigRemoveMember {
            old_member: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl MultisigRemoveMember {
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
                /* multisig */ (true, Some(false), false),
                /* config_authority */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), true),
                /* system_program */ (false, Some(false), true),
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
        let data = MultisigRemoveMemberCodec.encode(self, limits.max_bytes)?;
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
        MultisigRemoveMemberCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MultisigSetTimeLock`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigSetTimeLock {
    /// The `time_lock` wire field.
    pub time_lock: u32,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`MultisigSetTimeLock`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigSetTimeLockCodec;
impl Encoder<MultisigSetTimeLock> for MultisigSetTimeLockCodec {
    fn encoded_len(&self, value: &MultisigSetTimeLock) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.time_lock)?)
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MultisigSetTimeLock, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[148, 154, 121, 77, 212, 254, 155, 72]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.time_lock, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<MultisigSetTimeLock> for MultisigSetTimeLockCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigSetTimeLock, Error> {
        Constant(&[148, 154, 121, 77, 212, 254, 155, 72]).read(reader)?;
        Ok(MultisigSetTimeLock {
            time_lock: Number::<u32>::LE.read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl MultisigSetTimeLock {
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
                /* multisig */ (true, Some(false), false),
                /* config_authority */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), true),
                /* system_program */ (false, Some(false), true),
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
        let data = MultisigSetTimeLockCodec.encode(self, limits.max_bytes)?;
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
        MultisigSetTimeLockCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MultisigChangeThreshold`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigChangeThreshold {
    /// The `new_threshold` wire field.
    pub new_threshold: u16,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`MultisigChangeThreshold`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigChangeThresholdCodec;
impl Encoder<MultisigChangeThreshold> for MultisigChangeThresholdCodec {
    fn encoded_len(&self, value: &MultisigChangeThreshold) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.new_threshold)?)
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MultisigChangeThreshold, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[141, 42, 15, 126, 169, 92, 62, 181]).write(&(), writer)?;
        Number::<u16>::LE.write(&value.new_threshold, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<MultisigChangeThreshold> for MultisigChangeThresholdCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigChangeThreshold, Error> {
        Constant(&[141, 42, 15, 126, 169, 92, 62, 181]).read(reader)?;
        Ok(MultisigChangeThreshold {
            new_threshold: Number::<u16>::LE.read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl MultisigChangeThreshold {
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
                /* multisig */ (true, Some(false), false),
                /* config_authority */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), true),
                /* system_program */ (false, Some(false), true),
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
        let data = MultisigChangeThresholdCodec.encode(self, limits.max_bytes)?;
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
        MultisigChangeThresholdCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MultisigSetConfigAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigSetConfigAuthority {
    /// The `new_config_authority` wire field.
    pub new_config_authority: Address,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`MultisigSetConfigAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigSetConfigAuthorityCodec;
impl Encoder<MultisigSetConfigAuthority> for MultisigSetConfigAuthorityCodec {
    fn encoded_len(&self, value: &MultisigSetConfigAuthority) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.new_config_authority)?,
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &MultisigSetConfigAuthority,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[143, 93, 199, 143, 92, 169, 193, 232]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.new_config_authority, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<MultisigSetConfigAuthority> for MultisigSetConfigAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigSetConfigAuthority, Error> {
        Constant(&[143, 93, 199, 143, 92, 169, 193, 232]).read(reader)?;
        Ok(MultisigSetConfigAuthority {
            new_config_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl MultisigSetConfigAuthority {
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
                /* multisig */ (true, Some(false), false),
                /* config_authority */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), true),
                /* system_program */ (false, Some(false), true),
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
        let data = MultisigSetConfigAuthorityCodec.encode(self, limits.max_bytes)?;
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
        MultisigSetConfigAuthorityCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MultisigSetRentCollector`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigSetRentCollector {
    /// The `rent_collector` wire field.
    pub rent_collector: Option<Address>,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`MultisigSetRentCollector`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigSetRentCollectorCodec;
impl Encoder<MultisigSetRentCollector> for MultisigSetRentCollectorCodec {
    fn encoded_len(&self, value: &MultisigSetRentCollector) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.rent_collector)?,
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &MultisigSetRentCollector,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[48, 204, 65, 57, 210, 70, 156, 74]).write(&(), writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.rent_collector, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<MultisigSetRentCollector> for MultisigSetRentCollectorCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigSetRentCollector, Error> {
        Constant(&[48, 204, 65, 57, 210, 70, 156, 74]).read(reader)?;
        Ok(MultisigSetRentCollector {
            rent_collector: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl MultisigSetRentCollector {
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
                /* multisig */ (true, Some(false), false),
                /* config_authority */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), true),
                /* system_program */ (false, Some(false), true),
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
        let data = MultisigSetRentCollectorCodec.encode(self, limits.max_bytes)?;
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
        MultisigSetRentCollectorCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MultisigAddSpendingLimit`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigAddSpendingLimit {
    /// The `create_key` wire field.
    pub create_key: Address,
    /// The `vault_index` wire field.
    pub vault_index: u8,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `amount` wire field.
    pub amount: u64,
    /// The `period` wire field.
    pub period: Period,
    /// The `members` wire field.
    pub members: Vec<Address>,
    /// The `destinations` wire field.
    pub destinations: Vec<Address>,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`MultisigAddSpendingLimit`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigAddSpendingLimitCodec;
impl Encoder<MultisigAddSpendingLimit> for MultisigAddSpendingLimitCodec {
    fn encoded_len(&self, value: &MultisigAddSpendingLimit) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.create_key)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.vault_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.mint)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(PeriodCodec.encoded_len(&value.period)?)
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
                .encoded_len(value.members.as_slice())?,
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
                .encoded_len(value.destinations.as_slice())?,
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &MultisigAddSpendingLimit,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[11, 242, 159, 42, 86, 197, 89, 115]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.create_key, writer)?;
        Number::<u8>::LE.write(&value.vault_index, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        PeriodCodec.write(&value.period, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.members.as_slice(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.destinations.as_slice(), writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<MultisigAddSpendingLimit> for MultisigAddSpendingLimitCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigAddSpendingLimit, Error> {
        Constant(&[11, 242, 159, 42, 86, 197, 89, 115]).read(reader)?;
        Ok(MultisigAddSpendingLimit {
            create_key: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            vault_index: Number::<u8>::LE.read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            amount: Number::<u64>::LE.read(reader)?,
            period: PeriodCodec.read(reader)?,
            members: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
            destinations: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl MultisigAddSpendingLimit {
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
                /* multisig */ (false, Some(false), false),
                /* config_authority */ (false, Some(true), false),
                /* spending_limit */ (true, Some(false), false),
                /* rent_payer */ (true, Some(true), false),
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
        let data = MultisigAddSpendingLimitCodec.encode(self, limits.max_bytes)?;
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
        MultisigAddSpendingLimitCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MultisigRemoveSpendingLimit`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultisigRemoveSpendingLimit {
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`MultisigRemoveSpendingLimit`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigRemoveSpendingLimitCodec;
impl Encoder<MultisigRemoveSpendingLimit> for MultisigRemoveSpendingLimitCodec {
    fn encoded_len(&self, value: &MultisigRemoveSpendingLimit) -> Result<usize, Error> {
        let size = 8usize;
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &MultisigRemoveSpendingLimit,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[228, 198, 136, 111, 123, 4, 178, 113]).write(&(), writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<MultisigRemoveSpendingLimit> for MultisigRemoveSpendingLimitCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MultisigRemoveSpendingLimit, Error> {
        Constant(&[228, 198, 136, 111, 123, 4, 178, 113]).read(reader)?;
        Ok(MultisigRemoveSpendingLimit {
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl MultisigRemoveSpendingLimit {
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
                /* multisig */ (false, Some(false), false),
                /* config_authority */ (false, Some(true), false),
                /* spending_limit */ (true, Some(false), false),
                /* rent_collector */ (true, Some(false), false),
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
        let data = MultisigRemoveSpendingLimitCodec.encode(self, limits.max_bytes)?;
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
        MultisigRemoveSpendingLimitCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ConfigTransactionCreate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigTransactionCreate {
    /// The `actions` wire field.
    pub actions: Vec<ConfigAction>,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`ConfigTransactionCreate`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigTransactionCreateCodec;
impl Encoder<ConfigTransactionCreate> for ConfigTransactionCreateCodec {
    fn encoded_len(&self, value: &ConfigTransactionCreate) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ConfigActionCodec,
                    minimum: 2,
                }
                .encoded_len(value.actions.as_slice())?,
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ConfigTransactionCreate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[155, 236, 87, 228, 137, 75, 81, 39]).write(&(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ConfigActionCodec,
            minimum: 2,
        }
        .write(value.actions.as_slice(), writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<ConfigTransactionCreate> for ConfigTransactionCreateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigTransactionCreate, Error> {
        Constant(&[155, 236, 87, 228, 137, 75, 81, 39]).read(reader)?;
        Ok(ConfigTransactionCreate {
            actions: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ConfigActionCodec,
                minimum: 2,
            }
            .read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl ConfigTransactionCreate {
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
                /* multisig */ (true, Some(false), false),
                /* transaction */ (true, Some(false), false),
                /* creator */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), false),
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
        let data = ConfigTransactionCreateCodec.encode(self, limits.max_bytes)?;
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
        ConfigTransactionCreateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ConfigTransactionExecute`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigTransactionExecute {}
/// Bounded codec for [`ConfigTransactionExecute`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigTransactionExecuteCodec;
impl Encoder<ConfigTransactionExecute> for ConfigTransactionExecuteCodec {
    fn encoded_len(&self, value: &ConfigTransactionExecute) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfigTransactionExecute,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[114, 146, 244, 189, 252, 140, 36, 40]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ConfigTransactionExecute> for ConfigTransactionExecuteCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigTransactionExecute, Error> {
        Constant(&[114, 146, 244, 189, 252, 140, 36, 40]).read(reader)?;
        let _ = reader;
        Ok(ConfigTransactionExecute {})
    }
}
impl ConfigTransactionExecute {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 6 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* multisig */ (true, Some(false), false),
                /* member */ (false, Some(true), false),
                /* proposal */ (true, Some(false), false),
                /* transaction */ (false, Some(false), false),
                /* rent_payer */ (true, Some(true), true),
                /* system_program */ (false, Some(false), true),
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
        let data = ConfigTransactionExecuteCodec.encode(self, limits.max_bytes)?;
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
        ConfigTransactionExecuteCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `VaultTransactionCreate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultTransactionCreate {
    /// The `args` wire field.
    pub args: VaultTransactionCreateArgs,
}
/// Bounded codec for [`VaultTransactionCreate`].
#[derive(Clone, Copy, Debug)]
pub struct VaultTransactionCreateCodec;
impl Encoder<VaultTransactionCreate> for VaultTransactionCreateCodec {
    fn encoded_len(&self, value: &VaultTransactionCreate) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(VaultTransactionCreateArgsCodec.encoded_len(&value.args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &VaultTransactionCreate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[48, 250, 78, 168, 208, 226, 218, 211]).write(&(), writer)?;
        VaultTransactionCreateArgsCodec.write(&value.args, writer)?;
        Ok(())
    }
}
impl Decoder<VaultTransactionCreate> for VaultTransactionCreateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VaultTransactionCreate, Error> {
        Constant(&[48, 250, 78, 168, 208, 226, 218, 211]).read(reader)?;
        Ok(VaultTransactionCreate {
            args: VaultTransactionCreateArgsCodec.read(reader)?,
        })
    }
}
impl VaultTransactionCreate {
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
                /* multisig */ (true, Some(false), false),
                /* transaction */ (true, Some(false), false),
                /* creator */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), false),
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
        let data = VaultTransactionCreateCodec.encode(self, limits.max_bytes)?;
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
        VaultTransactionCreateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TransactionBufferCreate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionBufferCreate {
    /// The `buffer_index` wire field.
    pub buffer_index: u8,
    /// The `vault_index` wire field.
    pub vault_index: u8,
    /// The `final_buffer_hash` wire field.
    pub final_buffer_hash: [u8; 32],
    /// The `final_buffer_size` wire field.
    pub final_buffer_size: u16,
    /// The `buffer` wire field.
    pub buffer: Vec<u8>,
}
/// Bounded codec for [`TransactionBufferCreate`].
#[derive(Clone, Copy, Debug)]
pub struct TransactionBufferCreateCodec;
impl Encoder<TransactionBufferCreate> for TransactionBufferCreateCodec {
    fn encoded_len(&self, value: &TransactionBufferCreate) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.buffer_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.vault_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.final_buffer_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.final_buffer_size)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.buffer.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransactionBufferCreate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[245, 201, 113, 108, 37, 63, 29, 89]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.buffer_index, writer)?;
        Number::<u8>::LE.write(&value.vault_index, writer)?;
        FixedBytes::<32>.write(&value.final_buffer_hash, writer)?;
        Number::<u16>::LE.write(&value.final_buffer_size, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.buffer.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<TransactionBufferCreate> for TransactionBufferCreateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransactionBufferCreate, Error> {
        Constant(&[245, 201, 113, 108, 37, 63, 29, 89]).read(reader)?;
        Ok(TransactionBufferCreate {
            buffer_index: Number::<u8>::LE.read(reader)?,
            vault_index: Number::<u8>::LE.read(reader)?,
            final_buffer_hash: FixedBytes::<32>.read(reader)?,
            final_buffer_size: Number::<u16>::LE.read(reader)?,
            buffer: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
        })
    }
}
impl TransactionBufferCreate {
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
                /* multisig */ (false, Some(false), false),
                /* transaction_buffer */ (true, Some(false), false),
                /* creator */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), false),
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
        let data = TransactionBufferCreateCodec.encode(self, limits.max_bytes)?;
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
        TransactionBufferCreateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TransactionBufferClose`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionBufferClose {}
/// Bounded codec for [`TransactionBufferClose`].
#[derive(Clone, Copy, Debug)]
pub struct TransactionBufferCloseCodec;
impl Encoder<TransactionBufferClose> for TransactionBufferCloseCodec {
    fn encoded_len(&self, value: &TransactionBufferClose) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &TransactionBufferClose, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[17, 182, 208, 228, 136, 24, 178, 102]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<TransactionBufferClose> for TransactionBufferCloseCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransactionBufferClose, Error> {
        Constant(&[17, 182, 208, 228, 136, 24, 178, 102]).read(reader)?;
        let _ = reader;
        Ok(TransactionBufferClose {})
    }
}
impl TransactionBufferClose {
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
                /* multisig */ (false, Some(false), false),
                /* transaction_buffer */ (true, Some(false), false),
                /* creator */ (false, Some(true), false),
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
        let data = TransactionBufferCloseCodec.encode(self, limits.max_bytes)?;
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
        TransactionBufferCloseCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TransactionBufferExtend`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionBufferExtend {
    /// The `buffer` wire field.
    pub buffer: Vec<u8>,
}
/// Bounded codec for [`TransactionBufferExtend`].
#[derive(Clone, Copy, Debug)]
pub struct TransactionBufferExtendCodec;
impl Encoder<TransactionBufferExtend> for TransactionBufferExtendCodec {
    fn encoded_len(&self, value: &TransactionBufferExtend) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.buffer.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransactionBufferExtend, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[230, 157, 67, 56, 5, 238, 245, 146]).write(&(), writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.buffer.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<TransactionBufferExtend> for TransactionBufferExtendCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransactionBufferExtend, Error> {
        Constant(&[230, 157, 67, 56, 5, 238, 245, 146]).read(reader)?;
        Ok(TransactionBufferExtend {
            buffer: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
        })
    }
}
impl TransactionBufferExtend {
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
                /* multisig */ (false, Some(false), false),
                /* transaction_buffer */ (true, Some(false), false),
                /* creator */ (false, Some(true), false),
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
        let data = TransactionBufferExtendCodec.encode(self, limits.max_bytes)?;
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
        TransactionBufferExtendCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `VaultTransactionCreateFromBuffer`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultTransactionCreateFromBuffer {
    /// The `args` wire field.
    pub args: VaultTransactionCreateArgs,
}
/// Bounded codec for [`VaultTransactionCreateFromBuffer`].
#[derive(Clone, Copy, Debug)]
pub struct VaultTransactionCreateFromBufferCodec;
impl Encoder<VaultTransactionCreateFromBuffer> for VaultTransactionCreateFromBufferCodec {
    fn encoded_len(&self, value: &VaultTransactionCreateFromBuffer) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(VaultTransactionCreateArgsCodec.encoded_len(&value.args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &VaultTransactionCreateFromBuffer,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[222, 54, 149, 68, 87, 246, 48, 231]).write(&(), writer)?;
        VaultTransactionCreateArgsCodec.write(&value.args, writer)?;
        Ok(())
    }
}
impl Decoder<VaultTransactionCreateFromBuffer> for VaultTransactionCreateFromBufferCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VaultTransactionCreateFromBuffer, Error> {
        Constant(&[222, 54, 149, 68, 87, 246, 48, 231]).read(reader)?;
        Ok(VaultTransactionCreateFromBuffer {
            args: VaultTransactionCreateArgsCodec.read(reader)?,
        })
    }
}
impl VaultTransactionCreateFromBuffer {
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
                /* vault_transaction_create_multisig */ (true, Some(false), false),
                /* vault_transaction_create_transaction */ (true, Some(false), false),
                /* vault_transaction_create_creator */ (false, Some(true), false),
                /* vault_transaction_create_rent_payer */ (true, Some(true), false),
                /* vault_transaction_create_system_program */ (false, Some(false), false),
                /* transaction_buffer */ (true, Some(false), false),
                /* creator */ (true, Some(true), false),
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
        let data = VaultTransactionCreateFromBufferCodec.encode(self, limits.max_bytes)?;
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
        VaultTransactionCreateFromBufferCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `VaultTransactionExecute`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultTransactionExecute {}
/// Bounded codec for [`VaultTransactionExecute`].
#[derive(Clone, Copy, Debug)]
pub struct VaultTransactionExecuteCodec;
impl Encoder<VaultTransactionExecute> for VaultTransactionExecuteCodec {
    fn encoded_len(&self, value: &VaultTransactionExecute) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &VaultTransactionExecute, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[194, 8, 161, 87, 153, 164, 25, 171]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<VaultTransactionExecute> for VaultTransactionExecuteCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VaultTransactionExecute, Error> {
        Constant(&[194, 8, 161, 87, 153, 164, 25, 171]).read(reader)?;
        let _ = reader;
        Ok(VaultTransactionExecute {})
    }
}
impl VaultTransactionExecute {
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
                /* multisig */ (false, Some(false), false),
                /* proposal */ (true, Some(false), false),
                /* transaction */ (false, Some(false), false),
                /* member */ (false, Some(true), false),
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
        let data = VaultTransactionExecuteCodec.encode(self, limits.max_bytes)?;
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
        VaultTransactionExecuteCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `BatchCreate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchCreate {
    /// The `vault_index` wire field.
    pub vault_index: u8,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`BatchCreate`].
#[derive(Clone, Copy, Debug)]
pub struct BatchCreateCodec;
impl Encoder<BatchCreate> for BatchCreateCodec {
    fn encoded_len(&self, value: &BatchCreate) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.vault_index)?)
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &BatchCreate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[194, 142, 141, 17, 55, 185, 20, 248]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.vault_index, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<BatchCreate> for BatchCreateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BatchCreate, Error> {
        Constant(&[194, 142, 141, 17, 55, 185, 20, 248]).read(reader)?;
        Ok(BatchCreate {
            vault_index: Number::<u8>::LE.read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl BatchCreate {
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
                /* multisig */ (true, Some(false), false),
                /* batch */ (true, Some(false), false),
                /* creator */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), false),
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
        let data = BatchCreateCodec.encode(self, limits.max_bytes)?;
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
        BatchCreateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `BatchAddTransaction`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAddTransaction {
    /// The `ephemeral_signers` wire field.
    pub ephemeral_signers: u8,
    /// The `transaction_message` wire field.
    pub transaction_message: Vec<u8>,
}
/// Bounded codec for [`BatchAddTransaction`].
#[derive(Clone, Copy, Debug)]
pub struct BatchAddTransactionCodec;
impl Encoder<BatchAddTransaction> for BatchAddTransactionCodec {
    fn encoded_len(&self, value: &BatchAddTransaction) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.ephemeral_signers)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }
                .encoded_len(value.transaction_message.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &BatchAddTransaction, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[89, 100, 224, 18, 69, 70, 54, 76]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.ephemeral_signers, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }
        .write(value.transaction_message.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<BatchAddTransaction> for BatchAddTransactionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BatchAddTransaction, Error> {
        Constant(&[89, 100, 224, 18, 69, 70, 54, 76]).read(reader)?;
        Ok(BatchAddTransaction {
            ephemeral_signers: Number::<u8>::LE.read(reader)?,
            transaction_message: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }
            .read(reader)?,
        })
    }
}
impl BatchAddTransaction {
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
                /* multisig */ (false, Some(false), false),
                /* proposal */ (false, Some(false), false),
                /* batch */ (true, Some(false), false),
                /* transaction */ (true, Some(false), false),
                /* member */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), false),
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
        let data = BatchAddTransactionCodec.encode(self, limits.max_bytes)?;
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
        BatchAddTransactionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `BatchExecuteTransaction`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchExecuteTransaction {}
/// Bounded codec for [`BatchExecuteTransaction`].
#[derive(Clone, Copy, Debug)]
pub struct BatchExecuteTransactionCodec;
impl Encoder<BatchExecuteTransaction> for BatchExecuteTransactionCodec {
    fn encoded_len(&self, value: &BatchExecuteTransaction) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &BatchExecuteTransaction, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[172, 44, 179, 152, 21, 127, 234, 180]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<BatchExecuteTransaction> for BatchExecuteTransactionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BatchExecuteTransaction, Error> {
        Constant(&[172, 44, 179, 152, 21, 127, 234, 180]).read(reader)?;
        let _ = reader;
        Ok(BatchExecuteTransaction {})
    }
}
impl BatchExecuteTransaction {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 5 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* multisig */ (false, Some(false), false),
                /* member */ (false, Some(true), false),
                /* proposal */ (true, Some(false), false),
                /* batch */ (true, Some(false), false),
                /* transaction */ (false, Some(false), false),
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
        let data = BatchExecuteTransactionCodec.encode(self, limits.max_bytes)?;
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
        BatchExecuteTransactionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ProposalCreate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalCreate {
    /// The `transaction_index` wire field.
    pub transaction_index: u64,
    /// The `draft` wire field.
    pub draft: bool,
}
/// Bounded codec for [`ProposalCreate`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalCreateCodec;
impl Encoder<ProposalCreate> for ProposalCreateCodec {
    fn encoded_len(&self, value: &ProposalCreate) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.transaction_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.draft)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ProposalCreate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[220, 60, 73, 224, 30, 108, 79, 159]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.transaction_index, writer)?;
        Bool.write(&value.draft, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalCreate> for ProposalCreateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalCreate, Error> {
        Constant(&[220, 60, 73, 224, 30, 108, 79, 159]).read(reader)?;
        Ok(ProposalCreate {
            transaction_index: Number::<u64>::LE.read(reader)?,
            draft: Bool.read(reader)?,
        })
    }
}
impl ProposalCreate {
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
                /* multisig */ (false, Some(false), false),
                /* proposal */ (true, Some(false), false),
                /* creator */ (false, Some(true), false),
                /* rent_payer */ (true, Some(true), false),
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
        let data = ProposalCreateCodec.encode(self, limits.max_bytes)?;
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
        ProposalCreateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ProposalActivate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalActivate {}
/// Bounded codec for [`ProposalActivate`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalActivateCodec;
impl Encoder<ProposalActivate> for ProposalActivateCodec {
    fn encoded_len(&self, value: &ProposalActivate) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &ProposalActivate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[11, 34, 92, 248, 154, 27, 51, 106]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ProposalActivate> for ProposalActivateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalActivate, Error> {
        Constant(&[11, 34, 92, 248, 154, 27, 51, 106]).read(reader)?;
        let _ = reader;
        Ok(ProposalActivate {})
    }
}
impl ProposalActivate {
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
                /* multisig */ (false, Some(false), false),
                /* member */ (true, Some(true), false),
                /* proposal */ (true, Some(false), false),
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
        let data = ProposalActivateCodec.encode(self, limits.max_bytes)?;
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
        ProposalActivateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ProposalApprove`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalApprove {
    /// The `args` wire field.
    pub args: ProposalVoteArgs,
}
/// Bounded codec for [`ProposalApprove`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalApproveCodec;
impl Encoder<ProposalApprove> for ProposalApproveCodec {
    fn encoded_len(&self, value: &ProposalApprove) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(ProposalVoteArgsCodec.encoded_len(&value.args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ProposalApprove, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[144, 37, 164, 136, 188, 216, 42, 248]).write(&(), writer)?;
        ProposalVoteArgsCodec.write(&value.args, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalApprove> for ProposalApproveCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalApprove, Error> {
        Constant(&[144, 37, 164, 136, 188, 216, 42, 248]).read(reader)?;
        Ok(ProposalApprove {
            args: ProposalVoteArgsCodec.read(reader)?,
        })
    }
}
impl ProposalApprove {
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
                /* multisig */ (false, Some(false), false),
                /* member */ (true, Some(true), false),
                /* proposal */ (true, Some(false), false),
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
        let data = ProposalApproveCodec.encode(self, limits.max_bytes)?;
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
        ProposalApproveCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ProposalReject`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalReject {
    /// The `args` wire field.
    pub args: ProposalVoteArgs,
}
/// Bounded codec for [`ProposalReject`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalRejectCodec;
impl Encoder<ProposalReject> for ProposalRejectCodec {
    fn encoded_len(&self, value: &ProposalReject) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(ProposalVoteArgsCodec.encoded_len(&value.args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ProposalReject, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[243, 62, 134, 156, 230, 106, 246, 135]).write(&(), writer)?;
        ProposalVoteArgsCodec.write(&value.args, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalReject> for ProposalRejectCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalReject, Error> {
        Constant(&[243, 62, 134, 156, 230, 106, 246, 135]).read(reader)?;
        Ok(ProposalReject {
            args: ProposalVoteArgsCodec.read(reader)?,
        })
    }
}
impl ProposalReject {
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
                /* multisig */ (false, Some(false), false),
                /* member */ (true, Some(true), false),
                /* proposal */ (true, Some(false), false),
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
        let data = ProposalRejectCodec.encode(self, limits.max_bytes)?;
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
        ProposalRejectCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ProposalCancel`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalCancel {
    /// The `args` wire field.
    pub args: ProposalVoteArgs,
}
/// Bounded codec for [`ProposalCancel`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalCancelCodec;
impl Encoder<ProposalCancel> for ProposalCancelCodec {
    fn encoded_len(&self, value: &ProposalCancel) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(ProposalVoteArgsCodec.encoded_len(&value.args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ProposalCancel, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[27, 42, 127, 237, 38, 163, 84, 203]).write(&(), writer)?;
        ProposalVoteArgsCodec.write(&value.args, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalCancel> for ProposalCancelCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalCancel, Error> {
        Constant(&[27, 42, 127, 237, 38, 163, 84, 203]).read(reader)?;
        Ok(ProposalCancel {
            args: ProposalVoteArgsCodec.read(reader)?,
        })
    }
}
impl ProposalCancel {
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
                /* multisig */ (false, Some(false), false),
                /* member */ (true, Some(true), false),
                /* proposal */ (true, Some(false), false),
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
        let data = ProposalCancelCodec.encode(self, limits.max_bytes)?;
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
        ProposalCancelCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ProposalCancelV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalCancelV2 {
    /// The `args` wire field.
    pub args: ProposalVoteArgs,
}
/// Bounded codec for [`ProposalCancelV2`].
#[derive(Clone, Copy, Debug)]
pub struct ProposalCancelV2Codec;
impl Encoder<ProposalCancelV2> for ProposalCancelV2Codec {
    fn encoded_len(&self, value: &ProposalCancelV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(ProposalVoteArgsCodec.encoded_len(&value.args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ProposalCancelV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[205, 41, 194, 61, 220, 139, 16, 247]).write(&(), writer)?;
        ProposalVoteArgsCodec.write(&value.args, writer)?;
        Ok(())
    }
}
impl Decoder<ProposalCancelV2> for ProposalCancelV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProposalCancelV2, Error> {
        Constant(&[205, 41, 194, 61, 220, 139, 16, 247]).read(reader)?;
        Ok(ProposalCancelV2 {
            args: ProposalVoteArgsCodec.read(reader)?,
        })
    }
}
impl ProposalCancelV2 {
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
                /* multisig */ (false, Some(false), false),
                /* member */ (true, Some(true), false),
                /* proposal */ (true, Some(false), false),
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
        let data = ProposalCancelV2Codec.encode(self, limits.max_bytes)?;
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
        ProposalCancelV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SpendingLimitUse`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpendingLimitUse {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
    /// The `memo` wire field.
    pub memo: Option<String>,
}
/// Bounded codec for [`SpendingLimitUse`].
#[derive(Clone, Copy, Debug)]
pub struct SpendingLimitUseCodec;
impl Encoder<SpendingLimitUse> for SpendingLimitUseCodec {
    fn encoded_len(&self, value: &SpendingLimitUse) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
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
                .encoded_len(&value.memo)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SpendingLimitUse, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[16, 57, 130, 127, 193, 20, 155, 134]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.memo, writer)?;
        Ok(())
    }
}
impl Decoder<SpendingLimitUse> for SpendingLimitUseCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SpendingLimitUse, Error> {
        Constant(&[16, 57, 130, 127, 193, 20, 155, 134]).read(reader)?;
        Ok(SpendingLimitUse {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
            memo: OptionCodec::u8(OwnedString(ByteSizePrefix {
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
impl SpendingLimitUse {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 10 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* multisig */ (false, Some(false), false),
                /* member */ (false, Some(true), false),
                /* spending_limit */ (true, Some(false), false),
                /* vault */ (true, Some(false), false),
                /* destination */ (true, Some(false), false),
                /* system_program */ (false, Some(false), true),
                /* mint */ (false, Some(false), true),
                /* vault_token_account */ (true, Some(false), true),
                /* destination_token_account */ (true, Some(false), true),
                /* token_program */ (false, Some(false), true),
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
        let data = SpendingLimitUseCodec.encode(self, limits.max_bytes)?;
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
        SpendingLimitUseCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ConfigTransactionAccountsClose`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigTransactionAccountsClose {}
/// Bounded codec for [`ConfigTransactionAccountsClose`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigTransactionAccountsCloseCodec;
impl Encoder<ConfigTransactionAccountsClose> for ConfigTransactionAccountsCloseCodec {
    fn encoded_len(&self, value: &ConfigTransactionAccountsClose) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfigTransactionAccountsClose,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[80, 203, 84, 53, 151, 112, 187, 186]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ConfigTransactionAccountsClose> for ConfigTransactionAccountsCloseCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigTransactionAccountsClose, Error> {
        Constant(&[80, 203, 84, 53, 151, 112, 187, 186]).read(reader)?;
        let _ = reader;
        Ok(ConfigTransactionAccountsClose {})
    }
}
impl ConfigTransactionAccountsClose {
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
                /* multisig */ (false, Some(false), false),
                /* proposal */ (true, Some(false), false),
                /* transaction */ (true, Some(false), false),
                /* rent_collector */ (true, Some(false), false),
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
        let data = ConfigTransactionAccountsCloseCodec.encode(self, limits.max_bytes)?;
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
        ConfigTransactionAccountsCloseCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `VaultTransactionAccountsClose`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultTransactionAccountsClose {}
/// Bounded codec for [`VaultTransactionAccountsClose`].
#[derive(Clone, Copy, Debug)]
pub struct VaultTransactionAccountsCloseCodec;
impl Encoder<VaultTransactionAccountsClose> for VaultTransactionAccountsCloseCodec {
    fn encoded_len(&self, value: &VaultTransactionAccountsClose) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &VaultTransactionAccountsClose,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[196, 71, 187, 176, 2, 35, 170, 165]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<VaultTransactionAccountsClose> for VaultTransactionAccountsCloseCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VaultTransactionAccountsClose, Error> {
        Constant(&[196, 71, 187, 176, 2, 35, 170, 165]).read(reader)?;
        let _ = reader;
        Ok(VaultTransactionAccountsClose {})
    }
}
impl VaultTransactionAccountsClose {
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
                /* multisig */ (false, Some(false), false),
                /* proposal */ (true, Some(false), false),
                /* transaction */ (true, Some(false), false),
                /* rent_collector */ (true, Some(false), false),
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
        let data = VaultTransactionAccountsCloseCodec.encode(self, limits.max_bytes)?;
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
        VaultTransactionAccountsCloseCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `VaultBatchTransactionAccountClose`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultBatchTransactionAccountClose {}
/// Bounded codec for [`VaultBatchTransactionAccountClose`].
#[derive(Clone, Copy, Debug)]
pub struct VaultBatchTransactionAccountCloseCodec;
impl Encoder<VaultBatchTransactionAccountClose> for VaultBatchTransactionAccountCloseCodec {
    fn encoded_len(&self, value: &VaultBatchTransactionAccountClose) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &VaultBatchTransactionAccountClose,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[134, 18, 19, 106, 129, 68, 97, 247]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<VaultBatchTransactionAccountClose> for VaultBatchTransactionAccountCloseCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VaultBatchTransactionAccountClose, Error> {
        Constant(&[134, 18, 19, 106, 129, 68, 97, 247]).read(reader)?;
        let _ = reader;
        Ok(VaultBatchTransactionAccountClose {})
    }
}
impl VaultBatchTransactionAccountClose {
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
                /* multisig */ (false, Some(false), false),
                /* proposal */ (false, Some(false), false),
                /* batch */ (true, Some(false), false),
                /* transaction */ (true, Some(false), false),
                /* rent_collector */ (true, Some(false), false),
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
        let data = VaultBatchTransactionAccountCloseCodec.encode(self, limits.max_bytes)?;
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
        VaultBatchTransactionAccountCloseCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `BatchAccountsClose`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAccountsClose {}
/// Bounded codec for [`BatchAccountsClose`].
#[derive(Clone, Copy, Debug)]
pub struct BatchAccountsCloseCodec;
impl Encoder<BatchAccountsClose> for BatchAccountsCloseCodec {
    fn encoded_len(&self, value: &BatchAccountsClose) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &BatchAccountsClose, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[218, 196, 7, 175, 130, 102, 11, 255]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<BatchAccountsClose> for BatchAccountsCloseCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BatchAccountsClose, Error> {
        Constant(&[218, 196, 7, 175, 130, 102, 11, 255]).read(reader)?;
        let _ = reader;
        Ok(BatchAccountsClose {})
    }
}
impl BatchAccountsClose {
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
                /* multisig */ (false, Some(false), false),
                /* proposal */ (true, Some(false), false),
                /* batch */ (true, Some(false), false),
                /* rent_collector */ (true, Some(false), false),
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
        let data = BatchAccountsCloseCodec.encode(self, limits.max_bytes)?;
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
        BatchAccountsCloseCodec.decode(&instruction.data, limits)
    }
}
/// Program error codes. Unknown numeric codes remain lossless.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SquadsMultisigError {
    /// The `DuplicateMember` program error.
    DuplicateMember,
    /// The `EmptyMembers` program error.
    EmptyMembers,
    /// The `TooManyMembers` program error.
    TooManyMembers,
    /// The `InvalidThreshold` program error.
    InvalidThreshold,
    /// The `Unauthorized` program error.
    Unauthorized,
    /// The `NotAMember` program error.
    NotAMember,
    /// The `InvalidTransactionMessage` program error.
    InvalidTransactionMessage,
    /// The `StaleProposal` program error.
    StaleProposal,
    /// The `InvalidProposalStatus` program error.
    InvalidProposalStatus,
    /// The `InvalidTransactionIndex` program error.
    InvalidTransactionIndex,
    /// The `AlreadyApproved` program error.
    AlreadyApproved,
    /// The `AlreadyRejected` program error.
    AlreadyRejected,
    /// The `AlreadyCancelled` program error.
    AlreadyCancelled,
    /// The `InvalidNumberOfAccounts` program error.
    InvalidNumberOfAccounts,
    /// The `InvalidAccount` program error.
    InvalidAccount,
    /// The `RemoveLastMember` program error.
    RemoveLastMember,
    /// The `NoVoters` program error.
    NoVoters,
    /// The `NoProposers` program error.
    NoProposers,
    /// The `NoExecutors` program error.
    NoExecutors,
    /// The `InvalidStaleTransactionIndex` program error.
    InvalidStaleTransactionIndex,
    /// The `NotSupportedForControlled` program error.
    NotSupportedForControlled,
    /// The `TimeLockNotReleased` program error.
    TimeLockNotReleased,
    /// The `NoActions` program error.
    NoActions,
    /// The `MissingAccount` program error.
    MissingAccount,
    /// The `InvalidMint` program error.
    InvalidMint,
    /// The `InvalidDestination` program error.
    InvalidDestination,
    /// The `SpendingLimitExceeded` program error.
    SpendingLimitExceeded,
    /// The `DecimalsMismatch` program error.
    DecimalsMismatch,
    /// The `UnknownPermission` program error.
    UnknownPermission,
    /// The `ProtectedAccount` program error.
    ProtectedAccount,
    /// The `TimeLockExceedsMaxAllowed` program error.
    TimeLockExceedsMaxAllowed,
    /// The `IllegalAccountOwner` program error.
    IllegalAccountOwner,
    /// The `RentReclamationDisabled` program error.
    RentReclamationDisabled,
    /// The `InvalidRentCollector` program error.
    InvalidRentCollector,
    /// The `ProposalForAnotherMultisig` program error.
    ProposalForAnotherMultisig,
    /// The `TransactionForAnotherMultisig` program error.
    TransactionForAnotherMultisig,
    /// The `TransactionNotMatchingProposal` program error.
    TransactionNotMatchingProposal,
    /// The `TransactionNotLastInBatch` program error.
    TransactionNotLastInBatch,
    /// The `BatchNotEmpty` program error.
    BatchNotEmpty,
    /// The `SpendingLimitInvalidAmount` program error.
    SpendingLimitInvalidAmount,
    /// The `InvalidInstructionArgs` program error.
    InvalidInstructionArgs,
    /// The `FinalBufferHashMismatch` program error.
    FinalBufferHashMismatch,
    /// The `FinalBufferSizeExceeded` program error.
    FinalBufferSizeExceeded,
    /// The `FinalBufferSizeMismatch` program error.
    FinalBufferSizeMismatch,
    /// The `MultisigCreateDeprecated` program error.
    MultisigCreateDeprecated,
    /// An unrecognized program error code.
    Unknown(u32),
}
impl SquadsMultisigError {
    /// Decodes known or unknown numeric program error codes.
    pub const fn from_code(code: u32) -> Self {
        match code {
            PROGRAM_ERROR_6000 => Self::DuplicateMember,
            PROGRAM_ERROR_6001 => Self::EmptyMembers,
            PROGRAM_ERROR_6002 => Self::TooManyMembers,
            PROGRAM_ERROR_6003 => Self::InvalidThreshold,
            PROGRAM_ERROR_6004 => Self::Unauthorized,
            PROGRAM_ERROR_6005 => Self::NotAMember,
            PROGRAM_ERROR_6006 => Self::InvalidTransactionMessage,
            PROGRAM_ERROR_6007 => Self::StaleProposal,
            PROGRAM_ERROR_6008 => Self::InvalidProposalStatus,
            PROGRAM_ERROR_6009 => Self::InvalidTransactionIndex,
            PROGRAM_ERROR_6010 => Self::AlreadyApproved,
            PROGRAM_ERROR_6011 => Self::AlreadyRejected,
            PROGRAM_ERROR_6012 => Self::AlreadyCancelled,
            PROGRAM_ERROR_6013 => Self::InvalidNumberOfAccounts,
            PROGRAM_ERROR_6014 => Self::InvalidAccount,
            PROGRAM_ERROR_6015 => Self::RemoveLastMember,
            PROGRAM_ERROR_6016 => Self::NoVoters,
            PROGRAM_ERROR_6017 => Self::NoProposers,
            PROGRAM_ERROR_6018 => Self::NoExecutors,
            PROGRAM_ERROR_6019 => Self::InvalidStaleTransactionIndex,
            PROGRAM_ERROR_6020 => Self::NotSupportedForControlled,
            PROGRAM_ERROR_6021 => Self::TimeLockNotReleased,
            PROGRAM_ERROR_6022 => Self::NoActions,
            PROGRAM_ERROR_6023 => Self::MissingAccount,
            PROGRAM_ERROR_6024 => Self::InvalidMint,
            PROGRAM_ERROR_6025 => Self::InvalidDestination,
            PROGRAM_ERROR_6026 => Self::SpendingLimitExceeded,
            PROGRAM_ERROR_6027 => Self::DecimalsMismatch,
            PROGRAM_ERROR_6028 => Self::UnknownPermission,
            PROGRAM_ERROR_6029 => Self::ProtectedAccount,
            PROGRAM_ERROR_6030 => Self::TimeLockExceedsMaxAllowed,
            PROGRAM_ERROR_6031 => Self::IllegalAccountOwner,
            PROGRAM_ERROR_6032 => Self::RentReclamationDisabled,
            PROGRAM_ERROR_6033 => Self::InvalidRentCollector,
            PROGRAM_ERROR_6034 => Self::ProposalForAnotherMultisig,
            PROGRAM_ERROR_6035 => Self::TransactionForAnotherMultisig,
            PROGRAM_ERROR_6036 => Self::TransactionNotMatchingProposal,
            PROGRAM_ERROR_6037 => Self::TransactionNotLastInBatch,
            PROGRAM_ERROR_6038 => Self::BatchNotEmpty,
            PROGRAM_ERROR_6039 => Self::SpendingLimitInvalidAmount,
            PROGRAM_ERROR_6040 => Self::InvalidInstructionArgs,
            PROGRAM_ERROR_6041 => Self::FinalBufferHashMismatch,
            PROGRAM_ERROR_6042 => Self::FinalBufferSizeExceeded,
            PROGRAM_ERROR_6043 => Self::FinalBufferSizeMismatch,
            PROGRAM_ERROR_6044 => Self::MultisigCreateDeprecated,
            code => Self::Unknown(code),
        }
    }
    /// Returns the exact numeric code.
    pub const fn code(self) -> u32 {
        match self {
            Self::DuplicateMember => PROGRAM_ERROR_6000,
            Self::EmptyMembers => PROGRAM_ERROR_6001,
            Self::TooManyMembers => PROGRAM_ERROR_6002,
            Self::InvalidThreshold => PROGRAM_ERROR_6003,
            Self::Unauthorized => PROGRAM_ERROR_6004,
            Self::NotAMember => PROGRAM_ERROR_6005,
            Self::InvalidTransactionMessage => PROGRAM_ERROR_6006,
            Self::StaleProposal => PROGRAM_ERROR_6007,
            Self::InvalidProposalStatus => PROGRAM_ERROR_6008,
            Self::InvalidTransactionIndex => PROGRAM_ERROR_6009,
            Self::AlreadyApproved => PROGRAM_ERROR_6010,
            Self::AlreadyRejected => PROGRAM_ERROR_6011,
            Self::AlreadyCancelled => PROGRAM_ERROR_6012,
            Self::InvalidNumberOfAccounts => PROGRAM_ERROR_6013,
            Self::InvalidAccount => PROGRAM_ERROR_6014,
            Self::RemoveLastMember => PROGRAM_ERROR_6015,
            Self::NoVoters => PROGRAM_ERROR_6016,
            Self::NoProposers => PROGRAM_ERROR_6017,
            Self::NoExecutors => PROGRAM_ERROR_6018,
            Self::InvalidStaleTransactionIndex => PROGRAM_ERROR_6019,
            Self::NotSupportedForControlled => PROGRAM_ERROR_6020,
            Self::TimeLockNotReleased => PROGRAM_ERROR_6021,
            Self::NoActions => PROGRAM_ERROR_6022,
            Self::MissingAccount => PROGRAM_ERROR_6023,
            Self::InvalidMint => PROGRAM_ERROR_6024,
            Self::InvalidDestination => PROGRAM_ERROR_6025,
            Self::SpendingLimitExceeded => PROGRAM_ERROR_6026,
            Self::DecimalsMismatch => PROGRAM_ERROR_6027,
            Self::UnknownPermission => PROGRAM_ERROR_6028,
            Self::ProtectedAccount => PROGRAM_ERROR_6029,
            Self::TimeLockExceedsMaxAllowed => PROGRAM_ERROR_6030,
            Self::IllegalAccountOwner => PROGRAM_ERROR_6031,
            Self::RentReclamationDisabled => PROGRAM_ERROR_6032,
            Self::InvalidRentCollector => PROGRAM_ERROR_6033,
            Self::ProposalForAnotherMultisig => PROGRAM_ERROR_6034,
            Self::TransactionForAnotherMultisig => PROGRAM_ERROR_6035,
            Self::TransactionNotMatchingProposal => PROGRAM_ERROR_6036,
            Self::TransactionNotLastInBatch => PROGRAM_ERROR_6037,
            Self::BatchNotEmpty => PROGRAM_ERROR_6038,
            Self::SpendingLimitInvalidAmount => PROGRAM_ERROR_6039,
            Self::InvalidInstructionArgs => PROGRAM_ERROR_6040,
            Self::FinalBufferHashMismatch => PROGRAM_ERROR_6041,
            Self::FinalBufferSizeExceeded => PROGRAM_ERROR_6042,
            Self::FinalBufferSizeMismatch => PROGRAM_ERROR_6043,
            Self::MultisigCreateDeprecated => PROGRAM_ERROR_6044,
            Self::Unknown(code) => code,
        }
    }
    /// Returns the source message for known errors.
    pub const fn message(self) -> Option<&'static str> {
        match self {
            Self::DuplicateMember => Some("Found multiple members with the same pubkey"),
            Self::EmptyMembers => Some("Members array is empty"),
            Self::TooManyMembers => Some("Too many members, can be up to 65535"),
            Self::InvalidThreshold => Some(
                "Invalid threshold, must be between 1 and number of members with Vote permission",
            ),
            Self::Unauthorized => Some("Attempted to perform an unauthorized action"),
            Self::NotAMember => Some("Provided pubkey is not a member of multisig"),
            Self::InvalidTransactionMessage => Some("TransactionMessage is malformed."),
            Self::StaleProposal => Some("Proposal is stale"),
            Self::InvalidProposalStatus => Some("Invalid proposal status"),
            Self::InvalidTransactionIndex => Some("Invalid transaction index"),
            Self::AlreadyApproved => Some("Member already approved the transaction"),
            Self::AlreadyRejected => Some("Member already rejected the transaction"),
            Self::AlreadyCancelled => Some("Member already cancelled the transaction"),
            Self::InvalidNumberOfAccounts => Some("Wrong number of accounts provided"),
            Self::InvalidAccount => Some("Invalid account provided"),
            Self::RemoveLastMember => Some("Cannot remove last member"),
            Self::NoVoters => Some("Members don't include any voters"),
            Self::NoProposers => Some("Members don't include any proposers"),
            Self::NoExecutors => Some("Members don't include any executors"),
            Self::InvalidStaleTransactionIndex => {
                Some("`stale_transaction_index` must be <= `transaction_index`")
            }
            Self::NotSupportedForControlled => {
                Some("Instruction not supported for controlled multisig")
            }
            Self::TimeLockNotReleased => Some("Proposal time lock has not been released"),
            Self::NoActions => Some("Config transaction must have at least one action"),
            Self::MissingAccount => Some("Missing account"),
            Self::InvalidMint => Some("Invalid mint"),
            Self::InvalidDestination => Some("Invalid destination"),
            Self::SpendingLimitExceeded => Some("Spending limit exceeded"),
            Self::DecimalsMismatch => Some("Decimals don't match the mint"),
            Self::UnknownPermission => Some("Member has unknown permission"),
            Self::ProtectedAccount => {
                Some("Account is protected, it cannot be passed into a CPI as writable")
            }
            Self::TimeLockExceedsMaxAllowed => {
                Some("Time lock exceeds the maximum allowed (90 days)")
            }
            Self::IllegalAccountOwner => Some("Account is not owned by Multisig program"),
            Self::RentReclamationDisabled => Some("Rent reclamation is disabled for this multisig"),
            Self::InvalidRentCollector => Some("Invalid rent collector address"),
            Self::ProposalForAnotherMultisig => Some("Proposal is for another multisig"),
            Self::TransactionForAnotherMultisig => Some("Transaction is for another multisig"),
            Self::TransactionNotMatchingProposal => Some("Transaction doesn't match proposal"),
            Self::TransactionNotLastInBatch => Some("Transaction is not last in batch"),
            Self::BatchNotEmpty => Some("Batch is not empty"),
            Self::SpendingLimitInvalidAmount => Some("Invalid SpendingLimit amount"),
            Self::InvalidInstructionArgs => Some("Invalid Instruction Arguments"),
            Self::FinalBufferHashMismatch => {
                Some("Final message buffer hash doesnt match the expected hash")
            }
            Self::FinalBufferSizeExceeded => Some("Final buffer size cannot exceed 4000 bytes"),
            Self::FinalBufferSizeMismatch => Some("Final buffer size mismatch"),
            Self::MultisigCreateDeprecated => {
                Some("multisig_create has been deprecated. Use multisig_create_v2 instead.")
            }
            Self::Unknown(_) => None,
        }
    }
}
/// All selected instruction data variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SquadsMultisigInstructionData {
    /// Data for tag 184,188,198,195,205,124,117,216.
    ProgramConfigInit(ProgramConfigInit),
    /// Data for tag 238,242,36,181,32,143,216,75.
    ProgramConfigSetAuthority(ProgramConfigSetAuthority),
    /// Data for tag 101,160,249,63,154,215,153,13.
    ProgramConfigSetMultisigCreationFee(ProgramConfigSetMultisigCreationFee),
    /// Data for tag 111,46,243,117,144,188,162,107.
    ProgramConfigSetTreasury(ProgramConfigSetTreasury),
    /// Data for tag 122,77,80,159,84,88,90,197.
    MultisigCreate(MultisigCreate),
    /// Data for tag 50,221,199,93,40,245,139,233.
    MultisigCreateV2(MultisigCreateV2),
    /// Data for tag 1,219,215,108,184,229,214,8.
    MultisigAddMember(MultisigAddMember),
    /// Data for tag 217,117,177,210,182,145,218,72.
    MultisigRemoveMember(MultisigRemoveMember),
    /// Data for tag 148,154,121,77,212,254,155,72.
    MultisigSetTimeLock(MultisigSetTimeLock),
    /// Data for tag 141,42,15,126,169,92,62,181.
    MultisigChangeThreshold(MultisigChangeThreshold),
    /// Data for tag 143,93,199,143,92,169,193,232.
    MultisigSetConfigAuthority(MultisigSetConfigAuthority),
    /// Data for tag 48,204,65,57,210,70,156,74.
    MultisigSetRentCollector(MultisigSetRentCollector),
    /// Data for tag 11,242,159,42,86,197,89,115.
    MultisigAddSpendingLimit(MultisigAddSpendingLimit),
    /// Data for tag 228,198,136,111,123,4,178,113.
    MultisigRemoveSpendingLimit(MultisigRemoveSpendingLimit),
    /// Data for tag 155,236,87,228,137,75,81,39.
    ConfigTransactionCreate(ConfigTransactionCreate),
    /// Data for tag 114,146,244,189,252,140,36,40.
    ConfigTransactionExecute(ConfigTransactionExecute),
    /// Data for tag 48,250,78,168,208,226,218,211.
    VaultTransactionCreate(VaultTransactionCreate),
    /// Data for tag 245,201,113,108,37,63,29,89.
    TransactionBufferCreate(TransactionBufferCreate),
    /// Data for tag 17,182,208,228,136,24,178,102.
    TransactionBufferClose(TransactionBufferClose),
    /// Data for tag 230,157,67,56,5,238,245,146.
    TransactionBufferExtend(TransactionBufferExtend),
    /// Data for tag 222,54,149,68,87,246,48,231.
    VaultTransactionCreateFromBuffer(VaultTransactionCreateFromBuffer),
    /// Data for tag 194,8,161,87,153,164,25,171.
    VaultTransactionExecute(VaultTransactionExecute),
    /// Data for tag 194,142,141,17,55,185,20,248.
    BatchCreate(BatchCreate),
    /// Data for tag 89,100,224,18,69,70,54,76.
    BatchAddTransaction(BatchAddTransaction),
    /// Data for tag 172,44,179,152,21,127,234,180.
    BatchExecuteTransaction(BatchExecuteTransaction),
    /// Data for tag 220,60,73,224,30,108,79,159.
    ProposalCreate(ProposalCreate),
    /// Data for tag 11,34,92,248,154,27,51,106.
    ProposalActivate(ProposalActivate),
    /// Data for tag 144,37,164,136,188,216,42,248.
    ProposalApprove(ProposalApprove),
    /// Data for tag 243,62,134,156,230,106,246,135.
    ProposalReject(ProposalReject),
    /// Data for tag 27,42,127,237,38,163,84,203.
    ProposalCancel(ProposalCancel),
    /// Data for tag 205,41,194,61,220,139,16,247.
    ProposalCancelV2(ProposalCancelV2),
    /// Data for tag 16,57,130,127,193,20,155,134.
    SpendingLimitUse(SpendingLimitUse),
    /// Data for tag 80,203,84,53,151,112,187,186.
    ConfigTransactionAccountsClose(ConfigTransactionAccountsClose),
    /// Data for tag 196,71,187,176,2,35,170,165.
    VaultTransactionAccountsClose(VaultTransactionAccountsClose),
    /// Data for tag 134,18,19,106,129,68,97,247.
    VaultBatchTransactionAccountClose(VaultBatchTransactionAccountClose),
    /// Data for tag 218,196,7,175,130,102,11,255.
    BatchAccountsClose(BatchAccountsClose),
}
/// Strict bounded instruction dispatch codec.
#[derive(Clone, Copy, Debug)]
pub struct SquadsMultisigInstructionDataCodec;
impl Encoder<SquadsMultisigInstructionData> for SquadsMultisigInstructionDataCodec {
    fn encoded_len(&self, value: &SquadsMultisigInstructionData) -> Result<usize, Error> {
        Self::encoded_len_0(value)
    }
    fn write(
        &self,
        value: &SquadsMultisigInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Self::write_0(value, writer)
    }
}
impl Decoder<SquadsMultisigInstructionData> for SquadsMultisigInstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SquadsMultisigInstructionData, Error> {
        let tag = reader.remaining();
        if tag.is_empty() {
            return Err(Error::Truncated);
        }
        Self::read_0(tag, reader)
    }
}
impl SquadsMultisigInstructionDataCodec {
    fn encoded_len_0(value: &SquadsMultisigInstructionData) -> Result<usize, Error> {
        match value {
            SquadsMultisigInstructionData::ProgramConfigInit(value) => {
                ProgramConfigInitCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::ProgramConfigSetAuthority(value) => {
                ProgramConfigSetAuthorityCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::ProgramConfigSetMultisigCreationFee(value) => {
                ProgramConfigSetMultisigCreationFeeCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::ProgramConfigSetTreasury(value) => {
                ProgramConfigSetTreasuryCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::MultisigCreate(value) => {
                MultisigCreateCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::MultisigCreateV2(value) => {
                MultisigCreateV2Codec.encoded_len(value)
            }
            SquadsMultisigInstructionData::MultisigAddMember(value) => {
                MultisigAddMemberCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::MultisigRemoveMember(value) => {
                MultisigRemoveMemberCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::MultisigSetTimeLock(value) => {
                MultisigSetTimeLockCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::MultisigChangeThreshold(value) => {
                MultisigChangeThresholdCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::MultisigSetConfigAuthority(value) => {
                MultisigSetConfigAuthorityCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::MultisigSetRentCollector(value) => {
                MultisigSetRentCollectorCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::MultisigAddSpendingLimit(value) => {
                MultisigAddSpendingLimitCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::MultisigRemoveSpendingLimit(value) => {
                MultisigRemoveSpendingLimitCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::ConfigTransactionCreate(value) => {
                ConfigTransactionCreateCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::ConfigTransactionExecute(value) => {
                ConfigTransactionExecuteCodec.encoded_len(value)
            }
            _ => Self::encoded_len_1(value),
        }
    }
    fn encoded_len_1(value: &SquadsMultisigInstructionData) -> Result<usize, Error> {
        match value {
            SquadsMultisigInstructionData::VaultTransactionCreate(value) => {
                VaultTransactionCreateCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::TransactionBufferCreate(value) => {
                TransactionBufferCreateCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::TransactionBufferClose(value) => {
                TransactionBufferCloseCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::TransactionBufferExtend(value) => {
                TransactionBufferExtendCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::VaultTransactionCreateFromBuffer(value) => {
                VaultTransactionCreateFromBufferCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::VaultTransactionExecute(value) => {
                VaultTransactionExecuteCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::BatchCreate(value) => {
                BatchCreateCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::BatchAddTransaction(value) => {
                BatchAddTransactionCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::BatchExecuteTransaction(value) => {
                BatchExecuteTransactionCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::ProposalCreate(value) => {
                ProposalCreateCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::ProposalActivate(value) => {
                ProposalActivateCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::ProposalApprove(value) => {
                ProposalApproveCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::ProposalReject(value) => {
                ProposalRejectCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::ProposalCancel(value) => {
                ProposalCancelCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::ProposalCancelV2(value) => {
                ProposalCancelV2Codec.encoded_len(value)
            }
            SquadsMultisigInstructionData::SpendingLimitUse(value) => {
                SpendingLimitUseCodec.encoded_len(value)
            }
            _ => Self::encoded_len_2(value),
        }
    }
    fn encoded_len_2(value: &SquadsMultisigInstructionData) -> Result<usize, Error> {
        match value {
            SquadsMultisigInstructionData::ConfigTransactionAccountsClose(value) => {
                ConfigTransactionAccountsCloseCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::VaultTransactionAccountsClose(value) => {
                VaultTransactionAccountsCloseCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::VaultBatchTransactionAccountClose(value) => {
                VaultBatchTransactionAccountCloseCodec.encoded_len(value)
            }
            SquadsMultisigInstructionData::BatchAccountsClose(value) => {
                BatchAccountsCloseCodec.encoded_len(value)
            }
            _ => Err(Error::InvalidDiscriminator),
        }
    }
    fn write_0(
        value: &SquadsMultisigInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            SquadsMultisigInstructionData::ProgramConfigInit(value) => {
                ProgramConfigInitCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::ProgramConfigSetAuthority(value) => {
                ProgramConfigSetAuthorityCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::ProgramConfigSetMultisigCreationFee(value) => {
                ProgramConfigSetMultisigCreationFeeCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::ProgramConfigSetTreasury(value) => {
                ProgramConfigSetTreasuryCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::MultisigCreate(value) => {
                MultisigCreateCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::MultisigCreateV2(value) => {
                MultisigCreateV2Codec.write(value, writer)
            }
            SquadsMultisigInstructionData::MultisigAddMember(value) => {
                MultisigAddMemberCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::MultisigRemoveMember(value) => {
                MultisigRemoveMemberCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::MultisigSetTimeLock(value) => {
                MultisigSetTimeLockCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::MultisigChangeThreshold(value) => {
                MultisigChangeThresholdCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::MultisigSetConfigAuthority(value) => {
                MultisigSetConfigAuthorityCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::MultisigSetRentCollector(value) => {
                MultisigSetRentCollectorCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::MultisigAddSpendingLimit(value) => {
                MultisigAddSpendingLimitCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::MultisigRemoveSpendingLimit(value) => {
                MultisigRemoveSpendingLimitCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::ConfigTransactionCreate(value) => {
                ConfigTransactionCreateCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::ConfigTransactionExecute(value) => {
                ConfigTransactionExecuteCodec.write(value, writer)
            }
            _ => Self::write_1(value, writer),
        }
    }
    fn write_1(
        value: &SquadsMultisigInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            SquadsMultisigInstructionData::VaultTransactionCreate(value) => {
                VaultTransactionCreateCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::TransactionBufferCreate(value) => {
                TransactionBufferCreateCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::TransactionBufferClose(value) => {
                TransactionBufferCloseCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::TransactionBufferExtend(value) => {
                TransactionBufferExtendCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::VaultTransactionCreateFromBuffer(value) => {
                VaultTransactionCreateFromBufferCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::VaultTransactionExecute(value) => {
                VaultTransactionExecuteCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::BatchCreate(value) => {
                BatchCreateCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::BatchAddTransaction(value) => {
                BatchAddTransactionCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::BatchExecuteTransaction(value) => {
                BatchExecuteTransactionCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::ProposalCreate(value) => {
                ProposalCreateCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::ProposalActivate(value) => {
                ProposalActivateCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::ProposalApprove(value) => {
                ProposalApproveCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::ProposalReject(value) => {
                ProposalRejectCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::ProposalCancel(value) => {
                ProposalCancelCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::ProposalCancelV2(value) => {
                ProposalCancelV2Codec.write(value, writer)
            }
            SquadsMultisigInstructionData::SpendingLimitUse(value) => {
                SpendingLimitUseCodec.write(value, writer)
            }
            _ => Self::write_2(value, writer),
        }
    }
    fn write_2(
        value: &SquadsMultisigInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            SquadsMultisigInstructionData::ConfigTransactionAccountsClose(value) => {
                ConfigTransactionAccountsCloseCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::VaultTransactionAccountsClose(value) => {
                VaultTransactionAccountsCloseCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::VaultBatchTransactionAccountClose(value) => {
                VaultBatchTransactionAccountCloseCodec.write(value, writer)
            }
            SquadsMultisigInstructionData::BatchAccountsClose(value) => {
                BatchAccountsCloseCodec.write(value, writer)
            }
            _ => Err(Error::InvalidDiscriminator),
        }
    }
    fn read_0(tag: &[u8], reader: &mut Reader<'_>) -> Result<SquadsMultisigInstructionData, Error> {
        match tag {
            [184, 188, 198, 195, 205, 124, 117, 216, ..] => {
                Ok(SquadsMultisigInstructionData::ProgramConfigInit(
                    ProgramConfigInitCodec.read(reader)?,
                ))
            }
            [238, 242, 36, 181, 32, 143, 216, 75, ..] => {
                Ok(SquadsMultisigInstructionData::ProgramConfigSetAuthority(
                    ProgramConfigSetAuthorityCodec.read(reader)?,
                ))
            }
            [101, 160, 249, 63, 154, 215, 153, 13, ..] => Ok(
                SquadsMultisigInstructionData::ProgramConfigSetMultisigCreationFee(
                    ProgramConfigSetMultisigCreationFeeCodec.read(reader)?,
                ),
            ),
            [111, 46, 243, 117, 144, 188, 162, 107, ..] => {
                Ok(SquadsMultisigInstructionData::ProgramConfigSetTreasury(
                    ProgramConfigSetTreasuryCodec.read(reader)?,
                ))
            }
            [122, 77, 80, 159, 84, 88, 90, 197, ..] => Ok(
                SquadsMultisigInstructionData::MultisigCreate(MultisigCreateCodec.read(reader)?),
            ),
            [50, 221, 199, 93, 40, 245, 139, 233, ..] => {
                Ok(SquadsMultisigInstructionData::MultisigCreateV2(
                    MultisigCreateV2Codec.read(reader)?,
                ))
            }
            [1, 219, 215, 108, 184, 229, 214, 8, ..] => {
                Ok(SquadsMultisigInstructionData::MultisigAddMember(
                    MultisigAddMemberCodec.read(reader)?,
                ))
            }
            [217, 117, 177, 210, 182, 145, 218, 72, ..] => {
                Ok(SquadsMultisigInstructionData::MultisigRemoveMember(
                    MultisigRemoveMemberCodec.read(reader)?,
                ))
            }
            [148, 154, 121, 77, 212, 254, 155, 72, ..] => {
                Ok(SquadsMultisigInstructionData::MultisigSetTimeLock(
                    MultisigSetTimeLockCodec.read(reader)?,
                ))
            }
            [141, 42, 15, 126, 169, 92, 62, 181, ..] => {
                Ok(SquadsMultisigInstructionData::MultisigChangeThreshold(
                    MultisigChangeThresholdCodec.read(reader)?,
                ))
            }
            [143, 93, 199, 143, 92, 169, 193, 232, ..] => {
                Ok(SquadsMultisigInstructionData::MultisigSetConfigAuthority(
                    MultisigSetConfigAuthorityCodec.read(reader)?,
                ))
            }
            [48, 204, 65, 57, 210, 70, 156, 74, ..] => {
                Ok(SquadsMultisigInstructionData::MultisigSetRentCollector(
                    MultisigSetRentCollectorCodec.read(reader)?,
                ))
            }
            [11, 242, 159, 42, 86, 197, 89, 115, ..] => {
                Ok(SquadsMultisigInstructionData::MultisigAddSpendingLimit(
                    MultisigAddSpendingLimitCodec.read(reader)?,
                ))
            }
            [228, 198, 136, 111, 123, 4, 178, 113, ..] => {
                Ok(SquadsMultisigInstructionData::MultisigRemoveSpendingLimit(
                    MultisigRemoveSpendingLimitCodec.read(reader)?,
                ))
            }
            [155, 236, 87, 228, 137, 75, 81, 39, ..] => {
                Ok(SquadsMultisigInstructionData::ConfigTransactionCreate(
                    ConfigTransactionCreateCodec.read(reader)?,
                ))
            }
            [114, 146, 244, 189, 252, 140, 36, 40, ..] => {
                Ok(SquadsMultisigInstructionData::ConfigTransactionExecute(
                    ConfigTransactionExecuteCodec.read(reader)?,
                ))
            }
            _ => Self::read_1(tag, reader),
        }
    }
    fn read_1(tag: &[u8], reader: &mut Reader<'_>) -> Result<SquadsMultisigInstructionData, Error> {
        match tag {
            [48, 250, 78, 168, 208, 226, 218, 211, ..] => {
                Ok(SquadsMultisigInstructionData::VaultTransactionCreate(
                    VaultTransactionCreateCodec.read(reader)?,
                ))
            }
            [245, 201, 113, 108, 37, 63, 29, 89, ..] => {
                Ok(SquadsMultisigInstructionData::TransactionBufferCreate(
                    TransactionBufferCreateCodec.read(reader)?,
                ))
            }
            [17, 182, 208, 228, 136, 24, 178, 102, ..] => {
                Ok(SquadsMultisigInstructionData::TransactionBufferClose(
                    TransactionBufferCloseCodec.read(reader)?,
                ))
            }
            [230, 157, 67, 56, 5, 238, 245, 146, ..] => {
                Ok(SquadsMultisigInstructionData::TransactionBufferExtend(
                    TransactionBufferExtendCodec.read(reader)?,
                ))
            }
            [222, 54, 149, 68, 87, 246, 48, 231, ..] => Ok(
                SquadsMultisigInstructionData::VaultTransactionCreateFromBuffer(
                    VaultTransactionCreateFromBufferCodec.read(reader)?,
                ),
            ),
            [194, 8, 161, 87, 153, 164, 25, 171, ..] => {
                Ok(SquadsMultisigInstructionData::VaultTransactionExecute(
                    VaultTransactionExecuteCodec.read(reader)?,
                ))
            }
            [194, 142, 141, 17, 55, 185, 20, 248, ..] => Ok(
                SquadsMultisigInstructionData::BatchCreate(BatchCreateCodec.read(reader)?),
            ),
            [89, 100, 224, 18, 69, 70, 54, 76, ..] => {
                Ok(SquadsMultisigInstructionData::BatchAddTransaction(
                    BatchAddTransactionCodec.read(reader)?,
                ))
            }
            [172, 44, 179, 152, 21, 127, 234, 180, ..] => {
                Ok(SquadsMultisigInstructionData::BatchExecuteTransaction(
                    BatchExecuteTransactionCodec.read(reader)?,
                ))
            }
            [220, 60, 73, 224, 30, 108, 79, 159, ..] => Ok(
                SquadsMultisigInstructionData::ProposalCreate(ProposalCreateCodec.read(reader)?),
            ),
            [11, 34, 92, 248, 154, 27, 51, 106, ..] => {
                Ok(SquadsMultisigInstructionData::ProposalActivate(
                    ProposalActivateCodec.read(reader)?,
                ))
            }
            [144, 37, 164, 136, 188, 216, 42, 248, ..] => Ok(
                SquadsMultisigInstructionData::ProposalApprove(ProposalApproveCodec.read(reader)?),
            ),
            [243, 62, 134, 156, 230, 106, 246, 135, ..] => Ok(
                SquadsMultisigInstructionData::ProposalReject(ProposalRejectCodec.read(reader)?),
            ),
            [27, 42, 127, 237, 38, 163, 84, 203, ..] => Ok(
                SquadsMultisigInstructionData::ProposalCancel(ProposalCancelCodec.read(reader)?),
            ),
            [205, 41, 194, 61, 220, 139, 16, 247, ..] => {
                Ok(SquadsMultisigInstructionData::ProposalCancelV2(
                    ProposalCancelV2Codec.read(reader)?,
                ))
            }
            [16, 57, 130, 127, 193, 20, 155, 134, ..] => {
                Ok(SquadsMultisigInstructionData::SpendingLimitUse(
                    SpendingLimitUseCodec.read(reader)?,
                ))
            }
            _ => Self::read_2(tag, reader),
        }
    }
    fn read_2(tag: &[u8], reader: &mut Reader<'_>) -> Result<SquadsMultisigInstructionData, Error> {
        match tag {
            [80, 203, 84, 53, 151, 112, 187, 186, ..] => Ok(
                SquadsMultisigInstructionData::ConfigTransactionAccountsClose(
                    ConfigTransactionAccountsCloseCodec.read(reader)?,
                ),
            ),
            [196, 71, 187, 176, 2, 35, 170, 165, ..] => Ok(
                SquadsMultisigInstructionData::VaultTransactionAccountsClose(
                    VaultTransactionAccountsCloseCodec.read(reader)?,
                ),
            ),
            [134, 18, 19, 106, 129, 68, 97, 247, ..] => Ok(
                SquadsMultisigInstructionData::VaultBatchTransactionAccountClose(
                    VaultBatchTransactionAccountCloseCodec.read(reader)?,
                ),
            ),
            [218, 196, 7, 175, 130, 102, 11, 255, ..] => {
                Ok(SquadsMultisigInstructionData::BatchAccountsClose(
                    BatchAccountsCloseCodec.read(reader)?,
                ))
            }
            _ => Err(Error::InvalidDiscriminator),
        }
    }
}
/// Identifies a literal header without validating its payload.
pub fn identify_instruction(data: &[u8]) -> Result<&[u8], Error> {
    match data {
        [184, 188, 198, 195, 205, 124, 117, 216, ..] => Ok(&data[..8]),
        [238, 242, 36, 181, 32, 143, 216, 75, ..] => Ok(&data[..8]),
        [101, 160, 249, 63, 154, 215, 153, 13, ..] => Ok(&data[..8]),
        [111, 46, 243, 117, 144, 188, 162, 107, ..] => Ok(&data[..8]),
        [122, 77, 80, 159, 84, 88, 90, 197, ..] => Ok(&data[..8]),
        [50, 221, 199, 93, 40, 245, 139, 233, ..] => Ok(&data[..8]),
        [1, 219, 215, 108, 184, 229, 214, 8, ..] => Ok(&data[..8]),
        [217, 117, 177, 210, 182, 145, 218, 72, ..] => Ok(&data[..8]),
        [148, 154, 121, 77, 212, 254, 155, 72, ..] => Ok(&data[..8]),
        [141, 42, 15, 126, 169, 92, 62, 181, ..] => Ok(&data[..8]),
        [143, 93, 199, 143, 92, 169, 193, 232, ..] => Ok(&data[..8]),
        [48, 204, 65, 57, 210, 70, 156, 74, ..] => Ok(&data[..8]),
        [11, 242, 159, 42, 86, 197, 89, 115, ..] => Ok(&data[..8]),
        [228, 198, 136, 111, 123, 4, 178, 113, ..] => Ok(&data[..8]),
        [155, 236, 87, 228, 137, 75, 81, 39, ..] => Ok(&data[..8]),
        [114, 146, 244, 189, 252, 140, 36, 40, ..] => Ok(&data[..8]),
        [48, 250, 78, 168, 208, 226, 218, 211, ..] => Ok(&data[..8]),
        [245, 201, 113, 108, 37, 63, 29, 89, ..] => Ok(&data[..8]),
        [17, 182, 208, 228, 136, 24, 178, 102, ..] => Ok(&data[..8]),
        [230, 157, 67, 56, 5, 238, 245, 146, ..] => Ok(&data[..8]),
        [222, 54, 149, 68, 87, 246, 48, 231, ..] => Ok(&data[..8]),
        [194, 8, 161, 87, 153, 164, 25, 171, ..] => Ok(&data[..8]),
        [194, 142, 141, 17, 55, 185, 20, 248, ..] => Ok(&data[..8]),
        [89, 100, 224, 18, 69, 70, 54, 76, ..] => Ok(&data[..8]),
        [172, 44, 179, 152, 21, 127, 234, 180, ..] => Ok(&data[..8]),
        [220, 60, 73, 224, 30, 108, 79, 159, ..] => Ok(&data[..8]),
        [11, 34, 92, 248, 154, 27, 51, 106, ..] => Ok(&data[..8]),
        [144, 37, 164, 136, 188, 216, 42, 248, ..] => Ok(&data[..8]),
        [243, 62, 134, 156, 230, 106, 246, 135, ..] => Ok(&data[..8]),
        [27, 42, 127, 237, 38, 163, 84, 203, ..] => Ok(&data[..8]),
        [205, 41, 194, 61, 220, 139, 16, 247, ..] => Ok(&data[..8]),
        [16, 57, 130, 127, 193, 20, 155, 134, ..] => Ok(&data[..8]),
        [80, 203, 84, 53, 151, 112, 187, 186, ..] => Ok(&data[..8]),
        [196, 71, 187, 176, 2, 35, 170, 165, ..] => Ok(&data[..8]),
        [134, 18, 19, 106, 129, 68, 97, 247, ..] => Ok(&data[..8]),
        [218, 196, 7, 175, 130, 102, 11, 255, ..] => Ok(&data[..8]),
        [] => Err(Error::Truncated),
        _ => Err(Error::InvalidDiscriminator),
    }
}
/// Strictly parses a canonical program invocation.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<SquadsMultisigInstructionData, Error> {
    parse_instruction_with_program(PROGRAM, instruction, limits)
}
/// Strictly parses an explicit expected program identity.
pub fn parse_instruction_with_program(
    program: Address,
    instruction: &Instruction,
    limits: Limits,
) -> Result<SquadsMultisigInstructionData, Error> {
    match identify_instruction(&instruction.data)? {
        [184, 188, 198, 195, 205, 124, 117, 216, ..] => {
            Ok(SquadsMultisigInstructionData::ProgramConfigInit(
                ProgramConfigInit::parse_with_program(program, instruction, limits)?,
            ))
        }
        [238, 242, 36, 181, 32, 143, 216, 75, ..] => {
            Ok(SquadsMultisigInstructionData::ProgramConfigSetAuthority(
                ProgramConfigSetAuthority::parse_with_program(program, instruction, limits)?,
            ))
        }
        [101, 160, 249, 63, 154, 215, 153, 13, ..] => Ok(
            SquadsMultisigInstructionData::ProgramConfigSetMultisigCreationFee(
                ProgramConfigSetMultisigCreationFee::parse_with_program(
                    program,
                    instruction,
                    limits,
                )?,
            ),
        ),
        [111, 46, 243, 117, 144, 188, 162, 107, ..] => {
            Ok(SquadsMultisigInstructionData::ProgramConfigSetTreasury(
                ProgramConfigSetTreasury::parse_with_program(program, instruction, limits)?,
            ))
        }
        [122, 77, 80, 159, 84, 88, 90, 197, ..] => {
            Ok(SquadsMultisigInstructionData::MultisigCreate(
                MultisigCreate::parse_with_program(program, instruction, limits)?,
            ))
        }
        [50, 221, 199, 93, 40, 245, 139, 233, ..] => {
            Ok(SquadsMultisigInstructionData::MultisigCreateV2(
                MultisigCreateV2::parse_with_program(program, instruction, limits)?,
            ))
        }
        [1, 219, 215, 108, 184, 229, 214, 8, ..] => {
            Ok(SquadsMultisigInstructionData::MultisigAddMember(
                MultisigAddMember::parse_with_program(program, instruction, limits)?,
            ))
        }
        [217, 117, 177, 210, 182, 145, 218, 72, ..] => {
            Ok(SquadsMultisigInstructionData::MultisigRemoveMember(
                MultisigRemoveMember::parse_with_program(program, instruction, limits)?,
            ))
        }
        [148, 154, 121, 77, 212, 254, 155, 72, ..] => {
            Ok(SquadsMultisigInstructionData::MultisigSetTimeLock(
                MultisigSetTimeLock::parse_with_program(program, instruction, limits)?,
            ))
        }
        [141, 42, 15, 126, 169, 92, 62, 181, ..] => {
            Ok(SquadsMultisigInstructionData::MultisigChangeThreshold(
                MultisigChangeThreshold::parse_with_program(program, instruction, limits)?,
            ))
        }
        [143, 93, 199, 143, 92, 169, 193, 232, ..] => {
            Ok(SquadsMultisigInstructionData::MultisigSetConfigAuthority(
                MultisigSetConfigAuthority::parse_with_program(program, instruction, limits)?,
            ))
        }
        [48, 204, 65, 57, 210, 70, 156, 74, ..] => {
            Ok(SquadsMultisigInstructionData::MultisigSetRentCollector(
                MultisigSetRentCollector::parse_with_program(program, instruction, limits)?,
            ))
        }
        [11, 242, 159, 42, 86, 197, 89, 115, ..] => {
            Ok(SquadsMultisigInstructionData::MultisigAddSpendingLimit(
                MultisigAddSpendingLimit::parse_with_program(program, instruction, limits)?,
            ))
        }
        [228, 198, 136, 111, 123, 4, 178, 113, ..] => {
            Ok(SquadsMultisigInstructionData::MultisigRemoveSpendingLimit(
                MultisigRemoveSpendingLimit::parse_with_program(program, instruction, limits)?,
            ))
        }
        [155, 236, 87, 228, 137, 75, 81, 39, ..] => {
            Ok(SquadsMultisigInstructionData::ConfigTransactionCreate(
                ConfigTransactionCreate::parse_with_program(program, instruction, limits)?,
            ))
        }
        [114, 146, 244, 189, 252, 140, 36, 40, ..] => {
            Ok(SquadsMultisigInstructionData::ConfigTransactionExecute(
                ConfigTransactionExecute::parse_with_program(program, instruction, limits)?,
            ))
        }
        [48, 250, 78, 168, 208, 226, 218, 211, ..] => {
            Ok(SquadsMultisigInstructionData::VaultTransactionCreate(
                VaultTransactionCreate::parse_with_program(program, instruction, limits)?,
            ))
        }
        [245, 201, 113, 108, 37, 63, 29, 89, ..] => {
            Ok(SquadsMultisigInstructionData::TransactionBufferCreate(
                TransactionBufferCreate::parse_with_program(program, instruction, limits)?,
            ))
        }
        [17, 182, 208, 228, 136, 24, 178, 102, ..] => {
            Ok(SquadsMultisigInstructionData::TransactionBufferClose(
                TransactionBufferClose::parse_with_program(program, instruction, limits)?,
            ))
        }
        [230, 157, 67, 56, 5, 238, 245, 146, ..] => {
            Ok(SquadsMultisigInstructionData::TransactionBufferExtend(
                TransactionBufferExtend::parse_with_program(program, instruction, limits)?,
            ))
        }
        [222, 54, 149, 68, 87, 246, 48, 231, ..] => Ok(
            SquadsMultisigInstructionData::VaultTransactionCreateFromBuffer(
                VaultTransactionCreateFromBuffer::parse_with_program(program, instruction, limits)?,
            ),
        ),
        [194, 8, 161, 87, 153, 164, 25, 171, ..] => {
            Ok(SquadsMultisigInstructionData::VaultTransactionExecute(
                VaultTransactionExecute::parse_with_program(program, instruction, limits)?,
            ))
        }
        [194, 142, 141, 17, 55, 185, 20, 248, ..] => {
            Ok(SquadsMultisigInstructionData::BatchCreate(
                BatchCreate::parse_with_program(program, instruction, limits)?,
            ))
        }
        [89, 100, 224, 18, 69, 70, 54, 76, ..] => {
            Ok(SquadsMultisigInstructionData::BatchAddTransaction(
                BatchAddTransaction::parse_with_program(program, instruction, limits)?,
            ))
        }
        [172, 44, 179, 152, 21, 127, 234, 180, ..] => {
            Ok(SquadsMultisigInstructionData::BatchExecuteTransaction(
                BatchExecuteTransaction::parse_with_program(program, instruction, limits)?,
            ))
        }
        [220, 60, 73, 224, 30, 108, 79, 159, ..] => {
            Ok(SquadsMultisigInstructionData::ProposalCreate(
                ProposalCreate::parse_with_program(program, instruction, limits)?,
            ))
        }
        [11, 34, 92, 248, 154, 27, 51, 106, ..] => {
            Ok(SquadsMultisigInstructionData::ProposalActivate(
                ProposalActivate::parse_with_program(program, instruction, limits)?,
            ))
        }
        [144, 37, 164, 136, 188, 216, 42, 248, ..] => {
            Ok(SquadsMultisigInstructionData::ProposalApprove(
                ProposalApprove::parse_with_program(program, instruction, limits)?,
            ))
        }
        [243, 62, 134, 156, 230, 106, 246, 135, ..] => {
            Ok(SquadsMultisigInstructionData::ProposalReject(
                ProposalReject::parse_with_program(program, instruction, limits)?,
            ))
        }
        [27, 42, 127, 237, 38, 163, 84, 203, ..] => {
            Ok(SquadsMultisigInstructionData::ProposalCancel(
                ProposalCancel::parse_with_program(program, instruction, limits)?,
            ))
        }
        [205, 41, 194, 61, 220, 139, 16, 247, ..] => {
            Ok(SquadsMultisigInstructionData::ProposalCancelV2(
                ProposalCancelV2::parse_with_program(program, instruction, limits)?,
            ))
        }
        [16, 57, 130, 127, 193, 20, 155, 134, ..] => {
            Ok(SquadsMultisigInstructionData::SpendingLimitUse(
                SpendingLimitUse::parse_with_program(program, instruction, limits)?,
            ))
        }
        [80, 203, 84, 53, 151, 112, 187, 186, ..] => Ok(
            SquadsMultisigInstructionData::ConfigTransactionAccountsClose(
                ConfigTransactionAccountsClose::parse_with_program(program, instruction, limits)?,
            ),
        ),
        [196, 71, 187, 176, 2, 35, 170, 165, ..] => Ok(
            SquadsMultisigInstructionData::VaultTransactionAccountsClose(
                VaultTransactionAccountsClose::parse_with_program(program, instruction, limits)?,
            ),
        ),
        [134, 18, 19, 106, 129, 68, 97, 247, ..] => Ok(
            SquadsMultisigInstructionData::VaultBatchTransactionAccountClose(
                VaultBatchTransactionAccountClose::parse_with_program(
                    program,
                    instruction,
                    limits,
                )?,
            ),
        ),
        [218, 196, 7, 175, 130, 102, 11, 255, ..] => {
            Ok(SquadsMultisigInstructionData::BatchAccountsClose(
                BatchAccountsClose::parse_with_program(program, instruction, limits)?,
            ))
        }
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
use solana_kit_core::codecs::OptionCodec;
use solana_kit_core::codecs::Utf8;
use solana_kit_core::codecs::{ByteSizePrefix, FrameConsumption};
impl core::hash::Hash for Member {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.key, state);
        core::hash::Hash::hash(&self.permissions, state);
    }
}
impl core::fmt::Display for Member {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Permissions {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.mask, state);
    }
}
impl core::fmt::Display for Permissions {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VaultTransactionMessage {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.num_signers, state);
        core::hash::Hash::hash(&self.num_writable_signers, state);
        core::hash::Hash::hash(&self.num_writable_non_signers, state);
        core::hash::Hash::hash(&self.account_keys, state);
        core::hash::Hash::hash(&self.instructions, state);
        core::hash::Hash::hash(&self.address_table_lookups, state);
    }
}
impl core::fmt::Display for VaultTransactionMessage {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigCompiledInstruction {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.program_id_index, state);
        core::hash::Hash::hash(&self.account_indexes, state);
        core::hash::Hash::hash(&self.data, state);
    }
}
impl core::fmt::Display for MultisigCompiledInstruction {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigMessageAddressTableLookup {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.account_key, state);
        core::hash::Hash::hash(&self.writable_indexes, state);
        core::hash::Hash::hash(&self.readonly_indexes, state);
    }
}
impl core::fmt::Display for MultisigMessageAddressTableLookup {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalVoteArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for ProposalVoteArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VaultTransactionCreateArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.vault_index, state);
        core::hash::Hash::hash(&self.ephemeral_signers, state);
        core::hash::Hash::hash(&self.transaction_message, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for VaultTransactionCreateArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Vote {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Approve => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Reject => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Cancel => {
                core::hash::Hash::hash(&2usize, state);
            }
        }
    }
}
impl core::fmt::Display for Vote {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Period {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::OneTime => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Day => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Week => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::Month => {
                core::hash::Hash::hash(&3usize, state);
            }
        }
    }
}
impl core::fmt::Display for Period {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalStatusDraftPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.timestamp, state);
    }
}
impl core::fmt::Display for ProposalStatusDraftPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalStatusActivePayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.timestamp, state);
    }
}
impl core::fmt::Display for ProposalStatusActivePayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalStatusRejectedPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.timestamp, state);
    }
}
impl core::fmt::Display for ProposalStatusRejectedPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalStatusApprovedPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.timestamp, state);
    }
}
impl core::fmt::Display for ProposalStatusApprovedPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalStatusExecutedPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.timestamp, state);
    }
}
impl core::fmt::Display for ProposalStatusExecutedPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalStatusCancelledPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.timestamp, state);
    }
}
impl core::fmt::Display for ProposalStatusCancelledPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalStatus {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Draft(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Active(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Rejected(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Approved(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Executing => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::Executed(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Cancelled(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for ProposalStatus {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigActionAddMemberPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_member, state);
    }
}
impl core::fmt::Display for ConfigActionAddMemberPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigActionRemoveMemberPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.old_member, state);
    }
}
impl core::fmt::Display for ConfigActionRemoveMemberPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigActionChangeThresholdPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_threshold, state);
    }
}
impl core::fmt::Display for ConfigActionChangeThresholdPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigActionSetTimeLockPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_time_lock, state);
    }
}
impl core::fmt::Display for ConfigActionSetTimeLockPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigActionAddSpendingLimitPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.create_key, state);
        core::hash::Hash::hash(&self.vault_index, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.period, state);
        core::hash::Hash::hash(&self.members, state);
        core::hash::Hash::hash(&self.destinations, state);
    }
}
impl core::fmt::Display for ConfigActionAddSpendingLimitPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigActionRemoveSpendingLimitPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.spending_limit, state);
    }
}
impl core::fmt::Display for ConfigActionRemoveSpendingLimitPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigActionSetRentCollectorPayload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_rent_collector, state);
    }
}
impl core::fmt::Display for ConfigActionSetRentCollectorPayload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigAction {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::AddMember(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RemoveMember(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ChangeThreshold(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetTimeLock(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AddSpendingLimit(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RemoveSpendingLimit(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetRentCollector(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for ConfigAction {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Batch {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.multisig, state);
        core::hash::Hash::hash(&self.creator, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.vault_index, state);
        core::hash::Hash::hash(&self.vault_bump, state);
        core::hash::Hash::hash(&self.size, state);
        core::hash::Hash::hash(&self.executed_transaction_index, state);
    }
}
impl core::fmt::Display for Batch {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VaultBatchTransaction {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.ephemeral_signer_bumps, state);
        core::hash::Hash::hash(&self.message, state);
    }
}
impl core::fmt::Display for VaultBatchTransaction {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigTransaction {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.multisig, state);
        core::hash::Hash::hash(&self.creator, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.actions, state);
    }
}
impl core::fmt::Display for ConfigTransaction {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Multisig {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.create_key, state);
        core::hash::Hash::hash(&self.config_authority, state);
        core::hash::Hash::hash(&self.threshold, state);
        core::hash::Hash::hash(&self.time_lock, state);
        core::hash::Hash::hash(&self.transaction_index, state);
        core::hash::Hash::hash(&self.stale_transaction_index, state);
        core::hash::Hash::hash(&self.rent_collector, state);
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.members, state);
    }
}
impl core::fmt::Display for Multisig {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProgramConfig {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authority, state);
        core::hash::Hash::hash(&self.multisig_creation_fee, state);
        core::hash::Hash::hash(&self.treasury, state);
        core::hash::Hash::hash(&self.reserved, state);
    }
}
impl core::fmt::Display for ProgramConfig {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Proposal {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.multisig, state);
        core::hash::Hash::hash(&self.transaction_index, state);
        core::hash::Hash::hash(&self.status, state);
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.approved, state);
        core::hash::Hash::hash(&self.rejected, state);
        core::hash::Hash::hash(&self.cancelled, state);
    }
}
impl core::fmt::Display for Proposal {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SpendingLimit {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.multisig, state);
        core::hash::Hash::hash(&self.create_key, state);
        core::hash::Hash::hash(&self.vault_index, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.period, state);
        core::hash::Hash::hash(&self.remaining_amount, state);
        core::hash::Hash::hash(&self.last_reset, state);
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.members, state);
        core::hash::Hash::hash(&self.destinations, state);
    }
}
impl core::fmt::Display for SpendingLimit {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransactionBuffer {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.multisig, state);
        core::hash::Hash::hash(&self.creator, state);
        core::hash::Hash::hash(&self.buffer_index, state);
        core::hash::Hash::hash(&self.vault_index, state);
        core::hash::Hash::hash(&self.final_buffer_hash, state);
        core::hash::Hash::hash(&self.final_buffer_size, state);
        core::hash::Hash::hash(&self.buffer, state);
    }
}
impl core::fmt::Display for TransactionBuffer {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VaultTransaction {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.multisig, state);
        core::hash::Hash::hash(&self.creator, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.vault_index, state);
        core::hash::Hash::hash(&self.vault_bump, state);
        core::hash::Hash::hash(&self.ephemeral_signer_bumps, state);
        core::hash::Hash::hash(&self.message, state);
    }
}
impl core::fmt::Display for VaultTransaction {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProgramConfigInit {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authority, state);
        core::hash::Hash::hash(&self.multisig_creation_fee, state);
        core::hash::Hash::hash(&self.treasury, state);
    }
}
impl core::fmt::Display for ProgramConfigInit {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProgramConfigSetAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_authority, state);
    }
}
impl core::fmt::Display for ProgramConfigSetAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProgramConfigSetMultisigCreationFee {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_multisig_creation_fee, state);
    }
}
impl core::fmt::Display for ProgramConfigSetMultisigCreationFee {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProgramConfigSetTreasury {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_treasury, state);
    }
}
impl core::fmt::Display for ProgramConfigSetTreasury {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigCreate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for MultisigCreate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigCreateV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.config_authority, state);
        core::hash::Hash::hash(&self.threshold, state);
        core::hash::Hash::hash(&self.members, state);
        core::hash::Hash::hash(&self.time_lock, state);
        core::hash::Hash::hash(&self.rent_collector, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for MultisigCreateV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigAddMember {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_member, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for MultisigAddMember {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigRemoveMember {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.old_member, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for MultisigRemoveMember {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigSetTimeLock {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.time_lock, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for MultisigSetTimeLock {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigChangeThreshold {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_threshold, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for MultisigChangeThreshold {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigSetConfigAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_config_authority, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for MultisigSetConfigAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigSetRentCollector {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.rent_collector, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for MultisigSetRentCollector {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigAddSpendingLimit {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.create_key, state);
        core::hash::Hash::hash(&self.vault_index, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.period, state);
        core::hash::Hash::hash(&self.members, state);
        core::hash::Hash::hash(&self.destinations, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for MultisigAddSpendingLimit {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MultisigRemoveSpendingLimit {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for MultisigRemoveSpendingLimit {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigTransactionCreate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.actions, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for ConfigTransactionCreate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigTransactionExecute {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for ConfigTransactionExecute {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VaultTransactionCreate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.args, state);
    }
}
impl core::fmt::Display for VaultTransactionCreate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransactionBufferCreate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.buffer_index, state);
        core::hash::Hash::hash(&self.vault_index, state);
        core::hash::Hash::hash(&self.final_buffer_hash, state);
        core::hash::Hash::hash(&self.final_buffer_size, state);
        core::hash::Hash::hash(&self.buffer, state);
    }
}
impl core::fmt::Display for TransactionBufferCreate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransactionBufferClose {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for TransactionBufferClose {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransactionBufferExtend {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.buffer, state);
    }
}
impl core::fmt::Display for TransactionBufferExtend {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VaultTransactionCreateFromBuffer {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.args, state);
    }
}
impl core::fmt::Display for VaultTransactionCreateFromBuffer {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VaultTransactionExecute {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for VaultTransactionExecute {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BatchCreate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.vault_index, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for BatchCreate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BatchAddTransaction {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.ephemeral_signers, state);
        core::hash::Hash::hash(&self.transaction_message, state);
    }
}
impl core::fmt::Display for BatchAddTransaction {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BatchExecuteTransaction {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for BatchExecuteTransaction {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalCreate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.transaction_index, state);
        core::hash::Hash::hash(&self.draft, state);
    }
}
impl core::fmt::Display for ProposalCreate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalActivate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for ProposalActivate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalApprove {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.args, state);
    }
}
impl core::fmt::Display for ProposalApprove {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalReject {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.args, state);
    }
}
impl core::fmt::Display for ProposalReject {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalCancel {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.args, state);
    }
}
impl core::fmt::Display for ProposalCancel {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProposalCancelV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.args, state);
    }
}
impl core::fmt::Display for ProposalCancelV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SpendingLimitUse {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.decimals, state);
        core::hash::Hash::hash(&self.memo, state);
    }
}
impl core::fmt::Display for SpendingLimitUse {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConfigTransactionAccountsClose {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for ConfigTransactionAccountsClose {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VaultTransactionAccountsClose {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for VaultTransactionAccountsClose {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VaultBatchTransactionAccountClose {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for VaultBatchTransactionAccountClose {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BatchAccountsClose {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for BatchAccountsClose {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SquadsMultisigInstructionData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::ProgramConfigInit(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProgramConfigSetAuthority(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProgramConfigSetMultisigCreationFee(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProgramConfigSetTreasury(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MultisigCreate(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MultisigCreateV2(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MultisigAddMember(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MultisigRemoveMember(value) => {
                core::hash::Hash::hash(&7usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MultisigSetTimeLock(value) => {
                core::hash::Hash::hash(&8usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MultisigChangeThreshold(value) => {
                core::hash::Hash::hash(&9usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MultisigSetConfigAuthority(value) => {
                core::hash::Hash::hash(&10usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MultisigSetRentCollector(value) => {
                core::hash::Hash::hash(&11usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MultisigAddSpendingLimit(value) => {
                core::hash::Hash::hash(&12usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MultisigRemoveSpendingLimit(value) => {
                core::hash::Hash::hash(&13usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ConfigTransactionCreate(value) => {
                core::hash::Hash::hash(&14usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ConfigTransactionExecute(value) => {
                core::hash::Hash::hash(&15usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VaultTransactionCreate(value) => {
                core::hash::Hash::hash(&16usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransactionBufferCreate(value) => {
                core::hash::Hash::hash(&17usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransactionBufferClose(value) => {
                core::hash::Hash::hash(&18usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransactionBufferExtend(value) => {
                core::hash::Hash::hash(&19usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VaultTransactionCreateFromBuffer(value) => {
                core::hash::Hash::hash(&20usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VaultTransactionExecute(value) => {
                core::hash::Hash::hash(&21usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BatchCreate(value) => {
                core::hash::Hash::hash(&22usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BatchAddTransaction(value) => {
                core::hash::Hash::hash(&23usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BatchExecuteTransaction(value) => {
                core::hash::Hash::hash(&24usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProposalCreate(value) => {
                core::hash::Hash::hash(&25usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProposalActivate(value) => {
                core::hash::Hash::hash(&26usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProposalApprove(value) => {
                core::hash::Hash::hash(&27usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProposalReject(value) => {
                core::hash::Hash::hash(&28usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProposalCancel(value) => {
                core::hash::Hash::hash(&29usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProposalCancelV2(value) => {
                core::hash::Hash::hash(&30usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SpendingLimitUse(value) => {
                core::hash::Hash::hash(&31usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ConfigTransactionAccountsClose(value) => {
                core::hash::Hash::hash(&32usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VaultTransactionAccountsClose(value) => {
                core::hash::Hash::hash(&33usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VaultBatchTransactionAccountClose(value) => {
                core::hash::Hash::hash(&34usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BatchAccountsClose(value) => {
                core::hash::Hash::hash(&35usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for SquadsMultisigInstructionData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SquadsMultisigError {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.code(), state)
    }
}
impl core::fmt::Display for SquadsMultisigError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.message() {
            Some(message) => formatter.write_str(message),
            None => write!(formatter, "unknown program error {}", self.code()),
        }
    }
}
impl core::error::Error for SquadsMultisigError {}
