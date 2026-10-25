//! PDA recipes use fixed-width integers and never substitute a tree for its authority.
use crate::PROGRAM;
use solana_kit_core::{
    Address,
    crypto::{PdaError, ProgramDerivedAddress, find_program_address},
};

/// Derives the tree configuration authority from the complete tree address.
/// # Errors
/// Preserves the core PDA derivation failure.
pub fn find_tree_authority(tree: Address) -> Result<ProgramDerivedAddress, PdaError> {
    find_tree_authority_with_program(PROGRAM, tree)
}

/// Derives the same authority for an explicitly selected deployment.
/// # Errors
/// Preserves the core PDA derivation failure.
pub fn find_tree_authority_with_program(
    program: Address,
    tree: Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[tree.as_bytes()], &program)
}

/// Retains Dart's compatibility signer recipe. V2 Core CPI uses `find_mpl_core_cpi_signer`.
/// # Errors
/// Preserves the core PDA derivation failure.
pub fn find_bubblegum_signer() -> Result<ProgramDerivedAddress, PdaError> {
    find_bubblegum_signer_with_program(PROGRAM)
}

/// Derives a signer for an explicit deployment.
/// # Errors
/// Preserves the core PDA derivation failure.
pub fn find_bubblegum_signer_with_program(
    program: Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[b"bubblegum_signer"], &program)
}

/// Derives the actual asset identity. The asset domain prefix is mandatory.
/// # Errors
/// Preserves the core PDA derivation failure.
pub fn find_leaf_asset_id(tree: Address, nonce: u64) -> Result<ProgramDerivedAddress, PdaError> {
    find_leaf_asset_id_with_program(PROGRAM, tree, nonce)
}

/// Derives the asset for an explicit deployment and full-width nonce.
/// # Errors
/// Preserves the core PDA derivation failure.
pub fn find_leaf_asset_id_with_program(
    program: Address,
    tree: Address,
    nonce: u64,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[b"asset", tree.as_bytes(), &nonce.to_le_bytes()], &program)
}

/// Derives the redemption voucher identity for a tree and full-width nonce.
/// # Errors
/// Preserves the core PDA derivation failure.
pub fn find_voucher(tree: Address, nonce: u64) -> Result<ProgramDerivedAddress, PdaError> {
    find_voucher_with_program(PROGRAM, tree, nonce)
}

/// Derives the voucher for an explicitly selected deployment.
/// # Errors
/// Preserves the core PDA derivation failure.
pub fn find_voucher_with_program(
    program: Address,
    tree: Address,
    nonce: u64,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(
        &[b"voucher", tree.as_bytes(), &nonce.to_le_bytes()],
        &program,
    )
}

/// Derives the actual Core CPI signer used by V2 collection operations.
/// # Errors
/// Preserves the core PDA derivation failure.
pub fn find_mpl_core_cpi_signer() -> Result<ProgramDerivedAddress, PdaError> {
    find_mpl_core_cpi_signer_with_program(PROGRAM)
}

/// Derives the Core CPI signer for an explicit Bubblegum deployment.
/// # Errors
/// Preserves the core PDA derivation failure.
pub fn find_mpl_core_cpi_signer_with_program(
    program: Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[b"mpl_core_cpi_signer"], &program)
}
