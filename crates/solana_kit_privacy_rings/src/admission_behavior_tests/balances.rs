//! Balances admission regressions.

use super::*;

const SHARED_ASSET_ID: u64 = 7;
const NOTE_AMOUNT: u64 = 40;

#[test]
fn duplicate_leaf_aliases_both_orders_and_conflicts() {
    let owner = [1; 32];
    // One preimage and commitment, two indexer leaf labels.
    let first = verified_confidential(owner, 1, 1, 5, 40, [2; 32]);
    let alias = verified_confidential(owner, 1, 2, 5, 40, [2; 32]);
    assert_eq!(first.commitment(), alias.commitment());
    assert_ne!(first.leaf_index(), alias.leaf_index());

    let expected = vec![AssetBalance {
        asset_id: Some(5),
        mint: None,
        amount: 40,
    }];
    let forward = private_balances(&[first.clone(), alias.clone()], &BTreeSet::new()).unwrap();
    assert_eq!(forward, expected);
    let reverse = private_balances(&[alias, first.clone()], &BTreeSet::new()).unwrap();
    assert_eq!(reverse, expected);
    let clone = private_balances(&[first.clone(), first.clone()], &BTreeSet::new()).unwrap();
    assert_eq!(clone, expected);

    // A different commitment at one leaf still rejects, even when spent.
    let conflict = verified_confidential(owner, 1, 1, 5, 41, [9; 32]);
    assert_conflict(first, conflict, "conflicting leaf");
}

#[test]
fn spent_alias_and_total_overflow() {
    let owner = [1; 32];
    let first = verified_confidential(owner, 1, 1, 5, 10, [2; 32]);
    let alias = verified_confidential(owner, 1, 2, 5, 10, [2; 32]);
    let spent = BTreeSet::from([first.commitment()]);
    let totals = private_balances(&[first, alias], &spent).unwrap();
    assert!(totals.is_empty());

    let a = verified_confidential(owner, 1, 1, 5, u64::MAX, [2; 32]);
    let b = verified_confidential(owner, 1, 2, 5, 1, [3; 32]);
    assert_eq!(
        limit_message(private_balances(&[a, b], &BTreeSet::new()).unwrap_err()),
        "balance overflow"
    );
}

#[test]
fn same_commitment_across_rails_is_a_conflict_in_both_orders() {
    let owner = [3; 32];
    let tree = 2u16;
    let amount = NOTE_AMOUNT;
    let blinding = [4; 32];
    let commitment = confidential_commitment(owner, tree, SHARED_ASSET_ID, amount, blinding);
    let mint = [9; 32];
    let asset = right_align(&SHARED_ASSET_ID.to_be_bytes()).unwrap();

    let confidential = verify_note(
        OpenedNote::Confidential(confidential_plain(amount, SHARED_ASSET_ID, blinding)),
        tree,
        4,
        owner,
        commitment,
        |_| None,
    )
    .expect("confidential admits");
    let proofless = verify_note(
        OpenedNote::Proofless(proofless_plain(owner, mint, amount, blinding)),
        tree,
        5,
        owner,
        commitment,
        |requested| (*requested == mint).then_some(asset),
    )
    .expect("proofless admits");
    assert_eq!(confidential.commitment(), proofless.commitment());

    assert_conflict(confidential, proofless, "conflicting commitment");
}

#[test]
fn same_commitment_with_different_displayed_mints_is_a_conflict() {
    let owner = [3; 32];
    let tree = 2u16;
    let amount = NOTE_AMOUNT;
    let blinding = [4; 32];
    let asset = right_align(&SHARED_ASSET_ID.to_be_bytes()).unwrap();
    let commitment = proofless_commitment(owner, tree, asset, amount, blinding);
    let mint_a = [9; 32];
    let mint_b = [10; 32];

    let first = verify_note(
        OpenedNote::Proofless(proofless_plain(owner, mint_a, amount, blinding)),
        tree,
        1,
        owner,
        commitment,
        |requested| (*requested == mint_a).then_some(asset),
    )
    .expect("first mint admits");
    let second = verify_note(
        OpenedNote::Proofless(proofless_plain(owner, mint_b, amount, blinding)),
        tree,
        2,
        owner,
        commitment,
        |requested| (*requested == mint_b).then_some(asset),
    )
    .expect("second mint admits");
    assert_eq!(first.commitment(), second.commitment());

    assert_conflict(first, second, "conflicting commitment");
}

#[test]
fn equivalent_empty_records_deduplicate_across_leaves() {
    let first = verified_confidential([1; 32], 1, 1, 5, NOTE_AMOUNT, [2; 32]);
    let mut plain = confidential_plain(NOTE_AMOUNT, 5, [2; 32]);
    plain.records = vec![0];
    let alias = verify_note(
        OpenedNote::Confidential(plain),
        1,
        2,
        [1; 32],
        first.commitment(),
        |_| None,
    )
    .unwrap();

    for notes in [[first.clone(), alias.clone()], [alias, first]] {
        let totals = private_balances(&notes, &BTreeSet::new()).unwrap();
        assert_eq!(totals.len(), 1);
        assert_eq!(totals[0].amount, NOTE_AMOUNT);
    }
}
