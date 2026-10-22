// Generated from pinned System account metadata; see SYSTEM-LICENSE.
use crate::{
    ClientError, RECENT_BLOCKHASHES, RENT, SYSTEM_PROGRAM, client::validate_accounts, generated::*,
};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Limits},
};
const MAX_SEED_BYTES: usize = 32;

/// Ordered caller-supplied accounts for [`CreateAccount`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreateAccountAccounts {
    /// Address for `payer`.
    pub payer: Address,
    /// Address for `new_account`.
    pub new_account: Address,
}

impl CreateAccount {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: CreateAccountAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        let metas = alloc::vec![
            AccountMeta {
                address: accounts.payer,
                role: AccountRole::WritableSigner
            },
            AccountMeta {
                address: accounts.new_account,
                role: AccountRole::WritableSigner
            },
        ];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: CreateAccountCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, CreateAccountAccounts), ClientError> {
        validate_accounts(
            instruction,
            &[
                (AccountRole::WritableSigner, None),
                (AccountRole::WritableSigner, None),
            ],
            2,
        )?;
        let value = CreateAccountCodec.decode(&instruction.data, limits)?;
        let accounts = CreateAccountAccounts {
            payer: instruction.accounts[0].address,
            new_account: instruction.accounts[1].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`Assign`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssignAccounts {
    /// Address for `account`.
    pub account: Address,
}

impl Assign {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: AssignAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        let metas = alloc::vec![AccountMeta {
            address: accounts.account,
            role: AccountRole::WritableSigner
        },];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: AssignCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, AssignAccounts), ClientError> {
        validate_accounts(instruction, &[(AccountRole::WritableSigner, None)], 1)?;
        let value = AssignCodec.decode(&instruction.data, limits)?;
        let accounts = AssignAccounts {
            account: instruction.accounts[0].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`TransferSol`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransferSolAccounts {
    /// Address for `source`.
    pub source: Address,
    /// Address for `destination`.
    pub destination: Address,
}

impl TransferSol {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: TransferSolAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        let metas = alloc::vec![
            AccountMeta {
                address: accounts.source,
                role: AccountRole::WritableSigner
            },
            AccountMeta {
                address: accounts.destination,
                role: AccountRole::Writable
            },
        ];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: TransferSolCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, TransferSolAccounts), ClientError> {
        validate_accounts(
            instruction,
            &[
                (AccountRole::WritableSigner, None),
                (AccountRole::Writable, None),
            ],
            2,
        )?;
        let value = TransferSolCodec.decode(&instruction.data, limits)?;
        let accounts = TransferSolAccounts {
            source: instruction.accounts[0].address,
            destination: instruction.accounts[1].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`CreateAccountWithSeed`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreateAccountWithSeedAccounts {
    /// Address for `payer`.
    pub payer: Address,
    /// Address for `new_account`.
    pub new_account: Address,
    /// Address for `base_account`.
    pub base_account: Option<Address>,
}

impl CreateAccountWithSeed {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: CreateAccountWithSeedAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        if self.seed.len() > MAX_SEED_BYTES {
            return Err(ClientError::SeedTooLong);
        }

        if self.base != accounts.base_account.unwrap_or(accounts.payer) {
            return Err(ClientError::BaseMismatch);
        }

        let mut metas = alloc::vec![
            AccountMeta {
                address: accounts.payer,
                role: AccountRole::WritableSigner
            },
            AccountMeta {
                address: accounts.new_account,
                role: AccountRole::Writable
            },
        ];

        if let Some(address) = accounts.base_account {
            metas.push(AccountMeta {
                address,
                role: AccountRole::ReadonlySigner,
            });
        }

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: CreateAccountWithSeedCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, CreateAccountWithSeedAccounts), ClientError> {
        validate_accounts(
            instruction,
            &[
                (AccountRole::WritableSigner, None),
                (AccountRole::Writable, None),
                (AccountRole::ReadonlySigner, None),
            ],
            2,
        )?;
        let value = CreateAccountWithSeedCodec.decode(&instruction.data, limits)?;
        let accounts = CreateAccountWithSeedAccounts {
            payer: instruction.accounts[0].address,
            new_account: instruction.accounts[1].address,
            base_account: instruction.accounts.get(2).map(|meta| meta.address),
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`AdvanceNonceAccount`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdvanceNonceAccountAccounts {
    /// Address for `nonce_account`.
    pub nonce_account: Address,
    /// Address for `nonce_authority`.
    pub nonce_authority: Address,
}

impl AdvanceNonceAccount {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: AdvanceNonceAccountAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        let metas = alloc::vec![
            AccountMeta {
                address: accounts.nonce_account,
                role: AccountRole::Writable
            },
            AccountMeta {
                address: RECENT_BLOCKHASHES,
                role: AccountRole::Readonly
            },
            AccountMeta {
                address: accounts.nonce_authority,
                role: AccountRole::ReadonlySigner
            },
        ];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: AdvanceNonceAccountCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, AdvanceNonceAccountAccounts), ClientError> {
        validate_accounts(
            instruction,
            &[
                (AccountRole::Writable, None),
                (AccountRole::Readonly, Some(RECENT_BLOCKHASHES)),
                (AccountRole::ReadonlySigner, None),
            ],
            3,
        )?;
        let value = AdvanceNonceAccountCodec.decode(&instruction.data, limits)?;
        let accounts = AdvanceNonceAccountAccounts {
            nonce_account: instruction.accounts[0].address,
            nonce_authority: instruction.accounts[2].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`WithdrawNonceAccount`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WithdrawNonceAccountAccounts {
    /// Address for `nonce_account`.
    pub nonce_account: Address,
    /// Address for `recipient_account`.
    pub recipient_account: Address,
    /// Address for `nonce_authority`.
    pub nonce_authority: Address,
}

impl WithdrawNonceAccount {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: WithdrawNonceAccountAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        let metas = alloc::vec![
            AccountMeta {
                address: accounts.nonce_account,
                role: AccountRole::Writable
            },
            AccountMeta {
                address: accounts.recipient_account,
                role: AccountRole::Writable
            },
            AccountMeta {
                address: RECENT_BLOCKHASHES,
                role: AccountRole::Readonly
            },
            AccountMeta {
                address: RENT,
                role: AccountRole::Readonly
            },
            AccountMeta {
                address: accounts.nonce_authority,
                role: AccountRole::ReadonlySigner
            },
        ];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: WithdrawNonceAccountCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, WithdrawNonceAccountAccounts), ClientError> {
        validate_accounts(
            instruction,
            &[
                (AccountRole::Writable, None),
                (AccountRole::Writable, None),
                (AccountRole::Readonly, Some(RECENT_BLOCKHASHES)),
                (AccountRole::Readonly, Some(RENT)),
                (AccountRole::ReadonlySigner, None),
            ],
            5,
        )?;
        let value = WithdrawNonceAccountCodec.decode(&instruction.data, limits)?;
        let accounts = WithdrawNonceAccountAccounts {
            nonce_account: instruction.accounts[0].address,
            recipient_account: instruction.accounts[1].address,
            nonce_authority: instruction.accounts[4].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`InitializeNonceAccount`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InitializeNonceAccountAccounts {
    /// Address for `nonce_account`.
    pub nonce_account: Address,
}

impl InitializeNonceAccount {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: InitializeNonceAccountAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        let metas = alloc::vec![
            AccountMeta {
                address: accounts.nonce_account,
                role: AccountRole::Writable
            },
            AccountMeta {
                address: RECENT_BLOCKHASHES,
                role: AccountRole::Readonly
            },
            AccountMeta {
                address: RENT,
                role: AccountRole::Readonly
            },
        ];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: InitializeNonceAccountCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, InitializeNonceAccountAccounts), ClientError> {
        validate_accounts(
            instruction,
            &[
                (AccountRole::Writable, None),
                (AccountRole::Readonly, Some(RECENT_BLOCKHASHES)),
                (AccountRole::Readonly, Some(RENT)),
            ],
            3,
        )?;
        let value = InitializeNonceAccountCodec.decode(&instruction.data, limits)?;
        let accounts = InitializeNonceAccountAccounts {
            nonce_account: instruction.accounts[0].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`AuthorizeNonceAccount`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthorizeNonceAccountAccounts {
    /// Address for `nonce_account`.
    pub nonce_account: Address,
    /// Address for `nonce_authority`.
    pub nonce_authority: Address,
}

impl AuthorizeNonceAccount {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: AuthorizeNonceAccountAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        let metas = alloc::vec![
            AccountMeta {
                address: accounts.nonce_account,
                role: AccountRole::Writable
            },
            AccountMeta {
                address: accounts.nonce_authority,
                role: AccountRole::ReadonlySigner
            },
        ];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: AuthorizeNonceAccountCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, AuthorizeNonceAccountAccounts), ClientError> {
        validate_accounts(
            instruction,
            &[
                (AccountRole::Writable, None),
                (AccountRole::ReadonlySigner, None),
            ],
            2,
        )?;
        let value = AuthorizeNonceAccountCodec.decode(&instruction.data, limits)?;
        let accounts = AuthorizeNonceAccountAccounts {
            nonce_account: instruction.accounts[0].address,
            nonce_authority: instruction.accounts[1].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`Allocate`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AllocateAccounts {
    /// Address for `new_account`.
    pub new_account: Address,
}

impl Allocate {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: AllocateAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        let metas = alloc::vec![AccountMeta {
            address: accounts.new_account,
            role: AccountRole::WritableSigner
        },];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: AllocateCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, AllocateAccounts), ClientError> {
        validate_accounts(instruction, &[(AccountRole::WritableSigner, None)], 1)?;
        let value = AllocateCodec.decode(&instruction.data, limits)?;
        let accounts = AllocateAccounts {
            new_account: instruction.accounts[0].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`AllocateWithSeed`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AllocateWithSeedAccounts {
    /// Address for `new_account`.
    pub new_account: Address,
    /// Address for `base_account`.
    pub base_account: Address,
}

impl AllocateWithSeed {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: AllocateWithSeedAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        if self.seed.len() > MAX_SEED_BYTES {
            return Err(ClientError::SeedTooLong);
        }

        if self.base != accounts.base_account {
            return Err(ClientError::BaseMismatch);
        }

        let metas = alloc::vec![
            AccountMeta {
                address: accounts.new_account,
                role: AccountRole::Writable
            },
            AccountMeta {
                address: accounts.base_account,
                role: AccountRole::ReadonlySigner
            },
        ];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: AllocateWithSeedCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, AllocateWithSeedAccounts), ClientError> {
        validate_accounts(
            instruction,
            &[
                (AccountRole::Writable, None),
                (AccountRole::ReadonlySigner, None),
            ],
            2,
        )?;
        let value = AllocateWithSeedCodec.decode(&instruction.data, limits)?;
        let accounts = AllocateWithSeedAccounts {
            new_account: instruction.accounts[0].address,
            base_account: instruction.accounts[1].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`AssignWithSeed`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssignWithSeedAccounts {
    /// Address for `account`.
    pub account: Address,
    /// Address for `base_account`.
    pub base_account: Address,
}

impl AssignWithSeed {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: AssignWithSeedAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        if self.seed.len() > MAX_SEED_BYTES {
            return Err(ClientError::SeedTooLong);
        }

        if self.base != accounts.base_account {
            return Err(ClientError::BaseMismatch);
        }

        let metas = alloc::vec![
            AccountMeta {
                address: accounts.account,
                role: AccountRole::Writable
            },
            AccountMeta {
                address: accounts.base_account,
                role: AccountRole::ReadonlySigner
            },
        ];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: AssignWithSeedCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, AssignWithSeedAccounts), ClientError> {
        validate_accounts(
            instruction,
            &[
                (AccountRole::Writable, None),
                (AccountRole::ReadonlySigner, None),
            ],
            2,
        )?;
        let value = AssignWithSeedCodec.decode(&instruction.data, limits)?;
        let accounts = AssignWithSeedAccounts {
            account: instruction.accounts[0].address,
            base_account: instruction.accounts[1].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`TransferSolWithSeed`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransferSolWithSeedAccounts {
    /// Address for `source`.
    pub source: Address,
    /// Address for `base_account`.
    pub base_account: Address,
    /// Address for `destination`.
    pub destination: Address,
}

impl TransferSolWithSeed {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: TransferSolWithSeedAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        if self.from_seed.len() > MAX_SEED_BYTES {
            return Err(ClientError::SeedTooLong);
        }

        let metas = alloc::vec![
            AccountMeta {
                address: accounts.source,
                role: AccountRole::Writable
            },
            AccountMeta {
                address: accounts.base_account,
                role: AccountRole::ReadonlySigner
            },
            AccountMeta {
                address: accounts.destination,
                role: AccountRole::Writable
            },
        ];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: TransferSolWithSeedCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, TransferSolWithSeedAccounts), ClientError> {
        validate_accounts(
            instruction,
            &[
                (AccountRole::Writable, None),
                (AccountRole::ReadonlySigner, None),
                (AccountRole::Writable, None),
            ],
            3,
        )?;
        let value = TransferSolWithSeedCodec.decode(&instruction.data, limits)?;
        let accounts = TransferSolWithSeedAccounts {
            source: instruction.accounts[0].address,
            base_account: instruction.accounts[1].address,
            destination: instruction.accounts[2].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`UpgradeNonceAccount`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UpgradeNonceAccountAccounts {
    /// Address for `nonce_account`.
    pub nonce_account: Address,
}

impl UpgradeNonceAccount {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: UpgradeNonceAccountAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        let metas = alloc::vec![AccountMeta {
            address: accounts.nonce_account,
            role: AccountRole::Writable
        },];

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: UpgradeNonceAccountCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, UpgradeNonceAccountAccounts), ClientError> {
        validate_accounts(instruction, &[(AccountRole::Writable, None)], 1)?;
        let value = UpgradeNonceAccountCodec.decode(&instruction.data, limits)?;
        let accounts = UpgradeNonceAccountAccounts {
            nonce_account: instruction.accounts[0].address,
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Ordered caller-supplied accounts for [`CreateAccountAllowPrefund`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreateAccountAllowPrefundAccounts {
    /// Address for `new_account`.
    pub new_account: Address,
    /// Address for `payer`.
    pub payer: Option<Address>,
}

impl CreateAccountAllowPrefund {
    /// Builds an invocation with fixed System identity and canonical account privileges.
    pub fn instruction(
        &self,
        accounts: CreateAccountAllowPrefundAccounts,
        limits: Limits,
    ) -> Result<Instruction, ClientError> {
        if self.lamports != 0 && accounts.payer.is_none() {
            return Err(ClientError::MissingPayer);
        }

        let mut metas = alloc::vec![AccountMeta {
            address: accounts.new_account,
            role: AccountRole::WritableSigner
        },];

        if let Some(address) = accounts.payer {
            metas.push(AccountMeta {
                address,
                role: AccountRole::WritableSigner,
            });
        }

        Ok(Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: metas,
            data: CreateAccountAllowPrefundCodec.encode(self, limits.max_bytes)?,
        })
    }

    /// Parses program identity, tag, positional privileges and fixed sysvars.
    pub fn parse(
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<(Self, CreateAccountAllowPrefundAccounts), ClientError> {
        validate_accounts(
            instruction,
            &[
                (AccountRole::WritableSigner, None),
                (AccountRole::WritableSigner, None),
            ],
            1,
        )?;
        let value = CreateAccountAllowPrefundCodec.decode(&instruction.data, limits)?;
        let accounts = CreateAccountAllowPrefundAccounts {
            new_account: instruction.accounts[0].address,
            payer: instruction.accounts.get(1).map(|meta| meta.address),
        };
        value.instruction(accounts, limits)?;
        Ok((value, accounts))
    }
}

/// Identifies the tag prefix; full validation is provided by [`parse_instruction`].
pub fn identify_instruction(data: &[u8]) -> Result<u32, ClientError> {
    let tag = solana_kit_core::codecs::Number::<u32>::LE.decode(
        data.get(..4)
            .ok_or(solana_kit_core::codecs::Error::Truncated)?,
        Limits::new(4, 0),
    )?;

    if tag > LAST_SYSTEM_TAG {
        return Err(solana_kit_core::codecs::Error::InvalidDiscriminator.into());
    }

    Ok(tag)
}

const LAST_SYSTEM_TAG: u32 = 13;

/// Parses canonical data and ordered account contracts.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<SystemInstructionData, ClientError> {
    let parsed = match identify_instruction(&instruction.data)? {
        0 => SystemInstructionData::CreateAccount(CreateAccount::parse(instruction, limits)?.0),
        1 => SystemInstructionData::Assign(Assign::parse(instruction, limits)?.0),
        2 => SystemInstructionData::TransferSol(TransferSol::parse(instruction, limits)?.0),
        3 => SystemInstructionData::CreateAccountWithSeed(
            CreateAccountWithSeed::parse(instruction, limits)?.0,
        ),
        4 => SystemInstructionData::AdvanceNonceAccount(
            AdvanceNonceAccount::parse(instruction, limits)?.0,
        ),
        5 => SystemInstructionData::WithdrawNonceAccount(
            WithdrawNonceAccount::parse(instruction, limits)?.0,
        ),
        6 => SystemInstructionData::InitializeNonceAccount(
            InitializeNonceAccount::parse(instruction, limits)?.0,
        ),
        7 => SystemInstructionData::AuthorizeNonceAccount(
            AuthorizeNonceAccount::parse(instruction, limits)?.0,
        ),
        8 => SystemInstructionData::Allocate(Allocate::parse(instruction, limits)?.0),
        9 => {
            SystemInstructionData::AllocateWithSeed(AllocateWithSeed::parse(instruction, limits)?.0)
        }
        10 => SystemInstructionData::AssignWithSeed(AssignWithSeed::parse(instruction, limits)?.0),
        11 => SystemInstructionData::TransferSolWithSeed(
            TransferSolWithSeed::parse(instruction, limits)?.0,
        ),
        12 => SystemInstructionData::UpgradeNonceAccount(
            UpgradeNonceAccount::parse(instruction, limits)?.0,
        ),
        13 => SystemInstructionData::CreateAccountAllowPrefund(
            CreateAccountAllowPrefund::parse(instruction, limits)?.0,
        ),
        _ => return Err(solana_kit_core::codecs::Error::InvalidDiscriminator.into()),
    };

    Ok(parsed)
}
