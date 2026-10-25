// Generated from the exact pinned Codama input; see upstream notices.
use alloc::vec::Vec;
use solana_kit_core::codecs::Constant;
use solana_kit_core::codecs::Number;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Reader, Writer},
};
const FIXED_COUNT_8: usize = 8;
/// Validated data for `Epoch`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Epoch {
    /// The `value` wire field.
    pub value: u64,
}
/// Bounded codec for [`Epoch`].
#[derive(Clone, Copy, Debug)]
pub struct EpochCodec;
impl Encoder<Epoch> for EpochCodec {
    fn encoded_len(&self, value: &Epoch) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.value)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Epoch, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.value, writer)?;
        Ok(())
    }
}
impl Decoder<Epoch> for EpochCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Epoch, Error> {
        Ok(Epoch {
            value: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `UnixTimestamp`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnixTimestamp {
    /// The `value` wire field.
    pub value: i64,
}
/// Bounded codec for [`UnixTimestamp`].
#[derive(Clone, Copy, Debug)]
pub struct UnixTimestampCodec;
impl Encoder<UnixTimestamp> for UnixTimestampCodec {
    fn encoded_len(&self, value: &UnixTimestamp) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.value)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UnixTimestamp, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<i64>::LE.write(&value.value, writer)?;
        Ok(())
    }
}
impl Decoder<UnixTimestamp> for UnixTimestampCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UnixTimestamp, Error> {
        Ok(UnixTimestamp {
            value: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `StakeFlags`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StakeFlags {
    /// The `bits` wire field.
    pub bits: u8,
}
/// Bounded codec for [`StakeFlags`].
#[derive(Clone, Copy, Debug)]
pub struct StakeFlagsCodec;
impl Encoder<StakeFlags> for StakeFlagsCodec {
    fn encoded_len(&self, value: &StakeFlags) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bits)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &StakeFlags, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.bits, writer)?;
        Ok(())
    }
}
impl Decoder<StakeFlags> for StakeFlagsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<StakeFlags, Error> {
        Ok(StakeFlags {
            bits: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StakeState {
    /// The `Uninitialized` variant.
    Uninitialized,
    /// The `Initialized` variant.
    Initialized(Meta),
    /// The `Stake` variant.
    Stake(StakeStateStakePayload),
    /// The `RewardsPool` variant.
    RewardsPool,
}
/// Bounded codec for [`StakeState`].
#[derive(Clone, Copy, Debug)]
pub struct StakeStateCodec;
const STAKESTATE_TAG_0: u32 = 0;
const STAKESTATE_TAG_1: u32 = 1;
const STAKESTATE_TAG_2: u32 = 2;
const STAKESTATE_TAG_3: u32 = 3;
impl Encoder<StakeState> for StakeStateCodec {
    fn encoded_len(&self, value: &StakeState) -> Result<usize, Error> {
        let size = match value {
            StakeState::Uninitialized => 0usize,
            StakeState::Initialized(value) => MetaCodec.encoded_len(value)?,
            StakeState::Stake(value) => StakeStateStakePayloadCodec.encoded_len(value)?,
            StakeState::RewardsPool => 0usize,
        };

        size.checked_add(core::mem::size_of::<u32>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &StakeState, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            StakeState::Uninitialized => {
                Number::<u32>::LE.write(&STAKESTATE_TAG_0, writer)?;
                Ok(())
            }
            StakeState::Initialized(value) => {
                Number::<u32>::LE.write(&STAKESTATE_TAG_1, writer)?;
                MetaCodec.write(value, writer)
            }
            StakeState::Stake(value) => {
                Number::<u32>::LE.write(&STAKESTATE_TAG_2, writer)?;
                StakeStateStakePayloadCodec.write(value, writer)
            }
            StakeState::RewardsPool => {
                Number::<u32>::LE.write(&STAKESTATE_TAG_3, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<StakeState> for StakeStateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<StakeState, Error> {
        reader.transaction(|reader| match Number::<u32>::LE.read(reader)? {
            STAKESTATE_TAG_0 => Ok(StakeState::Uninitialized),
            STAKESTATE_TAG_1 => Ok(StakeState::Initialized(MetaCodec.read(reader)?)),
            STAKESTATE_TAG_2 => Ok(StakeState::Stake(StakeStateStakePayloadCodec.read(reader)?)),
            STAKESTATE_TAG_3 => Ok(StakeState::RewardsPool),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StakeStateV2 {
    /// The `Uninitialized` variant.
    Uninitialized,
    /// The `Initialized` variant.
    Initialized(Meta),
    /// The `Stake` variant.
    Stake(StakeStateV2StakePayload),
    /// The `RewardsPool` variant.
    RewardsPool,
}
/// Bounded codec for [`StakeStateV2`].
#[derive(Clone, Copy, Debug)]
pub struct StakeStateV2Codec;
const STAKESTATEV2_TAG_0: u32 = 0;
const STAKESTATEV2_TAG_1: u32 = 1;
const STAKESTATEV2_TAG_2: u32 = 2;
const STAKESTATEV2_TAG_3: u32 = 3;
impl Encoder<StakeStateV2> for StakeStateV2Codec {
    fn encoded_len(&self, value: &StakeStateV2) -> Result<usize, Error> {
        let size = match value {
            StakeStateV2::Uninitialized => 0usize,
            StakeStateV2::Initialized(value) => MetaCodec.encoded_len(value)?,
            StakeStateV2::Stake(value) => StakeStateV2StakePayloadCodec.encoded_len(value)?,
            StakeStateV2::RewardsPool => 0usize,
        };

        size.checked_add(core::mem::size_of::<u32>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &StakeStateV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            StakeStateV2::Uninitialized => {
                Number::<u32>::LE.write(&STAKESTATEV2_TAG_0, writer)?;
                Ok(())
            }
            StakeStateV2::Initialized(value) => {
                Number::<u32>::LE.write(&STAKESTATEV2_TAG_1, writer)?;
                MetaCodec.write(value, writer)
            }
            StakeStateV2::Stake(value) => {
                Number::<u32>::LE.write(&STAKESTATEV2_TAG_2, writer)?;
                StakeStateV2StakePayloadCodec.write(value, writer)
            }
            StakeStateV2::RewardsPool => {
                Number::<u32>::LE.write(&STAKESTATEV2_TAG_3, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<StakeStateV2> for StakeStateV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<StakeStateV2, Error> {
        reader.transaction(|reader| match Number::<u32>::LE.read(reader)? {
            STAKESTATEV2_TAG_0 => Ok(StakeStateV2::Uninitialized),
            STAKESTATEV2_TAG_1 => Ok(StakeStateV2::Initialized(MetaCodec.read(reader)?)),
            STAKESTATEV2_TAG_2 => Ok(StakeStateV2::Stake(
                StakeStateV2StakePayloadCodec.read(reader)?,
            )),
            STAKESTATEV2_TAG_3 => Ok(StakeStateV2::RewardsPool),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StakeAuthorize {
    /// The `Staker` variant.
    Staker,
    /// The `Withdrawer` variant.
    Withdrawer,
}
/// Bounded codec for [`StakeAuthorize`].
#[derive(Clone, Copy, Debug)]
pub struct StakeAuthorizeCodec;
const STAKEAUTHORIZE_TAG_0: u32 = 0;
const STAKEAUTHORIZE_TAG_1: u32 = 1;
impl Encoder<StakeAuthorize> for StakeAuthorizeCodec {
    fn encoded_len(&self, value: &StakeAuthorize) -> Result<usize, Error> {
        let size = match value {
            StakeAuthorize::Staker => 0usize,
            StakeAuthorize::Withdrawer => 0usize,
        };

        size.checked_add(core::mem::size_of::<u32>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &StakeAuthorize, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            StakeAuthorize::Staker => {
                Number::<u32>::LE.write(&STAKEAUTHORIZE_TAG_0, writer)?;
                Ok(())
            }
            StakeAuthorize::Withdrawer => {
                Number::<u32>::LE.write(&STAKEAUTHORIZE_TAG_1, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<StakeAuthorize> for StakeAuthorizeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<StakeAuthorize, Error> {
        reader.transaction(|reader| match Number::<u32>::LE.read(reader)? {
            STAKEAUTHORIZE_TAG_0 => Ok(StakeAuthorize::Staker),
            STAKEAUTHORIZE_TAG_1 => Ok(StakeAuthorize::Withdrawer),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `Lockup`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lockup {
    /// The `unix_timestamp` wire field.
    pub unix_timestamp: UnixTimestamp,
    /// The `epoch` wire field.
    pub epoch: Epoch,
    /// The `custodian` wire field.
    pub custodian: Address,
}
/// Bounded codec for [`Lockup`].
#[derive(Clone, Copy, Debug)]
pub struct LockupCodec;
impl Encoder<Lockup> for LockupCodec {
    fn encoded_len(&self, value: &Lockup) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(UnixTimestampCodec.encoded_len(&value.unix_timestamp)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(EpochCodec.encoded_len(&value.epoch)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.custodian)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Lockup, writer: &mut Writer<'_>) -> Result<(), Error> {
        UnixTimestampCodec.write(&value.unix_timestamp, writer)?;
        EpochCodec.write(&value.epoch, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.custodian, writer)?;
        Ok(())
    }
}
impl Decoder<Lockup> for LockupCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Lockup, Error> {
        Ok(Lockup {
            unix_timestamp: UnixTimestampCodec.read(reader)?,
            epoch: EpochCodec.read(reader)?,
            custodian: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `Authorized`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authorized {
    /// The `staker` wire field.
    pub staker: Address,
    /// The `withdrawer` wire field.
    pub withdrawer: Address,
}
/// Bounded codec for [`Authorized`].
#[derive(Clone, Copy, Debug)]
pub struct AuthorizedCodec;
impl Encoder<Authorized> for AuthorizedCodec {
    fn encoded_len(&self, value: &Authorized) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.staker)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.withdrawer)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Authorized, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.staker, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.withdrawer, writer)?;
        Ok(())
    }
}
impl Decoder<Authorized> for AuthorizedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Authorized, Error> {
        Ok(Authorized {
            staker: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            withdrawer: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `Meta`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Meta {
    /// The `rent_exempt_reserve` wire field.
    pub rent_exempt_reserve: u64,
    /// The `authorized` wire field.
    pub authorized: Authorized,
    /// The `lockup` wire field.
    pub lockup: Lockup,
}
/// Bounded codec for [`Meta`].
#[derive(Clone, Copy, Debug)]
pub struct MetaCodec;
impl Encoder<Meta> for MetaCodec {
    fn encoded_len(&self, value: &Meta) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.rent_exempt_reserve)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(AuthorizedCodec.encoded_len(&value.authorized)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(LockupCodec.encoded_len(&value.lockup)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Meta, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.rent_exempt_reserve, writer)?;
        AuthorizedCodec.write(&value.authorized, writer)?;
        LockupCodec.write(&value.lockup, writer)?;
        Ok(())
    }
}
impl Decoder<Meta> for MetaCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Meta, Error> {
        Ok(Meta {
            rent_exempt_reserve: Number::<u64>::LE.read(reader)?,
            authorized: AuthorizedCodec.read(reader)?,
            lockup: LockupCodec.read(reader)?,
        })
    }
}
/// Validated data for `Delegation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Delegation {
    /// The `voter_pubkey` wire field.
    pub voter_pubkey: Address,
    /// The `stake` wire field.
    pub stake: u64,
    /// The `activation_epoch` wire field.
    pub activation_epoch: Epoch,
    /// The `deactivation_epoch` wire field.
    pub deactivation_epoch: Epoch,
    /// The `reserved` wire field.
    pub reserved: Vec<u8>,
}
/// Bounded codec for [`Delegation`].
#[derive(Clone, Copy, Debug)]
pub struct DelegationCodec;
impl Encoder<Delegation> for DelegationCodec {
    fn encoded_len(&self, value: &Delegation) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.voter_pubkey)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.stake)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(EpochCodec.encoded_len(&value.activation_epoch)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(EpochCodec.encoded_len(&value.deactivation_epoch)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Number::<u8>::LE,
                    count: Count::Fixed(FIXED_COUNT_8),
                })
                .encoded_len(&value.reserved)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Delegation, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.voter_pubkey, writer)?;
        Number::<u64>::LE.write(&value.stake, writer)?;
        EpochCodec.write(&value.activation_epoch, writer)?;
        EpochCodec.write(&value.deactivation_epoch, writer)?;
        OwnedSequence(Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(FIXED_COUNT_8),
        })
        .write(&value.reserved, writer)?;
        Ok(())
    }
}
impl Decoder<Delegation> for DelegationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Delegation, Error> {
        Ok(Delegation {
            voter_pubkey: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            stake: Number::<u64>::LE.read(reader)?,
            activation_epoch: EpochCodec.read(reader)?,
            deactivation_epoch: EpochCodec.read(reader)?,
            reserved: OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::Fixed(FIXED_COUNT_8),
            })
            .read(reader)?,
        })
    }
}
/// Validated data for `Stake`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stake {
    /// The `delegation` wire field.
    pub delegation: Delegation,
    /// The `credits_observed` wire field.
    pub credits_observed: u64,
}
/// Bounded codec for [`Stake`].
#[derive(Clone, Copy, Debug)]
pub struct StakeCodec;
impl Encoder<Stake> for StakeCodec {
    fn encoded_len(&self, value: &Stake) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(DelegationCodec.encoded_len(&value.delegation)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.credits_observed)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Stake, writer: &mut Writer<'_>) -> Result<(), Error> {
        DelegationCodec.write(&value.delegation, writer)?;
        Number::<u64>::LE.write(&value.credits_observed, writer)?;
        Ok(())
    }
}
impl Decoder<Stake> for StakeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Stake, Error> {
        Ok(Stake {
            delegation: DelegationCodec.read(reader)?,
            credits_observed: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `StakeStateStakePayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StakeStateStakePayload {
    /// The `item0` wire field.
    pub item0: Meta,
    /// The `item1` wire field.
    pub item1: Stake,
}
/// Bounded codec for [`StakeStateStakePayload`].
#[derive(Clone, Copy, Debug)]
pub struct StakeStateStakePayloadCodec;
impl Encoder<StakeStateStakePayload> for StakeStateStakePayloadCodec {
    fn encoded_len(&self, value: &StakeStateStakePayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(MetaCodec.encoded_len(&value.item0)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(StakeCodec.encoded_len(&value.item1)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &StakeStateStakePayload, writer: &mut Writer<'_>) -> Result<(), Error> {
        MetaCodec.write(&value.item0, writer)?;
        StakeCodec.write(&value.item1, writer)?;
        Ok(())
    }
}
impl Decoder<StakeStateStakePayload> for StakeStateStakePayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<StakeStateStakePayload, Error> {
        Ok(StakeStateStakePayload {
            item0: MetaCodec.read(reader)?,
            item1: StakeCodec.read(reader)?,
        })
    }
}
/// Validated data for `StakeStateV2StakePayload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StakeStateV2StakePayload {
    /// The `item0` wire field.
    pub item0: Meta,
    /// The `item1` wire field.
    pub item1: Stake,
    /// The `item2` wire field.
    pub item2: StakeFlags,
}
/// Bounded codec for [`StakeStateV2StakePayload`].
#[derive(Clone, Copy, Debug)]
pub struct StakeStateV2StakePayloadCodec;
impl Encoder<StakeStateV2StakePayload> for StakeStateV2StakePayloadCodec {
    fn encoded_len(&self, value: &StakeStateV2StakePayload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(MetaCodec.encoded_len(&value.item0)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(StakeCodec.encoded_len(&value.item1)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(StakeFlagsCodec.encoded_len(&value.item2)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &StakeStateV2StakePayload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        MetaCodec.write(&value.item0, writer)?;
        StakeCodec.write(&value.item1, writer)?;
        StakeFlagsCodec.write(&value.item2, writer)?;
        Ok(())
    }
}
impl Decoder<StakeStateV2StakePayload> for StakeStateV2StakePayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<StakeStateV2StakePayload, Error> {
        Ok(StakeStateV2StakePayload {
            item0: MetaCodec.read(reader)?,
            item1: StakeCodec.read(reader)?,
            item2: StakeFlagsCodec.read(reader)?,
        })
    }
}
/// Every pinned instruction data variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StakeInstructionData {
    /// Data for `Initialize`.
    Initialize(Initialize),
    /// Data for `Authorize`.
    Authorize(Authorize),
    /// Data for `DelegateStake`.
    DelegateStake(DelegateStake),
    /// Data for `Split`.
    Split(Split),
    /// Data for `Withdraw`.
    Withdraw(Withdraw),
    /// Data for `Deactivate`.
    Deactivate(Deactivate),
    /// Data for `SetLockup`.
    SetLockup(SetLockup),
    /// Data for `Merge`.
    Merge(Merge),
    /// Data for `AuthorizeWithSeed`.
    AuthorizeWithSeed(AuthorizeWithSeed),
    /// Data for `InitializeChecked`.
    InitializeChecked(InitializeChecked),
    /// Data for `AuthorizeChecked`.
    AuthorizeChecked(AuthorizeChecked),
    /// Data for `AuthorizeCheckedWithSeed`.
    AuthorizeCheckedWithSeed(AuthorizeCheckedWithSeed),
    /// Data for `SetLockupChecked`.
    SetLockupChecked(SetLockupChecked),
    /// Data for `GetMinimumDelegation`.
    GetMinimumDelegation(GetMinimumDelegation),
    /// Data for `DeactivateDelinquent`.
    DeactivateDelinquent(DeactivateDelinquent),
    /// Data for `MoveStake`.
    MoveStake(MoveStake),
    /// Data for `MoveLamports`.
    MoveLamports(MoveLamports),
}
/// Prefix-safe multi-byte instruction dispatch.
#[derive(Clone, Copy, Debug)]
pub struct StakeInstructionDataCodec;
impl Encoder<StakeInstructionData> for StakeInstructionDataCodec {
    fn encoded_len(&self, value: &StakeInstructionData) -> Result<usize, Error> {
        match value {
            StakeInstructionData::Initialize(value) => InitializeCodec.encoded_len(value),
            StakeInstructionData::Authorize(value) => AuthorizeCodec.encoded_len(value),
            StakeInstructionData::DelegateStake(value) => DelegateStakeCodec.encoded_len(value),
            StakeInstructionData::Split(value) => SplitCodec.encoded_len(value),
            StakeInstructionData::Withdraw(value) => WithdrawCodec.encoded_len(value),
            StakeInstructionData::Deactivate(value) => DeactivateCodec.encoded_len(value),
            StakeInstructionData::SetLockup(value) => SetLockupCodec.encoded_len(value),
            StakeInstructionData::Merge(value) => MergeCodec.encoded_len(value),
            StakeInstructionData::AuthorizeWithSeed(value) => {
                AuthorizeWithSeedCodec.encoded_len(value)
            }
            StakeInstructionData::InitializeChecked(value) => {
                InitializeCheckedCodec.encoded_len(value)
            }
            StakeInstructionData::AuthorizeChecked(value) => {
                AuthorizeCheckedCodec.encoded_len(value)
            }
            StakeInstructionData::AuthorizeCheckedWithSeed(value) => {
                AuthorizeCheckedWithSeedCodec.encoded_len(value)
            }
            StakeInstructionData::SetLockupChecked(value) => {
                SetLockupCheckedCodec.encoded_len(value)
            }
            StakeInstructionData::GetMinimumDelegation(value) => {
                GetMinimumDelegationCodec.encoded_len(value)
            }
            StakeInstructionData::DeactivateDelinquent(value) => {
                DeactivateDelinquentCodec.encoded_len(value)
            }
            StakeInstructionData::MoveStake(value) => MoveStakeCodec.encoded_len(value),
            StakeInstructionData::MoveLamports(value) => MoveLamportsCodec.encoded_len(value),
        }
    }
    fn write(&self, value: &StakeInstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            StakeInstructionData::Initialize(value) => InitializeCodec.write(value, writer),
            StakeInstructionData::Authorize(value) => AuthorizeCodec.write(value, writer),
            StakeInstructionData::DelegateStake(value) => DelegateStakeCodec.write(value, writer),
            StakeInstructionData::Split(value) => SplitCodec.write(value, writer),
            StakeInstructionData::Withdraw(value) => WithdrawCodec.write(value, writer),
            StakeInstructionData::Deactivate(value) => DeactivateCodec.write(value, writer),
            StakeInstructionData::SetLockup(value) => SetLockupCodec.write(value, writer),
            StakeInstructionData::Merge(value) => MergeCodec.write(value, writer),
            StakeInstructionData::AuthorizeWithSeed(value) => {
                AuthorizeWithSeedCodec.write(value, writer)
            }
            StakeInstructionData::InitializeChecked(value) => {
                InitializeCheckedCodec.write(value, writer)
            }
            StakeInstructionData::AuthorizeChecked(value) => {
                AuthorizeCheckedCodec.write(value, writer)
            }
            StakeInstructionData::AuthorizeCheckedWithSeed(value) => {
                AuthorizeCheckedWithSeedCodec.write(value, writer)
            }
            StakeInstructionData::SetLockupChecked(value) => {
                SetLockupCheckedCodec.write(value, writer)
            }
            StakeInstructionData::GetMinimumDelegation(value) => {
                GetMinimumDelegationCodec.write(value, writer)
            }
            StakeInstructionData::DeactivateDelinquent(value) => {
                DeactivateDelinquentCodec.write(value, writer)
            }
            StakeInstructionData::MoveStake(value) => MoveStakeCodec.write(value, writer),
            StakeInstructionData::MoveLamports(value) => MoveLamportsCodec.write(value, writer),
        }
    }
}
impl Decoder<StakeInstructionData> for StakeInstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<StakeInstructionData, Error> {
        reader.transaction(|reader| match reader.remaining() {
            [0, 0, 0, 0, ..] => Ok(StakeInstructionData::Initialize(
                InitializeCodec.read(reader)?,
            )),
            [1, 0, 0, 0, ..] => Ok(StakeInstructionData::Authorize(
                AuthorizeCodec.read(reader)?,
            )),
            [2, 0, 0, 0, ..] => Ok(StakeInstructionData::DelegateStake(
                DelegateStakeCodec.read(reader)?,
            )),
            [3, 0, 0, 0, ..] => Ok(StakeInstructionData::Split(SplitCodec.read(reader)?)),
            [4, 0, 0, 0, ..] => Ok(StakeInstructionData::Withdraw(WithdrawCodec.read(reader)?)),
            [5, 0, 0, 0, ..] => Ok(StakeInstructionData::Deactivate(
                DeactivateCodec.read(reader)?,
            )),
            [6, 0, 0, 0, ..] => Ok(StakeInstructionData::SetLockup(
                SetLockupCodec.read(reader)?,
            )),
            [7, 0, 0, 0, ..] => Ok(StakeInstructionData::Merge(MergeCodec.read(reader)?)),
            [8, 0, 0, 0, ..] => Ok(StakeInstructionData::AuthorizeWithSeed(
                AuthorizeWithSeedCodec.read(reader)?,
            )),
            [9, 0, 0, 0, ..] => Ok(StakeInstructionData::InitializeChecked(
                InitializeCheckedCodec.read(reader)?,
            )),
            [10, 0, 0, 0, ..] => Ok(StakeInstructionData::AuthorizeChecked(
                AuthorizeCheckedCodec.read(reader)?,
            )),
            [11, 0, 0, 0, ..] => Ok(StakeInstructionData::AuthorizeCheckedWithSeed(
                AuthorizeCheckedWithSeedCodec.read(reader)?,
            )),
            [12, 0, 0, 0, ..] => Ok(StakeInstructionData::SetLockupChecked(
                SetLockupCheckedCodec.read(reader)?,
            )),
            [13, 0, 0, 0, ..] => Ok(StakeInstructionData::GetMinimumDelegation(
                GetMinimumDelegationCodec.read(reader)?,
            )),
            [14, 0, 0, 0, ..] => Ok(StakeInstructionData::DeactivateDelinquent(
                DeactivateDelinquentCodec.read(reader)?,
            )),
            [16, 0, 0, 0, ..] => Ok(StakeInstructionData::MoveStake(
                MoveStakeCodec.read(reader)?,
            )),
            [17, 0, 0, 0, ..] => Ok(StakeInstructionData::MoveLamports(
                MoveLamportsCodec.read(reader)?,
            )),
            [] => Err(Error::Truncated),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `StakeStateAccount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StakeStateAccount {
    /// The `state` wire field.
    pub state: StakeStateV2,
}
/// Bounded codec for [`StakeStateAccount`].
#[derive(Clone, Copy, Debug)]
pub struct StakeStateAccountCodec;
impl Encoder<StakeStateAccount> for StakeStateAccountCodec {
    fn encoded_len(&self, value: &StakeStateAccount) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(StakeStateV2Codec.encoded_len(&value.state)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &StakeStateAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        StakeStateV2Codec.write(&value.state, writer)?;
        Ok(())
    }
}
impl Decoder<StakeStateAccount> for StakeStateAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<StakeStateAccount, Error> {
        Ok(StakeStateAccount {
            state: StakeStateV2Codec.read(reader)?,
        })
    }
}
/// Validated data for `Initialize`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Initialize {
    /// The `arg0` wire field.
    pub arg0: Authorized,
    /// The `arg1` wire field.
    pub arg1: Lockup,
}
/// Bounded codec for [`Initialize`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeCodec;
impl Encoder<Initialize> for InitializeCodec {
    fn encoded_len(&self, value: &Initialize) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(AuthorizedCodec.encoded_len(&value.arg0)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(LockupCodec.encoded_len(&value.arg1)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Initialize, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[0, 0, 0, 0]).write(&(), writer)?;
        AuthorizedCodec.write(&value.arg0, writer)?;
        LockupCodec.write(&value.arg1, writer)?;
        Ok(())
    }
}
impl Decoder<Initialize> for InitializeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Initialize, Error> {
        Constant(&[0, 0, 0, 0]).read(reader)?;
        Ok(Initialize {
            arg0: AuthorizedCodec.read(reader)?,
            arg1: LockupCodec.read(reader)?,
        })
    }
}
/// Validated data for `Authorize`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authorize {
    /// The `arg0` wire field.
    pub arg0: Address,
    /// The `arg1` wire field.
    pub arg1: StakeAuthorize,
}
/// Bounded codec for [`Authorize`].
#[derive(Clone, Copy, Debug)]
pub struct AuthorizeCodec;
impl Encoder<Authorize> for AuthorizeCodec {
    fn encoded_len(&self, value: &Authorize) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.arg0)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(StakeAuthorizeCodec.encoded_len(&value.arg1)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Authorize, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[1, 0, 0, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.arg0, writer)?;
        StakeAuthorizeCodec.write(&value.arg1, writer)?;
        Ok(())
    }
}
impl Decoder<Authorize> for AuthorizeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Authorize, Error> {
        Constant(&[1, 0, 0, 0]).read(reader)?;
        Ok(Authorize {
            arg0: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            arg1: StakeAuthorizeCodec.read(reader)?,
        })
    }
}
/// Validated data for `DelegateStake`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateStake {}
/// Bounded codec for [`DelegateStake`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateStakeCodec;
impl Encoder<DelegateStake> for DelegateStakeCodec {
    fn encoded_len(&self, value: &DelegateStake) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &DelegateStake, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[2, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DelegateStake> for DelegateStakeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateStake, Error> {
        Constant(&[2, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(DelegateStake {})
    }
}
/// Validated data for `Split`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Split {
    /// The `args` wire field.
    pub args: u64,
}
/// Bounded codec for [`Split`].
#[derive(Clone, Copy, Debug)]
pub struct SplitCodec;
impl Encoder<Split> for SplitCodec {
    fn encoded_len(&self, value: &Split) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Split, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[3, 0, 0, 0]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.args, writer)?;
        Ok(())
    }
}
impl Decoder<Split> for SplitCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Split, Error> {
        Constant(&[3, 0, 0, 0]).read(reader)?;
        Ok(Split {
            args: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `Withdraw`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Withdraw {
    /// The `args` wire field.
    pub args: u64,
}
/// Bounded codec for [`Withdraw`].
#[derive(Clone, Copy, Debug)]
pub struct WithdrawCodec;
impl Encoder<Withdraw> for WithdrawCodec {
    fn encoded_len(&self, value: &Withdraw) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Withdraw, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[4, 0, 0, 0]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.args, writer)?;
        Ok(())
    }
}
impl Decoder<Withdraw> for WithdrawCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Withdraw, Error> {
        Constant(&[4, 0, 0, 0]).read(reader)?;
        Ok(Withdraw {
            args: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `Deactivate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deactivate {}
/// Bounded codec for [`Deactivate`].
#[derive(Clone, Copy, Debug)]
pub struct DeactivateCodec;
impl Encoder<Deactivate> for DeactivateCodec {
    fn encoded_len(&self, value: &Deactivate) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Deactivate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[5, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Deactivate> for DeactivateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Deactivate, Error> {
        Constant(&[5, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(Deactivate {})
    }
}
/// Validated data for `SetLockup`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetLockup {
    /// The `unix_timestamp` wire field.
    pub unix_timestamp: Option<UnixTimestamp>,
    /// The `epoch` wire field.
    pub epoch: Option<Epoch>,
    /// The `custodian` wire field.
    pub custodian: Option<Address>,
}
/// Bounded codec for [`SetLockup`].
#[derive(Clone, Copy, Debug)]
pub struct SetLockupCodec;
impl Encoder<SetLockup> for SetLockupCodec {
    fn encoded_len(&self, value: &SetLockup) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(OptionCodec::u8(UnixTimestampCodec).encoded_len(&value.unix_timestamp)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(EpochCodec).encoded_len(&value.epoch)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.custodian)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetLockup, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[6, 0, 0, 0]).write(&(), writer)?;
        OptionCodec::u8(UnixTimestampCodec).write(&value.unix_timestamp, writer)?;
        OptionCodec::u8(EpochCodec).write(&value.epoch, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.custodian, writer)?;
        Ok(())
    }
}
impl Decoder<SetLockup> for SetLockupCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetLockup, Error> {
        Constant(&[6, 0, 0, 0]).read(reader)?;
        Ok(SetLockup {
            unix_timestamp: OptionCodec::u8(UnixTimestampCodec).read(reader)?,
            epoch: OptionCodec::u8(EpochCodec).read(reader)?,
            custodian: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
        })
    }
}
/// Validated data for `Merge`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Merge {}
/// Bounded codec for [`Merge`].
#[derive(Clone, Copy, Debug)]
pub struct MergeCodec;
impl Encoder<Merge> for MergeCodec {
    fn encoded_len(&self, value: &Merge) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Merge, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[7, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Merge> for MergeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Merge, Error> {
        Constant(&[7, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(Merge {})
    }
}
/// Validated data for `AuthorizeWithSeed`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizeWithSeed {
    /// The `new_authorized_pubkey` wire field.
    pub new_authorized_pubkey: Address,
    /// The `stake_authorize` wire field.
    pub stake_authorize: StakeAuthorize,
    /// The `authority_seed` wire field.
    pub authority_seed: String,
    /// The `authority_owner` wire field.
    pub authority_owner: Address,
}
/// Bounded codec for [`AuthorizeWithSeed`].
#[derive(Clone, Copy, Debug)]
pub struct AuthorizeWithSeedCodec;
impl Encoder<AuthorizeWithSeed> for AuthorizeWithSeedCodec {
    fn encoded_len(&self, value: &AuthorizeWithSeed) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.new_authorized_pubkey)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(StakeAuthorizeCodec.encoded_len(&value.stake_authorize)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Number::<u64>::LE,
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.authority_seed.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.authority_owner)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AuthorizeWithSeed, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[8, 0, 0, 0]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.new_authorized_pubkey, writer)?;
        StakeAuthorizeCodec.write(&value.stake_authorize, writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Number::<u64>::LE,
            consumption: FrameConsumption::Exact,
        }
        .write(value.authority_seed.as_str(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.authority_owner, writer)?;
        Ok(())
    }
}
impl Decoder<AuthorizeWithSeed> for AuthorizeWithSeedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AuthorizeWithSeed, Error> {
        Constant(&[8, 0, 0, 0]).read(reader)?;
        Ok(AuthorizeWithSeed {
            new_authorized_pubkey: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            stake_authorize: StakeAuthorizeCodec.read(reader)?,
            authority_seed: ByteSizePrefix {
                item: Utf8,
                prefix: Number::<u64>::LE,
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            authority_owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeChecked`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeChecked {}
/// Bounded codec for [`InitializeChecked`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeCheckedCodec;
impl Encoder<InitializeChecked> for InitializeCheckedCodec {
    fn encoded_len(&self, value: &InitializeChecked) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &InitializeChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[9, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<InitializeChecked> for InitializeCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeChecked, Error> {
        Constant(&[9, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(InitializeChecked {})
    }
}
/// Validated data for `AuthorizeChecked`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizeChecked {
    /// The `stake_authorize` wire field.
    pub stake_authorize: StakeAuthorize,
}
/// Bounded codec for [`AuthorizeChecked`].
#[derive(Clone, Copy, Debug)]
pub struct AuthorizeCheckedCodec;
impl Encoder<AuthorizeChecked> for AuthorizeCheckedCodec {
    fn encoded_len(&self, value: &AuthorizeChecked) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(StakeAuthorizeCodec.encoded_len(&value.stake_authorize)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AuthorizeChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[10, 0, 0, 0]).write(&(), writer)?;
        StakeAuthorizeCodec.write(&value.stake_authorize, writer)?;
        Ok(())
    }
}
impl Decoder<AuthorizeChecked> for AuthorizeCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AuthorizeChecked, Error> {
        Constant(&[10, 0, 0, 0]).read(reader)?;
        Ok(AuthorizeChecked {
            stake_authorize: StakeAuthorizeCodec.read(reader)?,
        })
    }
}
/// Validated data for `AuthorizeCheckedWithSeed`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizeCheckedWithSeed {
    /// The `stake_authorize` wire field.
    pub stake_authorize: StakeAuthorize,
    /// The `authority_seed` wire field.
    pub authority_seed: String,
    /// The `authority_owner` wire field.
    pub authority_owner: Address,
}
/// Bounded codec for [`AuthorizeCheckedWithSeed`].
#[derive(Clone, Copy, Debug)]
pub struct AuthorizeCheckedWithSeedCodec;
impl Encoder<AuthorizeCheckedWithSeed> for AuthorizeCheckedWithSeedCodec {
    fn encoded_len(&self, value: &AuthorizeCheckedWithSeed) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(StakeAuthorizeCodec.encoded_len(&value.stake_authorize)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Number::<u64>::LE,
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.authority_seed.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.authority_owner)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &AuthorizeCheckedWithSeed,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[11, 0, 0, 0]).write(&(), writer)?;
        StakeAuthorizeCodec.write(&value.stake_authorize, writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Number::<u64>::LE,
            consumption: FrameConsumption::Exact,
        }
        .write(value.authority_seed.as_str(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.authority_owner, writer)?;
        Ok(())
    }
}
impl Decoder<AuthorizeCheckedWithSeed> for AuthorizeCheckedWithSeedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AuthorizeCheckedWithSeed, Error> {
        Constant(&[11, 0, 0, 0]).read(reader)?;
        Ok(AuthorizeCheckedWithSeed {
            stake_authorize: StakeAuthorizeCodec.read(reader)?,
            authority_seed: ByteSizePrefix {
                item: Utf8,
                prefix: Number::<u64>::LE,
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            authority_owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `SetLockupChecked`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetLockupChecked {
    /// The `unix_timestamp` wire field.
    pub unix_timestamp: Option<UnixTimestamp>,
    /// The `epoch` wire field.
    pub epoch: Option<Epoch>,
}
/// Bounded codec for [`SetLockupChecked`].
#[derive(Clone, Copy, Debug)]
pub struct SetLockupCheckedCodec;
impl Encoder<SetLockupChecked> for SetLockupCheckedCodec {
    fn encoded_len(&self, value: &SetLockupChecked) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(OptionCodec::u8(UnixTimestampCodec).encoded_len(&value.unix_timestamp)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(EpochCodec).encoded_len(&value.epoch)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetLockupChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[12, 0, 0, 0]).write(&(), writer)?;
        OptionCodec::u8(UnixTimestampCodec).write(&value.unix_timestamp, writer)?;
        OptionCodec::u8(EpochCodec).write(&value.epoch, writer)?;
        Ok(())
    }
}
impl Decoder<SetLockupChecked> for SetLockupCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetLockupChecked, Error> {
        Constant(&[12, 0, 0, 0]).read(reader)?;
        Ok(SetLockupChecked {
            unix_timestamp: OptionCodec::u8(UnixTimestampCodec).read(reader)?,
            epoch: OptionCodec::u8(EpochCodec).read(reader)?,
        })
    }
}
/// Validated data for `GetMinimumDelegation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GetMinimumDelegation {}
/// Bounded codec for [`GetMinimumDelegation`].
#[derive(Clone, Copy, Debug)]
pub struct GetMinimumDelegationCodec;
impl Encoder<GetMinimumDelegation> for GetMinimumDelegationCodec {
    fn encoded_len(&self, value: &GetMinimumDelegation) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &GetMinimumDelegation, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[13, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<GetMinimumDelegation> for GetMinimumDelegationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<GetMinimumDelegation, Error> {
        Constant(&[13, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(GetMinimumDelegation {})
    }
}
/// Validated data for `DeactivateDelinquent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeactivateDelinquent {}
/// Bounded codec for [`DeactivateDelinquent`].
#[derive(Clone, Copy, Debug)]
pub struct DeactivateDelinquentCodec;
impl Encoder<DeactivateDelinquent> for DeactivateDelinquentCodec {
    fn encoded_len(&self, value: &DeactivateDelinquent) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &DeactivateDelinquent, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[14, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DeactivateDelinquent> for DeactivateDelinquentCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DeactivateDelinquent, Error> {
        Constant(&[14, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(DeactivateDelinquent {})
    }
}
/// Validated data for `MoveStake`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MoveStake {
    /// The `args` wire field.
    pub args: u64,
}
/// Bounded codec for [`MoveStake`].
#[derive(Clone, Copy, Debug)]
pub struct MoveStakeCodec;
impl Encoder<MoveStake> for MoveStakeCodec {
    fn encoded_len(&self, value: &MoveStake) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MoveStake, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[16, 0, 0, 0]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.args, writer)?;
        Ok(())
    }
}
impl Decoder<MoveStake> for MoveStakeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MoveStake, Error> {
        Constant(&[16, 0, 0, 0]).read(reader)?;
        Ok(MoveStake {
            args: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `MoveLamports`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MoveLamports {
    /// The `args` wire field.
    pub args: u64,
}
/// Bounded codec for [`MoveLamports`].
#[derive(Clone, Copy, Debug)]
pub struct MoveLamportsCodec;
impl Encoder<MoveLamports> for MoveLamportsCodec {
    fn encoded_len(&self, value: &MoveLamports) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MoveLamports, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[17, 0, 0, 0]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.args, writer)?;
        Ok(())
    }
}
impl Decoder<MoveLamports> for MoveLamportsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MoveLamports, Error> {
        Constant(&[17, 0, 0, 0]).read(reader)?;
        Ok(MoveLamports {
            args: Number::<u64>::LE.read(reader)?,
        })
    }
}
use alloc::string::String;
use solana_kit_core::codecs::FixedBytes;
use solana_kit_core::codecs::Mapped;
use solana_kit_core::codecs::OptionCodec;
use solana_kit_core::codecs::Utf8;
use solana_kit_core::codecs::{ByteSizePrefix, FrameConsumption};
use solana_kit_core::codecs::{Count, OwnedSequence, Sequence};
