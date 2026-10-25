//! Distinct inputs and high-width nonces detect missing/reordered seed domains.
#[cfg(feature = "pda")]
use solana_kit_core::{Address, crypto::find_program_address};
#[cfg(feature = "pda")]
use solana_kit_mpl_bubblegum::*;

#[cfg(feature = "pda")]
pub fn complete_pda_recipes() {
    const CANONICAL_NONCE: u64 = 9;
    let tree = Address::from_bytes([1; 32]);
    let custom = Address::from_bytes([2; 32]);

    for program in [PROGRAM, custom] {
        explicit_pda_recipes(program, tree);
    }

    assert_eq!(
        find_tree_authority(tree).unwrap(),
        find_tree_authority_with_program(PROGRAM, tree).unwrap()
    );
    assert_eq!(
        find_bubblegum_signer().unwrap(),
        find_bubblegum_signer_with_program(PROGRAM).unwrap()
    );
    assert_eq!(
        find_leaf_asset_id(tree, CANONICAL_NONCE).unwrap(),
        find_leaf_asset_id_with_program(PROGRAM, tree, CANONICAL_NONCE).unwrap()
    );
}

#[cfg(feature = "pda")]
fn explicit_pda_recipes(program: Address, tree: Address) {
    let nonce = 0xffee_ddcc_bbaa_9988u64;
    assert_eq!(
        find_tree_authority_with_program(program, tree).unwrap(),
        find_program_address(&[tree.as_bytes()], &program).unwrap()
    );
    assert_eq!(
        find_bubblegum_signer_with_program(program).unwrap(),
        find_program_address(&[b"bubblegum_signer"], &program).unwrap()
    );
    let leaf = find_leaf_asset_id_with_program(program, tree, nonce).unwrap();
    assert_eq!(
        leaf,
        find_program_address(&[b"asset", tree.as_bytes(), &nonce.to_le_bytes()], &program).unwrap()
    );
    assert_ne!(
        leaf,
        find_program_address(&[tree.as_bytes(), &nonce.to_le_bytes()], &program).unwrap()
    );
    assert_ne!(
        leaf,
        find_leaf_asset_id_with_program(program, tree, nonce & 0xffff_ffff).unwrap()
    );

    assert_eq!(
        find_voucher_with_program(program, tree, nonce).unwrap(),
        find_program_address(
            &[b"voucher", tree.as_bytes(), &nonce.to_le_bytes()],
            &program
        )
        .unwrap()
    );
    assert_ne!(
        leaf,
        find_voucher_with_program(program, tree, nonce).unwrap()
    );
    assert_eq!(
        find_mpl_core_cpi_signer_with_program(program).unwrap(),
        find_program_address(&[b"mpl_core_cpi_signer"], &program).unwrap()
    );
}

#[cfg(feature = "pda")]
#[test]
fn canonical_and_explicit_pda_domains() {
    complete_pda_recipes();
}
