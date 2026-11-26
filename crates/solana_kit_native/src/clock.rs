//! Monotonic Tokio deadlines for caller-owned subscription Sessions.
use solana_kit_client::subscriptions::{Clock, ClockError};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::{Instant, Sleep};

/// A Tokio monotonic millisecond clock starting at construction.
///
/// `now` and every [`Clock::sleep_until`] deadline share this origin. The clock
/// creates no task, timer registry or background driver: each sleep owns one
/// Tokio timer and Drop cancels it. Create it inside the runtime that will poll
/// the future; a deadline already at or behind `now` resolves without a timer.
#[derive(Clone, Copy, Debug)]
pub struct TokioClock {
    origin: Instant,
}

impl TokioClock {
    /// Starts a monotonic millisecond namespace at the current Tokio instant.
    #[must_use]
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }
}

impl Default for TokioClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for TokioClock {
    type Sleep<'a> = TokioClockSleep;

    fn now(&self) -> u64 {
        u64::try_from(self.origin.elapsed().as_millis()).unwrap_or(u64::MAX)
    }

    fn sleep_until(&self, deadline: u64) -> TokioClockSleep {
        if deadline <= self.now() {
            return TokioClockSleep {
                state: SleepState::Complete(Some(Ok(()))),
            };
        }

        let Some(instant) = self.origin.checked_add(Duration::from_millis(deadline)) else {
            return TokioClockSleep {
                state: SleepState::Complete(Some(Err(ClockError::DeadlineRange))),
            };
        };

        TokioClockSleep {
            state: SleepState::Waiting(Box::pin(tokio::time::sleep_until(instant))),
        }
    }
}

/// One owned Tokio timer. Polling after completion panics, matching other clocks.
pub struct TokioClockSleep {
    state: SleepState,
}

enum SleepState {
    Waiting(Pin<Box<Sleep>>),
    Complete(Option<Result<(), ClockError>>),
}

impl Future for TokioClockSleep {
    type Output = Result<(), ClockError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match &mut this.state {
            SleepState::Waiting(sleep) => {
                core::task::ready!(sleep.as_mut().poll(context));
                this.state = SleepState::Complete(None);

                Poll::Ready(Ok(()))
            }
            SleepState::Complete(result) => Poll::Ready(
                result
                    .take()
                    .expect("completed Tokio clock sleep polled again"),
            ),
        }
    }
}
