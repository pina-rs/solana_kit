//! Associated-address derivation through the optional existing core crypto.

use crate::PROGRAM;
use solana_kit_core::{
    Address,
    crypto::{PdaError, ProgramDerivedAddress, find_program_address},
};

/// The exact ordered seeds used by the associated-address derivation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssociatedTokenSeeds {
    /// Owner wallet or associated account.
    pub owner: Address,
    /// Token program identity, including Token-2022 or a caller-selected program.
    pub token_program: Address,
    /// Token mint.
    pub mint: Address,
}

impl AssociatedTokenSeeds {
    /// Finds the highest valid bump under the canonical ATA program.
    ///
    /// # Errors
    /// Returns NoViableBump if all 256 candidates lie on the curve.
    pub fn find(self) -> Result<ProgramDerivedAddress, PdaError> {
        self.find_with_program(PROGRAM)
    }

    /// Finds the highest valid bump with an explicit caller-selected ATA program.
    ///
    /// # Errors
    /// Returns NoViableBump if all 256 candidates lie on the curve.
    pub fn find_with_program(self, program: Address) -> Result<ProgramDerivedAddress, PdaError> {
        find_program_address(
            &[
                self.owner.as_bytes(),
                self.token_program.as_bytes(),
                self.mint.as_bytes(),
            ],
            &program,
        )
    }
}
