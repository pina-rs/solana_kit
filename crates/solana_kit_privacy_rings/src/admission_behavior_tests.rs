//! Behavior regressions for note admission and private balances.

#[cfg(feature = "poseidon")]
mod poseidon_tests {
    use std::collections::BTreeSet;

    use crate::admission::{AssetBalance, VerifiedNote, private_balances, verify_note};
    use crate::encode::{ConfidentialPlaintext, write_confidential};
    use crate::poseidon::{UtxoPreimage, right_align, utxo_commitment};
    use crate::viewing::{OpenedNote, ProoflessNote, ViewingKey};
    use curve25519_dalek::montgomery::MontgomeryPoint;

    fn confidential_plain(amount: u64, asset_id: u64, blinding: [u8; 32]) -> ConfidentialPlaintext {
        ConfidentialPlaintext {
            asset_id,
            amount,
            blinding,
            ring_program_id: None,
            records: Vec::new(),
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

    fn decode_hex_32(s: &str) -> [u8; 32] {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let mut out = [0u8; 32];
        for i in 0..32 {
            out[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap();
        }
        out
    }

    #[test]
    fn known_real_positive_from_published_fixture() {
        // Independently pinned from crates/solana_kit_privacy_rings/tests/fixtures/transfer-2x2.json
        // output 0: outputTreeId 0x0b, hash, and UTXO preimage fields.
        let owner =
            decode_hex_32("0x060b3e358f59be14ee9dfe71185327c17a02f153b1f02e7a819cad0672b766d9");
        let blinding =
            decode_hex_32("0x19f4bac85c01894eae59dd3b41f37e95bcdf8d34a9b27da0e0e92f4ba9924ac1");
        let commitment =
            decode_hex_32("0x1c2acf5adf6ca59c3cda751f7ff55d63090ce0bb10df50e2cc229f453fbc2fde");
        let plain = ConfidentialPlaintext {
            asset_id: 7,
            amount: 0x69,
            blinding,
            ring_program_id: None,
            records: Vec::new(),
        };
        let verified = verify_note(
            OpenedNote::Confidential(plain),
            0x000b,
            0,
            owner,
            commitment,
            |_| None,
        )
        .expect("fixture commitment verifies");
        assert_eq!(verified.commitment(), commitment);
        assert_eq!(verified.tree_id(), 0x000b);
    }

    #[test]
    fn altered_fields_are_rejected() {
        let owner = [7; 32];
        let tree = 2u16;
        let leaf = 9u64;
        let blinding = [3; 32];
        let commitment = confidential_commitment(owner, tree, 5, 42, blinding);
        let base = confidential_plain(42, 5, blinding);

        let cases = [
            (
                "owner",
                OpenedNote::Confidential(base.clone()),
                tree,
                leaf,
                [8; 32],
                commitment,
            ),
            (
                "amount",
                OpenedNote::Confidential(confidential_plain(43, 5, blinding)),
                tree,
                leaf,
                owner,
                commitment,
            ),
            (
                "asset",
                OpenedNote::Confidential(confidential_plain(42, 6, blinding)),
                tree,
                leaf,
                owner,
                commitment,
            ),
            (
                "blinding",
                OpenedNote::Confidential(confidential_plain(42, 5, [4; 32])),
                tree,
                leaf,
                owner,
                commitment,
            ),
            (
                "tree",
                OpenedNote::Confidential(base.clone()),
                3,
                leaf,
                owner,
                commitment,
            ),
            (
                "commitment",
                OpenedNote::Confidential(base.clone()),
                tree,
                leaf,
                owner,
                [0; 32],
            ),
        ];

        for (label, note, t, l, expected_owner, published) in cases {
            let err =
                verify_note(note, t, l, expected_owner, published, |_| None).expect_err(label);
            let msg = format!("{err:?}");
            assert!(
                msg.contains("commitment mismatch")
                    || msg.contains("unsupported")
                    || msg.contains("owner"),
                "{label}: {msg}"
            );
        }
    }

    #[test]
    fn unsupported_confidential_shapes_are_rejected() {
        let owner = [1; 32];
        let tree = 1u16;
        let leaf = 0u64;
        let blinding = [2; 32];
        let commitment = confidential_commitment(owner, tree, 1, 1, blinding);

        let mut ring = confidential_plain(1, 1, blinding);
        ring.ring_program_id = Some([9; 32]);
        assert!(
            verify_note(
                OpenedNote::Confidential(ring),
                tree,
                leaf,
                owner,
                commitment,
                |_| None
            )
            .is_err()
        );

        let mut data = confidential_plain(1, 1, blinding);
        data.records = vec![0, 0]; // not canonical empty
        assert!(
            verify_note(
                OpenedNote::Confidential(data),
                tree,
                leaf,
                owner,
                commitment,
                |_| None
            )
            .is_err()
        );
    }

    #[test]
    fn canonical_data_free_confidential_record_is_accepted() {
        let owner = [1; 32];
        let tree = 1u16;
        let blinding = [2; 32];
        let commitment = confidential_commitment(owner, tree, 1, 1, blinding);
        let mut plain = confidential_plain(1, 1, blinding);
        plain.records = vec![0]; // canonical/default data marker
        assert!(
            verify_note(
                OpenedNote::Confidential(plain),
                tree,
                0,
                owner,
                commitment,
                |_| None
            )
            .is_ok()
        );
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

    #[test]
    fn proofless_requires_trusted_asset_resolution_and_owner_binding() {
        let owner = [4; 32];
        let mint = [9; 32];
        let resolved = [7; 32];
        let blinding = [5; 32];
        let commitment = proofless_commitment(owner, 0, resolved, 40, blinding);
        let note = OpenedNote::Proofless(ProoflessNote {
            owner,
            blinding,
            asset: mint,
            amount: 40,
            records: Vec::new(),
        });

        // Unresolved mint must not verify.
        assert!(verify_note(note.clone(), 0, 1, owner, commitment, |_| None).is_err());
        // Wrong owner binding must not verify.
        assert!(verify_note(note.clone(), 0, 1, [8; 32], commitment, |_| Some(resolved)).is_err());
        // Correct trusted mapping verifies.
        let ok = verify_note(note, 0, 1, owner, commitment, |m| {
            assert_eq!(*m, mint);
            Some(resolved)
        })
        .unwrap();
        assert_eq!(ok.commitment(), commitment);
        // Displayed mint remains the note mint, not the resolved confidential id.
        if let OpenedNote::Proofless(p) = ok.note() {
            assert_eq!(p.asset, mint);
        } else {
            panic!("expected proofless");
        }
    }

    #[test]
    fn duplicate_leaf_aliases_both_orders_and_conflicts() {
        let owner = [1; 32];
        let n1 = verified_confidential(owner, 1, 7, 5, 10, [2; 32]);
        let n1b = verified_confidential(owner, 1, 7, 5, 10, [2; 32]);
        let n2 = verified_confidential(owner, 1, 8, 5, 3, [3; 32]);

        let totals =
            private_balances(&[n1.clone(), n2.clone(), n1b.clone()], &BTreeSet::new()).unwrap();
        assert_eq!(
            totals,
            vec![AssetBalance {
                asset_id: Some(5),
                mint: None,
                amount: 13
            }]
        );
        let totals_rev = private_balances(&[n2, n1b, n1.clone()], &BTreeSet::new()).unwrap();
        assert_eq!(
            totals_rev,
            vec![AssetBalance {
                asset_id: Some(5),
                mint: None,
                amount: 13
            }]
        );

        // Conflicting commitment at same leaf must fail.
        let conflict = verified_confidential(owner, 1, 7, 5, 11, [9; 32]);
        assert!(private_balances(&[n1.clone(), conflict.clone()], &BTreeSet::new()).is_err());
        // Conflict is detected even when the conflicting note's commitment is marked spent.
        let mut spent = BTreeSet::new();
        spent.insert(conflict.commitment());
        assert!(private_balances(&[n1, conflict], &spent).is_err());
    }

    #[test]
    fn spent_alias_and_total_overflow() {
        let owner = [1; 32];
        let n1 = verified_confidential(owner, 1, 1, 5, 10, [2; 32]);
        let n1_alias = n1.clone();
        let mut spent = BTreeSet::new();
        spent.insert(n1.commitment());
        let totals = private_balances(&[n1_alias], &spent).unwrap();
        assert!(totals.is_empty());

        let a = verified_confidential(owner, 1, 1, 5, u64::MAX, [2; 32]);
        let b = verified_confidential(owner, 1, 2, 5, 1, [3; 32]);
        assert!(private_balances(&[a, b], &BTreeSet::new()).is_err());
    }

    #[test]
    fn verified_note_cannot_be_forged_publicly() {
        // Compile-time/API safety: VerifiedNote has no public constructor and
        // private fields; this test documents the invariant by constructing
        // only through verify_note.
        let owner = [1; 32];
        let note = verified_confidential(owner, 1, 1, 5, 10, [2; 32]);
        let _: [u8; 32] = note.commitment();
        let _: &OpenedNote = note.note();
        // No other construction path is available in this crate's public API.
    }

    #[test]
    fn real_ciphertext_parse_and_tamper_reject_admission() {
        let viewing = ViewingKey::from_bytes([7; 32]);
        let recipient = viewing.public_key().expect("viewing key");
        let owner = [7; 32];
        let plain = confidential_plain(99, 1, [2; 32]);
        let commitment = confidential_commitment(owner, 1, 1, 99, [2; 32]);
        let plaintext = write_confidential(&plain);
        let salt = [3; 16];
        let ephemeral_secret = [9; 32];
        let ephemeral_u = MontgomeryPoint::mul_base_clamped(ephemeral_secret).to_bytes();
        // derive shared/ciphertext via viewing.open_output path expectations by crafting payload
        // (reuse crate internals indirectly through a real open)
        let ciphertext = {
            use aes::Aes256;
            use ctr::Ctr32BE;
            use ctr::cipher::{KeyIvInit, StreamCipher};
            use curve25519_dalek::montgomery::MontgomeryPoint as Mp;
            use hkdf::Hkdf;
            use sha2::Sha256;
            type Aes256Ctr = Ctr32BE<Aes256>;
            let peer = {
                let mut s = [0u8; 32];
                s.copy_from_slice(&recipient[1..]);
                s
            };
            let shared = Mp(peer).mul_clamped(ephemeral_secret).to_bytes();
            let mut ikm = [0u8; 32 + 33 + 33];
            ikm[..32].copy_from_slice(&shared);
            let mut eph_tag = [4u8; 33];
            eph_tag[1..].copy_from_slice(&ephemeral_u);
            ikm[32..65].copy_from_slice(&eph_tag);
            ikm[65..].copy_from_slice(&recipient);
            let mut okm = [0u8; 44];
            Hkdf::<Sha256>::new(None, &ikm)
                .expand_multi_info(
                    &[b"TSPP/hpke/", b"TSPP/tx", &salt, &1u32.to_be_bytes()],
                    &mut okm,
                )
                .unwrap();
            let mut key = [0u8; 32];
            let mut nonce = [0u8; 12];
            key.copy_from_slice(&okm[..32]);
            nonce.copy_from_slice(&okm[32..]);
            let mut counter = [0u8; 16];
            counter[..12].copy_from_slice(&nonce);
            counter[15] = 2;
            let mut cipher = Aes256Ctr::new((&key).into(), (&counter).into());
            let mut buf = plaintext.clone();
            cipher.apply_keystream(&mut buf);
            buf
        };
        let mut eph_tag = [4u8; 33];
        eph_tag[1..].copy_from_slice(&ephemeral_u);
        let mut body = recipient.to_vec();
        body.extend(ciphertext.clone());
        let mut payload = vec![1u8];
        payload.extend(u32::try_from(1 + body.len()).unwrap().to_le_bytes());
        payload.push(3);
        payload.extend(body);
        let opened = viewing
            .open_output(&eph_tag, &salt, 1, &payload)
            .expect("real parse");
        let verified = verify_note(opened, 1, 1, owner, commitment, |_| None).expect("admits");
        assert_eq!(verified.commitment(), commitment);

        // Tamper ciphertext -> decrypted plaintext changes -> admission rejects.
        let mut tampered_ct = ciphertext;
        tampered_ct[0] ^= 1;
        let mut body2 = recipient.to_vec();
        body2.extend(tampered_ct);
        let mut payload2 = vec![1u8];
        payload2.extend(u32::try_from(1 + body2.len()).unwrap().to_le_bytes());
        payload2.push(3);
        payload2.extend(body2);
        let opened2 = viewing
            .open_output(&eph_tag, &salt, 1, &payload2)
            .expect("parse still succeeds for unauthenticated CTR");
        assert!(verify_note(opened2, 1, 1, owner, commitment, |_| None).is_err());
    }
}
