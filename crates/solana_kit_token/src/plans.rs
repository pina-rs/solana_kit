//! Source-defined mint and associated-token-account instruction plans.
//!
//! These helpers mirror the pinned handwritten Dart modules that build a
//! create-mint pair, idempotent-ATA then `MintToChecked`, and
//! idempotent-ATA then `TransferChecked`. They only construct instructions:
//! no signer, blockhash, network request, rent lookup, executor or transaction
//! is selected. Callers remain responsible for the live rent-exempt amount and
//! for compiling the plan into a transaction.
use crate::{InitializeMint2, MintToChecked, PROGRAM as TOKEN_PROGRAM, TransferChecked};
use alloc::vec;
use core::fmt;
use solana_kit_associated_token_account::{
    AssociatedTokenInstruction, AssociatedTokenSeeds, CreateAccounts,
    PROGRAM as ASSOCIATED_TOKEN_PROGRAM,
};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{self, Limits},
    crypto::PdaError,
    plans::Plan,
};
use solana_kit_system::{CreateAccount, CreateAccountAccounts, SYSTEM_PROGRAM};

/// Canonical SPL Token mint allocation in bytes.
pub const MINT_ACCOUNT_SIZE: u64 = 82;
/// The pinned source's rent-exempt funding default for a mint account.
pub const DEFAULT_MINT_ACCOUNT_LAMPORTS: u64 = 1_461_600;

/// A typed construction failure that preserves codec, System and PDA causes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanError {
    /// A Token or associated-token-account codec rejected the caller budget.
    Codec(codecs::Error),
    /// The System builder rejected the caller budget.
    System(solana_kit_system::ClientError),
    /// Associated-address derivation found no off-curve bump.
    Pda(PdaError),
}

impl From<codecs::Error> for PlanError {
    fn from(error: codecs::Error) -> Self {
        Self::Codec(error)
    }
}

impl From<PdaError> for PlanError {
    fn from(error: PdaError) -> Self {
        Self::Pda(error)
    }
}

impl fmt::Display for PlanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Codec(error) => write!(formatter, "token plan codec error: {error}"),
            Self::System(error) => write!(formatter, "token plan system error: {error}"),
            Self::Pda(error) => write!(formatter, "token plan address error: {error}"),
        }
    }
}

impl core::error::Error for PlanError {}

/// Input for [`create_mint_plan`].
///
/// The fields and their precedence match the pinned source. Negative
/// `BigInt` amounts cannot be represented by `u64`; callers validate bounds
/// before constructing this input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreateMintInput {
    /// Funding account address, ordinarily a system account.
    pub payer: Address,
    /// New mint account address.
    pub new_mint: Address,
    /// Number of base-10 digits to the right of the decimal place.
    pub decimals: u8,
    /// Authority that can mint new tokens.
    pub mint_authority: Address,
    /// Optional authority that can freeze token accounts.
    pub freeze_authority: Option<Address>,
    /// Optional override for the lamports funding the mint account.
    ///
    /// Defaults to [`DEFAULT_MINT_ACCOUNT_LAMPORTS`]. This is the source's
    /// pinned rent-exempt value, not a live `getMinimumBalanceForRentExemption`
    /// lookup.
    pub mint_account_lamports: Option<u64>,
}

/// Program overrides for [`create_mint_plan`].
///
/// Absent fields select the canonical Token and System programs. A configured
/// token program is used both as the account owner in the System
/// `CreateAccount` payload and as the `InitializeMint2` program identity. A
/// configured System program becomes the `CreateAccount` program identity.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CreateMintConfig {
    /// Token program address. Defaults to the canonical SPL Token program.
    pub token_program: Option<Address>,
    /// System program address. Defaults to the canonical System program.
    pub system_program: Option<Address>,
}

