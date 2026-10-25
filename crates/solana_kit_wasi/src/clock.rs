//! Fallible monotonic subscription deadlines on the caller's existing Reactor.
use crate::{Reactor, reactor::Wait};
use solana_kit_client::subscriptions::{Clock, ClockError};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use wasip2::clocks::monotonic_clock;

const NANOS_PER_MILLISECOND: u64 = 1_000_000;

/// Monotonic milliseconds with no task, executor or separate timer registry.
#[derive(Clone)]
pub struct WasiClock {
    reactor: Reactor,
}

impl WasiClock {
    /// Uses the same readiness owner as the application's sockets and HTTP work.
    pub fn new(reactor: Reactor) -> Self {
        Self { reactor }
    }
}

impl Clock for WasiClock {
    type Sleep<'a> = ClockSleep;

    fn now(&self) -> u64 {
        monotonic_clock::now() / NANOS_PER_MILLISECOND
    }

    fn sleep_until(&self, deadline: u64) -> Self::Sleep<'_> {
        let state = match deadline.checked_mul(NANOS_PER_MILLISECOND) {
            None => SleepState::Immediate(Err(ClockError::DeadlineRange)),

            Some(instant) if instant <= monotonic_clock::now() => SleepState::Immediate(Ok(())),

            Some(instant) => match self
                .reactor
                .wait(monotonic_clock::subscribe_instant(instant), instant)
            {
                Ok(wait) => SleepState::Waiting(wait),
                Err(()) => SleepState::Immediate(Err(ClockError::Capacity)),
            },
        };

        ClockSleep { state }
    }
}

/// One retained host pollable. Drop unregisters it before releasing its resources.
pub struct ClockSleep {
    state: SleepState,
}

enum SleepState {
    Immediate(Result<(), ClockError>),
    Waiting(Wait),
    Complete,
}

impl Future for ClockSleep {
    type Output = Result<(), ClockError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let SleepState::Waiting(wait) = &mut this.state {
            // Both host signals use the requested instant. Wait's deadline result
            // therefore means this clock elapsed, not that admission failed.
            match core::task::ready!(Pin::new(wait).poll(context)) {
                Ok(()) | Err(()) => (),
            }

            this.state = SleepState::Complete;

            return Poll::Ready(Ok(()));
        }

        match core::mem::replace(&mut this.state, SleepState::Complete) {
            SleepState::Immediate(result) => Poll::Ready(result),
            _ => panic!("completed clock sleep polled again"),
        }
    }
}
