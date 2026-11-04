//! Native Config program store and decoded account state.

use crate::util;
use solana_kit_config::{ConfigData, ConfigKey, PROGRAM, decode_account, store_config};
use solana_kit_program_integration::{CODEC_LIMITS, Error, IntegrationEnv};
use solana_kit_system::{CreateAccount, CreateAccountAccounts};

const CONFIG_SPACE: u64 = 1_000;
const STORED: &[u8] = b"hello config";

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let config = env.new_key().await?;
    let create = CreateAccount {
        lamports: env.rent(CONFIG_SPACE).await?,
        space: CONFIG_SPACE,
        program_address: PROGRAM,
    }
    .instruction(
        CreateAccountAccounts {
            payer,
            new_account: config.address(),
        },
        CODEC_LIMITS,
    )?;
    env.submit(&[create], &[env.payer(), &config]).await?;

    let value = stored_value();
    let store = store_config(PROGRAM, config.address(), true, &value, CODEC_LIMITS)?;
    let signature = env.submit(&[store], &[env.payer(), &config]).await?;

    let account = env
        .account(config.address())
        .await?
        .expect("config account exists");
    let decoded = decode_account(account, CODEC_LIMITS)?;
    assert_eq!(decoded.data.keys, value.keys);
    // The Config program stores the tail in the full allocation and zero-pads
    // the unused remainder, so the typed tail is prefix + reserved zeros.
    assert!(decoded.data.data.starts_with(STORED));
    assert!(
        decoded.data.data[STORED.len()..]
            .iter()
            .all(|byte| *byte == 0)
    );
    println!(
        "config: {signature} stored {} key(s) and {} byte(s); {} bytes of reserved zero padding decoded",
        decoded.data.keys.len(),
        STORED.len(),
        decoded.data.data.len() - STORED.len()
    );

    expect_unsigned_store_rejection(env, config.address()).await
}

fn stored_value() -> ConfigData {
    ConfigData {
        keys: vec![ConfigKey {
            address: PROGRAM,
            is_signer: false,
        }],
        data: STORED.to_vec(),
    }
}

async fn expect_unsigned_store_rejection(
    env: &IntegrationEnv,
    config: solana_kit_core::Address,
) -> Result<(), Error> {
    let store = store_config(PROGRAM, config, false, &stored_value(), CODEC_LIMITS)?;
    let error = env
        .submit(&[store], &[env.payer()])
        .await
        .expect_err("an unsigned store of a signer-free config must fail");

    match error.instruction_failure() {
        Some((
            0,
            solana_kit_client::methods::submission::InstructionFailure::MissingRequiredSignature,
        )) => {}
        other => panic!("expected config MissingRequiredSignature at 0, got {other:?} ({error:?})"),
    }

    println!("config: unsigned store rejected with MissingRequiredSignature");

    Ok(())
}

#[tokio::test]
async fn stores_and_decodes_keyed_config_data() {
    let mut env = util::start().await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
