// Generated source-backed account wrappers; see upstream notices.
use super::*;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits},
};
/// Canonical executable program address.
pub const PROGRAM: Address = Address::from_bytes([
    6, 161, 216, 23, 145, 55, 84, 42, 152, 52, 55, 189, 254, 42, 122, 178, 85, 127, 83, 92, 138,
    120, 114, 43, 104, 164, 157, 192, 0, 0, 0, 0,
]);
/// Decoded data borrowing every original invocation metadata field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedInstruction<'a, T> {
    /// Exact decoded data.
    pub data: T,
    /// Original instruction, including ordered accounts and remaining roles.
    pub instruction: &'a Instruction,
}
/// Ordered source accounts for `Initialize`.
#[derive(Clone, Debug)]
pub struct InitializeAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `rentSysvar`.
    pub rent_sysvar: Address,
}
impl InitializeAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Initialize,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (2usize)
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.rent_sysvar,
            role: AccountRole::Readonly,
        });

        accounts.extend_from_slice(remaining);
        let data = InitializeCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Initialize {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        InitializeCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `Authorize`.
#[derive(Clone, Debug)]
pub struct AuthorizeAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `clockSysvar`.
    pub clock_sysvar: Address,
    /// Source account `authority`.
    pub authority: Address,
    /// Source account `lockupAuthority`.
    pub lockup_authority: Option<Address>,
}
impl AuthorizeAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Authorize,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (3usize + usize::from(self.lockup_authority.is_some()))
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.clock_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.authority,
            role: AccountRole::ReadonlySigner,
        });

        if let Some(address) = self.lockup_authority {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::ReadonlySigner,
            });
        }

        accounts.extend_from_slice(remaining);
        let data = AuthorizeCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Authorize {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        AuthorizeCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `DelegateStake`.
#[derive(Clone, Debug)]
pub struct DelegateStakeAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `vote`.
    pub vote: Address,
    /// Source account `clockSysvar`.
    pub clock_sysvar: Address,
    /// Source account `stakeHistory`.
    pub stake_history: Address,
    /// Source account `unused`.
    pub unused: Address,
    /// Source account `stakeAuthority`.
    pub stake_authority: Address,
}
impl DelegateStakeAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &DelegateStake,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (6usize)
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.vote,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.clock_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.stake_history,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.unused,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.stake_authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = DelegateStakeCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl DelegateStake {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        DelegateStakeCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `Split`.
#[derive(Clone, Debug)]
pub struct SplitAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `splitStake`.
    pub split_stake: Address,
    /// Source account `stakeAuthority`.
    pub stake_authority: Address,
}
impl SplitAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Split,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (3usize)
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.split_stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.stake_authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = SplitCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Split {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        SplitCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `Withdraw`.
#[derive(Clone, Debug)]
pub struct WithdrawAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `recipient`.
    pub recipient: Address,
    /// Source account `clockSysvar`.
    pub clock_sysvar: Address,
    /// Source account `stakeHistory`.
    pub stake_history: Address,
    /// Source account `withdrawAuthority`.
    pub withdraw_authority: Address,
    /// Source account `lockupAuthority`.
    pub lockup_authority: Option<Address>,
}
impl WithdrawAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Withdraw,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (5usize + usize::from(self.lockup_authority.is_some()))
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.recipient,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.clock_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.stake_history,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.withdraw_authority,
            role: AccountRole::ReadonlySigner,
        });

        if let Some(address) = self.lockup_authority {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::ReadonlySigner,
            });
        }

        accounts.extend_from_slice(remaining);
        let data = WithdrawCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Withdraw {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        WithdrawCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `Deactivate`.
#[derive(Clone, Debug)]
pub struct DeactivateAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `clockSysvar`.
    pub clock_sysvar: Address,
    /// Source account `stakeAuthority`.
    pub stake_authority: Address,
}
impl DeactivateAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Deactivate,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (3usize)
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.clock_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.stake_authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = DeactivateCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Deactivate {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        DeactivateCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `SetLockup`.
#[derive(Clone, Debug)]
pub struct SetLockupAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `authority`.
    pub authority: Address,
}
impl SetLockupAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &SetLockup,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (2usize)
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = SetLockupCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl SetLockup {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        SetLockupCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `Merge`.
#[derive(Clone, Debug)]
pub struct MergeAccounts {
    /// Source account `destinationStake`.
    pub destination_stake: Address,
    /// Source account `sourceStake`.
    pub source_stake: Address,
    /// Source account `clockSysvar`.
    pub clock_sysvar: Address,
    /// Source account `stakeHistory`.
    pub stake_history: Address,
    /// Source account `stakeAuthority`.
    pub stake_authority: Address,
}
impl MergeAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Merge,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (5usize)
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.destination_stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.source_stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.clock_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.stake_history,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.stake_authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = MergeCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Merge {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        MergeCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `AuthorizeWithSeed`.
#[derive(Clone, Debug)]
pub struct AuthorizeWithSeedAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `base`.
    pub base: Address,
    /// Source account `clockSysvar`.
    pub clock_sysvar: Address,
    /// Source account `lockupAuthority`.
    pub lockup_authority: Option<Address>,
}
impl AuthorizeWithSeedAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &AuthorizeWithSeed,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (3usize + usize::from(self.lockup_authority.is_some()))
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.base,
            role: AccountRole::ReadonlySigner,
        });
        accounts.push(AccountMeta {
            address: self.clock_sysvar,
            role: AccountRole::Readonly,
        });

        if let Some(address) = self.lockup_authority {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::ReadonlySigner,
            });
        }

        accounts.extend_from_slice(remaining);
        let data = AuthorizeWithSeedCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl AuthorizeWithSeed {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        AuthorizeWithSeedCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `InitializeChecked`.
#[derive(Clone, Debug)]
pub struct InitializeCheckedAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `rentSysvar`.
    pub rent_sysvar: Address,
    /// Source account `stakeAuthority`.
    pub stake_authority: Address,
    /// Source account `withdrawAuthority`.
    pub withdraw_authority: Address,
}
impl InitializeCheckedAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &InitializeChecked,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (4usize)
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.rent_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.stake_authority,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.withdraw_authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = InitializeCheckedCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl InitializeChecked {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        InitializeCheckedCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `AuthorizeChecked`.
#[derive(Clone, Debug)]
pub struct AuthorizeCheckedAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `clockSysvar`.
    pub clock_sysvar: Address,
    /// Source account `authority`.
    pub authority: Address,
    /// Source account `newAuthority`.
    pub new_authority: Address,
    /// Source account `lockupAuthority`.
    pub lockup_authority: Option<Address>,
}
impl AuthorizeCheckedAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &AuthorizeChecked,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (4usize + usize::from(self.lockup_authority.is_some()))
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.clock_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.authority,
            role: AccountRole::ReadonlySigner,
        });
        accounts.push(AccountMeta {
            address: self.new_authority,
            role: AccountRole::ReadonlySigner,
        });

        if let Some(address) = self.lockup_authority {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::ReadonlySigner,
            });
        }

        accounts.extend_from_slice(remaining);
        let data = AuthorizeCheckedCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl AuthorizeChecked {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        AuthorizeCheckedCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `AuthorizeCheckedWithSeed`.
#[derive(Clone, Debug)]
pub struct AuthorizeCheckedWithSeedAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `base`.
    pub base: Address,
    /// Source account `clockSysvar`.
    pub clock_sysvar: Address,
    /// Source account `newAuthority`.
    pub new_authority: Address,
    /// Source account `lockupAuthority`.
    pub lockup_authority: Option<Address>,
}
impl AuthorizeCheckedWithSeedAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &AuthorizeCheckedWithSeed,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (4usize + usize::from(self.lockup_authority.is_some()))
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.base,
            role: AccountRole::ReadonlySigner,
        });
        accounts.push(AccountMeta {
            address: self.clock_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.new_authority,
            role: AccountRole::ReadonlySigner,
        });

        if let Some(address) = self.lockup_authority {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::ReadonlySigner,
            });
        }

        accounts.extend_from_slice(remaining);
        let data = AuthorizeCheckedWithSeedCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl AuthorizeCheckedWithSeed {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        AuthorizeCheckedWithSeedCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `SetLockupChecked`.
#[derive(Clone, Debug)]
pub struct SetLockupCheckedAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `authority`.
    pub authority: Address,
    /// Source account `newAuthority`.
    pub new_authority: Option<Address>,
}
impl SetLockupCheckedAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &SetLockupChecked,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (2usize + usize::from(self.new_authority.is_some()))
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.authority,
            role: AccountRole::ReadonlySigner,
        });

        if let Some(address) = self.new_authority {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::ReadonlySigner,
            });
        }

        accounts.extend_from_slice(remaining);
        let data = SetLockupCheckedCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl SetLockupChecked {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        SetLockupCheckedCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `GetMinimumDelegation`.
#[derive(Clone, Debug)]
pub struct GetMinimumDelegationAccounts {}
impl GetMinimumDelegationAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &GetMinimumDelegation,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (0usize)
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;

        accounts.extend_from_slice(remaining);
        let data = GetMinimumDelegationCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl GetMinimumDelegation {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        GetMinimumDelegationCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `DeactivateDelinquent`.
#[derive(Clone, Debug)]
pub struct DeactivateDelinquentAccounts {
    /// Source account `stake`.
    pub stake: Address,
    /// Source account `delinquentVote`.
    pub delinquent_vote: Address,
    /// Source account `referenceVote`.
    pub reference_vote: Address,
}
impl DeactivateDelinquentAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &DeactivateDelinquent,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (3usize)
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.delinquent_vote,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.reference_vote,
            role: AccountRole::Readonly,
        });

        accounts.extend_from_slice(remaining);
        let data = DeactivateDelinquentCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl DeactivateDelinquent {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        DeactivateDelinquentCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `MoveStake`.
#[derive(Clone, Debug)]
pub struct MoveStakeAccounts {
    /// Source account `sourceStake`.
    pub source_stake: Address,
    /// Source account `destinationStake`.
    pub destination_stake: Address,
    /// Source account `stakeAuthority`.
    pub stake_authority: Address,
}
impl MoveStakeAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &MoveStake,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (3usize)
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.source_stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.destination_stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.stake_authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = MoveStakeCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl MoveStake {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        MoveStakeCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
/// Ordered source accounts for `MoveLamports`.
#[derive(Clone, Debug)]
pub struct MoveLamportsAccounts {
    /// Source account `sourceStake`.
    pub source_stake: Address,
    /// Source account `destinationStake`.
    pub destination_stake: Address,
    /// Source account `stakeAuthority`.
    pub stake_authority: Address,
}
impl MoveLamportsAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &MoveLamports,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (3usize)
            .checked_add(remaining.len())
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = alloc::vec::Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.push(AccountMeta {
            address: self.source_stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.destination_stake,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.stake_authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = MoveLamportsCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl MoveLamports {
    /// Decodes exact data for an explicitly selected program.
    /// # Errors
    /// Rejects foreign programs, malformed data, limits, and trailing bytes.
    pub fn parse_instruction(
        instruction: &Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        if instruction.accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        MoveLamportsCodec.decode(
            &instruction.data,
            Limits::new(
                limits.max_bytes,
                limits.max_items - instruction.accounts.len(),
            ),
        )
    }
    /// Decodes while borrowing all original invocation metadata.
    /// # Errors
    /// Preserves every program, data and budget error from the strict parser.
    pub fn parse_with_metadata<'a>(
        instruction: &'a Instruction,
        program: Address,
        limits: Limits,
    ) -> Result<ParsedInstruction<'a, Self>, Error> {
        Ok(ParsedInstruction {
            data: Self::parse_instruction(instruction, program, limits)?,
            instruction,
        })
    }
}
