//! Source-pinned account inputs and instruction wrappers.
use crate::{InitializeTokenMetadata, InitializeTokenMetadataCodec, PROGRAM, invocation::*};
use alloc::vec::Vec;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits},
};
/// Caller-owned addresses in exact protocol order.
#[derive(Clone, Copy, Debug)]
pub struct InitializeTokenMetadataAccounts {
    /// Source role `metadata`.
    pub metadata: Address,
    /// Source role `update_authority`.
    pub update_authority: Address,
    /// Source role `mint`.
    pub mint: Address,
    /// Source role `mint_authority`.
    pub mint_authority: Address,
}
impl InitializeTokenMetadataAccounts {
    fn allocate(self, limits: Limits) -> Result<Vec<AccountMeta>, Error> {
        let count: usize = 4;

        let mut accounts = Vec::new();

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;

        Ok(accounts)
    }
    fn metas(self, program: Address, limits: Limits) -> Result<Vec<AccountMeta>, Error> {
        let mut accounts = self.allocate(limits)?;

        accounts.push(meta(self.metadata, AccountRole::Writable));
        accounts.push(meta(self.update_authority, AccountRole::Readonly));
        accounts.push(meta(self.mint, AccountRole::Readonly));
        accounts.push(meta(self.mint_authority, AccountRole::ReadonlySigner));

        let _ = program;
        Ok(accounts)
    }
}
impl InitializeTokenMetadata {
    /// Builds for the canonical Token-2022 identity.
    pub fn instruction(
        &self,
        input: InitializeTokenMetadataAccounts,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, input, limits)
    }
    /// Builds with caller-owned addresses and an explicit program identity.
    pub fn instruction_with_program(
        &self,
        program: Address,
        input: InitializeTokenMetadataAccounts,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let accounts = input.metas(program, limits)?;
        self.validate_accounts(program, &accounts, limits)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data: InitializeTokenMetadataCodec.encode(self, limits.max_bytes)?,
        })
    }
    /// Parses a complete frame after program and account validation.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        let value = InitializeTokenMetadataCodec.decode(&instruction.data, limits)?;
        value.validate_accounts(program, &instruction.accounts, limits)?;
        Ok(value)
    }
}
const FIXED_ROLES: &[Role] = &[
    Role {
        writable: true,
        signer: Signer::Never,
        optional: false,
    },
    Role {
        writable: false,
        signer: Signer::Never,
        optional: false,
    },
    Role {
        writable: false,
        signer: Signer::Never,
        optional: false,
    },
    Role {
        writable: false,
        signer: Signer::Required,
        optional: false,
    },
];
const REMAINING_ROLES: &[AccountRole] = &[];
impl InitializeTokenMetadata {
    pub(crate) fn validate_accounts(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        validate_accounts(
            program,
            accounts,
            FIXED_ROLES,
            REMAINING_ROLES,
            None,
            true,
            limits,
        )
    }
}
