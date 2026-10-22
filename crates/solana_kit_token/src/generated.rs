// Generated from a validated Codama model. See README.md and upstream notices.
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Constant, Decoder, Encoder, Error, Limits, Number, Reader, Writer},
};
/// Selected trusted program identity.
pub const PROGRAM: Address = Address::from_bytes([
    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237,
    95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
]);
const INSTRUCTION_TAG_0: u8 = 0;
const INSTRUCTION_TAG_1: u8 = 1;
const INSTRUCTION_TAG_2: u8 = 2;
const INSTRUCTION_TAG_3: u8 = 3;
const INSTRUCTION_TAG_4: u8 = 4;
const INSTRUCTION_TAG_5: u8 = 5;
const INSTRUCTION_TAG_6: u8 = 6;
const INSTRUCTION_TAG_7: u8 = 7;
const INSTRUCTION_TAG_8: u8 = 8;
const INSTRUCTION_TAG_9: u8 = 9;
const INSTRUCTION_TAG_10: u8 = 10;
const INSTRUCTION_TAG_11: u8 = 11;
const INSTRUCTION_TAG_12: u8 = 12;
const INSTRUCTION_TAG_13: u8 = 13;
const INSTRUCTION_TAG_14: u8 = 14;
const INSTRUCTION_TAG_15: u8 = 15;
const INSTRUCTION_TAG_16: u8 = 16;
const INSTRUCTION_TAG_17: u8 = 17;
const INSTRUCTION_TAG_18: u8 = 18;
const INSTRUCTION_TAG_19: u8 = 19;
const INSTRUCTION_TAG_20: u8 = 20;
const INSTRUCTION_TAG_21: u8 = 21;
const INSTRUCTION_TAG_22: u8 = 22;
const INSTRUCTION_TAG_23: u8 = 23;
const INSTRUCTION_TAG_24: u8 = 24;
const INSTRUCTION_TAG_38: u8 = 38;
const INSTRUCTION_TAG_45: u8 = 45;
const INSTRUCTION_TAG_255: u8 = 255;
const PROGRAM_ERROR_0: u32 = 0;
const PROGRAM_ERROR_1: u32 = 1;
const PROGRAM_ERROR_2: u32 = 2;
const PROGRAM_ERROR_3: u32 = 3;
const PROGRAM_ERROR_4: u32 = 4;
const PROGRAM_ERROR_5: u32 = 5;
const PROGRAM_ERROR_6: u32 = 6;
const PROGRAM_ERROR_7: u32 = 7;
const PROGRAM_ERROR_8: u32 = 8;
const PROGRAM_ERROR_9: u32 = 9;
const PROGRAM_ERROR_10: u32 = 10;
const PROGRAM_ERROR_11: u32 = 11;
const PROGRAM_ERROR_12: u32 = 12;
const PROGRAM_ERROR_13: u32 = 13;
const PROGRAM_ERROR_14: u32 = 14;
const PROGRAM_ERROR_15: u32 = 15;
const PROGRAM_ERROR_16: u32 = 16;
const PROGRAM_ERROR_17: u32 = 17;
const PROGRAM_ERROR_18: u32 = 18;
const PROGRAM_ERROR_19: u32 = 19;
const ACCOUNT_SIZE_82: usize = 82;
const ACCOUNT_SIZE_165: usize = 165;
const ACCOUNT_SIZE_355: usize = 355;
const FIXED_COUNT_11: usize = 11;
fn role_matches(
    program: Address,
    meta: &AccountMeta,
    expected: &(bool, Option<bool>, bool),
) -> bool {
    let (writable, signer, optional) = *expected;

    if optional && meta.address == program {
        return meta.role == AccountRole::Readonly;
    }

    meta.role.is_writable() == writable
        && signer.is_none_or(|signer| meta.role.is_signer() == signer)
}
/// Finite u8 tagged enum.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccountState {
    /// The `Uninitialized` variant.
    Uninitialized,
    /// The `Initialized` variant.
    Initialized,
    /// The `Frozen` variant.
    Frozen,
}
/// Strict codec for [`AccountState`].
#[derive(Clone, Copy, Debug)]
pub struct AccountStateCodec;
impl AccountStateCodec {
    const VALUES: &'static [AccountState] = &[
        AccountState::Uninitialized,
        AccountState::Initialized,
        AccountState::Frozen,
    ];
}
impl Encoder<AccountState> for AccountStateCodec {
    fn encoded_len(&self, _: &AccountState) -> Result<usize, Error> {
        Ok(1)
    }
    fn write(&self, value: &AccountState, writer: &mut Writer<'_>) -> Result<(), Error> {
        let index = Self::VALUES
            .iter()
            .position(|candidate| candidate == value)
            .ok_or(Error::InvalidDiscriminator)?;
        let tag = u8::try_from(index).map_err(|_| Error::Overflow)?;
        Number::<u8>::LE.write(&tag, writer)
    }
}
impl Decoder<AccountState> for AccountStateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AccountState, Error> {
        Self::VALUES
            .get(usize::from(Number::<u8>::LE.read(reader)?))
            .cloned()
            .ok_or(Error::InvalidDiscriminator)
    }
}
/// Finite u8 tagged enum.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorityType {
    /// The `MintTokens` variant.
    MintTokens,
    /// The `FreezeAccount` variant.
    FreezeAccount,
    /// The `AccountOwner` variant.
    AccountOwner,
    /// The `CloseAccount` variant.
    CloseAccount,
}
/// Strict codec for [`AuthorityType`].
#[derive(Clone, Copy, Debug)]
pub struct AuthorityTypeCodec;
impl AuthorityTypeCodec {
    const VALUES: &'static [AuthorityType] = &[
        AuthorityType::MintTokens,
        AuthorityType::FreezeAccount,
        AuthorityType::AccountOwner,
        AuthorityType::CloseAccount,
    ];
}
impl Encoder<AuthorityType> for AuthorityTypeCodec {
    fn encoded_len(&self, _: &AuthorityType) -> Result<usize, Error> {
        Ok(1)
    }
    fn write(&self, value: &AuthorityType, writer: &mut Writer<'_>) -> Result<(), Error> {
        let index = Self::VALUES
            .iter()
            .position(|candidate| candidate == value)
            .ok_or(Error::InvalidDiscriminator)?;
        let tag = u8::try_from(index).map_err(|_| Error::Overflow)?;
        Number::<u8>::LE.write(&tag, writer)
    }
}
impl Decoder<AuthorityType> for AuthorityTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AuthorityType, Error> {
        Self::VALUES
            .get(usize::from(Number::<u8>::LE.read(reader)?))
            .cloned()
            .ok_or(Error::InvalidDiscriminator)
    }
}
/// Validated data for `BatchDataItem`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchDataItem {
    /// The `number_of_accounts` wire field.
    pub number_of_accounts: u8,
    /// The `instruction_data` wire field.
    pub instruction_data: Vec<u8>,
}
/// Bounded codec for [`BatchDataItem`].
#[derive(Clone, Copy, Debug)]
pub struct BatchDataItemCodec;
impl Encoder<BatchDataItem> for BatchDataItemCodec {
    fn encoded_len(&self, value: &BatchDataItem) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.number_of_accounts)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Bytes,
                    prefix: Mapped::new(
                        Number::<u8>::LE,
                        |value: &u64| u8::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u8| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.instruction_data.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &BatchDataItem, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.number_of_accounts, writer)?;
        ByteSizePrefix {
            item: Bytes,
            prefix: Mapped::new(
                Number::<u8>::LE,
                |value: &u64| u8::try_from(*value).map_err(|_| Error::Overflow),
                |value: u8| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.instruction_data.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<BatchDataItem> for BatchDataItemCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BatchDataItem, Error> {
        Ok(BatchDataItem {
            number_of_accounts: Number::<u8>::LE.read(reader)?,
            instruction_data: ByteSizePrefix {
                item: Bytes,
                prefix: Mapped::new(
                    Number::<u8>::LE,
                    |value: &u64| u8::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u8| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `InitializeMint`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeMint {
    /// The `decimals` wire field.
    pub decimals: u8,
    /// The `mint_authority` wire field.
    pub mint_authority: Address,
    /// The `freeze_authority` wire field.
    pub freeze_authority: Option<Address>,
}
/// Bounded codec for [`InitializeMint`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeMintCodec;
impl Encoder<InitializeMint> for InitializeMintCodec {
    fn encoded_len(&self, value: &InitializeMint) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.mint_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.freeze_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeMint, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_0]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint_authority, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.freeze_authority, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeMint> for InitializeMintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeMint, Error> {
        Constant(&[INSTRUCTION_TAG_0]).read(reader)?;
        Ok(InitializeMint {
            decimals: Number::<u8>::LE.read(reader)?,
            mint_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            freeze_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
        })
    }
}
impl InitializeMint {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 2 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* mint */ (true, Some(false), false),
                /* rent */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = InitializeMintCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        InitializeMintCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `InitializeAccount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeAccount {}
/// Bounded codec for [`InitializeAccount`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeAccountCodec;
impl Encoder<InitializeAccount> for InitializeAccountCodec {
    fn encoded_len(&self, value: &InitializeAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &InitializeAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_1]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<InitializeAccount> for InitializeAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeAccount, Error> {
        Constant(&[INSTRUCTION_TAG_1]).read(reader)?;
        let _ = reader;
        Ok(InitializeAccount {})
    }
}
impl InitializeAccount {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* account */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* owner */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = InitializeAccountCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        InitializeAccountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `InitializeMultisig`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeMultisig {
    /// The `m` wire field.
    pub m: u8,
}
/// Bounded codec for [`InitializeMultisig`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeMultisigCodec;
impl Encoder<InitializeMultisig> for InitializeMultisigCodec {
    fn encoded_len(&self, value: &InitializeMultisig) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.m)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeMultisig, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_2]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.m, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeMultisig> for InitializeMultisigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeMultisig, Error> {
        Constant(&[INSTRUCTION_TAG_2]).read(reader)?;
        Ok(InitializeMultisig {
            m: Number::<u8>::LE.read(reader)?,
        })
    }
}
impl InitializeMultisig {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 2 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* multisig */ (true, Some(false), false),
                /* rent */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[2..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = InitializeMultisigCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        InitializeMultisigCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Transfer`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transfer {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`Transfer`].
#[derive(Clone, Copy, Debug)]
pub struct TransferCodec;
impl Encoder<Transfer> for TransferCodec {
    fn encoded_len(&self, value: &Transfer) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Transfer, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_3]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<Transfer> for TransferCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Transfer, Error> {
        Constant(&[INSTRUCTION_TAG_3]).read(reader)?;
        Ok(Transfer {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
impl Transfer {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* source */ (true, Some(false), false),
                /* destination */ (true, Some(false), false),
                /* authority */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = TransferCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        TransferCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Approve`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Approve {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`Approve`].
#[derive(Clone, Copy, Debug)]
pub struct ApproveCodec;
impl Encoder<Approve> for ApproveCodec {
    fn encoded_len(&self, value: &Approve) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Approve, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_4]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<Approve> for ApproveCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Approve, Error> {
        Constant(&[INSTRUCTION_TAG_4]).read(reader)?;
        Ok(Approve {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
impl Approve {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* source */ (true, Some(false), false),
                /* delegate */ (false, Some(false), false),
                /* owner */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = ApproveCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        ApproveCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Revoke`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Revoke {}
/// Bounded codec for [`Revoke`].
#[derive(Clone, Copy, Debug)]
pub struct RevokeCodec;
impl Encoder<Revoke> for RevokeCodec {
    fn encoded_len(&self, value: &Revoke) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Revoke, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_5]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Revoke> for RevokeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Revoke, Error> {
        Constant(&[INSTRUCTION_TAG_5]).read(reader)?;
        let _ = reader;
        Ok(Revoke {})
    }
}
impl Revoke {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 2 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* source */ (true, Some(false), false),
                /* owner */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[2..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = RevokeCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        RevokeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetAuthority {
    /// The `authority_type` wire field.
    pub authority_type: AuthorityType,
    /// The `new_authority` wire field.
    pub new_authority: Option<Address>,
}
/// Bounded codec for [`SetAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct SetAuthorityCodec;
impl Encoder<SetAuthority> for SetAuthorityCodec {
    fn encoded_len(&self, value: &SetAuthority) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(AuthorityTypeCodec.encoded_len(&value.authority_type)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.new_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetAuthority, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_6]).write(&(), writer)?;
        AuthorityTypeCodec.write(&value.authority_type, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.new_authority, writer)?;
        Ok(())
    }
}
impl Decoder<SetAuthority> for SetAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetAuthority, Error> {
        Constant(&[INSTRUCTION_TAG_6]).read(reader)?;
        Ok(SetAuthority {
            authority_type: AuthorityTypeCodec.read(reader)?,
            new_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
        })
    }
}
impl SetAuthority {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 2 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* owned */ (true, Some(false), false),
                /* owner */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[2..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = SetAuthorityCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        SetAuthorityCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MintTo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintTo {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`MintTo`].
#[derive(Clone, Copy, Debug)]
pub struct MintToCodec;
impl Encoder<MintTo> for MintToCodec {
    fn encoded_len(&self, value: &MintTo) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MintTo, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_7]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<MintTo> for MintToCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MintTo, Error> {
        Constant(&[INSTRUCTION_TAG_7]).read(reader)?;
        Ok(MintTo {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
impl MintTo {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* mint */ (true, Some(false), false),
                /* token */ (true, Some(false), false),
                /* mint_authority */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = MintToCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        MintToCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Burn`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Burn {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`Burn`].
#[derive(Clone, Copy, Debug)]
pub struct BurnCodec;
impl Encoder<Burn> for BurnCodec {
    fn encoded_len(&self, value: &Burn) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Burn, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_8]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<Burn> for BurnCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Burn, Error> {
        Constant(&[INSTRUCTION_TAG_8]).read(reader)?;
        Ok(Burn {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
impl Burn {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* account */ (true, Some(false), false),
                /* mint */ (true, Some(false), false),
                /* authority */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = BurnCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        BurnCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CloseAccount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseAccount {}
/// Bounded codec for [`CloseAccount`].
#[derive(Clone, Copy, Debug)]
pub struct CloseAccountCodec;
impl Encoder<CloseAccount> for CloseAccountCodec {
    fn encoded_len(&self, value: &CloseAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CloseAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_9]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CloseAccount> for CloseAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseAccount, Error> {
        Constant(&[INSTRUCTION_TAG_9]).read(reader)?;
        let _ = reader;
        Ok(CloseAccount {})
    }
}
impl CloseAccount {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* account */ (true, Some(false), false),
                /* destination */ (true, Some(false), false),
                /* owner */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = CloseAccountCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        CloseAccountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `FreezeAccount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreezeAccount {}
/// Bounded codec for [`FreezeAccount`].
#[derive(Clone, Copy, Debug)]
pub struct FreezeAccountCodec;
impl Encoder<FreezeAccount> for FreezeAccountCodec {
    fn encoded_len(&self, value: &FreezeAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &FreezeAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_10]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<FreezeAccount> for FreezeAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<FreezeAccount, Error> {
        Constant(&[INSTRUCTION_TAG_10]).read(reader)?;
        let _ = reader;
        Ok(FreezeAccount {})
    }
}
impl FreezeAccount {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* account */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* owner */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = FreezeAccountCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        FreezeAccountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ThawAccount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThawAccount {}
/// Bounded codec for [`ThawAccount`].
#[derive(Clone, Copy, Debug)]
pub struct ThawAccountCodec;
impl Encoder<ThawAccount> for ThawAccountCodec {
    fn encoded_len(&self, value: &ThawAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &ThawAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_11]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ThawAccount> for ThawAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ThawAccount, Error> {
        Constant(&[INSTRUCTION_TAG_11]).read(reader)?;
        let _ = reader;
        Ok(ThawAccount {})
    }
}
impl ThawAccount {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* account */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* owner */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = ThawAccountCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        ThawAccountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TransferChecked`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferChecked {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
}
/// Bounded codec for [`TransferChecked`].
#[derive(Clone, Copy, Debug)]
pub struct TransferCheckedCodec;
impl Encoder<TransferChecked> for TransferCheckedCodec {
    fn encoded_len(&self, value: &TransferChecked) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_12]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Ok(())
    }
}
impl Decoder<TransferChecked> for TransferCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferChecked, Error> {
        Constant(&[INSTRUCTION_TAG_12]).read(reader)?;
        Ok(TransferChecked {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
        })
    }
}
impl TransferChecked {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* source */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* destination */ (true, Some(false), false),
                /* authority */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[4..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = TransferCheckedCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        TransferCheckedCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ApproveChecked`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApproveChecked {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
}
/// Bounded codec for [`ApproveChecked`].
#[derive(Clone, Copy, Debug)]
pub struct ApproveCheckedCodec;
impl Encoder<ApproveChecked> for ApproveCheckedCodec {
    fn encoded_len(&self, value: &ApproveChecked) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ApproveChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_13]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Ok(())
    }
}
impl Decoder<ApproveChecked> for ApproveCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ApproveChecked, Error> {
        Constant(&[INSTRUCTION_TAG_13]).read(reader)?;
        Ok(ApproveChecked {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
        })
    }
}
impl ApproveChecked {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* source */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* delegate */ (false, Some(false), false),
                /* owner */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[4..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = ApproveCheckedCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        ApproveCheckedCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MintToChecked`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintToChecked {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
}
/// Bounded codec for [`MintToChecked`].
#[derive(Clone, Copy, Debug)]
pub struct MintToCheckedCodec;
impl Encoder<MintToChecked> for MintToCheckedCodec {
    fn encoded_len(&self, value: &MintToChecked) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MintToChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_14]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Ok(())
    }
}
impl Decoder<MintToChecked> for MintToCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MintToChecked, Error> {
        Constant(&[INSTRUCTION_TAG_14]).read(reader)?;
        Ok(MintToChecked {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
        })
    }
}
impl MintToChecked {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* mint */ (true, Some(false), false),
                /* token */ (true, Some(false), false),
                /* mint_authority */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = MintToCheckedCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        MintToCheckedCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `BurnChecked`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnChecked {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
}
/// Bounded codec for [`BurnChecked`].
#[derive(Clone, Copy, Debug)]
pub struct BurnCheckedCodec;
impl Encoder<BurnChecked> for BurnCheckedCodec {
    fn encoded_len(&self, value: &BurnChecked) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &BurnChecked, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_15]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Ok(())
    }
}
impl Decoder<BurnChecked> for BurnCheckedCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BurnChecked, Error> {
        Constant(&[INSTRUCTION_TAG_15]).read(reader)?;
        Ok(BurnChecked {
            amount: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
        })
    }
}
impl BurnChecked {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* account */ (true, Some(false), false),
                /* mint */ (true, Some(false), false),
                /* authority */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = BurnCheckedCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        BurnCheckedCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `InitializeAccount2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeAccount2 {
    /// The `owner` wire field.
    pub owner: Address,
}
/// Bounded codec for [`InitializeAccount2`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeAccount2Codec;
impl Encoder<InitializeAccount2> for InitializeAccount2Codec {
    fn encoded_len(&self, value: &InitializeAccount2) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.owner)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeAccount2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_16]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.owner, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeAccount2> for InitializeAccount2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeAccount2, Error> {
        Constant(&[INSTRUCTION_TAG_16]).read(reader)?;
        Ok(InitializeAccount2 {
            owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
impl InitializeAccount2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* account */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = InitializeAccount2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        InitializeAccount2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SyncNative`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyncNative {}
/// Bounded codec for [`SyncNative`].
#[derive(Clone, Copy, Debug)]
pub struct SyncNativeCodec;
impl Encoder<SyncNative> for SyncNativeCodec {
    fn encoded_len(&self, value: &SyncNative) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &SyncNative, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_17]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<SyncNative> for SyncNativeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SyncNative, Error> {
        Constant(&[INSTRUCTION_TAG_17]).read(reader)?;
        let _ = reader;
        Ok(SyncNative {})
    }
}
impl SyncNative {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.is_empty() || accounts.len() > 2 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* account */ (true, Some(false), false),
                /* rent */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = SyncNativeCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        SyncNativeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `InitializeAccount3`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeAccount3 {
    /// The `owner` wire field.
    pub owner: Address,
}
/// Bounded codec for [`InitializeAccount3`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeAccount3Codec;
impl Encoder<InitializeAccount3> for InitializeAccount3Codec {
    fn encoded_len(&self, value: &InitializeAccount3) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.owner)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeAccount3, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_18]).write(&(), writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.owner, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeAccount3> for InitializeAccount3Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeAccount3, Error> {
        Constant(&[INSTRUCTION_TAG_18]).read(reader)?;
        Ok(InitializeAccount3 {
            owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
impl InitializeAccount3 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 2 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* account */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = InitializeAccount3Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        InitializeAccount3Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `InitializeMultisig2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeMultisig2 {
    /// The `m` wire field.
    pub m: u8,
}
/// Bounded codec for [`InitializeMultisig2`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeMultisig2Codec;
impl Encoder<InitializeMultisig2> for InitializeMultisig2Codec {
    fn encoded_len(&self, value: &InitializeMultisig2) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.m)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeMultisig2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_19]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.m, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeMultisig2> for InitializeMultisig2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeMultisig2, Error> {
        Constant(&[INSTRUCTION_TAG_19]).read(reader)?;
        Ok(InitializeMultisig2 {
            m: Number::<u8>::LE.read(reader)?,
        })
    }
}
impl InitializeMultisig2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.is_empty() {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([/* multisig */ (true, Some(false), false)])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[1..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = InitializeMultisig2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        InitializeMultisig2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `InitializeMint2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeMint2 {
    /// The `decimals` wire field.
    pub decimals: u8,
    /// The `mint_authority` wire field.
    pub mint_authority: Address,
    /// The `freeze_authority` wire field.
    pub freeze_authority: Option<Address>,
}
/// Bounded codec for [`InitializeMint2`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeMint2Codec;
impl Encoder<InitializeMint2> for InitializeMint2Codec {
    fn encoded_len(&self, value: &InitializeMint2) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.mint_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.freeze_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &InitializeMint2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_20]).write(&(), writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint_authority, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.freeze_authority, writer)?;
        Ok(())
    }
}
impl Decoder<InitializeMint2> for InitializeMint2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeMint2, Error> {
        Constant(&[INSTRUCTION_TAG_20]).read(reader)?;
        Ok(InitializeMint2 {
            decimals: Number::<u8>::LE.read(reader)?,
            mint_authority: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            freeze_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
        })
    }
}
impl InitializeMint2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 1 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([/* mint */ (true, Some(false), false)])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = InitializeMint2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        InitializeMint2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `GetAccountDataSize`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GetAccountDataSize {}
/// Bounded codec for [`GetAccountDataSize`].
#[derive(Clone, Copy, Debug)]
pub struct GetAccountDataSizeCodec;
impl Encoder<GetAccountDataSize> for GetAccountDataSizeCodec {
    fn encoded_len(&self, value: &GetAccountDataSize) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &GetAccountDataSize, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_21]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<GetAccountDataSize> for GetAccountDataSizeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<GetAccountDataSize, Error> {
        Constant(&[INSTRUCTION_TAG_21]).read(reader)?;
        let _ = reader;
        Ok(GetAccountDataSize {})
    }
}
impl GetAccountDataSize {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 1 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([/* mint */ (false, Some(false), false)])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = GetAccountDataSizeCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        GetAccountDataSizeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `InitializeImmutableOwner`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeImmutableOwner {}
/// Bounded codec for [`InitializeImmutableOwner`].
#[derive(Clone, Copy, Debug)]
pub struct InitializeImmutableOwnerCodec;
impl Encoder<InitializeImmutableOwner> for InitializeImmutableOwnerCodec {
    fn encoded_len(&self, value: &InitializeImmutableOwner) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &InitializeImmutableOwner,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_22]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<InitializeImmutableOwner> for InitializeImmutableOwnerCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InitializeImmutableOwner, Error> {
        Constant(&[INSTRUCTION_TAG_22]).read(reader)?;
        let _ = reader;
        Ok(InitializeImmutableOwner {})
    }
}
impl InitializeImmutableOwner {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 1 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([/* account */ (true, Some(false), false)])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = InitializeImmutableOwnerCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        InitializeImmutableOwnerCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `AmountToUiAmount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AmountToUiAmount {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`AmountToUiAmount`].
#[derive(Clone, Copy, Debug)]
pub struct AmountToUiAmountCodec;
impl Encoder<AmountToUiAmount> for AmountToUiAmountCodec {
    fn encoded_len(&self, value: &AmountToUiAmount) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AmountToUiAmount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_23]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<AmountToUiAmount> for AmountToUiAmountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AmountToUiAmount, Error> {
        Constant(&[INSTRUCTION_TAG_23]).read(reader)?;
        Ok(AmountToUiAmount {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
impl AmountToUiAmount {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 1 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([/* mint */ (false, Some(false), false)])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = AmountToUiAmountCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        AmountToUiAmountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UiAmountToAmount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAmountToAmount {
    /// The `ui_amount` wire field.
    pub ui_amount: String,
}
/// Bounded codec for [`UiAmountToAmount`].
#[derive(Clone, Copy, Debug)]
pub struct UiAmountToAmountCodec;
impl Encoder<UiAmountToAmount> for UiAmountToAmountCodec {
    fn encoded_len(&self, value: &UiAmountToAmount) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Utf8.encoded_len(value.ui_amount.as_str())?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UiAmountToAmount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_24]).write(&(), writer)?;
        Utf8.write(value.ui_amount.as_str(), writer)?;
        Ok(())
    }
}
impl Decoder<UiAmountToAmount> for UiAmountToAmountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UiAmountToAmount, Error> {
        Constant(&[INSTRUCTION_TAG_24]).read(reader)?;
        Ok(UiAmountToAmount {
            ui_amount: Utf8.read(reader)?,
        })
    }
}
impl UiAmountToAmount {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 1 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([/* mint */ (false, Some(false), false)])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = UiAmountToAmountCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        UiAmountToAmountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `WithdrawExcessLamports`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawExcessLamports {}
/// Bounded codec for [`WithdrawExcessLamports`].
#[derive(Clone, Copy, Debug)]
pub struct WithdrawExcessLamportsCodec;
impl Encoder<WithdrawExcessLamports> for WithdrawExcessLamportsCodec {
    fn encoded_len(&self, value: &WithdrawExcessLamports) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &WithdrawExcessLamports, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_38]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<WithdrawExcessLamports> for WithdrawExcessLamportsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<WithdrawExcessLamports, Error> {
        Constant(&[INSTRUCTION_TAG_38]).read(reader)?;
        let _ = reader;
        Ok(WithdrawExcessLamports {})
    }
}
impl WithdrawExcessLamports {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* source */ (true, Some(false), false),
                /* destination */ (true, Some(false), false),
                /* authority */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = WithdrawExcessLamportsCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        WithdrawExcessLamportsCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UnwrapLamports`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnwrapLamports {
    /// The `amount` wire field.
    pub amount: Option<u64>,
}
/// Bounded codec for [`UnwrapLamports`].
#[derive(Clone, Copy, Debug)]
pub struct UnwrapLamportsCodec;
impl Encoder<UnwrapLamports> for UnwrapLamportsCodec {
    fn encoded_len(&self, value: &UnwrapLamports) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UnwrapLamports, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_45]).write(&(), writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<UnwrapLamports> for UnwrapLamportsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UnwrapLamports, Error> {
        Constant(&[INSTRUCTION_TAG_45]).read(reader)?;
        Ok(UnwrapLamports {
            amount: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
        })
    }
}
impl UnwrapLamports {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* source */ (true, Some(false), false),
                /* destination */ (true, Some(false), false),
                /* authority */ (false, None, false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(true, false);

        if accounts[3..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = UnwrapLamportsCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        UnwrapLamportsCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Batch`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Batch {
    /// The `data` wire field.
    pub data: Vec<BatchDataItem>,
}
/// Bounded codec for [`Batch`].
#[derive(Clone, Copy, Debug)]
pub struct BatchCodec;
impl Encoder<Batch> for BatchCodec {
    fn encoded_len(&self, value: &Batch) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: BatchDataItemCodec,
                    count: Count::Remainder,
                })
                .encoded_len(&value.data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Batch, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_255]).write(&(), writer)?;
        OwnedSequence(Sequence {
            item: BatchDataItemCodec,
            count: Count::Remainder,
        })
        .write(&value.data, writer)?;
        Ok(())
    }
}
impl Decoder<Batch> for BatchCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Batch, Error> {
        Constant(&[INSTRUCTION_TAG_255]).read(reader)?;
        Ok(Batch {
            data: OwnedSequence(Sequence {
                item: BatchDataItemCodec,
                count: Count::Remainder,
            })
            .read(reader)?,
        })
    }
}
impl Batch {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if !accounts.is_empty() {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = BatchCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        BatchCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Mint`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mint {
    /// The `mint_authority` wire field.
    pub mint_authority: Option<Address>,
    /// The `supply` wire field.
    pub supply: u64,
    /// The `decimals` wire field.
    pub decimals: u8,
    /// The `is_initialized` wire field.
    pub is_initialized: bool,
    /// The `freeze_authority` wire field.
    pub freeze_authority: Option<Address>,
}
/// Bounded codec for [`Mint`].
#[derive(Clone, Copy, Debug)]
pub struct MintCodec;
impl Encoder<Mint> for MintCodec {
    fn encoded_len(&self, value: &Mint) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::c_option(
                    Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    32,
                )
                .encoded_len(&value.mint_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.supply)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_initialized)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::c_option(
                    Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    32,
                )
                .encoded_len(&value.freeze_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Mint, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::c_option(
            Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            32,
        )
        .write(&value.mint_authority, writer)?;
        Number::<u64>::LE.write(&value.supply, writer)?;
        Number::<u8>::LE.write(&value.decimals, writer)?;
        Bool.write(&value.is_initialized, writer)?;
        OptionCodec::c_option(
            Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            32,
        )
        .write(&value.freeze_authority, writer)?;
        Ok(())
    }
}
impl Decoder<Mint> for MintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Mint, Error> {
        Ok(Mint {
            mint_authority: OptionCodec::c_option(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                32,
            )
            .read(reader)?,
            supply: Number::<u64>::LE.read(reader)?,
            decimals: Number::<u8>::LE.read(reader)?,
            is_initialized: Bool.read(reader)?,
            freeze_authority: OptionCodec::c_option(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                32,
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `Token`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    /// The `mint` wire field.
    pub mint: Address,
    /// The `owner` wire field.
    pub owner: Address,
    /// The `amount` wire field.
    pub amount: u64,
    /// The `delegate` wire field.
    pub delegate: Option<Address>,
    /// The `state` wire field.
    pub state: AccountState,
    /// The `is_native` wire field.
    pub is_native: Option<u64>,
    /// The `delegated_amount` wire field.
    pub delegated_amount: u64,
    /// The `close_authority` wire field.
    pub close_authority: Option<Address>,
}
/// Bounded codec for [`Token`].
#[derive(Clone, Copy, Debug)]
pub struct TokenCodec;
impl Encoder<Token> for TokenCodec {
    fn encoded_len(&self, value: &Token) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.mint)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.owner)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::c_option(
                    Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    32,
                )
                .encoded_len(&value.delegate)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(AccountStateCodec.encoded_len(&value.state)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::c_option(Number::<u64>::LE, 8).encoded_len(&value.is_native)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.delegated_amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::c_option(
                    Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    32,
                )
                .encoded_len(&value.close_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Token, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.owner, writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        OptionCodec::c_option(
            Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            32,
        )
        .write(&value.delegate, writer)?;
        AccountStateCodec.write(&value.state, writer)?;
        OptionCodec::c_option(Number::<u64>::LE, 8).write(&value.is_native, writer)?;
        Number::<u64>::LE.write(&value.delegated_amount, writer)?;
        OptionCodec::c_option(
            Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            32,
        )
        .write(&value.close_authority, writer)?;
        Ok(())
    }
}
impl Decoder<Token> for TokenCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Token, Error> {
        Ok(Token {
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            amount: Number::<u64>::LE.read(reader)?,
            delegate: OptionCodec::c_option(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                32,
            )
            .read(reader)?,
            state: AccountStateCodec.read(reader)?,
            is_native: OptionCodec::c_option(Number::<u64>::LE, 8).read(reader)?,
            delegated_amount: Number::<u64>::LE.read(reader)?,
            close_authority: OptionCodec::c_option(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                32,
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `Multisig`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Multisig {
    /// The `m` wire field.
    pub m: u8,
    /// The `n` wire field.
    pub n: u8,
    /// The `is_initialized` wire field.
    pub is_initialized: bool,
    /// The `signers` wire field.
    pub signers: Vec<Address>,
}
/// Bounded codec for [`Multisig`].
#[derive(Clone, Copy, Debug)]
pub struct MultisigCodec;
impl Encoder<Multisig> for MultisigCodec {
    fn encoded_len(&self, value: &Multisig) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.m)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.n)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_initialized)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    count: Count::Fixed(FIXED_COUNT_11),
                })
                .encoded_len(&value.signers)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Multisig, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(&value.m, writer)?;
        Number::<u8>::LE.write(&value.n, writer)?;
        Bool.write(&value.is_initialized, writer)?;
        OwnedSequence(Sequence {
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            count: Count::Fixed(FIXED_COUNT_11),
        })
        .write(&value.signers, writer)?;
        Ok(())
    }
}
impl Decoder<Multisig> for MultisigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Multisig, Error> {
        Ok(Multisig {
            m: Number::<u8>::LE.read(reader)?,
            n: Number::<u8>::LE.read(reader)?,
            is_initialized: Bool.read(reader)?,
            signers: OwnedSequence(Sequence {
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                count: Count::Fixed(FIXED_COUNT_11),
            })
            .read(reader)?,
        })
    }
}
/// Program error codes. Unknown numeric codes remain lossless.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenError {
    /// The `NotRentExempt` program error.
    NotRentExempt,
    /// The `InsufficientFunds` program error.
    InsufficientFunds,
    /// The `InvalidMint` program error.
    InvalidMint,
    /// The `MintMismatch` program error.
    MintMismatch,
    /// The `OwnerMismatch` program error.
    OwnerMismatch,
    /// The `FixedSupply` program error.
    FixedSupply,
    /// The `AlreadyInUse` program error.
    AlreadyInUse,
    /// The `InvalidNumberOfProvidedSigners` program error.
    InvalidNumberOfProvidedSigners,
    /// The `InvalidNumberOfRequiredSigners` program error.
    InvalidNumberOfRequiredSigners,
    /// The `UninitializedState` program error.
    UninitializedState,
    /// The `NativeNotSupported` program error.
    NativeNotSupported,
    /// The `NonNativeHasBalance` program error.
    NonNativeHasBalance,
    /// The `InvalidInstruction` program error.
    InvalidInstruction,
    /// The `InvalidState` program error.
    InvalidState,
    /// The `Overflow` program error.
    Overflow,
    /// The `AuthorityTypeNotSupported` program error.
    AuthorityTypeNotSupported,
    /// The `MintCannotFreeze` program error.
    MintCannotFreeze,
    /// The `AccountFrozen` program error.
    AccountFrozen,
    /// The `MintDecimalsMismatch` program error.
    MintDecimalsMismatch,
    /// The `NonNativeNotSupported` program error.
    NonNativeNotSupported,
    /// An unrecognized program error code.
    Unknown(u32),
}
impl TokenError {
    /// Decodes known or unknown numeric program error codes.
    pub const fn from_code(code: u32) -> Self {
        match code {
            PROGRAM_ERROR_0 => Self::NotRentExempt,
            PROGRAM_ERROR_1 => Self::InsufficientFunds,
            PROGRAM_ERROR_2 => Self::InvalidMint,
            PROGRAM_ERROR_3 => Self::MintMismatch,
            PROGRAM_ERROR_4 => Self::OwnerMismatch,
            PROGRAM_ERROR_5 => Self::FixedSupply,
            PROGRAM_ERROR_6 => Self::AlreadyInUse,
            PROGRAM_ERROR_7 => Self::InvalidNumberOfProvidedSigners,
            PROGRAM_ERROR_8 => Self::InvalidNumberOfRequiredSigners,
            PROGRAM_ERROR_9 => Self::UninitializedState,
            PROGRAM_ERROR_10 => Self::NativeNotSupported,
            PROGRAM_ERROR_11 => Self::NonNativeHasBalance,
            PROGRAM_ERROR_12 => Self::InvalidInstruction,
            PROGRAM_ERROR_13 => Self::InvalidState,
            PROGRAM_ERROR_14 => Self::Overflow,
            PROGRAM_ERROR_15 => Self::AuthorityTypeNotSupported,
            PROGRAM_ERROR_16 => Self::MintCannotFreeze,
            PROGRAM_ERROR_17 => Self::AccountFrozen,
            PROGRAM_ERROR_18 => Self::MintDecimalsMismatch,
            PROGRAM_ERROR_19 => Self::NonNativeNotSupported,
            code => Self::Unknown(code),
        }
    }
    /// Returns the exact numeric code.
    pub const fn code(self) -> u32 {
        match self {
            Self::NotRentExempt => PROGRAM_ERROR_0,
            Self::InsufficientFunds => PROGRAM_ERROR_1,
            Self::InvalidMint => PROGRAM_ERROR_2,
            Self::MintMismatch => PROGRAM_ERROR_3,
            Self::OwnerMismatch => PROGRAM_ERROR_4,
            Self::FixedSupply => PROGRAM_ERROR_5,
            Self::AlreadyInUse => PROGRAM_ERROR_6,
            Self::InvalidNumberOfProvidedSigners => PROGRAM_ERROR_7,
            Self::InvalidNumberOfRequiredSigners => PROGRAM_ERROR_8,
            Self::UninitializedState => PROGRAM_ERROR_9,
            Self::NativeNotSupported => PROGRAM_ERROR_10,
            Self::NonNativeHasBalance => PROGRAM_ERROR_11,
            Self::InvalidInstruction => PROGRAM_ERROR_12,
            Self::InvalidState => PROGRAM_ERROR_13,
            Self::Overflow => PROGRAM_ERROR_14,
            Self::AuthorityTypeNotSupported => PROGRAM_ERROR_15,
            Self::MintCannotFreeze => PROGRAM_ERROR_16,
            Self::AccountFrozen => PROGRAM_ERROR_17,
            Self::MintDecimalsMismatch => PROGRAM_ERROR_18,
            Self::NonNativeNotSupported => PROGRAM_ERROR_19,
            Self::Unknown(code) => code,
        }
    }
    /// Returns the source message for known errors.
    pub const fn message(self) -> Option<&'static str> {
        match self {
            Self::NotRentExempt => Some("Lamport balance below rent-exempt threshold"),
            Self::InsufficientFunds => Some("Insufficient funds"),
            Self::InvalidMint => Some("Invalid Mint"),
            Self::MintMismatch => Some("Account not associated with this Mint"),
            Self::OwnerMismatch => Some("Owner does not match"),
            Self::FixedSupply => Some("Fixed supply"),
            Self::AlreadyInUse => Some("Already in use"),
            Self::InvalidNumberOfProvidedSigners => Some("Invalid number of provided signers"),
            Self::InvalidNumberOfRequiredSigners => Some("Invalid number of required signers"),
            Self::UninitializedState => Some("State is unititialized"),
            Self::NativeNotSupported => Some("Instruction does not support native tokens"),
            Self::NonNativeHasBalance => {
                Some("Non-native account can only be closed if its balance is zero")
            }
            Self::InvalidInstruction => Some("Invalid instruction"),
            Self::InvalidState => Some("State is invalid for requested operation"),
            Self::Overflow => Some("Operation overflowed"),
            Self::AuthorityTypeNotSupported => {
                Some("Account does not support specified authority type")
            }
            Self::MintCannotFreeze => Some("This token mint cannot freeze accounts"),
            Self::AccountFrozen => Some("Account is frozen"),
            Self::MintDecimalsMismatch => {
                Some("The provided decimals value different from the Mint decimals")
            }
            Self::NonNativeNotSupported => Some("Instruction does not support non-native tokens"),
            Self::Unknown(_) => None,
        }
    }
}
/// Unique account size discriminators; identification does not validate fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenAccountKind {
    /// Account of 82 bytes.
    Mint,
    /// Account of 165 bytes.
    Token,
    /// Account of 355 bytes.
    Multisig,
}
/// Identifies an account by its explicit size discriminator.
pub fn identify_account(data: &[u8]) -> Result<TokenAccountKind, Error> {
    match data.len() {
        ACCOUNT_SIZE_82 => Ok(TokenAccountKind::Mint),
        ACCOUNT_SIZE_165 => Ok(TokenAccountKind::Token),
        ACCOUNT_SIZE_355 => Ok(TokenAccountKind::Multisig),
        _ => Err(Error::InvalidDiscriminator),
    }
}
/// All selected instruction data variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenInstructionData {
    /// Data for tag INSTRUCTION_TAG_0.
    InitializeMint(InitializeMint),
    /// Data for tag INSTRUCTION_TAG_1.
    InitializeAccount(InitializeAccount),
    /// Data for tag INSTRUCTION_TAG_2.
    InitializeMultisig(InitializeMultisig),
    /// Data for tag INSTRUCTION_TAG_3.
    Transfer(Transfer),
    /// Data for tag INSTRUCTION_TAG_4.
    Approve(Approve),
    /// Data for tag INSTRUCTION_TAG_5.
    Revoke(Revoke),
    /// Data for tag INSTRUCTION_TAG_6.
    SetAuthority(SetAuthority),
    /// Data for tag INSTRUCTION_TAG_7.
    MintTo(MintTo),
    /// Data for tag INSTRUCTION_TAG_8.
    Burn(Burn),
    /// Data for tag INSTRUCTION_TAG_9.
    CloseAccount(CloseAccount),
    /// Data for tag INSTRUCTION_TAG_10.
    FreezeAccount(FreezeAccount),
    /// Data for tag INSTRUCTION_TAG_11.
    ThawAccount(ThawAccount),
    /// Data for tag INSTRUCTION_TAG_12.
    TransferChecked(TransferChecked),
    /// Data for tag INSTRUCTION_TAG_13.
    ApproveChecked(ApproveChecked),
    /// Data for tag INSTRUCTION_TAG_14.
    MintToChecked(MintToChecked),
    /// Data for tag INSTRUCTION_TAG_15.
    BurnChecked(BurnChecked),
    /// Data for tag INSTRUCTION_TAG_16.
    InitializeAccount2(InitializeAccount2),
    /// Data for tag INSTRUCTION_TAG_17.
    SyncNative(SyncNative),
    /// Data for tag INSTRUCTION_TAG_18.
    InitializeAccount3(InitializeAccount3),
    /// Data for tag INSTRUCTION_TAG_19.
    InitializeMultisig2(InitializeMultisig2),
    /// Data for tag INSTRUCTION_TAG_20.
    InitializeMint2(InitializeMint2),
    /// Data for tag INSTRUCTION_TAG_21.
    GetAccountDataSize(GetAccountDataSize),
    /// Data for tag INSTRUCTION_TAG_22.
    InitializeImmutableOwner(InitializeImmutableOwner),
    /// Data for tag INSTRUCTION_TAG_23.
    AmountToUiAmount(AmountToUiAmount),
    /// Data for tag INSTRUCTION_TAG_24.
    UiAmountToAmount(UiAmountToAmount),
    /// Data for tag INSTRUCTION_TAG_38.
    WithdrawExcessLamports(WithdrawExcessLamports),
    /// Data for tag INSTRUCTION_TAG_45.
    UnwrapLamports(UnwrapLamports),
    /// Data for tag INSTRUCTION_TAG_255.
    Batch(Batch),
}
/// Strict bounded instruction dispatch codec.
#[derive(Clone, Copy, Debug)]
pub struct TokenInstructionDataCodec;
impl Encoder<TokenInstructionData> for TokenInstructionDataCodec {
    fn encoded_len(&self, value: &TokenInstructionData) -> Result<usize, Error> {
        match value {
            TokenInstructionData::InitializeMint(value) => InitializeMintCodec.encoded_len(value),
            TokenInstructionData::InitializeAccount(value) => {
                InitializeAccountCodec.encoded_len(value)
            }
            TokenInstructionData::InitializeMultisig(value) => {
                InitializeMultisigCodec.encoded_len(value)
            }
            TokenInstructionData::Transfer(value) => TransferCodec.encoded_len(value),
            TokenInstructionData::Approve(value) => ApproveCodec.encoded_len(value),
            TokenInstructionData::Revoke(value) => RevokeCodec.encoded_len(value),
            TokenInstructionData::SetAuthority(value) => SetAuthorityCodec.encoded_len(value),
            TokenInstructionData::MintTo(value) => MintToCodec.encoded_len(value),
            TokenInstructionData::Burn(value) => BurnCodec.encoded_len(value),
            TokenInstructionData::CloseAccount(value) => CloseAccountCodec.encoded_len(value),
            TokenInstructionData::FreezeAccount(value) => FreezeAccountCodec.encoded_len(value),
            TokenInstructionData::ThawAccount(value) => ThawAccountCodec.encoded_len(value),
            TokenInstructionData::TransferChecked(value) => TransferCheckedCodec.encoded_len(value),
            TokenInstructionData::ApproveChecked(value) => ApproveCheckedCodec.encoded_len(value),
            TokenInstructionData::MintToChecked(value) => MintToCheckedCodec.encoded_len(value),
            TokenInstructionData::BurnChecked(value) => BurnCheckedCodec.encoded_len(value),
            TokenInstructionData::InitializeAccount2(value) => {
                InitializeAccount2Codec.encoded_len(value)
            }
            TokenInstructionData::SyncNative(value) => SyncNativeCodec.encoded_len(value),
            TokenInstructionData::InitializeAccount3(value) => {
                InitializeAccount3Codec.encoded_len(value)
            }
            TokenInstructionData::InitializeMultisig2(value) => {
                InitializeMultisig2Codec.encoded_len(value)
            }
            TokenInstructionData::InitializeMint2(value) => InitializeMint2Codec.encoded_len(value),
            TokenInstructionData::GetAccountDataSize(value) => {
                GetAccountDataSizeCodec.encoded_len(value)
            }
            TokenInstructionData::InitializeImmutableOwner(value) => {
                InitializeImmutableOwnerCodec.encoded_len(value)
            }
            TokenInstructionData::AmountToUiAmount(value) => {
                AmountToUiAmountCodec.encoded_len(value)
            }
            TokenInstructionData::UiAmountToAmount(value) => {
                UiAmountToAmountCodec.encoded_len(value)
            }
            TokenInstructionData::WithdrawExcessLamports(value) => {
                WithdrawExcessLamportsCodec.encoded_len(value)
            }
            TokenInstructionData::UnwrapLamports(value) => UnwrapLamportsCodec.encoded_len(value),
            TokenInstructionData::Batch(value) => BatchCodec.encoded_len(value),
        }
    }
    fn write(&self, value: &TokenInstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            TokenInstructionData::InitializeMint(value) => InitializeMintCodec.write(value, writer),
            TokenInstructionData::InitializeAccount(value) => {
                InitializeAccountCodec.write(value, writer)
            }
            TokenInstructionData::InitializeMultisig(value) => {
                InitializeMultisigCodec.write(value, writer)
            }
            TokenInstructionData::Transfer(value) => TransferCodec.write(value, writer),
            TokenInstructionData::Approve(value) => ApproveCodec.write(value, writer),
            TokenInstructionData::Revoke(value) => RevokeCodec.write(value, writer),
            TokenInstructionData::SetAuthority(value) => SetAuthorityCodec.write(value, writer),
            TokenInstructionData::MintTo(value) => MintToCodec.write(value, writer),
            TokenInstructionData::Burn(value) => BurnCodec.write(value, writer),
            TokenInstructionData::CloseAccount(value) => CloseAccountCodec.write(value, writer),
            TokenInstructionData::FreezeAccount(value) => FreezeAccountCodec.write(value, writer),
            TokenInstructionData::ThawAccount(value) => ThawAccountCodec.write(value, writer),
            TokenInstructionData::TransferChecked(value) => {
                TransferCheckedCodec.write(value, writer)
            }
            TokenInstructionData::ApproveChecked(value) => ApproveCheckedCodec.write(value, writer),
            TokenInstructionData::MintToChecked(value) => MintToCheckedCodec.write(value, writer),
            TokenInstructionData::BurnChecked(value) => BurnCheckedCodec.write(value, writer),
            TokenInstructionData::InitializeAccount2(value) => {
                InitializeAccount2Codec.write(value, writer)
            }
            TokenInstructionData::SyncNative(value) => SyncNativeCodec.write(value, writer),
            TokenInstructionData::InitializeAccount3(value) => {
                InitializeAccount3Codec.write(value, writer)
            }
            TokenInstructionData::InitializeMultisig2(value) => {
                InitializeMultisig2Codec.write(value, writer)
            }
            TokenInstructionData::InitializeMint2(value) => {
                InitializeMint2Codec.write(value, writer)
            }
            TokenInstructionData::GetAccountDataSize(value) => {
                GetAccountDataSizeCodec.write(value, writer)
            }
            TokenInstructionData::InitializeImmutableOwner(value) => {
                InitializeImmutableOwnerCodec.write(value, writer)
            }
            TokenInstructionData::AmountToUiAmount(value) => {
                AmountToUiAmountCodec.write(value, writer)
            }
            TokenInstructionData::UiAmountToAmount(value) => {
                UiAmountToAmountCodec.write(value, writer)
            }
            TokenInstructionData::WithdrawExcessLamports(value) => {
                WithdrawExcessLamportsCodec.write(value, writer)
            }
            TokenInstructionData::UnwrapLamports(value) => UnwrapLamportsCodec.write(value, writer),
            TokenInstructionData::Batch(value) => BatchCodec.write(value, writer),
        }
    }
}
impl Decoder<TokenInstructionData> for TokenInstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TokenInstructionData, Error> {
        let tag = *reader.remaining().first().ok_or(Error::Truncated)?;

        match tag {
            INSTRUCTION_TAG_0 => Ok(TokenInstructionData::InitializeMint(
                InitializeMintCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_1 => Ok(TokenInstructionData::InitializeAccount(
                InitializeAccountCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_2 => Ok(TokenInstructionData::InitializeMultisig(
                InitializeMultisigCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_3 => Ok(TokenInstructionData::Transfer(TransferCodec.read(reader)?)),
            INSTRUCTION_TAG_4 => Ok(TokenInstructionData::Approve(ApproveCodec.read(reader)?)),
            INSTRUCTION_TAG_5 => Ok(TokenInstructionData::Revoke(RevokeCodec.read(reader)?)),
            INSTRUCTION_TAG_6 => Ok(TokenInstructionData::SetAuthority(
                SetAuthorityCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_7 => Ok(TokenInstructionData::MintTo(MintToCodec.read(reader)?)),
            INSTRUCTION_TAG_8 => Ok(TokenInstructionData::Burn(BurnCodec.read(reader)?)),
            INSTRUCTION_TAG_9 => Ok(TokenInstructionData::CloseAccount(
                CloseAccountCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_10 => Ok(TokenInstructionData::FreezeAccount(
                FreezeAccountCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_11 => Ok(TokenInstructionData::ThawAccount(
                ThawAccountCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_12 => Ok(TokenInstructionData::TransferChecked(
                TransferCheckedCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_13 => Ok(TokenInstructionData::ApproveChecked(
                ApproveCheckedCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_14 => Ok(TokenInstructionData::MintToChecked(
                MintToCheckedCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_15 => Ok(TokenInstructionData::BurnChecked(
                BurnCheckedCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_16 => Ok(TokenInstructionData::InitializeAccount2(
                InitializeAccount2Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_17 => Ok(TokenInstructionData::SyncNative(
                SyncNativeCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_18 => Ok(TokenInstructionData::InitializeAccount3(
                InitializeAccount3Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_19 => Ok(TokenInstructionData::InitializeMultisig2(
                InitializeMultisig2Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_20 => Ok(TokenInstructionData::InitializeMint2(
                InitializeMint2Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_21 => Ok(TokenInstructionData::GetAccountDataSize(
                GetAccountDataSizeCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_22 => Ok(TokenInstructionData::InitializeImmutableOwner(
                InitializeImmutableOwnerCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_23 => Ok(TokenInstructionData::AmountToUiAmount(
                AmountToUiAmountCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_24 => Ok(TokenInstructionData::UiAmountToAmount(
                UiAmountToAmountCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_38 => Ok(TokenInstructionData::WithdrawExcessLamports(
                WithdrawExcessLamportsCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_45 => Ok(TokenInstructionData::UnwrapLamports(
                UnwrapLamportsCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_255 => Ok(TokenInstructionData::Batch(BatchCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
}
/// Identifies a tag without asserting the full payload is valid.
pub fn identify_instruction(data: &[u8]) -> Result<u8, Error> {
    let tag = *data.first().ok_or(Error::Truncated)?;

    match tag {
        INSTRUCTION_TAG_0 => Ok(tag),
        INSTRUCTION_TAG_1 => Ok(tag),
        INSTRUCTION_TAG_2 => Ok(tag),
        INSTRUCTION_TAG_3 => Ok(tag),
        INSTRUCTION_TAG_4 => Ok(tag),
        INSTRUCTION_TAG_5 => Ok(tag),
        INSTRUCTION_TAG_6 => Ok(tag),
        INSTRUCTION_TAG_7 => Ok(tag),
        INSTRUCTION_TAG_8 => Ok(tag),
        INSTRUCTION_TAG_9 => Ok(tag),
        INSTRUCTION_TAG_10 => Ok(tag),
        INSTRUCTION_TAG_11 => Ok(tag),
        INSTRUCTION_TAG_12 => Ok(tag),
        INSTRUCTION_TAG_13 => Ok(tag),
        INSTRUCTION_TAG_14 => Ok(tag),
        INSTRUCTION_TAG_15 => Ok(tag),
        INSTRUCTION_TAG_16 => Ok(tag),
        INSTRUCTION_TAG_17 => Ok(tag),
        INSTRUCTION_TAG_18 => Ok(tag),
        INSTRUCTION_TAG_19 => Ok(tag),
        INSTRUCTION_TAG_20 => Ok(tag),
        INSTRUCTION_TAG_21 => Ok(tag),
        INSTRUCTION_TAG_22 => Ok(tag),
        INSTRUCTION_TAG_23 => Ok(tag),
        INSTRUCTION_TAG_24 => Ok(tag),
        INSTRUCTION_TAG_38 => Ok(tag),
        INSTRUCTION_TAG_45 => Ok(tag),
        INSTRUCTION_TAG_255 => Ok(tag),
        _ => Err(Error::InvalidDiscriminator),
    }
}
/// Strictly parses a canonical program invocation.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<TokenInstructionData, Error> {
    parse_instruction_with_program(PROGRAM, instruction, limits)
}
/// Strictly parses an explicit expected program identity.
pub fn parse_instruction_with_program(
    program: Address,
    instruction: &Instruction,
    limits: Limits,
) -> Result<TokenInstructionData, Error> {
    match identify_instruction(&instruction.data)? {
        INSTRUCTION_TAG_0 => Ok(TokenInstructionData::InitializeMint(
            InitializeMint::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_1 => Ok(TokenInstructionData::InitializeAccount(
            InitializeAccount::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_2 => Ok(TokenInstructionData::InitializeMultisig(
            InitializeMultisig::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_3 => Ok(TokenInstructionData::Transfer(
            Transfer::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_4 => Ok(TokenInstructionData::Approve(Approve::parse_with_program(
            program,
            instruction,
            limits,
        )?)),
        INSTRUCTION_TAG_5 => Ok(TokenInstructionData::Revoke(Revoke::parse_with_program(
            program,
            instruction,
            limits,
        )?)),
        INSTRUCTION_TAG_6 => Ok(TokenInstructionData::SetAuthority(
            SetAuthority::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_7 => Ok(TokenInstructionData::MintTo(MintTo::parse_with_program(
            program,
            instruction,
            limits,
        )?)),
        INSTRUCTION_TAG_8 => Ok(TokenInstructionData::Burn(Burn::parse_with_program(
            program,
            instruction,
            limits,
        )?)),
        INSTRUCTION_TAG_9 => Ok(TokenInstructionData::CloseAccount(
            CloseAccount::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_10 => Ok(TokenInstructionData::FreezeAccount(
            FreezeAccount::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_11 => Ok(TokenInstructionData::ThawAccount(
            ThawAccount::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_12 => Ok(TokenInstructionData::TransferChecked(
            TransferChecked::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_13 => Ok(TokenInstructionData::ApproveChecked(
            ApproveChecked::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_14 => Ok(TokenInstructionData::MintToChecked(
            MintToChecked::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_15 => Ok(TokenInstructionData::BurnChecked(
            BurnChecked::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_16 => Ok(TokenInstructionData::InitializeAccount2(
            InitializeAccount2::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_17 => Ok(TokenInstructionData::SyncNative(
            SyncNative::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_18 => Ok(TokenInstructionData::InitializeAccount3(
            InitializeAccount3::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_19 => Ok(TokenInstructionData::InitializeMultisig2(
            InitializeMultisig2::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_20 => Ok(TokenInstructionData::InitializeMint2(
            InitializeMint2::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_21 => Ok(TokenInstructionData::GetAccountDataSize(
            GetAccountDataSize::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_22 => Ok(TokenInstructionData::InitializeImmutableOwner(
            InitializeImmutableOwner::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_23 => Ok(TokenInstructionData::AmountToUiAmount(
            AmountToUiAmount::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_24 => Ok(TokenInstructionData::UiAmountToAmount(
            UiAmountToAmount::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_38 => Ok(TokenInstructionData::WithdrawExcessLamports(
            WithdrawExcessLamports::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_45 => Ok(TokenInstructionData::UnwrapLamports(
            UnwrapLamports::parse_with_program(program, instruction, limits)?,
        )),
        INSTRUCTION_TAG_255 => Ok(TokenInstructionData::Batch(Batch::parse_with_program(
            program,
            instruction,
            limits,
        )?)),
        _ => Err(Error::InvalidDiscriminator),
    }
}
use alloc::string::String;
use solana_kit_core::AccountMeta;
use solana_kit_core::AccountRole;
use solana_kit_core::codecs::Bool;
use solana_kit_core::codecs::Bytes;
use solana_kit_core::codecs::FixedBytes;
use solana_kit_core::codecs::Mapped;
use solana_kit_core::codecs::OptionCodec;
use solana_kit_core::codecs::Utf8;
use solana_kit_core::codecs::{ByteSizePrefix, FrameConsumption};
use solana_kit_core::codecs::{Count, OwnedSequence, Sequence};
