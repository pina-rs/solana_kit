//! Real Token Metadata program: create and update metadata for an SPL mint.

use crate::util;
use solana_kit_core::{AccountMeta, AccountRole, Address, Instruction, plans::Plan};
use solana_kit_mpl_token_metadata::{
    AllocationPolicy, CreateMetadataAccountV3, Creator, DataV2, UpdateMetadataAccountV2,
    decode_metadata_with_program, find_metadata,
};
use solana_kit_program_integration::{
    CODEC_LIMITS, Error, IntegrationEnv, expect_custom_error, fixtures::MPL_TOKEN_METADATA,
};
use solana_kit_system::SYSTEM_PROGRAM;
use solana_kit_token::{CreateMintConfig, CreateMintInput, create_mint_plan};

const NAME: &str = "Solana Kit Metadata";
const SYMBOL: &str = "KIT";
const URI: &str = "https://example.invalid/metadata-v1.json";
const UPDATED_NAME: &str = "Solana Kit Metadata v2";
const UPDATED_URI: &str = "https://example.invalid/metadata-v2.json";
const SELLER_FEE_BASIS_POINTS: u16 = 250;
const FOREIGN_AUTHORITY_ERROR: u32 = 7;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let mint = create_mint(env, payer).await?;
    let metadata = create_metadata(env, payer, mint).await?;
    update_metadata(env, payer, metadata).await?;

    expect_foreign_update_authority_rejection(env, metadata, payer).await
}

async fn create_mint(env: &mut IntegrationEnv, payer: Address) -> Result<Address, Error> {
    let mint = env.new_key().await?;
    let plan = create_mint_plan(
        CreateMintInput {
            payer,
            new_mint: mint.address(),
            decimals: 0,
            mint_authority: payer,
            freeze_authority: None,
            mint_account_lamports: Some(env.rent(solana_kit_token::MINT_ACCOUNT_SIZE).await?),
        },
        CreateMintConfig::default(),
        CODEC_LIMITS,
    )?;
    env.submit(&flatten(plan), &[env.payer(), &mint]).await?;

    Ok(mint.address())
}

async fn create_metadata(
    env: &mut IntegrationEnv,
    payer: Address,
    mint: Address,
) -> Result<Address, Error> {
    let program = MPL_TOKEN_METADATA.address();
    let metadata = find_metadata(mint)?.address();
    let create = CreateMetadataAccountV3 {
        data: metadata_data(payer, NAME, URI),
        is_mutable: true,
        collection_details: None,
    }
    .instruction_with_program(
        program,
        &[
            meta(metadata, AccountRole::Writable),
            meta(mint, AccountRole::Readonly),
            meta(payer, AccountRole::ReadonlySigner),
            meta(payer, AccountRole::WritableSigner),
            meta(payer, AccountRole::ReadonlySigner),
            meta(SYSTEM_PROGRAM, AccountRole::Readonly),
        ],
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("metadata create: {error}")))?;
    let signature = env.submit(&[create], &[env.payer()]).await?;

    let created = decode_metadata(env, metadata, program).await?;
    assert_eq!(created.mint, mint);
    assert_eq!(created.update_authority, payer);
    assert_eq!(
        created.data.seller_fee_basis_points,
        SELLER_FEE_BASIS_POINTS
    );
    assert!(created.data.name.starts_with(NAME));
    assert!(created.data.uri.starts_with(URI));
    assert!(created.is_mutable);
    println!(
        "metadata: create {signature} decoded mint={} authority={payer} fee={} name={:?}",
        created.mint,
        created.data.seller_fee_basis_points,
        created.data.name.trim_end_matches('\0')
    );

    Ok(metadata)
}

