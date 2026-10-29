//! Exact rate and total ceilings stay consistent across transaction versions.

use super::decimal::floor_at_most;
use crate::Decimal;
use core::{fmt, num::NonZeroU64};

const MICRO_LAMPORTS_PER_LAMPORT: u128 = 1_000_000;
const RATE_CEILING: u128 = u64::MAX as u128 + 1;

/// Exact priority-fee inputs; zero units remains a valid zero-total request.
#[derive(Debug, Clone, Copy)]
pub struct ResolvePriorityFeeInput<'a> {
    /// Provider rate in micro-lamports per compute unit, retaining its exact decimal.
    pub estimate: &'a Decimal,
    /// Compute-unit limit used to price the total fee.
    pub units: u64,
    /// Optional per-unit rate ceiling; negative values clamp to zero.
    pub rate_cap: Option<&'a Decimal>,
    /// Optional total lamport ceiling, floored before use; ignored for zero units.
    pub lamports_cap: Option<&'a Decimal>,
}

/// Whole rate and rounded-up total describing the same fee.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedPriorityFee {
    /// Micro-lamports per compute unit for a legacy or version-zero price instruction.
    pub rate: u64,
    /// Whole lamports for a version-one priority fee.
    pub lamports: u64,
}

/// A resolved fee cannot fit the network's unsigned wire representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityFeeError {
    /// The rate still exceeds u64 after applying every cap.
    RateOverflow,
    /// The rounded-up total exceeds u64.
    TotalOverflow,
}

impl fmt::Display for PriorityFeeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::RateOverflow => "resolved priority fee rate exceeds u64",
            Self::TotalOverflow => "resolved priority fee total exceeds u64",
        })
    }
}

impl std::error::Error for PriorityFeeError {}

/// Floors an exact estimate and clamps the rate to both caller ceilings.
///
/// Negative numbers clamp to zero. Total caps floor to whole lamports before
/// limiting the rate. The final total is ceil(rate * units / 1_000_000).
/// Caps apply before wire-range checks, so a large estimate can still resolve.
/// Decimal excludes Dart's NaN/Infinity inputs; outputs must fit wire u64 values.
pub fn resolve_priority_fee(
    input: ResolvePriorityFeeInput<'_>,
) -> Result<ResolvedPriorityFee, PriorityFeeError> {
    let mut rate = floor_at_most(input.estimate, RATE_CEILING);

    if let Some(cap) = input.rate_cap {
        rate = rate.min(floor_at_most(cap, RATE_CEILING));
    }

    if let (Some(cap), Some(units)) = (input.lamports_cap, NonZeroU64::new(input.units)) {
        rate = rate.min(rate_from_total_cap(cap, units)?);
    }

    let rate = u64::try_from(rate).map_err(|_| PriorityFeeError::RateOverflow)?;
    let micro_lamports = u128::from(rate)
        .checked_mul(u128::from(input.units))
        .ok_or(PriorityFeeError::TotalOverflow)?;
    let lamports = u64::try_from(micro_lamports.div_ceil(MICRO_LAMPORTS_PER_LAMPORT))
        .map_err(|_| PriorityFeeError::TotalOverflow)?;

    Ok(ResolvedPriorityFee { rate, lamports })
}

/// Keeps every binding total cap, including MAX+1, while bounding its multiplication.
fn rate_from_total_cap(cap: &Decimal, units: NonZeroU64) -> Result<u128, PriorityFeeError> {
    let units = u128::from(units.get());
    let ceiling = RATE_CEILING
        .checked_mul(units)
        .ok_or(PriorityFeeError::RateOverflow)?
        .div_ceil(MICRO_LAMPORTS_PER_LAMPORT);
    let lamports = floor_at_most(cap, ceiling);

    // Above this ceiling the quotient cannot constrain a supported u64 rate.
    // Its rounded-up multiple still fits u128 because units is at most u64::MAX.
    Ok(lamports
        .checked_mul(MICRO_LAMPORTS_PER_LAMPORT)
        .ok_or(PriorityFeeError::RateOverflow)?
        / units)
}
