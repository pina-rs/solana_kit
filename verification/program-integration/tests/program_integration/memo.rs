//! Memo v3 execution plus its program-level signer requirement.

use crate::util;
use solana_kit_core::{AccountMeta, AccountRole, Instruction};
use solana_kit_program_integration::{CODEC_LIMITS, Error, IntegrationEnv};

const MEMO_TEXT: &str = "solana-kit memo integration";

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let instruction = solana_kit_memo::add_memo_with_program(
        solana_kit_memo::LEGACY_PROGRAM_V3,
        MEMO_TEXT,
        &[env.payer_address()],
        CODEC_LIMITS,
    )?;
    let signature = env.submit(&[instruction], &[env.payer()]).await?;
    let logs = env.logs(signature).await?;

    assert!(
        logs.iter()
            .any(|line| line.contains("Program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"))
    );
    assert!(logs.iter().any(|line| line.contains(MEMO_TEXT)));
    println!("memo: v3 executed {signature}; logs contain the exact memo text");

    expect_non_signer_rejection(env).await
}

async fn expect_non_signer_rejection(env: &mut IntegrationEnv) -> Result<(), Error> {
    let intruder = env.new_key().await?;
    let payload = solana_kit_memo::add_memo_with_program(
        solana_kit_memo::LEGACY_PROGRAM_V3,
        MEMO_TEXT,
        &[],
        CODEC_LIMITS,
    )?
    .data;
    // Memo rejects any account that is not a signer of the transaction, so a
    // fresh address (never a tx signer) isolates that program check.
    let instruction = Instruction {
        program_address: solana_kit_memo::LEGACY_PROGRAM_V3,
        accounts: vec![AccountMeta {
            address: intruder.address(),
            role: AccountRole::Readonly,
        }],
        data: payload,
    };
    let error = env
        .submit(&[instruction], &[env.payer()])
        .await
        .expect_err("memo rejects a non-signer account");

    match error.instruction_failure() {
        Some((
            0,
            solana_kit_client::methods::submission::InstructionFailure::MissingRequiredSignature,
        )) => {}
        other => panic!(
            "expected memo MissingRequiredSignature at instruction 0, got {other:?} ({error:?})"
        ),
    }

    println!("memo: non-signer account rejected with MissingRequiredSignature");

    Ok(())
}

#[tokio::test]
async fn executes_memo_text_and_rejects_a_non_signer() {
    let mut env = util::start().await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
