//! Host-owned accounting for the fixed conformance workload.

use anyhow::{Context, Result, ensure};
use solana_kit_conformance::{EXPECTED_CASE_COUNT, signers::MAX_BATCH};

const FUEL_PER_ORDINARY_CASE: u64 = 15_000_000;
const FUEL_PER_SIGNED_TRANSACTION: u64 = 5_000_000;

/// Grants ordinary pooled work plus the batch's authenticated transactions.
pub(crate) fn allowance() -> Result<u64> {
    checked_allowance(u64::try_from(EXPECTED_CASE_COUNT)?, MAX_BATCH)
}

fn checked_allowance(case_count: u64, batch_size: usize) -> Result<u64> {
    ensure!(case_count > 0, "missing batch case");
    ensure!(
        (1..=MAX_BATCH).contains(&batch_size),
        "invalid fixed batch workload"
    );
    let ordinary = (case_count - 1)
        .checked_mul(FUEL_PER_ORDINARY_CASE)
        .context("ordinary fixture fuel overflow")?;
    let transactions = u64::try_from(batch_size)?
        .checked_add(1)
        .context("batch transaction count overflow")?;
    let batch = transactions
        .checked_mul(FUEL_PER_SIGNED_TRANSACTION)
        .context("batch fixture fuel overflow")?;

    ordinary.checked_add(batch).context("suite fuel overflow")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accounts_for_all_256_successes_and_the_early_stop_success() {
        assert_eq!(allowance().unwrap(), 2_785_000_000);
    }

    #[test]
    fn rejects_invalid_or_overflowing_metadata() {
        assert!(checked_allowance(0, MAX_BATCH).is_err());
        assert!(checked_allowance(92, 0).is_err());
        assert!(checked_allowance(92, MAX_BATCH + 1).is_err());
        assert!(checked_allowance(u64::MAX, MAX_BATCH).is_err());
        assert!(checked_allowance(u64::MAX / FUEL_PER_ORDINARY_CASE + 1, MAX_BATCH).is_err());
    }
}
