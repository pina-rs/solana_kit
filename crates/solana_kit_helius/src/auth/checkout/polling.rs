//! Finite checkout observation uses caller-owned monotonic time and sleep futures.

use super::{CheckoutStatusResponse, GetPaymentStatus};
use crate::service::{Error, Service, ServiceClient, ServiceTransport};
use std::{future::Future, ops::ControlFlow};

/// Finite polling policy. It never retries a payment submission.
#[derive(Debug, Clone, Copy)]
pub struct PollPolicy {
    /// Observation time budget in caller-monotonic milliseconds.
    pub timeout_ms: u64,
    /// Minimum wait between nonterminal observations.
    pub interval_ms: u64,
    /// Independent upper bound on status exchanges, including the first.
    pub max_polls: usize,
}

impl Default for PollPolicy {
    fn default() -> Self {
        const TIMEOUT_MS: u64 = 60_000;

        Self {
            timeout_ms: TIMEOUT_MS,
            interval_ms: 1_000,
            max_polls: 60,
        }
    }
}

/// Explicit checkout outcome; timeout never fabricates a provider response.
#[derive(Debug, Clone, PartialEq)]
pub enum PollOutcome {
    /// Provider reported readyToRedirect=true.
    Completed(CheckoutStatusResponse),
    /// Provider reported expiry, or returned HTTP410 with no status payload.
    Expired(Option<CheckoutStatusResponse>),
    /// Provider reported phase=failed.
    Failed(CheckoutStatusResponse),
    /// Caller time or observation budget ended. The last real status is retained.
    Timeout(Option<CheckoutStatusResponse>),
}

/// Service, clock and policy failures stay distinct from checkout outcomes.
#[derive(Debug)]
pub enum PollError<E> {
    /// Invalid interval, observation count, overflowing deadline or decreasing time.
    Policy,
    /// Actual service exchange failed.
    Service(Error),
    /// Caller-owned timer failed.
    Clock(E),
}

impl<E: core::fmt::Display> core::fmt::Display for PollError<E> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Policy => formatter.write_str("invalid Helius checkout polling policy or clock"),
            Self::Service(error) => error.fmt(formatter),
            Self::Clock(error) => write!(formatter, "Helius checkout timer failed: {error}"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for PollError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Service(error) => Some(error),
            Self::Clock(error) => Some(error),
            Self::Policy => None,
        }
    }
}

/// Polls status with explicit time/sleep functions and no hidden runtime or task.
///
/// Transport deadlines bound individual exchanges. The observation window is
/// checked between exchanges. Local Rc captures remain possible; concrete native
/// captures retain Send when their types do.
pub async fn poll_until_terminal<T, N, S, F, E>(
    client: &ServiceClient<T>,
    payment_intent_id: String,
    policy: PollPolicy,
    mut now: N,
    mut sleep_until: S,
) -> Result<PollOutcome, PollError<E>>
where
    T: ServiceTransport<Service>,
    N: FnMut() -> u64,
    S: FnMut(u64) -> F,
    F: Future<Output = Result<(), E>>,
{
    let mut clock = PollClock::new(policy, now())?;
    let mut last = None;

    for _ in 0..policy.max_polls {
        if !clock.advance(now())? {
            return Ok(PollOutcome::Timeout(last));
        }

        let reply = client
            .call(GetPaymentStatus {
                payment_intent_id: payment_intent_id.clone(),
            })
            .await;

        match observation(reply)? {
            ControlFlow::Break(outcome) => return Ok(outcome),
            ControlFlow::Continue(status) => last = Some(status),
        }

        if !clock.advance(now())? {
            return Ok(PollOutcome::Timeout(last));
        }

        sleep_until(clock.wake(policy.interval_ms))
            .await
            .map_err(PollError::Clock)?;
    }

    Ok(PollOutcome::Timeout(last))
}

/// A monotonic observation window shared by pre- and post-exchange checks.
struct PollClock {
    previous: u64,
    deadline: u64,
}

impl PollClock {
    fn new<E>(policy: PollPolicy, start: u64) -> Result<Self, PollError<E>> {
        if policy.interval_ms == 0 || policy.max_polls == 0 {
            return Err(PollError::Policy);
        }

        let deadline = start
            .checked_add(policy.timeout_ms)
            .ok_or(PollError::Policy)?;

        Ok(Self {
            previous: start,
            deadline,
        })
    }

    fn advance<E>(&mut self, tick: u64) -> Result<bool, PollError<E>> {
        if tick < self.previous {
            return Err(PollError::Policy);
        }

        self.previous = tick;
        Ok(tick < self.deadline)
    }

    fn wake(&self, interval: u64) -> u64 {
        self.previous.saturating_add(interval).min(self.deadline)
    }
}

fn observation<E>(
    reply: Result<CheckoutStatusResponse, Error>,
) -> Result<ControlFlow<PollOutcome, CheckoutStatusResponse>, PollError<E>> {
    const GONE: u16 = 410;
    let status = match reply {
        Ok(status) => status,
        Err(Error::Http { status: GONE, .. }) => {
            return Ok(ControlFlow::Break(PollOutcome::Expired(None)));
        }
        Err(error) => return Err(PollError::Service(error)),
    };

    Ok(match (status.ready_to_redirect, status.phase.as_str()) {
        (true, _) => ControlFlow::Break(PollOutcome::Completed(status)),
        (_, "expired") => ControlFlow::Break(PollOutcome::Expired(Some(status))),
        (_, "failed") => ControlFlow::Break(PollOutcome::Failed(status)),
        _ => ControlFlow::Continue(status),
    })
}
