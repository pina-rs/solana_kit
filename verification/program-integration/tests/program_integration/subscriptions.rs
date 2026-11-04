//! Real Subscriptions program: authority PDA, plan creation and decoded state.

use crate::util;
use solana_kit_core::{Address, Instruction, plans::Plan};
use solana_kit_program_integration::{
    CODEC_LIMITS, Error, IntegrationEnv, expect_custom_error, fixtures::SUBSCRIPTIONS,
};
use solana_kit_subscriptions::{
    CreatePlan, CreatePlanAccounts, InitSubscriptionAuthority, InitSubscriptionAuthorityAccounts,
    PlanData, PlanTerms, SubscriptionsError, decode_plan_with_program,
    decode_subscription_authority_with_program, find_plan, find_subscription_authority,
};
use solana_kit_token::{
    CreateMintConfig, CreateMintInput, MintToAtaConfig, MintToAtaDerivedInput, PROGRAM,
    create_mint_plan, mint_to_ata_derived_plan,
};

const DECIMALS: u8 = 9;
const PLAN_ID: u64 = 7;
const AMOUNT: u64 = 42;
const PERIOD_HOURS: u64 = 24;
const PLAN_EXISTS_ERROR: u32 = 518;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let mint = create_mint_and_ata(env, payer).await?;
    let authority = init_authority(env, payer, mint).await?;
    let plan = create_plan(env, payer, mint).await?;

    assert_authority(env, authority, payer, mint).await?;
    assert_plan(env, plan, payer, mint).await?;
    expect_duplicate_plan_rejection(env, payer, mint).await
}

async fn create_mint_and_ata(env: &mut IntegrationEnv, payer: Address) -> Result<Address, Error> {
    let mint = env.new_key().await?;
    let mint_plan = create_mint_plan(
        CreateMintInput {
            payer,
            new_mint: mint.address(),
            decimals: DECIMALS,
            mint_authority: payer,
            freeze_authority: None,
            mint_account_lamports: Some(env.rent(solana_kit_token::MINT_ACCOUNT_SIZE).await?),
        },
        CreateMintConfig::default(),
        CODEC_LIMITS,
    )?;
    env.submit(&flatten(mint_plan), &[env.payer(), &mint])
        .await?;

    let ata_plan = mint_to_ata_derived_plan(
        MintToAtaDerivedInput {
            payer,
            owner: payer,
            mint: mint.address(),
            mint_authority: payer,
            amount: 0,
            decimals: DECIMALS,
        },
        MintToAtaConfig::default(),
        CODEC_LIMITS,
    )?;
    env.submit(&flatten(ata_plan), &[env.payer()]).await?;

    Ok(mint.address())
}

async fn init_authority(
    env: &IntegrationEnv,
    payer: Address,
    mint: Address,
) -> Result<Address, Error> {
    let program = SUBSCRIPTIONS.address();
    let ata = derive_ata(payer, mint);
    let authority = find_subscription_authority(payer, mint)?.address();
    let init = InitSubscriptionAuthorityAccounts {
        owner: payer,
        subscription_authority: Some(authority),
        token_mint: mint,
        user_ata: ata,
        system_program: None,
        token_program: PROGRAM,
        payer: None,
    }
    .instruction_with_program(program, &InitSubscriptionAuthority {}, CODEC_LIMITS)
    .map_err(|error| Error::Program(format!("subscriptions authority: {error}")))?;
    let signature = env.submit(&[init], &[env.payer()]).await?;

    let account = env.account(authority).await?.expect("authority exists");
    let state = decode_subscription_authority_with_program(account, program, CODEC_LIMITS)?;
    assert_eq!(state.data.user, payer);
    assert_eq!(state.data.token_mint, mint);
    println!(
        "subscriptions: authority {signature} decoded user={} mint={} init_id={}",
        state.data.user, state.data.token_mint, state.data.init_id
    );

    Ok(authority)
}

