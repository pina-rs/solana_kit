//! Confidential admission regressions.

use super::*;

#[test]
fn known_real_positive_from_published_fixture() {
    // Independently pinned from crates/solana_kit_privacy_rings/tests/fixtures/transfer-2x2.json
    // output 0: outputTreeId 0x0b, hash, and UTXO preimage fields.
    let owner = decode_hex_32("0x060b3e358f59be14ee9dfe71185327c17a02f153b1f02e7a819cad0672b766d9");
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
fn altered_confidential_owner_rejects() {
    let mut case = AdmissionCase::confidential();
    case.owner = [8; 32];
    case.reject("commitment mismatch");
}

#[test]
fn altered_confidential_amount_rejects() {
    let mut case = AdmissionCase::confidential();

    if let OpenedNote::Confidential(plain) = &mut case.note {
        plain.amount += 1;
    }

    case.reject("commitment mismatch");
}

#[test]
fn altered_confidential_asset_rejects() {
    let mut case = AdmissionCase::confidential();

    if let OpenedNote::Confidential(plain) = &mut case.note {
        plain.asset_id += 1;
    }

    case.reject("commitment mismatch");
}

#[test]
fn altered_confidential_blinding_rejects() {
    let mut case = AdmissionCase::confidential();

    if let OpenedNote::Confidential(plain) = &mut case.note {
        plain.blinding = [6; 32];
    }

    case.reject("commitment mismatch");
}

#[test]
fn altered_confidential_tree_rejects() {
    let mut case = AdmissionCase::confidential();
    case.tree = 1;
    case.reject("commitment mismatch");
}

#[test]
fn altered_confidential_commitment_rejects() {
    let mut case = AdmissionCase::confidential();
    case.commitment = [0; 32];
    case.reject("commitment mismatch");
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
    let error = verify_note(
        OpenedNote::Confidential(ring),
        tree,
        leaf,
        owner,
        commitment,
        |_| None,
    )
    .expect_err("ring program");
    assert_eq!(limit_message(error), "unsupported note shape");

    let mut data = confidential_plain(1, 1, blinding);
    data.records = vec![0, 0];
    let error = verify_note(
        OpenedNote::Confidential(data),
        tree,
        leaf,
        owner,
        commitment,
        |_| None,
    )
    .expect_err("application records");
    assert_eq!(limit_message(error), "unsupported note shape");
}

#[test]
fn canonical_data_free_confidential_records_are_accepted() {
    let owner = [1; 32];
    let tree = 1u16;
    let blinding = [2; 32];
    let commitment = confidential_commitment(owner, tree, 1, 1, blinding);

    // The wire representation writes one zero byte; a caller may hold the
    // value as an empty vector. Both mean "no application records".
    for records in [Vec::new(), vec![0]] {
        let mut plain = confidential_plain(1, 1, blinding);
        plain.records = records;
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
}
