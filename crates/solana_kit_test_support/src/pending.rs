//! Manually controlled pending exchanges.

use crate::{
    lock::lock,
    script::{remote_body, success_body},
};
use solana_kit_client::{Reply, TransportError};
use std::{
    fmt,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

pub(crate) struct PendingState {
    request_id: Option<u64>,
    polls: usize,
    wakes: usize,
    // Arc cloning under the lock cannot invoke a caller's RawWaker callback.
    waker: Option<Arc<Waker>>,
    outcome: Option<Result<Reply, TransportError>>,
    completed: bool,
    delivered: bool,
    cancelled: bool,
}

impl fmt::Debug for PendingState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PendingState")
            .field("request_id", &self.request_id)
            .field("polls", &self.polls)
            .field("wakes", &self.wakes)
            .field("has_waker", &self.waker.is_some())
            .field("completed", &self.completed)
            .field("delivered", &self.delivered)
            .field("cancelled", &self.cancelled)
            .finish()
    }
}

/// Observable state of one scripted pending exchange.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PendingStatus {
    /// Correlation ID captured when the exchange starts.
    pub request_id: Option<u64>,
    /// Number of times the exchange future was polled.
    pub polls: usize,
    /// Number of successful wake notifications.
    pub wakes: usize,
    /// Whether a live waker is registered.
    pub has_waker: bool,
    /// Whether a reply or failure has been supplied.
    pub completed: bool,
    /// Whether the future returned the supplied outcome.
    pub delivered: bool,
    /// Whether the future was dropped before delivery.
    pub cancelled: bool,
}

/// Control handle for one scripted pending exchange.
///
/// Completion is accepted once. Dropping the exchange future without delivery
/// marks the exchange cancelled, clears its waker, and releases any retained
/// outcome. The handle never owns a runtime or task.
#[derive(Clone, Debug)]
pub struct PendingExchange {
    state: Arc<Mutex<PendingState>>,
}

impl PendingExchange {
    pub(crate) fn channel() -> (Arc<Mutex<PendingState>>, Self) {
        let state = Arc::new(Mutex::new(PendingState {
            request_id: None,
            polls: 0,
            wakes: 0,
            waker: None,
            outcome: None,
            completed: false,
            delivered: false,
            cancelled: false,
        }));

        (Arc::clone(&state), Self { state })
    }

    /// Returns a snapshot of the observable exchange state.
    #[must_use]
    pub fn status(&self) -> PendingStatus {
        let state = lock(&self.state);

        PendingStatus {
            request_id: state.request_id,
            polls: state.polls,
            wakes: state.wakes,
            has_waker: state.waker.is_some(),
            completed: state.completed,
            delivered: state.delivered,
            cancelled: state.cancelled,
        }
    }

    /// Wakes the registered exchange future once.
    ///
    /// Returns `false` before the first poll, after completion, or after the
    /// future was dropped.
    pub fn wake(&self) -> bool {
        let waker = {
            let mut state = lock(&self.state);

            if state.completed || state.cancelled {
                return false;
            }

            let Some(waker) = state.waker.clone() else {
                return false;
            };
            state.wakes += 1;
            waker
        };
        waker.wake_by_ref();

        true
    }

    /// Delivers an exact reply or typed failure to the exchange future.
    ///
    /// Returns `false` if completion was already supplied or the future was
    /// dropped. A registered waker is notified exactly once.
    pub fn complete(&self, outcome: Result<Reply, TransportError>) -> bool {
        let waker = {
            let mut state = lock(&self.state);

            if state.completed || state.cancelled {
                return false;
            }

            state.outcome = Some(outcome);
            state.completed = true;
            let waker = state.waker.take();
            if waker.is_some() {
                state.wakes += 1;
            }

            waker
        };

        if let Some(waker) = waker {
            waker.wake_by_ref();
        }

        true
    }

    /// Correlates and delivers a successful result using the captured request ID.
    ///
    /// Returns `false` before an exchange starts or when completion is no longer
    /// accepted.
    pub fn complete_success(&self, result: impl Into<Vec<u8>>) -> bool {
        let Some(id) = self.request_id() else {
            return false;
        };

        self.complete(Ok(Reply {
            status: 200,
            body: success_body(id, result.into()),
        }))
    }

    /// Correlates and delivers a remote error without a `data` member.
    ///
    /// Returns `false` before an exchange starts or when completion is no longer
    /// accepted.
    pub fn complete_remote_error(&self, code: i64, message: impl Into<String>) -> bool {
        let Some(id) = self.request_id() else {
            return false;
        };

        self.complete(Ok(Reply {
            status: 200,
            body: remote_body(id, code, message.into(), None),
        }))
    }

    fn request_id(&self) -> Option<u64> {
        lock(&self.state).request_id
    }
}

/// Future owned by one pending exchange.
#[derive(Debug)]
pub struct PendingFuture {
    state: Arc<Mutex<PendingState>>,
}

impl PendingFuture {
    pub(crate) fn new(state: Arc<Mutex<PendingState>>, id: u64) -> Self {
        lock(&state).request_id = Some(id);

        Self { state }
    }
}

impl Future for PendingFuture {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let incoming = Arc::new(context.waker().clone());
        let mut state = lock(&self.state);
        state.polls += 1;

        if let Some(outcome) = state.outcome.take() {
            state.delivered = true;
            let previous = state.waker.take();
            drop(state);
            drop(previous);

            return Poll::Ready(outcome);
        }

        let previous = state.waker.replace(incoming);
        drop(state);
        drop(previous);

        Poll::Pending
    }
}

impl Drop for PendingFuture {
    fn drop(&mut self) {
        let mut state = lock(&self.state);
        let waker = state.waker.take();
        let outcome = if !state.delivered {
            state.cancelled = true;
            state.outcome.take()
        } else {
            None
        };

        drop(state);
        drop((waker, outcome));
    }
}
