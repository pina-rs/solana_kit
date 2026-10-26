// Generated from a validated Codama model. See README.md and upstream notices.
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Constant, Decoder, Encoder, Error, Limits, Number, Reader, Writer},
};
/// Selected trusted program identity.
pub const PROGRAM: Address = Address::from_bytes([
    187, 199, 119, 175, 186, 104, 240, 18, 62, 192, 196, 209, 183, 238, 137, 110, 147, 1, 201, 41,
    197, 90, 6, 186, 149, 24, 33, 88, 37, 140, 21, 169,
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
const PROGRAM_ERROR_100: u32 = 100;
const PROGRAM_ERROR_101: u32 = 101;
const PROGRAM_ERROR_102: u32 = 102;
const PROGRAM_ERROR_103: u32 = 103;
const PROGRAM_ERROR_104: u32 = 104;
const PROGRAM_ERROR_105: u32 = 105;
const PROGRAM_ERROR_106: u32 = 106;
const PROGRAM_ERROR_107: u32 = 107;
const PROGRAM_ERROR_108: u32 = 108;
const PROGRAM_ERROR_109: u32 = 109;
const PROGRAM_ERROR_110: u32 = 110;
const PROGRAM_ERROR_111: u32 = 111;
const PROGRAM_ERROR_112: u32 = 112;
const PROGRAM_ERROR_113: u32 = 113;
const PROGRAM_ERROR_114: u32 = 114;
const PROGRAM_ERROR_115: u32 = 115;
const PROGRAM_ERROR_116: u32 = 116;
const PROGRAM_ERROR_117: u32 = 117;
const PROGRAM_ERROR_118: u32 = 118;
const PROGRAM_ERROR_119: u32 = 119;
const PROGRAM_ERROR_120: u32 = 120;
const PROGRAM_ERROR_121: u32 = 121;
const PROGRAM_ERROR_122: u32 = 122;
const PROGRAM_ERROR_123: u32 = 123;
const PROGRAM_ERROR_124: u32 = 124;
const PROGRAM_ERROR_125: u32 = 125;
const PROGRAM_ERROR_126: u32 = 126;
const PROGRAM_ERROR_127: u32 = 127;
const PROGRAM_ERROR_128: u32 = 128;
const PROGRAM_ERROR_129: u32 = 129;
const PROGRAM_ERROR_130: u32 = 130;
const PROGRAM_ERROR_131: u32 = 131;
const PROGRAM_ERROR_132: u32 = 132;
const PROGRAM_ERROR_133: u32 = 133;
const PROGRAM_ERROR_134: u32 = 134;
const PROGRAM_ERROR_135: u32 = 135;
const PROGRAM_ERROR_136: u32 = 136;
const PROGRAM_ERROR_137: u32 = 137;
const PROGRAM_ERROR_300: u32 = 300;
const PROGRAM_ERROR_301: u32 = 301;
const PROGRAM_ERROR_302: u32 = 302;
const PROGRAM_ERROR_400: u32 = 400;
const PROGRAM_ERROR_401: u32 = 401;
const PROGRAM_ERROR_402: u32 = 402;
const PROGRAM_ERROR_403: u32 = 403;
const PROGRAM_ERROR_404: u32 = 404;
const PROGRAM_ERROR_405: u32 = 405;
const PROGRAM_ERROR_406: u32 = 406;
const PROGRAM_ERROR_407: u32 = 407;
const PROGRAM_ERROR_408: u32 = 408;
const PROGRAM_ERROR_500: u32 = 500;
const PROGRAM_ERROR_501: u32 = 501;
const PROGRAM_ERROR_502: u32 = 502;
const PROGRAM_ERROR_503: u32 = 503;
const PROGRAM_ERROR_504: u32 = 504;
const PROGRAM_ERROR_505: u32 = 505;
const PROGRAM_ERROR_506: u32 = 506;
const PROGRAM_ERROR_507: u32 = 507;
const PROGRAM_ERROR_508: u32 = 508;
const PROGRAM_ERROR_509: u32 = 509;
const PROGRAM_ERROR_510: u32 = 510;
const PROGRAM_ERROR_511: u32 = 511;
const PROGRAM_ERROR_512: u32 = 512;
const PROGRAM_ERROR_513: u32 = 513;
const PROGRAM_ERROR_514: u32 = 514;
const PROGRAM_ERROR_515: u32 = 515;
const PROGRAM_ERROR_516: u32 = 516;
const PROGRAM_ERROR_517: u32 = 517;
const PROGRAM_ERROR_518: u32 = 518;
const PROGRAM_ERROR_519: u32 = 519;
const PROGRAM_ERROR_520: u32 = 520;
const PROGRAM_ERROR_521: u32 = 521;
const PROGRAM_ERROR_522: u32 = 522;
const PROGRAM_ERROR_600: u32 = 600;
const PROGRAM_ERROR_601: u32 = 601;
const PROGRAM_ERROR_602: u32 = 602;
const PROGRAM_ERROR_603: u32 = 603;
const PROGRAM_ERROR_604: u32 = 604;
const FIXED_COUNT_4: usize = 4;
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
/// Validated data for `CancelSubscriptionNowData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CancelSubscriptionNowData {
    /// The `expected_current_period_start_ts` wire field.
    pub expected_current_period_start_ts: i64,
}
/// Bounded codec for [`CancelSubscriptionNowData`].
#[derive(Clone, Copy, Debug)]
pub struct CancelSubscriptionNowDataCodec;
impl Encoder<CancelSubscriptionNowData> for CancelSubscriptionNowDataCodec {
    fn encoded_len(&self, value: &CancelSubscriptionNowData) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expected_current_period_start_ts)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &CancelSubscriptionNowData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<i64>::LE.write(&value.expected_current_period_start_ts, writer)?;
        Ok(())
    }
}
impl Decoder<CancelSubscriptionNowData> for CancelSubscriptionNowDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CancelSubscriptionNowData, Error> {
        Ok(CancelSubscriptionNowData {
            expected_current_period_start_ts: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `CreateFixedDelegationData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateFixedDelegationData {
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `amount` wire field.
    pub amount: u64,
    /// The `expiry_ts` wire field.
    pub expiry_ts: i64,
    /// The `expected_subscription_authority_init_id` wire field.
    pub expected_subscription_authority_init_id: i64,
}
/// Bounded codec for [`CreateFixedDelegationData`].
#[derive(Clone, Copy, Debug)]
pub struct CreateFixedDelegationDataCodec;
impl Encoder<CreateFixedDelegationData> for CreateFixedDelegationDataCodec {
    fn encoded_len(&self, value: &CreateFixedDelegationData) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expiry_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<i64>::LE.encoded_len(&value.expected_subscription_authority_init_id)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &CreateFixedDelegationData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<i64>::LE.write(&value.expiry_ts, writer)?;
        Number::<i64>::LE.write(&value.expected_subscription_authority_init_id, writer)?;
        Ok(())
    }
}
impl Decoder<CreateFixedDelegationData> for CreateFixedDelegationDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateFixedDelegationData, Error> {
        Ok(CreateFixedDelegationData {
            nonce: Number::<u64>::LE.read(reader)?,
            amount: Number::<u64>::LE.read(reader)?,
            expiry_ts: Number::<i64>::LE.read(reader)?,
            expected_subscription_authority_init_id: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `PlanTerms`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanTerms {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `period_hours` wire field.
    pub period_hours: u64,
    /// The `created_at` wire field.
    pub created_at: i64,
}
/// Bounded codec for [`PlanTerms`].
#[derive(Clone, Copy, Debug)]
pub struct PlanTermsCodec;
impl Encoder<PlanTerms> for PlanTermsCodec {
    fn encoded_len(&self, value: &PlanTerms) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.period_hours)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.created_at)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PlanTerms, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u64>::LE.write(&value.period_hours, writer)?;
        Number::<i64>::LE.write(&value.created_at, writer)?;
        Ok(())
    }
}
impl Decoder<PlanTerms> for PlanTermsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PlanTerms, Error> {
        Ok(PlanTerms {
            amount: Number::<u64>::LE.read(reader)?,
            period_hours: Number::<u64>::LE.read(reader)?,
            created_at: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `PlanData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanData {
    /// The `plan_id` wire field.
    pub plan_id: u64,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `terms` wire field.
    pub terms: PlanTerms,
    /// The `end_ts` wire field.
    pub end_ts: i64,
    /// The `destinations` wire field.
    pub destinations: Vec<Address>,
    /// The `pullers` wire field.
    pub pullers: Vec<Address>,
    /// The `metadata_uri` wire field.
    pub metadata_uri: String,
}
/// Bounded codec for [`PlanData`].
#[derive(Clone, Copy, Debug)]
pub struct PlanDataCodec;
impl Encoder<PlanData> for PlanDataCodec {
    fn encoded_len(&self, value: &PlanData) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.plan_id)?)
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
            .checked_add(PlanTermsCodec.encoded_len(&value.terms)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.end_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    count: Count::Fixed(FIXED_COUNT_4),
                })
                .encoded_len(&value.destinations)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    count: Count::Fixed(FIXED_COUNT_4),
                })
                .encoded_len(&value.pullers)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                FixedSize {
                    item: Utf8Config::default(),
                    size: 128,
                    decoded_size: None,
                    allow_truncation: false,
                    max_encoded_size: 128,
                }
                .encoded_len(value.metadata_uri.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PlanData, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.plan_id, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        PlanTermsCodec.write(&value.terms, writer)?;
        Number::<i64>::LE.write(&value.end_ts, writer)?;
        OwnedSequence(Sequence {
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            count: Count::Fixed(FIXED_COUNT_4),
        })
        .write(&value.destinations, writer)?;
        OwnedSequence(Sequence {
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            count: Count::Fixed(FIXED_COUNT_4),
        })
        .write(&value.pullers, writer)?;
        FixedSize {
            item: Utf8Config::default(),
            size: 128,
            decoded_size: None,
            allow_truncation: false,
            max_encoded_size: 128,
        }
        .write(value.metadata_uri.as_str(), writer)?;
        Ok(())
    }
}
impl Decoder<PlanData> for PlanDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PlanData, Error> {
        Ok(PlanData {
            plan_id: Number::<u64>::LE.read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            terms: PlanTermsCodec.read(reader)?,
            end_ts: Number::<i64>::LE.read(reader)?,
            destinations: OwnedSequence(Sequence {
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                count: Count::Fixed(FIXED_COUNT_4),
            })
            .read(reader)?,
            pullers: OwnedSequence(Sequence {
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                count: Count::Fixed(FIXED_COUNT_4),
            })
            .read(reader)?,
            metadata_uri: FixedSize {
                item: Utf8Config::default(),
                size: 128,
                decoded_size: None,
                allow_truncation: false,
                max_encoded_size: 128,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `CreateRecurringDelegationData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateRecurringDelegationData {
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `amount_per_period` wire field.
    pub amount_per_period: u64,
    /// The `period_length_s` wire field.
    pub period_length_s: u64,
    /// The `start_ts` wire field.
    pub start_ts: i64,
    /// The `expiry_ts` wire field.
    pub expiry_ts: i64,
    /// The `expected_subscription_authority_init_id` wire field.
    pub expected_subscription_authority_init_id: i64,
}
/// Bounded codec for [`CreateRecurringDelegationData`].
#[derive(Clone, Copy, Debug)]
pub struct CreateRecurringDelegationDataCodec;
impl Encoder<CreateRecurringDelegationData> for CreateRecurringDelegationDataCodec {
    fn encoded_len(&self, value: &CreateRecurringDelegationData) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount_per_period)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.period_length_s)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.start_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expiry_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<i64>::LE.encoded_len(&value.expected_subscription_authority_init_id)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &CreateRecurringDelegationData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u64>::LE.write(&value.amount_per_period, writer)?;
        Number::<u64>::LE.write(&value.period_length_s, writer)?;
        Number::<i64>::LE.write(&value.start_ts, writer)?;
        Number::<i64>::LE.write(&value.expiry_ts, writer)?;
        Number::<i64>::LE.write(&value.expected_subscription_authority_init_id, writer)?;
        Ok(())
    }
}
impl Decoder<CreateRecurringDelegationData> for CreateRecurringDelegationDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateRecurringDelegationData, Error> {
        Ok(CreateRecurringDelegationData {
            nonce: Number::<u64>::LE.read(reader)?,
            amount_per_period: Number::<u64>::LE.read(reader)?,
            period_length_s: Number::<u64>::LE.read(reader)?,
            start_ts: Number::<i64>::LE.read(reader)?,
            expiry_ts: Number::<i64>::LE.read(reader)?,
            expected_subscription_authority_init_id: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `SubscribeData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscribeData {
    /// The `plan_id` wire field.
    pub plan_id: u64,
    /// The `plan_bump` wire field.
    pub plan_bump: u8,
    /// The `expected_mint` wire field.
    pub expected_mint: Address,
    /// The `expected_amount` wire field.
    pub expected_amount: u64,
    /// The `expected_period_hours` wire field.
    pub expected_period_hours: u64,
    /// The `expected_created_at` wire field.
    pub expected_created_at: i64,
    /// The `expected_subscription_authority_init_id` wire field.
    pub expected_subscription_authority_init_id: i64,
}
/// Bounded codec for [`SubscribeData`].
#[derive(Clone, Copy, Debug)]
pub struct SubscribeDataCodec;
impl Encoder<SubscribeData> for SubscribeDataCodec {
    fn encoded_len(&self, value: &SubscribeData) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.plan_id)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.plan_bump)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.expected_mint)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.expected_amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.expected_period_hours)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expected_created_at)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Number::<i64>::LE.encoded_len(&value.expected_subscription_authority_init_id)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SubscribeData, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.plan_id, writer)?;
        Number::<u8>::LE.write(&value.plan_bump, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.expected_mint, writer)?;
        Number::<u64>::LE.write(&value.expected_amount, writer)?;
        Number::<u64>::LE.write(&value.expected_period_hours, writer)?;
        Number::<i64>::LE.write(&value.expected_created_at, writer)?;
        Number::<i64>::LE.write(&value.expected_subscription_authority_init_id, writer)?;
        Ok(())
    }
}
impl Decoder<SubscribeData> for SubscribeDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SubscribeData, Error> {
        Ok(SubscribeData {
            plan_id: Number::<u64>::LE.read(reader)?,
            plan_bump: Number::<u8>::LE.read(reader)?,
            expected_mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            expected_amount: Number::<u64>::LE.read(reader)?,
            expected_period_hours: Number::<u64>::LE.read(reader)?,
            expected_created_at: Number::<i64>::LE.read(reader)?,
            expected_subscription_authority_init_id: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `UpdatePlanData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdatePlanData {
    /// The `status` wire field.
    pub status: u8,
    /// The `end_ts` wire field.
    pub end_ts: i64,
    /// The `pullers` wire field.
    pub pullers: Vec<Address>,
    /// The `metadata_uri` wire field.
    pub metadata_uri: String,
    /// The `expected_created_at` wire field.
    pub expected_created_at: i64,
    /// The `expected_end_ts` wire field.
    pub expected_end_ts: i64,
    /// The `expected_pullers` wire field.
    pub expected_pullers: Vec<Address>,
    /// The `expected_metadata_uri` wire field.
    pub expected_metadata_uri: String,
}
/// Bounded codec for [`UpdatePlanData`].
#[derive(Clone, Copy, Debug)]
pub struct UpdatePlanDataCodec;
impl Encoder<UpdatePlanData> for UpdatePlanDataCodec {
    fn encoded_len(&self, value: &UpdatePlanData) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.status)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.end_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    count: Count::Fixed(FIXED_COUNT_4),
                })
                .encoded_len(&value.pullers)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                FixedSize {
                    item: Utf8Config::default(),
                    size: 128,
                    decoded_size: None,
                    allow_truncation: false,
                    max_encoded_size: 128,
                }
                .encoded_len(value.metadata_uri.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expected_created_at)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expected_end_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    count: Count::Fixed(FIXED_COUNT_4),
                })
                .encoded_len(&value.expected_pullers)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                FixedSize {
                    item: Utf8Config::default(),
                    size: 128,
                    decoded_size: None,
                    allow_truncation: false,
                    max_encoded_size: 128,
                }
                .encoded_len(value.expected_metadata_uri.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdatePlanData, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.status, writer)?;
        Number::<i64>::LE.write(&value.end_ts, writer)?;
        OwnedSequence(Sequence {
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            count: Count::Fixed(FIXED_COUNT_4),
        })
        .write(&value.pullers, writer)?;
        FixedSize {
            item: Utf8Config::default(),
            size: 128,
            decoded_size: None,
            allow_truncation: false,
            max_encoded_size: 128,
        }
        .write(value.metadata_uri.as_str(), writer)?;
        Number::<i64>::LE.write(&value.expected_created_at, writer)?;
        Number::<i64>::LE.write(&value.expected_end_ts, writer)?;
        OwnedSequence(Sequence {
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            count: Count::Fixed(FIXED_COUNT_4),
        })
        .write(&value.expected_pullers, writer)?;
        FixedSize {
            item: Utf8Config::default(),
            size: 128,
            decoded_size: None,
            allow_truncation: false,
            max_encoded_size: 128,
        }
        .write(value.expected_metadata_uri.as_str(), writer)?;
        Ok(())
    }
}
impl Decoder<UpdatePlanData> for UpdatePlanDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdatePlanData, Error> {
        Ok(UpdatePlanData {
            status: Number::<u8>::LE.read(reader)?,
            end_ts: Number::<i64>::LE.read(reader)?,
            pullers: OwnedSequence(Sequence {
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                count: Count::Fixed(FIXED_COUNT_4),
            })
            .read(reader)?,
            metadata_uri: FixedSize {
                item: Utf8Config::default(),
                size: 128,
                decoded_size: None,
                allow_truncation: false,
                max_encoded_size: 128,
            }
            .read(reader)?,
            expected_created_at: Number::<i64>::LE.read(reader)?,
            expected_end_ts: Number::<i64>::LE.read(reader)?,
            expected_pullers: OwnedSequence(Sequence {
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                count: Count::Fixed(FIXED_COUNT_4),
            })
            .read(reader)?,
            expected_metadata_uri: FixedSize {
                item: Utf8Config::default(),
                size: 128,
                decoded_size: None,
                allow_truncation: false,
                max_encoded_size: 128,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `TransferData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferData {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `delegator` wire field.
    pub delegator: Address,
    /// The `mint` wire field.
    pub mint: Address,
}
/// Bounded codec for [`TransferData`].
#[derive(Clone, Copy, Debug)]
pub struct TransferDataCodec;
impl Encoder<TransferData> for TransferDataCodec {
    fn encoded_len(&self, value: &TransferData) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegator)?,
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
        Ok(size)
    }
    fn write(&self, value: &TransferData, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.amount, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegator, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Ok(())
    }
}
impl Decoder<TransferData> for TransferDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferData, Error> {
        Ok(TransferData {
            amount: Number::<u64>::LE.read(reader)?,
            delegator: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `ResumeData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResumeData {
    /// The `expected_expires_at_ts` wire field.
    pub expected_expires_at_ts: i64,
}
/// Bounded codec for [`ResumeData`].
#[derive(Clone, Copy, Debug)]
pub struct ResumeDataCodec;
impl Encoder<ResumeData> for ResumeDataCodec {
    fn encoded_len(&self, value: &ResumeData) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expected_expires_at_ts)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ResumeData, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<i64>::LE.write(&value.expected_expires_at_ts, writer)?;
        Ok(())
    }
}
impl Decoder<ResumeData> for ResumeDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ResumeData, Error> {
        Ok(ResumeData {
            expected_expires_at_ts: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccountDiscriminator {
    /// The `SubscriptionAuthority` variant.
    SubscriptionAuthority,
    /// The `Plan` variant.
    Plan,
    /// The `FixedDelegation` variant.
    FixedDelegation,
    /// The `RecurringDelegation` variant.
    RecurringDelegation,
    /// The `SubscriptionDelegation` variant.
    SubscriptionDelegation,
}
/// Bounded codec for [`AccountDiscriminator`].
#[derive(Clone, Copy, Debug)]
pub struct AccountDiscriminatorCodec;
const ACCOUNTDISCRIMINATOR_TAG_0: u8 = 0;
const ACCOUNTDISCRIMINATOR_TAG_1: u8 = 1;
const ACCOUNTDISCRIMINATOR_TAG_2: u8 = 2;
const ACCOUNTDISCRIMINATOR_TAG_3: u8 = 3;
const ACCOUNTDISCRIMINATOR_TAG_4: u8 = 4;
impl Encoder<AccountDiscriminator> for AccountDiscriminatorCodec {
    fn encoded_len(&self, value: &AccountDiscriminator) -> Result<usize, Error> {
        let size = match value {
            AccountDiscriminator::SubscriptionAuthority => 0usize,
            AccountDiscriminator::Plan => 0usize,
            AccountDiscriminator::FixedDelegation => 0usize,
            AccountDiscriminator::RecurringDelegation => 0usize,
            AccountDiscriminator::SubscriptionDelegation => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &AccountDiscriminator, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            AccountDiscriminator::SubscriptionAuthority => {
                Number::<u8>::LE.write(&ACCOUNTDISCRIMINATOR_TAG_0, writer)?;
                Ok(())
            }
            AccountDiscriminator::Plan => {
                Number::<u8>::LE.write(&ACCOUNTDISCRIMINATOR_TAG_1, writer)?;
                Ok(())
            }
            AccountDiscriminator::FixedDelegation => {
                Number::<u8>::LE.write(&ACCOUNTDISCRIMINATOR_TAG_2, writer)?;
                Ok(())
            }
            AccountDiscriminator::RecurringDelegation => {
                Number::<u8>::LE.write(&ACCOUNTDISCRIMINATOR_TAG_3, writer)?;
                Ok(())
            }
            AccountDiscriminator::SubscriptionDelegation => {
                Number::<u8>::LE.write(&ACCOUNTDISCRIMINATOR_TAG_4, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<AccountDiscriminator> for AccountDiscriminatorCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AccountDiscriminator, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            ACCOUNTDISCRIMINATOR_TAG_0 => Ok(AccountDiscriminator::SubscriptionAuthority),
            ACCOUNTDISCRIMINATOR_TAG_1 => Ok(AccountDiscriminator::Plan),
            ACCOUNTDISCRIMINATOR_TAG_2 => Ok(AccountDiscriminator::FixedDelegation),
            ACCOUNTDISCRIMINATOR_TAG_3 => Ok(AccountDiscriminator::RecurringDelegation),
            ACCOUNTDISCRIMINATOR_TAG_4 => Ok(AccountDiscriminator::SubscriptionDelegation),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanStatus {
    /// The `Sunset` variant.
    Sunset,
    /// The `Active` variant.
    Active,
}
/// Bounded codec for [`PlanStatus`].
#[derive(Clone, Copy, Debug)]
pub struct PlanStatusCodec;
const PLANSTATUS_TAG_0: u8 = 0;
const PLANSTATUS_TAG_1: u8 = 1;
impl Encoder<PlanStatus> for PlanStatusCodec {
    fn encoded_len(&self, value: &PlanStatus) -> Result<usize, Error> {
        let size = match value {
            PlanStatus::Sunset => 0usize,
            PlanStatus::Active => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &PlanStatus, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            PlanStatus::Sunset => {
                Number::<u8>::LE.write(&PLANSTATUS_TAG_0, writer)?;
                Ok(())
            }
            PlanStatus::Active => {
                Number::<u8>::LE.write(&PLANSTATUS_TAG_1, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<PlanStatus> for PlanStatusCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PlanStatus, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            PLANSTATUS_TAG_0 => Ok(PlanStatus::Sunset),
            PLANSTATUS_TAG_1 => Ok(PlanStatus::Active),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `Header`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
    /// The `discriminator` wire field.
    pub discriminator: u8,
    /// The `version` wire field.
    pub version: u8,
    /// The `bump` wire field.
    pub bump: u8,
    /// The `delegator` wire field.
    pub delegator: Address,
    /// The `delegatee` wire field.
    pub delegatee: Address,
    /// The `payer` wire field.
    pub payer: Address,
    /// The `init_id` wire field.
    pub init_id: i64,
}
/// Bounded codec for [`Header`].
#[derive(Clone, Copy, Debug)]
pub struct HeaderCodec;
impl Encoder<Header> for HeaderCodec {
    fn encoded_len(&self, value: &Header) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.discriminator)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.version)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegator)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegatee)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.payer)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.init_id)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Header, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.discriminator, writer)?;
        Number::<u8>::LE.write(&value.version, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegator, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegatee, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.payer, writer)?;
        Number::<i64>::LE.write(&value.init_id, writer)?;
        Ok(())
    }
}
impl Decoder<Header> for HeaderCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Header, Error> {
        Ok(Header {
            discriminator: Number::<u8>::LE.read(reader)?,
            version: Number::<u8>::LE.read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
            delegator: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            delegatee: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            payer: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            init_id: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `FixedDelegation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedDelegation {
    /// The `header` wire field.
    pub header: Header,
    /// The `subscription_authority` wire field.
    pub subscription_authority: Address,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `amount` wire field.
    pub amount: u64,
    /// The `expiry_ts` wire field.
    pub expiry_ts: i64,
}
/// Bounded codec for [`FixedDelegation`].
#[derive(Clone, Copy, Debug)]
pub struct FixedDelegationCodec;
impl Encoder<FixedDelegation> for FixedDelegationCodec {
    fn encoded_len(&self, value: &FixedDelegation) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(HeaderCodec.encoded_len(&value.header)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.subscription_authority)?,
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
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expiry_ts)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &FixedDelegation, writer: &mut Writer<'_>) -> Result<(), Error> {
        HeaderCodec.write(&value.header, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.subscription_authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<i64>::LE.write(&value.expiry_ts, writer)?;
        Ok(())
    }
}
impl Decoder<FixedDelegation> for FixedDelegationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<FixedDelegation, Error> {
        Ok(FixedDelegation {
            header: HeaderCodec.read(reader)?,
            subscription_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            amount: Number::<u64>::LE.read(reader)?,
            expiry_ts: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `Plan`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    /// The `discriminator` wire field.
    pub discriminator: u8,
    /// The `owner` wire field.
    pub owner: Address,
    /// The `bump` wire field.
    pub bump: u8,
    /// The `status` wire field.
    pub status: u8,
    /// The `data` wire field.
    pub data: PlanData,
}
/// Bounded codec for [`Plan`].
#[derive(Clone, Copy, Debug)]
pub struct PlanCodec;
impl Encoder<Plan> for PlanCodec {
    fn encoded_len(&self, value: &Plan) -> Result<usize, Error> {
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
                .encoded_len(&value.owner)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.status)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(PlanDataCodec.encoded_len(&value.data)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Plan, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.discriminator, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.owner, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
        Number::<u8>::LE.write(&value.status, writer)?;
        PlanDataCodec.write(&value.data, writer)?;
        Ok(())
    }
}
impl Decoder<Plan> for PlanCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Plan, Error> {
        Ok(Plan {
            discriminator: Number::<u8>::LE.read(reader)?,
            owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
            status: Number::<u8>::LE.read(reader)?,
            data: PlanDataCodec.read(reader)?,
        })
    }
}
/// Validated data for `RecurringDelegation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecurringDelegation {
    /// The `header` wire field.
    pub header: Header,
    /// The `subscription_authority` wire field.
    pub subscription_authority: Address,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `current_period_start_ts` wire field.
    pub current_period_start_ts: i64,
    /// The `period_length_s` wire field.
    pub period_length_s: u64,
    /// The `expiry_ts` wire field.
    pub expiry_ts: i64,
    /// The `amount_per_period` wire field.
    pub amount_per_period: u64,
    /// The `amount_pulled_in_period` wire field.
    pub amount_pulled_in_period: u64,
}
/// Bounded codec for [`RecurringDelegation`].
#[derive(Clone, Copy, Debug)]
pub struct RecurringDelegationCodec;
impl Encoder<RecurringDelegation> for RecurringDelegationCodec {
    fn encoded_len(&self, value: &RecurringDelegation) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(HeaderCodec.encoded_len(&value.header)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.subscription_authority)?,
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
            .checked_add(Number::<i64>::LE.encoded_len(&value.current_period_start_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.period_length_s)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expiry_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount_per_period)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount_pulled_in_period)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &RecurringDelegation, writer: &mut Writer<'_>) -> Result<(), Error> {
        HeaderCodec.write(&value.header, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.subscription_authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Number::<i64>::LE.write(&value.current_period_start_ts, writer)?;
        Number::<u64>::LE.write(&value.period_length_s, writer)?;
        Number::<i64>::LE.write(&value.expiry_ts, writer)?;
        Number::<u64>::LE.write(&value.amount_per_period, writer)?;
        Number::<u64>::LE.write(&value.amount_pulled_in_period, writer)?;
        Ok(())
    }
}
impl Decoder<RecurringDelegation> for RecurringDelegationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RecurringDelegation, Error> {
        Ok(RecurringDelegation {
            header: HeaderCodec.read(reader)?,
            subscription_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            current_period_start_ts: Number::<i64>::LE.read(reader)?,
            period_length_s: Number::<u64>::LE.read(reader)?,
            expiry_ts: Number::<i64>::LE.read(reader)?,
            amount_per_period: Number::<u64>::LE.read(reader)?,
            amount_pulled_in_period: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `SubscriptionAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionAuthority {
    /// The `discriminator` wire field.
    pub discriminator: u8,
    /// The `user` wire field.
    pub user: Address,
    /// The `token_mint` wire field.
    pub token_mint: Address,
    /// The `payer` wire field.
    pub payer: Address,
    /// The `bump` wire field.
    pub bump: u8,
    /// The `init_id` wire field.
    pub init_id: i64,
}
/// Bounded codec for [`SubscriptionAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionAuthorityCodec;
impl Encoder<SubscriptionAuthority> for SubscriptionAuthorityCodec {
    fn encoded_len(&self, value: &SubscriptionAuthority) -> Result<usize, Error> {
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
                .encoded_len(&value.user)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.token_mint)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.payer)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.init_id)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SubscriptionAuthority, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.discriminator, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.user, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.token_mint, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.payer, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
        Number::<i64>::LE.write(&value.init_id, writer)?;
        Ok(())
    }
}
impl Decoder<SubscriptionAuthority> for SubscriptionAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SubscriptionAuthority, Error> {
        Ok(SubscriptionAuthority {
            discriminator: Number::<u8>::LE.read(reader)?,
            user: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            token_mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            payer: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
            init_id: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `SubscriptionDelegation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionDelegation {
    /// The `header` wire field.
    pub header: Header,
    /// The `terms` wire field.
    pub terms: PlanTerms,
    /// The `amount_pulled_in_period` wire field.
    pub amount_pulled_in_period: u64,
    /// The `current_period_start_ts` wire field.
    pub current_period_start_ts: i64,
    /// The `expires_at_ts` wire field.
    pub expires_at_ts: i64,
}
/// Bounded codec for [`SubscriptionDelegation`].
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionDelegationCodec;
impl Encoder<SubscriptionDelegation> for SubscriptionDelegationCodec {
    fn encoded_len(&self, value: &SubscriptionDelegation) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(HeaderCodec.encoded_len(&value.header)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(PlanTermsCodec.encoded_len(&value.terms)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount_pulled_in_period)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.current_period_start_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expires_at_ts)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SubscriptionDelegation, writer: &mut Writer<'_>) -> Result<(), Error> {
        HeaderCodec.write(&value.header, writer)?;
        PlanTermsCodec.write(&value.terms, writer)?;
        Number::<u64>::LE.write(&value.amount_pulled_in_period, writer)?;
        Number::<i64>::LE.write(&value.current_period_start_ts, writer)?;
        Number::<i64>::LE.write(&value.expires_at_ts, writer)?;
        Ok(())
    }
}
impl Decoder<SubscriptionDelegation> for SubscriptionDelegationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SubscriptionDelegation, Error> {
        Ok(SubscriptionDelegation {
            header: HeaderCodec.read(reader)?,
            terms: PlanTermsCodec.read(reader)?,
            amount_pulled_in_period: Number::<u64>::LE.read(reader)?,
            current_period_start_ts: Number::<i64>::LE.read(reader)?,
            expires_at_ts: Number::<i64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `EventAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventAuthority {}
/// Bounded codec for [`EventAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct EventAuthorityCodec;
impl Encoder<EventAuthority> for EventAuthorityCodec {
    fn encoded_len(&self, value: &EventAuthority) -> Result<usize, Error> {
        let size = 0usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &EventAuthority, writer: &mut Writer<'_>) -> Result<(), Error> {
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<EventAuthority> for EventAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EventAuthority, Error> {
        let _ = reader;
        Ok(EventAuthority {})
    }
}
/// Validated data for `FixedTransferEvent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedTransferEvent {
    /// The `delegation` wire field.
    pub delegation: Address,
    /// The `delegator` wire field.
    pub delegator: Address,
    /// The `delegatee` wire field.
    pub delegatee: Address,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `amount` wire field.
    pub amount: u64,
    /// The `remaining_amount` wire field.
    pub remaining_amount: u64,
    /// The `receiver` wire field.
    pub receiver: Address,
    /// The `receiver_token_account` wire field.
    pub receiver_token_account: Address,
}
/// Bounded codec for [`FixedTransferEvent`].
#[derive(Clone, Copy, Debug)]
pub struct FixedTransferEventCodec;
impl Encoder<FixedTransferEvent> for FixedTransferEventCodec {
    fn encoded_len(&self, value: &FixedTransferEvent) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 3],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.delegation)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegator)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegatee)?,
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
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.remaining_amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.receiver)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.receiver_token_account)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &FixedTransferEvent, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 3];
        let _record_start = writer.position();
        Affix {
            prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 3],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.delegation, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegator, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegatee, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u64>::LE.write(&value.remaining_amount, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.receiver, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.receiver_token_account, writer)?;
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
impl Decoder<FixedTransferEvent> for FixedTransferEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<FixedTransferEvent, Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 3];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = FixedTransferEvent {
            delegation: Affix {
                prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 3],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            delegator: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            delegatee: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            amount: Number::<u64>::LE.read(reader)?,
            remaining_amount: Number::<u64>::LE.read(reader)?,
            receiver: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            receiver_token_account: Mapped::new(
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
/// Validated data for `PlanUpdatedEvent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanUpdatedEvent {
    /// The `plan` wire field.
    pub plan: Address,
    /// The `owner` wire field.
    pub owner: Address,
    /// The `status` wire field.
    pub status: u8,
    /// The `end_ts` wire field.
    pub end_ts: i64,
    /// The `pullers` wire field.
    pub pullers: Vec<Address>,
}
/// Bounded codec for [`PlanUpdatedEvent`].
#[derive(Clone, Copy, Debug)]
pub struct PlanUpdatedEventCodec;
impl Encoder<PlanUpdatedEvent> for PlanUpdatedEventCodec {
    fn encoded_len(&self, value: &PlanUpdatedEvent) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 6],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.plan)?,
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
            .checked_add(Number::<u8>::LE.encoded_len(&value.status)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.end_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    count: Count::Fixed(FIXED_COUNT_4),
                })
                .encoded_len(&value.pullers)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PlanUpdatedEvent, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 6];
        let _record_start = writer.position();
        Affix {
            prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 6],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.plan, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.owner, writer)?;
        Number::<u8>::LE.write(&value.status, writer)?;
        Number::<i64>::LE.write(&value.end_ts, writer)?;
        OwnedSequence(Sequence {
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            count: Count::Fixed(FIXED_COUNT_4),
        })
        .write(&value.pullers, writer)?;
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
impl Decoder<PlanUpdatedEvent> for PlanUpdatedEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PlanUpdatedEvent, Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 6];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = PlanUpdatedEvent {
            plan: Affix {
                prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 6],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            status: Number::<u8>::LE.read(reader)?,
            end_ts: Number::<i64>::LE.read(reader)?,
            pullers: OwnedSequence(Sequence {
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                count: Count::Fixed(FIXED_COUNT_4),
            })
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
/// Validated data for `RecurringTransferEvent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecurringTransferEvent {
    /// The `delegation` wire field.
    pub delegation: Address,
    /// The `delegator` wire field.
    pub delegator: Address,
    /// The `delegatee` wire field.
    pub delegatee: Address,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `amount` wire field.
    pub amount: u64,
    /// The `period_start_ts` wire field.
    pub period_start_ts: i64,
    /// The `period_end_ts` wire field.
    pub period_end_ts: i64,
    /// The `amount_pulled_in_period` wire field.
    pub amount_pulled_in_period: u64,
    /// The `receiver` wire field.
    pub receiver: Address,
    /// The `receiver_token_account` wire field.
    pub receiver_token_account: Address,
}
/// Bounded codec for [`RecurringTransferEvent`].
#[derive(Clone, Copy, Debug)]
pub struct RecurringTransferEventCodec;
impl Encoder<RecurringTransferEvent> for RecurringTransferEventCodec {
    fn encoded_len(&self, value: &RecurringTransferEvent) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 4],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.delegation)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegator)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegatee)?,
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
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.period_start_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.period_end_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount_pulled_in_period)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.receiver)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.receiver_token_account)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &RecurringTransferEvent, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 4];
        let _record_start = writer.position();
        Affix {
            prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 4],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.delegation, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegator, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegatee, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<i64>::LE.write(&value.period_start_ts, writer)?;
        Number::<i64>::LE.write(&value.period_end_ts, writer)?;
        Number::<u64>::LE.write(&value.amount_pulled_in_period, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.receiver, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.receiver_token_account, writer)?;
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
impl Decoder<RecurringTransferEvent> for RecurringTransferEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RecurringTransferEvent, Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 4];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = RecurringTransferEvent {
            delegation: Affix {
                prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 4],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            delegator: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            delegatee: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            amount: Number::<u64>::LE.read(reader)?,
            period_start_ts: Number::<i64>::LE.read(reader)?,
            period_end_ts: Number::<i64>::LE.read(reader)?,
            amount_pulled_in_period: Number::<u64>::LE.read(reader)?,
            receiver: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            receiver_token_account: Mapped::new(
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
/// Validated data for `SubscriptionCancelledEvent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionCancelledEvent {
    /// The `plan` wire field.
    pub plan: Address,
    /// The `subscriber` wire field.
    pub subscriber: Address,
    /// The `expires_at_ts` wire field.
    pub expires_at_ts: i64,
}
/// Bounded codec for [`SubscriptionCancelledEvent`].
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionCancelledEventCodec;
impl Encoder<SubscriptionCancelledEvent> for SubscriptionCancelledEventCodec {
    fn encoded_len(&self, value: &SubscriptionCancelledEvent) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 1],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.plan)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.subscriber)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.expires_at_ts)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &SubscriptionCancelledEvent,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 1];
        let _record_start = writer.position();
        Affix {
            prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 1],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.plan, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.subscriber, writer)?;
        Number::<i64>::LE.write(&value.expires_at_ts, writer)?;
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
impl Decoder<SubscriptionCancelledEvent> for SubscriptionCancelledEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SubscriptionCancelledEvent, Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 1];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = SubscriptionCancelledEvent {
            plan: Affix {
                prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 1],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            subscriber: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            expires_at_ts: Number::<i64>::LE.read(reader)?,
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
/// Validated data for `SubscriptionCreatedEvent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionCreatedEvent {
    /// The `plan` wire field.
    pub plan: Address,
    /// The `subscriber` wire field.
    pub subscriber: Address,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `created_ts` wire field.
    pub created_ts: i64,
    /// The `payer` wire field.
    pub payer: Address,
}
/// Bounded codec for [`SubscriptionCreatedEvent`].
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionCreatedEventCodec;
impl Encoder<SubscriptionCreatedEvent> for SubscriptionCreatedEventCodec {
    fn encoded_len(&self, value: &SubscriptionCreatedEvent) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 0],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.plan)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.subscriber)?,
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
            .checked_add(Number::<i64>::LE.encoded_len(&value.created_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.payer)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &SubscriptionCreatedEvent,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 0];
        let _record_start = writer.position();
        Affix {
            prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 0],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.plan, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.subscriber, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Number::<i64>::LE.write(&value.created_ts, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.payer, writer)?;
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
impl Decoder<SubscriptionCreatedEvent> for SubscriptionCreatedEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SubscriptionCreatedEvent, Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 0];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = SubscriptionCreatedEvent {
            plan: Affix {
                prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 0],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            subscriber: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            created_ts: Number::<i64>::LE.read(reader)?,
            payer: Mapped::new(
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
/// Validated data for `SubscriptionResumedEvent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionResumedEvent {
    /// The `plan` wire field.
    pub plan: Address,
    /// The `subscriber` wire field.
    pub subscriber: Address,
    /// The `resumed_ts` wire field.
    pub resumed_ts: i64,
}
/// Bounded codec for [`SubscriptionResumedEvent`].
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionResumedEventCodec;
impl Encoder<SubscriptionResumedEvent> for SubscriptionResumedEventCodec {
    fn encoded_len(&self, value: &SubscriptionResumedEvent) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 5],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.plan)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.subscriber)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.resumed_ts)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &SubscriptionResumedEvent,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 5];
        let _record_start = writer.position();
        Affix {
            prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 5],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.plan, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.subscriber, writer)?;
        Number::<i64>::LE.write(&value.resumed_ts, writer)?;
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
impl Decoder<SubscriptionResumedEvent> for SubscriptionResumedEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SubscriptionResumedEvent, Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 5];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = SubscriptionResumedEvent {
            plan: Affix {
                prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 5],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            subscriber: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            resumed_ts: Number::<i64>::LE.read(reader)?,
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
/// Validated data for `SubscriptionTransferEvent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionTransferEvent {
    /// The `subscription` wire field.
    pub subscription: Address,
    /// The `plan` wire field.
    pub plan: Address,
    /// The `delegator` wire field.
    pub delegator: Address,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `amount` wire field.
    pub amount: u64,
    /// The `period_start_ts` wire field.
    pub period_start_ts: i64,
    /// The `period_end_ts` wire field.
    pub period_end_ts: i64,
    /// The `amount_pulled_in_period` wire field.
    pub amount_pulled_in_period: u64,
    /// The `receiver` wire field.
    pub receiver: Address,
    /// The `receiver_token_account` wire field.
    pub receiver_token_account: Address,
    /// The `puller` wire field.
    pub puller: Address,
}
/// Bounded codec for [`SubscriptionTransferEvent`].
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionTransferEventCodec;
impl Encoder<SubscriptionTransferEvent> for SubscriptionTransferEventCodec {
    fn encoded_len(&self, value: &SubscriptionTransferEvent) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 2],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.subscription)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.plan)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegator)?,
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
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.period_start_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<i64>::LE.encoded_len(&value.period_end_ts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount_pulled_in_period)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.receiver)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.receiver_token_account)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.puller)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &SubscriptionTransferEvent,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 2];
        let _record_start = writer.position();
        Affix {
            prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 2],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.subscription, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.plan, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegator, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<i64>::LE.write(&value.period_start_ts, writer)?;
        Number::<i64>::LE.write(&value.period_end_ts, writer)?;
        Number::<u64>::LE.write(&value.amount_pulled_in_period, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.receiver, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.receiver_token_account, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.puller, writer)?;
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
impl Decoder<SubscriptionTransferEvent> for SubscriptionTransferEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SubscriptionTransferEvent, Error> {
        const _DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 2];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = SubscriptionTransferEvent {
            subscription: Affix {
                prefix: &[228, 69, 165, 46, 81, 203, 154, 29, 2],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            plan: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            delegator: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            amount: Number::<u64>::LE.read(reader)?,
            period_start_ts: Number::<i64>::LE.read(reader)?,
            period_end_ts: Number::<i64>::LE.read(reader)?,
            amount_pulled_in_period: Number::<u64>::LE.read(reader)?,
            receiver: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            receiver_token_account: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            puller: Mapped::new(
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
/// Validated data for `InitSubscriptionAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitSubscriptionAuthority {}
/// Bounded codec for [`InitSubscriptionAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct InitSubscriptionAuthorityCodec;
impl Encoder<InitSubscriptionAuthority> for InitSubscriptionAuthorityCodec {
    fn encoded_len(&self, value: &InitSubscriptionAuthority) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitSubscriptionAuthority,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<InitSubscriptionAuthority> for InitSubscriptionAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitSubscriptionAuthority, Error> {
        Constant(&[INSTRUCTION_TAG_0]).read(reader)?;
        let _ = reader;
        Ok(InitSubscriptionAuthority {})
    }
}
impl InitSubscriptionAuthority {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 6 || accounts.len() > 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* owner */ (true, Some(true), false),
                /* subscription_authority */ (true, Some(false), false),
                /* token_mint */ (false, Some(false), false),
                /* user_ata */ (true, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* payer */ (true, Some(true), false),
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
        let data = InitSubscriptionAuthorityCodec.encode(self, limits.max_bytes)?;
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
        InitSubscriptionAuthorityCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateFixedDelegation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateFixedDelegation {
    /// The `fixed_delegation` wire field.
    pub fixed_delegation: CreateFixedDelegationData,
}
/// Bounded codec for [`CreateFixedDelegation`].
#[derive(Clone, Copy, Debug)]
pub struct CreateFixedDelegationCodec;
impl Encoder<CreateFixedDelegation> for CreateFixedDelegationCodec {
    fn encoded_len(&self, value: &CreateFixedDelegation) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(CreateFixedDelegationDataCodec.encoded_len(&value.fixed_delegation)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateFixedDelegation, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_1]).write(&(), writer)?;
        CreateFixedDelegationDataCodec.write(&value.fixed_delegation, writer)?;
        Ok(())
    }
}
impl Decoder<CreateFixedDelegation> for CreateFixedDelegationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateFixedDelegation, Error> {
        Constant(&[INSTRUCTION_TAG_1]).read(reader)?;
        Ok(CreateFixedDelegation {
            fixed_delegation: CreateFixedDelegationDataCodec.read(reader)?,
        })
    }
}
impl CreateFixedDelegation {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 5 || accounts.len() > 6 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* delegator */ (true, Some(true), false),
                /* subscription_authority */ (false, Some(false), false),
                /* delegation_account */ (true, Some(false), false),
                /* delegatee */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* payer */ (true, Some(true), false),
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
        let data = CreateFixedDelegationCodec.encode(self, limits.max_bytes)?;
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
        CreateFixedDelegationCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateRecurringDelegation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateRecurringDelegation {
    /// The `recurring_delegation` wire field.
    pub recurring_delegation: CreateRecurringDelegationData,
}
/// Bounded codec for [`CreateRecurringDelegation`].
#[derive(Clone, Copy, Debug)]
pub struct CreateRecurringDelegationCodec;
impl Encoder<CreateRecurringDelegation> for CreateRecurringDelegationCodec {
    fn encoded_len(&self, value: &CreateRecurringDelegation) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                CreateRecurringDelegationDataCodec.encoded_len(&value.recurring_delegation)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &CreateRecurringDelegation,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_2]).write(&(), writer)?;
        CreateRecurringDelegationDataCodec.write(&value.recurring_delegation, writer)?;
        Ok(())
    }
}
impl Decoder<CreateRecurringDelegation> for CreateRecurringDelegationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateRecurringDelegation, Error> {
        Constant(&[INSTRUCTION_TAG_2]).read(reader)?;
        Ok(CreateRecurringDelegation {
            recurring_delegation: CreateRecurringDelegationDataCodec.read(reader)?,
        })
    }
}
impl CreateRecurringDelegation {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 5 || accounts.len() > 6 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* delegator */ (true, Some(true), false),
                /* subscription_authority */ (false, Some(false), false),
                /* delegation_account */ (true, Some(false), false),
                /* delegatee */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* payer */ (true, Some(true), false),
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
        let data = CreateRecurringDelegationCodec.encode(self, limits.max_bytes)?;
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
        CreateRecurringDelegationCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RevokeDelegation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevokeDelegation {}
/// Bounded codec for [`RevokeDelegation`].
#[derive(Clone, Copy, Debug)]
pub struct RevokeDelegationCodec;
impl Encoder<RevokeDelegation> for RevokeDelegationCodec {
    fn encoded_len(&self, value: &RevokeDelegation) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &RevokeDelegation, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_3]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<RevokeDelegation> for RevokeDelegationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RevokeDelegation, Error> {
        Constant(&[INSTRUCTION_TAG_3]).read(reader)?;
        let _ = reader;
        Ok(RevokeDelegation {})
    }
}
impl RevokeDelegation {
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
                /* authority */ (true, Some(true), false),
                /* delegation_account */ (true, Some(false), false),
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
        let data = RevokeDelegationCodec.encode(self, limits.max_bytes)?;
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
        RevokeDelegationCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TransferFixed`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferFixed {
    /// The `transfer_data` wire field.
    pub transfer_data: TransferData,
}
/// Bounded codec for [`TransferFixed`].
#[derive(Clone, Copy, Debug)]
pub struct TransferFixedCodec;
impl Encoder<TransferFixed> for TransferFixedCodec {
    fn encoded_len(&self, value: &TransferFixed) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(TransferDataCodec.encoded_len(&value.transfer_data)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferFixed, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_4]).write(&(), writer)?;
        TransferDataCodec.write(&value.transfer_data, writer)?;
        Ok(())
    }
}
impl Decoder<TransferFixed> for TransferFixedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferFixed, Error> {
        Constant(&[INSTRUCTION_TAG_4]).read(reader)?;
        Ok(TransferFixed {
            transfer_data: TransferDataCodec.read(reader)?,
        })
    }
}
impl TransferFixed {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* delegation_pda */ (true, Some(false), false),
                /* subscription_authority */ (false, Some(false), false),
                /* delegator_ata */ (true, Some(false), false),
                /* receiver_ata */ (true, Some(false), false),
                /* token_mint */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* delegatee */ (false, Some(true), false),
                /* event_authority */ (false, Some(false), false),
                /* self_program */ (false, Some(false), false),
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
        let data = TransferFixedCodec.encode(self, limits.max_bytes)?;
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
        TransferFixedCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TransferRecurring`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferRecurring {
    /// The `transfer_data` wire field.
    pub transfer_data: TransferData,
}
/// Bounded codec for [`TransferRecurring`].
#[derive(Clone, Copy, Debug)]
pub struct TransferRecurringCodec;
impl Encoder<TransferRecurring> for TransferRecurringCodec {
    fn encoded_len(&self, value: &TransferRecurring) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(TransferDataCodec.encoded_len(&value.transfer_data)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferRecurring, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_5]).write(&(), writer)?;
        TransferDataCodec.write(&value.transfer_data, writer)?;
        Ok(())
    }
}
impl Decoder<TransferRecurring> for TransferRecurringCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferRecurring, Error> {
        Constant(&[INSTRUCTION_TAG_5]).read(reader)?;
        Ok(TransferRecurring {
            transfer_data: TransferDataCodec.read(reader)?,
        })
    }
}
impl TransferRecurring {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* delegation_pda */ (true, Some(false), false),
                /* subscription_authority */ (false, Some(false), false),
                /* delegator_ata */ (true, Some(false), false),
                /* receiver_ata */ (true, Some(false), false),
                /* token_mint */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* delegatee */ (false, Some(true), false),
                /* event_authority */ (false, Some(false), false),
                /* self_program */ (false, Some(false), false),
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
        let data = TransferRecurringCodec.encode(self, limits.max_bytes)?;
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
        TransferRecurringCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CloseSubscriptionAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseSubscriptionAuthority {}
/// Bounded codec for [`CloseSubscriptionAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct CloseSubscriptionAuthorityCodec;
impl Encoder<CloseSubscriptionAuthority> for CloseSubscriptionAuthorityCodec {
    fn encoded_len(&self, value: &CloseSubscriptionAuthority) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &CloseSubscriptionAuthority,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_6]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CloseSubscriptionAuthority> for CloseSubscriptionAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseSubscriptionAuthority, Error> {
        Constant(&[INSTRUCTION_TAG_6]).read(reader)?;
        let _ = reader;
        Ok(CloseSubscriptionAuthority {})
    }
}
impl CloseSubscriptionAuthority {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 2 || accounts.len() > 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* user */ (true, Some(true), false),
                /* subscription_authority */ (true, Some(false), false),
                /* receiver */ (true, Some(false), false),
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
        let data = CloseSubscriptionAuthorityCodec.encode(self, limits.max_bytes)?;
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
        CloseSubscriptionAuthorityCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreatePlan`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreatePlan {
    /// The `plan_data` wire field.
    pub plan_data: PlanData,
}
/// Bounded codec for [`CreatePlan`].
#[derive(Clone, Copy, Debug)]
pub struct CreatePlanCodec;
impl Encoder<CreatePlan> for CreatePlanCodec {
    fn encoded_len(&self, value: &CreatePlan) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PlanDataCodec.encoded_len(&value.plan_data)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreatePlan, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_7]).write(&(), writer)?;
        PlanDataCodec.write(&value.plan_data, writer)?;
        Ok(())
    }
}
impl Decoder<CreatePlan> for CreatePlanCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreatePlan, Error> {
        Constant(&[INSTRUCTION_TAG_7]).read(reader)?;
        Ok(CreatePlan {
            plan_data: PlanDataCodec.read(reader)?,
        })
    }
}
impl CreatePlan {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 5 || accounts.len() > 6 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* merchant */ (true, Some(true), false),
                /* plan_pda */ (true, Some(false), false),
                /* token_mint */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* payer */ (true, Some(true), false),
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
        let data = CreatePlanCodec.encode(self, limits.max_bytes)?;
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
        CreatePlanCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdatePlan`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdatePlan {
    /// The `update_plan_data` wire field.
    pub update_plan_data: UpdatePlanData,
}
/// Bounded codec for [`UpdatePlan`].
#[derive(Clone, Copy, Debug)]
pub struct UpdatePlanCodec;
impl Encoder<UpdatePlan> for UpdatePlanCodec {
    fn encoded_len(&self, value: &UpdatePlan) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(UpdatePlanDataCodec.encoded_len(&value.update_plan_data)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdatePlan, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_8]).write(&(), writer)?;
        UpdatePlanDataCodec.write(&value.update_plan_data, writer)?;
        Ok(())
    }
}
impl Decoder<UpdatePlan> for UpdatePlanCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdatePlan, Error> {
        Constant(&[INSTRUCTION_TAG_8]).read(reader)?;
        Ok(UpdatePlan {
            update_plan_data: UpdatePlanDataCodec.read(reader)?,
        })
    }
}
impl UpdatePlan {
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
                /* owner */ (false, Some(true), false),
                /* plan_pda */ (true, Some(false), false),
                /* event_authority */ (false, Some(false), false),
                /* self_program */ (false, Some(false), false),
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
        let data = UpdatePlanCodec.encode(self, limits.max_bytes)?;
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
        UpdatePlanCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `DeletePlan`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeletePlan {}
/// Bounded codec for [`DeletePlan`].
#[derive(Clone, Copy, Debug)]
pub struct DeletePlanCodec;
impl Encoder<DeletePlan> for DeletePlanCodec {
    fn encoded_len(&self, value: &DeletePlan) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &DeletePlan, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_9]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DeletePlan> for DeletePlanCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DeletePlan, Error> {
        Constant(&[INSTRUCTION_TAG_9]).read(reader)?;
        let _ = reader;
        Ok(DeletePlan {})
    }
}
impl DeletePlan {
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
                /* owner */ (true, Some(true), false),
                /* plan_pda */ (true, Some(false), false),
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
        let data = DeletePlanCodec.encode(self, limits.max_bytes)?;
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
        DeletePlanCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TransferSubscription`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferSubscription {
    /// The `transfer_data` wire field.
    pub transfer_data: TransferData,
}
/// Bounded codec for [`TransferSubscription`].
#[derive(Clone, Copy, Debug)]
pub struct TransferSubscriptionCodec;
impl Encoder<TransferSubscription> for TransferSubscriptionCodec {
    fn encoded_len(&self, value: &TransferSubscription) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(TransferDataCodec.encoded_len(&value.transfer_data)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferSubscription, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_10]).write(&(), writer)?;
        TransferDataCodec.write(&value.transfer_data, writer)?;
        Ok(())
    }
}
impl Decoder<TransferSubscription> for TransferSubscriptionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferSubscription, Error> {
        Constant(&[INSTRUCTION_TAG_10]).read(reader)?;
        Ok(TransferSubscription {
            transfer_data: TransferDataCodec.read(reader)?,
        })
    }
}
impl TransferSubscription {
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
                /* subscription_pda */ (true, Some(false), false),
                /* plan_pda */ (false, Some(false), false),
                /* subscription_authority */ (false, Some(false), false),
                /* delegator_ata */ (true, Some(false), false),
                /* receiver_ata */ (true, Some(false), false),
                /* caller */ (false, Some(true), false),
                /* token_mint */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* event_authority */ (false, Some(false), false),
                /* self_program */ (false, Some(false), false),
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
        let data = TransferSubscriptionCodec.encode(self, limits.max_bytes)?;
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
        TransferSubscriptionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Subscribe`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subscribe {
    /// The `subscribe_data` wire field.
    pub subscribe_data: SubscribeData,
}
/// Bounded codec for [`Subscribe`].
#[derive(Clone, Copy, Debug)]
pub struct SubscribeCodec;
impl Encoder<Subscribe> for SubscribeCodec {
    fn encoded_len(&self, value: &Subscribe) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(SubscribeDataCodec.encoded_len(&value.subscribe_data)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Subscribe, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_11]).write(&(), writer)?;
        SubscribeDataCodec.write(&value.subscribe_data, writer)?;
        Ok(())
    }
}
impl Decoder<Subscribe> for SubscribeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Subscribe, Error> {
        Constant(&[INSTRUCTION_TAG_11]).read(reader)?;
        Ok(Subscribe {
            subscribe_data: SubscribeDataCodec.read(reader)?,
        })
    }
}
impl Subscribe {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 8 || accounts.len() > 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* subscriber */ (true, Some(true), false),
                /* merchant */ (false, Some(false), false),
                /* plan_pda */ (false, Some(false), false),
                /* subscription_pda */ (true, Some(false), false),
                /* subscription_authority_pda */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* event_authority */ (false, Some(false), false),
                /* self_program */ (false, Some(false), false),
                /* payer */ (true, Some(true), false),
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
        let data = SubscribeCodec.encode(self, limits.max_bytes)?;
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
        SubscribeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CancelSubscription`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CancelSubscription {}
/// Bounded codec for [`CancelSubscription`].
#[derive(Clone, Copy, Debug)]
pub struct CancelSubscriptionCodec;
impl Encoder<CancelSubscription> for CancelSubscriptionCodec {
    fn encoded_len(&self, value: &CancelSubscription) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CancelSubscription, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_12]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CancelSubscription> for CancelSubscriptionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CancelSubscription, Error> {
        Constant(&[INSTRUCTION_TAG_12]).read(reader)?;
        let _ = reader;
        Ok(CancelSubscription {})
    }
}
impl CancelSubscription {
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
                /* subscriber */ (false, Some(true), false),
                /* plan_pda */ (false, Some(false), false),
                /* subscription_pda */ (true, Some(false), false),
                /* event_authority */ (false, Some(false), false),
                /* self_program */ (false, Some(false), false),
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
        let data = CancelSubscriptionCodec.encode(self, limits.max_bytes)?;
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
        CancelSubscriptionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ResumeSubscription`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResumeSubscription {
    /// The `resume_data` wire field.
    pub resume_data: ResumeData,
}
/// Bounded codec for [`ResumeSubscription`].
#[derive(Clone, Copy, Debug)]
pub struct ResumeSubscriptionCodec;
impl Encoder<ResumeSubscription> for ResumeSubscriptionCodec {
    fn encoded_len(&self, value: &ResumeSubscription) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(ResumeDataCodec.encoded_len(&value.resume_data)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ResumeSubscription, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_13]).write(&(), writer)?;
        ResumeDataCodec.write(&value.resume_data, writer)?;
        Ok(())
    }
}
impl Decoder<ResumeSubscription> for ResumeSubscriptionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ResumeSubscription, Error> {
        Constant(&[INSTRUCTION_TAG_13]).read(reader)?;
        Ok(ResumeSubscription {
            resume_data: ResumeDataCodec.read(reader)?,
        })
    }
}
impl ResumeSubscription {
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
                /* subscriber */ (false, Some(true), false),
                /* plan_pda */ (false, Some(false), false),
                /* subscription_pda */ (true, Some(false), false),
                /* subscription_authority */ (false, Some(false), false),
                /* event_authority */ (false, Some(false), false),
                /* self_program */ (false, Some(false), false),
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
        let data = ResumeSubscriptionCodec.encode(self, limits.max_bytes)?;
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
        ResumeSubscriptionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RevokeSubscriptionAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevokeSubscriptionAuthority {}
/// Bounded codec for [`RevokeSubscriptionAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct RevokeSubscriptionAuthorityCodec;
impl Encoder<RevokeSubscriptionAuthority> for RevokeSubscriptionAuthorityCodec {
    fn encoded_len(&self, value: &RevokeSubscriptionAuthority) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &RevokeSubscriptionAuthority,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_14]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<RevokeSubscriptionAuthority> for RevokeSubscriptionAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RevokeSubscriptionAuthority, Error> {
        Constant(&[INSTRUCTION_TAG_14]).read(reader)?;
        let _ = reader;
        Ok(RevokeSubscriptionAuthority {})
    }
}
impl RevokeSubscriptionAuthority {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 5 || accounts.len() > 6 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* user */ (true, Some(true), false),
                /* user_ata */ (true, Some(false), false),
                /* token_mint */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* subscription_authority */ (true, Some(false), false),
                /* receiver */ (true, Some(false), false),
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
        let data = RevokeSubscriptionAuthorityCodec.encode(self, limits.max_bytes)?;
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
        RevokeSubscriptionAuthorityCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RevokeAbandonedDelegation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevokeAbandonedDelegation {}
/// Bounded codec for [`RevokeAbandonedDelegation`].
#[derive(Clone, Copy, Debug)]
pub struct RevokeAbandonedDelegationCodec;
impl Encoder<RevokeAbandonedDelegation> for RevokeAbandonedDelegationCodec {
    fn encoded_len(&self, value: &RevokeAbandonedDelegation) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &RevokeAbandonedDelegation,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_15]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<RevokeAbandonedDelegation> for RevokeAbandonedDelegationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RevokeAbandonedDelegation, Error> {
        Constant(&[INSTRUCTION_TAG_15]).read(reader)?;
        let _ = reader;
        Ok(RevokeAbandonedDelegation {})
    }
}
impl RevokeAbandonedDelegation {
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
                /* payer */ (true, Some(true), false),
                /* delegation_account */ (true, Some(false), false),
                /* subscription_authority */ (false, Some(false), false),
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
        let data = RevokeAbandonedDelegationCodec.encode(self, limits.max_bytes)?;
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
        RevokeAbandonedDelegationCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RevokeAbandonedSubscription`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevokeAbandonedSubscription {}
/// Bounded codec for [`RevokeAbandonedSubscription`].
#[derive(Clone, Copy, Debug)]
pub struct RevokeAbandonedSubscriptionCodec;
impl Encoder<RevokeAbandonedSubscription> for RevokeAbandonedSubscriptionCodec {
    fn encoded_len(&self, value: &RevokeAbandonedSubscription) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &RevokeAbandonedSubscription,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_16]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<RevokeAbandonedSubscription> for RevokeAbandonedSubscriptionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RevokeAbandonedSubscription, Error> {
        Constant(&[INSTRUCTION_TAG_16]).read(reader)?;
        let _ = reader;
        Ok(RevokeAbandonedSubscription {})
    }
}
impl RevokeAbandonedSubscription {
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
                /* subscription_account */ (true, Some(false), false),
                /* subscription_authority */ (false, Some(false), false),
                /* plan_pda */ (false, Some(false), false),
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
        let data = RevokeAbandonedSubscriptionCodec.encode(self, limits.max_bytes)?;
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
        RevokeAbandonedSubscriptionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CancelSubscriptionNow`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CancelSubscriptionNow {
    /// The `cancel_subscription_now_data` wire field.
    pub cancel_subscription_now_data: CancelSubscriptionNowData,
}
/// Bounded codec for [`CancelSubscriptionNow`].
#[derive(Clone, Copy, Debug)]
pub struct CancelSubscriptionNowCodec;
impl Encoder<CancelSubscriptionNow> for CancelSubscriptionNowCodec {
    fn encoded_len(&self, value: &CancelSubscriptionNow) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                CancelSubscriptionNowDataCodec.encoded_len(&value.cancel_subscription_now_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CancelSubscriptionNow, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_17]).write(&(), writer)?;
        CancelSubscriptionNowDataCodec.write(&value.cancel_subscription_now_data, writer)?;
        Ok(())
    }
}
impl Decoder<CancelSubscriptionNow> for CancelSubscriptionNowCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CancelSubscriptionNow, Error> {
        Constant(&[INSTRUCTION_TAG_17]).read(reader)?;
        Ok(CancelSubscriptionNow {
            cancel_subscription_now_data: CancelSubscriptionNowDataCodec.read(reader)?,
        })
    }
}
impl CancelSubscriptionNow {
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
                /* subscriber */ (false, Some(true), false),
                /* merchant */ (false, Some(true), false),
                /* plan_pda */ (false, Some(false), false),
                /* subscription_pda */ (true, Some(false), false),
                /* event_authority */ (false, Some(false), false),
                /* self_program */ (false, Some(false), false),
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
        let data = CancelSubscriptionNowCodec.encode(self, limits.max_bytes)?;
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
        CancelSubscriptionNowCodec.decode(&instruction.data, limits)
    }
}
/// Program error codes. Unknown numeric codes remain lossless.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubscriptionsError {
    /// The `NotSigner` program error.
    NotSigner,
    /// The `InvalidAddress` program error.
    InvalidAddress,
    /// The `InvalidEscrowPda` program error.
    InvalidEscrowPda,
    /// The `InvalidSubscriptionAuthorityPda` program error.
    InvalidSubscriptionAuthorityPda,
    /// The `NotSystemProgram` program error.
    NotSystemProgram,
    /// The `InvalidTokenProgram` program error.
    InvalidTokenProgram,
    /// The `InvalidToken2022MintAccountData` program error.
    InvalidToken2022MintAccountData,
    /// The `InvalidToken2022TokenAccountData` program error.
    InvalidToken2022TokenAccountData,
    /// The `InvalidAssociatedTokenAccountDerivedAddress` program error.
    InvalidAssociatedTokenAccountDerivedAddress,
    /// The `InvalidTokenSplMintAccountData` program error.
    InvalidTokenSplMintAccountData,
    /// The `InvalidTokenSplTokenAccountData` program error.
    InvalidTokenSplTokenAccountData,
    /// The `InvalidAccountData` program error.
    InvalidAccountData,
    /// The `InvalidInstructionData` program error.
    InvalidInstructionData,
    /// The `NotEnoughAccountKeys` program error.
    NotEnoughAccountKeys,
    /// The `InvalidInstruction` program error.
    InvalidInstruction,
    /// The `ArithmeticOverflow` program error.
    ArithmeticOverflow,
    /// The `ArithmeticUnderflow` program error.
    ArithmeticUnderflow,
    /// The `InvalidAccountDiscriminator` program error.
    InvalidAccountDiscriminator,
    /// The `MintHasConfidentialTransfer` program error.
    MintHasConfidentialTransfer,
    /// The `MintHasNonTransferable` program error.
    MintHasNonTransferable,
    /// The `MintHasPermanentDelegate` program error.
    MintHasPermanentDelegate,
    /// The `MintHasTransferHook` program error.
    MintHasTransferHook,
    /// The `MintHasTransferFee` program error.
    MintHasTransferFee,
    /// The `MintHasMintCloseAuthority` program error.
    MintHasMintCloseAuthority,
    /// The `MintHasPausable` program error.
    MintHasPausable,
    /// The `MintMismatch` program error.
    MintMismatch,
    /// The `InvalidDelegatePda` program error.
    InvalidDelegatePda,
    /// The `InvalidHeaderData` program error.
    InvalidHeaderData,
    /// The `DelegationExpired` program error.
    DelegationExpired,
    /// The `InvalidAmount` program error.
    InvalidAmount,
    /// The `Unauthorized` program error.
    Unauthorized,
    /// The `AccountNotWritable` program error.
    AccountNotWritable,
    /// The `AtaOwnerMismatch` program error.
    AtaOwnerMismatch,
    /// The `DelegationVersionMismatch` program error.
    DelegationVersionMismatch,
    /// The `MigrationRequired` program error.
    MigrationRequired,
    /// The `DelegationAlreadyExists` program error.
    DelegationAlreadyExists,
    /// The `StaleSubscriptionAuthority` program error.
    StaleSubscriptionAuthority,
    /// The `TransferHookTooManyAccounts` program error.
    TransferHookTooManyAccounts,
    /// The `AmountExceedsLimit` program error.
    AmountExceedsLimit,
    /// The `FixedDelegationExpiryInPast` program error.
    FixedDelegationExpiryInPast,
    /// The `FixedDelegationAmountZero` program error.
    FixedDelegationAmountZero,
    /// The `AmountExceedsPeriodLimit` program error.
    AmountExceedsPeriodLimit,
    /// The `PeriodNotElapsed` program error.
    PeriodNotElapsed,
    /// The `InvalidPeriodLength` program error.
    InvalidPeriodLength,
    /// The `InvalidPayerData` program error.
    InvalidPayerData,
    /// The `RecurringDelegationStartTimeInPast` program error.
    RecurringDelegationStartTimeInPast,
    /// The `RecurringDelegationStartTimeGreaterThanExpiry` program error.
    RecurringDelegationStartTimeGreaterThanExpiry,
    /// The `RecurringDelegationAmountZero` program error.
    RecurringDelegationAmountZero,
    /// The `DelegationNotStarted` program error.
    DelegationNotStarted,
    /// The `RecurringDelegationStartOnLandingRequiresExpiry` program error.
    RecurringDelegationStartOnLandingRequiresExpiry,
    /// The `PlanSunset` program error.
    PlanSunset,
    /// The `PlanExpired` program error.
    PlanExpired,
    /// The `InvalidPlanPda` program error.
    InvalidPlanPda,
    /// The `InvalidSubscriptionPda` program error.
    InvalidSubscriptionPda,
    /// The `NotPlanOwner` program error.
    NotPlanOwner,
    /// The `SubscriptionPlanMismatch` program error.
    SubscriptionPlanMismatch,
    /// The `UnauthorizedDestination` program error.
    UnauthorizedDestination,
    /// The `InvalidNumDestinations` program error.
    InvalidNumDestinations,
    /// The `SubscriptionCancelled` program error.
    SubscriptionCancelled,
    /// The `SubscriptionAlreadyCancelled` program error.
    SubscriptionAlreadyCancelled,
    /// The `SubscriptionNotCancelled` program error.
    SubscriptionNotCancelled,
    /// The `InvalidEndTs` program error.
    InvalidEndTs,
    /// The `InvalidPlanStatus` program error.
    InvalidPlanStatus,
    /// The `PlanImmutableAfterSunset` program error.
    PlanImmutableAfterSunset,
    /// The `SunsetRequiresEndTs` program error.
    SunsetRequiresEndTs,
    /// The `PlanNotExpired` program error.
    PlanNotExpired,
    /// The `PlanClosed` program error.
    PlanClosed,
    /// The `AlreadySubscribed` program error.
    AlreadySubscribed,
    /// The `PlanAlreadyExists` program error.
    PlanAlreadyExists,
    /// The `PlanTermsMismatch` program error.
    PlanTermsMismatch,
    /// The `PlanEndTsCannotExtend` program error.
    PlanEndTsCannotExtend,
    /// The `StaleSubscriptionApproval` program error.
    StaleSubscriptionApproval,
    /// The `StalePlanApproval` program error.
    StalePlanApproval,
    /// The `InvalidEventAuthority` program error.
    InvalidEventAuthority,
    /// The `InvalidEventData` program error.
    InvalidEventData,
    /// The `InvalidEventTag` program error.
    InvalidEventTag,
    /// The `InvalidEventDiscriminator` program error.
    InvalidEventDiscriminator,
    /// The `InvalidSelfProgram` program error.
    InvalidSelfProgram,
    /// An unrecognized program error code.
    Unknown(u32),
}
impl SubscriptionsError {
    /// Decodes known or unknown numeric program error codes.
    pub const fn from_code(code: u32) -> Self {
        match code {
            PROGRAM_ERROR_100 => Self::NotSigner,
            PROGRAM_ERROR_101 => Self::InvalidAddress,
            PROGRAM_ERROR_102 => Self::InvalidEscrowPda,
            PROGRAM_ERROR_103 => Self::InvalidSubscriptionAuthorityPda,
            PROGRAM_ERROR_104 => Self::NotSystemProgram,
            PROGRAM_ERROR_105 => Self::InvalidTokenProgram,
            PROGRAM_ERROR_106 => Self::InvalidToken2022MintAccountData,
            PROGRAM_ERROR_107 => Self::InvalidToken2022TokenAccountData,
            PROGRAM_ERROR_108 => Self::InvalidAssociatedTokenAccountDerivedAddress,
            PROGRAM_ERROR_109 => Self::InvalidTokenSplMintAccountData,
            PROGRAM_ERROR_110 => Self::InvalidTokenSplTokenAccountData,
            PROGRAM_ERROR_111 => Self::InvalidAccountData,
            PROGRAM_ERROR_112 => Self::InvalidInstructionData,
            PROGRAM_ERROR_113 => Self::NotEnoughAccountKeys,
            PROGRAM_ERROR_114 => Self::InvalidInstruction,
            PROGRAM_ERROR_115 => Self::ArithmeticOverflow,
            PROGRAM_ERROR_116 => Self::ArithmeticUnderflow,
            PROGRAM_ERROR_117 => Self::InvalidAccountDiscriminator,
            PROGRAM_ERROR_118 => Self::MintHasConfidentialTransfer,
            PROGRAM_ERROR_119 => Self::MintHasNonTransferable,
            PROGRAM_ERROR_120 => Self::MintHasPermanentDelegate,
            PROGRAM_ERROR_121 => Self::MintHasTransferHook,
            PROGRAM_ERROR_122 => Self::MintHasTransferFee,
            PROGRAM_ERROR_123 => Self::MintHasMintCloseAuthority,
            PROGRAM_ERROR_124 => Self::MintHasPausable,
            PROGRAM_ERROR_125 => Self::MintMismatch,
            PROGRAM_ERROR_126 => Self::InvalidDelegatePda,
            PROGRAM_ERROR_127 => Self::InvalidHeaderData,
            PROGRAM_ERROR_128 => Self::DelegationExpired,
            PROGRAM_ERROR_129 => Self::InvalidAmount,
            PROGRAM_ERROR_130 => Self::Unauthorized,
            PROGRAM_ERROR_131 => Self::AccountNotWritable,
            PROGRAM_ERROR_132 => Self::AtaOwnerMismatch,
            PROGRAM_ERROR_133 => Self::DelegationVersionMismatch,
            PROGRAM_ERROR_134 => Self::MigrationRequired,
            PROGRAM_ERROR_135 => Self::DelegationAlreadyExists,
            PROGRAM_ERROR_136 => Self::StaleSubscriptionAuthority,
            PROGRAM_ERROR_137 => Self::TransferHookTooManyAccounts,
            PROGRAM_ERROR_300 => Self::AmountExceedsLimit,
            PROGRAM_ERROR_301 => Self::FixedDelegationExpiryInPast,
            PROGRAM_ERROR_302 => Self::FixedDelegationAmountZero,
            PROGRAM_ERROR_400 => Self::AmountExceedsPeriodLimit,
            PROGRAM_ERROR_401 => Self::PeriodNotElapsed,
            PROGRAM_ERROR_402 => Self::InvalidPeriodLength,
            PROGRAM_ERROR_403 => Self::InvalidPayerData,
            PROGRAM_ERROR_404 => Self::RecurringDelegationStartTimeInPast,
            PROGRAM_ERROR_405 => Self::RecurringDelegationStartTimeGreaterThanExpiry,
            PROGRAM_ERROR_406 => Self::RecurringDelegationAmountZero,
            PROGRAM_ERROR_407 => Self::DelegationNotStarted,
            PROGRAM_ERROR_408 => Self::RecurringDelegationStartOnLandingRequiresExpiry,
            PROGRAM_ERROR_500 => Self::PlanSunset,
            PROGRAM_ERROR_501 => Self::PlanExpired,
            PROGRAM_ERROR_502 => Self::InvalidPlanPda,
            PROGRAM_ERROR_503 => Self::InvalidSubscriptionPda,
            PROGRAM_ERROR_504 => Self::NotPlanOwner,
            PROGRAM_ERROR_505 => Self::SubscriptionPlanMismatch,
            PROGRAM_ERROR_506 => Self::UnauthorizedDestination,
            PROGRAM_ERROR_507 => Self::InvalidNumDestinations,
            PROGRAM_ERROR_508 => Self::SubscriptionCancelled,
            PROGRAM_ERROR_509 => Self::SubscriptionAlreadyCancelled,
            PROGRAM_ERROR_510 => Self::SubscriptionNotCancelled,
            PROGRAM_ERROR_511 => Self::InvalidEndTs,
            PROGRAM_ERROR_512 => Self::InvalidPlanStatus,
            PROGRAM_ERROR_513 => Self::PlanImmutableAfterSunset,
            PROGRAM_ERROR_514 => Self::SunsetRequiresEndTs,
            PROGRAM_ERROR_515 => Self::PlanNotExpired,
            PROGRAM_ERROR_516 => Self::PlanClosed,
            PROGRAM_ERROR_517 => Self::AlreadySubscribed,
            PROGRAM_ERROR_518 => Self::PlanAlreadyExists,
            PROGRAM_ERROR_519 => Self::PlanTermsMismatch,
            PROGRAM_ERROR_520 => Self::PlanEndTsCannotExtend,
            PROGRAM_ERROR_521 => Self::StaleSubscriptionApproval,
            PROGRAM_ERROR_522 => Self::StalePlanApproval,
            PROGRAM_ERROR_600 => Self::InvalidEventAuthority,
            PROGRAM_ERROR_601 => Self::InvalidEventData,
            PROGRAM_ERROR_602 => Self::InvalidEventTag,
            PROGRAM_ERROR_603 => Self::InvalidEventDiscriminator,
            PROGRAM_ERROR_604 => Self::InvalidSelfProgram,
            code => Self::Unknown(code),
        }
    }
    /// Returns the exact numeric code.
    pub const fn code(self) -> u32 {
        match self {
            Self::NotSigner => PROGRAM_ERROR_100,
            Self::InvalidAddress => PROGRAM_ERROR_101,
            Self::InvalidEscrowPda => PROGRAM_ERROR_102,
            Self::InvalidSubscriptionAuthorityPda => PROGRAM_ERROR_103,
            Self::NotSystemProgram => PROGRAM_ERROR_104,
            Self::InvalidTokenProgram => PROGRAM_ERROR_105,
            Self::InvalidToken2022MintAccountData => PROGRAM_ERROR_106,
            Self::InvalidToken2022TokenAccountData => PROGRAM_ERROR_107,
            Self::InvalidAssociatedTokenAccountDerivedAddress => PROGRAM_ERROR_108,
            Self::InvalidTokenSplMintAccountData => PROGRAM_ERROR_109,
            Self::InvalidTokenSplTokenAccountData => PROGRAM_ERROR_110,
            Self::InvalidAccountData => PROGRAM_ERROR_111,
            Self::InvalidInstructionData => PROGRAM_ERROR_112,
            Self::NotEnoughAccountKeys => PROGRAM_ERROR_113,
            Self::InvalidInstruction => PROGRAM_ERROR_114,
            Self::ArithmeticOverflow => PROGRAM_ERROR_115,
            Self::ArithmeticUnderflow => PROGRAM_ERROR_116,
            Self::InvalidAccountDiscriminator => PROGRAM_ERROR_117,
            Self::MintHasConfidentialTransfer => PROGRAM_ERROR_118,
            Self::MintHasNonTransferable => PROGRAM_ERROR_119,
            Self::MintHasPermanentDelegate => PROGRAM_ERROR_120,
            Self::MintHasTransferHook => PROGRAM_ERROR_121,
            Self::MintHasTransferFee => PROGRAM_ERROR_122,
            Self::MintHasMintCloseAuthority => PROGRAM_ERROR_123,
            Self::MintHasPausable => PROGRAM_ERROR_124,
            Self::MintMismatch => PROGRAM_ERROR_125,
            Self::InvalidDelegatePda => PROGRAM_ERROR_126,
            Self::InvalidHeaderData => PROGRAM_ERROR_127,
            Self::DelegationExpired => PROGRAM_ERROR_128,
            Self::InvalidAmount => PROGRAM_ERROR_129,
            Self::Unauthorized => PROGRAM_ERROR_130,
            Self::AccountNotWritable => PROGRAM_ERROR_131,
            Self::AtaOwnerMismatch => PROGRAM_ERROR_132,
            Self::DelegationVersionMismatch => PROGRAM_ERROR_133,
            Self::MigrationRequired => PROGRAM_ERROR_134,
            Self::DelegationAlreadyExists => PROGRAM_ERROR_135,
            Self::StaleSubscriptionAuthority => PROGRAM_ERROR_136,
            Self::TransferHookTooManyAccounts => PROGRAM_ERROR_137,
            Self::AmountExceedsLimit => PROGRAM_ERROR_300,
            Self::FixedDelegationExpiryInPast => PROGRAM_ERROR_301,
            Self::FixedDelegationAmountZero => PROGRAM_ERROR_302,
            Self::AmountExceedsPeriodLimit => PROGRAM_ERROR_400,
            Self::PeriodNotElapsed => PROGRAM_ERROR_401,
            Self::InvalidPeriodLength => PROGRAM_ERROR_402,
            Self::InvalidPayerData => PROGRAM_ERROR_403,
            Self::RecurringDelegationStartTimeInPast => PROGRAM_ERROR_404,
            Self::RecurringDelegationStartTimeGreaterThanExpiry => PROGRAM_ERROR_405,
            Self::RecurringDelegationAmountZero => PROGRAM_ERROR_406,
            Self::DelegationNotStarted => PROGRAM_ERROR_407,
            Self::RecurringDelegationStartOnLandingRequiresExpiry => PROGRAM_ERROR_408,
            Self::PlanSunset => PROGRAM_ERROR_500,
            Self::PlanExpired => PROGRAM_ERROR_501,
            Self::InvalidPlanPda => PROGRAM_ERROR_502,
            Self::InvalidSubscriptionPda => PROGRAM_ERROR_503,
            Self::NotPlanOwner => PROGRAM_ERROR_504,
            Self::SubscriptionPlanMismatch => PROGRAM_ERROR_505,
            Self::UnauthorizedDestination => PROGRAM_ERROR_506,
            Self::InvalidNumDestinations => PROGRAM_ERROR_507,
            Self::SubscriptionCancelled => PROGRAM_ERROR_508,
            Self::SubscriptionAlreadyCancelled => PROGRAM_ERROR_509,
            Self::SubscriptionNotCancelled => PROGRAM_ERROR_510,
            Self::InvalidEndTs => PROGRAM_ERROR_511,
            Self::InvalidPlanStatus => PROGRAM_ERROR_512,
            Self::PlanImmutableAfterSunset => PROGRAM_ERROR_513,
            Self::SunsetRequiresEndTs => PROGRAM_ERROR_514,
            Self::PlanNotExpired => PROGRAM_ERROR_515,
            Self::PlanClosed => PROGRAM_ERROR_516,
            Self::AlreadySubscribed => PROGRAM_ERROR_517,
            Self::PlanAlreadyExists => PROGRAM_ERROR_518,
            Self::PlanTermsMismatch => PROGRAM_ERROR_519,
            Self::PlanEndTsCannotExtend => PROGRAM_ERROR_520,
            Self::StaleSubscriptionApproval => PROGRAM_ERROR_521,
            Self::StalePlanApproval => PROGRAM_ERROR_522,
            Self::InvalidEventAuthority => PROGRAM_ERROR_600,
            Self::InvalidEventData => PROGRAM_ERROR_601,
            Self::InvalidEventTag => PROGRAM_ERROR_602,
            Self::InvalidEventDiscriminator => PROGRAM_ERROR_603,
            Self::InvalidSelfProgram => PROGRAM_ERROR_604,
            Self::Unknown(code) => code,
        }
    }
    /// Returns the source message for known errors.
    pub const fn message(self) -> Option<&'static str> {
        match self {
            Self::NotSigner => Some("Account must be a signer"),
            Self::InvalidAddress => Some("Invalid account address"),
            Self::InvalidEscrowPda => Some("Invalid escrow PDA derivation"),
            Self::InvalidSubscriptionAuthorityPda => {
                Some("Invalid subscription-authority PDA derivation")
            }
            Self::NotSystemProgram => Some("Expected system program"),
            Self::InvalidTokenProgram => Some("Token Program does not match other accounts"),
            Self::InvalidToken2022MintAccountData => Some("Invalid Token-2022 mint account data"),
            Self::InvalidToken2022TokenAccountData => Some("Invalid Token-2022 token account data"),
            Self::InvalidAssociatedTokenAccountDerivedAddress => {
                Some("Invalid associated token account address")
            }
            Self::InvalidTokenSplMintAccountData => Some("Invalid SPL Token mint account data"),
            Self::InvalidTokenSplTokenAccountData => Some("Invalid SPL Token account data"),
            Self::InvalidAccountData => Some("Invalid account data"),
            Self::InvalidInstructionData => Some("Invalid instruction data"),
            Self::NotEnoughAccountKeys => Some("Not enough account keys provided"),
            Self::InvalidInstruction => Some("Invalid instruction"),
            Self::ArithmeticOverflow => Some("Arithmetic Overflow"),
            Self::ArithmeticUnderflow => Some("Arithmetic Underflow"),
            Self::InvalidAccountDiscriminator => Some("Invalid account discriminator"),
            Self::MintHasConfidentialTransfer => Some("Mint has ConfidentialTransfer extension"),
            Self::MintHasNonTransferable => Some("Mint has NonTransferable extension"),
            Self::MintHasPermanentDelegate => Some("Mint has PermanentDelegate extension"),
            Self::MintHasTransferHook => Some("Mint has TransferHook extension"),
            Self::MintHasTransferFee => Some("Mint has TransferFee extension"),
            Self::MintHasMintCloseAuthority => Some("Mint has MintCloseAuthority extension"),
            Self::MintHasPausable => Some("Mint has Pausable extension"),
            Self::MintMismatch => Some("Token mint mismatch"),
            Self::InvalidDelegatePda => Some("Invalid delegation PDA derivation"),
            Self::InvalidHeaderData => Some("Invalid header data"),
            Self::DelegationExpired => Some("Delegation has expired"),
            Self::InvalidAmount => Some("Invalid amount specified"),
            Self::Unauthorized => Some("Caller not authorized for this action"),
            Self::AccountNotWritable => Some("Account must be writable"),
            Self::AtaOwnerMismatch => Some("Token account owner does not match expected"),
            Self::DelegationVersionMismatch => Some("Delegation header version is not compatible"),
            Self::MigrationRequired => Some("Account requires explicit migration"),
            Self::DelegationAlreadyExists => Some("Delegation account already exists"),
            Self::StaleSubscriptionAuthority => {
                Some("Delegation init_id does not match current SubscriptionAuthority")
            }
            Self::TransferHookTooManyAccounts => Some("Too many transfer hook accounts provided"),
            Self::AmountExceedsLimit => Some("Transfer amount exceeds delegation limit"),
            Self::FixedDelegationExpiryInPast => {
                Some("Expiry time specified is less than current time")
            }
            Self::FixedDelegationAmountZero => Some("zero amount specified"),
            Self::AmountExceedsPeriodLimit => Some("Transfer amount exceeds period limit"),
            Self::PeriodNotElapsed => Some("Period has not elapsed yet"),
            Self::InvalidPeriodLength => Some("Invalid Period length"),
            Self::InvalidPayerData => Some("Payer provided does not match delegation"),
            Self::RecurringDelegationStartTimeInPast => Some("Past start time specified"),
            Self::RecurringDelegationStartTimeGreaterThanExpiry => {
                Some("start time specified is greater than expiry")
            }
            Self::RecurringDelegationAmountZero => Some("zero amount specified"),
            Self::DelegationNotStarted => Some("Delegation period has not started yet"),
            Self::RecurringDelegationStartOnLandingRequiresExpiry => {
                Some("start_ts of 0 (start on landing) requires a non-zero expiry")
            }
            Self::PlanSunset => Some("Plan is in sunset status"),
            Self::PlanExpired => Some("Plan has expired"),
            Self::InvalidPlanPda => Some("Invalid Plan PDA derivation"),
            Self::InvalidSubscriptionPda => Some("Invalid subscription PDA derivation"),
            Self::NotPlanOwner => Some("Caller is not the plan owner"),
            Self::SubscriptionPlanMismatch => Some("Subscription does not belong to this plan"),
            Self::UnauthorizedDestination => Some("Destination not in plan whitelist"),
            Self::InvalidNumDestinations => Some("No valid destinations provided"),
            Self::SubscriptionCancelled => Some("Subscription cancelled and past valid period"),
            Self::SubscriptionAlreadyCancelled => Some("Subscription already cancelled"),
            Self::SubscriptionNotCancelled => Some("Subscription is not cancelled"),
            Self::InvalidEndTs => Some("End timestamp must be zero or in the future"),
            Self::InvalidPlanStatus => Some("Invalid plan status value"),
            Self::PlanImmutableAfterSunset => Some("Plan cannot be updated after sunset"),
            Self::SunsetRequiresEndTs => Some("Sunset requires a non-zero end timestamp"),
            Self::PlanNotExpired => Some("Plan must be expired to delete"),
            Self::PlanClosed => Some("Plan account has been closed"),
            Self::AlreadySubscribed => Some("Already subscribed to this plan"),
            Self::PlanAlreadyExists => Some("Plan account already exists"),
            Self::PlanTermsMismatch => {
                Some("Subscription plan terms do not match the current plan")
            }
            Self::PlanEndTsCannotExtend => {
                Some("A finite plan end timestamp can only be shortened, not removed or extended")
            }
            Self::StaleSubscriptionApproval => {
                Some("Subscription approval does not match the current subscription")
            }
            Self::StalePlanApproval => {
                Some("Plan update approval does not match the current plan state")
            }
            Self::InvalidEventAuthority => Some("Invalid event authority PDA"),
            Self::InvalidEventData => Some("Invalid event data"),
            Self::InvalidEventTag => Some("Invalid event tag prefix"),
            Self::InvalidEventDiscriminator => Some("Unknown event discriminator"),
            Self::InvalidSelfProgram => Some("Self program account does not match this program"),
            Self::Unknown(_) => None,
        }
    }
}
/// All selected instruction data variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionsInstructionData {
    /// Data for tag INSTRUCTION_TAG_0.
    InitSubscriptionAuthority(InitSubscriptionAuthority),
    /// Data for tag INSTRUCTION_TAG_1.
    CreateFixedDelegation(CreateFixedDelegation),
    /// Data for tag INSTRUCTION_TAG_2.
    CreateRecurringDelegation(CreateRecurringDelegation),
    /// Data for tag INSTRUCTION_TAG_3.
    RevokeDelegation(RevokeDelegation),
    /// Data for tag INSTRUCTION_TAG_4.
    TransferFixed(TransferFixed),
    /// Data for tag INSTRUCTION_TAG_5.
    TransferRecurring(TransferRecurring),
    /// Data for tag INSTRUCTION_TAG_6.
    CloseSubscriptionAuthority(CloseSubscriptionAuthority),
    /// Data for tag INSTRUCTION_TAG_7.
    CreatePlan(CreatePlan),
    /// Data for tag INSTRUCTION_TAG_8.
    UpdatePlan(UpdatePlan),
    /// Data for tag INSTRUCTION_TAG_9.
    DeletePlan(DeletePlan),
    /// Data for tag INSTRUCTION_TAG_10.
    TransferSubscription(TransferSubscription),
    /// Data for tag INSTRUCTION_TAG_11.
    Subscribe(Subscribe),
    /// Data for tag INSTRUCTION_TAG_12.
    CancelSubscription(CancelSubscription),
    /// Data for tag INSTRUCTION_TAG_13.
    ResumeSubscription(ResumeSubscription),
    /// Data for tag INSTRUCTION_TAG_14.
    RevokeSubscriptionAuthority(RevokeSubscriptionAuthority),
    /// Data for tag INSTRUCTION_TAG_15.
    RevokeAbandonedDelegation(RevokeAbandonedDelegation),
    /// Data for tag INSTRUCTION_TAG_16.
    RevokeAbandonedSubscription(RevokeAbandonedSubscription),
    /// Data for tag INSTRUCTION_TAG_17.
    CancelSubscriptionNow(CancelSubscriptionNow),
}
/// Strict bounded instruction dispatch codec.
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionsInstructionDataCodec;
impl Encoder<SubscriptionsInstructionData> for SubscriptionsInstructionDataCodec {
    fn encoded_len(&self, value: &SubscriptionsInstructionData) -> Result<usize, Error> {
        match value {
            SubscriptionsInstructionData::InitSubscriptionAuthority(value) => {
                InitSubscriptionAuthorityCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::CreateFixedDelegation(value) => {
                CreateFixedDelegationCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::CreateRecurringDelegation(value) => {
                CreateRecurringDelegationCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::RevokeDelegation(value) => {
                RevokeDelegationCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::TransferFixed(value) => {
                TransferFixedCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::TransferRecurring(value) => {
                TransferRecurringCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::CloseSubscriptionAuthority(value) => {
                CloseSubscriptionAuthorityCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::CreatePlan(value) => CreatePlanCodec.encoded_len(value),
            SubscriptionsInstructionData::UpdatePlan(value) => UpdatePlanCodec.encoded_len(value),
            SubscriptionsInstructionData::DeletePlan(value) => DeletePlanCodec.encoded_len(value),
            SubscriptionsInstructionData::TransferSubscription(value) => {
                TransferSubscriptionCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::Subscribe(value) => SubscribeCodec.encoded_len(value),
            SubscriptionsInstructionData::CancelSubscription(value) => {
                CancelSubscriptionCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::ResumeSubscription(value) => {
                ResumeSubscriptionCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::RevokeSubscriptionAuthority(value) => {
                RevokeSubscriptionAuthorityCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::RevokeAbandonedDelegation(value) => {
                RevokeAbandonedDelegationCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::RevokeAbandonedSubscription(value) => {
                RevokeAbandonedSubscriptionCodec.encoded_len(value)
            }
            SubscriptionsInstructionData::CancelSubscriptionNow(value) => {
                CancelSubscriptionNowCodec.encoded_len(value)
            }
        }
    }
    fn write(
        &self,
        value: &SubscriptionsInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            SubscriptionsInstructionData::InitSubscriptionAuthority(value) => {
                InitSubscriptionAuthorityCodec.write(value, writer)
            }
            SubscriptionsInstructionData::CreateFixedDelegation(value) => {
                CreateFixedDelegationCodec.write(value, writer)
            }
            SubscriptionsInstructionData::CreateRecurringDelegation(value) => {
                CreateRecurringDelegationCodec.write(value, writer)
            }
            SubscriptionsInstructionData::RevokeDelegation(value) => {
                RevokeDelegationCodec.write(value, writer)
            }
            SubscriptionsInstructionData::TransferFixed(value) => {
                TransferFixedCodec.write(value, writer)
            }
            SubscriptionsInstructionData::TransferRecurring(value) => {
                TransferRecurringCodec.write(value, writer)
            }
            SubscriptionsInstructionData::CloseSubscriptionAuthority(value) => {
                CloseSubscriptionAuthorityCodec.write(value, writer)
            }
            SubscriptionsInstructionData::CreatePlan(value) => CreatePlanCodec.write(value, writer),
            SubscriptionsInstructionData::UpdatePlan(value) => UpdatePlanCodec.write(value, writer),
            SubscriptionsInstructionData::DeletePlan(value) => DeletePlanCodec.write(value, writer),
            SubscriptionsInstructionData::TransferSubscription(value) => {
                TransferSubscriptionCodec.write(value, writer)
            }
            SubscriptionsInstructionData::Subscribe(value) => SubscribeCodec.write(value, writer),
            SubscriptionsInstructionData::CancelSubscription(value) => {
                CancelSubscriptionCodec.write(value, writer)
            }
            SubscriptionsInstructionData::ResumeSubscription(value) => {
                ResumeSubscriptionCodec.write(value, writer)
            }
            SubscriptionsInstructionData::RevokeSubscriptionAuthority(value) => {
                RevokeSubscriptionAuthorityCodec.write(value, writer)
            }
            SubscriptionsInstructionData::RevokeAbandonedDelegation(value) => {
                RevokeAbandonedDelegationCodec.write(value, writer)
            }
            SubscriptionsInstructionData::RevokeAbandonedSubscription(value) => {
                RevokeAbandonedSubscriptionCodec.write(value, writer)
            }
            SubscriptionsInstructionData::CancelSubscriptionNow(value) => {
                CancelSubscriptionNowCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<SubscriptionsInstructionData> for SubscriptionsInstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SubscriptionsInstructionData, Error> {
        let tag = *reader.remaining().first().ok_or(Error::Truncated)?;

        match tag {
            INSTRUCTION_TAG_0 => Ok(SubscriptionsInstructionData::InitSubscriptionAuthority(
                InitSubscriptionAuthorityCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_1 => Ok(SubscriptionsInstructionData::CreateFixedDelegation(
                CreateFixedDelegationCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_2 => Ok(SubscriptionsInstructionData::CreateRecurringDelegation(
                CreateRecurringDelegationCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_3 => Ok(SubscriptionsInstructionData::RevokeDelegation(
                RevokeDelegationCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_4 => Ok(SubscriptionsInstructionData::TransferFixed(
                TransferFixedCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_5 => Ok(SubscriptionsInstructionData::TransferRecurring(
                TransferRecurringCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_6 => Ok(SubscriptionsInstructionData::CloseSubscriptionAuthority(
                CloseSubscriptionAuthorityCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_7 => Ok(SubscriptionsInstructionData::CreatePlan(
                CreatePlanCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_8 => Ok(SubscriptionsInstructionData::UpdatePlan(
                UpdatePlanCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_9 => Ok(SubscriptionsInstructionData::DeletePlan(
                DeletePlanCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_10 => Ok(SubscriptionsInstructionData::TransferSubscription(
                TransferSubscriptionCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_11 => Ok(SubscriptionsInstructionData::Subscribe(
                SubscribeCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_12 => Ok(SubscriptionsInstructionData::CancelSubscription(
                CancelSubscriptionCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_13 => Ok(SubscriptionsInstructionData::ResumeSubscription(
                ResumeSubscriptionCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_14 => Ok(SubscriptionsInstructionData::RevokeSubscriptionAuthority(
                RevokeSubscriptionAuthorityCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_15 => Ok(SubscriptionsInstructionData::RevokeAbandonedDelegation(
                RevokeAbandonedDelegationCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_16 => Ok(SubscriptionsInstructionData::RevokeAbandonedSubscription(
                RevokeAbandonedSubscriptionCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_17 => Ok(SubscriptionsInstructionData::CancelSubscriptionNow(
                CancelSubscriptionNowCodec.read(reader)?,
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
        INSTRUCTION_TAG_8 => Ok(tag),
        INSTRUCTION_TAG_9 => Ok(tag),
        INSTRUCTION_TAG_10 => Ok(tag),
        INSTRUCTION_TAG_11 => Ok(tag),
        INSTRUCTION_TAG_12 => Ok(tag),
        INSTRUCTION_TAG_13 => Ok(tag),
        INSTRUCTION_TAG_14 => Ok(tag),
        INSTRUCTION_TAG_15 => Ok(tag),
        INSTRUCTION_TAG_16 => Ok(tag),
        INSTRUCTION_TAG_17 => Ok(tag),
        _ => Err(Error::InvalidDiscriminator),
    }
}
/// Strictly parses a canonical program invocation.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<SubscriptionsInstructionData, Error> {
    parse_instruction_with_program(PROGRAM, instruction, limits)
}
/// Strictly parses an explicit expected program identity.
pub fn parse_instruction_with_program(
    program: Address,
    instruction: &Instruction,
    limits: Limits,
) -> Result<SubscriptionsInstructionData, Error> {
    match identify_instruction(&instruction.data)? {
        INSTRUCTION_TAG_0 => Ok(SubscriptionsInstructionData::InitSubscriptionAuthority(
            InitSubscriptionAuthority::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_1 => Ok(SubscriptionsInstructionData::CreateFixedDelegation(
            CreateFixedDelegation::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_2 => Ok(SubscriptionsInstructionData::CreateRecurringDelegation(
            CreateRecurringDelegation::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_3 => Ok(SubscriptionsInstructionData::RevokeDelegation(
            RevokeDelegation::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_4 => Ok(SubscriptionsInstructionData::TransferFixed(
            TransferFixed::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_5 => Ok(SubscriptionsInstructionData::TransferRecurring(
            TransferRecurring::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_6 => Ok(SubscriptionsInstructionData::CloseSubscriptionAuthority(
            CloseSubscriptionAuthority::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_7 => Ok(SubscriptionsInstructionData::CreatePlan(
            CreatePlan::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_8 => Ok(SubscriptionsInstructionData::UpdatePlan(
            UpdatePlan::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_9 => Ok(SubscriptionsInstructionData::DeletePlan(
            DeletePlan::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_10 => Ok(SubscriptionsInstructionData::TransferSubscription(
            TransferSubscription::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_11 => Ok(SubscriptionsInstructionData::Subscribe(
            Subscribe::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_12 => Ok(SubscriptionsInstructionData::CancelSubscription(
            CancelSubscription::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_13 => Ok(SubscriptionsInstructionData::ResumeSubscription(
            ResumeSubscription::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_14 => Ok(SubscriptionsInstructionData::RevokeSubscriptionAuthority(
            RevokeSubscriptionAuthority::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_15 => Ok(SubscriptionsInstructionData::RevokeAbandonedDelegation(
            RevokeAbandonedDelegation::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_16 => Ok(SubscriptionsInstructionData::RevokeAbandonedSubscription(
            RevokeAbandonedSubscription::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_17 => Ok(SubscriptionsInstructionData::CancelSubscriptionNow(
            CancelSubscriptionNow::parse_with_program(program, instruction, limits)?,
        )),
        _ => Err(Error::InvalidDiscriminator),
    }
}
use alloc::string::String;
use solana_kit_core::AccountMeta;
use solana_kit_core::AccountRole;
use solana_kit_core::codecs::Affix;
use solana_kit_core::codecs::FixedBytes;
use solana_kit_core::codecs::FixedSize;
use solana_kit_core::codecs::Mapped;
use solana_kit_core::codecs::Utf8Config;
use solana_kit_core::codecs::{Count, OwnedSequence, Sequence};
impl core::hash::Hash for CancelSubscriptionNowData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.expected_current_period_start_ts, state);
    }
}
impl core::fmt::Display for CancelSubscriptionNowData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateFixedDelegationData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.expiry_ts, state);
        core::hash::Hash::hash(&self.expected_subscription_authority_init_id, state);
    }
}
impl core::fmt::Display for CreateFixedDelegationData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PlanTerms {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.period_hours, state);
        core::hash::Hash::hash(&self.created_at, state);
    }
}
impl core::fmt::Display for PlanTerms {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PlanData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plan_id, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.terms, state);
        core::hash::Hash::hash(&self.end_ts, state);
        core::hash::Hash::hash(&self.destinations, state);
        core::hash::Hash::hash(&self.pullers, state);
        core::hash::Hash::hash(&self.metadata_uri, state);
    }
}
impl core::fmt::Display for PlanData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateRecurringDelegationData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.amount_per_period, state);
        core::hash::Hash::hash(&self.period_length_s, state);
        core::hash::Hash::hash(&self.start_ts, state);
        core::hash::Hash::hash(&self.expiry_ts, state);
        core::hash::Hash::hash(&self.expected_subscription_authority_init_id, state);
    }
}
impl core::fmt::Display for CreateRecurringDelegationData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SubscribeData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plan_id, state);
        core::hash::Hash::hash(&self.plan_bump, state);
        core::hash::Hash::hash(&self.expected_mint, state);
        core::hash::Hash::hash(&self.expected_amount, state);
        core::hash::Hash::hash(&self.expected_period_hours, state);
        core::hash::Hash::hash(&self.expected_created_at, state);
        core::hash::Hash::hash(&self.expected_subscription_authority_init_id, state);
    }
}
impl core::fmt::Display for SubscribeData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdatePlanData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.status, state);
        core::hash::Hash::hash(&self.end_ts, state);
        core::hash::Hash::hash(&self.pullers, state);
        core::hash::Hash::hash(&self.metadata_uri, state);
        core::hash::Hash::hash(&self.expected_created_at, state);
        core::hash::Hash::hash(&self.expected_end_ts, state);
        core::hash::Hash::hash(&self.expected_pullers, state);
        core::hash::Hash::hash(&self.expected_metadata_uri, state);
    }
}
impl core::fmt::Display for UpdatePlanData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransferData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.delegator, state);
        core::hash::Hash::hash(&self.mint, state);
    }
}
impl core::fmt::Display for TransferData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ResumeData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.expected_expires_at_ts, state);
    }
}
impl core::fmt::Display for ResumeData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AccountDiscriminator {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::SubscriptionAuthority => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Plan => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::FixedDelegation => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::RecurringDelegation => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::SubscriptionDelegation => {
                core::hash::Hash::hash(&4usize, state);
            }
        }
    }
}
impl core::fmt::Display for AccountDiscriminator {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PlanStatus {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Sunset => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Active => {
                core::hash::Hash::hash(&1usize, state);
            }
        }
    }
}
impl core::fmt::Display for PlanStatus {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Header {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.discriminator, state);
        core::hash::Hash::hash(&self.version, state);
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.delegator, state);
        core::hash::Hash::hash(&self.delegatee, state);
        core::hash::Hash::hash(&self.payer, state);
        core::hash::Hash::hash(&self.init_id, state);
    }
}
impl core::fmt::Display for Header {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for FixedDelegation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.header, state);
        core::hash::Hash::hash(&self.subscription_authority, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.expiry_ts, state);
    }
}
impl core::fmt::Display for FixedDelegation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Plan {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.discriminator, state);
        core::hash::Hash::hash(&self.owner, state);
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.status, state);
        core::hash::Hash::hash(&self.data, state);
    }
}
impl core::fmt::Display for Plan {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RecurringDelegation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.header, state);
        core::hash::Hash::hash(&self.subscription_authority, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.current_period_start_ts, state);
        core::hash::Hash::hash(&self.period_length_s, state);
        core::hash::Hash::hash(&self.expiry_ts, state);
        core::hash::Hash::hash(&self.amount_per_period, state);
        core::hash::Hash::hash(&self.amount_pulled_in_period, state);
    }
}
impl core::fmt::Display for RecurringDelegation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SubscriptionAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.discriminator, state);
        core::hash::Hash::hash(&self.user, state);
        core::hash::Hash::hash(&self.token_mint, state);
        core::hash::Hash::hash(&self.payer, state);
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.init_id, state);
    }
}
impl core::fmt::Display for SubscriptionAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SubscriptionDelegation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.header, state);
        core::hash::Hash::hash(&self.terms, state);
        core::hash::Hash::hash(&self.amount_pulled_in_period, state);
        core::hash::Hash::hash(&self.current_period_start_ts, state);
        core::hash::Hash::hash(&self.expires_at_ts, state);
    }
}
impl core::fmt::Display for SubscriptionDelegation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for EventAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for EventAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for FixedTransferEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.delegation, state);
        core::hash::Hash::hash(&self.delegator, state);
        core::hash::Hash::hash(&self.delegatee, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.remaining_amount, state);
        core::hash::Hash::hash(&self.receiver, state);
        core::hash::Hash::hash(&self.receiver_token_account, state);
    }
}
impl core::fmt::Display for FixedTransferEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PlanUpdatedEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plan, state);
        core::hash::Hash::hash(&self.owner, state);
        core::hash::Hash::hash(&self.status, state);
        core::hash::Hash::hash(&self.end_ts, state);
        core::hash::Hash::hash(&self.pullers, state);
    }
}
impl core::fmt::Display for PlanUpdatedEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RecurringTransferEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.delegation, state);
        core::hash::Hash::hash(&self.delegator, state);
        core::hash::Hash::hash(&self.delegatee, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.period_start_ts, state);
        core::hash::Hash::hash(&self.period_end_ts, state);
        core::hash::Hash::hash(&self.amount_pulled_in_period, state);
        core::hash::Hash::hash(&self.receiver, state);
        core::hash::Hash::hash(&self.receiver_token_account, state);
    }
}
impl core::fmt::Display for RecurringTransferEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SubscriptionCancelledEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plan, state);
        core::hash::Hash::hash(&self.subscriber, state);
        core::hash::Hash::hash(&self.expires_at_ts, state);
    }
}
impl core::fmt::Display for SubscriptionCancelledEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SubscriptionCreatedEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plan, state);
        core::hash::Hash::hash(&self.subscriber, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.created_ts, state);
        core::hash::Hash::hash(&self.payer, state);
    }
}
impl core::fmt::Display for SubscriptionCreatedEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SubscriptionResumedEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plan, state);
        core::hash::Hash::hash(&self.subscriber, state);
        core::hash::Hash::hash(&self.resumed_ts, state);
    }
}
impl core::fmt::Display for SubscriptionResumedEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SubscriptionTransferEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.subscription, state);
        core::hash::Hash::hash(&self.plan, state);
        core::hash::Hash::hash(&self.delegator, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.period_start_ts, state);
        core::hash::Hash::hash(&self.period_end_ts, state);
        core::hash::Hash::hash(&self.amount_pulled_in_period, state);
        core::hash::Hash::hash(&self.receiver, state);
        core::hash::Hash::hash(&self.receiver_token_account, state);
        core::hash::Hash::hash(&self.puller, state);
    }
}
impl core::fmt::Display for SubscriptionTransferEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for InitSubscriptionAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for InitSubscriptionAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateFixedDelegation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.fixed_delegation, state);
    }
}
impl core::fmt::Display for CreateFixedDelegation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateRecurringDelegation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.recurring_delegation, state);
    }
}
impl core::fmt::Display for CreateRecurringDelegation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RevokeDelegation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for RevokeDelegation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransferFixed {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.transfer_data, state);
    }
}
impl core::fmt::Display for TransferFixed {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransferRecurring {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.transfer_data, state);
    }
}
impl core::fmt::Display for TransferRecurring {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CloseSubscriptionAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CloseSubscriptionAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreatePlan {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.plan_data, state);
    }
}
impl core::fmt::Display for CreatePlan {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdatePlan {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.update_plan_data, state);
    }
}
impl core::fmt::Display for UpdatePlan {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DeletePlan {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for DeletePlan {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransferSubscription {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.transfer_data, state);
    }
}
impl core::fmt::Display for TransferSubscription {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Subscribe {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.subscribe_data, state);
    }
}
impl core::fmt::Display for Subscribe {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CancelSubscription {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CancelSubscription {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ResumeSubscription {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.resume_data, state);
    }
}
impl core::fmt::Display for ResumeSubscription {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RevokeSubscriptionAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for RevokeSubscriptionAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RevokeAbandonedDelegation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for RevokeAbandonedDelegation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RevokeAbandonedSubscription {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for RevokeAbandonedSubscription {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CancelSubscriptionNow {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.cancel_subscription_now_data, state);
    }
}
impl core::fmt::Display for CancelSubscriptionNow {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SubscriptionsInstructionData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::InitSubscriptionAuthority(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateFixedDelegation(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateRecurringDelegation(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RevokeDelegation(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransferFixed(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransferRecurring(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CloseSubscriptionAuthority(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreatePlan(value) => {
                core::hash::Hash::hash(&7usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdatePlan(value) => {
                core::hash::Hash::hash(&8usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DeletePlan(value) => {
                core::hash::Hash::hash(&9usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransferSubscription(value) => {
                core::hash::Hash::hash(&10usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Subscribe(value) => {
                core::hash::Hash::hash(&11usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CancelSubscription(value) => {
                core::hash::Hash::hash(&12usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ResumeSubscription(value) => {
                core::hash::Hash::hash(&13usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RevokeSubscriptionAuthority(value) => {
                core::hash::Hash::hash(&14usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RevokeAbandonedDelegation(value) => {
                core::hash::Hash::hash(&15usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RevokeAbandonedSubscription(value) => {
                core::hash::Hash::hash(&16usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CancelSubscriptionNow(value) => {
                core::hash::Hash::hash(&17usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for SubscriptionsInstructionData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SubscriptionsError {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.code(), state)
    }
}
impl core::fmt::Display for SubscriptionsError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.message() {
            Some(message) => formatter.write_str(message),
            None => write!(formatter, "unknown program error {}", self.code()),
        }
    }
}
impl core::error::Error for SubscriptionsError {}

#[cfg(feature = "pda")]
/// A declared-default derivation or instruction construction failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DefaultError {
    /// Exact deterministic PDA failure.
    Pda(solana_kit_core::crypto::PdaError),
    /// Exact allocation, privilege, wire or caller-budget failure.
    Codec(Error),
}
#[cfg(feature = "pda")]
impl From<solana_kit_core::crypto::PdaError> for DefaultError {
    fn from(error: solana_kit_core::crypto::PdaError) -> Self {
        Self::Pda(error)
    }
}
#[cfg(feature = "pda")]
impl From<Error> for DefaultError {
    fn from(error: Error) -> Self {
        Self::Codec(error)
    }
}
#[cfg(feature = "pda")]
impl core::fmt::Display for DefaultError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Pda(error) => core::fmt::Display::fmt(error, formatter),
            Self::Codec(error) => core::fmt::Display::fmt(error, formatter),
        }
    }
}
#[cfg(feature = "pda")]
impl core::error::Error for DefaultError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Pda(error) => Some(error),
            Self::Codec(error) => Some(error),
        }
    }
}
#[cfg(feature = "pda")]
/// Finds the canonical `fixedDelegation` PDA with the complete pinned seed recipe.
pub fn find_fixed_delegation(
    seed_1: Address,
    seed_2: Address,
    seed_3: Address,
    seed_4: u64,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    find_fixed_delegation_with_program(PROGRAM, seed_1, seed_2, seed_3, seed_4)
}
#[cfg(feature = "pda")]
/// Finds the PDA for an explicit program; integer seeds are complete u64 little-endian.
pub fn find_fixed_delegation_with_program(
    program: Address,
    seed_1: Address,
    seed_2: Address,
    seed_3: Address,
    seed_4: u64,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    solana_kit_core::crypto::find_program_address(
        &[
            &[100, 101, 108, 101, 103, 97, 116, 105, 111, 110],
            seed_1.as_bytes(),
            seed_2.as_bytes(),
            seed_3.as_bytes(),
            &seed_4.to_le_bytes(),
        ],
        &program,
    )
}
#[cfg(feature = "pda")]
/// Finds the canonical `plan` PDA with the complete pinned seed recipe.
pub fn find_plan(
    seed_1: Address,
    seed_2: u64,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    find_plan_with_program(PROGRAM, seed_1, seed_2)
}
#[cfg(feature = "pda")]
/// Finds the PDA for an explicit program; integer seeds are complete u64 little-endian.
pub fn find_plan_with_program(
    program: Address,
    seed_1: Address,
    seed_2: u64,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    solana_kit_core::crypto::find_program_address(
        &[
            &[112, 108, 97, 110],
            seed_1.as_bytes(),
            &seed_2.to_le_bytes(),
        ],
        &program,
    )
}
#[cfg(feature = "pda")]
/// Finds the canonical `recurringDelegation` PDA with the complete pinned seed recipe.
pub fn find_recurring_delegation(
    seed_1: Address,
    seed_2: Address,
    seed_3: Address,
    seed_4: u64,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    find_recurring_delegation_with_program(PROGRAM, seed_1, seed_2, seed_3, seed_4)
}
#[cfg(feature = "pda")]
/// Finds the PDA for an explicit program; integer seeds are complete u64 little-endian.
pub fn find_recurring_delegation_with_program(
    program: Address,
    seed_1: Address,
    seed_2: Address,
    seed_3: Address,
    seed_4: u64,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    solana_kit_core::crypto::find_program_address(
        &[
            &[100, 101, 108, 101, 103, 97, 116, 105, 111, 110],
            seed_1.as_bytes(),
            seed_2.as_bytes(),
            seed_3.as_bytes(),
            &seed_4.to_le_bytes(),
        ],
        &program,
    )
}
#[cfg(feature = "pda")]
/// Finds the canonical `subscriptionAuthority` PDA with the complete pinned seed recipe.
pub fn find_subscription_authority(
    seed_1: Address,
    seed_2: Address,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    find_subscription_authority_with_program(PROGRAM, seed_1, seed_2)
}
#[cfg(feature = "pda")]
/// Finds the PDA for an explicit program; integer seeds are complete u64 little-endian.
pub fn find_subscription_authority_with_program(
    program: Address,
    seed_1: Address,
    seed_2: Address,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    solana_kit_core::crypto::find_program_address(
        &[
            &[
                83, 117, 98, 115, 99, 114, 105, 112, 116, 105, 111, 110, 65, 117, 116, 104, 111,
                114, 105, 116, 121,
            ],
            seed_1.as_bytes(),
            seed_2.as_bytes(),
        ],
        &program,
    )
}
#[cfg(feature = "pda")]
/// Finds the canonical `subscriptionDelegation` PDA with the complete pinned seed recipe.
pub fn find_subscription_delegation(
    seed_1: Address,
    seed_2: Address,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    find_subscription_delegation_with_program(PROGRAM, seed_1, seed_2)
}
#[cfg(feature = "pda")]
/// Finds the PDA for an explicit program; integer seeds are complete u64 little-endian.
pub fn find_subscription_delegation_with_program(
    program: Address,
    seed_1: Address,
    seed_2: Address,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    solana_kit_core::crypto::find_program_address(
        &[
            &[115, 117, 98, 115, 99, 114, 105, 112, 116, 105, 111, 110],
            seed_1.as_bytes(),
            seed_2.as_bytes(),
        ],
        &program,
    )
}
#[cfg(feature = "pda")]
/// Finds the canonical `eventAuthority` PDA with the complete pinned seed recipe.
pub fn find_event_authority()
-> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    find_event_authority_with_program(PROGRAM)
}
#[cfg(feature = "pda")]
/// Finds the PDA for an explicit program; integer seeds are complete u64 little-endian.
pub fn find_event_authority_with_program(
    program: Address,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    solana_kit_core::crypto::find_program_address(
        &[&[
            101, 118, 101, 110, 116, 95, 97, 117, 116, 104, 111, 114, 105, 116, 121,
        ]],
        &program,
    )
}
#[cfg(feature = "pda")]
/// Ordered `InitSubscriptionAuthority` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct InitSubscriptionAuthorityAccounts {
    /// The `owner` role, with explicit override support.
    pub owner: Address,
    /// The `subscription_authority` role, with explicit override support.
    pub subscription_authority: Option<Address>,
    /// The `token_mint` role, with explicit override support.
    pub token_mint: Address,
    /// The `user_ata` role, with explicit override support.
    pub user_ata: Address,
    /// The `system_program` role, with explicit override support.
    pub system_program: Option<Address>,
    /// The `token_program` role, with explicit override support.
    pub token_program: Address,
    /// The `payer` role, with explicit override support.
    pub payer: Option<Address>,
}
#[cfg(feature = "pda")]
impl InitSubscriptionAuthorityAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.owner;

        let _role_2 = self.token_mint;

        let _role_3 = self.user_ata;

        let _role_4 = match self.system_program {
            Some(address) => address,
            None => Address::from_bytes([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]),
        };

        let _role_5 = self.token_program;

        let _role_6 = self.payer;

        let _role_1 = match self.subscription_authority {
            Some(address) => address,
            None => find_subscription_authority_with_program(program, _role_0, _role_2)?.address(),
        };

        const ACCOUNT_COUNT: usize = 7;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        let mut _omitted = false;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_5,
            role: AccountRole::from_privileges(false, false),
        });

        if let Some(_role_6) = _role_6 {
            if _omitted {
                return Err(DefaultError::Codec(Error::InvalidValue));
            }

            accounts.push(AccountMeta {
                address: _role_6,
                role: AccountRole::from_privileges(true, true),
            });
        } else {
            _omitted = true;
        }

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &InitSubscriptionAuthority,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &InitSubscriptionAuthority,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `CreateFixedDelegation` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct CreateFixedDelegationAccounts {
    /// The `delegator` role, with explicit override support.
    pub delegator: Address,
    /// The `subscription_authority` role, with explicit override support.
    pub subscription_authority: Address,
    /// The `delegation_account` role, with explicit override support.
    pub delegation_account: Address,
    /// The `delegatee` role, with explicit override support.
    pub delegatee: Address,
    /// The `system_program` role, with explicit override support.
    pub system_program: Option<Address>,
    /// The `payer` role, with explicit override support.
    pub payer: Option<Address>,
}
#[cfg(feature = "pda")]
impl CreateFixedDelegationAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.delegator;

        let _role_1 = self.subscription_authority;

        let _role_2 = self.delegation_account;

        let _role_3 = self.delegatee;

        let _role_4 = match self.system_program {
            Some(address) => address,
            None => Address::from_bytes([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]),
        };

        let _role_5 = self.payer;

        const ACCOUNT_COUNT: usize = 6;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        let mut _omitted = false;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, false),
        });

        if let Some(_role_5) = _role_5 {
            if _omitted {
                return Err(DefaultError::Codec(Error::InvalidValue));
            }

            accounts.push(AccountMeta {
                address: _role_5,
                role: AccountRole::from_privileges(true, true),
            });
        } else {
            _omitted = true;
        }

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &CreateFixedDelegation,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &CreateFixedDelegation,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `CreateRecurringDelegation` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct CreateRecurringDelegationAccounts {
    /// The `delegator` role, with explicit override support.
    pub delegator: Address,
    /// The `subscription_authority` role, with explicit override support.
    pub subscription_authority: Address,
    /// The `delegation_account` role, with explicit override support.
    pub delegation_account: Address,
    /// The `delegatee` role, with explicit override support.
    pub delegatee: Address,
    /// The `system_program` role, with explicit override support.
    pub system_program: Option<Address>,
    /// The `payer` role, with explicit override support.
    pub payer: Option<Address>,
}
#[cfg(feature = "pda")]
impl CreateRecurringDelegationAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.delegator;

        let _role_1 = self.subscription_authority;

        let _role_2 = self.delegation_account;

        let _role_3 = self.delegatee;

        let _role_4 = match self.system_program {
            Some(address) => address,
            None => Address::from_bytes([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]),
        };

        let _role_5 = self.payer;

        const ACCOUNT_COUNT: usize = 6;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        let mut _omitted = false;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, false),
        });

        if let Some(_role_5) = _role_5 {
            if _omitted {
                return Err(DefaultError::Codec(Error::InvalidValue));
            }

            accounts.push(AccountMeta {
                address: _role_5,
                role: AccountRole::from_privileges(true, true),
            });
        } else {
            _omitted = true;
        }

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &CreateRecurringDelegation,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &CreateRecurringDelegation,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `RevokeDelegation` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct RevokeDelegationAccounts {
    /// The `authority` role, with explicit override support.
    pub authority: Address,
    /// The `delegation_account` role, with explicit override support.
    pub delegation_account: Address,
}
#[cfg(feature = "pda")]
impl RevokeDelegationAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.authority;

        let _role_1 = self.delegation_account;

        const ACCOUNT_COUNT: usize = 2;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, true),
        });

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &RevokeDelegation,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &RevokeDelegation,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `TransferFixed` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct TransferFixedAccounts {
    /// The `delegation_pda` role, with explicit override support.
    pub delegation_pda: Address,
    /// The `subscription_authority` role, with explicit override support.
    pub subscription_authority: Address,
    /// The `delegator_ata` role, with explicit override support.
    pub delegator_ata: Address,
    /// The `receiver_ata` role, with explicit override support.
    pub receiver_ata: Address,
    /// The `token_mint` role, with explicit override support.
    pub token_mint: Address,
    /// The `token_program` role, with explicit override support.
    pub token_program: Address,
    /// The `delegatee` role, with explicit override support.
    pub delegatee: Address,
    /// The `event_authority` role, with explicit override support.
    pub event_authority: Option<Address>,
    /// The `self_program` role, with explicit override support.
    pub self_program: Option<Address>,
}
#[cfg(feature = "pda")]
impl TransferFixedAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.delegation_pda;

        let _role_1 = self.subscription_authority;

        let _role_2 = self.delegator_ata;

        let _role_3 = self.receiver_ata;

        let _role_4 = self.token_mint;

        let _role_5 = self.token_program;

        let _role_6 = self.delegatee;

        let _role_7 = match self.event_authority {
            Some(address) => address,
            None => find_event_authority_with_program(program)?.address(),
        };

        let _role_8 = match self.self_program {
            Some(address) => address,
            None => Address::from_bytes([
                187, 199, 119, 175, 186, 104, 240, 18, 62, 192, 196, 209, 183, 238, 137, 110, 147,
                1, 201, 41, 197, 90, 6, 186, 149, 24, 33, 88, 37, 140, 21, 169,
            ]),
        };

        const ACCOUNT_COUNT: usize = 9;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_5,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_6,
            role: AccountRole::from_privileges(true, false),
        });

        accounts.push(AccountMeta {
            address: _role_7,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_8,
            role: AccountRole::from_privileges(false, false),
        });

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &TransferFixed,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &TransferFixed,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `TransferRecurring` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct TransferRecurringAccounts {
    /// The `delegation_pda` role, with explicit override support.
    pub delegation_pda: Address,
    /// The `subscription_authority` role, with explicit override support.
    pub subscription_authority: Address,
    /// The `delegator_ata` role, with explicit override support.
    pub delegator_ata: Address,
    /// The `receiver_ata` role, with explicit override support.
    pub receiver_ata: Address,
    /// The `token_mint` role, with explicit override support.
    pub token_mint: Address,
    /// The `token_program` role, with explicit override support.
    pub token_program: Address,
    /// The `delegatee` role, with explicit override support.
    pub delegatee: Address,
    /// The `event_authority` role, with explicit override support.
    pub event_authority: Option<Address>,
    /// The `self_program` role, with explicit override support.
    pub self_program: Option<Address>,
}
#[cfg(feature = "pda")]
impl TransferRecurringAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.delegation_pda;

        let _role_1 = self.subscription_authority;

        let _role_2 = self.delegator_ata;

        let _role_3 = self.receiver_ata;

        let _role_4 = self.token_mint;

        let _role_5 = self.token_program;

        let _role_6 = self.delegatee;

        let _role_7 = match self.event_authority {
            Some(address) => address,
            None => find_event_authority_with_program(program)?.address(),
        };

        let _role_8 = match self.self_program {
            Some(address) => address,
            None => Address::from_bytes([
                187, 199, 119, 175, 186, 104, 240, 18, 62, 192, 196, 209, 183, 238, 137, 110, 147,
                1, 201, 41, 197, 90, 6, 186, 149, 24, 33, 88, 37, 140, 21, 169,
            ]),
        };

        const ACCOUNT_COUNT: usize = 9;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_5,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_6,
            role: AccountRole::from_privileges(true, false),
        });

        accounts.push(AccountMeta {
            address: _role_7,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_8,
            role: AccountRole::from_privileges(false, false),
        });

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &TransferRecurring,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &TransferRecurring,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `CloseSubscriptionAuthority` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct CloseSubscriptionAuthorityAccounts {
    /// The `user` role, with explicit override support.
    pub user: Address,
    /// The `subscription_authority` role, with explicit override support.
    pub subscription_authority: Address,
    /// The `receiver` role, with explicit override support.
    pub receiver: Option<Address>,
}
#[cfg(feature = "pda")]
impl CloseSubscriptionAuthorityAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.user;

        let _role_1 = self.subscription_authority;

        let _role_2 = self.receiver;

        const ACCOUNT_COUNT: usize = 3;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        let mut _omitted = false;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, true),
        });

        if let Some(_role_2) = _role_2 {
            if _omitted {
                return Err(DefaultError::Codec(Error::InvalidValue));
            }

            accounts.push(AccountMeta {
                address: _role_2,
                role: AccountRole::from_privileges(false, true),
            });
        } else {
            _omitted = true;
        }

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &CloseSubscriptionAuthority,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &CloseSubscriptionAuthority,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `CreatePlan` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct CreatePlanAccounts {
    /// The `merchant` role, with explicit override support.
    pub merchant: Address,
    /// The `plan_pda` role, with explicit override support.
    pub plan_pda: Address,
    /// The `token_mint` role, with explicit override support.
    pub token_mint: Address,
    /// The `system_program` role, with explicit override support.
    pub system_program: Option<Address>,
    /// The `token_program` role, with explicit override support.
    pub token_program: Option<Address>,
    /// The `payer` role, with explicit override support.
    pub payer: Option<Address>,
}
#[cfg(feature = "pda")]
impl CreatePlanAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.merchant;

        let _role_1 = self.plan_pda;

        let _role_2 = self.token_mint;

        let _role_3 = match self.system_program {
            Some(address) => address,
            None => Address::from_bytes([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]),
        };

        let _role_4 = match self.token_program {
            Some(address) => address,
            None => Address::from_bytes([
                6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28,
                180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
            ]),
        };

        let _role_5 = self.payer;

        const ACCOUNT_COUNT: usize = 6;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        let mut _omitted = false;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, false),
        });

        if let Some(_role_5) = _role_5 {
            if _omitted {
                return Err(DefaultError::Codec(Error::InvalidValue));
            }

            accounts.push(AccountMeta {
                address: _role_5,
                role: AccountRole::from_privileges(true, true),
            });
        } else {
            _omitted = true;
        }

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &CreatePlan,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &CreatePlan,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `UpdatePlan` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct UpdatePlanAccounts {
    /// The `owner` role, with explicit override support.
    pub owner: Address,
    /// The `plan_pda` role, with explicit override support.
    pub plan_pda: Address,
    /// The `event_authority` role, with explicit override support.
    pub event_authority: Option<Address>,
    /// The `self_program` role, with explicit override support.
    pub self_program: Option<Address>,
}
#[cfg(feature = "pda")]
impl UpdatePlanAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.owner;

        let _role_1 = self.plan_pda;

        let _role_2 = match self.event_authority {
            Some(address) => address,
            None => find_event_authority_with_program(program)?.address(),
        };

        let _role_3 = match self.self_program {
            Some(address) => address,
            None => Address::from_bytes([
                187, 199, 119, 175, 186, 104, 240, 18, 62, 192, 196, 209, 183, 238, 137, 110, 147,
                1, 201, 41, 197, 90, 6, 186, 149, 24, 33, 88, 37, 140, 21, 169,
            ]),
        };

        const ACCOUNT_COUNT: usize = 4;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, false),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, false),
        });

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &UpdatePlan,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &UpdatePlan,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `DeletePlan` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct DeletePlanAccounts {
    /// The `owner` role, with explicit override support.
    pub owner: Address,
    /// The `plan_pda` role, with explicit override support.
    pub plan_pda: Address,
}
#[cfg(feature = "pda")]
impl DeletePlanAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.owner;

        let _role_1 = self.plan_pda;

        const ACCOUNT_COUNT: usize = 2;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, true),
        });

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &DeletePlan,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &DeletePlan,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `TransferSubscription` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct TransferSubscriptionAccounts {
    /// The `subscription_pda` role, with explicit override support.
    pub subscription_pda: Address,
    /// The `plan_pda` role, with explicit override support.
    pub plan_pda: Address,
    /// The `subscription_authority` role, with explicit override support.
    pub subscription_authority: Address,
    /// The `delegator_ata` role, with explicit override support.
    pub delegator_ata: Address,
    /// The `receiver_ata` role, with explicit override support.
    pub receiver_ata: Address,
    /// The `caller` role, with explicit override support.
    pub caller: Address,
    /// The `token_mint` role, with explicit override support.
    pub token_mint: Address,
    /// The `token_program` role, with explicit override support.
    pub token_program: Address,
    /// The `event_authority` role, with explicit override support.
    pub event_authority: Option<Address>,
    /// The `self_program` role, with explicit override support.
    pub self_program: Option<Address>,
}
#[cfg(feature = "pda")]
impl TransferSubscriptionAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.subscription_pda;

        let _role_1 = self.plan_pda;

        let _role_2 = self.subscription_authority;

        let _role_3 = self.delegator_ata;

        let _role_4 = self.receiver_ata;

        let _role_5 = self.caller;

        let _role_6 = self.token_mint;

        let _role_7 = self.token_program;

        let _role_8 = match self.event_authority {
            Some(address) => address,
            None => find_event_authority_with_program(program)?.address(),
        };

        let _role_9 = match self.self_program {
            Some(address) => address,
            None => Address::from_bytes([
                187, 199, 119, 175, 186, 104, 240, 18, 62, 192, 196, 209, 183, 238, 137, 110, 147,
                1, 201, 41, 197, 90, 6, 186, 149, 24, 33, 88, 37, 140, 21, 169,
            ]),
        };

        const ACCOUNT_COUNT: usize = 10;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_5,
            role: AccountRole::from_privileges(true, false),
        });

        accounts.push(AccountMeta {
            address: _role_6,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_7,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_8,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_9,
            role: AccountRole::from_privileges(false, false),
        });

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &TransferSubscription,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &TransferSubscription,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `Subscribe` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct SubscribeAccounts {
    /// The `subscriber` role, with explicit override support.
    pub subscriber: Address,
    /// The `merchant` role, with explicit override support.
    pub merchant: Address,
    /// The `plan_pda` role, with explicit override support.
    pub plan_pda: Address,
    /// The `subscription_pda` role, with explicit override support.
    pub subscription_pda: Option<Address>,
    /// The `subscription_authority_pda` role, with explicit override support.
    pub subscription_authority_pda: Address,
    /// The `system_program` role, with explicit override support.
    pub system_program: Option<Address>,
    /// The `event_authority` role, with explicit override support.
    pub event_authority: Option<Address>,
    /// The `self_program` role, with explicit override support.
    pub self_program: Option<Address>,
    /// The `payer` role, with explicit override support.
    pub payer: Option<Address>,
}
#[cfg(feature = "pda")]
impl SubscribeAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.subscriber;

        let _role_1 = self.merchant;

        let _role_2 = self.plan_pda;

        let _role_3 = match self.subscription_pda {
            Some(address) => address,
            None => find_subscription_delegation_with_program(program, _role_2, _role_0)?.address(),
        };

        let _role_4 = self.subscription_authority_pda;

        let _role_5 = match self.system_program {
            Some(address) => address,
            None => Address::from_bytes([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]),
        };

        let _role_6 = match self.event_authority {
            Some(address) => address,
            None => find_event_authority_with_program(program)?.address(),
        };

        let _role_7 = match self.self_program {
            Some(address) => address,
            None => Address::from_bytes([
                187, 199, 119, 175, 186, 104, 240, 18, 62, 192, 196, 209, 183, 238, 137, 110, 147,
                1, 201, 41, 197, 90, 6, 186, 149, 24, 33, 88, 37, 140, 21, 169,
            ]),
        };

        let _role_8 = self.payer;

        const ACCOUNT_COUNT: usize = 9;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        let mut _omitted = false;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_5,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_6,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_7,
            role: AccountRole::from_privileges(false, false),
        });

        if let Some(_role_8) = _role_8 {
            if _omitted {
                return Err(DefaultError::Codec(Error::InvalidValue));
            }

            accounts.push(AccountMeta {
                address: _role_8,
                role: AccountRole::from_privileges(true, true),
            });
        } else {
            _omitted = true;
        }

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &Subscribe,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &Subscribe,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `CancelSubscription` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct CancelSubscriptionAccounts {
    /// The `subscriber` role, with explicit override support.
    pub subscriber: Address,
    /// The `plan_pda` role, with explicit override support.
    pub plan_pda: Address,
    /// The `subscription_pda` role, with explicit override support.
    pub subscription_pda: Option<Address>,
    /// The `event_authority` role, with explicit override support.
    pub event_authority: Option<Address>,
    /// The `self_program` role, with explicit override support.
    pub self_program: Option<Address>,
}
#[cfg(feature = "pda")]
impl CancelSubscriptionAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.subscriber;

        let _role_1 = self.plan_pda;

        let _role_2 = match self.subscription_pda {
            Some(address) => address,
            None => find_subscription_delegation_with_program(program, _role_1, _role_0)?.address(),
        };

        let _role_3 = match self.event_authority {
            Some(address) => address,
            None => find_event_authority_with_program(program)?.address(),
        };

        let _role_4 = match self.self_program {
            Some(address) => address,
            None => Address::from_bytes([
                187, 199, 119, 175, 186, 104, 240, 18, 62, 192, 196, 209, 183, 238, 137, 110, 147,
                1, 201, 41, 197, 90, 6, 186, 149, 24, 33, 88, 37, 140, 21, 169,
            ]),
        };

        const ACCOUNT_COUNT: usize = 5;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, false),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, false),
        });

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &CancelSubscription,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &CancelSubscription,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `ResumeSubscription` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct ResumeSubscriptionAccounts {
    /// The `subscriber` role, with explicit override support.
    pub subscriber: Address,
    /// The `plan_pda` role, with explicit override support.
    pub plan_pda: Address,
    /// The `subscription_pda` role, with explicit override support.
    pub subscription_pda: Option<Address>,
    /// The `subscription_authority` role, with explicit override support.
    pub subscription_authority: Address,
    /// The `event_authority` role, with explicit override support.
    pub event_authority: Option<Address>,
    /// The `self_program` role, with explicit override support.
    pub self_program: Option<Address>,
}
#[cfg(feature = "pda")]
impl ResumeSubscriptionAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.subscriber;

        let _role_1 = self.plan_pda;

        let _role_2 = match self.subscription_pda {
            Some(address) => address,
            None => find_subscription_delegation_with_program(program, _role_1, _role_0)?.address(),
        };

        let _role_3 = self.subscription_authority;

        let _role_4 = match self.event_authority {
            Some(address) => address,
            None => find_event_authority_with_program(program)?.address(),
        };

        let _role_5 = match self.self_program {
            Some(address) => address,
            None => Address::from_bytes([
                187, 199, 119, 175, 186, 104, 240, 18, 62, 192, 196, 209, 183, 238, 137, 110, 147,
                1, 201, 41, 197, 90, 6, 186, 149, 24, 33, 88, 37, 140, 21, 169,
            ]),
        };

        const ACCOUNT_COUNT: usize = 6;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, false),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_5,
            role: AccountRole::from_privileges(false, false),
        });

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &ResumeSubscription,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &ResumeSubscription,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `RevokeSubscriptionAuthority` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct RevokeSubscriptionAuthorityAccounts {
    /// The `user` role, with explicit override support.
    pub user: Address,
    /// The `user_ata` role, with explicit override support.
    pub user_ata: Address,
    /// The `token_mint` role, with explicit override support.
    pub token_mint: Address,
    /// The `token_program` role, with explicit override support.
    pub token_program: Address,
    /// The `subscription_authority` role, with explicit override support.
    pub subscription_authority: Option<Address>,
    /// The `receiver` role, with explicit override support.
    pub receiver: Option<Address>,
}
#[cfg(feature = "pda")]
impl RevokeSubscriptionAuthorityAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.user;

        let _role_1 = self.user_ata;

        let _role_2 = self.token_mint;

        let _role_3 = self.token_program;

        let _role_4 = match self.subscription_authority {
            Some(address) => address,
            None => find_subscription_authority_with_program(program, _role_0, _role_2)?.address(),
        };

        let _role_5 = self.receiver;

        const ACCOUNT_COUNT: usize = 6;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        let mut _omitted = false;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, true),
        });

        if let Some(_role_5) = _role_5 {
            if _omitted {
                return Err(DefaultError::Codec(Error::InvalidValue));
            }

            accounts.push(AccountMeta {
                address: _role_5,
                role: AccountRole::from_privileges(false, true),
            });
        } else {
            _omitted = true;
        }

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &RevokeSubscriptionAuthority,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &RevokeSubscriptionAuthority,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `RevokeAbandonedDelegation` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct RevokeAbandonedDelegationAccounts {
    /// The `payer` role, with explicit override support.
    pub payer: Address,
    /// The `delegation_account` role, with explicit override support.
    pub delegation_account: Address,
    /// The `subscription_authority` role, with explicit override support.
    pub subscription_authority: Address,
}
#[cfg(feature = "pda")]
impl RevokeAbandonedDelegationAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.payer;

        let _role_1 = self.delegation_account;

        let _role_2 = self.subscription_authority;

        const ACCOUNT_COUNT: usize = 3;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, false),
        });

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &RevokeAbandonedDelegation,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &RevokeAbandonedDelegation,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `RevokeAbandonedSubscription` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct RevokeAbandonedSubscriptionAccounts {
    /// The `payer` role, with explicit override support.
    pub payer: Address,
    /// The `subscription_account` role, with explicit override support.
    pub subscription_account: Address,
    /// The `subscription_authority` role, with explicit override support.
    pub subscription_authority: Address,
    /// The `plan_pda` role, with explicit override support.
    pub plan_pda: Address,
}
#[cfg(feature = "pda")]
impl RevokeAbandonedSubscriptionAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.payer;

        let _role_1 = self.subscription_account;

        let _role_2 = self.subscription_authority;

        let _role_3 = self.plan_pda;

        const ACCOUNT_COUNT: usize = 4;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, true),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, false),
        });

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &RevokeAbandonedSubscription,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &RevokeAbandonedSubscription,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
#[cfg(feature = "pda")]
/// Ordered `CancelSubscriptionNow` inputs; None selects a default, a program sentinel, or an omitted suffix.
#[derive(Clone, Debug)]
pub struct CancelSubscriptionNowAccounts {
    /// The `subscriber` role, with explicit override support.
    pub subscriber: Address,
    /// The `merchant` role, with explicit override support.
    pub merchant: Address,
    /// The `plan_pda` role, with explicit override support.
    pub plan_pda: Address,
    /// The `subscription_pda` role, with explicit override support.
    pub subscription_pda: Option<Address>,
    /// The `event_authority` role, with explicit override support.
    pub event_authority: Option<Address>,
    /// The `self_program` role, with explicit override support.
    pub self_program: Option<Address>,
}
#[cfg(feature = "pda")]
impl CancelSubscriptionNowAccounts {
    /// Resolves the complete checked default graph, preserving explicit overrides.
    pub fn resolve(&self, program: Address) -> Result<Vec<AccountMeta>, DefaultError> {
        let _ = program;

        let _role_0 = self.subscriber;

        let _role_1 = self.merchant;

        let _role_2 = self.plan_pda;

        let _role_3 = match self.subscription_pda {
            Some(address) => address,
            None => find_subscription_delegation_with_program(program, _role_2, _role_0)?.address(),
        };

        let _role_4 = match self.event_authority {
            Some(address) => address,
            None => find_event_authority_with_program(program)?.address(),
        };

        let _role_5 = match self.self_program {
            Some(address) => address,
            None => Address::from_bytes([
                187, 199, 119, 175, 186, 104, 240, 18, 62, 192, 196, 209, 183, 238, 137, 110, 147,
                1, 201, 41, 197, 90, 6, 186, 149, 24, 33, 88, 37, 140, 21, 169,
            ]),
        };

        const ACCOUNT_COUNT: usize = 6;

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(ACCOUNT_COUNT)
            .map_err(|_| DefaultError::Codec(Error::AllocationFailed))?;

        accounts.push(AccountMeta {
            address: _role_0,
            role: AccountRole::from_privileges(true, false),
        });

        accounts.push(AccountMeta {
            address: _role_1,
            role: AccountRole::from_privileges(true, false),
        });

        accounts.push(AccountMeta {
            address: _role_2,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_3,
            role: AccountRole::from_privileges(false, true),
        });

        accounts.push(AccountMeta {
            address: _role_4,
            role: AccountRole::from_privileges(false, false),
        });

        accounts.push(AccountMeta {
            address: _role_5,
            role: AccountRole::from_privileges(false, false),
        });

        Ok(accounts)
    }
    /// Builds the canonical invocation with resolved defaults and caller limits.
    pub fn instruction(
        &self,
        data: &CancelSubscriptionNow,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        self.instruction_with_program(PROGRAM, data, limits)
    }
    /// Builds with an explicit program and derives every PDA against that program.
    pub fn instruction_with_program(
        &self,
        program: Address,
        data: &CancelSubscriptionNow,
        limits: Limits,
    ) -> Result<Instruction, DefaultError> {
        let accounts = self.resolve(program)?;

        Ok(data.instruction_with_program(program, &accounts, limits)?)
    }
}
