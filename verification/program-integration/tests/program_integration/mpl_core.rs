//! Real MPL Core asset creation, transfer and decoded account state.

use crate::util;
use solana_kit_core::{AccountMeta, AccountRole, Address, crypto::Keypair};
use solana_kit_mpl_core::{
    CreateV1, DataState, TransferV1, UpdateAuthority, decode_asset_v1_with_program,
};
use solana_kit_program_integration::{
    CODEC_LIMITS, Error, IntegrationEnv, expect_custom_error, fixtures::MPL_CORE,
};
use solana_kit_system::SYSTEM_PROGRAM;

const NAME: &str = "Solana Kit Core asset";
const URI: &str = "https://example.invalid/core.json";
const FOREIGN_AUTHORITY_ERROR: u32 = 26;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let asset = create_asset(env, payer).await?;
    let new_owner = transfer_asset(env, asset.address(), payer).await?;

    expect_foreign_authority_rejection(env, asset.address(), payer, new_owner).await
}

async fn create_asset(env: &mut IntegrationEnv, payer: Address) -> Result<Keypair, Error> {
    let program = MPL_CORE.address();
    let asset = env.new_key().await?;
    let create = CreateV1 {
        data_state: DataState::AccountState,
        name: NAME.to_owned(),
        uri: URI.to_owned(),
        plugins: None,
    }
    .instruction_with_program(
        program,
        &[
            meta(asset.address(), AccountRole::WritableSigner),
            meta(program, AccountRole::Readonly),
            meta(program, AccountRole::Readonly),
            meta(payer, AccountRole::WritableSigner),
            meta(payer, AccountRole::Readonly),
            meta(payer, AccountRole::Readonly),
            meta(SYSTEM_PROGRAM, AccountRole::Readonly),
            meta(program, AccountRole::Readonly),
        ],
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("mpl-core create: {error}")))?;
    let signature = env.submit(&[create], &[env.payer(), &asset]).await?;

    let created = decode_asset(env, asset.address(), program).await?;
    assert_eq!(created.owner, payer);
    assert_eq!(created.update_authority, UpdateAuthority::Address(payer));
    assert_eq!(created.name, NAME);
    assert_eq!(created.uri, URI);
    println!(
        "mpl_core: create {signature} decoded owner={} authority={payer} name={:?}",
        created.owner, created.name
    );

    Ok(asset)
}

async fn transfer_asset(
    env: &mut IntegrationEnv,
    asset: Address,
    payer: Address,
) -> Result<Address, Error> {
    let program = MPL_CORE.address();
    let new_owner = env.new_key().await?;
    let transfer = TransferV1 {
        compression_proof: None,
    }
    .instruction_with_program(
        program,
        &[
            meta(asset, AccountRole::Writable),
            meta(program, AccountRole::Readonly),
            meta(payer, AccountRole::WritableSigner),
            meta(payer, AccountRole::ReadonlySigner),
            meta(new_owner.address(), AccountRole::Readonly),
            meta(SYSTEM_PROGRAM, AccountRole::Readonly),
            meta(program, AccountRole::Readonly),
        ],
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("mpl-core transfer: {error}")))?;
    let signature = env.submit(&[transfer], &[env.payer()]).await?;

    let transferred = decode_asset(env, asset, program).await?;
    assert_eq!(transferred.owner, new_owner.address());
    assert_eq!(transferred.name, NAME);
    assert_eq!(transferred.uri, URI);
    println!(
        "mpl_core: transfer {signature} changed only the owner to {}",
        transferred.owner
    );

    Ok(new_owner.address())
}

fn meta(address: Address, role: AccountRole) -> AccountMeta {
    AccountMeta { address, role }
}

async fn decode_asset(
    env: &IntegrationEnv,
    asset: Address,
    program: Address,
) -> Result<solana_kit_mpl_core::AssetV1, Error> {
    let account = env.account(asset).await?.expect("Core asset exists");
    let decoded = decode_asset_v1_with_program(account, program, CODEC_LIMITS)?;

    Ok(decoded.data.base)
}

async fn expect_foreign_authority_rejection(
    env: &mut IntegrationEnv,
    asset: Address,
    payer: Address,
    current_owner: Address,
) -> Result<(), Error> {
    let intruder = env.new_key().await?;
    let transfer = TransferV1 {
        compression_proof: None,
    }
    .instruction_with_program(
        MPL_CORE.address(),
        &[
            meta(asset, AccountRole::Writable),
            meta(MPL_CORE.address(), AccountRole::Readonly),
            meta(payer, AccountRole::WritableSigner),
            meta(intruder.address(), AccountRole::ReadonlySigner),
            meta(payer, AccountRole::Readonly),
            meta(SYSTEM_PROGRAM, AccountRole::Readonly),
            meta(MPL_CORE.address(), AccountRole::Readonly),
        ],
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("mpl-core foreign transfer: {error}")))?;
    let error = env
        .submit(&[transfer], &[env.payer(), &intruder])
        .await
        .expect_err("a foreign authority cannot transfer the asset");

    expect_custom_error(&error, 0, FOREIGN_AUTHORITY_ERROR);
    let unchanged = decode_asset(env, asset, MPL_CORE.address()).await?;
    assert_eq!(unchanged.owner, current_owner);
    println!(
        "mpl_core: foreign authority rejected with 26 ({:?}) and the owner stayed unchanged",
        solana_kit_mpl_core::MplCoreError::from_code(FOREIGN_AUTHORITY_ERROR)
    );

    Ok(())
}

#[tokio::test]
async fn creates_and_transfers_a_core_asset_and_rejects_a_foreign_authority() {
    let mut env = util::start_with(&[MPL_CORE]).await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