/// Input for [`mint_to_ata_plan`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MintToAtaInput {
    /// Funding account for associated-account creation.
    pub payer: Address,
    /// Associated token account address to mint to.
    pub ata: Address,
    /// Wallet address that owns the associated account.
    pub owner: Address,
    /// Token mint.
    pub mint: Address,
    /// Mint authority required to sign the resulting transaction.
    pub mint_authority: Address,
    /// Amount of base units to mint.
    pub amount: u64,
    /// Expected number of decimals for the mint.
    pub decimals: u8,
}

/// Program overrides for mint-to-associated-account helpers.
///
/// Absent fields select the canonical Token, associated-token-account and
/// System programs. Configured values are applied to the instruction identity
/// and every account-metadata position that names that program.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MintToAtaConfig {
    /// Token program address. Defaults to the canonical SPL Token program.
    pub token_program: Option<Address>,
    /// Associated token program address. Defaults to the canonical ATA program.
    pub associated_token_program: Option<Address>,
    /// System program address. Defaults to the canonical System program.
    pub system_program: Option<Address>,
}

/// Input for [`mint_to_ata_derived_plan`].
///
/// This is the synchronous replacement for the pinned source's async
/// `getMintToAtaInstructionPlanAsync`: the associated account is derived from
/// `owner`, the selected token program and `mint`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MintToAtaDerivedInput {
    /// Funding account for associated-account creation.
    pub payer: Address,
    /// Wallet address that owns the derived associated account.
    pub owner: Address,
    /// Token mint.
    pub mint: Address,
    /// Mint authority required to sign the resulting transaction.
    pub mint_authority: Address,
    /// Amount of base units to mint.
    pub amount: u64,
    /// Expected number of decimals for the mint.
    pub decimals: u8,
}

/// Input for [`transfer_to_ata_plan`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransferToAtaInput {
    /// Funding account for associated-account creation.
    pub payer: Address,
    /// Token mint.
    pub mint: Address,
    /// Source token account.
    pub source: Address,
    /// Source account's owner or delegate.
    pub authority: Address,
    /// Destination associated token account address.
    pub destination: Address,
    /// Wallet address that owns the destination associated account.
    pub recipient: Address,
    /// Amount of base units to transfer.
    pub amount: u64,
    /// Expected number of decimals for the mint.
    pub decimals: u8,
}

/// Program overrides for transfer-to-associated-account helpers.
///
/// Absent fields select the canonical Token, associated-token-account and
/// System programs. Configured values are applied to the instruction identity
/// and every account-metadata position that names that program.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TransferToAtaConfig {
    /// Token program address. Defaults to the canonical SPL Token program.
    pub token_program: Option<Address>,
    /// Associated token program address. Defaults to the canonical ATA program.
    pub associated_token_program: Option<Address>,
    /// System program address. Defaults to the canonical System program.
    pub system_program: Option<Address>,
}

/// Input for [`transfer_to_ata_derived_plan`].
///
/// This is the synchronous replacement for the pinned source's async
/// `getTransferToAtaInstructionPlanAsync`: the destination associated account
/// derives from `recipient` and the source derives from `authority`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransferToAtaDerivedInput {
    /// Funding account for associated-account creation.
    pub payer: Address,
    /// Token mint.
    pub mint: Address,
    /// Source account's owner or delegate, and owner of the derived source.
    pub authority: Address,
    /// Wallet address that owns the derived destination associated account.
    pub recipient: Address,
    /// Amount of base units to transfer.
    pub amount: u64,
    /// Expected number of decimals for the mint.
    pub decimals: u8,
}

