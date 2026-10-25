//! Tree initialization reuses checked Compression geometry and actual PDA derivation.
use crate::{
    CompressionPrograms, CreateTree, CreateTreeV2, PROGRAM, find_tree_authority_with_program,
};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Error, Limits},
};

/// Accounts for initialization. The caller allocates/funds the tree separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TreeAccounts {
    /// Bubblegum deployment.
    pub program: Address,
    /// Already allocated Merkle tree account.
    pub tree: Address,
    /// Paying signer.
    pub payer: Address,
    /// V1 requires this signer. None explicitly uses the payer.
    /// For V2, None emits the program-ID sentinel and lets the program use the payer.
    pub creator: Option<Address>,
    /// Explicit dependency deployments.
    pub programs: CompressionPrograms,
}

impl TreeAccounts {
    /// Constructs canonical accounts with the payer as the default creator.
    #[must_use]
    pub fn canonical(tree: Address, payer: Address) -> Self {
        Self {
            program: PROGRAM,
            tree,
            payer,
            creator: None,
            programs: CompressionPrograms::default(),
        }
    }

    /// Returns the complete Compression allocation for an explicit canopy depth.
    /// # Errors
    /// Rejects invalid supported depth/buffer pairs, canopy and arithmetic overflow.
    pub fn allocation_size(depth: u32, buffer: u32, canopy: u32) -> Result<usize, Error> {
        solana_kit_account_compression::account_size(depth, buffer, canopy)
    }

    /// Creates only the Bubblegum V1 initialization instruction.
    /// # Errors
    /// Rejects invalid tree geometry, PDA derivation, data or account budgets.
    pub fn create_v1(&self, data: &CreateTree, limits: Limits) -> Result<Instruction, Error> {
        Self::allocation_size(data.max_depth, data.max_buffer_size, 0)?;
        data.instruction_with_program(self.program, &self.accounts(false)?, limits)
    }

    /// Creates only the Bubblegum V2 initialization instruction.
    /// # Errors
    /// Rejects invalid tree geometry, PDA derivation, data or account budgets.
    pub fn create_v2(&self, data: &CreateTreeV2, limits: Limits) -> Result<Instruction, Error> {
        Self::allocation_size(data.max_depth, data.max_buffer_size, 0)?;
        data.instruction_with_program(self.program, &self.accounts(true)?, limits)
    }

    fn accounts(&self, optional_creator: bool) -> Result<[AccountMeta; 7], Error> {
        let authority = find_tree_authority_with_program(self.program, self.tree)
            .map_err(|_| Error::InvalidValue)?
            .address();

        let (creator, role) = match (optional_creator, self.creator) {
            (true, None) => (self.program, AccountRole::Readonly),
            (_, creator) => (creator.unwrap_or(self.payer), AccountRole::ReadonlySigner),
        };

        Ok([
            meta(authority, AccountRole::Writable),
            meta(self.tree, AccountRole::Writable),
            meta(self.payer, AccountRole::WritableSigner),
            meta(creator, role),
            meta(self.programs.log_wrapper, AccountRole::Readonly),
            meta(self.programs.compression, AccountRole::Readonly),
            meta(self.programs.system, AccountRole::Readonly),
        ])
    }
}

fn meta(address: Address, role: AccountRole) -> AccountMeta {
    AccountMeta { address, role }
}
