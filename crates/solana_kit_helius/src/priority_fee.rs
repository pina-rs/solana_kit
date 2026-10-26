//! Priority fee operations preserve provider decimals exactly.

use crate::{
    Decimal,
    enums::{PriorityLevel, UiTransactionEncoding},
};
use serde::{Deserialize, Serialize};

const MAX_LOOKBACK_SLOTS: u8 = 150;

/// Number of recent slots used for fee estimation, constrained to the provider's 1–150 range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub struct LookbackSlots(u8);

impl TryFrom<u8> for LookbackSlots {
    type Error = InvalidLookbackSlots;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=MAX_LOOKBACK_SLOTS).contains(&value) {
            return Err(InvalidLookbackSlots);
        }

        Ok(Self(value))
    }
}

impl From<LookbackSlots> for u8 {
    fn from(value: LookbackSlots) -> Self {
        value.0
    }
}

/// A requested fee-estimation lookback falls outside the provider's accepted range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidLookbackSlots;

impl core::fmt::Display for InvalidLookbackSlots {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "priority fee lookback must be between 1 and {MAX_LOOKBACK_SLOTS} slots"
        )
    }
}

impl std::error::Error for InvalidLookbackSlots {}

request!(
    "Estimate fees for account keys or an encoded transaction.",
    GetPriorityFeeEstimateRequest {}
    optional {
        account_keys: Vec<String> => "accountKeys",
        transaction: String => "transaction",
        options: PriorityFeeOptions => "options",
    }
);
request!(
    "Fee estimation options from the pinned source.",
    PriorityFeeOptions {} optional {
        priority_level: PriorityLevel => "priorityLevel",
        include_all_priority_fee_levels: bool => "includeAllPriorityFeeLevels",
        transaction_encoding: UiTransactionEncoding => "transactionEncoding",
        lookback_slots: LookbackSlots => "lookbackSlots",
        include_vote: bool => "includeVote",
        recommended: bool => "recommended",
        evaluate_empty_slot_as_zero: bool => "evaluateEmptySlotAsZero",
    }
);
model!(
    "Estimated priority fees in micro-lamports.",
    GetPriorityFeeEstimateResponse {} optional {
        priority_fee_estimate: Decimal => "priorityFeeEstimate",
        priority_fee_levels: MicroLamportPriorityFeeLevels => "priorityFeeLevels",
    }
);
model!(
    "Fee estimates for each priority tier.",
    MicroLamportPriorityFeeLevels {} optional {
        min: Decimal => "min",
        low: Decimal => "low",
        medium: Decimal => "medium",
        high: Decimal => "high",
        very_high: Decimal => "veryHigh",
        unsafe_max: Decimal => "unsafeMax",
    }
);
method!(
    GetPriorityFeeEstimateRequest,
    "getPriorityFeeEstimate",
    GetPriorityFeeEstimateResponse
);
