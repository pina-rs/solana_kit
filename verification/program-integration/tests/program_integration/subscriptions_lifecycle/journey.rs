//! Subscribe, cancel, resume and their confirmed rejection transitions.

use super::events::{self, SubscriptionEvent};
use super::setup::Lifecycle;
use solana_kit_core::{Address, Instruction, crypto::Keypair};
use solana_kit_program_integration::{
    CODEC_LIMITS, Error, IntegrationEnv, expect_custom_error, fixtures::SUBSCRIPTIONS,
};
use solana_kit_subscriptions::{
    CancelSubscription, CancelSubscriptionAccounts, ResumeData, ResumeSubscription,
    ResumeSubscriptionAccounts, Subscribe, SubscribeAccounts, SubscribeData,
    SubscriptionDelegation, SubscriptionsError, decode_subscription_delegation_with_program,
};
use solana_kit_system::SYSTEM_PROGRAM;

const PERIOD_SECONDS: i64 = 24 * 60 * 60;
const SUBSCRIPTION_ACCOUNT_LEN: u64 = 155;
const SUBSCRIPTION_DISCRIMINATOR: u8 = 4;
const ACCOUNT_VERSION: u8 = 1;

const SUBSCRIBE_DISCRIMINATOR: u8 = 11;
const CANCEL_DISCRIMINATOR: u8 = 12;
const RESUME_DISCRIMINATOR: u8 = 13;
const SUBSCRIBE_DATA_LEN: usize = 1 + 8 + 1 + 32 + 8 + 8 + 8 + 8;
const RESUME_DATA_LEN: usize = 1 + 8;

const UNAUTHORIZED: u32 = 130;
const ALREADY_CANCELLED: u32 = 509;
const NOT_CANCELLED: u32 = 510;
const STALE_APPROVAL: u32 = 521;

pub(crate) async fn subscribe(env: &IntegrationEnv, lifecycle: &Lifecycle) -> Result<(), Error> {
    let program = SUBSCRIPTIONS.address();
    let instruction = SubscribeAccounts {
        subscriber: lifecycle.subscriber.address(),
        merchant: lifecycle.merchant,
        plan_pda: lifecycle.plan,
        subscription_pda: Some(lifecycle.subscription),
        subscription_authority_pda: lifecycle.authority,
        system_program: Some(SYSTEM_PROGRAM),
        event_authority: Some(lifecycle.event_authority),
        self_program: Some(program),
        payer: None,
    }
    .instruction_with_program(
        program,
        &Subscribe {
            subscribe_data: SubscribeData {
                plan_id: super::setup::PLAN_ID,
                plan_bump: lifecycle.plan_bump,
                expected_mint: lifecycle.mint,
                expected_amount: lifecycle.plan_terms.amount,
                expected_period_hours: lifecycle.plan_terms.period_hours,
                expected_created_at: lifecycle.plan_terms.created_at,
                expected_subscription_authority_init_id: lifecycle.authority_init_id,
            },
        },
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("subscriptions subscribe: {error}")))?;
    assert_eq!(instruction.data.first(), Some(&SUBSCRIBE_DISCRIMINATOR));
    assert_eq!(instruction.data.len(), SUBSCRIBE_DATA_LEN);
    let signature = env
        .submit(&[instruction], &[env.payer(), &lifecycle.subscriber])
        .await?;

    let account = subscription_account(env, lifecycle.subscription).await?;
    let state = &account.data;
    assert_eq!(account.space, SUBSCRIPTION_ACCOUNT_LEN);
    assert_eq!(state.header.discriminator, SUBSCRIPTION_DISCRIMINATOR);
    assert_eq!(state.header.version, ACCOUNT_VERSION);
    assert_eq!(state.header.delegator, lifecycle.subscriber.address());
    assert_eq!(state.header.delegatee, lifecycle.plan);
    assert_eq!(state.header.payer, lifecycle.subscriber.address());
    assert_eq!(state.header.init_id, lifecycle.authority_init_id);
    assert_eq!(state.terms, lifecycle.plan_terms);
    assert_eq!(state.amount_pulled_in_period, 0);
    assert_eq!(state.expires_at_ts, 0);
    assert!(state.current_period_start_ts > 0);

    let event = events::lifecycle_event(env, signature).await?;
    let SubscriptionEvent::Created(event) = event else {
        panic!("subscribe must emit SubscriptionCreated, got a different event");
    };
    assert_eq!(event.plan, lifecycle.plan);
    assert_eq!(event.subscriber, lifecycle.subscriber.address());
    assert_eq!(event.mint, lifecycle.mint);
    assert_eq!(event.created_ts, state.current_period_start_ts);
    assert_eq!(event.payer, lifecycle.subscriber.address());
    println!(
        "subscriptions-lifecycle: subscribe {signature} bump={} period_start={} expires={} pulled={} created_event_ts={}",
        state.header.bump,
        state.current_period_start_ts,
        state.expires_at_ts,
        state.amount_pulled_in_period,
        event.created_ts
    );

    Ok(())
}

