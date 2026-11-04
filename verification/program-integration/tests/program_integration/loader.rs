//! Upgradeable-loader buffer creation, write, decoded header and close.
//!
//! The separate loader-deploy journey exercises the pinned Noop ELF.

use crate::util;
use solana_kit_core::{Address, crypto::Keypair};
use solana_kit_loader::{
    BUFFER_HEADER_SIZE, UpgradeableState, decode_upgradeable_account,
    v3::{
        self, Close, CloseAccounts, InitializeBuffer, InitializeBufferAccounts, Write,
        WriteAccounts,
    },
};
use solana_kit_program_integration::{CODEC_LIMITS, Error, IntegrationEnv};
use solana_kit_system::{CreateAccount, CreateAccountAccounts};

const PAYLOAD: &[u8] = b"solana-kit loader buffer payload: no elf artifact is present";
const FEE_TOLERANCE: u64 = 5_000;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let (buffer, space, buffer_rent) = create_buffer(env).await?;
    write_payload(env, buffer.address(), space).await?;
    expect_foreign_authority_rejection(env, buffer.address()).await?;
    close_buffer(env, buffer.address(), buffer_rent, payer).await
}

async fn create_buffer(env: &mut IntegrationEnv) -> Result<(Keypair, u64, u64), Error> {
    let payer = env.payer_address();
    let buffer = env.new_key().await?;
    let space = (BUFFER_HEADER_SIZE + PAYLOAD.len()) as u64;
    let buffer_rent = env.rent(space).await?;

    let create = CreateAccount {
        lamports: buffer_rent,
        space,
        program_address: v3::PROGRAM,
    }
    .instruction(
        CreateAccountAccounts {
            payer,
            new_account: buffer.address(),
        },
        CODEC_LIMITS,
    )?;
    let initialize = InitializeBufferAccounts {
        source_account: buffer.address(),
        buffer_authority: payer,
    }
    .instruction(v3::PROGRAM, &InitializeBuffer {}, &[], CODEC_LIMITS)?;
    env.submit(&[create, initialize], &[env.payer(), &buffer])
        .await?;

    Ok((buffer, space, buffer_rent))
}

async fn write_payload(env: &IntegrationEnv, buffer: Address, space: u64) -> Result<(), Error> {
    let write = WriteAccounts {
        buffer_account: buffer,
        buffer_authority: env.payer_address(),
    }
    .instruction(
        v3::PROGRAM,
        &Write {
            offset: 0,
            bytes: PAYLOAD.to_vec(),
        },
        &[],
        CODEC_LIMITS,
    )?;
    let signature = env.submit(&[write], &[env.payer()]).await?;

    let account = env.account(buffer).await?.expect("buffer account exists");
    assert_eq!(account.data.get(BUFFER_HEADER_SIZE..), Some(PAYLOAD));
    let decoded = decode_upgradeable_account(account, CODEC_LIMITS)?;
    assert_eq!(
        decoded.data,
        UpgradeableState::Buffer {
            authority: Some(env.payer_address())
        }
    );
    assert_eq!(decoded.space, space);
    println!(
        "loader: {signature} wrote {} payload bytes into a {space}-byte Buffer header for {}",
        PAYLOAD.len(),
        env.payer_address()
    );

    Ok(())
}

async fn expect_foreign_authority_rejection(
    env: &mut IntegrationEnv,
    buffer: Address,
) -> Result<(), Error> {
    let intruder = env.new_key().await?;
    let write = WriteAccounts {
        buffer_account: buffer,
        buffer_authority: intruder.address(),
    }
    .instruction(
        v3::PROGRAM,
        &Write {
            offset: 0,
            bytes: PAYLOAD.to_vec(),
        },
        &[],
        CODEC_LIMITS,
    )?;
    let error = env
        .submit(&[write], &[env.payer(), &intruder])
        .await
        .expect_err("a foreign authority cannot write the buffer");

    assert!(
        matches!(
            error.instruction_failure(),
            Some((
                0,
                solana_kit_client::methods::submission::InstructionFailure::IncorrectAuthority
            ))
        ),
        "expected loader IncorrectAuthority at 0, got {error:?}"
    );

    Ok(())
}

async fn close_buffer(
    env: &IntegrationEnv,
    buffer: Address,
    buffer_rent: u64,
    destination: Address,
) -> Result<(), Error> {
    let before = env.balance(destination).await?;
    let close = CloseAccounts {
        buffer_or_program_data_account: buffer,
        destination_account: destination,
        authority: Some(destination),
        program_account: None,
    }
    .instruction(v3::PROGRAM, &Close {}, &[], CODEC_LIMITS)?;
    let signature = env.submit(&[close], &[env.payer()]).await?;

    assert!(env.account(buffer).await?.is_none());
    let after = env.balance(destination).await?;
    assert_eq!(after + FEE_TOLERANCE, before + buffer_rent);
    println!(
        "loader: {signature} closed the buffer and returned {buffer_rent} lamports rent to {destination}"
    );

    Ok(())
}

#[tokio::test]
async fn creates_writes_decodes_and_closes_a_loader_buffer() {
    let mut env = util::start().await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