async fn create_plan(
    env: &IntegrationEnv,
    payer: Address,
    mint: Address,
) -> Result<Address, Error> {
    let program = SUBSCRIPTIONS.address();
    let plan = find_plan(payer, PLAN_ID)?.address();
    let create = CreatePlanAccounts {
        merchant: payer,
        plan_pda: plan,
        token_mint: mint,
        system_program: None,
        token_program: None,
        payer: None,
    }
    .instruction_with_program(
        program,
        &CreatePlan {
            plan_data: plan_data(payer, mint),
        },
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("subscriptions plan: {error}")))?;
    let signature = env.submit(&[create], &[env.payer()]).await?;

    println!("subscriptions: plan {signature} created at {plan}");

    Ok(plan)
}

async fn assert_authority(
    env: &IntegrationEnv,
    authority: Address,
    payer: Address,
    mint: Address,
) -> Result<(), Error> {
    let account = env.account(authority).await?.expect("authority exists");
    let state =
        decode_subscription_authority_with_program(account, SUBSCRIPTIONS.address(), CODEC_LIMITS)?;
    assert_eq!(state.data.user, payer);
    assert_eq!(state.data.token_mint, mint);

    Ok(())
}

async fn assert_plan(
    env: &IntegrationEnv,
    plan: Address,
    payer: Address,
    mint: Address,
) -> Result<(), Error> {
    let account = env.account(plan).await?.expect("plan exists");
    let decoded = decode_plan_with_program(account, SUBSCRIPTIONS.address(), CODEC_LIMITS)?;
    assert_eq!(decoded.data.owner, payer);
    assert_eq!(decoded.data.data.plan_id, PLAN_ID);
    assert_eq!(decoded.data.data.mint, mint);
    assert_eq!(decoded.data.data.terms.amount, AMOUNT);
    assert_eq!(decoded.data.data.terms.period_hours, PERIOD_HOURS);
    println!(
        "subscriptions: plan decoded owner={} plan_id={} amount={} period_hours={}",
        decoded.data.owner,
        decoded.data.data.plan_id,
        decoded.data.data.terms.amount,
        decoded.data.data.terms.period_hours
    );

    Ok(())
}

fn plan_data(payer: Address, mint: Address) -> PlanData {
    PlanData {
        plan_id: PLAN_ID,
        mint,
        terms: PlanTerms {
            amount: AMOUNT,
            period_hours: PERIOD_HOURS,
            created_at: 0,
        },
        end_ts: 0,
        destinations: vec![payer; 4],
        pullers: vec![payer; 4],
        metadata_uri: "https://example.invalid/plan".to_owned(),
    }
}

fn derive_ata(owner: Address, mint: Address) -> Address {
    solana_kit_associated_token_account::AssociatedTokenSeeds {
        owner,
        token_program: PROGRAM,
        mint,
    }
    .find()
    .expect("canonical ATA derivation always finds a bump")
    .address()
}

fn flatten(plan: Plan<Instruction>) -> Vec<Instruction> {
    plan.flatten().into_iter().cloned().collect()
}

async fn expect_duplicate_plan_rejection(
    env: &IntegrationEnv,
    payer: Address,
    mint: Address,
) -> Result<(), Error> {
    let program = SUBSCRIPTIONS.address();
    let plan = find_plan(payer, PLAN_ID)?.address();
    let create = CreatePlanAccounts {
        merchant: payer,
        plan_pda: plan,
        token_mint: mint,
        system_program: None,
        token_program: None,
        payer: None,
    }
    .instruction_with_program(
        program,
        &CreatePlan {
            plan_data: plan_data(payer, mint),
        },
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("subscriptions duplicate plan: {error}")))?;
    let error = env
        .submit(&[create], &[env.payer()])
        .await
        .expect_err("an occupied plan PDA cannot be initialized twice");

    expect_custom_error(&error, 0, PLAN_EXISTS_ERROR);
    println!(
        "subscriptions: duplicate plan rejected with 518 ({:?})",
        SubscriptionsError::from_code(PLAN_EXISTS_ERROR)
    );

    Ok(())
}

#[tokio::test]
async fn creates_authority_and_plan_and_rejects_a_duplicate() {
    let mut env = util::start_with(&[SUBSCRIPTIONS]).await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
