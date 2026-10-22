//! Borrowed futures retain concrete native Send and local Rc capabilities.
use super::{Confirmation, Event, Failure};
use alloc::boxed::Box;
use core::{
    fmt,
    future::{Future, poll_fn},
    pin::Pin,
    task::Poll,
};

/// Caller-owned observations with explicit bounded cleanup.
///
/// Implementations register subscription routing before issuing reconciliation
/// reads, preserve connection generations and validate nonce account state.
/// Epoch and nonce reads must use the requested confirmation commitment; a
/// processed observation cannot establish expiry at confirmed or finalized.
/// `next` must not lose an event when its future is dropped. `close` must stop
/// all watchers and complete within an adapter-owned deadline, reporting false
/// unsubscribe acknowledgements and transport errors. Drop cancels local work;
/// it cannot promise awaited remote cleanup. No method starts a hidden executor.
pub trait Observer {
    /// One cancellable receive, preserving concrete auto traits.
    type Next<'a>: Future<Output = Result<Event, Failure>> + 'a
    where
        Self: 'a;
    /// Explicit cleanup, preserving concrete auto traits.
    type Close<'a>: Future<Output = Result<(), Failure>> + 'a
    where
        Self: 'a;
    /// Receives one event, or reports end/failure.
    fn next(&mut self) -> Self::Next<'_>;
    /// Stops losers and awaits bounded cleanup of owned resources.
    fn close(&mut self) -> Self::Close<'_>;
}

/// A strategy result or cleanup failure retaining the original outcome.
#[derive(Debug)]
pub enum WaitError {
    /// Strategy/observation failed and cleanup succeeded.
    Outcome(Failure),
    /// Cleanup failed; a prior successful result is never returned as success.
    Cleanup {
        /// Original race outcome, retained even when cleanup also failed.
        outcome: Result<(), Failure>,
        /// Adapter's exact cleanup failure.
        source: Box<Failure>,
    },
}
impl fmt::Display for WaitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "confirmation wait failed: {self:?}")
    }
}
impl core::error::Error for WaitError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Outcome(error) => Some(error),
            Self::Cleanup { source, .. } => Some(source.as_ref()),
        }
    }
}

/// Waits without an executor, clock, pump, reconnect or universal Send bound.
///
/// Cancellation has deterministic priority over simultaneously ready events.
/// Observation order determines competing event outcomes. The losing receive is
/// dropped before `close` is awaited. Dropping this function drops its observer;
/// explicit remote cleanup is guaranteed only when this function completes.
pub async fn wait<O: Observer, C: Future<Output = ()>>(
    mut confirmation: Confirmation,
    mut observer: O,
    cancellation: C,
) -> Result<(), WaitError> {
    let mut cancellation = core::pin::pin!(cancellation);

    let outcome = loop {
        if confirmation.is_settled() {
            break Err(Failure::AlreadySettled);
        }

        let event = receive(observer.next(), cancellation.as_mut()).await;

        let event = match event {
            Ok(event) => event,
            Err(error) => break Err(error),
        };

        if let Some(outcome) = confirmation.observe(event) {
            break outcome;
        }

        // A ready stream of irrelevant events must yield so a local producer
        // can deliver cancellation; the caller still owns all I/O and polling.
        yield_once().await;
    };

    if let Err(source) = observer.close().await {
        return Err(WaitError::Cleanup {
            outcome,
            source: Box::new(source),
        });
    }

    outcome.map_err(WaitError::Outcome)
}

async fn receive<C, F>(future: F, mut cancellation: Pin<&mut C>) -> Result<Event, Failure>
where
    C: Future<Output = ()>,
    F: Future<Output = Result<Event, Failure>>,
{
    let mut future = core::pin::pin!(future);
    poll_fn(|cx| {
        if cancellation.as_mut().poll(cx).is_ready() {
            return Poll::Ready(Err(Failure::Cancelled));
        }

        future.as_mut().poll(cx)
    })
    .await
}

async fn yield_once() {
    let mut yielded = false;
    poll_fn(|cx| {
        if yielded {
            return Poll::Ready(());
        }

        yielded = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    })
    .await;
}
