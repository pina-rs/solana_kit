//! Explicit snapshot calculations mirror primary transfer validation with checked arithmetic.
use crate::{PlanTerms, SubscriptionsError};

const SECONDS_PER_HOUR: u64 = 3600;

/// Hard expiry: equality is still billable; zero means no expiry.
pub const fn is_expired(expiry: i64, now: i64) -> bool {
    expiry != 0 && now > expiry
}

/// Checks a fixed allowance without changing the caller's snapshot.
/// # Errors
/// Returns the exact program amount, expiry or allowance error.
pub fn validate_fixed_transfer(
    amount: u64,
    remaining: u64,
    expiry: i64,
    now: i64,
) -> Result<(), SubscriptionsError> {
    if amount == 0 {
        return Err(SubscriptionsError::InvalidAmount);
    }

    if is_expired(expiry, now) {
        return Err(SubscriptionsError::DelegationExpired);
    }

    if amount > remaining {
        return Err(SubscriptionsError::AmountExceedsLimit);
    }

    Ok(())
}

/// A caller-owned recurring allowance before or after a proposed transfer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecurringAllowance {
    /// Start of the last observed billing period.
    pub period_start: i64,
    /// Tokens already pulled in that period.
    pub pulled: u64,
    /// Maximum tokens per period.
    pub amount_per_period: u64,
    /// Positive period length in seconds.
    pub period_seconds: u64,
    /// Hard expiry; zero means unbounded.
    pub expiry: i64,
}

impl RecurringAllowance {
    /// Returns the proposed new snapshot only when the whole transfer is valid.
    /// Finite expiry caps advancement at the final boundary strictly before expiry.
    /// # Errors
    /// Preserves source program amount, period, expiry and checked-arithmetic errors.
    pub fn transfer(self, amount: u64, now: i64) -> Result<Self, SubscriptionsError> {
        if amount == 0 {
            return Err(SubscriptionsError::InvalidAmount);
        }

        if is_expired(self.expiry, now) {
            return Err(SubscriptionsError::DelegationExpired);
        }

        let period = i64::try_from(self.period_seconds)
            .map_err(|_| SubscriptionsError::InvalidPeriodLength)?;

        if period == 0 {
            return Err(SubscriptionsError::InvalidPeriodLength);
        }

        if now < self.period_start {
            return Err(SubscriptionsError::DelegationNotStarted);
        }

        let mut proposed = self.advance(period, now)?;
        let available = self
            .amount_per_period
            .checked_sub(proposed.pulled)
            .ok_or(SubscriptionsError::ArithmeticUnderflow)?;

        if amount > available {
            return Err(SubscriptionsError::AmountExceedsPeriodLimit);
        }

        proposed.pulled = proposed
            .pulled
            .checked_add(amount)
            .ok_or(SubscriptionsError::ArithmeticOverflow)?;
        Ok(proposed)
    }

    fn advance(mut self, period: i64, now: i64) -> Result<Self, SubscriptionsError> {
        let elapsed_now = now
            .checked_sub(self.period_start)
            .ok_or(SubscriptionsError::ArithmeticOverflow)?;

        if elapsed_now < period {
            return Ok(self);
        }

        let last = if self.expiry == 0 {
            now
        } else {
            now.min(
                self.expiry
                    .checked_sub(1)
                    .ok_or(SubscriptionsError::ArithmeticUnderflow)?,
            )
        };

        if last < self.period_start {
            return Ok(self);
        }

        let elapsed = last
            .checked_sub(self.period_start)
            .ok_or(SubscriptionsError::ArithmeticOverflow)?;
        let increment = (elapsed / period)
            .checked_mul(period)
            .ok_or(SubscriptionsError::ArithmeticOverflow)?;
        let next = self
            .period_start
            .checked_add(increment)
            .ok_or(SubscriptionsError::ArithmeticOverflow)?;

        if next > self.period_start {
            self.period_start = next;
            self.pulled = 0;
        }

        Ok(self)
    }
}

/// Converts the full unsigned hours value without lossy casts or wraparound.
/// # Errors
/// Rejects zero periods, multiplication overflow and periods beyond signed timestamp range.
pub fn period_seconds(hours: u64) -> Result<i64, SubscriptionsError> {
    if hours == 0 {
        return Err(SubscriptionsError::InvalidPeriodLength);
    }

    let seconds = hours
        .checked_mul(SECONDS_PER_HOUR)
        .ok_or(SubscriptionsError::ArithmeticOverflow)?;
    i64::try_from(seconds).map_err(|_| SubscriptionsError::ArithmeticOverflow)
}

/// Validates the primary program's zero-or-at-least-one-full-period end timestamp.
/// # Errors
/// Returns InvalidEndTs or the exact checked arithmetic/period error.
pub fn validate_plan_end(end: i64, hours: u64, now: i64) -> Result<(), SubscriptionsError> {
    if end == 0 {
        return Ok(());
    }

    let earliest = now
        .checked_add(period_seconds(hours)?)
        .ok_or(SubscriptionsError::ArithmeticOverflow)?;

    if end < earliest {
        return Err(SubscriptionsError::InvalidEndTs);
    }

    Ok(())
}

/// Compares all immutable billing terms, retaining the source approval invariant.
/// # Errors
/// Returns PlanTermsMismatch if any field differs.
pub fn check_plan_terms(
    snapshot: &PlanTerms,
    expected: &PlanTerms,
) -> Result<(), SubscriptionsError> {
    if snapshot != expected {
        return Err(SubscriptionsError::PlanTermsMismatch);
    }

    Ok(())
}
