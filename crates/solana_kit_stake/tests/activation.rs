//! Literal account outcomes cover integer rate changes and incomplete history.
use solana_kit_core::{
    Address,
    codecs::Error,
    sysvars::{StakeHistoryData, StakeHistoryEntry},
};
use solana_kit_stake::{Delegation, Epoch};

type Expected = (u64, Option<u64>, [u64; 3]);
const OBSERVATION_EPOCH: u64 = 5;
const DEACTIVATION_EPOCH: u64 = 9;
const CLUSTER_PENDING: u64 = 500;

/// Bootstrap precedes the instant-deactivation rule, including the sentinel epoch.
#[cfg_attr(test, test)]
pub fn bootstrap_and_epoch_boundaries_preserve_upstream_precedence() {
    check(
        &delegation(100, 4, DEACTIVATION_EPOCH),
        &[],
        &[
            (3, None, [0, 0, 0]),
            (4, None, [0, 100, 0]),
            (OBSERVATION_EPOCH, None, [100, 0, 0]),
            (9, None, [100, 0, 100]),
            (10, None, [0, 0, 0]),
        ],
    );
    check(
        &delegation(100, 4, 4),
        &[],
        &[
            (0, None, [0, 0, 0]),
            (4, None, [0, 0, 0]),
            (OBSERVATION_EPOCH, None, [0, 0, 0]),
        ],
    );

    check(
        &delegation(100, u64::MAX, DEACTIVATION_EPOCH),
        &[],
        &[(0, None, [100, 0, 0])],
    );
    check(
        &delegation(100, u64::MAX, u64::MAX),
        &[],
        &[(u64::MAX, None, [100, 0, 100])],
    );
}

/// The rate applies to the epoch being entered, with integer truncation per epoch.
#[cfg_attr(test, test)]
pub fn warmup_uses_cluster_share_and_the_rate_change_epoch() {
    let mut value = delegation(100, 0, u64::MAX);
    let history = [entry(1, 1_000, 500, 500), entry(0, 1_000, 500, 500)];
    check(
        &value,
        &history,
        &[
            (1, None, [50, 50, 0]),
            (1, Some(0), [18, 82, 0]),
            (2, None, [75, 25, 0]),
            (2, Some(0), [32, 68, 0]),
            (2, Some(2), [59, 41, 0]),
        ],
    );

    value.reserved.fill(u8::MAX);
    check(&value, &history, &[(1, None, [50, 50, 0])]);
    check(
        &delegation(100, 0, 1),
        &history,
        &[(1, None, [50, 0, 50]), (2, None, [25, 0, 25])],
    );
}

/// Cooldown reduces the effective portion; it never reactivates cancelled warmup.
#[cfg_attr(test, test)]
pub fn cooldown_retains_only_effective_stake_and_uses_cluster_history() {
    let history = [entry(3, 1_000, 500, 500), entry(2, 1_000, 500, 500)];
    check(
        &delegation(100, 0, 2),
        &history,
        &[
            (2, None, [100, 0, 100]),
            (3, None, [50, 0, 50]),
            (4, None, [25, 0, 25]),
            (3, Some(0), [82, 0, 82]),
            (4, Some(0), [68, 0, 68]),
        ],
    );
}

/// An internal gap stops the walk; an absent initial entry uses upstream fallback.
#[cfg_attr(test, test)]
pub fn missing_history_distinguishes_initial_fallback_from_internal_gaps() {
    let warming = delegation(100, 0, u64::MAX);
    let history = [entry(2, 1_000, 500, 500), entry(0, 1_000, 500, 500)];
    check(&warming, &history, &[(3, None, [50, 50, 0])]);
    check(&warming, &history[..1], &[(3, None, [100, 0, 0])]);

    let cooling = delegation(100, 0, 2);
    let history = [entry(4, 1_000, 500, 500), entry(2, 1_000, 500, 500)];
    check(
        &cooling,
        &history,
        &[(OBSERVATION_EPOCH, None, [50, 0, 50])],
    );
    check(
        &cooling,
        &history[..1],
        &[(OBSERVATION_EPOCH, None, [0, 0, 0])],
    );
}