pub(crate) async fn expect_resume_before_cancel(
    env: &IntegrationEnv,
    lifecycle: &Lifecycle,
) -> Result<(), Error> {
    let instruction = resume_instruction(lifecycle, 0)?;

    expect_rejection(
        env,
        lifecycle.subscription,
        instruction,
        &[env.payer(), &lifecycle.subscriber],
        NOT_CANCELLED,
        "resume before cancel",
    )
    .await
}

pub(crate) async fn expect_wrong_subscriber_cancel(
    env: &IntegrationEnv,
    lifecycle: &Lifecycle,
) -> Result<(), Error> {
    let instruction = cancel_instruction(lifecycle, lifecycle.intruder.address())?;

    expect_rejection(
        env,
        lifecycle.subscription,
        instruction,
        &[env.payer(), &lifecycle.intruder],
        UNAUTHORIZED,
        "cancel by a different signer",
    )
    .await
}

pub(crate) async fn cancel(env: &IntegrationEnv, lifecycle: &Lifecycle) -> Result<i64, Error> {
    let instruction = cancel_instruction(lifecycle, lifecycle.subscriber.address())?;
    let signature = env
        .submit(&[instruction], &[env.payer(), &lifecycle.subscriber])
        .await?;

    let account = subscription_account(env, lifecycle.subscription).await?;
    let state = &account.data;
    let expected_expiry = state
        .current_period_start_ts
        .checked_add(PERIOD_SECONDS)
        .expect("the test clock stays inside the i64 domain");
    assert_eq!(state.expires_at_ts, expected_expiry);
    assert_eq!(state.amount_pulled_in_period, 0);

    let event = events::lifecycle_event(env, signature).await?;
    let SubscriptionEvent::Cancelled(event) = event else {
        panic!("cancel must emit SubscriptionCancelled, got a different event");
    };
    assert_eq!(event.plan, lifecycle.plan);
    assert_eq!(event.subscriber, lifecycle.subscriber.address());
    assert_eq!(event.expires_at_ts, state.expires_at_ts);
    println!(
        "subscriptions-lifecycle: cancel {signature} expires_at_ts={} period_start={} pulled={} cancelled_event_ts={}",
        state.expires_at_ts,
        state.current_period_start_ts,
        state.amount_pulled_in_period,
        event.expires_at_ts
    );

    Ok(state.expires_at_ts)
}

pub(crate) async fn expect_duplicate_cancel(
    env: &IntegrationEnv,
    lifecycle: &Lifecycle,
) -> Result<(), Error> {
    let instruction = cancel_instruction(lifecycle, lifecycle.subscriber.address())?;

    expect_rejection(
        env,
        lifecycle.subscription,
        instruction,
        &[env.payer(), &lifecycle.subscriber],
        ALREADY_CANCELLED,
        "duplicate cancel",
    )
    .await
}

pub(crate) async fn expect_stale_resume(
    env: &IntegrationEnv,
    lifecycle: &Lifecycle,
) -> Result<(), Error> {
    let current = subscription_account(env, lifecycle.subscription).await?;
    let instruction = resume_instruction(lifecycle, current.data.expires_at_ts + 1)?;

    expect_rejection(
        env,
        lifecycle.subscription,
        instruction,
        &[env.payer(), &lifecycle.subscriber],
        STALE_APPROVAL,
        "resume with a stale expiry approval",
    )
    .await
}

