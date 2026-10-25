// Generated source-backed account wrappers; see upstream notices.
use super::*;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits},
};
/// Canonical executable program address.
pub const PROGRAM: Address = Address::from_bytes([
    5, 18, 180, 17, 81, 81, 227, 122, 173, 10, 139, 197, 211, 136, 46, 123, 127, 218, 76, 243, 210,
    192, 40, 200, 207, 131, 54, 24, 0, 0, 0, 0,
]);
/// Decoded data borrowing every original invocation metadata field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedInstruction<'a, T> {
    /// Exact decoded data.
    pub data: T,
    /// Original instruction, including ordered accounts and remaining roles.
    pub instruction: &'a Instruction,
}
/// Ordered source accounts for `Write`.
#[derive(Clone, Debug)]
pub struct WriteAccounts {
    /// Source account `program`.
    pub program: Address,
    /// Source account `authority`.
    pub authority: Address,
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
            address: self.program,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.authority,
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
/// Ordered source accounts for `Truncate`.
#[derive(Clone, Debug)]
pub struct TruncateAccounts {
    /// Source account `program`.
    pub program: Address,
    /// Source account `authority`.
    pub authority: Address,
    /// Source account `destination`.
    pub destination: Option<Address>,
}
impl TruncateAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Truncate,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (2usize + usize::from(self.destination.is_some()))
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
            address: self.program,
            role: AccountRole::WritableSigner,
        });
        accounts.push(AccountMeta {
            address: self.authority,
            role: AccountRole::ReadonlySigner,
        });

        if let Some(address) = self.destination {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::Writable,
            });
        }

        accounts.extend_from_slice(remaining);
        let data = TruncateCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Truncate {
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

        TruncateCodec.decode(
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
/// Ordered source accounts for `Deploy`.
#[derive(Clone, Debug)]
pub struct DeployAccounts {
    /// Source account `program`.
    pub program: Address,
    /// Source account `authority`.
    pub authority: Address,
    /// Source account `source`.
    pub source: Option<Address>,
}
impl DeployAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Deploy,
        remaining: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let count = (2usize + usize::from(self.source.is_some()))
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
            address: self.program,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.authority,
            role: AccountRole::ReadonlySigner,
        });

        if let Some(address) = self.source {
            accounts.push(AccountMeta {
                address,
                role: AccountRole::Writable,
            });
        }

        accounts.extend_from_slice(remaining);
        let data = DeployCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Deploy {
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

        DeployCodec.decode(
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
/// Ordered source accounts for `Retract`.
#[derive(Clone, Debug)]
pub struct RetractAccounts {
    /// Source account `program`.
    pub program: Address,
    /// Source account `authority`.
    pub authority: Address,
}
impl RetractAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Retract,
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
            address: self.program,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.authority,
            role: AccountRole::ReadonlySigner,
        });

        accounts.extend_from_slice(remaining);
        let data = RetractCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Retract {
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

        RetractCodec.decode(
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
/// Ordered source accounts for `TransferAuthority`.
#[derive(Clone, Debug)]
pub struct TransferAuthorityAccounts {
    /// Source account `program`.
    pub program: Address,
    /// Source account `currentAuthority`.
    pub current_authority: Address,
    /// Source account `newAuthority`.
    pub new_authority: Address,
}
impl TransferAuthorityAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &TransferAuthority,
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
            address: self.program,
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
        let data = TransferAuthorityCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl TransferAuthority {
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

        TransferAuthorityCodec.decode(
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
/// Ordered source accounts for `Finalize`.
#[derive(Clone, Debug)]
pub struct FinalizeAccounts {
    /// Source account `program`.
    pub program: Address,
    /// Source account `authority`.
    pub authority: Address,
    /// Source account `nextVersion`.
    pub next_version: Address,
}
impl FinalizeAccounts {
    /// Builds source account order for an explicit program.
    /// # Errors
    /// Rejects allocation failure, invalid data, or caller limits.
    pub fn instruction(
        &self,
        program: Address,
        data: &Finalize,
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
            address: self.program,
            role: AccountRole::Writable,
        });
        accounts.push(AccountMeta {
            address: self.authority,
            role: AccountRole::ReadonlySigner,
        });
        accounts.push(AccountMeta {
            address: self.next_version,
            role: AccountRole::Readonly,
        });

        accounts.extend_from_slice(remaining);
        let data = FinalizeCodec.encode(data, limits.max_bytes)?;
        Ok(Instruction {
            program_address: program,
            accounts,
            data,
        })
    }
}
impl Finalize {
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

        FinalizeCodec.decode(
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
