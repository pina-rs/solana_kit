//! Source-pinned account inputs and instruction wrappers.
use crate::{InitializeTokenGroupMember, InitializeTokenGroupMemberCodec, PROGRAM, invocation::*};
use alloc::vec::Vec;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits},
};
/// Caller-owned addresses in exact protocol order.
#[derive(Clone, Copy, Debug)]
pub struct InitializeTokenGroupMemberAccounts {
    /// Source role `member`.
    pub member: Address,
    /// Source role `member_mint`.
    pub member_mint: Address,
    /// Source role `member_mint_authority`.
    pub member_mint_authority: Address,
    /// Source role `group`.
    pub group: Address,
    /// Source role `group_update_authority`.
    pub group_update_authority: Address,
}
impl InitializeTokenGroupMemberAccounts {
    fn allocate(self, limits: Limits) -> Result<Vec<AccountMeta>, Error> {
        let count: usize = 5;

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

        accounts.push(meta(self.member, AccountRole::Writable));
        accounts.push(meta(self.member_mint, AccountRole::Readonly));
        accounts.push(meta(
            self.member_mint_authority,
            AccountRole::ReadonlySigner,
        ));
        accounts.push(meta(self.group, AccountRole::Writable));
        accounts.push(meta(
            self.group_update_authority,
            AccountRole::ReadonlySigner,
        ));

        let _ = program;
        Ok(accounts)
    }
}
impl InitializeTokenGroupMember {
    /// Builds for the canonical Token-2022 identity.
    pub fn instruction(
        &self,
        input: InitializeTokenGroupMemberAccounts,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, input, limits)
    }
    /// Builds with caller-owned addresses and an explicit program identity.
    pub fn instruction_with_program(
        &self,
        program: Address,
        input: InitializeTokenGroupMemberAccounts,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let accounts = input.metas(program, limits)?;
        self.validate_accounts(program, &accounts, limits)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data: InitializeTokenGroupMemberCodec.encode(self, limits.max_bytes)?,
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

        let value = InitializeTokenGroupMemberCodec.decode(&instruction.data, limits)?;
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
        signer: Signer::Required,
        optional: false,
    },
    Role {
        writable: true,
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
impl InitializeTokenGroupMember {
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
