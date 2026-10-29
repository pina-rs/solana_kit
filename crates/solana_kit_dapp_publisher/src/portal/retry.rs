//! Only explicit idempotent ingestion creation retries; every losing future is dropped.

use super::{BackendIngestionSession, Client, CreateIngestionSession, Error, PortalService};
use solana_kit_client::{
    rest::{JsonError, Transport},
    subscriptions::Clock,
};
use std::{
    future::{Future, poll_fn},
    pin::pin,
    task::Poll,
};

/// Caller-selected backoff and absolute deadline in the supplied Clock's namespace.
#[derive(Clone, Copy, Debug)]
pub struct IngestionRetry {
    /// Base delay in milliseconds. The source default is 1500; waits are base and twice base.
    pub delay_ms: u64,
    /// Absolute deadline for exchanges, decoding and backoff; deadline wins completion ties.
    pub deadline: u64,
}

impl<T: Transport<PortalService>> Client<T> {
    /// Creates ingestion with at most three attempts and the same input/idempotency key.
    ///
    /// No other operation uses this policy. Dropping the returned future drops
    /// its active exchange or timer; a remote accepted request cannot be undone.
    pub async fn create_ingestion_with_retry<C: Clock>(
        &self,
        request: &CreateIngestionSession,
        clock: &C,
        policy: IngestionRetry,
    ) -> Result<BackendIngestionSession, Error> {
        const ATTEMPTS: u64 = 3;

        if request.idempotency_key.trim().is_empty() {
            return Err(Error::Request(
                "ingestion retries require an idempotency key",
            ));
        }

        policy
            .delay_ms
            .checked_mul(ATTEMPTS - 1)
            .ok_or(Error::Request("ingestion backoff overflow"))?;

        let mut attempt = 0;

        loop {
            let result = until(self.call(request), clock, policy.deadline).await;

            let error = match result {
                Ok(value) => return Ok(value),
                Err(error) => error,
            };

            if attempt == ATTEMPTS - 1 || !retryable(&error) {
                return Err(error);
            }

            let wake = next_wake(clock, policy, attempt)?;

            until(
                async { clock.sleep_until(wake).await.map_err(Error::Clock) },
                clock,
                policy.deadline,
            )
            .await?;
            attempt += 1;
        }
    }
}

/// Backoff is admitted only when the entire delay fits inside the caller's deadline.
fn next_wake<C: Clock>(clock: &C, policy: IngestionRetry, attempt: u64) -> Result<u64, Error> {
    let delay = policy.delay_ms * (attempt + 1);
    let wake = clock
        .now()
        .checked_add(delay)
        .ok_or(Error::Request("ingestion backoff overflow"))?;

    if wake >= policy.deadline {
        return Err(Error::Deadline);
    }

    Ok(wake)
}

/// Host polls and ready response decoding can advance time, so deadline arbitration surrounds them.
async fn until<F: Future<Output = Result<R, Error>>, R, C: Clock>(
    operation: F,
    clock: &C,
    deadline: u64,
) -> Result<R, Error> {
    if clock.now() >= deadline {
        return Err(Error::Deadline);
    }

    let mut operation = pin!(operation);
    let mut timer = pin!(clock.sleep_until(deadline));
    poll_fn(|cx| {
        if clock.now() >= deadline {
            return Poll::Ready(Err(Error::Deadline));
        }

        let timeout = timer.as_mut().poll(cx);

        if clock.now() >= deadline {
            return Poll::Ready(Err(Error::Deadline));
        }

        match timeout {
            Poll::Ready(Ok(())) => return Poll::Ready(Err(Error::Deadline)),
            Poll::Ready(Err(error)) => return Poll::Ready(Err(Error::Clock(error))),
            Poll::Pending => {}
        }

        let result = operation.as_mut().poll(cx);

        if clock.now() >= deadline {
            return Poll::Ready(Err(Error::Deadline));
        }

        result
    })
    .await
}

fn retryable(error: &Error) -> bool {
    const BAD_GATEWAY: u16 = 502;
    const GATEWAY_TIMEOUT: u16 = 504;

    match error {
        Error::Http {
            status: BAD_GATEWAY..=GATEWAY_TIMEOUT,
            ..
        } => true,
        Error::Json(JsonError::Schema(error)) => error.is_syntax() || error.is_eof(),
        Error::Provider {
            message: Some(message),
            ..
        } => {
            let message = message.to_lowercase();
            [
                "gateway timeout",
                "bad gateway",
                "service unavailable",
                "unexpected token <",
            ]
            .iter()
            .any(|part| message.contains(part))
        }
        _ => false,
    }
}