async fn update_metadata(
    env: &IntegrationEnv,
    payer: Address,
    metadata: Address,
) -> Result<(), Error> {
    let program = MPL_TOKEN_METADATA.address();
    let update = UpdateMetadataAccountV2 {
        data: Some(metadata_data(payer, UPDATED_NAME, UPDATED_URI)),
        new_update_authority: None,
        primary_sale_happened: None,
        is_mutable: None,
    }
    .instruction_with_program(
        program,
        &[
            meta(metadata, AccountRole::Writable),
            meta(payer, AccountRole::ReadonlySigner),
        ],
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("metadata update: {error}")))?;
    let signature = env.submit(&[update], &[env.payer()]).await?;

    let updated = decode_metadata(env, metadata, program).await?;
    assert!(updated.data.name.starts_with(UPDATED_NAME));
    assert!(updated.data.uri.starts_with(UPDATED_URI));
    assert_eq!(
        updated.data.seller_fee_basis_points,
        SELLER_FEE_BASIS_POINTS
    );
    assert!(updated.is_mutable);
    println!(
        "metadata: update {signature} decoded name={:?} uri={:?}",
        updated.data.name.trim_end_matches('\0'),
        updated.data.uri.trim_end_matches('\0')
    );

    Ok(())
}

fn meta(address: Address, role: AccountRole) -> AccountMeta {
    AccountMeta { address, role }
}

fn metadata_data(payer: Address, name: &str, uri: &str) -> DataV2 {
    DataV2 {
        name: name.to_owned(),
        symbol: SYMBOL.to_owned(),
        uri: uri.to_owned(),
        seller_fee_basis_points: SELLER_FEE_BASIS_POINTS,
        creators: Some(vec![Creator {
            address: payer,
            verified: true,
            share: 100,
        }]),
        collection: None,
        uses: None,
    }
}

fn flatten(plan: Plan<Instruction>) -> Vec<Instruction> {
    plan.flatten().into_iter().cloned().collect()
}

async fn decode_metadata(
    env: &IntegrationEnv,
    metadata: Address,
    program: Address,
) -> Result<solana_kit_mpl_token_metadata::Metadata, Error> {
    let account = env.account(metadata).await?.expect("metadata exists");
    let decoded = decode_metadata_with_program(
        account,
        program,
        AllocationPolicy::PreserveTail,
        CODEC_LIMITS,
    )?;

    Ok(decoded.data.value)
}

async fn expect_foreign_update_authority_rejection(
    env: &mut IntegrationEnv,
    metadata: Address,
    payer: Address,
) -> Result<(), Error> {
    let program = MPL_TOKEN_METADATA.address();
    let intruder = env.new_key().await?;
    let update = UpdateMetadataAccountV2 {
        data: Some(unauthorized_data(&intruder.address())),
        new_update_authority: None,
        primary_sale_happened: None,
        is_mutable: None,
    }
    .instruction_with_program(
        program,
        &[
            meta(metadata, AccountRole::Writable),
            meta(intruder.address(), AccountRole::ReadonlySigner),
        ],
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("metadata foreign update: {error}")))?;
    let error = env
        .submit(&[update], &[env.payer(), &intruder])
        .await
        .expect_err("a foreign signer cannot update the metadata");

    expect_custom_error(&error, 0, FOREIGN_AUTHORITY_ERROR);
    let unchanged = decode_metadata(env, metadata, program).await?;
    assert!(unchanged.data.name.starts_with(UPDATED_NAME));
    assert_eq!(unchanged.update_authority, payer);
    println!(
        "metadata: foreign update authority rejected with 7 ({:?}) and state stayed unchanged",
        solana_kit_mpl_token_metadata::MplTokenMetadataError::from_code(FOREIGN_AUTHORITY_ERROR)
    );

    Ok(())
}

fn unauthorized_data(payer: &Address) -> DataV2 {
    DataV2 {
        name: "Unauthorized".to_owned(),
        symbol: SYMBOL.to_owned(),
        uri: UPDATED_URI.to_owned(),
        seller_fee_basis_points: SELLER_FEE_BASIS_POINTS,
        creators: Some(vec![Creator {
            address: *payer,
            verified: true,
            share: 100,
        }]),
        collection: None,
        uses: None,
    }
}

#[tokio::test]
async fn creates_and_updates_metadata_and_rejects_a_foreign_authority() {
    let mut env = util::start_with(&[MPL_TOKEN_METADATA]).await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
