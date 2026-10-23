//! Source-pinned account inputs and instruction wrappers.
use crate::{
    ConfigureConfidentialTransferAccountWithRegistry,
    ConfigureConfidentialTransferAccountWithRegistryCodec, PROGRAM, invocation::*,
};
use alloc::vec::Vec;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits},
};
/// Caller-owned addresses in exact protocol order.
#[derive(Clone, Copy, Debug)]
pub struct ConfigureConfidentialTransferAccountWithRegistryAccounts {
    /// Source role `token`.
    pub token: Address,
    /// Source role `mint`.
    pub mint: Address,
    /// Source role `elgamal_registry`.
    pub elgamal_registry: Address,
    /// Source role `payer`.
    pub payer: Option<Address>,
    /// Source role `system_program`.
    pub system_program: Option<Address>,
}
impl ConfigureConfidentialTransferAccountWithRegistryAccounts {
    fn allocate(self, limits: Limits) -> Result<Vec<AccountMeta>, Error> {
        let count: usize =
            3 + usize::from(self.payer.is_some()) + usize::from(self.system_program.is_some());

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

        accounts.push(meta(self.token, AccountRole::Writable));
        accounts.push(meta(self.mint, AccountRole::Readonly));
        accounts.push(meta(self.elgamal_registry, AccountRole::Readonly));
        append_optional(
            &mut accounts,
            self.payer,
            program,
            AccountRole::WritableSigner,
            true,
        );
        append_optional(
            &mut accounts,
            self.system_program,
            program,
            AccountRole::Readonly,
            true,
        );

        let _ = program;
        Ok(accounts)
    }
}
impl ConfigureConfidentialTransferAccountWithRegistry {
    /// Builds for the canonical Token-2022 identity.
    pub fn instruction(
        &self,
        input: ConfigureConfidentialTransferAccountWithRegistryAccounts,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, input, limits)
    }
    /// Builds with caller-owned addresses and an explicit program identity.
    pub fn instruction_with_program(
        &self,
        program: Address,
        input: ConfigureConfidentialTransferAccountWithRegistryAccounts,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let accounts = input.metas(program, limits)?;
        self.validate_accounts(program, &accounts, limits)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data: ConfigureConfidentialTransferAccountWithRegistryCodec
                .encode(self, limits.max_bytes)?,
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

        let value = ConfigureConfidentialTransferAccountWithRegistryCodec
            .decode(&instruction.data, limits)?;
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
        writable: true,
        signer: Signer::Required,
        optional: true,
    },
    Role {
        writable: false,
        signer: Signer::Never,
        optional: true,
    },
];
const REMAINING_ROLES: &[AccountRole] = &[];
impl ConfigureConfidentialTransferAccountWithRegistry {
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
            false,
            limits,
        )
    }
}
