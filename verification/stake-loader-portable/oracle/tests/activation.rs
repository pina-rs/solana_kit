//! Native-only differential tests against the exact published Stake Interface.
#![cfg(not(target_arch = "wasm32"))]

use solana_kit_core::{
    Address,
    sysvars::{StakeHistoryData, StakeHistoryEntry},
};
use solana_kit_stake::{Delegation, Epoch};
use solana_stake_interface::{
    stake_history::{
        MAX_ENTRIES, StakeHistory, StakeHistoryEntry as ReferenceEntry, StakeHistoryGetEntry,
    },
    state::Delegation as ReferenceDelegation,
};
use std::collections::BTreeMap;

const HISTORY_EPOCHS: u64 = 8;
const AMOUNT_CHOICES: u64 = 5;
const DEACTIVATION_EPOCH: u64 = 5;

/// The public interface accepts sparse lookups; its concrete sysvar container does not.
struct ByEpoch(BTreeMap<u64, ReferenceEntry>);

impl StakeHistoryGetEntry for ByEpoch {
    fn get_entry(&self, epoch: u64) -> Option<ReferenceEntry> {
        self.0.get(&epoch).cloned()
    }
}

/// Exercises high balances, zero portions, gaps and both rate schedules independently.
#[test]
fn integer_activation_matches_the_published_program_interface() {
    assert_eq!(MAX_ENTRIES, 512);
    compare_history(&[]);
    let mut seed = 0x9c1d_347a_29b0_4821_u64;

    for _ in 0..128 {
        let history: Vec<_> = (0..HISTORY_EPOCHS)
            .rev()
            .map(|epoch| cluster(epoch, &mut seed))
            .collect();
        compare_history(&history);
        let gapped: Vec<_> = history
            .into_iter()
            .filter(|entry| entry.epoch % 3 != 1)
            .collect();
        compare_history(&gapped);
    }
}

/// All seed arithmetic is explicit and reproducible, including intentional wraparound.
fn next(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    *seed
}

/// Includes zero, modest and near-maximum observations without runtime randomness.
fn cluster(epoch: u64, seed: &mut u64) -> StakeHistoryEntry {
    let amounts = [0, 1, 1_000, next(seed), u64::MAX];
    let effective = amounts[(next(seed) % AMOUNT_CHOICES) as usize];
    let activating = amounts[(next(seed) % AMOUNT_CHOICES) as usize];
    let deactivating = amounts[(next(seed) % AMOUNT_CHOICES) as usize];
    StakeHistoryEntry {
        epoch,
        stake_history: StakeHistoryData {
            effective,
            activating,
            deactivating,
        },
    }
}

/// Checks real sysvar indexing only for contiguous input, plus independent map lookup.
fn compare_history(history: &[StakeHistoryEntry]) {
    let mut reference = StakeHistory::default();
    let mut sparse = ByEpoch(BTreeMap::new());

    for item in history {
        let data = item.stake_history;
        let entry = ReferenceEntry {
            effective: data.effective,
            activating: data.activating,
            deactivating: data.deactivating,
        };
        reference.add(item.epoch, entry.clone());
        sparse.0.insert(item.epoch, entry);
    }

    let contiguous = history
        .iter()
        .zip(history.iter().skip(1))
        .all(|(newer, older)| newer.epoch.checked_sub(1) == Some(older.epoch));

    if contiguous {
        compare_variants(history, &reference);
    }

    compare_variants(history, &sparse);
}

/// Reuses each input case with the published generic calculation, never copied math.
fn compare_variants<T: StakeHistoryGetEntry>(history: &[StakeHistoryEntry], reference: &T) {
    for (activation, deactivation) in [
        (0, 2),
        (0, DEACTIVATION_EPOCH),
        (2, 2),
        (2, DEACTIVATION_EPOCH),
        (2, u64::MAX),
        (u64::MAX, DEACTIVATION_EPOCH),
        (u64::MAX, u64::MAX),
    ] {
        for stake in [0, 1, 100, 400_000_000_000_000_000, u64::MAX] {
            let value = Delegation {
                voter_pubkey: Address::from_bytes([4; 32]),
                stake,
                activation_epoch: Epoch { value: activation },
                deactivation_epoch: Epoch {
                    value: deactivation,
                },
                reserved: vec![0; 8],
            };
            compare_delegation(&value, history, reference);
        }
    }
}

/// Compares all three amounts instead of reducing the oracle to a status label.
fn compare_delegation<T: StakeHistoryGetEntry>(
    value: &Delegation,
    history: &[StakeHistoryEntry],
    reference_history: &T,
) {
    let reference = ReferenceDelegation {
        stake: value.stake,
        activation_epoch: value.activation_epoch.value,
        deactivation_epoch: value.deactivation_epoch.value,
        ..ReferenceDelegation::default()
    };

    for epoch in (0..=HISTORY_EPOCHS).chain([u64::MAX]) {
        for rate in [None, Some(0), Some(3), Some(u64::MAX)] {
            let actual = value.activation_status_v2(epoch, history, rate).unwrap();
            let expected =
                reference.stake_activating_and_deactivating_v2(epoch, reference_history, rate);
            assert_eq!(
                [actual.effective, actual.activating, actual.deactivating],
                [
                    expected.effective,
                    expected.activating,
                    expected.deactivating
                ],
                "delegation={value:?}, epoch={epoch}, rate={rate:?}, history={history:?}"
            );
        }
    }
}
