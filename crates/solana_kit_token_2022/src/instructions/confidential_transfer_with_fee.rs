//! Source-pinned account inputs and instruction wrappers.
use crate::{
    ConfidentialTransferWithFee, ConfidentialTransferWithFeeCodec, PROGRAM, invocation::*,
};
use alloc::vec::Vec;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits},
};
/// Caller-owned addresses in exact protocol order.
#[derive(Clone, Copy, Debug)]
pub struct ConfidentialTransferWithFeeAccounts<'a> {
    /// Source role `source_token`.
    pub source_token: Address,
    /// Source role `mint`.
    pub mint: Address,
    /// Source role `destination_token`.
    pub destination_token: Address,
    /// Source role `instructions_sysvar`.
    pub instructions_sysvar: Option<Address>,
    /// Source role `equality_record`.
    pub equality_record: Option<Address>,
    /// Source role `transfer_amount_ciphertext_validity_record`.
    pub transfer_amount_ciphertext_validity_record: Option<Address>,
    /// Source role `fee_sigma_record`.
    pub fee_sigma_record: Option<Address>,
    /// Source role `fee_ciphertext_validity_record`.
    pub fee_ciphertext_validity_record: Option<Address>,
    /// Source role `range_record`.
    pub range_record: Option<Address>,
    /// Source role `authority`.
    pub authority: Authority,
    /// Ordered `multi_signers` list; duplicate addresses are preserved.
    pub multi_signers: &'a [Address],
}
impl<'a> ConfidentialTransferWithFeeAccounts<'a> {
    fn allocate(self, limits: Limits) -> Result<Vec<AccountMeta>, Error> {
        let count: usize = 4
            + usize::from(self.instructions_sysvar.is_some())
            + usize::from(self.equality_record.is_some())
            + usize::from(self.transfer_amount_ciphertext_validity_record.is_some())
            + usize::from(self.fee_sigma_record.is_some())
            + usize::from(self.fee_ciphertext_validity_record.is_some())
            + usize::from(self.range_record.is_some());
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

        accounts.push(meta(self.source_token, AccountRole::Writable));
        accounts.push(meta(self.mint, AccountRole::Readonly));
        accounts.push(meta(self.destination_token, AccountRole::Writable));
        append_optional(
            &mut accounts,
            self.instructions_sysvar,
            program,
            AccountRole::Readonly,
            true,
        );
        append_optional(
            &mut accounts,
            self.equality_record,
            program,
            AccountRole::Readonly,
            true,
        );
        append_optional(
            &mut accounts,
            self.transfer_amount_ciphertext_validity_record,
            program,
            AccountRole::Readonly,
            true,
        );
        append_optional(
            &mut accounts,
            self.fee_sigma_record,
            program,
            AccountRole::Readonly,
            true,
        );
        append_optional(
            &mut accounts,
            self.fee_ciphertext_validity_record,
            program,
            AccountRole::Readonly,
            true,
        );
        append_optional(
            &mut accounts,
            self.range_record,
            program,
            AccountRole::Readonly,
            true,
        );
        let authority = self.authority;
        let role = AccountRole::from_privileges(authority.is_signer, false);
        accounts.push(meta(authority.address, role));

        for address in self.multi_signers {
            accounts.push(meta(*address, AccountRole::ReadonlySigner));
        }

        let _ = program;
        Ok(accounts)
    }
}
impl ConfidentialTransferWithFee {
    /// Builds for the canonical Token-2022 identity.
    pub fn instruction(
        &self,
        input: ConfidentialTransferWithFeeAccounts<'_>,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, input, limits)
    }
    /// Builds with caller-owned addresses and an explicit program identity.
    pub fn instruction_with_program(
        &self,
        program: Address,
        input: ConfidentialTransferWithFeeAccounts<'_>,
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let accounts = input.metas(program, limits)?;
        self.validate_accounts(program, &accounts, limits)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data: ConfidentialTransferWithFeeCodec.encode(self, limits.max_bytes)?,
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

        let value = ConfidentialTransferWithFeeCodec.decode(&instruction.data, limits)?;
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
        writable: true,
        signer: Signer::Never,
        optional: false,
    },
    Role {
        writable: false,
        signer: Signer::Never,
        optional: true,
    },
    Role {
        writable: false,
        signer: Signer::Never,
        optional: true,
    },
    Role {
        writable: false,
        signer: Signer::Never,
        optional: true,
    },
    Role {
        writable: false,
        signer: Signer::Never,
        optional: true,
    },
    Role {
        writable: false,
        signer: Signer::Never,
        optional: true,
    },
    Role {
        writable: false,
        signer: Signer::Never,
        optional: true,
    },
    Role {
        writable: false,
        signer: Signer::Either,
        optional: false,
    },
];
const REMAINING_ROLES: &[AccountRole] = &[AccountRole::from_privileges(true, false)];
impl ConfidentialTransferWithFee {
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
