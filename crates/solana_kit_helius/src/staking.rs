//! Local Helius staking preparation reuses Kit program builders and caller-owned RPC/signing.
//!
//! Amounts are exact lamports. Applications own the new stake-account signer, entropy,
//! transport, deadlines and executor. Preparation does not submit or confirm a transaction.
//! State-dependent withdrawal evaluation remains under provider review.

mod accounts;
mod errors;
mod instructions;
mod transactions;

pub use accounts::{StakeAccountError, StakeAccountQuery, get_helius_stake_accounts};
pub use errors::{PreparationError, StakingBuildError};
pub use instructions::{
    HELIUS_VALIDATOR, build_stake_instructions, get_unstake_instruction, get_withdraw_instruction,
};
pub use transactions::{
    create_stake_transaction, create_unstake_transaction, create_withdraw_transaction,
    get_stake_instructions, serialize_transaction,
};

use solana_kit_client::methods::Commitment;
use solana_kit_core::{Address, Instruction, codecs::Limits, transaction::Transaction};

/// Caller-selected confirmation and per-instruction codec limits.
#[derive(Debug, Clone, Copy)]
pub struct PreparationOptions {
    /// Node default when absent, applied to rent and blockhash requests.
    pub commitment: Option<Commitment>,
    /// Limits for each instruction builder and its nested codecs.
    pub instruction_limits: Limits,
}

/// Exact stake funding and an explicit caller-owned new account identity.
#[derive(Debug, Clone, Copy)]
pub struct StakeRequest {
    /// Fee payer, stake authority, withdrawal authority and initial custodian.
    pub owner: Address,
    /// Address of a new account whose signature the application must supply.
    pub stake_account: Address,
    /// Delegated amount, excluding rent reserve; added with checked arithmetic.
    pub lamports: u64,
}

/// Explicit deactivation identities.
#[derive(Debug, Clone, Copy)]
pub struct UnstakeRequest {
    /// Fee payer and current stake authority.
    pub owner: Address,
    /// Stake account to deactivate.
    pub stake_account: Address,
}

/// Exact withdrawal instruction inputs; the program determines whether withdrawal is allowed.
#[derive(Debug, Clone, Copy)]
pub struct WithdrawalRequest {
    /// Fee payer and withdrawal authority.
    pub authority: Address,
    /// Stake account to withdraw from.
    pub stake_account: Address,
    /// Recipient account.
    pub destination: Address,
    /// Exact amount requested for withdrawal.
    pub lamports: u64,
}

/// Create, initialize and delegate instructions in their required order.
#[derive(Debug, Clone)]
pub struct StakeInstructions {
    /// New account identity supplied by the caller.
    pub stake_account: Address,
    /// Three source-defined instructions; combine them into one transaction.
    pub instructions: [Instruction; 3],
}

/// A prepared signed transaction and the caller's new stake-account identity.
#[derive(Debug, Clone)]
pub struct PreparedStakeTransaction {
    /// New account identity; no local entropy was generated for it.
    pub stake_account: Address,
    /// Immutable signed result; cryptography is checked when the crypto feature is enabled.
    pub transaction: Transaction,
}
