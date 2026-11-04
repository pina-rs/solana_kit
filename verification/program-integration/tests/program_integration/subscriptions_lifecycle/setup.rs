//! Fresh subscriber, merchant, mint, authority and plan for the lifecycle.

use solana_kit_client::parsed::{ParsedAccountData, TokenAccountState, TokenProgramAccount};
use solana_kit_core::{Address, Instruction, crypto::Keypair};
use solana_kit_program_integration::{
    CODEC_LIMITS, Error, IntegrationEnv, PARSED_LIMITS, fixtures::SUBSCRIPTIONS,
};
use solana_kit_subscriptions::{
    CreatePlan, CreatePlanAccounts, InitSubscriptionAuthority, InitSubscriptionAuthorityAccounts,
    PlanData, PlanTerms, decode_plan_with_program, decode_subscription_authority_with_program,
    find_plan, find_subscription_authority, find_subscription_delegation,
};
use solana_kit_system::{SYSTEM_PROGRAM, TransferSol, TransferSolAccounts};
use solana_kit_token::{
    AccountState, CreateMintConfig, CreateMintInput, MintCodec, MintToAtaConfig,
    MintToAtaDerivedInput, PROGRAM, TokenCodec, create_mint_plan, mint_to_ata_derived_plan,
};

const DECIMALS: u8 = 9;
pub(crate) const PLAN_ID: u64 = 7;
const AMOUNT: u64 = 42;
const PERIOD_HOURS: u64 = 24;
const MINTED: u64 = 1_000_000;
const SUBSCRIBER_FUNDING: u64 = 1_000_000_000;
const MERCHANT_FUNDING: u64 = 1_000_000_000;
const INTRUDER_FUNDING: u64 = 1_000_000_000;

// Source literals from the pinned program, asserted against the node rather
// than trusted from the client encoder.
const PLAN_ACCOUNT_LEN: u64 = 491;
const AUTHORITY_ACCOUNT_LEN: u64 = 106;
const PLAN_DISCRIMINATOR: u8 = 1;
const PLAN_STATUS_ACTIVE: u8 = 1;

/// Stable inputs for the subscribe, cancel, resume and rejection journeys.
pub(crate) struct Lifecycle {
    pub(crate) subscriber: Keypair,
    pub(crate) intruder: Keypair,
    pub(crate) merchant: Address,
    pub(crate) mint: Address,
    pub(crate) subscriber_ata: Address,
    pub(crate) authority: Address,
    pub(crate) authority_init_id: i64,
    pub(crate) plan: Address,
    pub(crate) plan_bump: u8,
    pub(crate) plan_terms: PlanTerms,
    pub(crate) subscription: Address,
    pub(crate) event_authority: Address,
}

pub(crate) async fn create(env: &mut IntegrationEnv) -> Result<Lifecycle, Error> {
    let payer = env.payer_address();
    let subscriber = funded_signer(env, SUBSCRIBER_FUNDING).await?;
    let merchant = funded_signer(env, MERCHANT_FUNDING).await?;
    let intruder = funded_signer(env, INTRUDER_FUNDING).await?;
    let mint = env.new_key().await?;

    create_mint(env, payer, &mint).await?;
    let subscriber_ata = derive_ata(subscriber.address(), mint.address());
    mint_to_subscriber(env, payer, subscriber.address(), mint.address()).await?;

    let authority = find_subscription_authority(subscriber.address(), mint.address())?.address();
    let authority_init_id =
        initialize_authority(env, &subscriber, mint.address(), subscriber_ata).await?;
    let (plan, plan_bump, plan_terms) = create_plan(env, &merchant, mint.address()).await?;
    let subscription = find_subscription_delegation(plan, subscriber.address())?.address();
    let event_authority = find_event_authority_address();

    println!(
        "subscriptions-lifecycle: subscriber={} merchant={} intruder={} mint={} subscriber_ata={subscriber_ata}",
        subscriber.address(),
        merchant.address(),
        intruder.address(),
        mint.address()
    );
    println!(
        "subscriptions-lifecycle: authority={authority} authority_init_id={authority_init_id} plan={plan} plan_bump={plan_bump} plan_created_at={} subscription={subscription} event_authority={event_authority}",
        plan_terms.created_at
    );

    Ok(Lifecycle {
        subscriber,
        intruder,
        merchant: merchant.address(),
        mint: mint.address(),
        subscriber_ata,
        authority,
        authority_init_id,
        plan,
        plan_bump,
        plan_terms,
        subscription,
        event_authority,
    })
}

