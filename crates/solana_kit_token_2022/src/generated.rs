// Generated from the exact pinned Codama input; see upstream notices.
use alloc::vec::Vec;
use solana_kit_core::codecs::Constant;
use solana_kit_core::codecs::Number;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Reader, Writer},
};
const FIXED_COUNT_11: usize = 11;

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

struct TlvSequence<C, F>(C, F);
impl<C: Encoder<T>, F: Fn(&T) -> bool, T> Encoder<Vec<T>> for TlvSequence<C, F> {
    fn encoded_len(&self, value: &Vec<T>) -> Result<usize, Error> {
        if value.iter().any(&self.1) {
            return Err(Error::InvalidValue);
        }

        OwnedSequence(Sequence {
            item: &self.0,
            count: Count::Remainder,
        })
        .encoded_len(value)
    }
    fn write(&self, value: &Vec<T>, writer: &mut Writer<'_>) -> Result<(), Error> {
        if value.iter().any(&self.1) {
            return Err(Error::InvalidValue);
        }

        OwnedSequence(Sequence {
            item: &self.0,
            count: Count::Remainder,
        })
        .write(value, writer)
    }
}
impl<C: Decoder<T>, F, T> Decoder<Vec<T>> for TlvSequence<C, F> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
        reader.transaction(|reader| {
            let mut values = Vec::new();

            while let [first, second, ..] = reader.remaining() {
                if u16::from_le_bytes([*first, *second]) == 0 {
                    break;
                }

                let before = reader.position();
                let value = self.0.read(reader)?;

                if reader.position() <= before {
                    return Err(Error::InvalidValue);
                }

                reader.claim(1)?;
                values
                    .try_reserve_exact(1)
                    .map_err(|_| Error::AllocationFailed)?;
                values.push(value);
            }

            reader.take(reader.remaining().len())?;
            Ok(values)
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, PartialEq)]
pub enum AccountState {
    /// The `Uninitialized` variant.
    Uninitialized,
    /// The `Initialized` variant.
    Initialized,
    /// The `Frozen` variant.
    Frozen,
}
/// Bounded codec for [`AccountState`].
#[derive(Clone, Copy, Debug)]
pub struct AccountStateCodec;
const ACCOUNTSTATE_TAG_0: u8 = 0;
const ACCOUNTSTATE_TAG_1: u8 = 1;
const ACCOUNTSTATE_TAG_2: u8 = 2;
impl Encoder<AccountState> for AccountStateCodec {
    fn encoded_len(&self, value: &AccountState) -> Result<usize, Error> {
        let size = match value {
            AccountState::Uninitialized => 0usize,
            AccountState::Initialized => 0usize,
            AccountState::Frozen => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &AccountState, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            AccountState::Uninitialized => {
                Number::<u8>::LE.write(&ACCOUNTSTATE_TAG_0, writer)?;
                Ok(())
            }
            AccountState::Initialized => {
                Number::<u8>::LE.write(&ACCOUNTSTATE_TAG_1, writer)?;
                Ok(())
            }
            AccountState::Frozen => {
                Number::<u8>::LE.write(&ACCOUNTSTATE_TAG_2, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<AccountState> for AccountStateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AccountState, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            ACCOUNTSTATE_TAG_0 => Ok(AccountState::Uninitialized),
            ACCOUNTSTATE_TAG_1 => Ok(AccountState::Initialized),
            ACCOUNTSTATE_TAG_2 => Ok(AccountState::Frozen),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, PartialEq)]
pub enum AuthorityType {
    /// The `MintTokens` variant.
    MintTokens,
    /// The `FreezeAccount` variant.
    FreezeAccount,
    /// The `AccountOwner` variant.
    AccountOwner,
    /// The `CloseAccount` variant.
    CloseAccount,
    /// The `TransferFeeConfig` variant.
    TransferFeeConfig,
    /// The `WithheldWithdraw` variant.
    WithheldWithdraw,
    /// The `CloseMint` variant.
    CloseMint,
    /// The `InterestRate` variant.
    InterestRate,
    /// The `PermanentDelegate` variant.
    PermanentDelegate,
    /// The `ConfidentialTransferMint` variant.
    ConfidentialTransferMint,
    /// The `TransferHookProgramId` variant.
    TransferHookProgramId,
    /// The `ConfidentialTransferFeeConfig` variant.
    ConfidentialTransferFeeConfig,
    /// The `MetadataPointer` variant.
    MetadataPointer,
    /// The `GroupPointer` variant.
    GroupPointer,
    /// The `GroupMemberPointer` variant.
    GroupMemberPointer,
    /// The `ScaledUiAmount` variant.
    ScaledUiAmount,
    /// The `Pause` variant.
    Pause,
    /// The `PermissionedBurn` variant.
    PermissionedBurn,
}
/// Bounded codec for [`AuthorityType`].
#[derive(Clone, Copy, Debug)]
pub struct AuthorityTypeCodec;
const AUTHORITYTYPE_TAG_0: u8 = 0;
const AUTHORITYTYPE_TAG_1: u8 = 1;
const AUTHORITYTYPE_TAG_2: u8 = 2;
const AUTHORITYTYPE_TAG_3: u8 = 3;
const AUTHORITYTYPE_TAG_4: u8 = 4;
const AUTHORITYTYPE_TAG_5: u8 = 5;
const AUTHORITYTYPE_TAG_6: u8 = 6;
const AUTHORITYTYPE_TAG_7: u8 = 7;
const AUTHORITYTYPE_TAG_8: u8 = 8;
const AUTHORITYTYPE_TAG_9: u8 = 9;
const AUTHORITYTYPE_TAG_10: u8 = 10;
const AUTHORITYTYPE_TAG_11: u8 = 11;
const AUTHORITYTYPE_TAG_12: u8 = 12;
const AUTHORITYTYPE_TAG_13: u8 = 13;
const AUTHORITYTYPE_TAG_14: u8 = 14;
const AUTHORITYTYPE_TAG_15: u8 = 15;
const AUTHORITYTYPE_TAG_16: u8 = 16;
const AUTHORITYTYPE_TAG_17: u8 = 17;
impl Encoder<AuthorityType> for AuthorityTypeCodec {
    fn encoded_len(&self, value: &AuthorityType) -> Result<usize, Error> {
        let size = match value {
            AuthorityType::MintTokens => 0usize,
            AuthorityType::FreezeAccount => 0usize,
            AuthorityType::AccountOwner => 0usize,
            AuthorityType::CloseAccount => 0usize,
            AuthorityType::TransferFeeConfig => 0usize,
            AuthorityType::WithheldWithdraw => 0usize,
            AuthorityType::CloseMint => 0usize,
            AuthorityType::InterestRate => 0usize,
            AuthorityType::PermanentDelegate => 0usize,
            AuthorityType::ConfidentialTransferMint => 0usize,
            AuthorityType::TransferHookProgramId => 0usize,
            AuthorityType::ConfidentialTransferFeeConfig => 0usize,
            AuthorityType::MetadataPointer => 0usize,
            AuthorityType::GroupPointer => 0usize,
            AuthorityType::GroupMemberPointer => 0usize,
            AuthorityType::ScaledUiAmount => 0usize,
            AuthorityType::Pause => 0usize,
            AuthorityType::PermissionedBurn => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &AuthorityType, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            AuthorityType::MintTokens => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_0, writer)?;
                Ok(())
            }
            AuthorityType::FreezeAccount => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_1, writer)?;
                Ok(())
            }
            AuthorityType::AccountOwner => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_2, writer)?;
                Ok(())
            }
            AuthorityType::CloseAccount => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_3, writer)?;
                Ok(())
            }
            AuthorityType::TransferFeeConfig => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_4, writer)?;
                Ok(())
            }
            AuthorityType::WithheldWithdraw => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_5, writer)?;
                Ok(())
            }
            AuthorityType::CloseMint => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_6, writer)?;
                Ok(())
            }
            AuthorityType::InterestRate => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_7, writer)?;
                Ok(())
            }
            AuthorityType::PermanentDelegate => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_8, writer)?;
                Ok(())
            }
            AuthorityType::ConfidentialTransferMint => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_9, writer)?;
                Ok(())
            }
            AuthorityType::TransferHookProgramId => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_10, writer)?;
                Ok(())
            }
            AuthorityType::ConfidentialTransferFeeConfig => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_11, writer)?;
                Ok(())
            }
            AuthorityType::MetadataPointer => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_12, writer)?;
                Ok(())
            }
            AuthorityType::GroupPointer => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_13, writer)?;
                Ok(())
            }
            AuthorityType::GroupMemberPointer => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_14, writer)?;
                Ok(())
            }
            AuthorityType::ScaledUiAmount => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_15, writer)?;
                Ok(())
            }
            AuthorityType::Pause => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_16, writer)?;
                Ok(())
            }
            AuthorityType::PermissionedBurn => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_17, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<AuthorityType> for AuthorityTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AuthorityType, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            AUTHORITYTYPE_TAG_0 => Ok(AuthorityType::MintTokens),
            AUTHORITYTYPE_TAG_1 => Ok(AuthorityType::FreezeAccount),
            AUTHORITYTYPE_TAG_2 => Ok(AuthorityType::AccountOwner),
            AUTHORITYTYPE_TAG_3 => Ok(AuthorityType::CloseAccount),
            AUTHORITYTYPE_TAG_4 => Ok(AuthorityType::TransferFeeConfig),
            AUTHORITYTYPE_TAG_5 => Ok(AuthorityType::WithheldWithdraw),
            AUTHORITYTYPE_TAG_6 => Ok(AuthorityType::CloseMint),
            AUTHORITYTYPE_TAG_7 => Ok(AuthorityType::InterestRate),
            AUTHORITYTYPE_TAG_8 => Ok(AuthorityType::PermanentDelegate),
            AUTHORITYTYPE_TAG_9 => Ok(AuthorityType::ConfidentialTransferMint),
            AUTHORITYTYPE_TAG_10 => Ok(AuthorityType::TransferHookProgramId),
            AUTHORITYTYPE_TAG_11 => Ok(AuthorityType::ConfidentialTransferFeeConfig),
            AUTHORITYTYPE_TAG_12 => Ok(AuthorityType::MetadataPointer),
            AUTHORITYTYPE_TAG_13 => Ok(AuthorityType::GroupPointer),
            AUTHORITYTYPE_TAG_14 => Ok(AuthorityType::GroupMemberPointer),
            AUTHORITYTYPE_TAG_15 => Ok(AuthorityType::ScaledUiAmount),
            AUTHORITYTYPE_TAG_16 => Ok(AuthorityType::Pause),
            AUTHORITYTYPE_TAG_17 => Ok(AuthorityType::PermissionedBurn),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `TransferFee`.
