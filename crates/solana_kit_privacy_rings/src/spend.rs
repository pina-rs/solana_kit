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
        .filter(|note| {
            !nullifier_of
                .get(&note.commitment())
                .is_some_and(|nullifier| spent.contains(nullifier))
        })
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
        .filter(|note| match nullifier_of.get(&note.commitment()) {
            Some(nullifier) => unspent.contains(nullifier),
            None => true,
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "poseidon")]
    use super::*;
    #[cfg(feature = "poseidon")]
    use crate::admission::verify_note;
    #[cfg(feature = "poseidon")]
    use crate::encode::ConfidentialPlaintext;
    #[cfg(feature = "poseidon")]
    use crate::poseidon::{UtxoPreimage, right_align, utxo_commitment};
    #[cfg(feature = "poseidon")]
    use crate::viewing::OpenedNote;

    #[cfg(feature = "poseidon")]
    fn note(
        tree_id: u16,
        leaf_index: u64,
        blinding_byte: u8,
        amount: u64,
    ) -> crate::admission::VerifiedNote {
        let plain = ConfidentialPlaintext {
            asset_id: 1,
            amount,
            blinding: [blinding_byte; 32],
            ring_program_id: None,
            records: Vec::new(),
        };
        let expected_owner = [7; 32];
        let published = utxo_commitment(&UtxoPreimage {
            domain: right_align(&[3u8]).unwrap(),
            tree_id: right_align(&tree_id.to_be_bytes()).unwrap(),
            owner: expected_owner,
            asset: right_align(&1u64.to_be_bytes()).unwrap(),
            amount: right_align(&amount.to_be_bytes()).unwrap(),
            blinding: [blinding_byte; 32],
            data_hash: [0; 32],
            ring_data_hash: [0; 32],
            ring_program_id: [0; 32],
        })
        .unwrap();
        verify_note(
            OpenedNote::Confidential(plain),
            tree_id,
            leaf_index,
            expected_owner,
            published,
            |_| None,
        )
        .unwrap()
    }

    #[cfg(feature = "poseidon")]
    #[test]
    fn spent_nullifiers_drop_and_non_inclusion_keeps() {
        let n2 = note(1, 2, 2, 4);
        let n3 = note(1, 3, 3, 5);
        assert_ne!(n2.commitment(), n3.commitment());
        let notes = vec![n2.clone(), n3.clone()];
        let mut derived = BTreeMap::new();
        derived.insert(n2.commitment(), [9; 32]);
        let mut spent = BTreeSet::new();
        spent.insert([9; 32]);
        let left = subtract_nullified(&notes, &derived, &spent);
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].commitment(), n3.commitment());

        let mut unspent = BTreeSet::new();
        unspent.insert([9; 32]);
        let mut both = derived.clone();
        both.insert(n3.commitment(), [8; 32]);
        let kept = retain_non_inclusion(&notes, &both, &unspent);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].commitment(), n2.commitment());
        // Unknown commitment remains unprovable and is kept.
        assert_eq!(retain_non_inclusion(&notes, &derived, &unspent).len(), 2);
    }

    #[cfg(feature = "poseidon")]
    #[test]
    fn duplicate_aliases_filter_by_commitment() {
        let n2 = note(1, 2, 2, 4);
        // The same commitment published at a different leaf is still the same
        // note: filtering keys on the commitment, not the indexer leaf label.
        let alias = note(1, 3, 2, 4);
        assert_eq!(n2.commitment(), alias.commitment());
        assert_ne!(n2.leaf_index(), alias.leaf_index());
        let notes = vec![n2.clone(), alias];
        let mut derived = BTreeMap::new();
        derived.insert(n2.commitment(), [9; 32]);
        let spent = BTreeSet::from([[9; 32]]);
        assert!(subtract_nullified(&notes, &derived, &spent).is_empty());
        let unspent = BTreeSet::from([[8; 32]]);
        assert!(retain_non_inclusion(&notes, &derived, &unspent).is_empty());
    }
}
