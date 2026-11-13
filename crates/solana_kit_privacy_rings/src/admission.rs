//! Verified notes and private-balance admission.

use std::collections::{BTreeMap, BTreeSet};

use crate::Error;
use crate::encode::ConfidentialPlaintext;
use crate::viewing::{OpenedNote, ProoflessNote};

/// A note that passed ownership and commitment verification and may enter a
/// private balance. Fields are private so callers cannot fabricate one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedNote {
    tree_id: u16,
    leaf_index: u64,
    commitment: [u8; 32],
    note: OpenedNote,
}

impl VerifiedNote {
    /// State tree id.
    #[must_use]
    pub fn tree_id(&self) -> u16 {
        self.tree_id
    }

    /// Leaf index.
    #[must_use]
    pub fn leaf_index(&self) -> u64 {
        self.leaf_index
    }

    /// Verified commitment that identifies this note.
    #[must_use]
    pub fn commitment(&self) -> [u8; 32] {
        self.commitment
    }

    /// Decrypted or proofless note.
    #[must_use]
    pub fn note(&self) -> &OpenedNote {
        &self.note
    }
}

/// Balance for one admitted asset.
///
/// `asset_id` identifies confidential registry assets. `mint` identifies
/// proofless deposits by their displayed mint bytes. Only one is set for a
/// given balance entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetBalance {
    /// Confidential registry asset id.
    pub asset_id: Option<u64>,
    /// Proofless mint or token address bytes.
    pub mint: Option<[u8; 32]>,
    /// Total amount in base units.
    pub amount: u64,
}

/// Sums verified notes whose commitment is not in `spent`.
///
/// # Errors
/// Rejects conflicting leaf or commitment claims and balance overflow.
pub fn private_balances(
    notes: &[VerifiedNote],
    spent: &BTreeSet<[u8; 32]>,
) -> Result<Vec<AssetBalance>, Error> {
    let mut by_leaf: BTreeMap<(u16, u64), [u8; 32]> = BTreeMap::new();
    let mut seen: BTreeSet<[u8; 32]> = BTreeSet::new();
    let mut totals: BTreeMap<(u8, [u8; 8], [u8; 32]), u64> = BTreeMap::new();

    for note in notes {
        let commitment = note.commitment();
        let leaf = (note.tree_id(), note.leaf_index());
        if let Some(existing) = by_leaf.insert(leaf, commitment) {
            if existing != commitment {
                return Err(Error::Limit("conflicting leaf"));
            }
        }

        if spent.contains(&commitment) {
            continue;
        }

        if !seen.insert(commitment) {
            // Same verified commitment may appear again only through the same
            // leaf claim above; duplicates do not contribute twice.
            continue;
        }

        add_note_total(&mut totals, note.note())?;
    }

    Ok(totals
        .into_iter()
        .map(|((kind, id, mint), amount)| AssetBalance {
            asset_id: (kind == 0).then(|| u64::from_le_bytes(id)),
            mint: (kind == 1).then_some(mint),
            amount,
        })
        .collect())
}

fn add_note_total(
    totals: &mut BTreeMap<(u8, [u8; 8], [u8; 32]), u64>,
    note: &OpenedNote,
) -> Result<(), Error> {
    let (kind, id, mint, amount) = match note {
        OpenedNote::Confidential(plain) => (0, plain.asset_id.to_le_bytes(), [0; 32], plain.amount),
        OpenedNote::Proofless(plain) => (1, [0; 8], plain.asset, plain.amount),
    };

    let entry = totals.entry((kind, id, mint)).or_insert(0);
    *entry = entry
        .checked_add(amount)
        .ok_or(Error::Limit("balance overflow"))?;

    Ok(())
}

