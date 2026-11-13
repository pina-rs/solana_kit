//! Subtract notes a nullifier or a non-inclusion proof can account for.
//!
//! The indexer does not say which nullifier spent which leaf. A caller that
//! has derived the nullifier for a leaf can drop that note when the nullifier
//! appears in history. A non-inclusion proof for the same nullifier says the
//! note is still unspent. Notes with no derived nullifier stay in the set:
//! this module will not guess.

use std::collections::{BTreeMap, BTreeSet};

use crate::admission::VerifiedNote;

/// Drops notes whose derived nullifier is in `spent`.
///
/// A note with no entry in `nullifier_of` is kept. The map is keyed by the
/// note's verified commitment, so relabelling a leaf cannot bypass filtering.
#[must_use]
pub fn subtract_nullified(
    notes: &[VerifiedNote],
    nullifier_of: &BTreeMap<[u8; 32], [u8; 32]>,
    spent: &BTreeSet<[u8; 32]>,
) -> Vec<VerifiedNote> {
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
    notes: &[VerifiedNote],
    nullifier_of: &BTreeMap<[u8; 32], [u8; 32]>,
    unspent: &BTreeSet<[u8; 32]>,
) -> Vec<VerifiedNote> {
    notes
        .iter()
        .filter(|note| is_unspent(note, nullifier_of, unspent))
        .cloned()
        .collect()
}

fn is_spent(
    note: &VerifiedNote,
    nullifier_of: &BTreeMap<[u8; 32], [u8; 32]>,
    spent: &BTreeSet<[u8; 32]>,
) -> bool {
    nullifier_of
        .get(&note.commitment())
        .is_some_and(|nullifier| spent.contains(nullifier))
}

fn is_unspent(
    note: &VerifiedNote,
    nullifier_of: &BTreeMap<[u8; 32], [u8; 32]>,
    unspent: &BTreeSet<[u8; 32]>,
) -> bool {
    match nullifier_of.get(&note.commitment()) {
        Some(nullifier) => unspent.contains(nullifier),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "poseidon")]
    use crate::encode::ConfidentialPlaintext;
    #[cfg(feature = "poseidon")]
    use crate::poseidon::{UtxoPreimage, right_align, utxo_commitment};
    #[cfg(feature = "poseidon")]
    use crate::viewing::{OpenedNote, VerifiedNote, verify_note};

    #[cfg(feature = "poseidon")]
    fn note(tree_id: u16, leaf_index: u64, commitment_byte: u8) -> VerifiedNote {
        let plain = ConfidentialPlaintext {
            asset_id: 1,
            amount: 4,
            blinding: [2; 32],
            ring_program_id: None,
            records: Vec::new(),
        };
        let expected_owner = [7; 32];
        let published = utxo_commitment(&UtxoPreimage {
            domain: right_align(&[3u8]).unwrap(),
            tree_id: right_align(&tree_id.to_be_bytes()).unwrap(),
            owner: expected_owner,
            asset: right_align(&1u64.to_be_bytes()).unwrap(),
            amount: right_align(&4u64.to_be_bytes()).unwrap(),
            blinding: [2; 32],
            data_hash: [0; 32],
            ring_data_hash: [0; 32],
            ring_program_id: [0; 32],
        })
        .unwrap();
        let _ = commitment_byte;
        verify_note(
            OpenedNote::Confidential(plain),
            tree_id,
            leaf_index,
            expected_owner,
            published,
        )
        .unwrap()
    }

    #[cfg(feature = "poseidon")]
    #[test]
    fn spent_nullifiers_drop_and_non_inclusion_keeps() {
        let n2 = note(1, 2, 0);
        let n3 = note(1, 3, 0);
        let notes = vec![n2.clone(), n3.clone()];
        let mut derived = BTreeMap::new();
        derived.insert(n2.commitment(), [9; 32]);
        let mut spent = BTreeSet::new();
        spent.insert([9; 32]);
        let left = subtract_nullified(&notes, &derived, &spent);
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].leaf_index(), 3);

        let mut unspent = BTreeSet::new();
        unspent.insert([9; 32]);
        let mut both = derived.clone();
        both.insert(n3.commitment(), [8; 32]);
        let kept = retain_non_inclusion(&notes, &both, &unspent);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].leaf_index(), 2);
        assert_eq!(retain_non_inclusion(&notes, &derived, &unspent).len(), 2);
    }
}
