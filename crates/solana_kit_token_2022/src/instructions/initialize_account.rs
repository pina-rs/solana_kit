//! Source-pinned account inputs and instruction wrappers.
use crate::{InitializeAccount, InitializeAccountCodec, PROGRAM, invocation::*};
use alloc::vec::Vec;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits},
};
/// Caller-owned addresses in exact protocol order.
#[derive(Clone, Copy, Debug)]
pub struct InitializeAccountAccounts {
    /// Source role `account`.
    pub account: Address,
    /// Source role `mint`.
    pub mint: Address,
    /// Source role `owner`.
    pub owner: Address,
    /// Source role `rent`.
    pub rent: Option<Address>,
}
impl InitializeAccountAccounts {
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

        accounts.push(meta(self.account, AccountRole::Writable));
        accounts.push(meta(self.mint, AccountRole::Readonly));
        accounts.push(meta(self.owner, AccountRole::Readonly));
        accounts.push(meta(
            self.rent.unwrap_or(Address::from_bytes([
                6, 167, 213, 23, 25, 44, 92, 81, 33, 140, 201, 76, 61, 74, 241, 127, 88, 218, 238,
                8, 155, 161, 253, 68, 227, 219, 217, 138, 0, 0, 0, 0,
            ])),
            AccountRole::Readonly,
        ));

        let _ = program;
        Ok(accounts)
    }
}
impl InitializeAccount {
    /// Builds for the canonical Token-2022 identity.
    pub fn instruction(
        &self,
        input: InitializeAccountAccounts,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, input, limits)
    }
    /// Builds with caller-owned addresses and an explicit program identity.
    pub fn instruction_with_program(
        &self,
        program: Address,
        input: InitializeAccountAccounts,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let accounts = input.metas(program, limits)?;
        self.validate_accounts(program, &accounts, limits)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data: InitializeAccountCodec.encode(self, limits.max_bytes)?,
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

        let value = InitializeAccountCodec.decode(&instruction.data, limits)?;
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
        signer: Signer::Never,
        optional: false,
    },
];
const REMAINING_ROLES: &[AccountRole] = &[];
impl InitializeAccount {
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
