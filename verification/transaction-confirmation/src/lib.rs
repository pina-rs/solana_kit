#![forbid(unsafe_code)]
//! Isolated confirmation groups, deliberately absent from the shared registry.
use solana_kit_client::{
    confirmation::{Confirmation, Event, Failure, Policy, commitment_comparator},
    methods::{Commitment, node::EpochInfo},
};
use solana_kit_core::{Hash, Signature, transaction::Lifetime};
const LAST_VALID_HEIGHT: u64 = 41;
const EXPIRED_HEIGHT: u64 = 42;

/// Exact commitment ordering, reusable by later runtime integration.
#[must_use]
pub fn commitment_order() -> bool {
    let levels = [
        Commitment::Processed,
        Commitment::Confirmed,
        Commitment::Finalized,
    ];
    levels.iter().enumerate().all(|(i, a)| {
        levels
            .iter()
            .enumerate()
            .all(|(j, b)| commitment_comparator(*a, *b) == i.cmp(&j))
    })
}

/// Independent inclusive expiry boundary with a maximum exact integer control.
#[must_use]
pub fn expiry_boundary() -> bool {
    let signature = Signature::from_bytes([7; 64]);
    let lifetime = Lifetime::Blockhash {
        blockhash: Hash::from_bytes([1; 32]),
        last_valid_block_height: LAST_VALID_HEIGHT,
    };
    let Ok(mut confirmation) = Confirmation::new(signature, lifetime, Policy::default()) else {
        return false;
    };
    let boundary = Event::Epoch(epoch(LAST_VALID_HEIGHT));
    let after = Event::Epoch(epoch(EXPIRED_HEIGHT));
    confirmation.observe(boundary).is_none()
        && matches!(
            confirmation.observe(after),
            Some(Err(Failure::BlockHeightExceeded {
                current: EXPIRED_HEIGHT,
                last_valid: LAST_VALID_HEIGHT
            }))
        )
}
fn epoch(block_height: u64) -> EpochInfo {
    EpochInfo {
        absolute_slot: u64::MAX,
        block_height,
        epoch: 0,
        slot_index: 0,
        slots_in_epoch: 100,
        transaction_count: None,
    }
}