pub(crate) async fn resume(
    env: &IntegrationEnv,
    lifecycle: &Lifecycle,
    expires_at_ts: i64,
) -> Result<(), Error> {
    let instruction = resume_instruction(lifecycle, expires_at_ts)?;
    let signature = env
        .submit(&[instruction], &[env.payer(), &lifecycle.subscriber])
        .await?;

    let account = subscription_account(env, lifecycle.subscription).await?;
    let state = &account.data;
    assert_eq!(state.expires_at_ts, 0);
    assert_eq!(state.amount_pulled_in_period, 0);
    assert_eq!(state.terms, lifecycle.plan_terms);

    let event = events::lifecycle_event(env, signature).await?;
    let SubscriptionEvent::Resumed(event) = event else {
        panic!("resume must emit SubscriptionResumed, got a different event");
    };
    assert_eq!(event.plan, lifecycle.plan);
    assert_eq!(event.subscriber, lifecycle.subscriber.address());
    assert!(event.resumed_ts > 0);
    assert!(event.resumed_ts >= state.current_period_start_ts);
    assert!(
        event.resumed_ts < expires_at_ts,
        "resume only succeeds before the cancelled period ends"
    );
    println!(
        "subscriptions-lifecycle: resume {signature} expires_at_ts={} period_start={} pulled={} resumed_event_ts={} previous_expiry={expires_at_ts}",
        state.expires_at_ts,
        state.current_period_start_ts,
        state.amount_pulled_in_period,
        event.resumed_ts
    );

    Ok(())
}

async fn expect_rejection(
    env: &IntegrationEnv,
    subscription: Address,
    instruction: Instruction,
    signers: &[&Keypair],
    code: u32,
    label: &str,
) -> Result<(), Error> {
    let before = env.account(subscription).await?;

    let error = match env.submit(&[instruction], signers).await {
        Ok(signature) => {
            return Err(Error::Program(format!(
                "{label} unexpectedly confirmed as {signature}"
            )));
        }
        Err(error) => error,
    };

    expect_custom_error(&error, 0, code);
    let after = env.account(subscription).await?;

    assert_eq!(
        before, after,
        "{label} must not change any subscription account byte"
    );
    println!(
        "subscriptions-lifecycle: {label} rejected with {code} ({:?}) and left the account byte-identical",
        SubscriptionsError::from_code(code)
    );

    Ok(())
}

async fn subscription_account(
    env: &IntegrationEnv,
    address: Address,
) -> Result<solana_kit_core::accounts::Account<SubscriptionDelegation>, Error> {
    let account = env
        .account(address)
        .await?
        .ok_or(Error::Client("the subscription delegation does not exist"))?;

    decode_subscription_delegation_with_program(account, SUBSCRIPTIONS.address(), CODEC_LIMITS)
        .map_err(Error::from)
}

fn cancel_instruction(lifecycle: &Lifecycle, subscriber: Address) -> Result<Instruction, Error> {
    let program = SUBSCRIPTIONS.address();
    let instruction = CancelSubscriptionAccounts {
        subscriber,
        plan_pda: lifecycle.plan,
        subscription_pda: Some(lifecycle.subscription),
        event_authority: Some(lifecycle.event_authority),
        self_program: Some(program),
    }
    .instruction_with_program(program, &CancelSubscription {}, CODEC_LIMITS)
    .map_err(|error| Error::Program(format!("subscriptions cancel: {error}")))?;
    assert_eq!(instruction.data, [CANCEL_DISCRIMINATOR]);

    Ok(instruction)
}

fn resume_instruction(
    lifecycle: &Lifecycle,
    expected_expires_at_ts: i64,
) -> Result<Instruction, Error> {
    let program = SUBSCRIPTIONS.address();
    let instruction = ResumeSubscriptionAccounts {
        subscriber: lifecycle.subscriber.address(),
        plan_pda: lifecycle.plan,
        subscription_pda: Some(lifecycle.subscription),
        subscription_authority: lifecycle.authority,
        event_authority: Some(lifecycle.event_authority),
        self_program: Some(program),
    }
    .instruction_with_program(
        program,
        &ResumeSubscription {
            resume_data: ResumeData {
                expected_expires_at_ts,
            },
        },
        CODEC_LIMITS,
    )
    .map_err(|error| Error::Program(format!("subscriptions resume: {error}")))?;
    assert_eq!(instruction.data.first(), Some(&RESUME_DISCRIMINATOR));
    assert_eq!(instruction.data.len(), RESUME_DATA_LEN);

    Ok(instruction)
}
