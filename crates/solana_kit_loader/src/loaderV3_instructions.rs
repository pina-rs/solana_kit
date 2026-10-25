// Generated source-backed account wrappers; see upstream notices.
use super::*;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits},
};
/// Canonical executable program address.
pub const PROGRAM: Address = Address::from_bytes([
    2, 168, 246, 145, 78, 136, 161, 176, 226, 16, 21, 62, 247, 99, 174, 43, 0, 194, 185, 61, 22,
    193, 36, 210, 192, 83, 122, 16, 4, 128, 0, 0,
]);
/// Decoded data borrowing every original invocation metadata field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedInstruction<'a, T> {
    /// Exact decoded data.
    pub data: T,
    /// Original instruction, including ordered accounts and remaining roles.
    pub instruction: &'a Instruction,
}
/// Ordered source accounts for `InitializeBuffer`.
#[derive(Clone, Debug)]
pub struct InitializeBufferAccounts {
    /// Source account `sourceAccount`.
    pub source_account: Address,
    /// Source account `bufferAuthority`.
    pub buffer_authority: Address,
}
impl InitializeBufferAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &InitializeBuffer,
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
            address: self.source_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.buffer_authority,
            role: AccountRole::Readonly,
        });

        accounts.extend_from_slice(remaining);
        let data = InitializeBufferCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl InitializeBuffer {
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

        InitializeBufferCodec.decode(
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
/// Ordered source accounts for `Write`.
#[derive(Clone, Debug)]
pub struct WriteAccounts {
    /// Source account `bufferAccount`.
    pub buffer_account: Address,
    /// Source account `bufferAuthority`.
    pub buffer_authority: Address,
}
impl WriteAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Write,
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
            address: self.buffer_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.buffer_authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = WriteCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Write {
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

        WriteCodec.decode(
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
/// Ordered source accounts for `DeployWithMaxDataLen`.
#[derive(Clone, Debug)]
pub struct DeployWithMaxDataLenAccounts {
    /// Source account `payerAccount`.
    pub payer_account: Address,
    /// Source account `programDataAccount`.
    pub program_data_account: Address,
    /// Source account `programAccount`.
    pub program_account: Address,
    /// Source account `bufferAccount`.
    pub buffer_account: Address,
    /// Source account `rentSysvar`.
    pub rent_sysvar: Address,
    /// Source account `clockSysvar`.
    pub clock_sysvar: Address,
    /// Source account `systemProgram`.
    pub system_program: Address,
    /// Source account `authority`.
    pub authority: Address,
}
impl DeployWithMaxDataLenAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &DeployWithMaxDataLen,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (8usize)
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
            address: self.payer_account,
            role: AccountRole::WritableSigner,
        });
        accounts.push(AccountMeta {
            address: self.program_data_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.program_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.buffer_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.rent_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.clock_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.system_program,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = DeployWithMaxDataLenCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl DeployWithMaxDataLen {
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

        DeployWithMaxDataLenCodec.decode(
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
/// Ordered source accounts for `Upgrade`.
#[derive(Clone, Debug)]
pub struct UpgradeAccounts {
    /// Source account `programDataAccount`.
    pub program_data_account: Address,
    /// Source account `programAccount`.
    pub program_account: Address,
    /// Source account `bufferAccount`.
    pub buffer_account: Address,
    /// Source account `spillAccount`.
    pub spill_account: Address,
    /// Source account `rentSysvar`.
    pub rent_sysvar: Address,
    /// Source account `clockSysvar`.
    pub clock_sysvar: Address,
    /// Source account `authority`.
    pub authority: Address,
}
impl UpgradeAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Upgrade,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (7usize)
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
            address: self.program_data_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.program_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.buffer_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.spill_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.rent_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.clock_sysvar,
            role: AccountRole::Readonly,
        });
        accounts.push(AccountMeta {
            address: self.authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = UpgradeCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Upgrade {
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

        UpgradeCodec.decode(
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
/// Ordered source accounts for `SetAuthority`.
#[derive(Clone, Debug)]
pub struct SetAuthorityAccounts {
    /// Source account `bufferOrProgramDataAccount`.
    pub buffer_or_program_data_account: Address,
    /// Source account `currentAuthority`.
    pub current_authority: Address,
    /// Source account `newAuthority`.
    pub new_authority: Option<Address>,
}
impl SetAuthorityAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &SetAuthority,
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
            address: self.buffer_or_program_data_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.current_authority,
            role: AccountRole::ReadonlySigner,
        });

        if let Some(address) = self.new_authority {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::Readonly,
            });
        }

        accounts.extend_from_slice(remaining);
        let data = SetAuthorityCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl SetAuthority {
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

        SetAuthorityCodec.decode(
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
/// Ordered source accounts for `Close`.
#[derive(Clone, Debug)]
pub struct CloseAccounts {
    /// Source account `bufferOrProgramDataAccount`.
    pub buffer_or_program_data_account: Address,
    /// Source account `destinationAccount`.
    pub destination_account: Address,
    /// Source account `authority`.
    pub authority: Option<Address>,
    /// Source account `programAccount`.
    pub program_account: Option<Address>,
}
impl CloseAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Close,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (2usize
            + usize::from(self.authority.is_some())
            + usize::from(self.program_account.is_some()))
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
            address: self.buffer_or_program_data_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.destination_account,
            role: AccountRole::Writable,
        });

        if let Some(address) = self.authority {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::ReadonlySigner,
            });
        }

        if let Some(address) = self.program_account {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::Readonly,
            });
        }

        accounts.extend_from_slice(remaining);
        let data = CloseCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Close {
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

        CloseCodec.decode(
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
/// Ordered source accounts for `ExtendProgram`.
#[derive(Clone, Debug)]
pub struct ExtendProgramAccounts {
    /// Source account `programDataAccount`.
    pub program_data_account: Address,
    /// Source account `programAccount`.
    pub program_account: Address,
    /// Source account `systemProgram`.
    pub system_program: Option<Address>,
    /// Source account `payer`.
    pub payer: Option<Address>,
}
impl ExtendProgramAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &ExtendProgram,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (2usize
            + usize::from(self.system_program.is_some())
            + usize::from(self.payer.is_some()))
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
            address: self.program_data_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.program_account,
            role: AccountRole::Writable,
        });

        if let Some(address) = self.system_program {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::Readonly,
            });
        }

        if let Some(address) = self.payer {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::WritableSigner,
            });
        }

        accounts.extend_from_slice(remaining);
        let data = ExtendProgramCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl ExtendProgram {
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

        ExtendProgramCodec.decode(
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
/// Ordered source accounts for `SetAuthorityChecked`.
#[derive(Clone, Debug)]
pub struct SetAuthorityCheckedAccounts {
    /// Source account `bufferOrProgramDataAccount`.
    pub buffer_or_program_data_account: Address,
    /// Source account `currentAuthority`.
    pub current_authority: Address,
    /// Source account `newAuthority`.
    pub new_authority: Address,
}
impl SetAuthorityCheckedAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &SetAuthorityChecked,
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
            address: self.buffer_or_program_data_account,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.current_authority,
            role: AccountRole::ReadonlySigner,
        });
        accounts.push(AccountMeta {
            address: self.new_authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = SetAuthorityCheckedCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl SetAuthorityChecked {
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

        SetAuthorityCheckedCodec.decode(
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
