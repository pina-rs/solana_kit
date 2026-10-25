//! Complete V1 leaf operations choose an actual signer and preserve caller proof data.
use crate::{Burn, Delegate, Node, Transfer};
use alloc::vec::Vec;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Error, Limits},
};

/// Canonical program dependencies, with explicit override fields for alternate deployments.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompressionPrograms {
    /// Noop event-log wrapper.
    pub log_wrapper: Address,
    /// SPL Account Compression deployment.
    pub compression: Address,
    /// System program deployment.
    pub system: Address,
}

impl Default for CompressionPrograms {
    fn default() -> Self {
        Self {
            log_wrapper: Address::from_bytes([
                11, 188, 15, 192, 187, 71, 202, 47, 116, 196, 17, 46, 148, 171, 19, 207, 163, 198,
                52, 229, 220, 23, 234, 203, 3, 205, 26, 35, 205, 126, 120, 124,
            ]),
            compression: Address::from_bytes([
                9, 42, 19, 238, 149, 196, 28, 186, 8, 166, 127, 90, 198, 126, 141, 247, 225, 218,
                17, 98, 94, 29, 100, 19, 127, 143, 79, 35, 131, 3, 127, 20,
            ]),
            system: Address::from_bytes([0; 32]),
        }
    }
}

/// An actual signing authority for owner-or-delegate operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeafAuthority {
    /// The owner signs the instruction.
    Owner,
    /// The existing delegate signs the instruction.
    Delegate,
}

impl LeafAuthority {
    fn roles(self) -> (AccountRole, AccountRole) {
        match self {
            Self::Owner => (AccountRole::ReadonlySigner, AccountRole::Readonly),
            Self::Delegate => (AccountRole::Readonly, AccountRole::ReadonlySigner),
        }
    }
}

/// Explicit leaf identity and deployment. No account is inferred from a fake tree authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeafAccounts {
    /// Bubblegum deployment selected by the caller.
    pub program: Address,
    /// Derived tree authority, distinct from the Merkle tree account.
    pub tree_authority: Address,
    /// Owner currently committed to the leaf.
    pub owner: Address,
    /// Delegate currently committed to the leaf.
    pub delegate: Address,
    /// Mutable Merkle tree account.
    pub tree: Address,
    /// Explicit dependency programs.
    pub programs: CompressionPrograms,
}

impl LeafAccounts {
    /// Builds canonical accounts by actually deriving the authority.
    /// # Errors
    /// Preserves PDA derivation failure.
    #[cfg(feature = "pda")]
    pub fn derive(
        tree: Address,
        owner: Address,
        delegate: Option<Address>,
    ) -> Result<Self, solana_kit_core::crypto::PdaError> {
        Ok(Self {
            program: crate::PROGRAM,
            tree_authority: crate::find_tree_authority(tree)?.address(),
            owner,
            delegate: delegate.unwrap_or(owner),
            tree,
            programs: CompressionPrograms::default(),
        })
    }

    /// Builds a burn requiring either the actual owner or delegate to sign.
    /// # Errors
    /// Preserves complete data, account and caller-budget errors.
    pub fn burn(
        &self,
        data: &Burn,
        signer: LeafAuthority,
        proof: &[Node],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        data.instruction_with_program(
            self.program,
            &self.accounts(signer, None, proof, limits)?,
            limits,
        )
    }

    /// Transfers the leaf to an explicit recipient while retaining the current delegate commitment.
    /// # Errors
    /// Preserves complete data, account and caller-budget errors.
    pub fn transfer(
        &self,
        data: &Transfer,
        recipient: Address,
        signer: LeafAuthority,
        proof: &[Node],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        data.instruction_with_program(
            self.program,
            &self.accounts(signer, Some(recipient), proof, limits)?,
            limits,
        )
    }

    /// Changes the delegate with the owner as the required signer.
    /// # Errors
    /// Preserves complete data, account and caller-budget errors.
    pub fn delegate(
        &self,
        data: &Delegate,
        new_delegate: Address,
        proof: &[Node],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let accounts = self.accounts(LeafAuthority::Owner, Some(new_delegate), proof, limits)?;
        data.instruction_with_program(self.program, &accounts, limits)
    }

    fn accounts(
        &self,
        signer: LeafAuthority,
        recipient: Option<Address>,
        proof: &[Node],
        limits: Limits,
    ) -> Result<Vec<AccountMeta>, Error> {
        const BASE_ACCOUNTS: usize = 7;
        let fixed = BASE_ACCOUNTS + usize::from(recipient.is_some());
        let count = fixed.checked_add(proof.len()).ok_or(Error::Overflow)?;

        if count > limits.max_items
            || proof
                .len()
                .checked_mul(core::mem::size_of::<Node>())
                .ok_or(Error::Overflow)?
                > limits.max_bytes
        {
            return Err(Error::LimitExceeded);
        }

        let (owner_role, delegate_role) = signer.roles();

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.extend([
            meta(self.tree_authority, AccountRole::Readonly),
            meta(self.owner, owner_role),
            meta(self.delegate, delegate_role),
        ]);

        if let Some(recipient) = recipient {
            accounts.push(meta(recipient, AccountRole::Readonly));
        }

        accounts.extend([
            meta(self.tree, AccountRole::Writable),
            meta(self.programs.log_wrapper, AccountRole::Readonly),
            meta(self.programs.compression, AccountRole::Readonly),
            meta(self.programs.system, AccountRole::Readonly),
        ]);
        accounts.extend(
            proof
                .iter()
                .map(|node| meta(Address::from_bytes(*node), AccountRole::Readonly)),
        );
        Ok(accounts)
    }
}

fn meta(address: Address, role: AccountRole) -> AccountMeta {
    AccountMeta { address, role }
}
