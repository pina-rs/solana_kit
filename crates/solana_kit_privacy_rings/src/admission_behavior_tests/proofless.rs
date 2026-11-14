//! Proofless admission regressions.

use super::*;

#[test]
fn altered_proofless_owner_rejects() {
    let mut case = AdmissionCase::proofless();
    case.owner = [8; 32];
    case.reject("owner mismatch");
}

#[test]
fn altered_proofless_amount_rejects() {
    let mut case = AdmissionCase::proofless();

    if let OpenedNote::Proofless(plain) = &mut case.note {
        plain.amount += 1;
    }

    case.reject("commitment mismatch");
}

#[test]
fn altered_proofless_blinding_rejects() {
    let mut case = AdmissionCase::proofless();

    if let OpenedNote::Proofless(plain) = &mut case.note {
        plain.blinding = [6; 32];
    }

    case.reject("commitment mismatch");
}

#[test]
fn altered_proofless_tree_rejects() {
    let mut case = AdmissionCase::proofless();
    case.tree = 1;
    case.reject("commitment mismatch");
}

#[test]
fn altered_proofless_commitment_rejects() {
    let mut case = AdmissionCase::proofless();
    case.commitment = [0; 32];
    case.reject("commitment mismatch");
}

#[test]
fn altered_proofless_missing_asset_rejects() {
    let mut case = AdmissionCase::proofless();
    case.asset = None;
    case.reject("unresolved proofless asset");
}

#[test]
fn altered_proofless_foreign_asset_rejects() {
    let mut case = AdmissionCase::proofless();
    case.asset = Some([8; 32]);
    case.reject("commitment mismatch");
}

#[test]
fn altered_proofless_mint_rejects() {
    let mut case = AdmissionCase::proofless();

    if let OpenedNote::Proofless(plain) = &mut case.note {
        plain.asset = [10; 32];
    }

    case.reject("unresolved proofless asset");
}

#[test]
fn non_canonical_proofless_tail_is_retained_then_rejected() {
    let owner = [4; 32];
    let mint = [9; 32];
    let asset = right_align(&7u64.to_be_bytes()).unwrap();
    let blinding = [5; 32];

    let commitment = proofless_commitment(owner, 0, asset, 40, blinding);
    let tails = [
        vec![],
        vec![0; 5],
        vec![0; 7],
        vec![2, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0, 1],
    ];

    for tail in tails {
        let mut fields = Vec::new();
        fields.extend(owner);
        fields.extend(blinding);
        fields.extend(mint);
        const AMOUNT: u64 = 40;
        fields.extend(AMOUNT.to_le_bytes());
        fields.extend(&tail);

        let payload = framed(0, &fields);
        let opened = ViewingKey::from_bytes([1; 32])
            .open_output(&[4; 33], &[0; 16], 0, &payload)
            .expect("raw tail stays parseable");

        assert!(matches!(&opened, OpenedNote::Proofless(note) if note.records == tail));

        let error = verify_note(opened, 0, 1, owner, commitment, |requested| {
            (*requested == mint).then_some(asset)
        })
        .expect_err("non-canonical tail");
        assert_eq!(limit_message(error), "unsupported note shape");
    }
}

#[test]
fn proofless_matches_pinned_fixture_with_trusted_mint_mapping() {
    let owner = decode_hex_32("0x060b3e358f59be14ee9dfe71185327c17a02f153b1f02e7a819cad0672b766d9");
    let blinding =
        decode_hex_32("0x19f4bac85c01894eae59dd3b41f37e95bcdf8d34a9b27da0e0e92f4ba9924ac1");
    let commitment =
        decode_hex_32("0x1c2acf5adf6ca59c3cda751f7ff55d63090ce0bb10df50e2cc229f453fbc2fde");
    let fixture_asset = right_align(&7u64.to_be_bytes()).unwrap();
    let mint = [9; 32];

    let mut fields = Vec::new();
    fields.extend(owner);
    fields.extend(blinding);
    fields.extend(mint);

    fields.extend(0x69u64.to_le_bytes());
    fields.extend(CANONICAL_TAIL);

    let payload = framed(0, &fields);
    let opened = ViewingKey::from_bytes([1; 32])
        .open_output(&[4; 33], &[0; 16], 0, &payload)
        .expect("canonical tail decodes");
    let verified = verify_note(opened, 0x000b, 0, owner, commitment, |requested| {
        (*requested == mint).then_some(fixture_asset)
    })
    .expect("fixture commitment verifies through a trusted mapping");
    assert_eq!(verified.commitment(), commitment);
    assert!(matches!(
        verified.note(),
        OpenedNote::Proofless(note) if note.asset == mint
            && note.records.as_slice() == CANONICAL_TAIL
    ));
}
