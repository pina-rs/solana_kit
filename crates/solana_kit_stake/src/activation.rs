//! Integer activation adapted from Solana Stake Interface at b37e4f76 (Apache-2.0).
//! Modified to use Kit values, validate history and walk borrowed entries without allocation.

use crate::Delegation;
use solana_kit_core::{
    codecs::Error,
    sysvars::{StakeHistoryData, StakeHistoryEntry},
};

const MAX_HISTORY_ENTRIES: usize = 512;
const BASIS_POINTS: u128 = 10_000;
const ORIGINAL_RATE: u128 = 2_500;
const REDUCED_RATE: u128 = 900;

impl Delegation {
    /// Estimates effective, activating and deactivating lamports with integer V2 rules.
    ///
    /// Supply unique history epochs in newest-first order, at most 512 entries. Gaps stop
    /// a history walk. An absent activation entry assumes full activation; an absent
    /// deactivation entry assumes full cooldown, matching the upstream history fallback.
    /// These estimates alone do not establish withdrawable funds or authenticate RPC data.
    ///
    /// `new_rate_activation_epoch` selects the first epoch with the reduced 9% rate.
    /// `None` retains 25%, including epoch `u64::MAX`; `Some(0)` uses 9% from genesis.
    /// The reserved wire bytes do not affect this integer calculation.
    ///
    /// Returns [`Error::LimitExceeded`] for oversized history and [`Error::InvalidValue`]
    /// for duplicate/out-of-order epochs or deactivation before a non-bootstrap activation.
    pub fn activation_status_v2(
        &self,
        epoch: u64,
        history: &[StakeHistoryEntry],
        new_rate_activation_epoch: Option<u64>,
    ) -> Result<StakeHistoryData, Error> {
        validate_history(history)?;

        if self.activation_epoch.value != u64::MAX
            && self.deactivation_epoch.value < self.activation_epoch.value
        {
            return Err(Error::InvalidValue);
        }

        let (effective, activating) = activate(self, epoch, history, new_rate_activation_epoch);

        if epoch < self.deactivation_epoch.value {
            return Ok(StakeHistoryData {
                effective,
                activating,
                deactivating: 0,
            });
        }

        let remaining = if epoch == self.deactivation_epoch.value {
            effective
        } else {
            cool_down(self, effective, epoch, history, new_rate_activation_epoch)
        };

        Ok(StakeHistoryData {
            effective: remaining,
            activating: 0,
            deactivating: remaining,
        })
    }
}

/// Rejects ambiguous history before any fallback can interpret a missing epoch.
fn validate_history(history: &[StakeHistoryEntry]) -> Result<(), Error> {
    if history.len() > MAX_HISTORY_ENTRIES {
        return Err(Error::LimitExceeded);
    }

    if history
        .iter()
        .zip(history.iter().skip(1))
        .any(|(newer, older)| newer.epoch <= older.epoch)
    {
        return Err(Error::InvalidValue);
    }

    Ok(())
}

/// Uses validated newest-first ordering to avoid a linear search per epoch.
fn entry(history: &[StakeHistoryEntry], epoch: u64) -> Option<StakeHistoryData> {
    history
        .binary_search_by(|entry| epoch.cmp(&entry.epoch))
        .ok()
        .map(|index| history[index].stake_history)
}

/// Resolves bootstrap and activation boundaries before a bounded history walk.
fn activate(
    delegation: &Delegation,
    epoch: u64,
    history: &[StakeHistoryEntry],
    rate_epoch: Option<u64>,
) -> (u64, u64) {
    let activation = delegation.activation_epoch.value;

    if activation == u64::MAX {
        return (delegation.stake, 0);
    }

    if activation == delegation.deactivation_epoch.value || epoch < activation {
        return (0, 0);
    }

    if epoch == activation {
        return (0, delegation.stake);
    }

    if entry(history, activation).is_none() {
        return (delegation.stake, 0);
    }

    let effective = warm_up(delegation, epoch, history, rate_epoch);
    (effective, delegation.stake - effective)
}

/// Applies each previous epoch's cluster allowance until time, stake or history runs out.
fn warm_up(
    delegation: &Delegation,
    epoch: u64,
    history: &[StakeHistoryEntry],
    rate_epoch: Option<u64>,
) -> u64 {
    let end = epoch.min(delegation.deactivation_epoch.value);
    let mut previous = delegation.activation_epoch.value;
    let mut effective = 0;

    while previous < end && effective < delegation.stake {
        let Some(cluster) = entry(history, previous) else {
            break;
        };

        if cluster.activating == 0 {
            break;
        }

        let remaining = delegation.stake - effective;
        let change = allowance(
            previous + 1,
            remaining,
            cluster.activating,
            cluster.effective,
            rate_epoch,
        );
        effective += change.max(1);
        previous += 1;
    }

    effective
}

/// Keeps incomplete cooldown history conservative after the initial fallback check.
fn cool_down(
    delegation: &Delegation,
    mut remaining: u64,
    epoch: u64,
    history: &[StakeHistoryEntry],
    rate_epoch: Option<u64>,
) -> u64 {
    let mut previous = delegation.deactivation_epoch.value;

    if entry(history, previous).is_none() {
        return 0;
    }

    while previous < epoch && remaining > 0 {
        let Some(cluster) = entry(history, previous) else {
            break;
        };

        if cluster.deactivating == 0 {
            break;
        }

        let change = allowance(
            previous + 1,
            remaining,
            cluster.deactivating,
            cluster.effective,
            rate_epoch,
        );
        remaining = remaining.saturating_sub(change.max(1));
        previous += 1;
    }

    remaining
}

/// Saturating integer products retain the upstream limit at extreme cluster balances.
fn allowance(
    epoch: u64,
    portion: u64,
    cluster_portion: u64,
    effective: u64,
    rate_epoch: Option<u64>,
) -> u64 {
    if portion == 0 || cluster_portion == 0 || effective == 0 {
        return 0;
    }

    let rate = if rate_epoch.is_some_and(|start| epoch >= start) {
        REDUCED_RATE
    } else {
        ORIGINAL_RATE
    };
    let numerator = u128::from(portion)
        .saturating_mul(u128::from(effective))
        .saturating_mul(rate);
    let denominator = u128::from(cluster_portion) * BASIS_POINTS;

    (numerator / denominator).min(u128::from(portion)) as u64
}