async fn funded_signer(env: &mut IntegrationEnv, amount: u64) -> Result<Keypair, Error> {
    let key = env.new_key().await?;
    let transfer = TransferSol { amount }.instruction(
        TransferSolAccounts {
            source: env.payer_address(),
            destination: key.address(),
        },
        CODEC_LIMITS,
    )?;
    env.submit(&[transfer], &[env.payer()]).await?;

    assert_eq!(
        env.balance(key.address()).await?,
        amount,
        "the fresh signer must hold exactly the transferred lamports"
    );

    Ok(key)
}

async fn create_mint(env: &IntegrationEnv, payer: Address, mint: &Keypair) -> Result<(), Error> {
    let plan = create_mint_plan(
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
    env.submit(&flatten(plan), &[env.payer(), mint]).await?;

    Ok(())
}

async fn mint_to_subscriber(
    env: &IntegrationEnv,
    payer: Address,
    subscriber: Address,
    mint: Address,
) -> Result<(), Error> {
    let plan = mint_to_ata_derived_plan(
        MintToAtaDerivedInput {
            payer,
            owner: subscriber,
            mint,
            mint_authority: payer,
            amount: MINTED,
            decimals: DECIMALS,
        },
        MintToAtaConfig::default(),
        CODEC_LIMITS,
    )?;
    let signature = env.submit(&flatten(plan), &[env.payer()]).await?;

    let account = env
        .account(mint)
        .await?
        .ok_or(Error::Client("the subscription mint was not created"))?;
    let state = account.decode(&MintCodec, PROGRAM, CODEC_LIMITS)?;
    assert_eq!(state.data.supply, MINTED);
    assert_eq!(state.data.decimals, DECIMALS);
    println!("subscriptions-lifecycle: mint {signature} supply={MINTED} decimals={DECIMALS}");

    Ok(())
}

async fn initialize_authority(
    env: &IntegrationEnv,
    subscriber: &Keypair,
    mint: Address,
    subscriber_ata: Address,
) -> Result<i64, Error> {
    let program = SUBSCRIPTIONS.address();
    let authority = find_subscription_authority(subscriber.address(), mint)?.address();
    let instruction = InitSubscriptionAuthorityAccounts {
        owner: subscriber.address(),
        subscription_authority: Some(authority),
        token_mint: mint,
        user_ata: subscriber_ata,
        system_program: Some(SYSTEM_PROGRAM),
        token_program: PROGRAM,
        payer: None,
    }
    .instruction_with_program(program, &InitSubscriptionAuthority {}, CODEC_LIMITS)
    .map_err(|error| Error::Program(format!("subscriptions authority: {error}")))?;
    let signature = env
        .submit(&[instruction], &[env.payer(), subscriber])
        .await?;

    let account = env
        .account(authority)
        .await?
        .ok_or(Error::Client("the subscription authority was not created"))?;
    assert_eq!(account.space, AUTHORITY_ACCOUNT_LEN);
    let state = decode_subscription_authority_with_program(account, program, CODEC_LIMITS)?;
    assert_eq!(state.data.discriminator, 0);
    assert_eq!(state.data.user, subscriber.address());
    assert_eq!(state.data.token_mint, mint);
    assert_eq!(state.data.payer, subscriber.address());
    assert!(state.data.init_id > 0, "the init id is the creation slot");
    println!(
        "subscriptions-lifecycle: authority {signature} decoded user={} mint={} payer={} bump={} init_id={}",
        state.data.user,
        state.data.token_mint,
        state.data.payer,
        state.data.bump,
        state.data.init_id
    );

    Ok(state.data.init_id)
}

async fn create_plan(
    env: &IntegrationEnv,
    merchant: &Keypair,
    mint: Address,
) -> Result<(Address, u8, PlanTerms), Error> {
    let program = SUBSCRIPTIONS.address();
    let plan = find_plan(merchant.address(), PLAN_ID)?;
    let instruction = CreatePlanAccounts {
        merchant: merchant.address(),
        plan_pda: plan.address(),
        token_mint: mint,
        system_program: Some(SYSTEM_PROGRAM),
        token_program: Some(PROGRAM),
        payer: None,
    }
    .instruction_with_program(
        program,
        &CreatePlan {
            plan_data: plan_data(merchant.address(), mint),
        },
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("subscriptions plan: {error}")))?;
    let signature = env.submit(&[instruction], &[env.payer(), merchant]).await?;

    let account = env
        .account(plan.address())
        .await?
        .ok_or(Error::Client("the subscription plan was not created"))?;
    assert_eq!(account.space, PLAN_ACCOUNT_LEN);
    let state = decode_plan_with_program(account, program, CODEC_LIMITS)?;
    assert_eq!(state.data.discriminator, PLAN_DISCRIMINATOR);
    assert_eq!(state.data.owner, merchant.address());
    assert_eq!(state.data.bump, plan.bump());
    assert_eq!(state.data.status, PLAN_STATUS_ACTIVE);
    assert_eq!(state.data.data.plan_id, PLAN_ID);
    assert_eq!(state.data.data.mint, mint);
    assert_eq!(state.data.data.terms.amount, AMOUNT);
    assert_eq!(state.data.data.terms.period_hours, PERIOD_HOURS);
    assert_eq!(state.data.data.end_ts, 0);
    assert!(state.data.data.terms.created_at > 0);
    println!(
        "subscriptions-lifecycle: plan {signature} decoded owner={} plan_id={} amount={} period_hours={} created_at={} end_ts={} status={}",
        state.data.owner,
        state.data.data.plan_id,
        state.data.data.terms.amount,
        state.data.data.terms.period_hours,
        state.data.data.terms.created_at,
        state.data.data.end_ts,
        state.data.status
    );

    Ok((plan.address(), plan.bump(), state.data.data.terms))
}

