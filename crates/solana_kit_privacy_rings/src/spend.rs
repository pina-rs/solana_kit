//! Subtract notes a nullifier or a non-inclusion proof can account for.
//!
//! The indexer does not say which nullifier spent which leaf. A caller that
//! has derived the nullifier for a leaf can drop that note when the nullifier
//! appears in history. A non-inclusion proof for the same nullifier says the
//! note is still unspent. Notes with no derived nullifier stay in the set:
//! this module will not guess.

use std::collections::{BTreeMap, BTreeSet};

use crate::viewing::TrackedNote;

/// Drops notes whose derived nullifier is in `spent`.
///
/// A note with no entry in `nullifier_of` is kept.
#[must_use]
pub fn subtract_nullified(
    notes: &[TrackedNote],
    nullifier_of: &BTreeMap<(u16, u64), [u8; 32]>,
    spent: &BTreeSet<[u8; 32]>,
) -> Vec<TrackedNote> {
    notes
        .iter()
        .filter(|note| !is_spent(note, nullifier_of, spent))
        .cloned()
        .collect()
}

/// Keeps a note when its nullifier has a non-inclusion proof.
///
/// A note whose nullifier is known and absent from `unspent` is removed. A
/// note with no derived nullifier is kept, because the proof cannot name it.
#[must_use]
pub fn retain_non_inclusion(
    notes: &[TrackedNote],
    nullifier_of: &BTreeMap<(u16, u64), [u8; 32]>,
    unspent: &BTreeSet<[u8; 32]>,
) -> Vec<TrackedNote> {
    notes
        .iter()
        .filter(|note| is_unspent(note, nullifier_of, unspent))
        .cloned()
        .collect()
}

fn is_spent(
    note: &TrackedNote,
    nullifier_of: &BTreeMap<(u16, u64), [u8; 32]>,
    spent: &BTreeSet<[u8; 32]>,
) -> bool {
    nullifier_of
        .get(&(note.tree_id, note.leaf_index))
        .is_some_and(|nullifier| spent.contains(nullifier))
}

fn is_unspent(
    note: &TrackedNote,
    nullifier_of: &BTreeMap<(u16, u64), [u8; 32]>,
    unspent: &BTreeSet<[u8; 32]>,
) -> bool {
    match nullifier_of.get(&(note.tree_id, note.leaf_index)) {
        Some(nullifier) => unspent.contains(nullifier),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode::ConfidentialPlaintext;
    use crate::viewing::OpenedNote;

    fn note(tree_id: u16, leaf_index: u64) -> TrackedNote {
        TrackedNote {
            tree_id,
            leaf_index,
            note: OpenedNote::Confidential(ConfidentialPlaintext {
                asset_id: 1,
                amount: 4,
                blinding: [2; 32],
                ring_program_id: None,
                records: Vec::new(),
            }),
        }
    }

    #[test]
    fn spent_nullifiers_drop_and_non_inclusion_keeps() {
        let notes = vec![note(1, 2), note(1, 3)];
        let mut derived = BTreeMap::new();
        derived.insert((1, 2), [9; 32]);
        let mut spent = BTreeSet::new();
        spent.insert([9; 32]);
        let left = subtract_nullified(&notes, &derived, &spent);
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].leaf_index, 3);

        let mut unspent = BTreeSet::new();
        unspent.insert([9; 32]);
        let mut both = derived.clone();
        both.insert((1, 3), [8; 32]);
        let kept = retain_non_inclusion(&notes, &both, &unspent);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].leaf_index, 2);
        assert_eq!(retain_non_inclusion(&notes, &derived, &unspent).len(), 2);
    }
}
