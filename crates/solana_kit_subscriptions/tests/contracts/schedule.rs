//! Primary period/expiry vectors and checked arithmetic failures, with immutable snapshots.
use super::support::*;
use solana_kit_subscriptions::*;

const PERIOD: u64 = 30;
const ALLOWANCE: u64 = 100;
const EXPIRY: i64 = 90;
const HOUR_SECONDS: i64 = 3600;
const ERROR_COUNT: usize = 78;
const UNKNOWN_CODES: &[u32] = &[0, 99, 605, u32::MAX];

fn allowance(start: i64, pulled: u64, expiry: i64) -> RecurringAllowance {
    RecurringAllowance {
        period_start: start,
        pulled,
        amount_per_period: ALLOWANCE,
        period_seconds: PERIOD,
        expiry,
    }
}

#[cfg_attr(test, test)]
pub fn schedule_exact_boundaries_checked_amounts_and_no_partial_mutation() {
    let original = allowance(0, ALLOWANCE, EXPIRY);
    assert_eq!(
        original.transfer(ALLOWANCE, EXPIRY).unwrap(),
        allowance(60, ALLOWANCE, EXPIRY)
    );
    assert_eq!(
        original.transfer(ALLOWANCE, 35).unwrap(),
        allowance(30, ALLOWANCE, EXPIRY)
    );
    assert_eq!(
        allowance(60, ALLOWANCE, EXPIRY).transfer(1, EXPIRY),
        Err(SubscriptionsError::AmountExceedsPeriodLimit)
    );
    assert_eq!(
        original.transfer(1, EXPIRY + 1),
        Err(SubscriptionsError::DelegationExpired)
    );
    assert_eq!(
        original.transfer(0, 0),
        Err(SubscriptionsError::InvalidAmount)
    );
    assert_eq!(
        original.transfer(1, -1),
        Err(SubscriptionsError::DelegationNotStarted)
    );
    assert_eq!(original.pulled, ALLOWANCE);

    assert!(!is_expired(EXPIRY, EXPIRY));
    assert!(is_expired(EXPIRY, EXPIRY + 1));
    assert!(!is_expired(0, i64::MAX));
    assert!(
        allowance(i64::MIN, 0, i64::MIN)
            .transfer(1, i64::MIN)
            .is_ok()
    );
    assert_eq!(
        validate_fixed_transfer(1, 0, 0, 0),
        Err(SubscriptionsError::AmountExceedsLimit)
    );
    assert_eq!(
        validate_fixed_transfer(0, ALLOWANCE, 0, 0),
        Err(SubscriptionsError::InvalidAmount)
    );
    assert_eq!(
        validate_fixed_transfer(1, ALLOWANCE, EXPIRY, EXPIRY + 1),
        Err(SubscriptionsError::DelegationExpired)
    );
    assert!(validate_fixed_transfer(ALLOWANCE, ALLOWANCE, EXPIRY, EXPIRY).is_ok());
}

#[cfg_attr(test, test)]
pub fn schedule_invalid_periods_overflow_and_full_plan_approval_contract() {
    let mut value = allowance(0, 0, 0);
    value.period_seconds = 0;
    assert_eq!(
        value.transfer(1, 0),
        Err(SubscriptionsError::InvalidPeriodLength)
    );
    value.period_seconds = u64::MAX;
    assert_eq!(
        value.transfer(1, 0),
        Err(SubscriptionsError::InvalidPeriodLength)
    );
    let mut value = allowance(i64::MIN, 0, 0);
    assert_eq!(
        value.transfer(1, i64::MAX),
        Err(SubscriptionsError::ArithmeticOverflow)
    );
    value = allowance(0, ALLOWANCE + 1, 0);

    assert_eq!(
        value.transfer(1, 0),
        Err(SubscriptionsError::ArithmeticUnderflow)
    );
    assert_eq!(period_seconds(1), Ok(HOUR_SECONDS));
    assert_eq!(
        period_seconds(0),
        Err(SubscriptionsError::InvalidPeriodLength)
    );
    assert_eq!(
        period_seconds(u64::MAX),
        Err(SubscriptionsError::ArithmeticOverflow)
    );
    assert!(validate_plan_end(HOUR_SECONDS, 1, 0).is_ok());
    assert_eq!(
        validate_plan_end(HOUR_SECONDS - 1, 1, 0),
        Err(SubscriptionsError::InvalidEndTs)
    );
    assert_eq!(
        validate_plan_end(i64::MAX, 1, i64::MAX),
        Err(SubscriptionsError::ArithmeticOverflow)
    );
    assert!(validate_plan_end(0, u64::MAX, i64::MAX).is_ok());

    let mut changed = terms();
    assert!(check_plan_terms(&terms(), &changed).is_ok());
    changed.amount += 1;
    assert_eq!(
        check_plan_terms(&terms(), &changed),
        Err(SubscriptionsError::PlanTermsMismatch)
    );
}

#[cfg_attr(test, test)]
pub fn every_primary_error_name_code_message_and_unknown_code() {
    let mut count = 0;

    for line in include_str!("../fixtures/errors.tsv").lines() {
        let fields: Vec<_> = line.split('\t').collect();
        let code = fields[0].parse().unwrap();
        let error = SubscriptionsError::from_code(code);
        let name = format!("{}{}", fields[1][..1].to_ascii_uppercase(), &fields[1][1..]);
        assert_eq!(format!("{error:?}"), name);
        assert_eq!(error.code(), code);
        assert_eq!(error.message(), Some(fields[2]));

        count += 1;
    }

    assert_eq!(count, ERROR_COUNT);

    for &code in UNKNOWN_CODES {
        let error = SubscriptionsError::from_code(code);
        assert_eq!(error, SubscriptionsError::Unknown(code));
        assert_eq!(error.code(), code);
        assert_eq!(error.message(), None);
    }
}
