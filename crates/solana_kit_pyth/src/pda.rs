//! Receiver and guardian-set derivation reuse the optional existing Kit crypto.
use crate::{CONFIG_PDA_SEED, GUARDIAN_SET_PDA_SEED, ReceiverAccounts, TREASURY_PDA_SEED};
use solana_kit_core::{
    Address,
    crypto::{PdaError, ProgramDerivedAddress, find_program_address},
};

/// Derives receiver configuration under the caller's selected program.
///
/// # Errors
/// Returns the core PDA failure if no viable bump exists.
pub fn config_address(program: Address) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[CONFIG_PDA_SEED], &program)
}

/// Derives a receiver treasury with one unsigned treasury byte.
///
/// # Errors
/// Returns the core PDA failure if no viable bump exists.
pub fn treasury_address(
    treasury_id: u8,
    program: Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[TREASURY_PDA_SEED, &[treasury_id]], &program)
}

/// Derives a Wormhole guardian set with the exact big-endian u32 seed.
///
/// # Errors
/// Returns the core PDA failure if no viable bump exists.
pub fn guardian_set_address(
    index: u32,
    program: Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[GUARDIAN_SET_PDA_SEED, &index.to_be_bytes()], &program)
}

impl ReceiverAccounts {
    /// Derives receiver accounts, defaulting write authority to the payer.
    /// Guardian and encoded-VAA identities remain separate instruction inputs.
    ///
    /// # Errors
    /// Propagates configuration or treasury PDA failures without starting I/O.
    pub fn derive(
        payer: Address,
        price_update_account: Address,
        treasury_id: u8,
        program: Address,
    ) -> Result<Self, PdaError> {
        Ok(Self {
            program_address: program,
            config: config_address(program)?.address(),
            treasury: treasury_address(treasury_id, program)?.address(),
            payer,
            price_update_account,
            write_authority: payer,
        })
    }
}
