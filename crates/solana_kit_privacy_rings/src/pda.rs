//! PDA helpers. Seeds match zolana v0.8.0 `pda.rs` and the user-registry crate.

use solana_kit_core::Address;
use solana_kit_core::crypto::{ProgramDerivedAddress, find_program_address};

use crate::Error;
use crate::address::{
    NULLIFIER_SEED, SHIELDED_POOL_PROGRAM, SOL_INTERFACE_SEED, SPL_INTERFACE_SEED, TREE_SEED,
    USER_RECORD_SEED, USER_REGISTRY_PROGRAM,
};

/// State tree PDA for `tree_id`. Tree 0 is [`crate::STATE_TREE`] on the deployment.
///
/// # Errors
/// Rejects an on-curve hash, which this seed schema does not produce.
pub fn state_tree(tree_id: u16) -> Result<ProgramDerivedAddress, Error> {
    find_program_address(&[TREE_SEED, &tree_id.to_le_bytes()], &SHIELDED_POOL_PROGRAM)
        .map_err(Error::Pda)
}

/// SPL interface PDA for `mint`.
///
/// # Errors
/// Rejects an on-curve hash.
pub fn spl_interface(mint: &Address) -> Result<ProgramDerivedAddress, Error> {
    find_program_address(
        &[SPL_INTERFACE_SEED, mint.as_bytes()],
        &SHIELDED_POOL_PROGRAM,
    )
    .map_err(Error::Pda)
}

/// Nullifier PDA under `tree`.
///
/// # Errors
/// Rejects an on-curve hash.
pub fn nullifier_account(
    tree: &Address,
    nullifier: &[u8; 32],
) -> Result<ProgramDerivedAddress, Error> {
    find_program_address(
        &[NULLIFIER_SEED, tree.as_bytes(), nullifier],
        &SHIELDED_POOL_PROGRAM,
    )
    .map_err(Error::Pda)
}

/// User-registry record PDA for a Solana owner.
///
/// # Errors
/// Rejects an on-curve hash.
pub fn user_record(owner: &Address) -> Result<ProgramDerivedAddress, Error> {
    find_program_address(
        &[USER_RECORD_SEED, owner.as_bytes()],
        &USER_REGISTRY_PROGRAM,
    )
    .map_err(Error::Pda)
}

/// SOL interface PDA. The published account is [`crate::SOL_INTERFACE`], bump 248.
///
/// # Errors
/// Rejects an on-curve hash.
pub fn sol_interface() -> Result<ProgramDerivedAddress, Error> {
    find_program_address(&[SOL_INTERFACE_SEED, &[0]], &SHIELDED_POOL_PROGRAM).map_err(Error::Pda)
}

#[cfg(test)]
mod tests {
    use super::{sol_interface, state_tree};
    use crate::{CPI_AUTHORITY, CPI_AUTHORITY_BUMP, SOL_INTERFACE, SOL_INTERFACE_BUMP, STATE_TREE};
    use solana_kit_core::crypto::find_program_address;

    #[test]
    fn published_pdas_match_canonical_bump_search() {
        let sol = sol_interface().expect("sol interface");
        assert_eq!(sol.address(), SOL_INTERFACE);
        assert_eq!(sol.bump(), SOL_INTERFACE_BUMP);
        let tree = state_tree(0).expect("tree 0");
        assert_eq!(tree.address(), STATE_TREE);
        let authority = find_program_address(&[b"cpi_authority"], &crate::SHIELDED_POOL_PROGRAM)
            .expect("cpi authority");
        assert_eq!(authority.address(), CPI_AUTHORITY);
        assert_eq!(authority.bump(), CPI_AUTHORITY_BUMP);
    }
}
