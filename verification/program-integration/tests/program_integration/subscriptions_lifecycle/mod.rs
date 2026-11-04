//! Real Subscriptions lifecycle: subscribe, cancel and resume with events.
//!
//! The pinned `subscriptions-v0.5.0.so` artifact's `subscribe` instruction creates the subscription
//! delegation account, `cancel_subscription` sets `expires_at_ts` to the end of
//! the current period, and `resume_subscription` clears that value after a
//! checked approval. This journey observes those transitions independently
//! through typed account reads, token state, inner-instruction events and
//! confirmed failures.

mod events;
mod journey;
mod setup;

use crate::util;
use solana_kit_program_integration::{Error, IntegrationEnv, fixtures::SUBSCRIPTIONS};

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let lifecycle = setup::create(env).await?;

    setup::assert_token_delegation(env, &lifecycle).await?;
    journey::subscribe(env, &lifecycle).await?;
    journey::expect_resume_before_cancel(env, &lifecycle).await?;
    journey::expect_wrong_subscriber_cancel(env, &lifecycle).await?;
    let expires_at_ts = journey::cancel(env, &lifecycle).await?;
    journey::expect_duplicate_cancel(env, &lifecycle).await?;
    journey::expect_stale_resume(env, &lifecycle).await?;
    journey::resume(env, &lifecycle, expires_at_ts).await?;

    setup::assert_token_delegation(env, &lifecycle).await
}

#[tokio::test]
async fn executes_subscribe_cancel_resume_with_exact_state_and_events() {
    let mut env = util::start_with(&[SUBSCRIPTIONS]).await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
