//! System account creation, SOL transfer and the durable-nonce lifecycle.

use crate::util;
use solana_kit_client::parsed::{ParsedAccountData, SysvarAccount};
use solana_kit_core::crypto::Keypair;
use solana_kit_core::sysvars::{CLOCK_ADDRESS, RENT_ADDRESS};
use solana_kit_core::{
    Address,
    transaction::{DurableNonce, Lifetime},
};
use solana_kit_program_integration::{
    CODEC_LIMITS, Error, IntegrationEnv, PARSED_LIMITS, expect_custom_error,
};
use solana_kit_system::{
    AdvanceNonceAccount, AdvanceNonceAccountAccounts, CreateAccount, CreateAccountAccounts,
    InitializeNonceAccount, InitializeNonceAccountAccounts, NONCE_SIZE, SYSTEM_PROGRAM,
    TransferSol, TransferSolAccounts, WithdrawNonceAccount, WithdrawNonceAccountAccounts,
    decode_nonce,
};

const TRANSFER: u64 = 1_000_000_000;
const NONCE_WITHDRAWAL: u64 = 1_000_000_000;
const ACCOUNT_SPACE: u64 = 64;
const NONCE_FUNDING: u64 = 5_000_000_000;
const ZERO_BYTE_RENT: u64 = 890_880;
const LAMPORTS_PER_BYTE: u64 = 6_960;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    transfer(env).await?;
    exercise_account_creation(env).await?;
    exercise_nonce(env).await?;
    exercise_sysvars(env).await?;
    expect_transfer_rejection(env).await
}

async fn transfer(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let recipient = env.new_key().await?;
    let before = env.balance(payer).await?;
    let transfer = TransferSol { amount: TRANSFER }.instruction(
        TransferSolAccounts {
            source: payer,
            destination: recipient.address(),
        },
        CODEC_LIMITS,
    )?;
    let signature = env.submit(&[transfer], &[env.payer()]).await?;

    assert_eq!(env.balance(recipient.address()).await?, TRANSFER);
    assert!(env.balance(payer).await? < before);
    println!(
        "system: transfer {TRANSFER} lamports confirmed as {signature}; recipient balance is exact"
    );

    Ok(())
}

async fn exercise_account_creation(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let account = env.new_key().await?;
    let rent = env.rent(ACCOUNT_SPACE).await?;
    let create = CreateAccount {
        lamports: rent,
        space: ACCOUNT_SPACE,
        program_address: SYSTEM_PROGRAM,
    }
    .instruction(
        CreateAccountAccounts {
            payer,
            new_account: account.address(),
        },
        CODEC_LIMITS,
    )?;
    env.submit(&[create], &[env.payer(), &account]).await?;

    let snapshot = env
        .account(account.address())
        .await?
        .expect("created account exists");
    assert_eq!(snapshot.owner, SYSTEM_PROGRAM);
    assert_eq!(snapshot.space, ACCOUNT_SPACE);
    assert_eq!(snapshot.lamports, rent);
    println!(
        "system: createAccount owner={} space={} lamports={rent}",
        snapshot.owner, snapshot.space
    );

    Ok(())
}

async fn exercise_nonce(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let (nonce_account, rent) = create_nonce(env, payer).await?;
    let initial = initialized_nonce(env, nonce_account.address()).await?;
    assert_parsed_nonce(env, nonce_account.address(), &initial).await?;
    advance_nonce(env, &nonce_account, payer, initial).await?;
    withdraw_nonce(env, nonce_account.address(), payer, rent).await
}

async fn create_nonce(env: &mut IntegrationEnv, payer: Address) -> Result<(Keypair, u64), Error> {
    let nonce_account = env.new_key().await?;
    let rent = env.rent(NONCE_SIZE as u64).await?;
    let create = CreateAccount {
        lamports: rent + NONCE_FUNDING,
        space: NONCE_SIZE as u64,
        program_address: SYSTEM_PROGRAM,
    }
    .instruction(
        CreateAccountAccounts {
            payer,
            new_account: nonce_account.address(),
        },
        CODEC_LIMITS,
    )?;
    let initialize = InitializeNonceAccount {
        nonce_authority: payer,
    }
    .instruction(
        InitializeNonceAccountAccounts {
            nonce_account: nonce_account.address(),
        },
        CODEC_LIMITS,
    )?;
    env.submit(&[create, initialize], &[env.payer(), &nonce_account])
        .await?;

    Ok((nonce_account, rent))
}

async fn assert_parsed_nonce(
    env: &IntegrationEnv,
    address: Address,
    initial: &solana_kit_system::NonceData,
) -> Result<(), Error> {
    let parsed = env
        .parsed_account(address)
        .await?
        .expect("parsed nonce exists");

    match parsed.decode_typed(PARSED_LIMITS)? {
        ParsedAccountData::Nonce(info) => {
            assert_eq!(info.authority, initial.authority);
            assert_eq!(info.blockhash, initial.blockhash);
            assert_eq!(
                info.fee_calculator.lamports_per_signature,
                initial.lamports_per_signature
            );
        }
        other => panic!("expected a parsed nonce account, got {other:?}"),
    }

    Ok(())
}

