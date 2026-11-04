//! Address lookup table creation, extension, deactivation and decoded state.

use crate::util;
use solana_kit_address_lookup_table::{
    Funding, Operation, PROGRAM, decode_account, find_lookup_table,
};
use solana_kit_client::parsed::ParsedAccountData;
use solana_kit_core::Address;
use solana_kit_program_integration::{CODEC_LIMITS, Error, IntegrationEnv, PARSED_LIMITS};
use solana_kit_system::SYSTEM_PROGRAM;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let authority = env.payer_address();
    let (table, addresses) = create_and_extend(env, authority).await?;
    deactivate(env, table, authority, &addresses).await?;

    expect_foreign_authority_rejection(env, table, authority).await
}

async fn create_and_extend(
    env: &mut IntegrationEnv,
    authority: Address,
) -> Result<(Address, Vec<Address>), Error> {
    let recent_slot = env.slot().await?;
    let derived = find_lookup_table(authority, recent_slot, PROGRAM)?;
    let table = derived.address();
    let funding = Funding {
        payer: authority,
        system_program: SYSTEM_PROGRAM,
    };

    let create = Operation::Create {
        table,
        authority,
        funding,
        recent_slot,
        bump: derived.bump(),
    }
    .build(PROGRAM, CODEC_LIMITS)?;
    env.submit(&[create], &[env.payer()]).await?;

    let addresses = vec![
        env.new_key().await?.address(),
        env.new_key().await?.address(),
    ];
    let extend = Operation::Extend {
        table,
        authority,
        funding: Some(funding),
        addresses: addresses.clone(),
    }
    .build(PROGRAM, CODEC_LIMITS)?;
    let signature = env.submit(&[extend], &[env.payer()]).await?;

    let state = decode_table(env, table).await?;
    assert_eq!(state.authority, Some(authority));
    assert_eq!(state.addresses, addresses);
    assert_eq!(state.deactivation_slot, u64::MAX);
    assert_parsed_table(env, table, &state).await?;
    println!(
        "alt: create+extend {signature} decoded {} active addresses at slot {recent_slot}",
        state.addresses.len()
    );

    Ok((table, addresses))
}

async fn deactivate(
    env: &mut IntegrationEnv,
    table: Address,
    authority: Address,
    addresses: &[Address],
) -> Result<(), Error> {
    let deactivate = Operation::Deactivate { table, authority }.build(PROGRAM, CODEC_LIMITS)?;
    env.submit(&[deactivate], &[env.payer()]).await?;

    let state = decode_table(env, table).await?;
    assert_ne!(state.deactivation_slot, u64::MAX);
    assert_eq!(state.addresses, addresses);
    assert_parsed_table(env, table, &state).await?;
    println!(
        "alt: deactivate recorded slot {} and retained {} addresses",
        state.deactivation_slot,
        state.addresses.len()
    );

    Ok(())
}

async fn assert_parsed_table(
    env: &IntegrationEnv,
    table: Address,
    expected: &solana_kit_address_lookup_table::AddressLookupTable,
) -> Result<(), Error> {
    let parsed = env
        .parsed_account(table)
        .await?
        .expect("parsed lookup table exists");

    match parsed.decode_typed(PARSED_LIMITS)? {
        ParsedAccountData::AddressLookupTable(info) => {
            assert_eq!(info.addresses, expected.addresses);
            assert_eq!(info.authority, expected.authority);
            assert_eq!(info.deactivation_slot, expected.deactivation_slot);
            assert_eq!(info.last_extended_slot, expected.last_extended_slot);
            assert_eq!(
                info.last_extended_slot_start_index,
                expected.last_extended_slot_start_index
            );
        }
        other => panic!("expected a parsed address lookup table, got {other:?}"),
    }

    Ok(())
}

async fn decode_table(
    env: &IntegrationEnv,
    table: Address,
) -> Result<solana_kit_address_lookup_table::AddressLookupTable, Error> {
    let account = env.account(table).await?.expect("lookup table exists");
    let decoded = decode_account(account, CODEC_LIMITS)?;

    Ok(decoded.data)
}

async fn expect_foreign_authority_rejection(
    env: &mut IntegrationEnv,
    table: Address,
    authority: Address,
) -> Result<(), Error> {
    let intruder = env.new_key().await?;
    let extend = Operation::Extend {
        table,
        authority: intruder.address(),
        funding: Some(Funding {
            payer: authority,
            system_program: SYSTEM_PROGRAM,
        }),
        addresses: vec![env.new_key().await?.address()],
    }
    .build(PROGRAM, CODEC_LIMITS)?;
    let error = env
        .submit(&[extend], &[env.payer(), &intruder])
        .await
        .expect_err("a foreign signer cannot extend the table");

    assert!(
        matches!(
            error.instruction_failure(),
            Some((
                0,
                solana_kit_client::methods::submission::InstructionFailure::IncorrectAuthority
            ))
        ),
        "expected ALT IncorrectAuthority at 0, got {error:?}"
    );

    Ok(())
}

#[tokio::test]
async fn creates_extends_and_deactivates_a_lookup_table() {
    let mut env = util::start().await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