#[derive(Clone, Debug, PartialEq)]
pub struct TransferFee {
    /// The `epoch` wire field.
    pub epoch: u64,
    /// The `maximum_fee` wire field.
    pub maximum_fee: u64,
    /// The `transfer_fee_basis_points` wire field.
    pub transfer_fee_basis_points: u16,
}
/// Bounded codec for [`TransferFee`].
#[derive(Clone, Copy, Debug)]
pub struct TransferFeeCodec;
impl Encoder<TransferFee> for TransferFeeCodec {
    fn encoded_len(&self, value: &TransferFee) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.epoch)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.maximum_fee)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.transfer_fee_basis_points)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferFee, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.epoch, writer)?;
        Number::<u64>::LE.write(&value.maximum_fee, writer)?;
        Number::<u16>::LE.write(&value.transfer_fee_basis_points, writer)?;
        Ok(())
    }
}
impl Decoder<TransferFee> for TransferFeeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferFee, Error> {
        Ok(TransferFee {
            epoch: Number::<u64>::LE.read(reader)?,
            maximum_fee: Number::<u64>::LE.read(reader)?,
            transfer_fee_basis_points: Number::<u16>::LE.read(reader)?,
        })
    }
}
/// Validated data for `EncryptedBalance`.
#[derive(Clone, Debug, PartialEq)]
pub struct EncryptedBalance {
    /// The `value` wire field.
    pub value: [u8; 64],
}
/// Bounded codec for [`EncryptedBalance`].
#[derive(Clone, Copy, Debug)]
pub struct EncryptedBalanceCodec;
impl Encoder<EncryptedBalance> for EncryptedBalanceCodec {
    fn encoded_len(&self, value: &EncryptedBalance) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(FixedBytes::<64>.encoded_len(&value.value)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &EncryptedBalance, writer: &mut Writer<'_>) -> Result<(), Error> {
        FixedBytes::<64>.write(&value.value, writer)?;
        Ok(())
    }
}
impl Decoder<EncryptedBalance> for EncryptedBalanceCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EncryptedBalance, Error> {
        Ok(EncryptedBalance {
            value: FixedBytes::<64>.read(reader)?,
        })
    }
}
/// Validated data for `DecryptableBalance`.
#[derive(Clone, Debug, PartialEq)]
pub struct DecryptableBalance {
    /// The `value` wire field.
    pub value: [u8; 36],
}
/// Bounded codec for [`DecryptableBalance`].
#[derive(Clone, Copy, Debug)]
pub struct DecryptableBalanceCodec;
impl Encoder<DecryptableBalance> for DecryptableBalanceCodec {
    fn encoded_len(&self, value: &DecryptableBalance) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(FixedBytes::<36>.encoded_len(&value.value)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &DecryptableBalance, writer: &mut Writer<'_>) -> Result<(), Error> {
        FixedBytes::<36>.write(&value.value, writer)?;
        Ok(())
    }
}
impl Decoder<DecryptableBalance> for DecryptableBalanceCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DecryptableBalance, Error> {
        Ok(DecryptableBalance {
            value: FixedBytes::<36>.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionTransferFeeConfigPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTransferFeeConfigPayload {
    /// The `transfer_fee_config_authority` wire field.
    pub transfer_fee_config_authority: Address,
    /// The `withdraw_withheld_authority` wire field.
    pub withdraw_withheld_authority: Address,
    /// The `withheld_amount` wire field.
    pub withheld_amount: u64,
    /// The `older_transfer_fee` wire field.
    pub older_transfer_fee: TransferFee,
    /// The `newer_transfer_fee` wire field.
    pub newer_transfer_fee: TransferFee,
}
/// Bounded codec for [`ExtensionTransferFeeConfigPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionTransferFeeConfigPayloadCodec;
impl Encoder<ExtensionTransferFeeConfigPayload> for ExtensionTransferFeeConfigPayloadCodec {
    fn encoded_len(&self, value: &ExtensionTransferFeeConfigPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.transfer_fee_config_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.withdraw_withheld_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.withheld_amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(TransferFeeCodec.encoded_len(&value.older_transfer_fee)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(TransferFeeCodec.encoded_len(&value.newer_transfer_fee)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionTransferFeeConfigPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.transfer_fee_config_authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.withdraw_withheld_authority, writer)?;
        Number::<u64>::LE.write(&value.withheld_amount, writer)?;
        TransferFeeCodec.write(&value.older_transfer_fee, writer)?;
        TransferFeeCodec.write(&value.newer_transfer_fee, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionTransferFeeConfigPayload> for ExtensionTransferFeeConfigPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionTransferFeeConfigPayload, Error> {
        Ok(ExtensionTransferFeeConfigPayload {
            transfer_fee_config_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            withdraw_withheld_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            withheld_amount: Number::<u64>::LE.read(reader)?,
            older_transfer_fee: TransferFeeCodec.read(reader)?,
            newer_transfer_fee: TransferFeeCodec.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionTransferFeeAmountPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTransferFeeAmountPayload {
    /// The `withheld_amount` wire field.
    pub withheld_amount: u64,
}
/// Bounded codec for [`ExtensionTransferFeeAmountPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionTransferFeeAmountPayloadCodec;
impl Encoder<ExtensionTransferFeeAmountPayload> for ExtensionTransferFeeAmountPayloadCodec {
    fn encoded_len(&self, value: &ExtensionTransferFeeAmountPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.withheld_amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionTransferFeeAmountPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.withheld_amount, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionTransferFeeAmountPayload> for ExtensionTransferFeeAmountPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionTransferFeeAmountPayload, Error> {
        Ok(ExtensionTransferFeeAmountPayload {
            withheld_amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionMintCloseAuthorityPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionMintCloseAuthorityPayload {
    /// The `close_authority` wire field.
    pub close_authority: Address,
}
/// Bounded codec for [`ExtensionMintCloseAuthorityPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionMintCloseAuthorityPayloadCodec;
impl Encoder<ExtensionMintCloseAuthorityPayload> for ExtensionMintCloseAuthorityPayloadCodec {
    fn encoded_len(&self, value: &ExtensionMintCloseAuthorityPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.close_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionMintCloseAuthorityPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.close_authority, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionMintCloseAuthorityPayload> for ExtensionMintCloseAuthorityPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionMintCloseAuthorityPayload, Error> {
        Ok(ExtensionMintCloseAuthorityPayload {
            close_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `ExtensionConfidentialTransferMintPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionConfidentialTransferMintPayload {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `auto_approve_new_accounts` wire field.
    pub auto_approve_new_accounts: bool,
    /// The `auditor_elgamal_pubkey` wire field.
    pub auditor_elgamal_pubkey: Option<Address>,
}
/// Bounded codec for [`ExtensionConfidentialTransferMintPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionConfidentialTransferMintPayloadCodec;
impl Encoder<ExtensionConfidentialTransferMintPayload>
    for ExtensionConfidentialTransferMintPayloadCodec
{
    fn encoded_len(
        &self,
        value: &ExtensionConfidentialTransferMintPayload,
    ) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.auto_approve_new_accounts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.auditor_elgamal_pubkey)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionConfidentialTransferMintPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Bool.write(&value.auto_approve_new_accounts, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.auditor_elgamal_pubkey, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionConfidentialTransferMintPayload>
    for ExtensionConfidentialTransferMintPayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ExtensionConfidentialTransferMintPayload, Error> {
        Ok(ExtensionConfidentialTransferMintPayload {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            auto_approve_new_accounts: Bool.read(reader)?,
            auditor_elgamal_pubkey: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `ExtensionConfidentialTransferAccountPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionConfidentialTransferAccountPayload {
    /// The `approved` wire field.
    pub approved: bool,
    /// The `elgamal_pubkey` wire field.
    pub elgamal_pubkey: Address,
    /// The `pending_balance_low` wire field.
    pub pending_balance_low: EncryptedBalance,
    /// The `pending_balance_high` wire field.
    pub pending_balance_high: EncryptedBalance,
    /// The `available_balance` wire field.
    pub available_balance: EncryptedBalance,
    /// The `decryptable_available_balance` wire field.
    pub decryptable_available_balance: DecryptableBalance,
    /// The `allow_confidential_credits` wire field.
    pub allow_confidential_credits: bool,
    /// The `allow_non_confidential_credits` wire field.
    pub allow_non_confidential_credits: bool,
    /// The `pending_balance_credit_counter` wire field.
    pub pending_balance_credit_counter: u64,
    /// The `maximum_pending_balance_credit_counter` wire field.
    pub maximum_pending_balance_credit_counter: u64,
    /// The `expected_pending_balance_credit_counter` wire field.
    pub expected_pending_balance_credit_counter: u64,
    /// The `actual_pending_balance_credit_counter` wire field.
    pub actual_pending_balance_credit_counter: u64,
}
/// Bounded codec for [`ExtensionConfidentialTransferAccountPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionConfidentialTransferAccountPayloadCodec;
impl Encoder<ExtensionConfidentialTransferAccountPayload>
    for ExtensionConfidentialTransferAccountPayloadCodec
{
    fn encoded_len(
        &self,
        value: &ExtensionConfidentialTransferAccountPayload,
    ) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.approved)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.elgamal_pubkey)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(EncryptedBalanceCodec.encoded_len(&value.pending_balance_low)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(EncryptedBalanceCodec.encoded_len(&value.pending_balance_high)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(EncryptedBalanceCodec.encoded_len(&value.available_balance)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(DecryptableBalanceCodec.encoded_len(&value.decryptable_available_balance)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.allow_confidential_credits)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.allow_non_confidential_credits)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.pending_balance_credit_counter)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<u64>::LE.encoded_len(&value.maximum_pending_balance_credit_counter)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<u64>::LE.encoded_len(&value.expected_pending_balance_credit_counter)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<u64>::LE.encoded_len(&value.actual_pending_balance_credit_counter)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionConfidentialTransferAccountPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Bool.write(&value.approved, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.elgamal_pubkey, writer)?;
        EncryptedBalanceCodec.write(&value.pending_balance_low, writer)?;
        EncryptedBalanceCodec.write(&value.pending_balance_high, writer)?;
        EncryptedBalanceCodec.write(&value.available_balance, writer)?;
        DecryptableBalanceCodec.write(&value.decryptable_available_balance, writer)?;
        Bool.write(&value.allow_confidential_credits, writer)?;
        Bool.write(&value.allow_non_confidential_credits, writer)?;
        Number::<u64>::LE.write(&value.pending_balance_credit_counter, writer)?;
        Number::<u64>::LE.write(&value.maximum_pending_balance_credit_counter, writer)?;
        Number::<u64>::LE.write(&value.expected_pending_balance_credit_counter, writer)?;
        Number::<u64>::LE.write(&value.actual_pending_balance_credit_counter, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionConfidentialTransferAccountPayload>
    for ExtensionConfidentialTransferAccountPayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ExtensionConfidentialTransferAccountPayload, Error> {
        Ok(ExtensionConfidentialTransferAccountPayload {
            approved: Bool.read(reader)?,
            elgamal_pubkey: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            pending_balance_low: EncryptedBalanceCodec.read(reader)?,
            pending_balance_high: EncryptedBalanceCodec.read(reader)?,
            available_balance: EncryptedBalanceCodec.read(reader)?,
            decryptable_available_balance: DecryptableBalanceCodec.read(reader)?,
            allow_confidential_credits: Bool.read(reader)?,
            allow_non_confidential_credits: Bool.read(reader)?,
            pending_balance_credit_counter: Number::<u64>::LE.read(reader)?,
            maximum_pending_balance_credit_counter: Number::<u64>::LE.read(reader)?,
            expected_pending_balance_credit_counter: Number::<u64>::LE.read(reader)?,
            actual_pending_balance_credit_counter: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionDefaultAccountStatePayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionDefaultAccountStatePayload {
    /// The `state` wire field.
    pub state: AccountState,
}
/// Bounded codec for [`ExtensionDefaultAccountStatePayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionDefaultAccountStatePayloadCodec;
impl Encoder<ExtensionDefaultAccountStatePayload> for ExtensionDefaultAccountStatePayloadCodec {
    fn encoded_len(&self, value: &ExtensionDefaultAccountStatePayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(AccountStateCodec.encoded_len(&value.state)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionDefaultAccountStatePayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        AccountStateCodec.write(&value.state, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionDefaultAccountStatePayload> for ExtensionDefaultAccountStatePayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionDefaultAccountStatePayload, Error> {
        Ok(ExtensionDefaultAccountStatePayload {
            state: AccountStateCodec.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionImmutableOwnerPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionImmutableOwnerPayload {}
/// Bounded codec for [`ExtensionImmutableOwnerPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionImmutableOwnerPayloadCodec;
impl Encoder<ExtensionImmutableOwnerPayload> for ExtensionImmutableOwnerPayloadCodec {
    fn encoded_len(&self, value: &ExtensionImmutableOwnerPayload) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionImmutableOwnerPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ExtensionImmutableOwnerPayload> for ExtensionImmutableOwnerPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionImmutableOwnerPayload, Error> {
        let _ = reader;
        Ok(ExtensionImmutableOwnerPayload {})
    }
}
/// Validated data for `ExtensionMemoTransferPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionMemoTransferPayload {
    /// The `require_incoming_transfer_memos` wire field.
    pub require_incoming_transfer_memos: bool,
}
/// Bounded codec for [`ExtensionMemoTransferPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionMemoTransferPayloadCodec;
impl Encoder<ExtensionMemoTransferPayload> for ExtensionMemoTransferPayloadCodec {
    fn encoded_len(&self, value: &ExtensionMemoTransferPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.require_incoming_transfer_memos)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionMemoTransferPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Bool.write(&value.require_incoming_transfer_memos, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionMemoTransferPayload> for ExtensionMemoTransferPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionMemoTransferPayload, Error> {
        Ok(ExtensionMemoTransferPayload {
            require_incoming_transfer_memos: Bool.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionNonTransferablePayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionNonTransferablePayload {}
/// Bounded codec for [`ExtensionNonTransferablePayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionNonTransferablePayloadCodec;
impl Encoder<ExtensionNonTransferablePayload> for ExtensionNonTransferablePayloadCodec {
    fn encoded_len(&self, value: &ExtensionNonTransferablePayload) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionNonTransferablePayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ExtensionNonTransferablePayload> for ExtensionNonTransferablePayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionNonTransferablePayload, Error> {
        let _ = reader;
        Ok(ExtensionNonTransferablePayload {})
    }
}
/// Validated data for `ExtensionInterestBearingConfigPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionInterestBearingConfigPayload {
    /// The `rate_authority` wire field.
    pub rate_authority: Address,
    /// The `initialization_timestamp` wire field.
    pub initialization_timestamp: u64,
    /// The `pre_update_average_rate` wire field.
    pub pre_update_average_rate: i16,
    /// The `last_update_timestamp` wire field.
    pub last_update_timestamp: u64,
    /// The `current_rate` wire field.
    pub current_rate: i16,
}
/// Bounded codec for [`ExtensionInterestBearingConfigPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionInterestBearingConfigPayloadCodec;
impl Encoder<ExtensionInterestBearingConfigPayload> for ExtensionInterestBearingConfigPayloadCodec {
    fn encoded_len(&self, value: &ExtensionInterestBearingConfigPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.rate_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.initialization_timestamp)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i16>::LE.encoded_len(&value.pre_update_average_rate)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.last_update_timestamp)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i16>::LE.encoded_len(&value.current_rate)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionInterestBearingConfigPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.rate_authority, writer)?;
        Number::<u64>::LE.write(&value.initialization_timestamp, writer)?;
        Number::<i16>::LE.write(&value.pre_update_average_rate, writer)?;
        Number::<u64>::LE.write(&value.last_update_timestamp, writer)?;
        Number::<i16>::LE.write(&value.current_rate, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionInterestBearingConfigPayload> for ExtensionInterestBearingConfigPayloadCodec {
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ExtensionInterestBearingConfigPayload, Error> {
        Ok(ExtensionInterestBearingConfigPayload {
            rate_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            initialization_timestamp: Number::<u64>::LE.read(reader)?,
            pre_update_average_rate: Number::<i16>::LE.read(reader)?,
            last_update_timestamp: Number::<u64>::LE.read(reader)?,
            current_rate: Number::<i16>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionCpiGuardPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionCpiGuardPayload {
    /// The `lock_cpi` wire field.
    pub lock_cpi: bool,
}
/// Bounded codec for [`ExtensionCpiGuardPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionCpiGuardPayloadCodec;
impl Encoder<ExtensionCpiGuardPayload> for ExtensionCpiGuardPayloadCodec {
    fn encoded_len(&self, value: &ExtensionCpiGuardPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.lock_cpi)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionCpiGuardPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Bool.write(&value.lock_cpi, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionCpiGuardPayload> for ExtensionCpiGuardPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionCpiGuardPayload, Error> {
        Ok(ExtensionCpiGuardPayload {
            lock_cpi: Bool.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionPermanentDelegatePayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionPermanentDelegatePayload {
    /// The `delegate` wire field.
    pub delegate: Address,
}
/// Bounded codec for [`ExtensionPermanentDelegatePayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionPermanentDelegatePayloadCodec;
impl Encoder<ExtensionPermanentDelegatePayload> for ExtensionPermanentDelegatePayloadCodec {
    fn encoded_len(&self, value: &ExtensionPermanentDelegatePayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegate)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionPermanentDelegatePayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegate, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionPermanentDelegatePayload> for ExtensionPermanentDelegatePayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionPermanentDelegatePayload, Error> {
        Ok(ExtensionPermanentDelegatePayload {
            delegate: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `ExtensionNonTransferableAccountPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionNonTransferableAccountPayload {}
/// Bounded codec for [`ExtensionNonTransferableAccountPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionNonTransferableAccountPayloadCodec;
impl Encoder<ExtensionNonTransferableAccountPayload>
    for ExtensionNonTransferableAccountPayloadCodec
{
    fn encoded_len(&self, value: &ExtensionNonTransferableAccountPayload) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionNonTransferableAccountPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ExtensionNonTransferableAccountPayload>
    for ExtensionNonTransferableAccountPayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ExtensionNonTransferableAccountPayload, Error> {
        let _ = reader;
        Ok(ExtensionNonTransferableAccountPayload {})
    }
}
/// Validated data for `ExtensionTransferHookPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTransferHookPayload {
    /// The `authority` wire field.
    pub authority: Address,
    /// The `program_id` wire field.
    pub program_id: Address,
}
/// Bounded codec for [`ExtensionTransferHookPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionTransferHookPayloadCodec;
impl Encoder<ExtensionTransferHookPayload> for ExtensionTransferHookPayloadCodec {
    fn encoded_len(&self, value: &ExtensionTransferHookPayload) -> Result<usize, Error> {
        let size = 0usize;
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
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.program_id)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionTransferHookPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.program_id, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionTransferHookPayload> for ExtensionTransferHookPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionTransferHookPayload, Error> {
        Ok(ExtensionTransferHookPayload {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            program_id: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `ExtensionTransferHookAccountPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTransferHookAccountPayload {
    /// The `transferring` wire field.
    pub transferring: bool,
}
/// Bounded codec for [`ExtensionTransferHookAccountPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionTransferHookAccountPayloadCodec;
impl Encoder<ExtensionTransferHookAccountPayload> for ExtensionTransferHookAccountPayloadCodec {
    fn encoded_len(&self, value: &ExtensionTransferHookAccountPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.transferring)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionTransferHookAccountPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Bool.write(&value.transferring, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionTransferHookAccountPayload> for ExtensionTransferHookAccountPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionTransferHookAccountPayload, Error> {
        Ok(ExtensionTransferHookAccountPayload {
            transferring: Bool.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionConfidentialTransferFeePayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionConfidentialTransferFeePayload {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `elgamal_pubkey` wire field.
    pub elgamal_pubkey: Address,
    /// The `harvest_to_mint_enabled` wire field.
    pub harvest_to_mint_enabled: bool,
    /// The `withheld_amount` wire field.
    pub withheld_amount: EncryptedBalance,
}
/// Bounded codec for [`ExtensionConfidentialTransferFeePayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionConfidentialTransferFeePayloadCodec;
impl Encoder<ExtensionConfidentialTransferFeePayload>
    for ExtensionConfidentialTransferFeePayloadCodec
{
    fn encoded_len(&self, value: &ExtensionConfidentialTransferFeePayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.elgamal_pubkey)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.harvest_to_mint_enabled)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(EncryptedBalanceCodec.encoded_len(&value.withheld_amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionConfidentialTransferFeePayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.elgamal_pubkey, writer)?;
        Bool.write(&value.harvest_to_mint_enabled, writer)?;
        EncryptedBalanceCodec.write(&value.withheld_amount, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionConfidentialTransferFeePayload>
    for ExtensionConfidentialTransferFeePayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ExtensionConfidentialTransferFeePayload, Error> {
        Ok(ExtensionConfidentialTransferFeePayload {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            elgamal_pubkey: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            harvest_to_mint_enabled: Bool.read(reader)?,
            withheld_amount: EncryptedBalanceCodec.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionConfidentialTransferFeeAmountPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionConfidentialTransferFeeAmountPayload {
    /// The `withheld_amount` wire field.
    pub withheld_amount: EncryptedBalance,
}
/// Bounded codec for [`ExtensionConfidentialTransferFeeAmountPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionConfidentialTransferFeeAmountPayloadCodec;
impl Encoder<ExtensionConfidentialTransferFeeAmountPayload>
    for ExtensionConfidentialTransferFeeAmountPayloadCodec
{
    fn encoded_len(
        &self,
        value: &ExtensionConfidentialTransferFeeAmountPayload,
    ) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(EncryptedBalanceCodec.encoded_len(&value.withheld_amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionConfidentialTransferFeeAmountPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        EncryptedBalanceCodec.write(&value.withheld_amount, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionConfidentialTransferFeeAmountPayload>
    for ExtensionConfidentialTransferFeeAmountPayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ExtensionConfidentialTransferFeeAmountPayload, Error> {
        Ok(ExtensionConfidentialTransferFeeAmountPayload {
            withheld_amount: EncryptedBalanceCodec.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionMetadataPointerPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionMetadataPointerPayload {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `metadata_address` wire field.
    pub metadata_address: Option<Address>,
}
/// Bounded codec for [`ExtensionMetadataPointerPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionMetadataPointerPayloadCodec;
impl Encoder<ExtensionMetadataPointerPayload> for ExtensionMetadataPointerPayloadCodec {
    fn encoded_len(&self, value: &ExtensionMetadataPointerPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.metadata_address)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionMetadataPointerPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.metadata_address, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionMetadataPointerPayload> for ExtensionMetadataPointerPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionMetadataPointerPayload, Error> {
        Ok(ExtensionMetadataPointerPayload {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            metadata_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `ExtensionTokenMetadataPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTokenMetadataPayload {
    /// The `update_authority` wire field.
    pub update_authority: Option<Address>,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `name` wire field.
    pub name: String,
    /// The `symbol` wire field.
    pub symbol: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `additional_metadata` wire field.
    pub additional_metadata: Vec<(String, String)>,
}
/// Bounded codec for [`ExtensionTokenMetadataPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionTokenMetadataPayloadCodec;
impl Encoder<ExtensionTokenMetadataPayload> for ExtensionTokenMetadataPayloadCodec {
    fn encoded_len(&self, value: &ExtensionTokenMetadataPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.update_authority)?,
            )
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
                .encoded_len(value.symbol.as_str())?,
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
                    item: Tuple(
                        OwnedString(ByteSizePrefix {
                            item: Utf8,
                            prefix: Mapped::new(
                                Number::<u32>::LE,
                                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                                |value: u32| Ok::<_, Error>(u64::from(value)),
                            ),
                            consumption: FrameConsumption::Exact,
                        }),
                        OwnedString(ByteSizePrefix {
                            item: Utf8,
                            prefix: Mapped::new(
                                Number::<u32>::LE,
                                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                                |value: u32| Ok::<_, Error>(u64::from(value)),
                            ),
                            consumption: FrameConsumption::Exact,
                        }),
                    ),
                    minimum: 8,
                }
                .encoded_len(value.additional_metadata.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionTokenMetadataPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.update_authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
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
        .write(value.symbol.as_str(), writer)?;
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
            item: Tuple(
                OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }),
                OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }),
            ),
            minimum: 8,
        }
        .write(value.additional_metadata.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionTokenMetadataPayload> for ExtensionTokenMetadataPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionTokenMetadataPayload, Error> {
        Ok(ExtensionTokenMetadataPayload {
            update_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
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
            additional_metadata: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Tuple(
                    OwnedString(ByteSizePrefix {
                        item: Utf8,
                        prefix: Mapped::new(
                            Number::<u32>::LE,
                            |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                            |value: u32| Ok::<_, Error>(u64::from(value)),
                        ),
                        consumption: FrameConsumption::Exact,
                    }),
                    OwnedString(ByteSizePrefix {
                        item: Utf8,
                        prefix: Mapped::new(
                            Number::<u32>::LE,
                            |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                            |value: u32| Ok::<_, Error>(u64::from(value)),
                        ),
                        consumption: FrameConsumption::Exact,
                    }),
                ),
                minimum: 8,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `ExtensionGroupPointerPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionGroupPointerPayload {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `group_address` wire field.
    pub group_address: Option<Address>,
}
/// Bounded codec for [`ExtensionGroupPointerPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionGroupPointerPayloadCodec;
impl Encoder<ExtensionGroupPointerPayload> for ExtensionGroupPointerPayloadCodec {
    fn encoded_len(&self, value: &ExtensionGroupPointerPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.group_address)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionGroupPointerPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.group_address, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionGroupPointerPayload> for ExtensionGroupPointerPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionGroupPointerPayload, Error> {
        Ok(ExtensionGroupPointerPayload {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            group_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `ExtensionTokenGroupPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTokenGroupPayload {
    /// The `update_authority` wire field.
    pub update_authority: Option<Address>,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `size` wire field.
    pub size: u64,
    /// The `max_size` wire field.
    pub max_size: u64,
}
/// Bounded codec for [`ExtensionTokenGroupPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionTokenGroupPayloadCodec;
impl Encoder<ExtensionTokenGroupPayload> for ExtensionTokenGroupPayloadCodec {
    fn encoded_len(&self, value: &ExtensionTokenGroupPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.update_authority)?,
            )
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
            .checked_add(Number::<u64>::LE.encoded_len(&value.size)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.max_size)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionTokenGroupPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.update_authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Number::<u64>::LE.write(&value.size, writer)?;
        Number::<u64>::LE.write(&value.max_size, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionTokenGroupPayload> for ExtensionTokenGroupPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionTokenGroupPayload, Error> {
        Ok(ExtensionTokenGroupPayload {
            update_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            size: Number::<u64>::LE.read(reader)?,
            max_size: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionGroupMemberPointerPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionGroupMemberPointerPayload {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `member_address` wire field.
    pub member_address: Option<Address>,
}
/// Bounded codec for [`ExtensionGroupMemberPointerPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionGroupMemberPointerPayloadCodec;
impl Encoder<ExtensionGroupMemberPointerPayload> for ExtensionGroupMemberPointerPayloadCodec {
    fn encoded_len(&self, value: &ExtensionGroupMemberPointerPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.member_address)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionGroupMemberPointerPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.member_address, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionGroupMemberPointerPayload> for ExtensionGroupMemberPointerPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionGroupMemberPointerPayload, Error> {
        Ok(ExtensionGroupMemberPointerPayload {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            member_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `ExtensionTokenGroupMemberPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTokenGroupMemberPayload {
    /// The `mint` wire field.
    pub mint: Address,
    /// The `group` wire field.
    pub group: Address,
    /// The `member_number` wire field.
    pub member_number: u64,
}
/// Bounded codec for [`ExtensionTokenGroupMemberPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionTokenGroupMemberPayloadCodec;
impl Encoder<ExtensionTokenGroupMemberPayload> for ExtensionTokenGroupMemberPayloadCodec {
    fn encoded_len(&self, value: &ExtensionTokenGroupMemberPayload) -> Result<usize, Error> {
        let size = 0usize;
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
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.group)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.member_number)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionTokenGroupMemberPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.group, writer)?;
        Number::<u64>::LE.write(&value.member_number, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionTokenGroupMemberPayload> for ExtensionTokenGroupMemberPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionTokenGroupMemberPayload, Error> {
        Ok(ExtensionTokenGroupMemberPayload {
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            group: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            member_number: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionConfidentialMintBurnPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionConfidentialMintBurnPayload {
    /// The `confidential_supply` wire field.
    pub confidential_supply: EncryptedBalance,
    /// The `decryptable_supply` wire field.
    pub decryptable_supply: DecryptableBalance,
    /// The `supply_elgamal_pubkey` wire field.
    pub supply_elgamal_pubkey: Address,
    /// The `pending_burn` wire field.
    pub pending_burn: EncryptedBalance,
}
/// Bounded codec for [`ExtensionConfidentialMintBurnPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionConfidentialMintBurnPayloadCodec;
impl Encoder<ExtensionConfidentialMintBurnPayload> for ExtensionConfidentialMintBurnPayloadCodec {
    fn encoded_len(&self, value: &ExtensionConfidentialMintBurnPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(EncryptedBalanceCodec.encoded_len(&value.confidential_supply)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(DecryptableBalanceCodec.encoded_len(&value.decryptable_supply)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.supply_elgamal_pubkey)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(EncryptedBalanceCodec.encoded_len(&value.pending_burn)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionConfidentialMintBurnPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        EncryptedBalanceCodec.write(&value.confidential_supply, writer)?;
        DecryptableBalanceCodec.write(&value.decryptable_supply, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.supply_elgamal_pubkey, writer)?;
        EncryptedBalanceCodec.write(&value.pending_burn, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionConfidentialMintBurnPayload> for ExtensionConfidentialMintBurnPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionConfidentialMintBurnPayload, Error> {
        Ok(ExtensionConfidentialMintBurnPayload {
            confidential_supply: EncryptedBalanceCodec.read(reader)?,
            decryptable_supply: DecryptableBalanceCodec.read(reader)?,
            supply_elgamal_pubkey: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            pending_burn: EncryptedBalanceCodec.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionScaledUiAmountConfigPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionScaledUiAmountConfigPayload {
    /// The `authority` wire field.
    pub authority: Address,
    /// The `multiplier` wire field.
    pub multiplier: f64,
    /// The `new_multiplier_effective_timestamp` wire field.
    pub new_multiplier_effective_timestamp: u64,
    /// The `new_multiplier` wire field.
    pub new_multiplier: f64,
}
/// Bounded codec for [`ExtensionScaledUiAmountConfigPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionScaledUiAmountConfigPayloadCodec;
impl Encoder<ExtensionScaledUiAmountConfigPayload> for ExtensionScaledUiAmountConfigPayloadCodec {
    fn encoded_len(&self, value: &ExtensionScaledUiAmountConfigPayload) -> Result<usize, Error> {
        let size = 0usize;
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
            .checked_add(Number::<f64>::LE.encoded_len(&value.multiplier)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.new_multiplier_effective_timestamp)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<f64>::LE.encoded_len(&value.new_multiplier)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionScaledUiAmountConfigPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.authority, writer)?;
        Number::<f64>::LE.write(&value.multiplier, writer)?;
        Number::<u64>::LE.write(&value.new_multiplier_effective_timestamp, writer)?;
        Number::<f64>::LE.write(&value.new_multiplier, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionScaledUiAmountConfigPayload> for ExtensionScaledUiAmountConfigPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionScaledUiAmountConfigPayload, Error> {
        Ok(ExtensionScaledUiAmountConfigPayload {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            multiplier: Number::<f64>::LE.read(reader)?,
            new_multiplier_effective_timestamp: Number::<u64>::LE.read(reader)?,
            new_multiplier: Number::<f64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionPausableConfigPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionPausableConfigPayload {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `paused` wire field.
    pub paused: bool,
}
/// Bounded codec for [`ExtensionPausableConfigPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionPausableConfigPayloadCodec;
impl Encoder<ExtensionPausableConfigPayload> for ExtensionPausableConfigPayloadCodec {
    fn encoded_len(&self, value: &ExtensionPausableConfigPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.paused)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionPausableConfigPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Bool.write(&value.paused, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionPausableConfigPayload> for ExtensionPausableConfigPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionPausableConfigPayload, Error> {
        Ok(ExtensionPausableConfigPayload {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            paused: Bool.read(reader)?,
        })
    }
}
/// Validated data for `ExtensionPausableAccountPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionPausableAccountPayload {}
/// Bounded codec for [`ExtensionPausableAccountPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionPausableAccountPayloadCodec;
impl Encoder<ExtensionPausableAccountPayload> for ExtensionPausableAccountPayloadCodec {
    fn encoded_len(&self, value: &ExtensionPausableAccountPayload) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionPausableAccountPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ExtensionPausableAccountPayload> for ExtensionPausableAccountPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionPausableAccountPayload, Error> {
        let _ = reader;
        Ok(ExtensionPausableAccountPayload {})
    }
}
/// Validated data for `ExtensionPermissionedBurnPayload`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionPermissionedBurnPayload {
    /// The `authority` wire field.
    pub authority: Option<Address>,
}
/// Bounded codec for [`ExtensionPermissionedBurnPayload`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionPermissionedBurnPayloadCodec;
impl Encoder<ExtensionPermissionedBurnPayload> for ExtensionPermissionedBurnPayloadCodec {
    fn encoded_len(&self, value: &ExtensionPermissionedBurnPayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ExtensionPermissionedBurnPayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Ok(())
    }
}
impl Decoder<ExtensionPermissionedBurnPayload> for ExtensionPermissionedBurnPayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionPermissionedBurnPayload, Error> {
        Ok(ExtensionPermissionedBurnPayload {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, PartialEq)]
pub enum Extension {
    /// The `Uninitialized` variant.
    Uninitialized,
    /// The `TransferFeeConfig` variant.
    TransferFeeConfig(ExtensionTransferFeeConfigPayload),
    /// The `TransferFeeAmount` variant.
    TransferFeeAmount(ExtensionTransferFeeAmountPayload),
    /// The `MintCloseAuthority` variant.
    MintCloseAuthority(ExtensionMintCloseAuthorityPayload),
    /// The `ConfidentialTransferMint` variant.
    ConfidentialTransferMint(ExtensionConfidentialTransferMintPayload),
    /// The `ConfidentialTransferAccount` variant.
    ConfidentialTransferAccount(ExtensionConfidentialTransferAccountPayload),
    /// The `DefaultAccountState` variant.
    DefaultAccountState(ExtensionDefaultAccountStatePayload),
    /// The `ImmutableOwner` variant.
    ImmutableOwner(ExtensionImmutableOwnerPayload),
    /// The `MemoTransfer` variant.
    MemoTransfer(ExtensionMemoTransferPayload),
    /// The `NonTransferable` variant.
    NonTransferable(ExtensionNonTransferablePayload),
    /// The `InterestBearingConfig` variant.
    InterestBearingConfig(ExtensionInterestBearingConfigPayload),
    /// The `CpiGuard` variant.
    CpiGuard(ExtensionCpiGuardPayload),
    /// The `PermanentDelegate` variant.
    PermanentDelegate(ExtensionPermanentDelegatePayload),
    /// The `NonTransferableAccount` variant.
    NonTransferableAccount(ExtensionNonTransferableAccountPayload),
    /// The `TransferHook` variant.
    TransferHook(ExtensionTransferHookPayload),
    /// The `TransferHookAccount` variant.
    TransferHookAccount(ExtensionTransferHookAccountPayload),
    /// The `ConfidentialTransferFee` variant.
    ConfidentialTransferFee(ExtensionConfidentialTransferFeePayload),
    /// The `ConfidentialTransferFeeAmount` variant.
    ConfidentialTransferFeeAmount(ExtensionConfidentialTransferFeeAmountPayload),
    /// The `MetadataPointer` variant.
    MetadataPointer(ExtensionMetadataPointerPayload),
    /// The `TokenMetadata` variant.
    TokenMetadata(ExtensionTokenMetadataPayload),
    /// The `GroupPointer` variant.
    GroupPointer(ExtensionGroupPointerPayload),
    /// The `TokenGroup` variant.
    TokenGroup(ExtensionTokenGroupPayload),
    /// The `GroupMemberPointer` variant.
    GroupMemberPointer(ExtensionGroupMemberPointerPayload),
    /// The `TokenGroupMember` variant.
    TokenGroupMember(ExtensionTokenGroupMemberPayload),
    /// The `ConfidentialMintBurn` variant.
    ConfidentialMintBurn(ExtensionConfidentialMintBurnPayload),
    /// The `ScaledUiAmountConfig` variant.
    ScaledUiAmountConfig(ExtensionScaledUiAmountConfigPayload),
    /// The `PausableConfig` variant.
    PausableConfig(ExtensionPausableConfigPayload),
    /// The `PausableAccount` variant.
    PausableAccount(ExtensionPausableAccountPayload),
    /// The `PermissionedBurn` variant.
    PermissionedBurn(ExtensionPermissionedBurnPayload),
}
/// Bounded codec for [`Extension`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionCodec;
const EXTENSION_TAG_0: u16 = 0;
const EXTENSION_TAG_1: u16 = 1;
const EXTENSION_TAG_2: u16 = 2;
const EXTENSION_TAG_3: u16 = 3;
const EXTENSION_TAG_4: u16 = 4;
const EXTENSION_TAG_5: u16 = 5;
const EXTENSION_TAG_6: u16 = 6;
const EXTENSION_TAG_7: u16 = 7;
const EXTENSION_TAG_8: u16 = 8;
const EXTENSION_TAG_9: u16 = 9;
const EXTENSION_TAG_10: u16 = 10;
const EXTENSION_TAG_11: u16 = 11;
const EXTENSION_TAG_12: u16 = 12;
const EXTENSION_TAG_13: u16 = 13;
const EXTENSION_TAG_14: u16 = 14;
const EXTENSION_TAG_15: u16 = 15;
const EXTENSION_TAG_16: u16 = 16;
const EXTENSION_TAG_17: u16 = 17;
const EXTENSION_TAG_18: u16 = 18;
const EXTENSION_TAG_19: u16 = 19;
const EXTENSION_TAG_20: u16 = 20;
const EXTENSION_TAG_21: u16 = 21;
const EXTENSION_TAG_22: u16 = 22;
const EXTENSION_TAG_23: u16 = 23;
const EXTENSION_TAG_24: u16 = 24;
const EXTENSION_TAG_25: u16 = 25;
const EXTENSION_TAG_26: u16 = 26;
const EXTENSION_TAG_27: u16 = 27;
const EXTENSION_TAG_28: u16 = 28;
impl Encoder<Extension> for ExtensionCodec {
    fn encoded_len(&self, value: &Extension) -> Result<usize, Error> {
        let size = match value {
            Extension::Uninitialized => 0usize,
            Extension::TransferFeeConfig(value) => ByteSizePrefix {
                item: ExtensionTransferFeeConfigPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::TransferFeeAmount(value) => ByteSizePrefix {
                item: ExtensionTransferFeeAmountPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::MintCloseAuthority(value) => ByteSizePrefix {
                item: ExtensionMintCloseAuthorityPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::ConfidentialTransferMint(value) => ByteSizePrefix {
                item: ExtensionConfidentialTransferMintPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::ConfidentialTransferAccount(value) => ByteSizePrefix {
                item: ExtensionConfidentialTransferAccountPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::DefaultAccountState(value) => ByteSizePrefix {
                item: ExtensionDefaultAccountStatePayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::ImmutableOwner(value) => ByteSizePrefix {
                item: ExtensionImmutableOwnerPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::MemoTransfer(value) => ByteSizePrefix {
                item: ExtensionMemoTransferPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::NonTransferable(value) => ByteSizePrefix {
                item: ExtensionNonTransferablePayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::InterestBearingConfig(value) => ByteSizePrefix {
                item: ExtensionInterestBearingConfigPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::CpiGuard(value) => ByteSizePrefix {
                item: ExtensionCpiGuardPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::PermanentDelegate(value) => ByteSizePrefix {
                item: ExtensionPermanentDelegatePayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::NonTransferableAccount(value) => ByteSizePrefix {
                item: ExtensionNonTransferableAccountPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::TransferHook(value) => ByteSizePrefix {
                item: ExtensionTransferHookPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::TransferHookAccount(value) => ByteSizePrefix {
                item: ExtensionTransferHookAccountPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::ConfidentialTransferFee(value) => ByteSizePrefix {
                item: ExtensionConfidentialTransferFeePayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::ConfidentialTransferFeeAmount(value) => ByteSizePrefix {
                item: ExtensionConfidentialTransferFeeAmountPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::MetadataPointer(value) => ByteSizePrefix {
                item: ExtensionMetadataPointerPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::TokenMetadata(value) => ByteSizePrefix {
                item: ExtensionTokenMetadataPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::GroupPointer(value) => ByteSizePrefix {
                item: ExtensionGroupPointerPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::TokenGroup(value) => ByteSizePrefix {
                item: ExtensionTokenGroupPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::GroupMemberPointer(value) => ByteSizePrefix {
                item: ExtensionGroupMemberPointerPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::TokenGroupMember(value) => ByteSizePrefix {
                item: ExtensionTokenGroupMemberPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::ConfidentialMintBurn(value) => ByteSizePrefix {
                item: ExtensionConfidentialMintBurnPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::ScaledUiAmountConfig(value) => ByteSizePrefix {
                item: ExtensionScaledUiAmountConfigPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::PausableConfig(value) => ByteSizePrefix {
                item: ExtensionPausableConfigPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::PausableAccount(value) => ByteSizePrefix {
                item: ExtensionPausableAccountPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
            Extension::PermissionedBurn(value) => ByteSizePrefix {
                item: ExtensionPermissionedBurnPayloadCodec,
                prefix: Mapped::new(
                    Number::<u16>::LE,
                    |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u16>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &Extension, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            Extension::Uninitialized => {
                Number::<u16>::LE.write(&EXTENSION_TAG_0, writer)?;
                Ok(())
            }
            Extension::TransferFeeConfig(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_1, writer)?;
                ByteSizePrefix {
                    item: ExtensionTransferFeeConfigPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::TransferFeeAmount(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_2, writer)?;
                ByteSizePrefix {
                    item: ExtensionTransferFeeAmountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::MintCloseAuthority(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_3, writer)?;
                ByteSizePrefix {
                    item: ExtensionMintCloseAuthorityPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::ConfidentialTransferMint(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_4, writer)?;
                ByteSizePrefix {
                    item: ExtensionConfidentialTransferMintPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::ConfidentialTransferAccount(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_5, writer)?;
                ByteSizePrefix {
                    item: ExtensionConfidentialTransferAccountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::DefaultAccountState(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_6, writer)?;
                ByteSizePrefix {
                    item: ExtensionDefaultAccountStatePayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::ImmutableOwner(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_7, writer)?;
                ByteSizePrefix {
                    item: ExtensionImmutableOwnerPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::MemoTransfer(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_8, writer)?;
                ByteSizePrefix {
                    item: ExtensionMemoTransferPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::NonTransferable(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_9, writer)?;
                ByteSizePrefix {
                    item: ExtensionNonTransferablePayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::InterestBearingConfig(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_10, writer)?;
                ByteSizePrefix {
                    item: ExtensionInterestBearingConfigPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::CpiGuard(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_11, writer)?;
                ByteSizePrefix {
                    item: ExtensionCpiGuardPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::PermanentDelegate(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_12, writer)?;
                ByteSizePrefix {
                    item: ExtensionPermanentDelegatePayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::NonTransferableAccount(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_13, writer)?;
                ByteSizePrefix {
                    item: ExtensionNonTransferableAccountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::TransferHook(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_14, writer)?;
                ByteSizePrefix {
                    item: ExtensionTransferHookPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::TransferHookAccount(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_15, writer)?;
                ByteSizePrefix {
                    item: ExtensionTransferHookAccountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::ConfidentialTransferFee(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_16, writer)?;
                ByteSizePrefix {
                    item: ExtensionConfidentialTransferFeePayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::ConfidentialTransferFeeAmount(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_17, writer)?;
                ByteSizePrefix {
                    item: ExtensionConfidentialTransferFeeAmountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::MetadataPointer(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_18, writer)?;
                ByteSizePrefix {
                    item: ExtensionMetadataPointerPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::TokenMetadata(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_19, writer)?;
                ByteSizePrefix {
                    item: ExtensionTokenMetadataPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::GroupPointer(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_20, writer)?;
                ByteSizePrefix {
                    item: ExtensionGroupPointerPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::TokenGroup(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_21, writer)?;
                ByteSizePrefix {
                    item: ExtensionTokenGroupPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::GroupMemberPointer(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_22, writer)?;
                ByteSizePrefix {
                    item: ExtensionGroupMemberPointerPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::TokenGroupMember(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_23, writer)?;
                ByteSizePrefix {
                    item: ExtensionTokenGroupMemberPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::ConfidentialMintBurn(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_24, writer)?;
                ByteSizePrefix {
                    item: ExtensionConfidentialMintBurnPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::ScaledUiAmountConfig(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_25, writer)?;
                ByteSizePrefix {
                    item: ExtensionScaledUiAmountConfigPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::PausableConfig(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_26, writer)?;
                ByteSizePrefix {
                    item: ExtensionPausableConfigPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::PausableAccount(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_27, writer)?;
                ByteSizePrefix {
                    item: ExtensionPausableAccountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
            Extension::PermissionedBurn(value) => {
                Number::<u16>::LE.write(&EXTENSION_TAG_28, writer)?;
                ByteSizePrefix {
                    item: ExtensionPermissionedBurnPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value, writer)
            }
        }
    }
}
impl Decoder<Extension> for ExtensionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Extension, Error> {
        reader.transaction(|reader| match Number::<u16>::LE.read(reader)? {
            EXTENSION_TAG_0 => Ok(Extension::Uninitialized),
            EXTENSION_TAG_1 => Ok(Extension::TransferFeeConfig(
                ByteSizePrefix {
                    item: ExtensionTransferFeeConfigPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_2 => Ok(Extension::TransferFeeAmount(
                ByteSizePrefix {
                    item: ExtensionTransferFeeAmountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_3 => Ok(Extension::MintCloseAuthority(
                ByteSizePrefix {
                    item: ExtensionMintCloseAuthorityPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_4 => Ok(Extension::ConfidentialTransferMint(
                ByteSizePrefix {
                    item: ExtensionConfidentialTransferMintPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_5 => Ok(Extension::ConfidentialTransferAccount(
                ByteSizePrefix {
                    item: ExtensionConfidentialTransferAccountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_6 => Ok(Extension::DefaultAccountState(
                ByteSizePrefix {
                    item: ExtensionDefaultAccountStatePayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_7 => Ok(Extension::ImmutableOwner(
                ByteSizePrefix {
                    item: ExtensionImmutableOwnerPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_8 => Ok(Extension::MemoTransfer(
                ByteSizePrefix {
                    item: ExtensionMemoTransferPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_9 => Ok(Extension::NonTransferable(
                ByteSizePrefix {
                    item: ExtensionNonTransferablePayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_10 => Ok(Extension::InterestBearingConfig(
                ByteSizePrefix {
                    item: ExtensionInterestBearingConfigPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_11 => Ok(Extension::CpiGuard(
                ByteSizePrefix {
                    item: ExtensionCpiGuardPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_12 => Ok(Extension::PermanentDelegate(
                ByteSizePrefix {
                    item: ExtensionPermanentDelegatePayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_13 => Ok(Extension::NonTransferableAccount(
                ByteSizePrefix {
                    item: ExtensionNonTransferableAccountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_14 => Ok(Extension::TransferHook(
                ByteSizePrefix {
                    item: ExtensionTransferHookPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_15 => Ok(Extension::TransferHookAccount(
                ByteSizePrefix {
                    item: ExtensionTransferHookAccountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_16 => Ok(Extension::ConfidentialTransferFee(
                ByteSizePrefix {
                    item: ExtensionConfidentialTransferFeePayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_17 => Ok(Extension::ConfidentialTransferFeeAmount(
                ByteSizePrefix {
                    item: ExtensionConfidentialTransferFeeAmountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_18 => Ok(Extension::MetadataPointer(
                ByteSizePrefix {
                    item: ExtensionMetadataPointerPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_19 => Ok(Extension::TokenMetadata(
                ByteSizePrefix {
                    item: ExtensionTokenMetadataPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_20 => Ok(Extension::GroupPointer(
                ByteSizePrefix {
                    item: ExtensionGroupPointerPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_21 => Ok(Extension::TokenGroup(
                ByteSizePrefix {
                    item: ExtensionTokenGroupPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_22 => Ok(Extension::GroupMemberPointer(
                ByteSizePrefix {
                    item: ExtensionGroupMemberPointerPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_23 => Ok(Extension::TokenGroupMember(
                ByteSizePrefix {
                    item: ExtensionTokenGroupMemberPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_24 => Ok(Extension::ConfidentialMintBurn(
                ByteSizePrefix {
                    item: ExtensionConfidentialMintBurnPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_25 => Ok(Extension::ScaledUiAmountConfig(
                ByteSizePrefix {
                    item: ExtensionScaledUiAmountConfigPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_26 => Ok(Extension::PausableConfig(
                ByteSizePrefix {
                    item: ExtensionPausableConfigPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_27 => Ok(Extension::PausableAccount(
                ByteSizePrefix {
                    item: ExtensionPausableAccountPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            EXTENSION_TAG_28 => Ok(Extension::PermissionedBurn(
                ByteSizePrefix {
                    item: ExtensionPermissionedBurnPayloadCodec,
                    prefix: Mapped::new(
                        Number::<u16>::LE,
                        |value: &u64| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, PartialEq)]
pub enum ExtensionType {
    /// The `Uninitialized` variant.
    Uninitialized,
    /// The `TransferFeeConfig` variant.
    TransferFeeConfig,
    /// The `TransferFeeAmount` variant.
    TransferFeeAmount,
    /// The `MintCloseAuthority` variant.
    MintCloseAuthority,
    /// The `ConfidentialTransferMint` variant.
    ConfidentialTransferMint,
    /// The `ConfidentialTransferAccount` variant.
    ConfidentialTransferAccount,
    /// The `DefaultAccountState` variant.
    DefaultAccountState,
    /// The `ImmutableOwner` variant.
    ImmutableOwner,
    /// The `MemoTransfer` variant.
    MemoTransfer,
    /// The `NonTransferable` variant.
    NonTransferable,
    /// The `InterestBearingConfig` variant.
    InterestBearingConfig,
    /// The `CpiGuard` variant.
    CpiGuard,
    /// The `PermanentDelegate` variant.
    PermanentDelegate,
    /// The `NonTransferableAccount` variant.
    NonTransferableAccount,
    /// The `TransferHook` variant.
    TransferHook,
    /// The `TransferHookAccount` variant.
    TransferHookAccount,
    /// The `ConfidentialTransferFee` variant.
    ConfidentialTransferFee,
    /// The `ConfidentialTransferFeeAmount` variant.
    ConfidentialTransferFeeAmount,
    /// The `MetadataPointer` variant.
    MetadataPointer,
    /// The `TokenMetadata` variant.
    TokenMetadata,
    /// The `GroupPointer` variant.
    GroupPointer,
    /// The `TokenGroup` variant.
    TokenGroup,
    /// The `GroupMemberPointer` variant.
    GroupMemberPointer,
    /// The `TokenGroupMember` variant.
    TokenGroupMember,
    /// The `ConfidentialMintBurn` variant.
    ConfidentialMintBurn,
    /// The `ScaledUiAmountConfig` variant.
    ScaledUiAmountConfig,
    /// The `PausableConfig` variant.
    PausableConfig,
    /// The `PausableAccount` variant.
    PausableAccount,
    /// The `PermissionedBurn` variant.
    PermissionedBurn,
}
/// Bounded codec for [`ExtensionType`].
#[derive(Clone, Copy, Debug)]
pub struct ExtensionTypeCodec;
const EXTENSIONTYPE_TAG_0: u16 = 0;
const EXTENSIONTYPE_TAG_1: u16 = 1;
const EXTENSIONTYPE_TAG_2: u16 = 2;
const EXTENSIONTYPE_TAG_3: u16 = 3;
const EXTENSIONTYPE_TAG_4: u16 = 4;
const EXTENSIONTYPE_TAG_5: u16 = 5;
const EXTENSIONTYPE_TAG_6: u16 = 6;
const EXTENSIONTYPE_TAG_7: u16 = 7;
const EXTENSIONTYPE_TAG_8: u16 = 8;
const EXTENSIONTYPE_TAG_9: u16 = 9;
const EXTENSIONTYPE_TAG_10: u16 = 10;
const EXTENSIONTYPE_TAG_11: u16 = 11;
const EXTENSIONTYPE_TAG_12: u16 = 12;
const EXTENSIONTYPE_TAG_13: u16 = 13;
const EXTENSIONTYPE_TAG_14: u16 = 14;
const EXTENSIONTYPE_TAG_15: u16 = 15;
const EXTENSIONTYPE_TAG_16: u16 = 16;
const EXTENSIONTYPE_TAG_17: u16 = 17;
const EXTENSIONTYPE_TAG_18: u16 = 18;
const EXTENSIONTYPE_TAG_19: u16 = 19;
const EXTENSIONTYPE_TAG_20: u16 = 20;
const EXTENSIONTYPE_TAG_21: u16 = 21;
const EXTENSIONTYPE_TAG_22: u16 = 22;
const EXTENSIONTYPE_TAG_23: u16 = 23;
const EXTENSIONTYPE_TAG_24: u16 = 24;
const EXTENSIONTYPE_TAG_25: u16 = 25;
const EXTENSIONTYPE_TAG_26: u16 = 26;
const EXTENSIONTYPE_TAG_27: u16 = 27;
const EXTENSIONTYPE_TAG_28: u16 = 28;
impl Encoder<ExtensionType> for ExtensionTypeCodec {
    fn encoded_len(&self, value: &ExtensionType) -> Result<usize, Error> {
        let size = match value {
            ExtensionType::Uninitialized => 0usize,
            ExtensionType::TransferFeeConfig => 0usize,
            ExtensionType::TransferFeeAmount => 0usize,
            ExtensionType::MintCloseAuthority => 0usize,
            ExtensionType::ConfidentialTransferMint => 0usize,
            ExtensionType::ConfidentialTransferAccount => 0usize,
            ExtensionType::DefaultAccountState => 0usize,
            ExtensionType::ImmutableOwner => 0usize,
            ExtensionType::MemoTransfer => 0usize,
            ExtensionType::NonTransferable => 0usize,
            ExtensionType::InterestBearingConfig => 0usize,
            ExtensionType::CpiGuard => 0usize,
            ExtensionType::PermanentDelegate => 0usize,
            ExtensionType::NonTransferableAccount => 0usize,
            ExtensionType::TransferHook => 0usize,
            ExtensionType::TransferHookAccount => 0usize,
            ExtensionType::ConfidentialTransferFee => 0usize,
            ExtensionType::ConfidentialTransferFeeAmount => 0usize,
            ExtensionType::MetadataPointer => 0usize,
            ExtensionType::TokenMetadata => 0usize,
            ExtensionType::GroupPointer => 0usize,
            ExtensionType::TokenGroup => 0usize,
            ExtensionType::GroupMemberPointer => 0usize,
            ExtensionType::TokenGroupMember => 0usize,
            ExtensionType::ConfidentialMintBurn => 0usize,
            ExtensionType::ScaledUiAmountConfig => 0usize,
            ExtensionType::PausableConfig => 0usize,
            ExtensionType::PausableAccount => 0usize,
            ExtensionType::PermissionedBurn => 0usize,
        };
        size.checked_add(core::mem::size_of::<u16>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &ExtensionType, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            ExtensionType::Uninitialized => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_0, writer)?;
                Ok(())
            }
            ExtensionType::TransferFeeConfig => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_1, writer)?;
                Ok(())
            }
            ExtensionType::TransferFeeAmount => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_2, writer)?;
                Ok(())
            }
            ExtensionType::MintCloseAuthority => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_3, writer)?;
                Ok(())
            }
            ExtensionType::ConfidentialTransferMint => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_4, writer)?;
                Ok(())
            }
            ExtensionType::ConfidentialTransferAccount => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_5, writer)?;
                Ok(())
            }
            ExtensionType::DefaultAccountState => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_6, writer)?;
                Ok(())
            }
            ExtensionType::ImmutableOwner => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_7, writer)?;
                Ok(())
            }
            ExtensionType::MemoTransfer => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_8, writer)?;
                Ok(())
            }
            ExtensionType::NonTransferable => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_9, writer)?;
                Ok(())
            }
            ExtensionType::InterestBearingConfig => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_10, writer)?;
                Ok(())
            }
            ExtensionType::CpiGuard => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_11, writer)?;
                Ok(())
            }
            ExtensionType::PermanentDelegate => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_12, writer)?;
                Ok(())
            }
            ExtensionType::NonTransferableAccount => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_13, writer)?;
                Ok(())
            }
            ExtensionType::TransferHook => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_14, writer)?;
                Ok(())
            }
            ExtensionType::TransferHookAccount => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_15, writer)?;
                Ok(())
            }
            ExtensionType::ConfidentialTransferFee => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_16, writer)?;
                Ok(())
            }
            ExtensionType::ConfidentialTransferFeeAmount => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_17, writer)?;
                Ok(())
            }
            ExtensionType::MetadataPointer => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_18, writer)?;
                Ok(())
            }
            ExtensionType::TokenMetadata => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_19, writer)?;
                Ok(())
            }
            ExtensionType::GroupPointer => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_20, writer)?;
                Ok(())
            }
            ExtensionType::TokenGroup => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_21, writer)?;
                Ok(())
            }
            ExtensionType::GroupMemberPointer => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_22, writer)?;
                Ok(())
            }
            ExtensionType::TokenGroupMember => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_23, writer)?;
                Ok(())
            }
            ExtensionType::ConfidentialMintBurn => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_24, writer)?;
                Ok(())
            }
            ExtensionType::ScaledUiAmountConfig => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_25, writer)?;
                Ok(())
            }
            ExtensionType::PausableConfig => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_26, writer)?;
                Ok(())
            }
            ExtensionType::PausableAccount => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_27, writer)?;
                Ok(())
            }
            ExtensionType::PermissionedBurn => {
                Number::<u16>::LE.write(&EXTENSIONTYPE_TAG_28, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<ExtensionType> for ExtensionTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtensionType, Error> {
        reader.transaction(|reader| match Number::<u16>::LE.read(reader)? {
            EXTENSIONTYPE_TAG_0 => Ok(ExtensionType::Uninitialized),
            EXTENSIONTYPE_TAG_1 => Ok(ExtensionType::TransferFeeConfig),
            EXTENSIONTYPE_TAG_2 => Ok(ExtensionType::TransferFeeAmount),
            EXTENSIONTYPE_TAG_3 => Ok(ExtensionType::MintCloseAuthority),
            EXTENSIONTYPE_TAG_4 => Ok(ExtensionType::ConfidentialTransferMint),
            EXTENSIONTYPE_TAG_5 => Ok(ExtensionType::ConfidentialTransferAccount),
            EXTENSIONTYPE_TAG_6 => Ok(ExtensionType::DefaultAccountState),
            EXTENSIONTYPE_TAG_7 => Ok(ExtensionType::ImmutableOwner),
            EXTENSIONTYPE_TAG_8 => Ok(ExtensionType::MemoTransfer),
            EXTENSIONTYPE_TAG_9 => Ok(ExtensionType::NonTransferable),
            EXTENSIONTYPE_TAG_10 => Ok(ExtensionType::InterestBearingConfig),
            EXTENSIONTYPE_TAG_11 => Ok(ExtensionType::CpiGuard),
            EXTENSIONTYPE_TAG_12 => Ok(ExtensionType::PermanentDelegate),
            EXTENSIONTYPE_TAG_13 => Ok(ExtensionType::NonTransferableAccount),
            EXTENSIONTYPE_TAG_14 => Ok(ExtensionType::TransferHook),
            EXTENSIONTYPE_TAG_15 => Ok(ExtensionType::TransferHookAccount),
            EXTENSIONTYPE_TAG_16 => Ok(ExtensionType::ConfidentialTransferFee),
            EXTENSIONTYPE_TAG_17 => Ok(ExtensionType::ConfidentialTransferFeeAmount),
            EXTENSIONTYPE_TAG_18 => Ok(ExtensionType::MetadataPointer),
            EXTENSIONTYPE_TAG_19 => Ok(ExtensionType::TokenMetadata),
            EXTENSIONTYPE_TAG_20 => Ok(ExtensionType::GroupPointer),
            EXTENSIONTYPE_TAG_21 => Ok(ExtensionType::TokenGroup),
            EXTENSIONTYPE_TAG_22 => Ok(ExtensionType::GroupMemberPointer),
            EXTENSIONTYPE_TAG_23 => Ok(ExtensionType::TokenGroupMember),
            EXTENSIONTYPE_TAG_24 => Ok(ExtensionType::ConfidentialMintBurn),
            EXTENSIONTYPE_TAG_25 => Ok(ExtensionType::ScaledUiAmountConfig),
            EXTENSIONTYPE_TAG_26 => Ok(ExtensionType::PausableConfig),
            EXTENSIONTYPE_TAG_27 => Ok(ExtensionType::PausableAccount),
            EXTENSIONTYPE_TAG_28 => Ok(ExtensionType::PermissionedBurn),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenMetadataField {
    /// The `Name` variant.
    Name,
    /// The `Symbol` variant.
    Symbol,
    /// The `Uri` variant.
    Uri,
    /// The `Key` variant.
    Key(String),
}
/// Bounded codec for [`TokenMetadataField`].
#[derive(Clone, Copy, Debug)]
pub struct TokenMetadataFieldCodec;
const TOKENMETADATAFIELD_TAG_0: u8 = 0;
const TOKENMETADATAFIELD_TAG_1: u8 = 1;
const TOKENMETADATAFIELD_TAG_2: u8 = 2;
const TOKENMETADATAFIELD_TAG_3: u8 = 3;
impl Encoder<TokenMetadataField> for TokenMetadataFieldCodec {
    fn encoded_len(&self, value: &TokenMetadataField) -> Result<usize, Error> {
        let size = match value {
            TokenMetadataField::Name => 0usize,
            TokenMetadataField::Symbol => 0usize,
            TokenMetadataField::Uri => 0usize,
            TokenMetadataField::Key(value) => ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .encoded_len(value.as_str())?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &TokenMetadataField, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            TokenMetadataField::Name => {
                Number::<u8>::LE.write(&TOKENMETADATAFIELD_TAG_0, writer)?;
                Ok(())
            }
            TokenMetadataField::Symbol => {
                Number::<u8>::LE.write(&TOKENMETADATAFIELD_TAG_1, writer)?;
                Ok(())
            }
            TokenMetadataField::Uri => {
                Number::<u8>::LE.write(&TOKENMETADATAFIELD_TAG_2, writer)?;
                Ok(())
            }
            TokenMetadataField::Key(value) => {
                Number::<u8>::LE.write(&TOKENMETADATAFIELD_TAG_3, writer)?;
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .write(value.as_str(), writer)
            }
        }
    }
}
impl Decoder<TokenMetadataField> for TokenMetadataFieldCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TokenMetadataField, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            TOKENMETADATAFIELD_TAG_0 => Ok(TokenMetadataField::Name),
            TOKENMETADATAFIELD_TAG_1 => Ok(TokenMetadataField::Symbol),
            TOKENMETADATAFIELD_TAG_2 => Ok(TokenMetadataField::Uri),
            TOKENMETADATAFIELD_TAG_3 => Ok(TokenMetadataField::Key(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `BatchDataItem`.
#[derive(Clone, Debug, PartialEq)]
pub struct BatchDataItem {
    /// The `number_of_accounts` wire field.
    pub number_of_accounts: u8,
    /// The `instruction_data` wire field.
    pub instruction_data: Vec<u8>,
}
/// Bounded codec for [`BatchDataItem`].
#[derive(Clone, Copy, Debug)]
pub struct BatchDataItemCodec;
impl Encoder<BatchDataItem> for BatchDataItemCodec {
    fn encoded_len(&self, value: &BatchDataItem) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.number_of_accounts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Bytes,
                    prefix: Mapped::new(
                        Number::<u8>::LE,
                        |value: &u64| u8::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u8| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.instruction_data.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &BatchDataItem, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.number_of_accounts, writer)?;
        ByteSizePrefix {
            item: Bytes,
            prefix: Mapped::new(
                Number::<u8>::LE,
                |value: &u64| u8::try_from(*value).map_err(|_| Error::Overflow),
                |value: u8| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.instruction_data.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<BatchDataItem> for BatchDataItemCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BatchDataItem, Error> {
        Ok(BatchDataItem {
            number_of_accounts: Number::<u8>::LE.read(reader)?,
            instruction_data: ByteSizePrefix {
                item: Bytes,
                prefix: Mapped::new(
                    Number::<u8>::LE,
                    |value: &u64| u8::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u8| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
        })
    }
}
/// Every pinned instruction data variant.
#[derive(Clone, Debug, PartialEq)]
pub enum Token2022InstructionData {
    /// Data for `InitializeMint`.
    InitializeMint(InitializeMint),
    /// Data for `InitializeAccount`.
    InitializeAccount(InitializeAccount),
    /// Data for `InitializeMultisig`.
    InitializeMultisig(InitializeMultisig),
    /// Data for `Transfer`.
    Transfer(Transfer),
    /// Data for `Approve`.
    Approve(Approve),
    /// Data for `Revoke`.
    Revoke(Revoke),
    /// Data for `SetAuthority`.
    SetAuthority(SetAuthority),
    /// Data for `MintTo`.
    MintTo(MintTo),
    /// Data for `Burn`.
    Burn(Burn),
    /// Data for `CloseAccount`.
    CloseAccount(CloseAccount),
    /// Data for `FreezeAccount`.
    FreezeAccount(FreezeAccount),
    /// Data for `ThawAccount`.
    ThawAccount(ThawAccount),
    /// Data for `TransferChecked`.
    TransferChecked(TransferChecked),
    /// Data for `ApproveChecked`.
    ApproveChecked(ApproveChecked),
    /// Data for `MintToChecked`.
    MintToChecked(MintToChecked),
    /// Data for `BurnChecked`.
    BurnChecked(BurnChecked),
    /// Data for `InitializeAccount2`.
    InitializeAccount2(InitializeAccount2),
    /// Data for `SyncNative`.
    SyncNative(SyncNative),
    /// Data for `InitializeAccount3`.
    InitializeAccount3(InitializeAccount3),
    /// Data for `InitializeMultisig2`.
    InitializeMultisig2(InitializeMultisig2),
    /// Data for `InitializeMint2`.
    InitializeMint2(InitializeMint2),
    /// Data for `GetAccountDataSize`.
    GetAccountDataSize(GetAccountDataSize),
    /// Data for `InitializeImmutableOwner`.
    InitializeImmutableOwner(InitializeImmutableOwner),
    /// Data for `AmountToUiAmount`.
    AmountToUiAmount(AmountToUiAmount),
    /// Data for `UiAmountToAmount`.
    UiAmountToAmount(UiAmountToAmount),
    /// Data for `InitializeMintCloseAuthority`.
    InitializeMintCloseAuthority(InitializeMintCloseAuthority),
    /// Data for `InitializeTransferFeeConfig`.
    InitializeTransferFeeConfig(InitializeTransferFeeConfig),
    /// Data for `TransferCheckedWithFee`.
    TransferCheckedWithFee(TransferCheckedWithFee),
    /// Data for `WithdrawWithheldTokensFromMint`.
    WithdrawWithheldTokensFromMint(WithdrawWithheldTokensFromMint),
    /// Data for `WithdrawWithheldTokensFromAccounts`.
    WithdrawWithheldTokensFromAccounts(WithdrawWithheldTokensFromAccounts),
    /// Data for `HarvestWithheldTokensToMint`.
    HarvestWithheldTokensToMint(HarvestWithheldTokensToMint),
    /// Data for `SetTransferFee`.
    SetTransferFee(SetTransferFee),
    /// Data for `InitializeConfidentialTransferMint`.
    InitializeConfidentialTransferMint(InitializeConfidentialTransferMint),
    /// Data for `UpdateConfidentialTransferMint`.
    UpdateConfidentialTransferMint(UpdateConfidentialTransferMint),
    /// Data for `ConfigureConfidentialTransferAccount`.
    ConfigureConfidentialTransferAccount(ConfigureConfidentialTransferAccount),
    /// Data for `ApproveConfidentialTransferAccount`.
    ApproveConfidentialTransferAccount(ApproveConfidentialTransferAccount),
    /// Data for `EmptyConfidentialTransferAccount`.
    EmptyConfidentialTransferAccount(EmptyConfidentialTransferAccount),
    /// Data for `ConfidentialDeposit`.
    ConfidentialDeposit(ConfidentialDeposit),
    /// Data for `ConfidentialWithdraw`.
    ConfidentialWithdraw(ConfidentialWithdraw),
    /// Data for `ConfidentialTransfer`.
    ConfidentialTransfer(ConfidentialTransfer),
    /// Data for `ApplyConfidentialPendingBalance`.
    ApplyConfidentialPendingBalance(ApplyConfidentialPendingBalance),
    /// Data for `EnableConfidentialCredits`.
    EnableConfidentialCredits(EnableConfidentialCredits),
    /// Data for `DisableConfidentialCredits`.
    DisableConfidentialCredits(DisableConfidentialCredits),
    /// Data for `EnableNonConfidentialCredits`.
    EnableNonConfidentialCredits(EnableNonConfidentialCredits),
    /// Data for `DisableNonConfidentialCredits`.
    DisableNonConfidentialCredits(DisableNonConfidentialCredits),
    /// Data for `ConfidentialTransferWithFee`.
    ConfidentialTransferWithFee(ConfidentialTransferWithFee),
    /// Data for `ConfigureConfidentialTransferAccountWithRegistry`.
    ConfigureConfidentialTransferAccountWithRegistry(
        ConfigureConfidentialTransferAccountWithRegistry,
    ),
    /// Data for `InitializeDefaultAccountState`.
    InitializeDefaultAccountState(InitializeDefaultAccountState),
    /// Data for `UpdateDefaultAccountState`.
    UpdateDefaultAccountState(UpdateDefaultAccountState),
    /// Data for `Reallocate`.
    Reallocate(Reallocate),
    /// Data for `EnableMemoTransfers`.
    EnableMemoTransfers(EnableMemoTransfers),
    /// Data for `DisableMemoTransfers`.
    DisableMemoTransfers(DisableMemoTransfers),
    /// Data for `CreateNativeMint`.
    CreateNativeMint(CreateNativeMint),
    /// Data for `InitializeNonTransferableMint`.
    InitializeNonTransferableMint(InitializeNonTransferableMint),
    /// Data for `InitializeInterestBearingMint`.
    InitializeInterestBearingMint(InitializeInterestBearingMint),
    /// Data for `UpdateRateInterestBearingMint`.
    UpdateRateInterestBearingMint(UpdateRateInterestBearingMint),
    /// Data for `EnableCpiGuard`.
    EnableCpiGuard(EnableCpiGuard),
    /// Data for `DisableCpiGuard`.
    DisableCpiGuard(DisableCpiGuard),
    /// Data for `InitializePermanentDelegate`.
    InitializePermanentDelegate(InitializePermanentDelegate),
    /// Data for `InitializeTransferHook`.
    InitializeTransferHook(InitializeTransferHook),
    /// Data for `UpdateTransferHook`.
    UpdateTransferHook(UpdateTransferHook),
    /// Data for `InitializeConfidentialTransferFee`.
    InitializeConfidentialTransferFee(InitializeConfidentialTransferFee),
    /// Data for `WithdrawWithheldTokensFromMintForConfidentialTransferFee`.
    WithdrawWithheldTokensFromMintForConfidentialTransferFee(
        WithdrawWithheldTokensFromMintForConfidentialTransferFee,
    ),
    /// Data for `WithdrawWithheldTokensFromAccountsForConfidentialTransferFee`.
    WithdrawWithheldTokensFromAccountsForConfidentialTransferFee(
        WithdrawWithheldTokensFromAccountsForConfidentialTransferFee,
    ),
    /// Data for `HarvestWithheldTokensToMintForConfidentialTransferFee`.
    HarvestWithheldTokensToMintForConfidentialTransferFee(
        HarvestWithheldTokensToMintForConfidentialTransferFee,
    ),
    /// Data for `EnableHarvestToMint`.
    EnableHarvestToMint(EnableHarvestToMint),
    /// Data for `DisableHarvestToMint`.
    DisableHarvestToMint(DisableHarvestToMint),
    /// Data for `WithdrawExcessLamports`.
    WithdrawExcessLamports(WithdrawExcessLamports),
    /// Data for `InitializeMetadataPointer`.
    InitializeMetadataPointer(InitializeMetadataPointer),
    /// Data for `UpdateMetadataPointer`.
    UpdateMetadataPointer(UpdateMetadataPointer),
    /// Data for `InitializeGroupPointer`.
    InitializeGroupPointer(InitializeGroupPointer),
    /// Data for `UpdateGroupPointer`.
    UpdateGroupPointer(UpdateGroupPointer),
    /// Data for `InitializeGroupMemberPointer`.
    InitializeGroupMemberPointer(InitializeGroupMemberPointer),
    /// Data for `UpdateGroupMemberPointer`.
    UpdateGroupMemberPointer(UpdateGroupMemberPointer),
    /// Data for `InitializeConfidentialMintBurn`.
    InitializeConfidentialMintBurn(InitializeConfidentialMintBurn),
    /// Data for `RotateSupplyElgamalPubkey`.
    RotateSupplyElgamalPubkey(RotateSupplyElgamalPubkey),
    /// Data for `UpdateConfidentialMintBurnDecryptableSupply`.
    UpdateConfidentialMintBurnDecryptableSupply(UpdateConfidentialMintBurnDecryptableSupply),
    /// Data for `ConfidentialMint`.
    ConfidentialMint(ConfidentialMint),
    /// Data for `ConfidentialBurn`.
    ConfidentialBurn(ConfidentialBurn),
    /// Data for `ApplyConfidentialPendingBurn`.
    ApplyConfidentialPendingBurn(ApplyConfidentialPendingBurn),
    /// Data for `InitializeScaledUiAmountMint`.
    InitializeScaledUiAmountMint(InitializeScaledUiAmountMint),
    /// Data for `UpdateMultiplierScaledUiMint`.
    UpdateMultiplierScaledUiMint(UpdateMultiplierScaledUiMint),
    /// Data for `InitializePausableConfig`.
    InitializePausableConfig(InitializePausableConfig),
    /// Data for `Pause`.
    Pause(Pause),
    /// Data for `Resume`.
    Resume(Resume),
    /// Data for `InitializeTokenMetadata`.
    InitializeTokenMetadata(InitializeTokenMetadata),
    /// Data for `UpdateTokenMetadataField`.
    UpdateTokenMetadataField(UpdateTokenMetadataField),
    /// Data for `RemoveTokenMetadataKey`.
    RemoveTokenMetadataKey(RemoveTokenMetadataKey),
    /// Data for `UpdateTokenMetadataUpdateAuthority`.
    UpdateTokenMetadataUpdateAuthority(UpdateTokenMetadataUpdateAuthority),
    /// Data for `EmitTokenMetadata`.
    EmitTokenMetadata(EmitTokenMetadata),
    /// Data for `InitializeTokenGroup`.
    InitializeTokenGroup(InitializeTokenGroup),
    /// Data for `UpdateTokenGroupMaxSize`.
    UpdateTokenGroupMaxSize(UpdateTokenGroupMaxSize),
    /// Data for `UpdateTokenGroupUpdateAuthority`.
    UpdateTokenGroupUpdateAuthority(UpdateTokenGroupUpdateAuthority),
    /// Data for `InitializeTokenGroupMember`.
    InitializeTokenGroupMember(InitializeTokenGroupMember),
    /// Data for `UnwrapLamports`.
    UnwrapLamports(UnwrapLamports),
    /// Data for `InitializePermissionedBurn`.
    InitializePermissionedBurn(InitializePermissionedBurn),
    /// Data for `PermissionedBurn`.
    PermissionedBurn(PermissionedBurn),
    /// Data for `PermissionedBurnChecked`.
    PermissionedBurnChecked(PermissionedBurnChecked),
    /// Data for `PermissionedConfidentialBurn`.
    PermissionedConfidentialBurn(PermissionedConfidentialBurn),
    /// Data for `Batch`.
    Batch(Batch),
}
/// Prefix-safe multi-byte instruction dispatch.
#[derive(Clone, Copy, Debug)]
pub struct Token2022InstructionDataCodec;
impl Encoder<Token2022InstructionData> for Token2022InstructionDataCodec {
    fn encoded_len(&self, value: &Token2022InstructionData) -> Result<usize, Error> {
        match value {
Token2022InstructionData::InitializeMint(value)=>InitializeMintCodec.encoded_len(value),
Token2022InstructionData::InitializeAccount(value)=>InitializeAccountCodec.encoded_len(value),
Token2022InstructionData::InitializeMultisig(value)=>InitializeMultisigCodec.encoded_len(value),
Token2022InstructionData::Transfer(value)=>TransferCodec.encoded_len(value),
Token2022InstructionData::Approve(value)=>ApproveCodec.encoded_len(value),
Token2022InstructionData::Revoke(value)=>RevokeCodec.encoded_len(value),
Token2022InstructionData::SetAuthority(value)=>SetAuthorityCodec.encoded_len(value),
Token2022InstructionData::MintTo(value)=>MintToCodec.encoded_len(value),
Token2022InstructionData::Burn(value)=>BurnCodec.encoded_len(value),
Token2022InstructionData::CloseAccount(value)=>CloseAccountCodec.encoded_len(value),
Token2022InstructionData::FreezeAccount(value)=>FreezeAccountCodec.encoded_len(value),
Token2022InstructionData::ThawAccount(value)=>ThawAccountCodec.encoded_len(value),
Token2022InstructionData::TransferChecked(value)=>TransferCheckedCodec.encoded_len(value),
Token2022InstructionData::ApproveChecked(value)=>ApproveCheckedCodec.encoded_len(value),
Token2022InstructionData::MintToChecked(value)=>MintToCheckedCodec.encoded_len(value),
Token2022InstructionData::BurnChecked(value)=>BurnCheckedCodec.encoded_len(value),
Token2022InstructionData::InitializeAccount2(value)=>InitializeAccount2Codec.encoded_len(value),
Token2022InstructionData::SyncNative(value)=>SyncNativeCodec.encoded_len(value),
Token2022InstructionData::InitializeAccount3(value)=>InitializeAccount3Codec.encoded_len(value),
Token2022InstructionData::InitializeMultisig2(value)=>InitializeMultisig2Codec.encoded_len(value),
Token2022InstructionData::InitializeMint2(value)=>InitializeMint2Codec.encoded_len(value),
Token2022InstructionData::GetAccountDataSize(value)=>GetAccountDataSizeCodec.encoded_len(value),
Token2022InstructionData::InitializeImmutableOwner(value)=>InitializeImmutableOwnerCodec.encoded_len(value),
Token2022InstructionData::AmountToUiAmount(value)=>AmountToUiAmountCodec.encoded_len(value),
Token2022InstructionData::UiAmountToAmount(value)=>UiAmountToAmountCodec.encoded_len(value),
Token2022InstructionData::InitializeMintCloseAuthority(value)=>InitializeMintCloseAuthorityCodec.encoded_len(value),
Token2022InstructionData::InitializeTransferFeeConfig(value)=>InitializeTransferFeeConfigCodec.encoded_len(value),
Token2022InstructionData::TransferCheckedWithFee(value)=>TransferCheckedWithFeeCodec.encoded_len(value),
Token2022InstructionData::WithdrawWithheldTokensFromMint(value)=>WithdrawWithheldTokensFromMintCodec.encoded_len(value),
Token2022InstructionData::WithdrawWithheldTokensFromAccounts(value)=>WithdrawWithheldTokensFromAccountsCodec.encoded_len(value),
Token2022InstructionData::HarvestWithheldTokensToMint(value)=>HarvestWithheldTokensToMintCodec.encoded_len(value),
Token2022InstructionData::SetTransferFee(value)=>SetTransferFeeCodec.encoded_len(value),
Token2022InstructionData::InitializeConfidentialTransferMint(value)=>InitializeConfidentialTransferMintCodec.encoded_len(value),
Token2022InstructionData::UpdateConfidentialTransferMint(value)=>UpdateConfidentialTransferMintCodec.encoded_len(value),
Token2022InstructionData::ConfigureConfidentialTransferAccount(value)=>ConfigureConfidentialTransferAccountCodec.encoded_len(value),
Token2022InstructionData::ApproveConfidentialTransferAccount(value)=>ApproveConfidentialTransferAccountCodec.encoded_len(value),
Token2022InstructionData::EmptyConfidentialTransferAccount(value)=>EmptyConfidentialTransferAccountCodec.encoded_len(value),
Token2022InstructionData::ConfidentialDeposit(value)=>ConfidentialDepositCodec.encoded_len(value),
Token2022InstructionData::ConfidentialWithdraw(value)=>ConfidentialWithdrawCodec.encoded_len(value),
Token2022InstructionData::ConfidentialTransfer(value)=>ConfidentialTransferCodec.encoded_len(value),
Token2022InstructionData::ApplyConfidentialPendingBalance(value)=>ApplyConfidentialPendingBalanceCodec.encoded_len(value),
Token2022InstructionData::EnableConfidentialCredits(value)=>EnableConfidentialCreditsCodec.encoded_len(value),
Token2022InstructionData::DisableConfidentialCredits(value)=>DisableConfidentialCreditsCodec.encoded_len(value),
Token2022InstructionData::EnableNonConfidentialCredits(value)=>EnableNonConfidentialCreditsCodec.encoded_len(value),
Token2022InstructionData::DisableNonConfidentialCredits(value)=>DisableNonConfidentialCreditsCodec.encoded_len(value),
Token2022InstructionData::ConfidentialTransferWithFee(value)=>ConfidentialTransferWithFeeCodec.encoded_len(value),
Token2022InstructionData::ConfigureConfidentialTransferAccountWithRegistry(value)=>ConfigureConfidentialTransferAccountWithRegistryCodec.encoded_len(value),
Token2022InstructionData::InitializeDefaultAccountState(value)=>InitializeDefaultAccountStateCodec.encoded_len(value),
Token2022InstructionData::UpdateDefaultAccountState(value)=>UpdateDefaultAccountStateCodec.encoded_len(value),
Token2022InstructionData::Reallocate(value)=>ReallocateCodec.encoded_len(value),
Token2022InstructionData::EnableMemoTransfers(value)=>EnableMemoTransfersCodec.encoded_len(value),
Token2022InstructionData::DisableMemoTransfers(value)=>DisableMemoTransfersCodec.encoded_len(value),
Token2022InstructionData::CreateNativeMint(value)=>CreateNativeMintCodec.encoded_len(value),
Token2022InstructionData::InitializeNonTransferableMint(value)=>InitializeNonTransferableMintCodec.encoded_len(value),
Token2022InstructionData::InitializeInterestBearingMint(value)=>InitializeInterestBearingMintCodec.encoded_len(value),
Token2022InstructionData::UpdateRateInterestBearingMint(value)=>UpdateRateInterestBearingMintCodec.encoded_len(value),
Token2022InstructionData::EnableCpiGuard(value)=>EnableCpiGuardCodec.encoded_len(value),
Token2022InstructionData::DisableCpiGuard(value)=>DisableCpiGuardCodec.encoded_len(value),
Token2022InstructionData::InitializePermanentDelegate(value)=>InitializePermanentDelegateCodec.encoded_len(value),
Token2022InstructionData::InitializeTransferHook(value)=>InitializeTransferHookCodec.encoded_len(value),
Token2022InstructionData::UpdateTransferHook(value)=>UpdateTransferHookCodec.encoded_len(value),
Token2022InstructionData::InitializeConfidentialTransferFee(value)=>InitializeConfidentialTransferFeeCodec.encoded_len(value),
Token2022InstructionData::WithdrawWithheldTokensFromMintForConfidentialTransferFee(value)=>WithdrawWithheldTokensFromMintForConfidentialTransferFeeCodec.encoded_len(value),
Token2022InstructionData::WithdrawWithheldTokensFromAccountsForConfidentialTransferFee(value)=>WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeCodec.encoded_len(value),
Token2022InstructionData::HarvestWithheldTokensToMintForConfidentialTransferFee(value)=>HarvestWithheldTokensToMintForConfidentialTransferFeeCodec.encoded_len(value),
Token2022InstructionData::EnableHarvestToMint(value)=>EnableHarvestToMintCodec.encoded_len(value),
Token2022InstructionData::DisableHarvestToMint(value)=>DisableHarvestToMintCodec.encoded_len(value),
Token2022InstructionData::WithdrawExcessLamports(value)=>WithdrawExcessLamportsCodec.encoded_len(value),
Token2022InstructionData::InitializeMetadataPointer(value)=>InitializeMetadataPointerCodec.encoded_len(value),
Token2022InstructionData::UpdateMetadataPointer(value)=>UpdateMetadataPointerCodec.encoded_len(value),
Token2022InstructionData::InitializeGroupPointer(value)=>InitializeGroupPointerCodec.encoded_len(value),
Token2022InstructionData::UpdateGroupPointer(value)=>UpdateGroupPointerCodec.encoded_len(value),
Token2022InstructionData::InitializeGroupMemberPointer(value)=>InitializeGroupMemberPointerCodec.encoded_len(value),
Token2022InstructionData::UpdateGroupMemberPointer(value)=>UpdateGroupMemberPointerCodec.encoded_len(value),
Token2022InstructionData::InitializeConfidentialMintBurn(value)=>InitializeConfidentialMintBurnCodec.encoded_len(value),
Token2022InstructionData::RotateSupplyElgamalPubkey(value)=>RotateSupplyElgamalPubkeyCodec.encoded_len(value),
Token2022InstructionData::UpdateConfidentialMintBurnDecryptableSupply(value)=>UpdateConfidentialMintBurnDecryptableSupplyCodec.encoded_len(value),
Token2022InstructionData::ConfidentialMint(value)=>ConfidentialMintCodec.encoded_len(value),
Token2022InstructionData::ConfidentialBurn(value)=>ConfidentialBurnCodec.encoded_len(value),
Token2022InstructionData::ApplyConfidentialPendingBurn(value)=>ApplyConfidentialPendingBurnCodec.encoded_len(value),
Token2022InstructionData::InitializeScaledUiAmountMint(value)=>InitializeScaledUiAmountMintCodec.encoded_len(value),
Token2022InstructionData::UpdateMultiplierScaledUiMint(value)=>UpdateMultiplierScaledUiMintCodec.encoded_len(value),
Token2022InstructionData::InitializePausableConfig(value)=>InitializePausableConfigCodec.encoded_len(value),
Token2022InstructionData::Pause(value)=>PauseCodec.encoded_len(value),
Token2022InstructionData::Resume(value)=>ResumeCodec.encoded_len(value),
Token2022InstructionData::InitializeTokenMetadata(value)=>InitializeTokenMetadataCodec.encoded_len(value),
Token2022InstructionData::UpdateTokenMetadataField(value)=>UpdateTokenMetadataFieldCodec.encoded_len(value),
Token2022InstructionData::RemoveTokenMetadataKey(value)=>RemoveTokenMetadataKeyCodec.encoded_len(value),
Token2022InstructionData::UpdateTokenMetadataUpdateAuthority(value)=>UpdateTokenMetadataUpdateAuthorityCodec.encoded_len(value),
Token2022InstructionData::EmitTokenMetadata(value)=>EmitTokenMetadataCodec.encoded_len(value),
Token2022InstructionData::InitializeTokenGroup(value)=>InitializeTokenGroupCodec.encoded_len(value),
Token2022InstructionData::UpdateTokenGroupMaxSize(value)=>UpdateTokenGroupMaxSizeCodec.encoded_len(value),
Token2022InstructionData::UpdateTokenGroupUpdateAuthority(value)=>UpdateTokenGroupUpdateAuthorityCodec.encoded_len(value),
Token2022InstructionData::InitializeTokenGroupMember(value)=>InitializeTokenGroupMemberCodec.encoded_len(value),
Token2022InstructionData::UnwrapLamports(value)=>UnwrapLamportsCodec.encoded_len(value),
Token2022InstructionData::InitializePermissionedBurn(value)=>InitializePermissionedBurnCodec.encoded_len(value),
Token2022InstructionData::PermissionedBurn(value)=>PermissionedBurnCodec.encoded_len(value),
Token2022InstructionData::PermissionedBurnChecked(value)=>PermissionedBurnCheckedCodec.encoded_len(value),
Token2022InstructionData::PermissionedConfidentialBurn(value)=>PermissionedConfidentialBurnCodec.encoded_len(value),
Token2022InstructionData::Batch(value)=>BatchCodec.encoded_len(value),
}
    }
    fn write(
        &self,
        value: &Token2022InstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
Token2022InstructionData::InitializeMint(value)=>InitializeMintCodec.write(value,writer),
Token2022InstructionData::InitializeAccount(value)=>InitializeAccountCodec.write(value,writer),
Token2022InstructionData::InitializeMultisig(value)=>InitializeMultisigCodec.write(value,writer),
Token2022InstructionData::Transfer(value)=>TransferCodec.write(value,writer),
Token2022InstructionData::Approve(value)=>ApproveCodec.write(value,writer),
Token2022InstructionData::Revoke(value)=>RevokeCodec.write(value,writer),
Token2022InstructionData::SetAuthority(value)=>SetAuthorityCodec.write(value,writer),
Token2022InstructionData::MintTo(value)=>MintToCodec.write(value,writer),
Token2022InstructionData::Burn(value)=>BurnCodec.write(value,writer),
Token2022InstructionData::CloseAccount(value)=>CloseAccountCodec.write(value,writer),
Token2022InstructionData::FreezeAccount(value)=>FreezeAccountCodec.write(value,writer),
Token2022InstructionData::ThawAccount(value)=>ThawAccountCodec.write(value,writer),
Token2022InstructionData::TransferChecked(value)=>TransferCheckedCodec.write(value,writer),
Token2022InstructionData::ApproveChecked(value)=>ApproveCheckedCodec.write(value,writer),
Token2022InstructionData::MintToChecked(value)=>MintToCheckedCodec.write(value,writer),
Token2022InstructionData::BurnChecked(value)=>BurnCheckedCodec.write(value,writer),
Token2022InstructionData::InitializeAccount2(value)=>InitializeAccount2Codec.write(value,writer),
Token2022InstructionData::SyncNative(value)=>SyncNativeCodec.write(value,writer),
Token2022InstructionData::InitializeAccount3(value)=>InitializeAccount3Codec.write(value,writer),
Token2022InstructionData::InitializeMultisig2(value)=>InitializeMultisig2Codec.write(value,writer),
Token2022InstructionData::InitializeMint2(value)=>InitializeMint2Codec.write(value,writer),
Token2022InstructionData::GetAccountDataSize(value)=>GetAccountDataSizeCodec.write(value,writer),
Token2022InstructionData::InitializeImmutableOwner(value)=>InitializeImmutableOwnerCodec.write(value,writer),
Token2022InstructionData::AmountToUiAmount(value)=>AmountToUiAmountCodec.write(value,writer),
Token2022InstructionData::UiAmountToAmount(value)=>UiAmountToAmountCodec.write(value,writer),
Token2022InstructionData::InitializeMintCloseAuthority(value)=>InitializeMintCloseAuthorityCodec.write(value,writer),
Token2022InstructionData::InitializeTransferFeeConfig(value)=>InitializeTransferFeeConfigCodec.write(value,writer),
Token2022InstructionData::TransferCheckedWithFee(value)=>TransferCheckedWithFeeCodec.write(value,writer),
Token2022InstructionData::WithdrawWithheldTokensFromMint(value)=>WithdrawWithheldTokensFromMintCodec.write(value,writer),
Token2022InstructionData::WithdrawWithheldTokensFromAccounts(value)=>WithdrawWithheldTokensFromAccountsCodec.write(value,writer),
Token2022InstructionData::HarvestWithheldTokensToMint(value)=>HarvestWithheldTokensToMintCodec.write(value,writer),
Token2022InstructionData::SetTransferFee(value)=>SetTransferFeeCodec.write(value,writer),
Token2022InstructionData::InitializeConfidentialTransferMint(value)=>InitializeConfidentialTransferMintCodec.write(value,writer),
Token2022InstructionData::UpdateConfidentialTransferMint(value)=>UpdateConfidentialTransferMintCodec.write(value,writer),
Token2022InstructionData::ConfigureConfidentialTransferAccount(value)=>ConfigureConfidentialTransferAccountCodec.write(value,writer),
Token2022InstructionData::ApproveConfidentialTransferAccount(value)=>ApproveConfidentialTransferAccountCodec.write(value,writer),
Token2022InstructionData::EmptyConfidentialTransferAccount(value)=>EmptyConfidentialTransferAccountCodec.write(value,writer),
Token2022InstructionData::ConfidentialDeposit(value)=>ConfidentialDepositCodec.write(value,writer),
Token2022InstructionData::ConfidentialWithdraw(value)=>ConfidentialWithdrawCodec.write(value,writer),
Token2022InstructionData::ConfidentialTransfer(value)=>ConfidentialTransferCodec.write(value,writer),
Token2022InstructionData::ApplyConfidentialPendingBalance(value)=>ApplyConfidentialPendingBalanceCodec.write(value,writer),
Token2022InstructionData::EnableConfidentialCredits(value)=>EnableConfidentialCreditsCodec.write(value,writer),
Token2022InstructionData::DisableConfidentialCredits(value)=>DisableConfidentialCreditsCodec.write(value,writer),
Token2022InstructionData::EnableNonConfidentialCredits(value)=>EnableNonConfidentialCreditsCodec.write(value,writer),
Token2022InstructionData::DisableNonConfidentialCredits(value)=>DisableNonConfidentialCreditsCodec.write(value,writer),
Token2022InstructionData::ConfidentialTransferWithFee(value)=>ConfidentialTransferWithFeeCodec.write(value,writer),
Token2022InstructionData::ConfigureConfidentialTransferAccountWithRegistry(value)=>ConfigureConfidentialTransferAccountWithRegistryCodec.write(value,writer),
Token2022InstructionData::InitializeDefaultAccountState(value)=>InitializeDefaultAccountStateCodec.write(value,writer),
Token2022InstructionData::UpdateDefaultAccountState(value)=>UpdateDefaultAccountStateCodec.write(value,writer),
Token2022InstructionData::Reallocate(value)=>ReallocateCodec.write(value,writer),
Token2022InstructionData::EnableMemoTransfers(value)=>EnableMemoTransfersCodec.write(value,writer),
Token2022InstructionData::DisableMemoTransfers(value)=>DisableMemoTransfersCodec.write(value,writer),
Token2022InstructionData::CreateNativeMint(value)=>CreateNativeMintCodec.write(value,writer),
Token2022InstructionData::InitializeNonTransferableMint(value)=>InitializeNonTransferableMintCodec.write(value,writer),
Token2022InstructionData::InitializeInterestBearingMint(value)=>InitializeInterestBearingMintCodec.write(value,writer),
Token2022InstructionData::UpdateRateInterestBearingMint(value)=>UpdateRateInterestBearingMintCodec.write(value,writer),
Token2022InstructionData::EnableCpiGuard(value)=>EnableCpiGuardCodec.write(value,writer),
Token2022InstructionData::DisableCpiGuard(value)=>DisableCpiGuardCodec.write(value,writer),
Token2022InstructionData::InitializePermanentDelegate(value)=>InitializePermanentDelegateCodec.write(value,writer),
Token2022InstructionData::InitializeTransferHook(value)=>InitializeTransferHookCodec.write(value,writer),
Token2022InstructionData::UpdateTransferHook(value)=>UpdateTransferHookCodec.write(value,writer),
Token2022InstructionData::InitializeConfidentialTransferFee(value)=>InitializeConfidentialTransferFeeCodec.write(value,writer),
Token2022InstructionData::WithdrawWithheldTokensFromMintForConfidentialTransferFee(value)=>WithdrawWithheldTokensFromMintForConfidentialTransferFeeCodec.write(value,writer),
Token2022InstructionData::WithdrawWithheldTokensFromAccountsForConfidentialTransferFee(value)=>WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeCodec.write(value,writer),
Token2022InstructionData::HarvestWithheldTokensToMintForConfidentialTransferFee(value)=>HarvestWithheldTokensToMintForConfidentialTransferFeeCodec.write(value,writer),
Token2022InstructionData::EnableHarvestToMint(value)=>EnableHarvestToMintCodec.write(value,writer),
Token2022InstructionData::DisableHarvestToMint(value)=>DisableHarvestToMintCodec.write(value,writer),
Token2022InstructionData::WithdrawExcessLamports(value)=>WithdrawExcessLamportsCodec.write(value,writer),
Token2022InstructionData::InitializeMetadataPointer(value)=>InitializeMetadataPointerCodec.write(value,writer),
Token2022InstructionData::UpdateMetadataPointer(value)=>UpdateMetadataPointerCodec.write(value,writer),
Token2022InstructionData::InitializeGroupPointer(value)=>InitializeGroupPointerCodec.write(value,writer),
Token2022InstructionData::UpdateGroupPointer(value)=>UpdateGroupPointerCodec.write(value,writer),
Token2022InstructionData::InitializeGroupMemberPointer(value)=>InitializeGroupMemberPointerCodec.write(value,writer),
Token2022InstructionData::UpdateGroupMemberPointer(value)=>UpdateGroupMemberPointerCodec.write(value,writer),
Token2022InstructionData::InitializeConfidentialMintBurn(value)=>InitializeConfidentialMintBurnCodec.write(value,writer),
Token2022InstructionData::RotateSupplyElgamalPubkey(value)=>RotateSupplyElgamalPubkeyCodec.write(value,writer),
Token2022InstructionData::UpdateConfidentialMintBurnDecryptableSupply(value)=>UpdateConfidentialMintBurnDecryptableSupplyCodec.write(value,writer),
Token2022InstructionData::ConfidentialMint(value)=>ConfidentialMintCodec.write(value,writer),
Token2022InstructionData::ConfidentialBurn(value)=>ConfidentialBurnCodec.write(value,writer),
Token2022InstructionData::ApplyConfidentialPendingBurn(value)=>ApplyConfidentialPendingBurnCodec.write(value,writer),
Token2022InstructionData::InitializeScaledUiAmountMint(value)=>InitializeScaledUiAmountMintCodec.write(value,writer),
Token2022InstructionData::UpdateMultiplierScaledUiMint(value)=>UpdateMultiplierScaledUiMintCodec.write(value,writer),
Token2022InstructionData::InitializePausableConfig(value)=>InitializePausableConfigCodec.write(value,writer),
Token2022InstructionData::Pause(value)=>PauseCodec.write(value,writer),
Token2022InstructionData::Resume(value)=>ResumeCodec.write(value,writer),
Token2022InstructionData::InitializeTokenMetadata(value)=>InitializeTokenMetadataCodec.write(value,writer),
Token2022InstructionData::UpdateTokenMetadataField(value)=>UpdateTokenMetadataFieldCodec.write(value,writer),
Token2022InstructionData::RemoveTokenMetadataKey(value)=>RemoveTokenMetadataKeyCodec.write(value,writer),
Token2022InstructionData::UpdateTokenMetadataUpdateAuthority(value)=>UpdateTokenMetadataUpdateAuthorityCodec.write(value,writer),
Token2022InstructionData::EmitTokenMetadata(value)=>EmitTokenMetadataCodec.write(value,writer),
Token2022InstructionData::InitializeTokenGroup(value)=>InitializeTokenGroupCodec.write(value,writer),
Token2022InstructionData::UpdateTokenGroupMaxSize(value)=>UpdateTokenGroupMaxSizeCodec.write(value,writer),
Token2022InstructionData::UpdateTokenGroupUpdateAuthority(value)=>UpdateTokenGroupUpdateAuthorityCodec.write(value,writer),
Token2022InstructionData::InitializeTokenGroupMember(value)=>InitializeTokenGroupMemberCodec.write(value,writer),
Token2022InstructionData::UnwrapLamports(value)=>UnwrapLamportsCodec.write(value,writer),
Token2022InstructionData::InitializePermissionedBurn(value)=>InitializePermissionedBurnCodec.write(value,writer),
Token2022InstructionData::PermissionedBurn(value)=>PermissionedBurnCodec.write(value,writer),
Token2022InstructionData::PermissionedBurnChecked(value)=>PermissionedBurnCheckedCodec.write(value,writer),
Token2022InstructionData::PermissionedConfidentialBurn(value)=>PermissionedConfidentialBurnCodec.write(value,writer),
Token2022InstructionData::Batch(value)=>BatchCodec.write(value,writer),
}
    }
}
impl Decoder<Token2022InstructionData> for Token2022InstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Token2022InstructionData, Error> {
        reader.transaction(|reader|{
match reader.remaining() {
[0,..]=>Ok(Token2022InstructionData::InitializeMint(InitializeMintCodec.read(reader)?)),
[1,..]=>Ok(Token2022InstructionData::InitializeAccount(InitializeAccountCodec.read(reader)?)),
[2,..]=>Ok(Token2022InstructionData::InitializeMultisig(InitializeMultisigCodec.read(reader)?)),
[3,..]=>Ok(Token2022InstructionData::Transfer(TransferCodec.read(reader)?)),
[4,..]=>Ok(Token2022InstructionData::Approve(ApproveCodec.read(reader)?)),
[5,..]=>Ok(Token2022InstructionData::Revoke(RevokeCodec.read(reader)?)),
[6,..]=>Ok(Token2022InstructionData::SetAuthority(SetAuthorityCodec.read(reader)?)),
[7,..]=>Ok(Token2022InstructionData::MintTo(MintToCodec.read(reader)?)),
[8,..]=>Ok(Token2022InstructionData::Burn(BurnCodec.read(reader)?)),
[9,..]=>Ok(Token2022InstructionData::CloseAccount(CloseAccountCodec.read(reader)?)),
[10,..]=>Ok(Token2022InstructionData::FreezeAccount(FreezeAccountCodec.read(reader)?)),
[11,..]=>Ok(Token2022InstructionData::ThawAccount(ThawAccountCodec.read(reader)?)),
[12,..]=>Ok(Token2022InstructionData::TransferChecked(TransferCheckedCodec.read(reader)?)),
[13,..]=>Ok(Token2022InstructionData::ApproveChecked(ApproveCheckedCodec.read(reader)?)),
[14,..]=>Ok(Token2022InstructionData::MintToChecked(MintToCheckedCodec.read(reader)?)),
[15,..]=>Ok(Token2022InstructionData::BurnChecked(BurnCheckedCodec.read(reader)?)),
[16,..]=>Ok(Token2022InstructionData::InitializeAccount2(InitializeAccount2Codec.read(reader)?)),
[17,..]=>Ok(Token2022InstructionData::SyncNative(SyncNativeCodec.read(reader)?)),
[18,..]=>Ok(Token2022InstructionData::InitializeAccount3(InitializeAccount3Codec.read(reader)?)),
[19,..]=>Ok(Token2022InstructionData::InitializeMultisig2(InitializeMultisig2Codec.read(reader)?)),
[20,..]=>Ok(Token2022InstructionData::InitializeMint2(InitializeMint2Codec.read(reader)?)),
[21,..]=>Ok(Token2022InstructionData::GetAccountDataSize(GetAccountDataSizeCodec.read(reader)?)),
[22,..]=>Ok(Token2022InstructionData::InitializeImmutableOwner(InitializeImmutableOwnerCodec.read(reader)?)),
[23,..]=>Ok(Token2022InstructionData::AmountToUiAmount(AmountToUiAmountCodec.read(reader)?)),
[24,..]=>Ok(Token2022InstructionData::UiAmountToAmount(UiAmountToAmountCodec.read(reader)?)),
[25,..]=>Ok(Token2022InstructionData::InitializeMintCloseAuthority(InitializeMintCloseAuthorityCodec.read(reader)?)),
[26,0,..]=>Ok(Token2022InstructionData::InitializeTransferFeeConfig(InitializeTransferFeeConfigCodec.read(reader)?)),
[26,1,..]=>Ok(Token2022InstructionData::TransferCheckedWithFee(TransferCheckedWithFeeCodec.read(reader)?)),
[26,2,..]=>Ok(Token2022InstructionData::WithdrawWithheldTokensFromMint(WithdrawWithheldTokensFromMintCodec.read(reader)?)),
[26,3,..]=>Ok(Token2022InstructionData::WithdrawWithheldTokensFromAccounts(WithdrawWithheldTokensFromAccountsCodec.read(reader)?)),
[26,4,..]=>Ok(Token2022InstructionData::HarvestWithheldTokensToMint(HarvestWithheldTokensToMintCodec.read(reader)?)),
[26,5,..]=>Ok(Token2022InstructionData::SetTransferFee(SetTransferFeeCodec.read(reader)?)),
[27,0,..]=>Ok(Token2022InstructionData::InitializeConfidentialTransferMint(InitializeConfidentialTransferMintCodec.read(reader)?)),
[27,1,..]=>Ok(Token2022InstructionData::UpdateConfidentialTransferMint(UpdateConfidentialTransferMintCodec.read(reader)?)),
[27,2,..]=>Ok(Token2022InstructionData::ConfigureConfidentialTransferAccount(ConfigureConfidentialTransferAccountCodec.read(reader)?)),
[27,3,..]=>Ok(Token2022InstructionData::ApproveConfidentialTransferAccount(ApproveConfidentialTransferAccountCodec.read(reader)?)),
[27,4,..]=>Ok(Token2022InstructionData::EmptyConfidentialTransferAccount(EmptyConfidentialTransferAccountCodec.read(reader)?)),
[27,5,..]=>Ok(Token2022InstructionData::ConfidentialDeposit(ConfidentialDepositCodec.read(reader)?)),
[27,6,..]=>Ok(Token2022InstructionData::ConfidentialWithdraw(ConfidentialWithdrawCodec.read(reader)?)),
[27,7,..]=>Ok(Token2022InstructionData::ConfidentialTransfer(ConfidentialTransferCodec.read(reader)?)),
[27,8,..]=>Ok(Token2022InstructionData::ApplyConfidentialPendingBalance(ApplyConfidentialPendingBalanceCodec.read(reader)?)),
[27,9,..]=>Ok(Token2022InstructionData::EnableConfidentialCredits(EnableConfidentialCreditsCodec.read(reader)?)),
[27,10,..]=>Ok(Token2022InstructionData::DisableConfidentialCredits(DisableConfidentialCreditsCodec.read(reader)?)),
[27,11,..]=>Ok(Token2022InstructionData::EnableNonConfidentialCredits(EnableNonConfidentialCreditsCodec.read(reader)?)),
[27,12,..]=>Ok(Token2022InstructionData::DisableNonConfidentialCredits(DisableNonConfidentialCreditsCodec.read(reader)?)),
[27,13,..]=>Ok(Token2022InstructionData::ConfidentialTransferWithFee(ConfidentialTransferWithFeeCodec.read(reader)?)),
[27,14,..]=>Ok(Token2022InstructionData::ConfigureConfidentialTransferAccountWithRegistry(ConfigureConfidentialTransferAccountWithRegistryCodec.read(reader)?)),
[28,0,..]=>Ok(Token2022InstructionData::InitializeDefaultAccountState(InitializeDefaultAccountStateCodec.read(reader)?)),
[28,1,..]=>Ok(Token2022InstructionData::UpdateDefaultAccountState(UpdateDefaultAccountStateCodec.read(reader)?)),
[29,..]=>Ok(Token2022InstructionData::Reallocate(ReallocateCodec.read(reader)?)),
[30,0,..]=>Ok(Token2022InstructionData::EnableMemoTransfers(EnableMemoTransfersCodec.read(reader)?)),
[30,1,..]=>Ok(Token2022InstructionData::DisableMemoTransfers(DisableMemoTransfersCodec.read(reader)?)),
[31,..]=>Ok(Token2022InstructionData::CreateNativeMint(CreateNativeMintCodec.read(reader)?)),
[32,..]=>Ok(Token2022InstructionData::InitializeNonTransferableMint(InitializeNonTransferableMintCodec.read(reader)?)),
[33,0,..]=>Ok(Token2022InstructionData::InitializeInterestBearingMint(InitializeInterestBearingMintCodec.read(reader)?)),
[33,1,..]=>Ok(Token2022InstructionData::UpdateRateInterestBearingMint(UpdateRateInterestBearingMintCodec.read(reader)?)),
[34,0,..]=>Ok(Token2022InstructionData::EnableCpiGuard(EnableCpiGuardCodec.read(reader)?)),
[34,1,..]=>Ok(Token2022InstructionData::DisableCpiGuard(DisableCpiGuardCodec.read(reader)?)),
[35,..]=>Ok(Token2022InstructionData::InitializePermanentDelegate(InitializePermanentDelegateCodec.read(reader)?)),
[36,0,..]=>Ok(Token2022InstructionData::InitializeTransferHook(InitializeTransferHookCodec.read(reader)?)),
[36,1,..]=>Ok(Token2022InstructionData::UpdateTransferHook(UpdateTransferHookCodec.read(reader)?)),
[37,0,..]=>Ok(Token2022InstructionData::InitializeConfidentialTransferFee(InitializeConfidentialTransferFeeCodec.read(reader)?)),
[37,1,..]=>Ok(Token2022InstructionData::WithdrawWithheldTokensFromMintForConfidentialTransferFee(WithdrawWithheldTokensFromMintForConfidentialTransferFeeCodec.read(reader)?)),
[37,2,..]=>Ok(Token2022InstructionData::WithdrawWithheldTokensFromAccountsForConfidentialTransferFee(WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeCodec.read(reader)?)),
[37,3,..]=>Ok(Token2022InstructionData::HarvestWithheldTokensToMintForConfidentialTransferFee(HarvestWithheldTokensToMintForConfidentialTransferFeeCodec.read(reader)?)),
[37,4,..]=>Ok(Token2022InstructionData::EnableHarvestToMint(EnableHarvestToMintCodec.read(reader)?)),
[37,5,..]=>Ok(Token2022InstructionData::DisableHarvestToMint(DisableHarvestToMintCodec.read(reader)?)),
[38,..]=>Ok(Token2022InstructionData::WithdrawExcessLamports(WithdrawExcessLamportsCodec.read(reader)?)),
[39,0,..]=>Ok(Token2022InstructionData::InitializeMetadataPointer(InitializeMetadataPointerCodec.read(reader)?)),
[39,1,..]=>Ok(Token2022InstructionData::UpdateMetadataPointer(UpdateMetadataPointerCodec.read(reader)?)),
[40,0,..]=>Ok(Token2022InstructionData::InitializeGroupPointer(InitializeGroupPointerCodec.read(reader)?)),
[40,1,..]=>Ok(Token2022InstructionData::UpdateGroupPointer(UpdateGroupPointerCodec.read(reader)?)),
[41,0,..]=>Ok(Token2022InstructionData::InitializeGroupMemberPointer(InitializeGroupMemberPointerCodec.read(reader)?)),
[41,1,..]=>Ok(Token2022InstructionData::UpdateGroupMemberPointer(UpdateGroupMemberPointerCodec.read(reader)?)),
[42,0,..]=>Ok(Token2022InstructionData::InitializeConfidentialMintBurn(InitializeConfidentialMintBurnCodec.read(reader)?)),
[42,1,..]=>Ok(Token2022InstructionData::RotateSupplyElgamalPubkey(RotateSupplyElgamalPubkeyCodec.read(reader)?)),
[42,2,..]=>Ok(Token2022InstructionData::UpdateConfidentialMintBurnDecryptableSupply(UpdateConfidentialMintBurnDecryptableSupplyCodec.read(reader)?)),
[42,3,..]=>Ok(Token2022InstructionData::ConfidentialMint(ConfidentialMintCodec.read(reader)?)),
[42,4,..]=>Ok(Token2022InstructionData::ConfidentialBurn(ConfidentialBurnCodec.read(reader)?)),
[42,5,..]=>Ok(Token2022InstructionData::ApplyConfidentialPendingBurn(ApplyConfidentialPendingBurnCodec.read(reader)?)),
[43,0,..]=>Ok(Token2022InstructionData::InitializeScaledUiAmountMint(InitializeScaledUiAmountMintCodec.read(reader)?)),
[43,1,..]=>Ok(Token2022InstructionData::UpdateMultiplierScaledUiMint(UpdateMultiplierScaledUiMintCodec.read(reader)?)),
[44,0,..]=>Ok(Token2022InstructionData::InitializePausableConfig(InitializePausableConfigCodec.read(reader)?)),
[44,1,..]=>Ok(Token2022InstructionData::Pause(PauseCodec.read(reader)?)),
[44,2,..]=>Ok(Token2022InstructionData::Resume(ResumeCodec.read(reader)?)),
[210,225,30,162,88,184,77,141,..]=>Ok(Token2022InstructionData::InitializeTokenMetadata(InitializeTokenMetadataCodec.read(reader)?)),
[221,233,49,45,181,202,220,200,..]=>Ok(Token2022InstructionData::UpdateTokenMetadataField(UpdateTokenMetadataFieldCodec.read(reader)?)),
[234,18,32,56,89,141,37,181,..]=>Ok(Token2022InstructionData::RemoveTokenMetadataKey(RemoveTokenMetadataKeyCodec.read(reader)?)),
[215,228,166,228,84,100,86,123,..]=>Ok(Token2022InstructionData::UpdateTokenMetadataUpdateAuthority(UpdateTokenMetadataUpdateAuthorityCodec.read(reader)?)),
[250,166,180,250,13,12,184,70,..]=>Ok(Token2022InstructionData::EmitTokenMetadata(EmitTokenMetadataCodec.read(reader)?)),
[121,113,108,39,54,51,0,4,..]=>Ok(Token2022InstructionData::InitializeTokenGroup(InitializeTokenGroupCodec.read(reader)?)),
[108,37,171,143,248,30,18,110,..]=>Ok(Token2022InstructionData::UpdateTokenGroupMaxSize(UpdateTokenGroupMaxSizeCodec.read(reader)?)),
[161,105,88,1,237,221,216,203,..]=>Ok(Token2022InstructionData::UpdateTokenGroupUpdateAuthority(UpdateTokenGroupUpdateAuthorityCodec.read(reader)?)),
[152,32,222,176,223,237,116,134,..]=>Ok(Token2022InstructionData::InitializeTokenGroupMember(InitializeTokenGroupMemberCodec.read(reader)?)),
[45,..]=>Ok(Token2022InstructionData::UnwrapLamports(UnwrapLamportsCodec.read(reader)?)),
[46,0,..]=>Ok(Token2022InstructionData::InitializePermissionedBurn(InitializePermissionedBurnCodec.read(reader)?)),
[46,1,..]=>Ok(Token2022InstructionData::PermissionedBurn(PermissionedBurnCodec.read(reader)?)),
[46,2,..]=>Ok(Token2022InstructionData::PermissionedBurnChecked(PermissionedBurnCheckedCodec.read(reader)?)),
[46,3,..]=>Ok(Token2022InstructionData::PermissionedConfidentialBurn(PermissionedConfidentialBurnCodec.read(reader)?)),
[255,..]=>Ok(Token2022InstructionData::Batch(BatchCodec.read(reader)?)),
[]=>Err(Error::Truncated),
_=>Err(Error::InvalidDiscriminator),
}
})
    }
}
/// Validated data for `Mint`.
#[derive(Clone, Debug, PartialEq)]
pub struct Mint {
    /// The `mint_authority` wire field.
    pub mint_authority: Option<Address>,
    /// The `supply` wire field.
    pub supply: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
    /// The `is_initialized` wire field.
    pub is_initialized: bool,
    /// The `freeze_authority` wire field.
    pub freeze_authority: Option<Address>,
    /// The `extensions` wire field.
    pub extensions: Option<Vec<Extension>>,
}
/// Bounded codec for [`Mint`].
#[derive(Clone, Copy, Debug)]
pub struct MintCodec;
impl Encoder<Mint> for MintCodec {
    fn encoded_len(&self, value: &Mint) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::c_option(
                    Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    32,
                )
                .encoded_len(&value.mint_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.supply)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_initialized)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::c_option(
                    Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    32,
                )
                .encoded_len(&value.freeze_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::remainder(Affix {
                    prefix: &[
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 1,
                    ],
                    item: TlvSequence(ExtensionCodec, |value: &Extension| {
                        matches!(value, Extension::Uninitialized)
                    }),
                    suffix: &[],
                })
                .encoded_len(&value.extensions)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Mint, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::c_option(
            Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            32,
        )
        .write(&value.mint_authority, writer)?;
        Number::<u64>::LE.write(&value.supply, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Bool.write(&value.is_initialized, writer)?;
        OptionCodec::c_option(
            Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            32,
        )
        .write(&value.freeze_authority, writer)?;
        OptionCodec::remainder(Affix {
            prefix: &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            item: TlvSequence(ExtensionCodec, |value: &Extension| {
                matches!(value, Extension::Uninitialized)
            }),
            suffix: &[],
        })
        .write(&value.extensions, writer)?;
        Ok(())
    }
}
impl Decoder<Mint> for MintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Mint, Error> {
        Ok(Mint {
            mint_authority: OptionCodec::c_option(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                32,
            )
            .read(reader)?,
            supply: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
            is_initialized: Bool.read(reader)?,
            freeze_authority: OptionCodec::c_option(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                32,
            )
            .read(reader)?,
            extensions: OptionCodec::remainder(Affix {
                prefix: &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ],
                item: TlvSequence(ExtensionCodec, |value: &Extension| {
                    matches!(value, Extension::Uninitialized)
                }),
                suffix: &[],
            })
            .read(reader)?,
        })
    }
}
/// Validated data for `Token`.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The `mint` wire field.
    pub mint: Address,
    /// The `owner` wire field.
    pub owner: Address,
    /// The `amount` wire field.
    pub amount: u64,
    /// The `delegate` wire field.
    pub delegate: Option<Address>,
    /// The `state` wire field.
    pub state: AccountState,
    /// The `is_native` wire field.
    pub is_native: Option<u64>,
    /// The `delegated_amount` wire field.
    pub delegated_amount: u64,
    /// The `close_authority` wire field.
    pub close_authority: Option<Address>,
    /// The `extensions` wire field.
    pub extensions: Option<Vec<Extension>>,
}
/// Bounded codec for [`Token`].
#[derive(Clone, Copy, Debug)]
pub struct TokenCodec;
impl Encoder<Token> for TokenCodec {
    fn encoded_len(&self, value: &Token) -> Result<usize, Error> {
        let size = 0usize;
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
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::c_option(
                    Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    32,
                )
                .encoded_len(&value.delegate)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(AccountStateCodec.encoded_len(&value.state)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::c_option(Number::<u64>::LE, 8).encoded_len(&value.is_native)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.delegated_amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::c_option(
                    Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    32,
                )
                .encoded_len(&value.close_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::remainder(Affix {
                    prefix: &[2],
                    item: TlvSequence(ExtensionCodec, |value: &Extension| {
                        matches!(value, Extension::Uninitialized)
                    }),
                    suffix: &[],
                })
                .encoded_len(&value.extensions)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Token, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.owner, writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        OptionCodec::c_option(
            Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            32,
        )
        .write(&value.delegate, writer)?;
        AccountStateCodec.write(&value.state, writer)?;
        OptionCodec::c_option(Number::<u64>::LE, 8).write(&value.is_native, writer)?;
        Number::<u64>::LE.write(&value.delegated_amount, writer)?;
        OptionCodec::c_option(
            Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            32,
        )
        .write(&value.close_authority, writer)?;
        OptionCodec::remainder(Affix {
            prefix: &[2],
            item: TlvSequence(ExtensionCodec, |value: &Extension| {
                matches!(value, Extension::Uninitialized)
            }),
            suffix: &[],
        })
        .write(&value.extensions, writer)?;
        Ok(())
    }
}
impl Decoder<Token> for TokenCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Token, Error> {
        Ok(Token {
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            amount: Number::<u64>::LE.read(reader)?,
            delegate: OptionCodec::c_option(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                32,
            )
            .read(reader)?,
            state: AccountStateCodec.read(reader)?,
            is_native: OptionCodec::c_option(Number::<u64>::LE, 8).read(reader)?,
            delegated_amount: Number::<u64>::LE.read(reader)?,
            close_authority: OptionCodec::c_option(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                32,
            )
            .read(reader)?,
            extensions: OptionCodec::remainder(Affix {
                prefix: &[2],
                item: TlvSequence(ExtensionCodec, |value: &Extension| {
                    matches!(value, Extension::Uninitialized)
                }),
                suffix: &[],
            })
            .read(reader)?,
        })
    }
}
/// Validated data for `Multisig`.
#[derive(Clone, Debug, PartialEq)]
pub struct Multisig {
    /// The `m` wire field.
    pub m: u8,
    /// The `n` wire field.
    pub n: u8,
    /// The `is_initialized` wire field.
    pub is_initialized: bool,
    /// The `signers` wire field.
    pub signers: Vec<Address>,
}
/// Bounded codec for [`Multisig`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigCodec;
impl Encoder<Multisig> for MultisigCodec {
    fn encoded_len(&self, value: &Multisig) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.m)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.n)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_initialized)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    count: Count::Fixed(FIXED_COUNT_11),
                })
                .encoded_len(&value.signers)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Multisig, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.m, writer)?;
        Number::<u8>::LE.write(&value.n, writer)?;
        Bool.write(&value.is_initialized, writer)?;
        OwnedSequence(Sequence {
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            count: Count::Fixed(FIXED_COUNT_11),
        })
        .write(&value.signers, writer)?;
        Ok(())
    }
}
impl Decoder<Multisig> for MultisigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Multisig, Error> {
        Ok(Multisig {
            m: Number::<u8>::LE.read(reader)?,
            n: Number::<u8>::LE.read(reader)?,
            is_initialized: Bool.read(reader)?,
            signers: OwnedSequence(Sequence {
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                count: Count::Fixed(FIXED_COUNT_11),
            })
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMint {
    /// The `decimals` wire field.
    pub decimals: u8,
    /// The `mint_authority` wire field.
    pub mint_authority: Address,
    /// The `freeze_authority` wire field.
    pub freeze_authority: Option<Address>,
}
/// Bounded codec for [`InitializeMint`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeMintCodec;
impl Encoder<InitializeMint> for InitializeMintCodec {
    fn encoded_len(&self, value: &InitializeMint) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.mint_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.freeze_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeMint, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[0]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint_authority, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.freeze_authority, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeMint> for InitializeMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeMint, Error> {
        Constant(&[0]).read(reader)?;
        Ok(InitializeMint {
            decimals: Number::<u8>::LE.read(reader)?,
            mint_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            freeze_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeAccount`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeAccount {}
/// Bounded codec for [`InitializeAccount`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeAccountCodec;
impl Encoder<InitializeAccount> for InitializeAccountCodec {
    fn encoded_len(&self, value: &InitializeAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &InitializeAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[1]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<InitializeAccount> for InitializeAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeAccount, Error> {
        Constant(&[1]).read(reader)?;
        let _ = reader;
        Ok(InitializeAccount {})
    }
}
/// Validated data for `InitializeMultisig`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMultisig {
    /// The `m` wire field.
    pub m: u8,
}
/// Bounded codec for [`InitializeMultisig`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeMultisigCodec;
impl Encoder<InitializeMultisig> for InitializeMultisigCodec {
    fn encoded_len(&self, value: &InitializeMultisig) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.m)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeMultisig, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[2]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.m, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeMultisig> for InitializeMultisigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeMultisig, Error> {
        Constant(&[2]).read(reader)?;
        Ok(InitializeMultisig {
            m: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `Transfer`.
#[derive(Clone, Debug, PartialEq)]
pub struct Transfer {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`Transfer`].
#[derive(Clone, Copy, Debug)]
pub struct TransferCodec;
impl Encoder<Transfer> for TransferCodec {
    fn encoded_len(&self, value: &Transfer) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Transfer, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[3]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<Transfer> for TransferCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Transfer, Error> {
        Constant(&[3]).read(reader)?;
        Ok(Transfer {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `Approve`.
#[derive(Clone, Debug, PartialEq)]
pub struct Approve {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`Approve`].
#[derive(Clone, Copy, Debug)]
pub struct ApproveCodec;
impl Encoder<Approve> for ApproveCodec {
    fn encoded_len(&self, value: &Approve) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Approve, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[4]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<Approve> for ApproveCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Approve, Error> {
        Constant(&[4]).read(reader)?;
        Ok(Approve {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `Revoke`.
#[derive(Clone, Debug, PartialEq)]
pub struct Revoke {}
/// Bounded codec for [`Revoke`].
#[derive(Clone, Copy, Debug)]
pub struct RevokeCodec;
impl Encoder<Revoke> for RevokeCodec {
    fn encoded_len(&self, value: &Revoke) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Revoke, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[5]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Revoke> for RevokeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Revoke, Error> {
        Constant(&[5]).read(reader)?;
        let _ = reader;
        Ok(Revoke {})
    }
}
/// Validated data for `SetAuthority`.
#[derive(Clone, Debug, PartialEq)]
pub struct SetAuthority {
    /// The `authority_type` wire field.
    pub authority_type: AuthorityType,
    /// The `new_authority` wire field.
    pub new_authority: Option<Address>,
}
/// Bounded codec for [`SetAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct SetAuthorityCodec;
impl Encoder<SetAuthority> for SetAuthorityCodec {
    fn encoded_len(&self, value: &SetAuthority) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(AuthorityTypeCodec.encoded_len(&value.authority_type)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.new_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetAuthority, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[6]).write(&(), writer)?;
        AuthorityTypeCodec.write(&value.authority_type, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.new_authority, writer)?;
        Ok(())
    }
}
impl Decoder<SetAuthority> for SetAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetAuthority, Error> {
        Constant(&[6]).read(reader)?;
        Ok(SetAuthority {
            authority_type: AuthorityTypeCodec.read(reader)?,
            new_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
        })
    }
}
/// Validated data for `MintTo`.
#[derive(Clone, Debug, PartialEq)]
pub struct MintTo {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`MintTo`].
#[derive(Clone, Copy, Debug)]
pub struct MintToCodec;
impl Encoder<MintTo> for MintToCodec {
    fn encoded_len(&self, value: &MintTo) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MintTo, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[7]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<MintTo> for MintToCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MintTo, Error> {
        Constant(&[7]).read(reader)?;
        Ok(MintTo {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `Burn`.
#[derive(Clone, Debug, PartialEq)]
pub struct Burn {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`Burn`].
#[derive(Clone, Copy, Debug)]
pub struct BurnCodec;
impl Encoder<Burn> for BurnCodec {
    fn encoded_len(&self, value: &Burn) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Burn, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[8]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<Burn> for BurnCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Burn, Error> {
        Constant(&[8]).read(reader)?;
        Ok(Burn {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `CloseAccount`.
#[derive(Clone, Debug, PartialEq)]
pub struct CloseAccount {}
/// Bounded codec for [`CloseAccount`].
#[derive(Clone, Copy, Debug)]
pub struct CloseAccountCodec;
impl Encoder<CloseAccount> for CloseAccountCodec {
    fn encoded_len(&self, value: &CloseAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CloseAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[9]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CloseAccount> for CloseAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseAccount, Error> {
        Constant(&[9]).read(reader)?;
        let _ = reader;
        Ok(CloseAccount {})
    }
}
/// Validated data for `FreezeAccount`.
#[derive(Clone, Debug, PartialEq)]
pub struct FreezeAccount {}
/// Bounded codec for [`FreezeAccount`].
#[derive(Clone, Copy, Debug)]
pub struct FreezeAccountCodec;
impl Encoder<FreezeAccount> for FreezeAccountCodec {
    fn encoded_len(&self, value: &FreezeAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &FreezeAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[10]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<FreezeAccount> for FreezeAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<FreezeAccount, Error> {
        Constant(&[10]).read(reader)?;
        let _ = reader;
        Ok(FreezeAccount {})
    }
}
/// Validated data for `ThawAccount`.
#[derive(Clone, Debug, PartialEq)]
pub struct ThawAccount {}
/// Bounded codec for [`ThawAccount`].
#[derive(Clone, Copy, Debug)]
pub struct ThawAccountCodec;
impl Encoder<ThawAccount> for ThawAccountCodec {
    fn encoded_len(&self, value: &ThawAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &ThawAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[11]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ThawAccount> for ThawAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ThawAccount, Error> {
        Constant(&[11]).read(reader)?;
        let _ = reader;
        Ok(ThawAccount {})
    }
}
/// Validated data for `TransferChecked`.
#[derive(Clone, Debug, PartialEq)]
pub struct TransferChecked {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
}
/// Bounded codec for [`TransferChecked`].
#[derive(Clone, Copy, Debug)]
pub struct TransferCheckedCodec;
impl Encoder<TransferChecked> for TransferCheckedCodec {
    fn encoded_len(&self, value: &TransferChecked) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[12]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Ok(())
    }
}
impl Decoder<TransferChecked> for TransferCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferChecked, Error> {
        Constant(&[12]).read(reader)?;
        Ok(TransferChecked {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ApproveChecked`.
#[derive(Clone, Debug, PartialEq)]
pub struct ApproveChecked {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
}
/// Bounded codec for [`ApproveChecked`].
#[derive(Clone, Copy, Debug)]
pub struct ApproveCheckedCodec;
impl Encoder<ApproveChecked> for ApproveCheckedCodec {
    fn encoded_len(&self, value: &ApproveChecked) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ApproveChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[13]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Ok(())
    }
}
impl Decoder<ApproveChecked> for ApproveCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ApproveChecked, Error> {
        Constant(&[13]).read(reader)?;
        Ok(ApproveChecked {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `MintToChecked`.
#[derive(Clone, Debug, PartialEq)]
pub struct MintToChecked {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
}
/// Bounded codec for [`MintToChecked`].
#[derive(Clone, Copy, Debug)]
pub struct MintToCheckedCodec;
impl Encoder<MintToChecked> for MintToCheckedCodec {
    fn encoded_len(&self, value: &MintToChecked) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MintToChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[14]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Ok(())
    }
}
impl Decoder<MintToChecked> for MintToCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MintToChecked, Error> {
        Constant(&[14]).read(reader)?;
        Ok(MintToChecked {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `BurnChecked`.
#[derive(Clone, Debug, PartialEq)]
pub struct BurnChecked {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
}
/// Bounded codec for [`BurnChecked`].
#[derive(Clone, Copy, Debug)]
pub struct BurnCheckedCodec;
impl Encoder<BurnChecked> for BurnCheckedCodec {
    fn encoded_len(&self, value: &BurnChecked) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &BurnChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[15]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Ok(())
    }
}
impl Decoder<BurnChecked> for BurnCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BurnChecked, Error> {
        Constant(&[15]).read(reader)?;
        Ok(BurnChecked {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `InitializeAccount2`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeAccount2 {
    /// The `owner` wire field.
    pub owner: Address,
}
/// Bounded codec for [`InitializeAccount2`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeAccount2Codec;
impl Encoder<InitializeAccount2> for InitializeAccount2Codec {
    fn encoded_len(&self, value: &InitializeAccount2) -> Result<usize, Error> {
        let size = 1usize;
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
        Ok(size)
    }
    fn write(&self, value: &InitializeAccount2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[16]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.owner, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeAccount2> for InitializeAccount2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeAccount2, Error> {
        Constant(&[16]).read(reader)?;
        Ok(InitializeAccount2 {
            owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `SyncNative`.
#[derive(Clone, Debug, PartialEq)]
pub struct SyncNative {}
/// Bounded codec for [`SyncNative`].
#[derive(Clone, Copy, Debug)]
pub struct SyncNativeCodec;
impl Encoder<SyncNative> for SyncNativeCodec {
    fn encoded_len(&self, value: &SyncNative) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &SyncNative, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[17]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<SyncNative> for SyncNativeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SyncNative, Error> {
        Constant(&[17]).read(reader)?;
        let _ = reader;
        Ok(SyncNative {})
    }
}
/// Validated data for `InitializeAccount3`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeAccount3 {
    /// The `owner` wire field.
    pub owner: Address,
}
/// Bounded codec for [`InitializeAccount3`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeAccount3Codec;
impl Encoder<InitializeAccount3> for InitializeAccount3Codec {
    fn encoded_len(&self, value: &InitializeAccount3) -> Result<usize, Error> {
        let size = 1usize;
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
        Ok(size)
    }
    fn write(&self, value: &InitializeAccount3, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[18]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.owner, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeAccount3> for InitializeAccount3Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeAccount3, Error> {
        Constant(&[18]).read(reader)?;
        Ok(InitializeAccount3 {
            owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeMultisig2`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMultisig2 {
    /// The `m` wire field.
    pub m: u8,
}
/// Bounded codec for [`InitializeMultisig2`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeMultisig2Codec;
impl Encoder<InitializeMultisig2> for InitializeMultisig2Codec {
    fn encoded_len(&self, value: &InitializeMultisig2) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.m)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeMultisig2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[19]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.m, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeMultisig2> for InitializeMultisig2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeMultisig2, Error> {
        Constant(&[19]).read(reader)?;
        Ok(InitializeMultisig2 {
            m: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `InitializeMint2`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMint2 {
    /// The `decimals` wire field.
    pub decimals: u8,
    /// The `mint_authority` wire field.
    pub mint_authority: Address,
    /// The `freeze_authority` wire field.
    pub freeze_authority: Option<Address>,
}
/// Bounded codec for [`InitializeMint2`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeMint2Codec;
impl Encoder<InitializeMint2> for InitializeMint2Codec {
    fn encoded_len(&self, value: &InitializeMint2) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.mint_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.freeze_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeMint2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[20]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint_authority, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.freeze_authority, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeMint2> for InitializeMint2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeMint2, Error> {
        Constant(&[20]).read(reader)?;
        Ok(InitializeMint2 {
            decimals: Number::<u8>::LE.read(reader)?,
            mint_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            freeze_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
        })
    }
}
/// Validated data for `GetAccountDataSize`.
#[derive(Clone, Debug, PartialEq)]
pub struct GetAccountDataSize {}
/// Bounded codec for [`GetAccountDataSize`].
#[derive(Clone, Copy, Debug)]
pub struct GetAccountDataSizeCodec;
impl Encoder<GetAccountDataSize> for GetAccountDataSizeCodec {
    fn encoded_len(&self, value: &GetAccountDataSize) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &GetAccountDataSize, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[21]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<GetAccountDataSize> for GetAccountDataSizeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<GetAccountDataSize, Error> {
        Constant(&[21]).read(reader)?;
        let _ = reader;
        Ok(GetAccountDataSize {})
    }
}
/// Validated data for `InitializeImmutableOwner`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeImmutableOwner {}
/// Bounded codec for [`InitializeImmutableOwner`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeImmutableOwnerCodec;
impl Encoder<InitializeImmutableOwner> for InitializeImmutableOwnerCodec {
    fn encoded_len(&self, value: &InitializeImmutableOwner) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeImmutableOwner,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[22]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<InitializeImmutableOwner> for InitializeImmutableOwnerCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeImmutableOwner, Error> {
        Constant(&[22]).read(reader)?;
        let _ = reader;
        Ok(InitializeImmutableOwner {})
    }
}
/// Validated data for `AmountToUiAmount`.
#[derive(Clone, Debug, PartialEq)]
pub struct AmountToUiAmount {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`AmountToUiAmount`].
#[derive(Clone, Copy, Debug)]
pub struct AmountToUiAmountCodec;
impl Encoder<AmountToUiAmount> for AmountToUiAmountCodec {
    fn encoded_len(&self, value: &AmountToUiAmount) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AmountToUiAmount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[23]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<AmountToUiAmount> for AmountToUiAmountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AmountToUiAmount, Error> {
        Constant(&[23]).read(reader)?;
        Ok(AmountToUiAmount {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `UiAmountToAmount`.
#[derive(Clone, Debug, PartialEq)]
pub struct UiAmountToAmount {
    /// The `ui_amount` wire field.
    pub ui_amount: String,
}
/// Bounded codec for [`UiAmountToAmount`].
#[derive(Clone, Copy, Debug)]
pub struct UiAmountToAmountCodec;
impl Encoder<UiAmountToAmount> for UiAmountToAmountCodec {
    fn encoded_len(&self, value: &UiAmountToAmount) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Utf8.encoded_len(value.ui_amount.as_str())?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UiAmountToAmount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[24]).write(&(), writer)?;
        Utf8.write(value.ui_amount.as_str(), writer)?;
        Ok(())
    }
}
impl Decoder<UiAmountToAmount> for UiAmountToAmountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UiAmountToAmount, Error> {
        Constant(&[24]).read(reader)?;
        Ok(UiAmountToAmount {
            ui_amount: Utf8.read(reader)?,
        })
    }
}
/// Validated data for `InitializeMintCloseAuthority`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMintCloseAuthority {
    /// The `close_authority` wire field.
    pub close_authority: Option<Address>,
}
/// Bounded codec for [`InitializeMintCloseAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeMintCloseAuthorityCodec;
impl Encoder<InitializeMintCloseAuthority> for InitializeMintCloseAuthorityCodec {
    fn encoded_len(&self, value: &InitializeMintCloseAuthority) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.close_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeMintCloseAuthority,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[25]).write(&(), writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.close_authority, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeMintCloseAuthority> for InitializeMintCloseAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeMintCloseAuthority, Error> {
        Constant(&[25]).read(reader)?;
        Ok(InitializeMintCloseAuthority {
            close_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeTransferFeeConfig`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTransferFeeConfig {
    /// The `transfer_fee_config_authority` wire field.
    pub transfer_fee_config_authority: Option<Address>,
    /// The `withdraw_withheld_authority` wire field.
    pub withdraw_withheld_authority: Option<Address>,
    /// The `transfer_fee_basis_points` wire field.
    pub transfer_fee_basis_points: u16,
    /// The `maximum_fee` wire field.
    pub maximum_fee: u64,
}
/// Bounded codec for [`InitializeTransferFeeConfig`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeTransferFeeConfigCodec;
impl Encoder<InitializeTransferFeeConfig> for InitializeTransferFeeConfigCodec {
    fn encoded_len(&self, value: &InitializeTransferFeeConfig) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.transfer_fee_config_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.withdraw_withheld_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.transfer_fee_basis_points)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.maximum_fee)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeTransferFeeConfig,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[26, 0]).write(&(), writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.transfer_fee_config_authority, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.withdraw_withheld_authority, writer)?;
        Number::<u16>::LE.write(&value.transfer_fee_basis_points, writer)?;
        Number::<u64>::LE.write(&value.maximum_fee, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeTransferFeeConfig> for InitializeTransferFeeConfigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeTransferFeeConfig, Error> {
        Constant(&[26, 0]).read(reader)?;
        Ok(InitializeTransferFeeConfig {
            transfer_fee_config_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            withdraw_withheld_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            transfer_fee_basis_points: Number::<u16>::LE.read(reader)?,
            maximum_fee: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `TransferCheckedWithFee`.
#[derive(Clone, Debug, PartialEq)]
pub struct TransferCheckedWithFee {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
    /// The `fee` wire field.
    pub fee: u64,
}
/// Bounded codec for [`TransferCheckedWithFee`].
#[derive(Clone, Copy, Debug)]
pub struct TransferCheckedWithFeeCodec;
impl Encoder<TransferCheckedWithFee> for TransferCheckedWithFeeCodec {
    fn encoded_len(&self, value: &TransferCheckedWithFee) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.fee)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferCheckedWithFee, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[26, 1]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Number::<u64>::LE.write(&value.fee, writer)?;
        Ok(())
    }
}
impl Decoder<TransferCheckedWithFee> for TransferCheckedWithFeeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferCheckedWithFee, Error> {
        Constant(&[26, 1]).read(reader)?;
        Ok(TransferCheckedWithFee {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
            fee: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `WithdrawWithheldTokensFromMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawWithheldTokensFromMint {}
/// Bounded codec for [`WithdrawWithheldTokensFromMint`].
#[derive(Clone, Copy, Debug)]
pub struct WithdrawWithheldTokensFromMintCodec;
impl Encoder<WithdrawWithheldTokensFromMint> for WithdrawWithheldTokensFromMintCodec {
    fn encoded_len(&self, value: &WithdrawWithheldTokensFromMint) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &WithdrawWithheldTokensFromMint,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[26, 2]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<WithdrawWithheldTokensFromMint> for WithdrawWithheldTokensFromMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<WithdrawWithheldTokensFromMint, Error> {
        Constant(&[26, 2]).read(reader)?;
        let _ = reader;
        Ok(WithdrawWithheldTokensFromMint {})
    }
}
/// Validated data for `WithdrawWithheldTokensFromAccounts`.
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawWithheldTokensFromAccounts {
    /// The `num_token_accounts` wire field.
    pub num_token_accounts: u8,
}
/// Bounded codec for [`WithdrawWithheldTokensFromAccounts`].
#[derive(Clone, Copy, Debug)]
pub struct WithdrawWithheldTokensFromAccountsCodec;
impl Encoder<WithdrawWithheldTokensFromAccounts> for WithdrawWithheldTokensFromAccountsCodec {
    fn encoded_len(&self, value: &WithdrawWithheldTokensFromAccounts) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.num_token_accounts)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &WithdrawWithheldTokensFromAccounts,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[26, 3]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.num_token_accounts, writer)?;
        Ok(())
    }
}
impl Decoder<WithdrawWithheldTokensFromAccounts> for WithdrawWithheldTokensFromAccountsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<WithdrawWithheldTokensFromAccounts, Error> {
        Constant(&[26, 3]).read(reader)?;
        Ok(WithdrawWithheldTokensFromAccounts {
            num_token_accounts: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `HarvestWithheldTokensToMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct HarvestWithheldTokensToMint {}
/// Bounded codec for [`HarvestWithheldTokensToMint`].
#[derive(Clone, Copy, Debug)]
pub struct HarvestWithheldTokensToMintCodec;
impl Encoder<HarvestWithheldTokensToMint> for HarvestWithheldTokensToMintCodec {
    fn encoded_len(&self, value: &HarvestWithheldTokensToMint) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &HarvestWithheldTokensToMint,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[26, 4]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<HarvestWithheldTokensToMint> for HarvestWithheldTokensToMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<HarvestWithheldTokensToMint, Error> {
        Constant(&[26, 4]).read(reader)?;
        let _ = reader;
        Ok(HarvestWithheldTokensToMint {})
    }
}
/// Validated data for `SetTransferFee`.
#[derive(Clone, Debug, PartialEq)]
pub struct SetTransferFee {
    /// The `transfer_fee_basis_points` wire field.
    pub transfer_fee_basis_points: u16,
    /// The `maximum_fee` wire field.
    pub maximum_fee: u64,
}
/// Bounded codec for [`SetTransferFee`].
#[derive(Clone, Copy, Debug)]
pub struct SetTransferFeeCodec;
impl Encoder<SetTransferFee> for SetTransferFeeCodec {
    fn encoded_len(&self, value: &SetTransferFee) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.transfer_fee_basis_points)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.maximum_fee)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetTransferFee, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[26, 5]).write(&(), writer)?;
        Number::<u16>::LE.write(&value.transfer_fee_basis_points, writer)?;
        Number::<u64>::LE.write(&value.maximum_fee, writer)?;
        Ok(())
    }
}
impl Decoder<SetTransferFee> for SetTransferFeeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetTransferFee, Error> {
        Constant(&[26, 5]).read(reader)?;
        Ok(SetTransferFee {
            transfer_fee_basis_points: Number::<u16>::LE.read(reader)?,
            maximum_fee: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `InitializeConfidentialTransferMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeConfidentialTransferMint {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `auto_approve_new_accounts` wire field.
    pub auto_approve_new_accounts: bool,
    /// The `auditor_elgamal_pubkey` wire field.
    pub auditor_elgamal_pubkey: Option<Address>,
}
/// Bounded codec for [`InitializeConfidentialTransferMint`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeConfidentialTransferMintCodec;
impl Encoder<InitializeConfidentialTransferMint> for InitializeConfidentialTransferMintCodec {
    fn encoded_len(&self, value: &InitializeConfidentialTransferMint) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.auto_approve_new_accounts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.auditor_elgamal_pubkey)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeConfidentialTransferMint,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Bool.write(&value.auto_approve_new_accounts, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.auditor_elgamal_pubkey, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeConfidentialTransferMint> for InitializeConfidentialTransferMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeConfidentialTransferMint, Error> {
        Constant(&[27, 0]).read(reader)?;
        Ok(InitializeConfidentialTransferMint {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            auto_approve_new_accounts: Bool.read(reader)?,
            auditor_elgamal_pubkey: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `UpdateConfidentialTransferMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateConfidentialTransferMint {
    /// The `auto_approve_new_accounts` wire field.
    pub auto_approve_new_accounts: bool,
    /// The `auditor_elgamal_pubkey` wire field.
    pub auditor_elgamal_pubkey: Option<Address>,
}
/// Bounded codec for [`UpdateConfidentialTransferMint`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateConfidentialTransferMintCodec;
impl Encoder<UpdateConfidentialTransferMint> for UpdateConfidentialTransferMintCodec {
    fn encoded_len(&self, value: &UpdateConfidentialTransferMint) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.auto_approve_new_accounts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.auditor_elgamal_pubkey)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateConfidentialTransferMint,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 1]).write(&(), writer)?;
        Bool.write(&value.auto_approve_new_accounts, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.auditor_elgamal_pubkey, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateConfidentialTransferMint> for UpdateConfidentialTransferMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateConfidentialTransferMint, Error> {
        Constant(&[27, 1]).read(reader)?;
        Ok(UpdateConfidentialTransferMint {
            auto_approve_new_accounts: Bool.read(reader)?,
            auditor_elgamal_pubkey: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `ConfigureConfidentialTransferAccount`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigureConfidentialTransferAccount {
    /// The `decryptable_zero_balance` wire field.
    pub decryptable_zero_balance: DecryptableBalance,
    /// The `maximum_pending_balance_credit_counter` wire field.
    pub maximum_pending_balance_credit_counter: u64,
    /// The `proof_instruction_offset` wire field.
    pub proof_instruction_offset: i8,
}
/// Bounded codec for [`ConfigureConfidentialTransferAccount`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigureConfidentialTransferAccountCodec;
impl Encoder<ConfigureConfidentialTransferAccount> for ConfigureConfidentialTransferAccountCodec {
    fn encoded_len(&self, value: &ConfigureConfidentialTransferAccount) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(DecryptableBalanceCodec.encoded_len(&value.decryptable_zero_balance)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<u64>::LE.encoded_len(&value.maximum_pending_balance_credit_counter)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfigureConfidentialTransferAccount,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 2]).write(&(), writer)?;
        DecryptableBalanceCodec.write(&value.decryptable_zero_balance, writer)?;
        Number::<u64>::LE.write(&value.maximum_pending_balance_credit_counter, writer)?;
        Number::<i8>::LE.write(&value.proof_instruction_offset, writer)?;
        Ok(())
    }
}
impl Decoder<ConfigureConfidentialTransferAccount> for ConfigureConfidentialTransferAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigureConfidentialTransferAccount, Error> {
        Constant(&[27, 2]).read(reader)?;
        Ok(ConfigureConfidentialTransferAccount {
            decryptable_zero_balance: DecryptableBalanceCodec.read(reader)?,
            maximum_pending_balance_credit_counter: Number::<u64>::LE.read(reader)?,
            proof_instruction_offset: Number::<i8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ApproveConfidentialTransferAccount`.
#[derive(Clone, Debug, PartialEq)]
pub struct ApproveConfidentialTransferAccount {}
/// Bounded codec for [`ApproveConfidentialTransferAccount`].
#[derive(Clone, Copy, Debug)]
pub struct ApproveConfidentialTransferAccountCodec;
impl Encoder<ApproveConfidentialTransferAccount> for ApproveConfidentialTransferAccountCodec {
    fn encoded_len(&self, value: &ApproveConfidentialTransferAccount) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &ApproveConfidentialTransferAccount,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 3]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ApproveConfidentialTransferAccount> for ApproveConfidentialTransferAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ApproveConfidentialTransferAccount, Error> {
        Constant(&[27, 3]).read(reader)?;
        let _ = reader;
        Ok(ApproveConfidentialTransferAccount {})
    }
}
/// Validated data for `EmptyConfidentialTransferAccount`.
#[derive(Clone, Debug, PartialEq)]
pub struct EmptyConfidentialTransferAccount {
    /// The `proof_instruction_offset` wire field.
    pub proof_instruction_offset: i8,
}
/// Bounded codec for [`EmptyConfidentialTransferAccount`].
#[derive(Clone, Copy, Debug)]
pub struct EmptyConfidentialTransferAccountCodec;
impl Encoder<EmptyConfidentialTransferAccount> for EmptyConfidentialTransferAccountCodec {
    fn encoded_len(&self, value: &EmptyConfidentialTransferAccount) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &EmptyConfidentialTransferAccount,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 4]).write(&(), writer)?;
        Number::<i8>::LE.write(&value.proof_instruction_offset, writer)?;
        Ok(())
    }
}
impl Decoder<EmptyConfidentialTransferAccount> for EmptyConfidentialTransferAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EmptyConfidentialTransferAccount, Error> {
        Constant(&[27, 4]).read(reader)?;
        Ok(EmptyConfidentialTransferAccount {
            proof_instruction_offset: Number::<i8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ConfidentialDeposit`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfidentialDeposit {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
}
/// Bounded codec for [`ConfidentialDeposit`].
#[derive(Clone, Copy, Debug)]
pub struct ConfidentialDepositCodec;
impl Encoder<ConfidentialDeposit> for ConfidentialDepositCodec {
    fn encoded_len(&self, value: &ConfidentialDeposit) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ConfidentialDeposit, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[27, 5]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Ok(())
    }
}
impl Decoder<ConfidentialDeposit> for ConfidentialDepositCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfidentialDeposit, Error> {
        Constant(&[27, 5]).read(reader)?;
        Ok(ConfidentialDeposit {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ConfidentialWithdraw`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfidentialWithdraw {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
    /// The `new_decryptable_available_balance` wire field.
    pub new_decryptable_available_balance: DecryptableBalance,
    /// The `equality_proof_instruction_offset` wire field.
    pub equality_proof_instruction_offset: i8,
    /// The `range_proof_instruction_offset` wire field.
    pub range_proof_instruction_offset: i8,
}
/// Bounded codec for [`ConfidentialWithdraw`].
#[derive(Clone, Copy, Debug)]
pub struct ConfidentialWithdrawCodec;
impl Encoder<ConfidentialWithdraw> for ConfidentialWithdrawCodec {
    fn encoded_len(&self, value: &ConfidentialWithdraw) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                DecryptableBalanceCodec.encoded_len(&value.new_decryptable_available_balance)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.equality_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.range_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ConfidentialWithdraw, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[27, 6]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        DecryptableBalanceCodec.write(&value.new_decryptable_available_balance, writer)?;
        Number::<i8>::LE.write(&value.equality_proof_instruction_offset, writer)?;
        Number::<i8>::LE.write(&value.range_proof_instruction_offset, writer)?;
        Ok(())
    }
}
impl Decoder<ConfidentialWithdraw> for ConfidentialWithdrawCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfidentialWithdraw, Error> {
        Constant(&[27, 6]).read(reader)?;
        Ok(ConfidentialWithdraw {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
            new_decryptable_available_balance: DecryptableBalanceCodec.read(reader)?,
            equality_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            range_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ConfidentialTransfer`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfidentialTransfer {
    /// The `new_source_decryptable_available_balance` wire field.
    pub new_source_decryptable_available_balance: DecryptableBalance,
    /// The `transfer_amount_auditor_ciphertext_lo` wire field.
    pub transfer_amount_auditor_ciphertext_lo: EncryptedBalance,
    /// The `transfer_amount_auditor_ciphertext_hi` wire field.
    pub transfer_amount_auditor_ciphertext_hi: EncryptedBalance,
    /// The `equality_proof_instruction_offset` wire field.
    pub equality_proof_instruction_offset: i8,
    /// The `ciphertext_validity_proof_instruction_offset` wire field.
    pub ciphertext_validity_proof_instruction_offset: i8,
    /// The `range_proof_instruction_offset` wire field.
    pub range_proof_instruction_offset: i8,
}
/// Bounded codec for [`ConfidentialTransfer`].
#[derive(Clone, Copy, Debug)]
pub struct ConfidentialTransferCodec;
impl Encoder<ConfidentialTransfer> for ConfidentialTransferCodec {
    fn encoded_len(&self, value: &ConfidentialTransfer) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                DecryptableBalanceCodec
                    .encoded_len(&value.new_source_decryptable_available_balance)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                EncryptedBalanceCodec.encoded_len(&value.transfer_amount_auditor_ciphertext_lo)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                EncryptedBalanceCodec.encoded_len(&value.transfer_amount_auditor_ciphertext_hi)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.equality_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<i8>::LE
                    .encoded_len(&value.ciphertext_validity_proof_instruction_offset)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.range_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ConfidentialTransfer, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[27, 7]).write(&(), writer)?;
        DecryptableBalanceCodec.write(&value.new_source_decryptable_available_balance, writer)?;
        EncryptedBalanceCodec.write(&value.transfer_amount_auditor_ciphertext_lo, writer)?;
        EncryptedBalanceCodec.write(&value.transfer_amount_auditor_ciphertext_hi, writer)?;
        Number::<i8>::LE.write(&value.equality_proof_instruction_offset, writer)?;
        Number::<i8>::LE.write(&value.ciphertext_validity_proof_instruction_offset, writer)?;
        Number::<i8>::LE.write(&value.range_proof_instruction_offset, writer)?;
        Ok(())
    }
}
impl Decoder<ConfidentialTransfer> for ConfidentialTransferCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfidentialTransfer, Error> {
        Constant(&[27, 7]).read(reader)?;
        Ok(ConfidentialTransfer {
            new_source_decryptable_available_balance: DecryptableBalanceCodec.read(reader)?,
            transfer_amount_auditor_ciphertext_lo: EncryptedBalanceCodec.read(reader)?,
            transfer_amount_auditor_ciphertext_hi: EncryptedBalanceCodec.read(reader)?,
            equality_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            ciphertext_validity_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            range_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ApplyConfidentialPendingBalance`.
#[derive(Clone, Debug, PartialEq)]
pub struct ApplyConfidentialPendingBalance {
    /// The `expected_pending_balance_credit_counter` wire field.
    pub expected_pending_balance_credit_counter: u64,
    /// The `new_decryptable_available_balance` wire field.
    pub new_decryptable_available_balance: DecryptableBalance,
}
/// Bounded codec for [`ApplyConfidentialPendingBalance`].
#[derive(Clone, Copy, Debug)]
pub struct ApplyConfidentialPendingBalanceCodec;
impl Encoder<ApplyConfidentialPendingBalance> for ApplyConfidentialPendingBalanceCodec {
    fn encoded_len(&self, value: &ApplyConfidentialPendingBalance) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Number::<u64>::LE.encoded_len(&value.expected_pending_balance_credit_counter)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                DecryptableBalanceCodec.encoded_len(&value.new_decryptable_available_balance)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ApplyConfidentialPendingBalance,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 8]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.expected_pending_balance_credit_counter, writer)?;
        DecryptableBalanceCodec.write(&value.new_decryptable_available_balance, writer)?;
        Ok(())
    }
}
impl Decoder<ApplyConfidentialPendingBalance> for ApplyConfidentialPendingBalanceCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ApplyConfidentialPendingBalance, Error> {
        Constant(&[27, 8]).read(reader)?;
        Ok(ApplyConfidentialPendingBalance {
            expected_pending_balance_credit_counter: Number::<u64>::LE.read(reader)?,
            new_decryptable_available_balance: DecryptableBalanceCodec.read(reader)?,
        })
    }
}
/// Validated data for `EnableConfidentialCredits`.
#[derive(Clone, Debug, PartialEq)]
pub struct EnableConfidentialCredits {}
/// Bounded codec for [`EnableConfidentialCredits`].
#[derive(Clone, Copy, Debug)]
pub struct EnableConfidentialCreditsCodec;
impl Encoder<EnableConfidentialCredits> for EnableConfidentialCreditsCodec {
    fn encoded_len(&self, value: &EnableConfidentialCredits) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &EnableConfidentialCredits,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 9]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<EnableConfidentialCredits> for EnableConfidentialCreditsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EnableConfidentialCredits, Error> {
        Constant(&[27, 9]).read(reader)?;
        let _ = reader;
        Ok(EnableConfidentialCredits {})
    }
}
/// Validated data for `DisableConfidentialCredits`.
#[derive(Clone, Debug, PartialEq)]
pub struct DisableConfidentialCredits {}
/// Bounded codec for [`DisableConfidentialCredits`].
#[derive(Clone, Copy, Debug)]
pub struct DisableConfidentialCreditsCodec;
impl Encoder<DisableConfidentialCredits> for DisableConfidentialCreditsCodec {
    fn encoded_len(&self, value: &DisableConfidentialCredits) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &DisableConfidentialCredits,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 10]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DisableConfidentialCredits> for DisableConfidentialCreditsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DisableConfidentialCredits, Error> {
        Constant(&[27, 10]).read(reader)?;
        let _ = reader;
        Ok(DisableConfidentialCredits {})
    }
}
/// Validated data for `EnableNonConfidentialCredits`.
#[derive(Clone, Debug, PartialEq)]
pub struct EnableNonConfidentialCredits {}
/// Bounded codec for [`EnableNonConfidentialCredits`].
#[derive(Clone, Copy, Debug)]
pub struct EnableNonConfidentialCreditsCodec;
impl Encoder<EnableNonConfidentialCredits> for EnableNonConfidentialCreditsCodec {
    fn encoded_len(&self, value: &EnableNonConfidentialCredits) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &EnableNonConfidentialCredits,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 11]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<EnableNonConfidentialCredits> for EnableNonConfidentialCreditsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EnableNonConfidentialCredits, Error> {
        Constant(&[27, 11]).read(reader)?;
        let _ = reader;
        Ok(EnableNonConfidentialCredits {})
    }
}
/// Validated data for `DisableNonConfidentialCredits`.
#[derive(Clone, Debug, PartialEq)]
pub struct DisableNonConfidentialCredits {}
/// Bounded codec for [`DisableNonConfidentialCredits`].
#[derive(Clone, Copy, Debug)]
pub struct DisableNonConfidentialCreditsCodec;
impl Encoder<DisableNonConfidentialCredits> for DisableNonConfidentialCreditsCodec {
    fn encoded_len(&self, value: &DisableNonConfidentialCredits) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &DisableNonConfidentialCredits,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 12]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DisableNonConfidentialCredits> for DisableNonConfidentialCreditsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DisableNonConfidentialCredits, Error> {
        Constant(&[27, 12]).read(reader)?;
        let _ = reader;
        Ok(DisableNonConfidentialCredits {})
    }
}
/// Validated data for `ConfidentialTransferWithFee`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfidentialTransferWithFee {
    /// The `new_source_decryptable_available_balance` wire field.
    pub new_source_decryptable_available_balance: DecryptableBalance,
    /// The `transfer_amount_auditor_ciphertext_lo` wire field.
    pub transfer_amount_auditor_ciphertext_lo: EncryptedBalance,
    /// The `transfer_amount_auditor_ciphertext_hi` wire field.
    pub transfer_amount_auditor_ciphertext_hi: EncryptedBalance,
    /// The `equality_proof_instruction_offset` wire field.
    pub equality_proof_instruction_offset: i8,
    /// The `transfer_amount_ciphertext_validity_proof_instruction_offset` wire field.
    pub transfer_amount_ciphertext_validity_proof_instruction_offset: i8,
    /// The `fee_sigma_proof_instruction_offset` wire field.
    pub fee_sigma_proof_instruction_offset: i8,
    /// The `fee_ciphertext_validity_proof_instruction_offset` wire field.
    pub fee_ciphertext_validity_proof_instruction_offset: i8,
    /// The `range_proof_instruction_offset` wire field.
    pub range_proof_instruction_offset: i8,
}
/// Bounded codec for [`ConfidentialTransferWithFee`].
#[derive(Clone, Copy, Debug)]
pub struct ConfidentialTransferWithFeeCodec;
impl Encoder<ConfidentialTransferWithFee> for ConfidentialTransferWithFeeCodec {
    fn encoded_len(&self, value: &ConfidentialTransferWithFee) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                DecryptableBalanceCodec
                    .encoded_len(&value.new_source_decryptable_available_balance)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                EncryptedBalanceCodec.encoded_len(&value.transfer_amount_auditor_ciphertext_lo)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                EncryptedBalanceCodec.encoded_len(&value.transfer_amount_auditor_ciphertext_hi)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.equality_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        let size =
            size.checked_add(Number::<i8>::LE.encoded_len(
                &value.transfer_amount_ciphertext_validity_proof_instruction_offset,
            )?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.fee_sigma_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<i8>::LE
                    .encoded_len(&value.fee_ciphertext_validity_proof_instruction_offset)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.range_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfidentialTransferWithFee,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 13]).write(&(), writer)?;
        DecryptableBalanceCodec.write(&value.new_source_decryptable_available_balance, writer)?;
        EncryptedBalanceCodec.write(&value.transfer_amount_auditor_ciphertext_lo, writer)?;
        EncryptedBalanceCodec.write(&value.transfer_amount_auditor_ciphertext_hi, writer)?;
        Number::<i8>::LE.write(&value.equality_proof_instruction_offset, writer)?;
        Number::<i8>::LE.write(
            &value.transfer_amount_ciphertext_validity_proof_instruction_offset,
            writer,
        )?;
        Number::<i8>::LE.write(&value.fee_sigma_proof_instruction_offset, writer)?;
        Number::<i8>::LE.write(
            &value.fee_ciphertext_validity_proof_instruction_offset,
            writer,
        )?;
        Number::<i8>::LE.write(&value.range_proof_instruction_offset, writer)?;
        Ok(())
    }
}
impl Decoder<ConfidentialTransferWithFee> for ConfidentialTransferWithFeeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfidentialTransferWithFee, Error> {
        Constant(&[27, 13]).read(reader)?;
        Ok(ConfidentialTransferWithFee {
            new_source_decryptable_available_balance: DecryptableBalanceCodec.read(reader)?,
            transfer_amount_auditor_ciphertext_lo: EncryptedBalanceCodec.read(reader)?,
            transfer_amount_auditor_ciphertext_hi: EncryptedBalanceCodec.read(reader)?,
            equality_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            transfer_amount_ciphertext_validity_proof_instruction_offset: Number::<i8>::LE
                .read(reader)?,
            fee_sigma_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            fee_ciphertext_validity_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            range_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ConfigureConfidentialTransferAccountWithRegistry`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigureConfidentialTransferAccountWithRegistry {}
/// Bounded codec for [`ConfigureConfidentialTransferAccountWithRegistry`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigureConfidentialTransferAccountWithRegistryCodec;
impl Encoder<ConfigureConfidentialTransferAccountWithRegistry>
    for ConfigureConfidentialTransferAccountWithRegistryCodec
{
    fn encoded_len(
        &self,
        value: &ConfigureConfidentialTransferAccountWithRegistry,
    ) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConfigureConfidentialTransferAccountWithRegistry,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[27, 14]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ConfigureConfidentialTransferAccountWithRegistry>
    for ConfigureConfidentialTransferAccountWithRegistryCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<ConfigureConfidentialTransferAccountWithRegistry, Error> {
        Constant(&[27, 14]).read(reader)?;
        let _ = reader;
        Ok(ConfigureConfidentialTransferAccountWithRegistry {})
    }
}
/// Validated data for `InitializeDefaultAccountState`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeDefaultAccountState {
    /// The `state` wire field.
    pub state: AccountState,
}
/// Bounded codec for [`InitializeDefaultAccountState`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeDefaultAccountStateCodec;
impl Encoder<InitializeDefaultAccountState> for InitializeDefaultAccountStateCodec {
    fn encoded_len(&self, value: &InitializeDefaultAccountState) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(AccountStateCodec.encoded_len(&value.state)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeDefaultAccountState,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[28, 0]).write(&(), writer)?;
        AccountStateCodec.write(&value.state, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeDefaultAccountState> for InitializeDefaultAccountStateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeDefaultAccountState, Error> {
        Constant(&[28, 0]).read(reader)?;
        Ok(InitializeDefaultAccountState {
            state: AccountStateCodec.read(reader)?,
        })
    }
}
/// Validated data for `UpdateDefaultAccountState`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDefaultAccountState {
    /// The `state` wire field.
    pub state: AccountState,
}
/// Bounded codec for [`UpdateDefaultAccountState`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateDefaultAccountStateCodec;
impl Encoder<UpdateDefaultAccountState> for UpdateDefaultAccountStateCodec {
    fn encoded_len(&self, value: &UpdateDefaultAccountState) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(AccountStateCodec.encoded_len(&value.state)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateDefaultAccountState,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[28, 1]).write(&(), writer)?;
        AccountStateCodec.write(&value.state, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateDefaultAccountState> for UpdateDefaultAccountStateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateDefaultAccountState, Error> {
        Constant(&[28, 1]).read(reader)?;
        Ok(UpdateDefaultAccountState {
            state: AccountStateCodec.read(reader)?,
        })
    }
}
/// Validated data for `Reallocate`.
#[derive(Clone, Debug, PartialEq)]
pub struct Reallocate {
    /// The `new_extension_types` wire field.
    pub new_extension_types: Vec<ExtensionType>,
}
/// Bounded codec for [`Reallocate`].
#[derive(Clone, Copy, Debug)]
pub struct ReallocateCodec;
impl Encoder<Reallocate> for ReallocateCodec {
    fn encoded_len(&self, value: &Reallocate) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: ExtensionTypeCodec,
                    count: Count::Remainder,
                })
                .encoded_len(&value.new_extension_types)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Reallocate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[29]).write(&(), writer)?;
        OwnedSequence(Sequence {
            item: ExtensionTypeCodec,
            count: Count::Remainder,
        })
        .write(&value.new_extension_types, writer)?;
        Ok(())
    }
}
impl Decoder<Reallocate> for ReallocateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Reallocate, Error> {
        Constant(&[29]).read(reader)?;
        Ok(Reallocate {
            new_extension_types: OwnedSequence(Sequence {
                item: ExtensionTypeCodec,
                count: Count::Remainder,
            })
            .read(reader)?,
        })
    }
}
/// Validated data for `EnableMemoTransfers`.
#[derive(Clone, Debug, PartialEq)]
pub struct EnableMemoTransfers {}
/// Bounded codec for [`EnableMemoTransfers`].
#[derive(Clone, Copy, Debug)]
pub struct EnableMemoTransfersCodec;
impl Encoder<EnableMemoTransfers> for EnableMemoTransfersCodec {
    fn encoded_len(&self, value: &EnableMemoTransfers) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &EnableMemoTransfers, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[30, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<EnableMemoTransfers> for EnableMemoTransfersCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EnableMemoTransfers, Error> {
        Constant(&[30, 0]).read(reader)?;
        let _ = reader;
        Ok(EnableMemoTransfers {})
    }
}
/// Validated data for `DisableMemoTransfers`.
#[derive(Clone, Debug, PartialEq)]
pub struct DisableMemoTransfers {}
/// Bounded codec for [`DisableMemoTransfers`].
#[derive(Clone, Copy, Debug)]
pub struct DisableMemoTransfersCodec;
impl Encoder<DisableMemoTransfers> for DisableMemoTransfersCodec {
    fn encoded_len(&self, value: &DisableMemoTransfers) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &DisableMemoTransfers, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[30, 1]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DisableMemoTransfers> for DisableMemoTransfersCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DisableMemoTransfers, Error> {
        Constant(&[30, 1]).read(reader)?;
        let _ = reader;
        Ok(DisableMemoTransfers {})
    }
}
/// Validated data for `CreateNativeMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct CreateNativeMint {}
/// Bounded codec for [`CreateNativeMint`].
#[derive(Clone, Copy, Debug)]
pub struct CreateNativeMintCodec;
impl Encoder<CreateNativeMint> for CreateNativeMintCodec {
    fn encoded_len(&self, value: &CreateNativeMint) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CreateNativeMint, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[31]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CreateNativeMint> for CreateNativeMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateNativeMint, Error> {
        Constant(&[31]).read(reader)?;
        let _ = reader;
        Ok(CreateNativeMint {})
    }
}
/// Validated data for `InitializeNonTransferableMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeNonTransferableMint {}
/// Bounded codec for [`InitializeNonTransferableMint`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeNonTransferableMintCodec;
impl Encoder<InitializeNonTransferableMint> for InitializeNonTransferableMintCodec {
    fn encoded_len(&self, value: &InitializeNonTransferableMint) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeNonTransferableMint,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[32]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<InitializeNonTransferableMint> for InitializeNonTransferableMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeNonTransferableMint, Error> {
        Constant(&[32]).read(reader)?;
        let _ = reader;
        Ok(InitializeNonTransferableMint {})
    }
}
/// Validated data for `InitializeInterestBearingMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeInterestBearingMint {
    /// The `rate_authority` wire field.
    pub rate_authority: Option<Address>,
    /// The `rate` wire field.
    pub rate: i16,
}
/// Bounded codec for [`InitializeInterestBearingMint`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeInterestBearingMintCodec;
impl Encoder<InitializeInterestBearingMint> for InitializeInterestBearingMintCodec {
    fn encoded_len(&self, value: &InitializeInterestBearingMint) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.rate_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i16>::LE.encoded_len(&value.rate)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeInterestBearingMint,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[33, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.rate_authority, writer)?;
        Number::<i16>::LE.write(&value.rate, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeInterestBearingMint> for InitializeInterestBearingMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeInterestBearingMint, Error> {
        Constant(&[33, 0]).read(reader)?;
        Ok(InitializeInterestBearingMint {
            rate_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            rate: Number::<i16>::LE.read(reader)?,
        })
    }
}
/// Validated data for `UpdateRateInterestBearingMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRateInterestBearingMint {
    /// The `rate` wire field.
    pub rate: i16,
}
/// Bounded codec for [`UpdateRateInterestBearingMint`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateRateInterestBearingMintCodec;
impl Encoder<UpdateRateInterestBearingMint> for UpdateRateInterestBearingMintCodec {
    fn encoded_len(&self, value: &UpdateRateInterestBearingMint) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<i16>::LE.encoded_len(&value.rate)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateRateInterestBearingMint,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[33, 1]).write(&(), writer)?;
        Number::<i16>::LE.write(&value.rate, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateRateInterestBearingMint> for UpdateRateInterestBearingMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateRateInterestBearingMint, Error> {
        Constant(&[33, 1]).read(reader)?;
        Ok(UpdateRateInterestBearingMint {
            rate: Number::<i16>::LE.read(reader)?,
        })
    }
}
/// Validated data for `EnableCpiGuard`.
#[derive(Clone, Debug, PartialEq)]
pub struct EnableCpiGuard {}
/// Bounded codec for [`EnableCpiGuard`].
#[derive(Clone, Copy, Debug)]
pub struct EnableCpiGuardCodec;
impl Encoder<EnableCpiGuard> for EnableCpiGuardCodec {
    fn encoded_len(&self, value: &EnableCpiGuard) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &EnableCpiGuard, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[34, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<EnableCpiGuard> for EnableCpiGuardCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EnableCpiGuard, Error> {
        Constant(&[34, 0]).read(reader)?;
        let _ = reader;
        Ok(EnableCpiGuard {})
    }
}
/// Validated data for `DisableCpiGuard`.
#[derive(Clone, Debug, PartialEq)]
pub struct DisableCpiGuard {}
/// Bounded codec for [`DisableCpiGuard`].
#[derive(Clone, Copy, Debug)]
pub struct DisableCpiGuardCodec;
impl Encoder<DisableCpiGuard> for DisableCpiGuardCodec {
    fn encoded_len(&self, value: &DisableCpiGuard) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &DisableCpiGuard, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[34, 1]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DisableCpiGuard> for DisableCpiGuardCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DisableCpiGuard, Error> {
        Constant(&[34, 1]).read(reader)?;
        let _ = reader;
        Ok(DisableCpiGuard {})
    }
}
/// Validated data for `InitializePermanentDelegate`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePermanentDelegate {
    /// The `delegate` wire field.
    pub delegate: Address,
}
/// Bounded codec for [`InitializePermanentDelegate`].
#[derive(Clone, Copy, Debug)]
pub struct InitializePermanentDelegateCodec;
impl Encoder<InitializePermanentDelegate> for InitializePermanentDelegateCodec {
    fn encoded_len(&self, value: &InitializePermanentDelegate) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegate)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializePermanentDelegate,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[35]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegate, writer)?;
        Ok(())
    }
}
impl Decoder<InitializePermanentDelegate> for InitializePermanentDelegateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializePermanentDelegate, Error> {
        Constant(&[35]).read(reader)?;
        Ok(InitializePermanentDelegate {
            delegate: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeTransferHook`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTransferHook {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `program_id` wire field.
    pub program_id: Option<Address>,
}
/// Bounded codec for [`InitializeTransferHook`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeTransferHookCodec;
impl Encoder<InitializeTransferHook> for InitializeTransferHookCodec {
    fn encoded_len(&self, value: &InitializeTransferHook) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.program_id)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeTransferHook, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[36, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.program_id, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeTransferHook> for InitializeTransferHookCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeTransferHook, Error> {
        Constant(&[36, 0]).read(reader)?;
        Ok(InitializeTransferHook {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            program_id: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `UpdateTransferHook`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTransferHook {
    /// The `program_id` wire field.
    pub program_id: Option<Address>,
}
/// Bounded codec for [`UpdateTransferHook`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateTransferHookCodec;
impl Encoder<UpdateTransferHook> for UpdateTransferHookCodec {
    fn encoded_len(&self, value: &UpdateTransferHook) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.program_id)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateTransferHook, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[36, 1]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.program_id, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateTransferHook> for UpdateTransferHookCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateTransferHook, Error> {
        Constant(&[36, 1]).read(reader)?;
        Ok(UpdateTransferHook {
            program_id: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeConfidentialTransferFee`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeConfidentialTransferFee {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `withdraw_withheld_authority_el_gamal_pubkey` wire field.
    pub withdraw_withheld_authority_el_gamal_pubkey: Address,
}
/// Bounded codec for [`InitializeConfidentialTransferFee`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeConfidentialTransferFeeCodec;
impl Encoder<InitializeConfidentialTransferFee> for InitializeConfidentialTransferFeeCodec {
    fn encoded_len(&self, value: &InitializeConfidentialTransferFee) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.withdraw_withheld_authority_el_gamal_pubkey)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeConfidentialTransferFee,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[37, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.withdraw_withheld_authority_el_gamal_pubkey, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeConfidentialTransferFee> for InitializeConfidentialTransferFeeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeConfidentialTransferFee, Error> {
        Constant(&[37, 0]).read(reader)?;
        Ok(InitializeConfidentialTransferFee {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            withdraw_withheld_authority_el_gamal_pubkey: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `WithdrawWithheldTokensFromMintForConfidentialTransferFee`.
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawWithheldTokensFromMintForConfidentialTransferFee {
    /// The `proof_instruction_offset` wire field.
    pub proof_instruction_offset: i8,
    /// The `new_decryptable_available_balance` wire field.
    pub new_decryptable_available_balance: DecryptableBalance,
}
/// Bounded codec for [`WithdrawWithheldTokensFromMintForConfidentialTransferFee`].
#[derive(Clone, Copy, Debug)]
pub struct WithdrawWithheldTokensFromMintForConfidentialTransferFeeCodec;
impl Encoder<WithdrawWithheldTokensFromMintForConfidentialTransferFee>
    for WithdrawWithheldTokensFromMintForConfidentialTransferFeeCodec
{
    fn encoded_len(
        &self,
        value: &WithdrawWithheldTokensFromMintForConfidentialTransferFee,
    ) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                DecryptableBalanceCodec.encoded_len(&value.new_decryptable_available_balance)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &WithdrawWithheldTokensFromMintForConfidentialTransferFee,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[37, 1]).write(&(), writer)?;
        Number::<i8>::LE.write(&value.proof_instruction_offset, writer)?;
        DecryptableBalanceCodec.write(&value.new_decryptable_available_balance, writer)?;
        Ok(())
    }
}
impl Decoder<WithdrawWithheldTokensFromMintForConfidentialTransferFee>
    for WithdrawWithheldTokensFromMintForConfidentialTransferFeeCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<WithdrawWithheldTokensFromMintForConfidentialTransferFee, Error> {
        Constant(&[37, 1]).read(reader)?;
        Ok(WithdrawWithheldTokensFromMintForConfidentialTransferFee {
            proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            new_decryptable_available_balance: DecryptableBalanceCodec.read(reader)?,
        })
    }
}
/// Validated data for `WithdrawWithheldTokensFromAccountsForConfidentialTransferFee`.
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawWithheldTokensFromAccountsForConfidentialTransferFee {
    /// The `num_token_accounts` wire field.
    pub num_token_accounts: u8,
    /// The `proof_instruction_offset` wire field.
    pub proof_instruction_offset: i8,
    /// The `new_decryptable_available_balance` wire field.
    pub new_decryptable_available_balance: DecryptableBalance,
}
/// Bounded codec for [`WithdrawWithheldTokensFromAccountsForConfidentialTransferFee`].
#[derive(Clone, Copy, Debug)]
pub struct WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeCodec;
impl Encoder<WithdrawWithheldTokensFromAccountsForConfidentialTransferFee>
    for WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeCodec
{
    fn encoded_len(
        &self,
        value: &WithdrawWithheldTokensFromAccountsForConfidentialTransferFee,
    ) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.num_token_accounts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                DecryptableBalanceCodec.encoded_len(&value.new_decryptable_available_balance)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &WithdrawWithheldTokensFromAccountsForConfidentialTransferFee,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[37, 2]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.num_token_accounts, writer)?;
        Number::<i8>::LE.write(&value.proof_instruction_offset, writer)?;
        DecryptableBalanceCodec.write(&value.new_decryptable_available_balance, writer)?;
        Ok(())
    }
}
impl Decoder<WithdrawWithheldTokensFromAccountsForConfidentialTransferFee>
    for WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<WithdrawWithheldTokensFromAccountsForConfidentialTransferFee, Error> {
        Constant(&[37, 2]).read(reader)?;
        Ok(
            WithdrawWithheldTokensFromAccountsForConfidentialTransferFee {
                num_token_accounts: Number::<u8>::LE.read(reader)?,
                proof_instruction_offset: Number::<i8>::LE.read(reader)?,
                new_decryptable_available_balance: DecryptableBalanceCodec.read(reader)?,
            },
        )
    }
}
/// Validated data for `HarvestWithheldTokensToMintForConfidentialTransferFee`.
#[derive(Clone, Debug, PartialEq)]
pub struct HarvestWithheldTokensToMintForConfidentialTransferFee {}
/// Bounded codec for [`HarvestWithheldTokensToMintForConfidentialTransferFee`].
#[derive(Clone, Copy, Debug)]
pub struct HarvestWithheldTokensToMintForConfidentialTransferFeeCodec;
impl Encoder<HarvestWithheldTokensToMintForConfidentialTransferFee>
    for HarvestWithheldTokensToMintForConfidentialTransferFeeCodec
{
    fn encoded_len(
        &self,
        value: &HarvestWithheldTokensToMintForConfidentialTransferFee,
    ) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &HarvestWithheldTokensToMintForConfidentialTransferFee,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[37, 3]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<HarvestWithheldTokensToMintForConfidentialTransferFee>
    for HarvestWithheldTokensToMintForConfidentialTransferFeeCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<HarvestWithheldTokensToMintForConfidentialTransferFee, Error> {
        Constant(&[37, 3]).read(reader)?;
        let _ = reader;
        Ok(HarvestWithheldTokensToMintForConfidentialTransferFee {})
    }
}
/// Validated data for `EnableHarvestToMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct EnableHarvestToMint {}
/// Bounded codec for [`EnableHarvestToMint`].
#[derive(Clone, Copy, Debug)]
pub struct EnableHarvestToMintCodec;
impl Encoder<EnableHarvestToMint> for EnableHarvestToMintCodec {
    fn encoded_len(&self, value: &EnableHarvestToMint) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &EnableHarvestToMint, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[37, 4]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<EnableHarvestToMint> for EnableHarvestToMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EnableHarvestToMint, Error> {
        Constant(&[37, 4]).read(reader)?;
        let _ = reader;
        Ok(EnableHarvestToMint {})
    }
}
/// Validated data for `DisableHarvestToMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct DisableHarvestToMint {}
/// Bounded codec for [`DisableHarvestToMint`].
#[derive(Clone, Copy, Debug)]
pub struct DisableHarvestToMintCodec;
impl Encoder<DisableHarvestToMint> for DisableHarvestToMintCodec {
    fn encoded_len(&self, value: &DisableHarvestToMint) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &DisableHarvestToMint, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[37, 5]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DisableHarvestToMint> for DisableHarvestToMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DisableHarvestToMint, Error> {
        Constant(&[37, 5]).read(reader)?;
        let _ = reader;
        Ok(DisableHarvestToMint {})
    }
}
/// Validated data for `WithdrawExcessLamports`.
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawExcessLamports {}
/// Bounded codec for [`WithdrawExcessLamports`].
#[derive(Clone, Copy, Debug)]
pub struct WithdrawExcessLamportsCodec;
impl Encoder<WithdrawExcessLamports> for WithdrawExcessLamportsCodec {
    fn encoded_len(&self, value: &WithdrawExcessLamports) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &WithdrawExcessLamports, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[38]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<WithdrawExcessLamports> for WithdrawExcessLamportsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<WithdrawExcessLamports, Error> {
        Constant(&[38]).read(reader)?;
        let _ = reader;
        Ok(WithdrawExcessLamports {})
    }
}
/// Validated data for `InitializeMetadataPointer`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMetadataPointer {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `metadata_address` wire field.
    pub metadata_address: Option<Address>,
}
/// Bounded codec for [`InitializeMetadataPointer`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeMetadataPointerCodec;
impl Encoder<InitializeMetadataPointer> for InitializeMetadataPointerCodec {
    fn encoded_len(&self, value: &InitializeMetadataPointer) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.metadata_address)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeMetadataPointer,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[39, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.metadata_address, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeMetadataPointer> for InitializeMetadataPointerCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeMetadataPointer, Error> {
        Constant(&[39, 0]).read(reader)?;
        Ok(InitializeMetadataPointer {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            metadata_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `UpdateMetadataPointer`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMetadataPointer {
    /// The `metadata_address` wire field.
    pub metadata_address: Option<Address>,
}
/// Bounded codec for [`UpdateMetadataPointer`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateMetadataPointerCodec;
impl Encoder<UpdateMetadataPointer> for UpdateMetadataPointerCodec {
    fn encoded_len(&self, value: &UpdateMetadataPointer) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.metadata_address)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateMetadataPointer, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[39, 1]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.metadata_address, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateMetadataPointer> for UpdateMetadataPointerCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateMetadataPointer, Error> {
        Constant(&[39, 1]).read(reader)?;
        Ok(UpdateMetadataPointer {
            metadata_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeGroupPointer`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeGroupPointer {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `group_address` wire field.
    pub group_address: Option<Address>,
}
/// Bounded codec for [`InitializeGroupPointer`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeGroupPointerCodec;
impl Encoder<InitializeGroupPointer> for InitializeGroupPointerCodec {
    fn encoded_len(&self, value: &InitializeGroupPointer) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.group_address)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeGroupPointer, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[40, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.group_address, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeGroupPointer> for InitializeGroupPointerCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeGroupPointer, Error> {
        Constant(&[40, 0]).read(reader)?;
        Ok(InitializeGroupPointer {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            group_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `UpdateGroupPointer`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateGroupPointer {
    /// The `group_address` wire field.
    pub group_address: Option<Address>,
}
/// Bounded codec for [`UpdateGroupPointer`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateGroupPointerCodec;
impl Encoder<UpdateGroupPointer> for UpdateGroupPointerCodec {
    fn encoded_len(&self, value: &UpdateGroupPointer) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.group_address)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateGroupPointer, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[40, 1]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.group_address, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateGroupPointer> for UpdateGroupPointerCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateGroupPointer, Error> {
        Constant(&[40, 1]).read(reader)?;
        Ok(UpdateGroupPointer {
            group_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeGroupMemberPointer`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeGroupMemberPointer {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `member_address` wire field.
    pub member_address: Option<Address>,
}
/// Bounded codec for [`InitializeGroupMemberPointer`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeGroupMemberPointerCodec;
impl Encoder<InitializeGroupMemberPointer> for InitializeGroupMemberPointerCodec {
    fn encoded_len(&self, value: &InitializeGroupMemberPointer) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.member_address)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeGroupMemberPointer,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[41, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.member_address, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeGroupMemberPointer> for InitializeGroupMemberPointerCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeGroupMemberPointer, Error> {
        Constant(&[41, 0]).read(reader)?;
        Ok(InitializeGroupMemberPointer {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            member_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `UpdateGroupMemberPointer`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateGroupMemberPointer {
    /// The `member_address` wire field.
    pub member_address: Option<Address>,
}
/// Bounded codec for [`UpdateGroupMemberPointer`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateGroupMemberPointerCodec;
impl Encoder<UpdateGroupMemberPointer> for UpdateGroupMemberPointerCodec {
    fn encoded_len(&self, value: &UpdateGroupMemberPointer) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.member_address)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateGroupMemberPointer,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[41, 1]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.member_address, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateGroupMemberPointer> for UpdateGroupMemberPointerCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateGroupMemberPointer, Error> {
        Constant(&[41, 1]).read(reader)?;
        Ok(UpdateGroupMemberPointer {
            member_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeConfidentialMintBurn`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeConfidentialMintBurn {
    /// The `supply_elgamal_pubkey` wire field.
    pub supply_elgamal_pubkey: Address,
    /// The `decryptable_supply` wire field.
    pub decryptable_supply: DecryptableBalance,
}
/// Bounded codec for [`InitializeConfidentialMintBurn`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeConfidentialMintBurnCodec;
impl Encoder<InitializeConfidentialMintBurn> for InitializeConfidentialMintBurnCodec {
    fn encoded_len(&self, value: &InitializeConfidentialMintBurn) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.supply_elgamal_pubkey)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(DecryptableBalanceCodec.encoded_len(&value.decryptable_supply)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeConfidentialMintBurn,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[42, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.supply_elgamal_pubkey, writer)?;
        DecryptableBalanceCodec.write(&value.decryptable_supply, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeConfidentialMintBurn> for InitializeConfidentialMintBurnCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeConfidentialMintBurn, Error> {
        Constant(&[42, 0]).read(reader)?;
        Ok(InitializeConfidentialMintBurn {
            supply_elgamal_pubkey: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            decryptable_supply: DecryptableBalanceCodec.read(reader)?,
        })
    }
}
/// Validated data for `RotateSupplyElgamalPubkey`.
#[derive(Clone, Debug, PartialEq)]
pub struct RotateSupplyElgamalPubkey {
    /// The `new_supply_elgamal_pubkey` wire field.
    pub new_supply_elgamal_pubkey: Address,
    /// The `proof_instruction_offset` wire field.
    pub proof_instruction_offset: i8,
}
/// Bounded codec for [`RotateSupplyElgamalPubkey`].
#[derive(Clone, Copy, Debug)]
pub struct RotateSupplyElgamalPubkeyCodec;
impl Encoder<RotateSupplyElgamalPubkey> for RotateSupplyElgamalPubkeyCodec {
    fn encoded_len(&self, value: &RotateSupplyElgamalPubkey) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.new_supply_elgamal_pubkey)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &RotateSupplyElgamalPubkey,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[42, 1]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.new_supply_elgamal_pubkey, writer)?;
        Number::<i8>::LE.write(&value.proof_instruction_offset, writer)?;
        Ok(())
    }
}
impl Decoder<RotateSupplyElgamalPubkey> for RotateSupplyElgamalPubkeyCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RotateSupplyElgamalPubkey, Error> {
        Constant(&[42, 1]).read(reader)?;
        Ok(RotateSupplyElgamalPubkey {
            new_supply_elgamal_pubkey: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            proof_instruction_offset: Number::<i8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `UpdateConfidentialMintBurnDecryptableSupply`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateConfidentialMintBurnDecryptableSupply {
    /// The `new_decryptable_supply` wire field.
    pub new_decryptable_supply: DecryptableBalance,
}
/// Bounded codec for [`UpdateConfidentialMintBurnDecryptableSupply`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateConfidentialMintBurnDecryptableSupplyCodec;
impl Encoder<UpdateConfidentialMintBurnDecryptableSupply>
    for UpdateConfidentialMintBurnDecryptableSupplyCodec
{
    fn encoded_len(
        &self,
        value: &UpdateConfidentialMintBurnDecryptableSupply,
    ) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(DecryptableBalanceCodec.encoded_len(&value.new_decryptable_supply)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateConfidentialMintBurnDecryptableSupply,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[42, 2]).write(&(), writer)?;
        DecryptableBalanceCodec.write(&value.new_decryptable_supply, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateConfidentialMintBurnDecryptableSupply>
    for UpdateConfidentialMintBurnDecryptableSupplyCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<UpdateConfidentialMintBurnDecryptableSupply, Error> {
        Constant(&[42, 2]).read(reader)?;
        Ok(UpdateConfidentialMintBurnDecryptableSupply {
            new_decryptable_supply: DecryptableBalanceCodec.read(reader)?,
        })
    }
}
/// Validated data for `ConfidentialMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfidentialMint {
    /// The `new_decryptable_supply` wire field.
    pub new_decryptable_supply: DecryptableBalance,
    /// The `mint_amount_auditor_ciphertext_lo` wire field.
    pub mint_amount_auditor_ciphertext_lo: EncryptedBalance,
    /// The `mint_amount_auditor_ciphertext_hi` wire field.
    pub mint_amount_auditor_ciphertext_hi: EncryptedBalance,
    /// The `equality_proof_instruction_offset` wire field.
    pub equality_proof_instruction_offset: i8,
    /// The `ciphertext_validity_proof_instruction_offset` wire field.
    pub ciphertext_validity_proof_instruction_offset: i8,
    /// The `range_proof_instruction_offset` wire field.
    pub range_proof_instruction_offset: i8,
}
/// Bounded codec for [`ConfidentialMint`].
#[derive(Clone, Copy, Debug)]
pub struct ConfidentialMintCodec;
impl Encoder<ConfidentialMint> for ConfidentialMintCodec {
    fn encoded_len(&self, value: &ConfidentialMint) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(DecryptableBalanceCodec.encoded_len(&value.new_decryptable_supply)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                EncryptedBalanceCodec.encoded_len(&value.mint_amount_auditor_ciphertext_lo)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                EncryptedBalanceCodec.encoded_len(&value.mint_amount_auditor_ciphertext_hi)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.equality_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<i8>::LE
                    .encoded_len(&value.ciphertext_validity_proof_instruction_offset)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.range_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ConfidentialMint, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[42, 3]).write(&(), writer)?;
        DecryptableBalanceCodec.write(&value.new_decryptable_supply, writer)?;
        EncryptedBalanceCodec.write(&value.mint_amount_auditor_ciphertext_lo, writer)?;
        EncryptedBalanceCodec.write(&value.mint_amount_auditor_ciphertext_hi, writer)?;
        Number::<i8>::LE.write(&value.equality_proof_instruction_offset, writer)?;
        Number::<i8>::LE.write(&value.ciphertext_validity_proof_instruction_offset, writer)?;
        Number::<i8>::LE.write(&value.range_proof_instruction_offset, writer)?;
        Ok(())
    }
}
impl Decoder<ConfidentialMint> for ConfidentialMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfidentialMint, Error> {
        Constant(&[42, 3]).read(reader)?;
        Ok(ConfidentialMint {
            new_decryptable_supply: DecryptableBalanceCodec.read(reader)?,
            mint_amount_auditor_ciphertext_lo: EncryptedBalanceCodec.read(reader)?,
            mint_amount_auditor_ciphertext_hi: EncryptedBalanceCodec.read(reader)?,
            equality_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            ciphertext_validity_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            range_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ConfidentialBurn`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfidentialBurn {
    /// The `new_decryptable_available_balance` wire field.
    pub new_decryptable_available_balance: DecryptableBalance,
    /// The `burn_amount_auditor_ciphertext_lo` wire field.
    pub burn_amount_auditor_ciphertext_lo: EncryptedBalance,
    /// The `burn_amount_auditor_ciphertext_hi` wire field.
    pub burn_amount_auditor_ciphertext_hi: EncryptedBalance,
    /// The `equality_proof_instruction_offset` wire field.
    pub equality_proof_instruction_offset: i8,
    /// The `ciphertext_validity_proof_instruction_offset` wire field.
    pub ciphertext_validity_proof_instruction_offset: i8,
    /// The `range_proof_instruction_offset` wire field.
    pub range_proof_instruction_offset: i8,
}
/// Bounded codec for [`ConfidentialBurn`].
#[derive(Clone, Copy, Debug)]
pub struct ConfidentialBurnCodec;
impl Encoder<ConfidentialBurn> for ConfidentialBurnCodec {
    fn encoded_len(&self, value: &ConfidentialBurn) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                DecryptableBalanceCodec.encoded_len(&value.new_decryptable_available_balance)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                EncryptedBalanceCodec.encoded_len(&value.burn_amount_auditor_ciphertext_lo)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                EncryptedBalanceCodec.encoded_len(&value.burn_amount_auditor_ciphertext_hi)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.equality_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<i8>::LE
                    .encoded_len(&value.ciphertext_validity_proof_instruction_offset)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.range_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ConfidentialBurn, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[42, 4]).write(&(), writer)?;
        DecryptableBalanceCodec.write(&value.new_decryptable_available_balance, writer)?;
        EncryptedBalanceCodec.write(&value.burn_amount_auditor_ciphertext_lo, writer)?;
        EncryptedBalanceCodec.write(&value.burn_amount_auditor_ciphertext_hi, writer)?;
        Number::<i8>::LE.write(&value.equality_proof_instruction_offset, writer)?;
        Number::<i8>::LE.write(&value.ciphertext_validity_proof_instruction_offset, writer)?;
        Number::<i8>::LE.write(&value.range_proof_instruction_offset, writer)?;
        Ok(())
    }
}
impl Decoder<ConfidentialBurn> for ConfidentialBurnCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfidentialBurn, Error> {
        Constant(&[42, 4]).read(reader)?;
        Ok(ConfidentialBurn {
            new_decryptable_available_balance: DecryptableBalanceCodec.read(reader)?,
            burn_amount_auditor_ciphertext_lo: EncryptedBalanceCodec.read(reader)?,
            burn_amount_auditor_ciphertext_hi: EncryptedBalanceCodec.read(reader)?,
            equality_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            ciphertext_validity_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            range_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ApplyConfidentialPendingBurn`.
#[derive(Clone, Debug, PartialEq)]
pub struct ApplyConfidentialPendingBurn {}
/// Bounded codec for [`ApplyConfidentialPendingBurn`].
#[derive(Clone, Copy, Debug)]
pub struct ApplyConfidentialPendingBurnCodec;
impl Encoder<ApplyConfidentialPendingBurn> for ApplyConfidentialPendingBurnCodec {
    fn encoded_len(&self, value: &ApplyConfidentialPendingBurn) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &ApplyConfidentialPendingBurn,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[42, 5]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ApplyConfidentialPendingBurn> for ApplyConfidentialPendingBurnCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ApplyConfidentialPendingBurn, Error> {
        Constant(&[42, 5]).read(reader)?;
        let _ = reader;
        Ok(ApplyConfidentialPendingBurn {})
    }
}
/// Validated data for `InitializeScaledUiAmountMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeScaledUiAmountMint {
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `multiplier` wire field.
    pub multiplier: f64,
}
/// Bounded codec for [`InitializeScaledUiAmountMint`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeScaledUiAmountMintCodec;
impl Encoder<InitializeScaledUiAmountMint> for InitializeScaledUiAmountMintCodec {
    fn encoded_len(&self, value: &InitializeScaledUiAmountMint) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<f64>::LE.encoded_len(&value.multiplier)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeScaledUiAmountMint,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[43, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Number::<f64>::LE.write(&value.multiplier, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeScaledUiAmountMint> for InitializeScaledUiAmountMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeScaledUiAmountMint, Error> {
        Constant(&[43, 0]).read(reader)?;
        Ok(InitializeScaledUiAmountMint {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            multiplier: Number::<f64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `UpdateMultiplierScaledUiMint`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMultiplierScaledUiMint {
    /// The `multiplier` wire field.
    pub multiplier: f64,
    /// The `effective_timestamp` wire field.
    pub effective_timestamp: i64,
}
/// Bounded codec for [`UpdateMultiplierScaledUiMint`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateMultiplierScaledUiMintCodec;
impl Encoder<UpdateMultiplierScaledUiMint> for UpdateMultiplierScaledUiMintCodec {
    fn encoded_len(&self, value: &UpdateMultiplierScaledUiMint) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<f64>::LE.encoded_len(&value.multiplier)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.effective_timestamp)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateMultiplierScaledUiMint,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[43, 1]).write(&(), writer)?;
        Number::<f64>::LE.write(&value.multiplier, writer)?;
        Number::<i64>::LE.write(&value.effective_timestamp, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateMultiplierScaledUiMint> for UpdateMultiplierScaledUiMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateMultiplierScaledUiMint, Error> {
        Constant(&[43, 1]).read(reader)?;
        Ok(UpdateMultiplierScaledUiMint {
            multiplier: Number::<f64>::LE.read(reader)?,
            effective_timestamp: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `InitializePausableConfig`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePausableConfig {
    /// The `authority` wire field.
    pub authority: Option<Address>,
}
/// Bounded codec for [`InitializePausableConfig`].
#[derive(Clone, Copy, Debug)]
pub struct InitializePausableConfigCodec;
impl Encoder<InitializePausableConfig> for InitializePausableConfigCodec {
    fn encoded_len(&self, value: &InitializePausableConfig) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializePausableConfig,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[44, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.authority, writer)?;
        Ok(())
    }
}
impl Decoder<InitializePausableConfig> for InitializePausableConfigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializePausableConfig, Error> {
        Constant(&[44, 0]).read(reader)?;
        Ok(InitializePausableConfig {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `Pause`.
#[derive(Clone, Debug, PartialEq)]
pub struct Pause {}
/// Bounded codec for [`Pause`].
#[derive(Clone, Copy, Debug)]
pub struct PauseCodec;
impl Encoder<Pause> for PauseCodec {
    fn encoded_len(&self, value: &Pause) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Pause, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[44, 1]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Pause> for PauseCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Pause, Error> {
        Constant(&[44, 1]).read(reader)?;
        let _ = reader;
        Ok(Pause {})
    }
}
/// Validated data for `Resume`.
#[derive(Clone, Debug, PartialEq)]
pub struct Resume {}
/// Bounded codec for [`Resume`].
#[derive(Clone, Copy, Debug)]
pub struct ResumeCodec;
impl Encoder<Resume> for ResumeCodec {
    fn encoded_len(&self, value: &Resume) -> Result<usize, Error> {
        let size = 2usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Resume, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[44, 2]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Resume> for ResumeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Resume, Error> {
        Constant(&[44, 2]).read(reader)?;
        let _ = reader;
        Ok(Resume {})
    }
}
/// Validated data for `InitializeTokenMetadata`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTokenMetadata {
    /// The `name` wire field.
    pub name: String,
    /// The `symbol` wire field.
    pub symbol: String,
    /// The `uri` wire field.
    pub uri: String,
}
/// Bounded codec for [`InitializeTokenMetadata`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeTokenMetadataCodec;
impl Encoder<InitializeTokenMetadata> for InitializeTokenMetadataCodec {
    fn encoded_len(&self, value: &InitializeTokenMetadata) -> Result<usize, Error> {
        let size = 8usize;
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
                .encoded_len(value.symbol.as_str())?,
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
        Ok(size)
    }
    fn write(&self, value: &InitializeTokenMetadata, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[210, 225, 30, 162, 88, 184, 77, 141]).write(&(), writer)?;
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
        .write(value.symbol.as_str(), writer)?;
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
impl Decoder<InitializeTokenMetadata> for InitializeTokenMetadataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeTokenMetadata, Error> {
        Constant(&[210, 225, 30, 162, 88, 184, 77, 141]).read(reader)?;
        Ok(InitializeTokenMetadata {
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
/// Validated data for `UpdateTokenMetadataField`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTokenMetadataField {
    /// The `field` wire field.
    pub field: TokenMetadataField,
    /// The `value` wire field.
    pub value: String,
}
/// Bounded codec for [`UpdateTokenMetadataField`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateTokenMetadataFieldCodec;
impl Encoder<UpdateTokenMetadataField> for UpdateTokenMetadataFieldCodec {
    fn encoded_len(&self, value: &UpdateTokenMetadataField) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(TokenMetadataFieldCodec.encoded_len(&value.field)?)
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
    fn write(
        &self,
        value: &UpdateTokenMetadataField,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[221, 233, 49, 45, 181, 202, 220, 200]).write(&(), writer)?;
        TokenMetadataFieldCodec.write(&value.field, writer)?;
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
impl Decoder<UpdateTokenMetadataField> for UpdateTokenMetadataFieldCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateTokenMetadataField, Error> {
        Constant(&[221, 233, 49, 45, 181, 202, 220, 200]).read(reader)?;
        Ok(UpdateTokenMetadataField {
            field: TokenMetadataFieldCodec.read(reader)?,
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
/// Validated data for `RemoveTokenMetadataKey`.
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveTokenMetadataKey {
    /// The `idempotent` wire field.
    pub idempotent: bool,
    /// The `key` wire field.
    pub key: String,
}
/// Bounded codec for [`RemoveTokenMetadataKey`].
#[derive(Clone, Copy, Debug)]
pub struct RemoveTokenMetadataKeyCodec;
impl Encoder<RemoveTokenMetadataKey> for RemoveTokenMetadataKeyCodec {
    fn encoded_len(&self, value: &RemoveTokenMetadataKey) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.idempotent)?)
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
                .encoded_len(value.key.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &RemoveTokenMetadataKey, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[234, 18, 32, 56, 89, 141, 37, 181]).write(&(), writer)?;
        Bool.write(&value.idempotent, writer)?;
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
        Ok(())
    }
}
impl Decoder<RemoveTokenMetadataKey> for RemoveTokenMetadataKeyCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RemoveTokenMetadataKey, Error> {
        Constant(&[234, 18, 32, 56, 89, 141, 37, 181]).read(reader)?;
        Ok(RemoveTokenMetadataKey {
            idempotent: Bool.read(reader)?,
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
        })
    }
}
/// Validated data for `UpdateTokenMetadataUpdateAuthority`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTokenMetadataUpdateAuthority {
    /// The `new_update_authority` wire field.
    pub new_update_authority: Option<Address>,
}
/// Bounded codec for [`UpdateTokenMetadataUpdateAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateTokenMetadataUpdateAuthorityCodec;
impl Encoder<UpdateTokenMetadataUpdateAuthority> for UpdateTokenMetadataUpdateAuthorityCodec {
    fn encoded_len(&self, value: &UpdateTokenMetadataUpdateAuthority) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.new_update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateTokenMetadataUpdateAuthority,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[215, 228, 166, 228, 84, 100, 86, 123]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.new_update_authority, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateTokenMetadataUpdateAuthority> for UpdateTokenMetadataUpdateAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateTokenMetadataUpdateAuthority, Error> {
        Constant(&[215, 228, 166, 228, 84, 100, 86, 123]).read(reader)?;
        Ok(UpdateTokenMetadataUpdateAuthority {
            new_update_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `EmitTokenMetadata`.
#[derive(Clone, Debug, PartialEq)]
pub struct EmitTokenMetadata {
    /// The `start` wire field.
    pub start: Option<u64>,
    /// The `end` wire field.
    pub end: Option<u64>,
}
/// Bounded codec for [`EmitTokenMetadata`].
#[derive(Clone, Copy, Debug)]
pub struct EmitTokenMetadataCodec;
impl Encoder<EmitTokenMetadata> for EmitTokenMetadataCodec {
    fn encoded_len(&self, value: &EmitTokenMetadata) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.start)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.end)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &EmitTokenMetadata, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[250, 166, 180, 250, 13, 12, 184, 70]).write(&(), writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.start, writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.end, writer)?;
        Ok(())
    }
}
impl Decoder<EmitTokenMetadata> for EmitTokenMetadataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EmitTokenMetadata, Error> {
        Constant(&[250, 166, 180, 250, 13, 12, 184, 70]).read(reader)?;
        Ok(EmitTokenMetadata {
            start: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
            end: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
        })
    }
}
/// Validated data for `InitializeTokenGroup`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTokenGroup {
    /// The `update_authority` wire field.
    pub update_authority: Option<Address>,
    /// The `max_size` wire field.
    pub max_size: u64,
}
/// Bounded codec for [`InitializeTokenGroup`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeTokenGroupCodec;
impl Encoder<InitializeTokenGroup> for InitializeTokenGroupCodec {
    fn encoded_len(&self, value: &InitializeTokenGroup) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.max_size)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeTokenGroup, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[121, 113, 108, 39, 54, 51, 0, 4]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.update_authority, writer)?;
        Number::<u64>::LE.write(&value.max_size, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeTokenGroup> for InitializeTokenGroupCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeTokenGroup, Error> {
        Constant(&[121, 113, 108, 39, 54, 51, 0, 4]).read(reader)?;
        Ok(InitializeTokenGroup {
            update_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
            max_size: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `UpdateTokenGroupMaxSize`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTokenGroupMaxSize {
    /// The `max_size` wire field.
    pub max_size: u64,
}
/// Bounded codec for [`UpdateTokenGroupMaxSize`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateTokenGroupMaxSizeCodec;
impl Encoder<UpdateTokenGroupMaxSize> for UpdateTokenGroupMaxSizeCodec {
    fn encoded_len(&self, value: &UpdateTokenGroupMaxSize) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.max_size)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateTokenGroupMaxSize, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[108, 37, 171, 143, 248, 30, 18, 110]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.max_size, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateTokenGroupMaxSize> for UpdateTokenGroupMaxSizeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateTokenGroupMaxSize, Error> {
        Constant(&[108, 37, 171, 143, 248, 30, 18, 110]).read(reader)?;
        Ok(UpdateTokenGroupMaxSize {
            max_size: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `UpdateTokenGroupUpdateAuthority`.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTokenGroupUpdateAuthority {
    /// The `new_update_authority` wire field.
    pub new_update_authority: Option<Address>,
}
/// Bounded codec for [`UpdateTokenGroupUpdateAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateTokenGroupUpdateAuthorityCodec;
impl Encoder<UpdateTokenGroupUpdateAuthority> for UpdateTokenGroupUpdateAuthorityCodec {
    fn encoded_len(&self, value: &UpdateTokenGroupUpdateAuthority) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Option<Address>| {
                        Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                    },
                    |bytes| {
                        Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes)))
                    },
                )
                .encoded_len(&value.new_update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateTokenGroupUpdateAuthority,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[161, 105, 88, 1, 237, 221, 216, 203]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Option<Address>| {
                Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
            },
            |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
        )
        .write(&value.new_update_authority, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateTokenGroupUpdateAuthority> for UpdateTokenGroupUpdateAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateTokenGroupUpdateAuthority, Error> {
        Constant(&[161, 105, 88, 1, 237, 221, 216, 203]).read(reader)?;
        Ok(UpdateTokenGroupUpdateAuthority {
            new_update_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Option<Address>| {
                    Ok::<_, Error>(value.map_or([0; 32], |address| *address.as_bytes()))
                },
                |bytes| Ok::<_, Error>((bytes != [0; 32]).then_some(Address::from_bytes(bytes))),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeTokenGroupMember`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeTokenGroupMember {}
/// Bounded codec for [`InitializeTokenGroupMember`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeTokenGroupMemberCodec;
impl Encoder<InitializeTokenGroupMember> for InitializeTokenGroupMemberCodec {
    fn encoded_len(&self, value: &InitializeTokenGroupMember) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeTokenGroupMember,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[152, 32, 222, 176, 223, 237, 116, 134]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<InitializeTokenGroupMember> for InitializeTokenGroupMemberCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeTokenGroupMember, Error> {
        Constant(&[152, 32, 222, 176, 223, 237, 116, 134]).read(reader)?;
        let _ = reader;
        Ok(InitializeTokenGroupMember {})
    }
}
/// Validated data for `UnwrapLamports`.
#[derive(Clone, Debug, PartialEq)]
pub struct UnwrapLamports {
    /// The `amount` wire field.
    pub amount: Option<u64>,
}
/// Bounded codec for [`UnwrapLamports`].
#[derive(Clone, Copy, Debug)]
pub struct UnwrapLamportsCodec;
impl Encoder<UnwrapLamports> for UnwrapLamportsCodec {
    fn encoded_len(&self, value: &UnwrapLamports) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UnwrapLamports, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[45]).write(&(), writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<UnwrapLamports> for UnwrapLamportsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UnwrapLamports, Error> {
        Constant(&[45]).read(reader)?;
        Ok(UnwrapLamports {
            amount: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
        })
    }
}
/// Validated data for `InitializePermissionedBurn`.
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePermissionedBurn {
    /// The `authority` wire field.
    pub authority: Address,
}
/// Bounded codec for [`InitializePermissionedBurn`].
#[derive(Clone, Copy, Debug)]
pub struct InitializePermissionedBurnCodec;
impl Encoder<InitializePermissionedBurn> for InitializePermissionedBurnCodec {
    fn encoded_len(&self, value: &InitializePermissionedBurn) -> Result<usize, Error> {
        let size = 2usize;
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
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializePermissionedBurn,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[46, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.authority, writer)?;
        Ok(())
    }
}
impl Decoder<InitializePermissionedBurn> for InitializePermissionedBurnCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializePermissionedBurn, Error> {
        Constant(&[46, 0]).read(reader)?;
        Ok(InitializePermissionedBurn {
            authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `PermissionedBurn`.
#[derive(Clone, Debug, PartialEq)]
pub struct PermissionedBurn {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`PermissionedBurn`].
#[derive(Clone, Copy, Debug)]
pub struct PermissionedBurnCodec;
impl Encoder<PermissionedBurn> for PermissionedBurnCodec {
    fn encoded_len(&self, value: &PermissionedBurn) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PermissionedBurn, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[46, 1]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<PermissionedBurn> for PermissionedBurnCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PermissionedBurn, Error> {
        Constant(&[46, 1]).read(reader)?;
        Ok(PermissionedBurn {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `PermissionedBurnChecked`.
#[derive(Clone, Debug, PartialEq)]
pub struct PermissionedBurnChecked {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
}
/// Bounded codec for [`PermissionedBurnChecked`].
#[derive(Clone, Copy, Debug)]
pub struct PermissionedBurnCheckedCodec;
impl Encoder<PermissionedBurnChecked> for PermissionedBurnCheckedCodec {
    fn encoded_len(&self, value: &PermissionedBurnChecked) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PermissionedBurnChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[46, 2]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Ok(())
    }
}
impl Decoder<PermissionedBurnChecked> for PermissionedBurnCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PermissionedBurnChecked, Error> {
        Constant(&[46, 2]).read(reader)?;
        Ok(PermissionedBurnChecked {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `PermissionedConfidentialBurn`.
#[derive(Clone, Debug, PartialEq)]
pub struct PermissionedConfidentialBurn {
    /// The `new_decryptable_available_balance` wire field.
    pub new_decryptable_available_balance: DecryptableBalance,
    /// The `burn_amount_auditor_ciphertext_lo` wire field.
    pub burn_amount_auditor_ciphertext_lo: EncryptedBalance,
    /// The `burn_amount_auditor_ciphertext_hi` wire field.
    pub burn_amount_auditor_ciphertext_hi: EncryptedBalance,
    /// The `equality_proof_instruction_offset` wire field.
    pub equality_proof_instruction_offset: i8,
    /// The `ciphertext_validity_proof_instruction_offset` wire field.
    pub ciphertext_validity_proof_instruction_offset: i8,
    /// The `range_proof_instruction_offset` wire field.
    pub range_proof_instruction_offset: i8,
}
/// Bounded codec for [`PermissionedConfidentialBurn`].
#[derive(Clone, Copy, Debug)]
pub struct PermissionedConfidentialBurnCodec;
impl Encoder<PermissionedConfidentialBurn> for PermissionedConfidentialBurnCodec {
    fn encoded_len(&self, value: &PermissionedConfidentialBurn) -> Result<usize, Error> {
        let size = 2usize;
        let size = size
            .checked_add(
                DecryptableBalanceCodec.encoded_len(&value.new_decryptable_available_balance)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                EncryptedBalanceCodec.encoded_len(&value.burn_amount_auditor_ciphertext_lo)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                EncryptedBalanceCodec.encoded_len(&value.burn_amount_auditor_ciphertext_hi)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.equality_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<i8>::LE
                    .encoded_len(&value.ciphertext_validity_proof_instruction_offset)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i8>::LE.encoded_len(&value.range_proof_instruction_offset)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &PermissionedConfidentialBurn,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[46, 3]).write(&(), writer)?;
        DecryptableBalanceCodec.write(&value.new_decryptable_available_balance, writer)?;
        EncryptedBalanceCodec.write(&value.burn_amount_auditor_ciphertext_lo, writer)?;
        EncryptedBalanceCodec.write(&value.burn_amount_auditor_ciphertext_hi, writer)?;
        Number::<i8>::LE.write(&value.equality_proof_instruction_offset, writer)?;
        Number::<i8>::LE.write(&value.ciphertext_validity_proof_instruction_offset, writer)?;
        Number::<i8>::LE.write(&value.range_proof_instruction_offset, writer)?;
        Ok(())
    }
}
impl Decoder<PermissionedConfidentialBurn> for PermissionedConfidentialBurnCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PermissionedConfidentialBurn, Error> {
        Constant(&[46, 3]).read(reader)?;
        Ok(PermissionedConfidentialBurn {
            new_decryptable_available_balance: DecryptableBalanceCodec.read(reader)?,
            burn_amount_auditor_ciphertext_lo: EncryptedBalanceCodec.read(reader)?,
            burn_amount_auditor_ciphertext_hi: EncryptedBalanceCodec.read(reader)?,
            equality_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            ciphertext_validity_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
            range_proof_instruction_offset: Number::<i8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `Batch`.
#[derive(Clone, Debug, PartialEq)]
pub struct Batch {
    /// The `data` wire field.
    pub data: Vec<BatchDataItem>,
}
/// Bounded codec for [`Batch`].
#[derive(Clone, Copy, Debug)]
pub struct BatchCodec;
impl Encoder<Batch> for BatchCodec {
    fn encoded_len(&self, value: &Batch) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: BatchDataItemCodec,
                    count: Count::Remainder,
                })
                .encoded_len(&value.data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Batch, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[255]).write(&(), writer)?;
        OwnedSequence(Sequence {
            item: BatchDataItemCodec,
            count: Count::Remainder,
        })
        .write(&value.data, writer)?;
        Ok(())
    }
}
impl Decoder<Batch> for BatchCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Batch, Error> {
        Constant(&[255]).read(reader)?;
        Ok(Batch {
            data: OwnedSequence(Sequence {
                item: BatchDataItemCodec,
                count: Count::Remainder,
            })
            .read(reader)?,
        })
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
use solana_kit_core::codecs::Affix;
use solana_kit_core::codecs::Bool;
use solana_kit_core::codecs::Bytes;
use solana_kit_core::codecs::Counted;
use solana_kit_core::codecs::FixedBytes;
use solana_kit_core::codecs::Mapped;
use solana_kit_core::codecs::OptionCodec;
use solana_kit_core::codecs::Tuple;
use solana_kit_core::codecs::Utf8;
use solana_kit_core::codecs::{ByteSizePrefix, FrameConsumption};
use solana_kit_core::codecs::{Count, OwnedSequence, Sequence};