async fn advance_nonce(
    env: &IntegrationEnv,
    nonce_account: &Keypair,
    payer: Address,
    initial: solana_kit_system::NonceData,
) -> Result<(), Error> {
    let token = initial.blockhash;
    let durable = DurableNonce::new(token, nonce_account.address(), payer);
    let advance = AdvanceNonceAccount {}.instruction(
        AdvanceNonceAccountAccounts {
            nonce_account: nonce_account.address(),
            nonce_authority: payer,
        },
        CODEC_LIMITS,
    )?;
    let signature = env
        .submit_with_lifetime(Lifetime::DurableNonce(durable), &[advance], &[env.payer()])
        .await?;
    let advanced = initialized_nonce(env, nonce_account.address())
        .await?
        .blockhash;

    assert_ne!(advanced, token);
    println!("system: nonce advance {signature} replaced token {token} with {advanced}");

    Ok(())
}

async fn withdraw_nonce(
    env: &mut IntegrationEnv,
    nonce_account: Address,
    payer: Address,
    rent: u64,
) -> Result<(), Error> {
    let recipient = env.new_key().await?;
    let withdraw = WithdrawNonceAccount {
        withdraw_amount: NONCE_WITHDRAWAL,
    }
    .instruction(
        WithdrawNonceAccountAccounts {
            nonce_account,
            recipient_account: recipient.address(),
            nonce_authority: payer,
        },
        CODEC_LIMITS,
    )?;
    env.submit(&[withdraw], &[env.payer()]).await?;

    assert_eq!(env.balance(recipient.address()).await?, NONCE_WITHDRAWAL);
    let remaining = env
        .account(nonce_account)
        .await?
        .expect("nonce account remains");
    assert!(remaining.lamports >= rent);
    println!(
        "system: withdrawNonceAccount returned {NONCE_WITHDRAWAL} lamports and kept {rent} rent-exempt"
    );

    Ok(())
}

async fn initialized_nonce(
    env: &IntegrationEnv,
    address: Address,
) -> Result<solana_kit_system::NonceData, Error> {
    let snapshot = env.account(address).await?.expect("nonce account exists");
    let nonce = decode_nonce(snapshot, CODEC_LIMITS)?;
    let data = *nonce
        .data
        .durable_data()
        .expect("nonce is initialized with a current durable token");

    assert_eq!(data.authority, env.payer_address());

    Ok(data)
}

async fn exercise_sysvars(env: &IntegrationEnv) -> Result<(), Error> {
    assert_clock(env).await?;
    assert_rent(env).await
}

async fn assert_clock(env: &IntegrationEnv) -> Result<(), Error> {
    let clock = env
        .parsed_account(CLOCK_ADDRESS)
        .await?
        .expect("clock sysvar exists");

    match clock.decode_typed(PARSED_LIMITS)? {
        ParsedAccountData::Sysvar(SysvarAccount::Clock(info)) => {
            let current = env.slot().await?;
            assert!(info.slot > 0 && info.slot <= current);
            assert!(info.unix_timestamp > 0);
            assert_eq!(info.epoch, 0);
            println!(
                "system: parsed Clock slot={} epoch={} unix_timestamp={} (node slot {current})",
                info.slot, info.epoch, info.unix_timestamp
            );
        }
        other => panic!("expected a parsed Clock sysvar, got {other:?}"),
    }

    Ok(())
}

async fn assert_rent(env: &IntegrationEnv) -> Result<(), Error> {
    let rent = env
        .parsed_account(RENT_ADDRESS)
        .await?
        .expect("rent sysvar exists");

    match rent.decode_typed(PARSED_LIMITS)? {
        ParsedAccountData::Sysvar(SysvarAccount::Rent(info)) => {
            println!(
                "system: parsed Rent per_byte_year={:?} per_byte={:?} threshold={:?} burn={:?}",
                info.lamports_per_byte_year,
                info.lamports_per_byte,
                info.exemption_threshold,
                info.burn_percent
            );
            assert_eq!(info.lamports_per_byte, Some(LAMPORTS_PER_BYTE));
            // Agave 4.1+ reports only the per-byte rate; the legacy threshold
            // and burn fields are explicitly absent, not invented as zero.
            assert_eq!(info.lamports_per_byte_year, None);
            assert_eq!(info.exemption_threshold, None);
            assert_eq!(info.burn_percent, None);
            let minimum = env.rent(0).await?;
            assert_eq!(minimum, ZERO_BYTE_RENT);
        }
        other => panic!("expected a parsed Rent sysvar, got {other:?}"),
    }

    Ok(())
}

async fn expect_transfer_rejection(env: &mut IntegrationEnv) -> Result<(), Error> {
    let recipient = env.new_key().await?;
    let transfer = TransferSol { amount: u64::MAX }.instruction(
        TransferSolAccounts {
            source: env.payer_address(),
            destination: recipient.address(),
        },
        CODEC_LIMITS,
    )?;
    let error = env
        .submit(&[transfer], &[env.payer()])
        .await
        .expect_err("u64::MAX lamports cannot be transferred");

    expect_custom_error(&error, 0, 1);
    println!("system: oversized transfer rejected by System custom error 1");

    Ok(())
}

#[tokio::test]
async fn creates_accounts_transfers_and_advances_a_durable_nonce() {
    let mut env = util::start().await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
