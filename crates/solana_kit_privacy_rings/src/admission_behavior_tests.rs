//! Public behavior regressions and shared note fixtures.

use std::collections::BTreeSet;

use curve25519_dalek::montgomery::MontgomeryPoint;

use crate::Error;
use crate::admission::{AssetBalance, VerifiedNote, private_balances, verify_note};
use crate::encode::{ConfidentialPlaintext, write_confidential};
use crate::poseidon::{UtxoPreimage, right_align, utxo_commitment};
use crate::viewing::{OpenedNote, ProoflessNote, ViewingKey};

/// Six borsh `Option` tags, all `None`.
const CANONICAL_TAIL: [u8; 6] = [0; 6];

fn confidential_plain(amount: u64, asset_id: u64, blinding: [u8; 32]) -> ConfidentialPlaintext {
    ConfidentialPlaintext {
        asset_id,
        amount,
        blinding,
        ring_program_id: None,
        records: Vec::new(),
    }
}

fn proofless_plain(
    owner: [u8; 32],
    mint: [u8; 32],
    amount: u64,
    blinding: [u8; 32],
) -> ProoflessNote {
    ProoflessNote {
        owner,
        blinding,
        asset: mint,
        amount,
        records: CANONICAL_TAIL.to_vec(),
    }
}

fn confidential_commitment(
    owner: [u8; 32],
    tree_id: u16,
    asset_id: u64,
    amount: u64,
    blinding: [u8; 32],
) -> [u8; 32] {
    utxo_commitment(&UtxoPreimage {
        domain: right_align(&[3u8]).unwrap(),
        tree_id: right_align(&tree_id.to_be_bytes()).unwrap(),
        owner,
        asset: right_align(&asset_id.to_be_bytes()).unwrap(),
        amount: right_align(&amount.to_be_bytes()).unwrap(),
        blinding,
        data_hash: [0; 32],
        ring_data_hash: [0; 32],
        ring_program_id: [0; 32],
    })
    .unwrap()
}

fn proofless_commitment(
    owner: [u8; 32],
    tree_id: u16,
    asset_field: [u8; 32],
    amount: u64,
    blinding: [u8; 32],
) -> [u8; 32] {
    utxo_commitment(&UtxoPreimage {
        domain: right_align(&[3u8]).unwrap(),
        tree_id: right_align(&tree_id.to_be_bytes()).unwrap(),
        owner,
        asset: asset_field,
        amount: right_align(&amount.to_be_bytes()).unwrap(),
        blinding,
        data_hash: [0; 32],
        ring_data_hash: [0; 32],
        ring_program_id: [0; 32],
    })
    .unwrap()
}

fn verified_confidential(
    owner: [u8; 32],
    tree_id: u16,
    leaf_index: u64,
    asset_id: u64,
    amount: u64,
    blinding: [u8; 32],
) -> VerifiedNote {
    let commitment = confidential_commitment(owner, tree_id, asset_id, amount, blinding);
    verify_note(
        OpenedNote::Confidential(confidential_plain(amount, asset_id, blinding)),
        tree_id,
        leaf_index,
        owner,
        commitment,
        |_| None,
    )
    .unwrap()
}

fn limit_message(error: Error) -> &'static str {
    match error {
        Error::Limit(message) => message,
        other => panic!("expected Error::Limit, got {other:?}"),
    }
}

fn framed(scheme: u8, fields: &[u8]) -> Vec<u8> {
    let mut body = vec![scheme];
    body.extend_from_slice(fields);
    let mut payload = vec![0u8];
    payload.extend(u32::try_from(body.len()).expect("length").to_le_bytes());
    payload.extend(body);
    payload
}

fn decode_hex_32(s: &str) -> [u8; 32] {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap();
    }
    out
}

mod balances;
mod ciphertext;
mod confidential;
mod proofless;

struct AdmissionCase {
    note: OpenedNote,
    tree: u16,
    owner: [u8; 32],
    commitment: [u8; 32],
    asset: Option<[u8; 32]>,
}

impl AdmissionCase {
    fn confidential() -> Self {
        let owner = [4; 32];
        Self {
            note: OpenedNote::Confidential(confidential_plain(40, 7, [5; 32])),
            tree: 0,
            owner,
            commitment: confidential_commitment(owner, 0, 7, 40, [5; 32]),
            asset: None,
        }
    }

    fn proofless() -> Self {
        let mut case = Self::confidential();
        case.note = OpenedNote::Proofless(proofless_plain(case.owner, [9; 32], 40, [5; 32]));
        case.asset = Some(right_align(&7u64.to_be_bytes()).unwrap());
        case
    }

    fn reject(self, expected: &str) {
        let error = verify_note(
            self.note,
            self.tree,
            1,
            self.owner,
            self.commitment,
            |mint| (*mint == [9; 32]).then_some(self.asset).flatten(),
        )
        .expect_err(expected);
        assert_eq!(limit_message(error), expected);
    }
}

fn assert_conflict(first: VerifiedNote, second: VerifiedNote, expected: &str) {
    for notes in [
        [first.clone(), second.clone()],
        [second.clone(), first.clone()],
    ] {
        for spent in [
            BTreeSet::new(),
            BTreeSet::from([first.commitment(), second.commitment()]),
        ] {
            let error = private_balances(&notes, &spent).expect_err(expected);
            assert_eq!(limit_message(error), expected);
        }
    }
}
