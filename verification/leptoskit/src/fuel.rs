//! Host-owned accounting for the fixed conformance workload.

use anyhow::{Context, Result, ensure};
use solana_kit_conformance::{
    EXPECTED_CASE_COUNT,
    message_cases::{CRYPTO_CALLS_PER_MESSAGE, FIXED_CRYPTO_CALLS, MESSAGE_COUNT},
    signers::MAX_BATCH,
};

const FUEL_PER_ORDINARY_CASE: u64 = 15_000_000;
const FUEL_PER_CRYPTO_UNIT: u64 = 5_000_000;
const SEPARATELY_ACCOUNTED_GROUPS: u64 = 3;

/// Grants pooled ordinary work, authenticated transactions, and actual message crypto calls.
pub(crate) fn allowance() -> Result<u64> {
    checked_allowance(
        u64::try_from(EXPECTED_CASE_COUNT)?,
        MAX_BATCH,
        MESSAGE_COUNT,
    )
}

fn checked_allowance(case_count: u64, batch_size: usize, message_count: usize) -> Result<u64> {
    ensure!(
        case_count >= SEPARATELY_ACCOUNTED_GROUPS,
        "missing metered cases"
    );
    ensure!(
        (1..=MAX_BATCH).contains(&batch_size),
        "invalid fixed transaction workload"
    );
    ensure!(
        (1..=MESSAGE_COUNT).contains(&message_count),
        "invalid fixed message workload"
    );
    let ordinary = (case_count - SEPARATELY_ACCOUNTED_GROUPS)
        .checked_mul(FUEL_PER_ORDINARY_CASE)
        .context("ordinary fixture fuel overflow")?;
    let transactions = u64::try_from(batch_size)?
        .checked_add(1)
        .context("batch transaction count overflow")?;
    let message_calls = u64::try_from(message_count)?
        .checked_mul(CRYPTO_CALLS_PER_MESSAGE)
        .and_then(|calls| calls.checked_add(FIXED_CRYPTO_CALLS))
        .context("message crypto count overflow")?;
    let crypto = transactions
        .checked_add(message_calls)
        .and_then(|units| units.checked_mul(FUEL_PER_CRYPTO_UNIT))
        .context("crypto fixture fuel overflow")?;

    ordinary.checked_add(crypto).context("suite fuel overflow")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accounts_for_transaction_effects_and_distinct_message_crypto() {
        assert_eq!(allowance().unwrap(), 6_720_000_000);
        assert_eq!(MESSAGE_COUNT, 256);
        assert_eq!(
            MESSAGE_COUNT as u64 * CRYPTO_CALLS_PER_MESSAGE + FIXED_CRYPTO_CALLS,
            772
        );
    }

    #[test]
    fn rejects_invalid_or_overflowing_metadata() {
        assert!(checked_allowance(0, MAX_BATCH, MESSAGE_COUNT).is_err());
        assert!(checked_allowance(103, 0, MESSAGE_COUNT).is_err());
        assert!(checked_allowance(103, MAX_BATCH + 1, MESSAGE_COUNT).is_err());
        assert!(checked_allowance(103, MAX_BATCH, 0).is_err());
        assert!(checked_allowance(103, MAX_BATCH, MESSAGE_COUNT + 1).is_err());
        assert!(checked_allowance(u64::MAX, MAX_BATCH, MESSAGE_COUNT).is_err());
        assert!(
            checked_allowance(
                u64::MAX / FUEL_PER_ORDINARY_CASE + SEPARATELY_ACCOUNTED_GROUPS,
                MAX_BATCH,
                MESSAGE_COUNT
            )
            .is_err()
        );
    }
}