pub(crate) async fn assert_token_delegation(
    env: &IntegrationEnv,
    lifecycle: &Lifecycle,
) -> Result<(), Error> {
    let account = env
        .account(lifecycle.subscriber_ata)
        .await?
        .ok_or(Error::Client("the subscriber token account does not exist"))?;
    let state = account.decode(&TokenCodec, PROGRAM, CODEC_LIMITS)?;
    assert_eq!(state.data.mint, lifecycle.mint);
    assert_eq!(state.data.owner, lifecycle.subscriber.address());
    assert_eq!(state.data.amount, MINTED);
    assert_eq!(state.data.delegate, Some(lifecycle.authority));
    assert_eq!(state.data.delegated_amount, u64::MAX);
    assert_eq!(state.data.state, AccountState::Initialized);

    let parsed = env
        .parsed_account(lifecycle.subscriber_ata)
        .await?
        .ok_or(Error::Client(
            "the subscriber token account is absent from jsonParsed",
        ))?;

    match parsed.decode_typed(PARSED_LIMITS)? {
        ParsedAccountData::Token(TokenProgramAccount::Account(info)) => {
            assert_eq!(info.mint, lifecycle.mint);
            assert_eq!(info.owner, lifecycle.subscriber.address());
            assert_eq!(info.token_amount.amount, MINTED);
            assert_eq!(info.delegate, Some(lifecycle.authority));
            let delegated = info
                .delegated_amount
                .expect("an approved delegate has a delegated amount");
            assert_eq!(delegated.amount, u64::MAX);
            assert_eq!(info.state, TokenAccountState::Initialized);
        }
        other => panic!("expected a parsed SPL Token account, got {other:?}"),
    }

    println!(
        "subscriptions-lifecycle: subscriber ATA decoded amount={} delegate={} delegated_amount={} state=initialized",
        state.data.amount, lifecycle.authority, state.data.delegated_amount
    );

    Ok(())
}

fn find_event_authority_address() -> Address {
    solana_kit_subscriptions::find_event_authority()
        .expect("the canonical event authority PDA exists")
        .address()
}

fn plan_data(merchant: Address, mint: Address) -> PlanData {
    PlanData {
        plan_id: PLAN_ID,
        mint,
        terms: PlanTerms {
            amount: AMOUNT,
            period_hours: PERIOD_HOURS,
            created_at: 0,
        },
        end_ts: 0,
        destinations: vec![merchant; 4],
        pullers: vec![merchant; 4],
        metadata_uri: "https://example.invalid/subscription".to_owned(),
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

fn flatten(plan: solana_kit_core::plans::Plan<Instruction>) -> Vec<Instruction> {
    plan.flatten().into_iter().cloned().collect()
}
