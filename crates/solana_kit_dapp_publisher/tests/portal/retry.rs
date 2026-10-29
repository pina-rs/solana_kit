//! Explicit ingestion retry owns one exchange/timer at a time and never retries submissions.

use super::checks::{Outcome, require};
use super::{routes::ingestion, support::*};
use solana_kit_client::subscriptions::ClockError;
use solana_kit_dapp_publisher::portal::*;
use std::{
    boxed::Box,
    future::Future,
    task::{Context, Poll, Waker},
    vec,
};

const DEADLINE: u64 = 10_000;
const BASE_DELAY: u64 = 1500;
const TOTAL_DELAY: u64 = 4500;
const BAD_GATEWAY: u16 = 502;
const SUCCESS: u16 = 200;
const UNAUTHORIZED: u16 = 401;
const UNAVAILABLE: u16 = 503;
const POLICY: IngestionRetry = IngestionRetry {
    delay_ms: BASE_DELAY,
    deadline: DEADLINE,
};

#[cfg_attr(test, test)]
pub fn explicit_ingestion_retries_reuse_exact_input_with_source_delays_and_three_attempts()
-> Outcome {
    let (client, state) = client(vec![
        reply(BAD_GATEWAY, "synthetic gateway"),
        reply(SUCCESS, "<synthetic html>"),
        success(r#"{"id":"ready"}"#),
    ]);
    let clock = TestClock::new(DEADLINE);
    let request = ingestion();
    let result = complete(client.create_ingestion_with_retry(&request, &clock, POLICY))?
        .map_err(|_| "valid transient ingestion retry failed")?;

    require(
        matches!(result.id, solana_kit_client::Presence::Present(Some(id)) if id == "ready"),
        "retried ingestion result changed",
    )?;
    let requests = state.requests.borrow();
    require(requests.len() == 3, "ingestion retry attempt count changed")?;
    let first = requests.first().ok_or("ingestion request missing")?;
    require(
        requests
            .iter()
            .all(|request| request.body.as_deref() == first.body.as_deref()),
        "ingestion retry input changed",
    )?;
    require(
        clock.sleeps.borrow().as_slice()
            == [
                DEADLINE,
                DEADLINE,
                BASE_DELAY,
                DEADLINE,
                DEADLINE,
                TOTAL_DELAY,
                DEADLINE,
            ],
        "ingestion retry delay/deadline schedule changed",
    )?;
    require(
        state.cancelled.get() == 0,
        "completed ingestion retry was cancelled",
    )
}

#[cfg_attr(test, test)]
pub fn permanent_schema_duplicate_budget_and_auth_failures_do_not_retry() -> Outcome {
    for reply in [
        reply(UNAUTHORIZED, "synthetic denied"),
        success(r#"{"id":false}"#),
        success(r#"{"id":"a","id":"b"}"#),
        success(r#"{"_tag":"future","right":{}}"#),
        reply(SUCCESS, &"x".repeat(BUDGETS.bytes + 1)),
    ] {
        let (client, state) = client(vec![reply]);
        let clock = TestClock::new(DEADLINE);
        require(
            complete(client.create_ingestion_with_retry(&ingestion(), &clock, POLICY))?.is_err(),
            "permanent ingestion failure became success",
        )?;
        require(
            state.requests.borrow().len() == 1,
            "permanent ingestion failure retried",
        )?;
    }
    Ok(())
}

#[cfg_attr(test, test)]
pub fn exhausted_transient_errors_and_side_effecting_operations_never_become_success() -> Outcome {
    let (ingestion_client, state) = client(vec![
        reply(UNAVAILABLE, "one"),
        reply(UNAVAILABLE, "two"),
        reply(UNAVAILABLE, "three"),
    ]);
    let clock = TestClock::new(DEADLINE);
    let error =
        complete(ingestion_client.create_ingestion_with_retry(&ingestion(), &clock, POLICY))?
            .err()
            .ok_or("exhausted ingestion became success")?;

    require(
        matches!(error, Error::Http { status:503, body } if body == b"three"),
        "last exhausted ingestion error changed",
    )?;
    require(
        state.requests.borrow().len() == 3,
        "exhausted ingestion attempt count changed",
    )?;

    let request = SubmitSignedTransaction {
        signed_transaction: "explicit".into(),
        publication_session_id: None,
    };
    let (client, state) = client(vec![reply(UNAVAILABLE, "no retry")]);
    require(
        complete(client.call(&request))?.is_err(),
        "submission failure became success",
    )?;
    require(
        state.requests.borrow().len() == 1,
        "signed transaction submission retried",
    )
}

#[cfg_attr(test, test)]
pub fn retry_policy_validation_and_deadlines_stop_admission_and_backoff() -> Outcome {
    let (client, state) = client(vec![reply(UNAVAILABLE, "transient")]);
    let clock = TestClock::new(DEADLINE);
    let mut request = ingestion();
    request.idempotency_key.clear();
    require(
        matches!(
            complete(client.create_ingestion_with_retry(&request, &clock, POLICY))?,
            Err(Error::Request(_))
        ),
        "empty retry key accepted",
    )?;
    require(
        state.requests.borrow().is_empty(),
        "empty retry key reached transport",
    )?;

    let overflow = IngestionRetry {
        delay_ms: u64::MAX,
        ..POLICY
    };
    require(
        matches!(
            complete(client.create_ingestion_with_retry(&ingestion(), &clock, overflow))?,
            Err(Error::Request(_))
        ),
        "overflowing retry policy accepted",
    )?;
    require(
        state.requests.borrow().is_empty(),
        "overflowing retry policy reached transport",
    )?;

    clock.now.set(DEADLINE);
    require(
        matches!(
            complete(client.create_ingestion_with_retry(&ingestion(), &clock, POLICY))?,
            Err(Error::Deadline)
        ),
        "elapsed retry deadline accepted",
    )?;
    require(
        state.requests.borrow().is_empty(),
        "elapsed retry deadline reached transport",
    )?;
    clock.now.set(0);
    let short = IngestionRetry {
        deadline: 1000,
        ..POLICY
    };
    let clock = TestClock::new(short.deadline);
    require(
        matches!(
            complete(client.create_ingestion_with_retry(&ingestion(), &clock, short))?,
            Err(Error::Deadline)
        ),
        "backoff crossing deadline accepted",
    )?;
    require(
        state.requests.borrow().len() == 1,
        "deadline-crossing backoff admitted another exchange",
    )
}

#[cfg_attr(test, test)]
pub fn cancellation_and_clock_failure_drop_owned_attempts_without_future_admission() -> Outcome {
    let (client, state) = client(vec![None]);
    let clock = TestClock::new(DEADLINE);
    let request = ingestion();
    let mut operation = Box::pin(client.create_ingestion_with_retry(&request, &clock, POLICY));

    require(
        matches!(
            operation
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Pending
        ),
        "pending retry completed unexpectedly",
    )?;
    require(
        state.cancelled.get() == 0,
        "live retry exchange already cancelled",
    )?;
    drop(operation);
    require(
        state.cancelled.get() == 1,
        "retry exchange did not cancel on drop",
    )?;
    require(clock.dropped.get() == 1, "retry timer did not drop")?;

    clock.fail.set(true);
    let result = complete(client.create_ingestion_with_retry(&request, &clock, POLICY))?;
    require(
        matches!(result, Err(Error::Clock(ClockError::Capacity))),
        "clock failure was not propagated",
    )?;
    require(
        state.requests.borrow().len() == 1,
        "failed clock admitted another retry exchange",
    )
}

#[cfg_attr(test, test)]
pub fn elapsed_deadline_drops_the_pending_attempt_without_retrying() -> Outcome {
    let (client, state) = client(vec![None]);
    let clock = TestClock::new(DEADLINE);
    let request = ingestion();
    let mut operation = Box::pin(client.create_ingestion_with_retry(&request, &clock, POLICY));
    let mut context = Context::from_waker(Waker::noop());

    require(
        matches!(operation.as_mut().poll(&mut context), Poll::Pending),
        "pending ingestion completed unexpectedly",
    )?;
    require(
        state.requests.borrow().len() == 1,
        "pending ingestion exchange count changed",
    )?;
    clock.now.set(DEADLINE);
    require(
        matches!(
            operation.as_mut().poll(&mut context),
            Poll::Ready(Err(Error::Deadline))
        ),
        "elapsed deadline did not fail the pending ingestion",
    )?;
    require(
        state.cancelled.get() == 1,
        "deadline did not cancel pending ingestion",
    )?;
    require(clock.dropped.get() == 1, "deadline timer did not drop")?;
    require(
        state.requests.borrow().len() == 1,
        "deadline admitted another retry exchange",
    )
}