/// Creates an indivisible System `CreateAccount` then `InitializeMint2` plan.
///
/// The two instructions must stay in one transaction or bundle, so an
/// uninitialized funded mint is never exposed between transactions. The
/// allocation is exactly 82 bytes and the default funding is 1,461,600
/// lamports from the pinned source. The caller must supply the current
/// cluster's rent-exempt amount when it differs from that default.
///
/// # Errors
/// Returns [`PlanError::System`] or [`PlanError::Codec`] when the caller's
/// [`Limits`] cannot contain either instruction.
pub fn create_mint_plan(
    input: CreateMintInput,
    config: CreateMintConfig,
    limits: Limits,
) -> Result<Plan<Instruction>, PlanError> {
    let token_program = config.token_program.unwrap_or(TOKEN_PROGRAM);
    let system_program = config.system_program.unwrap_or(SYSTEM_PROGRAM);
    let lamports = input
        .mint_account_lamports
        .unwrap_or(DEFAULT_MINT_ACCOUNT_LAMPORTS);

    let mut create_account = CreateAccount {
        lamports,
        space: MINT_ACCOUNT_SIZE,
        program_address: token_program,
    }
    .instruction(
        CreateAccountAccounts {
            payer: input.payer,
            new_account: input.new_mint,
        },
        limits,
    )
    .map_err(PlanError::System)?;
    create_account.program_address = system_program;

    let initialize_mint = InitializeMint2 {
        decimals: input.decimals,
        mint_authority: input.mint_authority,
        freeze_authority: input.freeze_authority,
    }
    .instruction_with_program(token_program, &[writable(input.new_mint)], limits)?;

    Ok(Plan::indivisible(vec![
        Plan::Single(create_account),
        Plan::Single(initialize_mint),
    ]))
}

/// Creates a divisible ATA `CreateIdempotent` then `MintToChecked` plan.
///
/// The associated account may already exist; the idempotent constructor then
/// succeeds without changing it. Amounts are raw `u64` base units and may be
/// zero. No balance, owner or rent check is performed here.
///
/// # Errors
/// Returns [`PlanError::Codec`] when the caller's [`Limits`] cannot contain
/// either instruction.
pub fn mint_to_ata_plan(
    input: MintToAtaInput,
    config: MintToAtaConfig,
    limits: Limits,
) -> Result<Plan<Instruction>, PlanError> {
    let token_program = config.token_program.unwrap_or(TOKEN_PROGRAM);
    let ata_program = config
        .associated_token_program
        .unwrap_or(ASSOCIATED_TOKEN_PROGRAM);
    let system_program = config.system_program.unwrap_or(SYSTEM_PROGRAM);

    let create_ata = create_idempotent_ata(
        ata_program,
        CreateAccounts {
            payer: input.payer,
            associated_token: input.ata,
            owner: input.owner,
            mint: input.mint,
            system_program,
            token_program,
        },
        limits,
    )?;
    let mint_to = MintToChecked {
        amount: input.amount,
        decimals: input.decimals,
    }
    .instruction_with_program(
        token_program,
        &[
            writable(input.mint),
            writable(input.ata),
            signer(input.mint_authority),
        ],
        limits,
    )?;

    Ok(Plan::sequential(vec![
        Plan::Single(create_ata),
        Plan::Single(mint_to),
    ]))
}

/// Derives the owner's associated account, then delegates to [`mint_to_ata_plan`].
///
/// The derivation uses `owner`, the selected token program and `mint` as
/// ordered seeds under the selected associated-token-account program.
///
/// # Errors
/// Returns [`PlanError::Pda`] when no off-curve bump exists, or the same
/// failures as [`mint_to_ata_plan`].
pub fn mint_to_ata_derived_plan(
    input: MintToAtaDerivedInput,
    config: MintToAtaConfig,
    limits: Limits,
) -> Result<Plan<Instruction>, PlanError> {
    let token_program = config.token_program.unwrap_or(TOKEN_PROGRAM);
    let ata_program = config
        .associated_token_program
        .unwrap_or(ASSOCIATED_TOKEN_PROGRAM);
    let ata = associated_token_address(input.owner, token_program, input.mint, ata_program)?;

    mint_to_ata_plan(
        MintToAtaInput {
            payer: input.payer,
            ata,
            owner: input.owner,
            mint: input.mint,
            mint_authority: input.mint_authority,
            amount: input.amount,
            decimals: input.decimals,
        },
        config,
        limits,
    )
}

