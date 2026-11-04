//! Compute-budget limits and prices measured through confirmed transaction metadata.

use crate::util;
use solana_kit_compute_budget::{SetComputeUnitLimit, SetComputeUnitPrice};
use solana_kit_core::Signature;
use solana_kit_program_integration::{CODEC_LIMITS, Error, IntegrationEnv};

const MEMO_TEXT: &str = "compute-budget integration";
const UNIT_LIMIT: u32 = 100_000;
const MICRO_LAMPORTS: u64 = 1_000;
const BASE_FEE: u64 = 5_000;

pub async fn run(env: &IntegrationEnv) -> Result<(), Error> {
    let signature = execute_memo_with_budget(env).await?;
    assert_confirmed_budget(env, signature).await?;
    expect_low_limit_rejection(env).await
}

async fn execute_memo_with_budget(env: &IntegrationEnv) -> Result<Signature, Error> {
    let limit = SetComputeUnitLimit { units: UNIT_LIMIT }.instruction(CODEC_LIMITS)?;
    let price = SetComputeUnitPrice {
        micro_lamports: MICRO_LAMPORTS,
    }
    .instruction(CODEC_LIMITS)?;
    let memo = memo(env)?;

    Ok(env.submit(&[limit, price, memo], &[env.payer()]).await?)
}

async fn assert_confirmed_budget(env: &IntegrationEnv, signature: Signature) -> Result<(), Error> {
    let logs = env.logs(signature).await?;
    let units = env.compute_units(signature).await?;
    let fee = env
        .transaction(signature)
        .await?
        .and_then(|transaction| transaction.details.meta)
        .map(|meta| meta.fee)
        .ok_or(Error::Client("confirmed transaction metadata is absent"))?;

    assert!(logs.iter().any(|line| line.contains(MEMO_TEXT)));
    assert!(
        logs.iter()
            .any(|line| line.contains("Program ComputeBudget111111111111111111111111111111"))
    );
    assert!((1..=u64::from(UNIT_LIMIT)).contains(&units));
    assert!(fee > BASE_FEE, "priority fee must exceed the base fee");
    println!(
        "compute_budget: memo executed within {units} units (limit {UNIT_LIMIT}); fee {fee} lamports; logs contain the memo"
    );

    Ok(())
}

async fn expect_low_limit_rejection(env: &IntegrationEnv) -> Result<(), Error> {
    let low_limit = SetComputeUnitLimit { units: 1 }.instruction(CODEC_LIMITS)?;
    let memo = memo(env)?;
    let error = env
        .submit(&[low_limit, memo], &[env.payer()])
        .await
        .expect_err("one compute unit cannot execute the memo program");

    assert!(matches!(
        error.instruction_failure(),
        Some((0, solana_kit_client::methods::submission::InstructionFailure::ComputationalBudgetExceeded))
    ), "expected compute exhaustion at instruction 0, got {error:?}");

    println!(
        "compute_budget: one-unit limit rejected the memo with {:?}",
        error
    );

    Ok(())
}

fn memo(env: &IntegrationEnv) -> Result<solana_kit_core::Instruction, Error> {
    Ok(solana_kit_memo::add_memo_with_program(
        solana_kit_memo::LEGACY_PROGRAM_V3,
        MEMO_TEXT,
        &[env.payer_address()],
        CODEC_LIMITS,
    )?)
}

#[tokio::test]
async fn enforces_a_real_compute_limit_and_price() {
    let env = util::start().await;
    let result = run(&env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