/// Zero cluster portions stop progress; zero allowances still move one lamport.
#[cfg_attr(test, test)]
pub fn minimum_progress_zero_portions_and_empty_accounts_are_distinct() {
    let history = [entry(2, 0, 1, 1), entry(1, 0, 1, 1), entry(0, 0, 1, 1)];
    check(
        &delegation(3, 0, u64::MAX),
        &history,
        &[(3, None, [3, 0, 0])],
    );
    check(
        &delegation(3, 0, 1),
        &history[0..2],
        &[(3, None, [1, 0, 1])],
    );
    check(
        &delegation(0, 0, u64::MAX),
        &history,
        &[(3, None, [0, 0, 0])],
    );

    let stopped = [entry(1, 1_000, 0, 0), entry(0, 1_000, 0, 0)];
    check(
        &delegation(3, 0, u64::MAX),
        &stopped,
        &[(2, None, [0, 3, 0])],
    );
    check(
        &delegation(3, u64::MAX, 1),
        &stopped,
        &[(2, None, [3, 0, 3])],
    );
}

/// Literal upstream saturation and the final representable epoch avoid float drift.
#[cfg_attr(test, test)]
pub fn saturation_and_maximum_epoch_keep_integer_rate_semantics() {
    let supply = 400_000_000_000_000_000;
    let history = [entry(0, supply, supply, supply)];
    check(
        &delegation(supply, 0, u64::MAX),
        &history,
        &[(
            1,
            None,
            [85_070_591_730_234_615, 314_929_408_269_765_385, 0],
        )],
    );

    let history = [entry(u64::MAX - 1, 1_000, 500, 500)];
    check(
        &delegation(100, u64::MAX - 1, u64::MAX),
        &history,
        &[
            (u64::MAX, None, [50, 0, 50]),
            (u64::MAX, Some(u64::MAX), [18, 0, 18]),
        ],
    );
}

/// Reject malformed ordering and excessive history before applying missing-entry rules.
#[cfg_attr(test, test)]
pub fn ambiguous_history_and_impossible_non_bootstrap_epochs_are_rejected() {
    let value = delegation(100, 4, DEACTIVATION_EPOCH);
    let older = entry(0, 1_000, CLUSTER_PENDING, CLUSTER_PENDING);
    let newer = entry(1, 1_000, CLUSTER_PENDING, CLUSTER_PENDING);

    for invalid in [[older, older], [older, newer]] {
        assert_eq!(
            value.activation_status_v2(OBSERVATION_EPOCH, &invalid, None),
            Err(Error::InvalidValue)
        );
    }

    let history: Vec<_> = (0..512).rev().map(|epoch| entry(epoch, 0, 0, 0)).collect();
    assert!(value.activation_status_v2(0, &history, None).is_ok());
    let excessive: Vec<_> = (0..513).rev().map(|epoch| entry(epoch, 0, 0, 0)).collect();
    assert_eq!(
        value.activation_status_v2(0, &excessive, None),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        delegation(100, 4, 3).activation_status_v2(OBSERVATION_EPOCH, &[], None),
        Err(Error::InvalidValue)
    );
}

/// Keeps fixture amounts independent of the production calculation.
fn check(value: &Delegation, history: &[StakeHistoryEntry], cases: &[Expected]) {
    for &(epoch, rate, expected) in cases {
        let status = value.activation_status_v2(epoch, history, rate).unwrap();
        assert_eq!(
            [status.effective, status.activating, status.deactivating],
            expected,
            "epoch={epoch}, rate={rate:?}"
        );
    }
}

/// Constructs a wire-valid account without a runtime or signer dependency.
fn delegation(stake: u64, activation: u64, deactivation: u64) -> Delegation {
    Delegation {
        voter_pubkey: Address::from_bytes([4; 32]),
        stake,
        activation_epoch: Epoch { value: activation },
        deactivation_epoch: Epoch {
            value: deactivation,
        },
        reserved: vec![0; 8],
    }
}

/// Constructs a literal cluster observation at one epoch.
fn entry(epoch: u64, effective: u64, activating: u64, deactivating: u64) -> StakeHistoryEntry {
    StakeHistoryEntry {
        epoch,
        stake_history: StakeHistoryData {
            effective,
            activating,
            deactivating,
        },
    }
}
