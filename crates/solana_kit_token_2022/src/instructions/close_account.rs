//! Source-pinned account inputs and instruction wrappers.
use crate::{CloseAccount, CloseAccountCodec, PROGRAM, invocation::*};
use alloc::vec::Vec;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits},
};
/// Caller-owned addresses in exact protocol order.
#[derive(Clone, Copy, Debug)]
pub struct CloseAccountAccounts<'a> {
    /// Source role `account`.
    pub account: Address,
    /// Source role `destination`.
    pub destination: Address,
    /// Source role `owner`.
    pub owner: Authority,
    /// Ordered `multi_signers` list; duplicate addresses are preserved.
    pub multi_signers: &'a [Address],
}
impl<'a> CloseAccountAccounts<'a> {
    fn allocate(self, limits: Limits) -> Result<Vec<AccountMeta>, Error> {
        let count: usize = 3;
        let count = count
            .checked_add(self.multi_signers.len())
            .ok_or(Error::Overflow)?;

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

        accounts.push(meta(self.account, AccountRole::Writable));
        accounts.push(meta(self.destination, AccountRole::Writable));
        let authority = self.owner;
        let role = AccountRole::from_privileges(authority.is_signer, false);
        accounts.push(meta(authority.address, role));

        for address in self.multi_signers {
            accounts.push(meta(*address, AccountRole::ReadonlySigner));
        }

        let _ = program;
        Ok(accounts)
    }
}
impl CloseAccount {
    /// Builds for the canonical Token-2022 identity.
    pub fn instruction(
        &self,
        input: CloseAccountAccounts<'_>,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, input, limits)
    }
    /// Builds with caller-owned addresses and an explicit program identity.
    pub fn instruction_with_program(
        &self,
        program: Address,
        input: CloseAccountAccounts<'_>,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let accounts = input.metas(program, limits)?;
        self.validate_accounts(program, &accounts, limits)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data: CloseAccountCodec.encode(self, limits.max_bytes)?,
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

        let value = CloseAccountCodec.decode(&instruction.data, limits)?;
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
        writable: true,
        signer: Signer::Never,
        optional: false,
    },
    Role {
        writable: false,
        signer: Signer::Either,
        optional: false,
    },
];
const REMAINING_ROLES: &[AccountRole] = &[AccountRole::from_privileges(true, false)];
impl CloseAccount {
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