/// Creates a divisible ATA `CreateIdempotent` then `TransferChecked` plan.
///
/// The destination associated account may already exist; the idempotent
/// constructor then succeeds without changing it. Amounts are raw `u64` base
/// units and may be zero. No balance, delegate or rent check is performed
/// here.
///
/// # Errors
/// Returns [`PlanError::Codec`] when the caller's [`Limits`] cannot contain
/// either instruction.
pub fn transfer_to_ata_plan(
    input: TransferToAtaInput,
    config: TransferToAtaConfig,
    limits: Limits,
) -> Result<Plan<Instruction>, PlanError> {
    let token_program = config.token_program.unwrap_or(TOKEN_PROGRAM);
    let ata_program = config
        .associated_token_program
        .unwrap_or(ASSOCIATED_TOKEN_PROGRAM);
    let system_program = config.system_program.unwrap_or(SYSTEM_PROGRAM);

    let create_ata = create_idempotent_ata(
        ata_program,
        CreateAccounts {
            payer: input.payer,
            associated_token: input.destination,
            owner: input.recipient,
            mint: input.mint,
            system_program,
            token_program,
        },
        limits,
    )?;
    let transfer = TransferChecked {
        amount: input.amount,
        decimals: input.decimals,
    }
    .instruction_with_program(
        token_program,
        &[
            writable(input.source),
            readonly(input.mint),
            writable(input.destination),
            signer(input.authority),
        ],
        limits,
    )?;

    Ok(Plan::sequential(vec![
        Plan::Single(create_ata),
        Plan::Single(transfer),
    ]))
}

/// Derives both associated accounts, then delegates to [`transfer_to_ata_plan`].
///
/// The destination derives from `recipient`; the source derives from
/// `authority`. Both use the selected token program and `mint` under the
/// selected associated-token-account program.
///
/// # Errors
/// Returns [`PlanError::Pda`] when no off-curve bump exists, or the same
/// failures as [`transfer_to_ata_plan`].
pub fn transfer_to_ata_derived_plan(
    input: TransferToAtaDerivedInput,
    config: TransferToAtaConfig,
    limits: Limits,
) -> Result<Plan<Instruction>, PlanError> {
    let token_program = config.token_program.unwrap_or(TOKEN_PROGRAM);
    let ata_program = config
        .associated_token_program
        .unwrap_or(ASSOCIATED_TOKEN_PROGRAM);
    let destination =
        associated_token_address(input.recipient, token_program, input.mint, ata_program)?;
    let source = associated_token_address(input.authority, token_program, input.mint, ata_program)?;

    transfer_to_ata_plan(
        TransferToAtaInput {
            payer: input.payer,
            mint: input.mint,
            source,
            authority: input.authority,
            destination,
            recipient: input.recipient,
            amount: input.amount,
            decimals: input.decimals,
        },
        config,
        limits,
    )
}

fn writable(address: Address) -> AccountMeta {
    AccountMeta {
        address,
        role: AccountRole::Writable,
    }
}

fn readonly(address: Address) -> AccountMeta {
    AccountMeta {
        address,
        role: AccountRole::Readonly,
    }
}

fn signer(address: Address) -> AccountMeta {
    AccountMeta {
        address,
        role: AccountRole::ReadonlySigner,
    }
}

fn associated_token_address(
    owner: Address,
    token_program: Address,
    mint: Address,
    program: Address,
) -> Result<Address, PlanError> {
    Ok(AssociatedTokenSeeds {
        owner,
        token_program,
        mint,
    }
    .find_with_program(program)?
    .address())
}

fn create_idempotent_ata(
    program_address: Address,
    accounts: CreateAccounts,
    limits: Limits,
) -> Result<Instruction, PlanError> {
    let mut instruction =
        AssociatedTokenInstruction::CreateIdempotent(accounts).instruction(limits)?;
    instruction.program_address = program_address;
    Ok(instruction)
}