/// Verifies a note's owner and recomputes its UTXO commitment.
///
/// `resolve_proofless_asset` supplies the trusted asset field used in the
/// proofless commitment for the note's displayed mint. It is consulted only
/// for [`OpenedNote::Proofless`] notes and must bind the returned field to that
/// mint.
///
/// # Errors
/// Rejects owner, shape, or commitment mismatches and unsupported records.
#[cfg(feature = "poseidon")]
pub fn verify_note<F>(
    note: OpenedNote,
    tree_id: u16,
    leaf_index: u64,
    expected_owner: [u8; 32],
    published_commitment: [u8; 32],
    resolve_proofless_asset: F,
) -> Result<VerifiedNote, Error>
where
    F: FnOnce(&[u8; 32]) -> Option<[u8; 32]>,
{
    match note {
        OpenedNote::Confidential(plain) => verify_confidential_note(
            plain,
            tree_id,
            leaf_index,
            expected_owner,
            published_commitment,
        ),
        OpenedNote::Proofless(plain) => verify_proofless_note(
            plain,
            tree_id,
            leaf_index,
            expected_owner,
            published_commitment,
            resolve_proofless_asset(&plain.asset),
        ),
    }
}

#[cfg(feature = "poseidon")]
fn verify_confidential_note(
    plain: ConfidentialPlaintext,
    tree_id: u16,
    leaf_index: u64,
    expected_owner: [u8; 32],
    published_commitment: [u8; 32],
) -> Result<VerifiedNote, Error> {
    use crate::poseidon::{UtxoPreimage, right_align, utxo_commitment};

    const UTXO_DOMAIN: u8 = 3;

    if plain.ring_program_id.is_some() {
        return Err(Error::Limit("unsupported note shape"));
    }

    if !plain.records.is_empty() && plain.records != [0] {
        return Err(Error::Limit("unsupported note shape"));
    }

    let asset = right_align(&plain.asset_id.to_be_bytes())?;
    let amount = right_align(&plain.amount.to_be_bytes())?;
    let tree = right_align(&tree_id.to_be_bytes())?;
    let domain = right_align(&[UTXO_DOMAIN])?;

    let preimage = UtxoPreimage {
        domain,
        tree_id: tree,
        owner: expected_owner,
        asset,
        amount,
        blinding: plain.blinding,
        data_hash: [0; 32],
        ring_data_hash: [0; 32],
        ring_program_id: [0; 32],
    };
    let commitment = utxo_commitment(&preimage)?;

    if commitment != published_commitment {
        return Err(Error::Limit("commitment mismatch"));
    }

    Ok(VerifiedNote {
        tree_id,
        leaf_index,
        commitment: published_commitment,
        note: OpenedNote::Confidential(plain),
    })
}

#[cfg(feature = "poseidon")]
fn verify_proofless_note(
    plain: ProoflessNote,
    tree_id: u16,
    leaf_index: u64,
    expected_owner: [u8; 32],
    published_commitment: [u8; 32],
    resolved_asset: Option<[u8; 32]>,
) -> Result<VerifiedNote, Error> {
    use crate::poseidon::{UtxoPreimage, right_align, utxo_commitment};

    const UTXO_DOMAIN: u8 = 3;

    if plain.owner != expected_owner {
        return Err(Error::Limit("owner mismatch"));
    }

    if !plain.records.is_empty() {
        return Err(Error::Limit("unsupported note shape"));
    }

    let asset = resolved_asset.ok_or(Error::Limit("unresolved proofless asset"))?;
    let amount = right_align(&plain.amount.to_be_bytes())?;
    let tree = right_align(&tree_id.to_be_bytes())?;
    let domain = right_align(&[UTXO_DOMAIN])?;

    let preimage = UtxoPreimage {
        domain,
        tree_id: tree,
        owner: plain.owner,
        asset,
        amount,
        blinding: plain.blinding,
        data_hash: [0; 32],
        ring_data_hash: [0; 32],
        ring_program_id: [0; 32],
    };
    let commitment = utxo_commitment(&preimage)?;

    if commitment != published_commitment {
        return Err(Error::Limit("commitment mismatch"));
    }

    Ok(VerifiedNote {
        tree_id,
        leaf_index,
        commitment: published_commitment,
        note: OpenedNote::Proofless(plain),
    })
}
