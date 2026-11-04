//! Real Squads V4: program config, multisig creation and member expansion.

use crate::util;
use solana_kit_core::codecs::{Decoder, Encoder};
use solana_kit_core::{AccountMeta, AccountRole, Address};
use solana_kit_program_integration::{
    CODEC_LIMITS, Error, IntegrationEnv, expect_custom_error, fixtures::SQUADS_MULTISIG,
};
use solana_kit_squads::{
    Member, Multisig, MultisigAddMember, MultisigCodec, MultisigCreateV2, Permissions,
    ProgramConfig, ProgramConfigCodec, SquadsMultisigError, decode_program_config, find_multisig,
    find_program_config,
};
use solana_kit_system::SYSTEM_PROGRAM;

const ALL_PERMISSIONS: u8 = 7;
const TREASURY: Address = Address::from_bytes([0x44; 32]);
const FOREIGN_AUTHORITY_ERROR: u32 = 6_004;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let program_config = find_program_config()?.address();
    assert_program_config(env, program_config, payer).await?;

    let create_key = env.new_key().await?;
    let multisig = find_multisig(create_key.address())?.address();
    create_multisig(env, multisig, &create_key, payer).await?;
    assert_multisig(env, multisig, create_key.address(), payer, 1).await?;

    let second = env.new_key().await?.address();
    add_member(env, multisig, second).await?;
    assert_multisig(env, multisig, create_key.address(), payer, 2).await?;

    expect_foreign_config_authority_rejection(env, multisig).await
}

async fn assert_program_config(
    env: &IntegrationEnv,
    program_config: Address,
    payer: Address,
) -> Result<(), Error> {
    let account = env
        .account(program_config)
        .await?
        .expect("injected program config exists");
    let config = decode_program_config(account, CODEC_LIMITS)?;
    assert_eq!(config.data.authority, payer);
    assert_eq!(config.data.multisig_creation_fee, 0);
    assert_eq!(config.data.treasury, TREASURY);
    println!(
        "squads: injected program config decoded authority={} fee={} treasury={}",
        config.data.authority, config.data.multisig_creation_fee, config.data.treasury
    );

    Ok(())
}

async fn create_multisig(
    env: &IntegrationEnv,
    multisig: Address,
    create_key: &solana_kit_core::crypto::Keypair,
    payer: Address,
) -> Result<(), Error> {
    let program = SQUADS_MULTISIG.address();
    let program_config = find_program_config()?.address();
    let create = MultisigCreateV2 {
        config_authority: Some(payer),
        threshold: 1,
        members: vec![member(payer)],
        time_lock: 0,
        rent_collector: None,
        memo: Some("created by the solana-kit program integration suite".to_owned()),
    }
    .instruction_with_program(
        program,
        &[
            meta(program_config, AccountRole::Readonly),
            meta(TREASURY, AccountRole::Writable),
            meta(multisig, AccountRole::Writable),
            meta(create_key.address(), AccountRole::ReadonlySigner),
            meta(payer, AccountRole::WritableSigner),
            meta(SYSTEM_PROGRAM, AccountRole::Readonly),
        ],
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("squads create: {error}")))?;
    let signature = env.submit(&[create], &[env.payer(), create_key]).await?;

    println!("squads: create {signature} landed at {multisig}");

    Ok(())
}

async fn add_member(env: &IntegrationEnv, multisig: Address, second: Address) -> Result<(), Error> {
    let program = SQUADS_MULTISIG.address();
    let add = MultisigAddMember {
        new_member: member(second),
        memo: Some("add second member".to_owned()),
    }
    .instruction_with_program(
        program,
        &[
            meta(multisig, AccountRole::Writable),
            meta(env.payer_address(), AccountRole::ReadonlySigner),
            meta(env.payer_address(), AccountRole::WritableSigner),
            meta(SYSTEM_PROGRAM, AccountRole::Readonly),
        ],
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("squads add member: {error}")))?;
    let signature = env.submit(&[add], &[env.payer()]).await?;

    println!("squads: addMember {signature} expanded the multisig");

    Ok(())
}

async fn assert_multisig(
    env: &IntegrationEnv,
    multisig: Address,
    create_key: Address,
    payer: Address,
    expected_members: usize,
) -> Result<(), Error> {
    let value = decode_expanded_multisig(env, multisig).await?;
    assert_eq!(value.create_key, create_key);
    assert_eq!(value.config_authority, payer);
    assert_eq!(value.threshold, 1);
    assert_eq!(value.members.len(), expected_members);
    println!(
        "squads: decoded multisig create_key={} threshold=1 members={}",
        value.create_key,
        value.members.len()
    );

    Ok(())
}

async fn decode_expanded_multisig(
    env: &IntegrationEnv,
    address: Address,
) -> Result<Multisig, Error> {
    let account = env.account(address).await?.expect("multisig exists");
    assert_eq!(account.owner, SQUADS_MULTISIG.address());
    // Squads reallocates spare tail bytes; the canonical header prefix is strict.
    let (value, _) = MultisigCodec.decode_prefix(&account.data, 0, CODEC_LIMITS)?;

    Ok(value)
}

fn meta(address: Address, role: AccountRole) -> AccountMeta {
    AccountMeta { address, role }
}

fn member(key: Address) -> Member {
    Member {
        key,
        permissions: Permissions {
            mask: ALL_PERMISSIONS,
        },
    }
}

async fn expect_foreign_config_authority_rejection(
    env: &mut IntegrationEnv,
    multisig: Address,
) -> Result<(), Error> {
    let intruder = env.new_key().await?;
    let add = MultisigAddMember {
        new_member: member(env.new_key().await?.address()),
        memo: None,
    }
    .instruction_with_program(
        SQUADS_MULTISIG.address(),
        &[
            meta(multisig, AccountRole::Writable),
            meta(intruder.address(), AccountRole::ReadonlySigner),
            meta(env.payer_address(), AccountRole::WritableSigner),
            meta(SYSTEM_PROGRAM, AccountRole::Readonly),
        ],
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("squads foreign add: {error}")))?;
    let error = env
        .submit(&[add], &[env.payer(), &intruder])
        .await
        .expect_err("a foreign config authority cannot expand the multisig");

    expect_custom_error(&error, 0, FOREIGN_AUTHORITY_ERROR);
    let unchanged = decode_expanded_multisig(env, multisig).await?;
    assert_eq!(unchanged.members.len(), 2);
    println!(
        "squads: foreign config authority rejected with 6004 ({:?}) and members stayed unchanged",
        SquadsMultisigError::from_code(FOREIGN_AUTHORITY_ERROR)
    );

    Ok(())
}

#[tokio::test]
async fn creates_a_multisig_and_expands_its_members() {
    let program_config = find_program_config().expect("program config PDA").address();
    let mut env = util::start_with_accounts(&[SQUADS_MULTISIG], move |payer| {
        let data = ProgramConfigCodec.encode(
            &ProgramConfig {
                authority: payer,
                multisig_creation_fee: 0,
                treasury: TREASURY,
                reserved: [0; 64],
            },
            1 << 20,
        )?;

        Ok(vec![
            solana_kit_program_integration::fixtures::LocalAccount {
                address: program_config,
                lamports: 2_000_000,
                owner: SQUADS_MULTISIG.address(),
                data,
            },
        ])
    })
    .await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
