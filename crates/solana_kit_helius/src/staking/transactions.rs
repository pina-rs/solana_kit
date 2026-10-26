//! RPC preparation borrows caller-owned signing capabilities without starting a runtime.

use super::{
    PreparationError, PreparationOptions, PreparedStakeTransaction, StakeInstructions,
    StakeRequest, StakingBuildError, UnstakeRequest, WithdrawalRequest, build_stake_instructions,
    get_unstake_instruction, get_withdraw_instruction,
};
use core::convert::Infallible;
use solana_kit_client::{
    Client, Transport,
    methods::{
        Commitment, LatestBlockhash, ReadConfig,
        accounts::{CommitmentConfig, MinimumBalanceForRentExemption},
    },
    plans::Signer,
};
use solana_kit_core::{
    Address, Instruction,
    codecs::{self, Limits},
    transaction::{self, Lifetime, Message, Transaction, Version},
};

async fn minimum_rent<T: Transport>(
    client: &Client<T>,
    commitment: Option<Commitment>,
) -> Result<u64, solana_kit_client::Error> {
    client
        .call(MinimumBalanceForRentExemption(
            solana_kit_stake::STAKE_STATE_ACCOUNT_SIZE,
            commitment.map(|commitment| CommitmentConfig {
                commitment: Some(commitment),
            }),
        ))
        .await
}

async fn recent_lifetime<T: Transport>(
    client: &Client<T>,
    commitment: Option<Commitment>,
) -> Result<Lifetime, solana_kit_client::Error> {
    let blockhash = client
        .call(LatestBlockhash(commitment.map(|commitment| ReadConfig {
            commitment: Some(commitment),
            min_context_slot: None,
        })))
        .await?
        .value;

    Ok(Lifetime::Blockhash {
        blockhash: blockhash.blockhash,
        last_valid_block_height: blockhash.last_valid_block_height,
    })
}

fn message(
    payer: Address,
    lifetime: Lifetime,
    instructions: &[Instruction],
) -> Result<Message, StakingBuildError> {
    Ok(Message::compile(
        Version::V0,
        payer,
        lifetime,
        instructions,
        &[],
    )?)
}

async fn finish<S: Signer>(
    signer: &S,
    message: Message,
) -> Result<Transaction, PreparationError<S::Error>> {
    let signed = signer
        .sign(message.clone())
        .await
        .map_err(PreparationError::Signer)?;

    if signed.message() != &message {
        return Err(StakingBuildError::Transaction(transaction::Error::ChangedMessage).into());
    }

    if signed.signatures().iter().any(Option::is_none) {
        return Err(StakingBuildError::Transaction(transaction::Error::MissingSignature).into());
    }

    #[cfg(feature = "crypto")]
    signed.verify().map_err(StakingBuildError::from)?;

    Ok(signed)
}

/// Fetches rent once and builds the three staking instructions for a caller-supplied new account.
///
/// The caller owns that account's key and entropy. The returned instructions do not
/// imply a valid account balance, current minimum delegation or successful execution.
pub async fn get_stake_instructions<T: Transport>(
    client: &Client<T>,
    request: StakeRequest,
    options: PreparationOptions,
) -> Result<StakeInstructions, PreparationError<Infallible>> {
    let rent = minimum_rent(client, options.commitment).await?;
    Ok(build_stake_instructions(
        request,
        rent,
        options.instruction_limits,
    )?)
}

/// Prepares and signs one v0 transaction without submitting it.
///
/// The caller's aggregate signer must provide the owner and new stake-account
/// signatures. Changed messages and missing signatures reject; crypto additionally
/// authenticates every signature. Dropping the future drops its current exchange or signer future.
pub async fn create_stake_transaction<T: Transport, S: Signer>(
    client: &Client<T>,
    signer: &S,
    request: StakeRequest,
    options: PreparationOptions,
) -> Result<PreparedStakeTransaction, PreparationError<S::Error>> {
    let rent = minimum_rent(client, options.commitment).await?;
    let instructions = build_stake_instructions(request, rent, options.instruction_limits)?;
    let lifetime = recent_lifetime(client, options.commitment).await?;
    let transaction = finish(
        signer,
        message(request.owner, lifetime, &instructions.instructions)?,
    )
    .await?;

    Ok(PreparedStakeTransaction {
        stake_account: request.stake_account,
        transaction,
    })
}

/// Prepares and signs one deactivation transaction; it does not wait for cooldown or submit.
pub async fn create_unstake_transaction<T: Transport, S: Signer>(
    client: &Client<T>,
    signer: &S,
    request: UnstakeRequest,
    options: PreparationOptions,
) -> Result<Transaction, PreparationError<S::Error>> {
    let instruction = get_unstake_instruction(request, options.instruction_limits)?;
    let lifetime = recent_lifetime(client, options.commitment).await?;
    finish(signer, message(request.owner, lifetime, &[instruction])?).await
}

/// Prepares and signs the requested withdrawal without assuming that the chain will allow it.
pub async fn create_withdraw_transaction<T: Transport, S: Signer>(
    client: &Client<T>,
    signer: &S,
    request: WithdrawalRequest,
    options: PreparationOptions,
) -> Result<Transaction, PreparationError<S::Error>> {
    let instruction = get_withdraw_instruction(request, options.instruction_limits)?;
    let lifetime = recent_lifetime(client, options.commitment).await?;
    finish(
        signer,
        message(request.authority, lifetime, &[instruction])?,
    )
    .await
}

/// Produces canonical Base64 after requiring every signature slot; crypto verifies signatures too.
pub fn serialize_transaction(
    transaction: &Transaction,
    limits: Limits,
) -> Result<String, StakingBuildError> {
    #[cfg(feature = "crypto")]
    transaction.verify()?;

    let wire = transaction.encode(true)?;
    let size = base64::encoded_len(wire.len(), true).ok_or(codecs::Error::Overflow)?;

    if wire.len() > limits.max_bytes || size > limits.max_items {
        return Err(codecs::Error::LimitExceeded.into());
    }

    use base64::Engine;
    Ok(base64::engine::general_purpose::STANDARD.encode(wire))
}
