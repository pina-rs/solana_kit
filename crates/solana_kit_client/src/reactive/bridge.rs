//! Snapshot observation with one accepted-value slot and retained error priority.
use super::{
    CancellationToken, Cancelled, ListenerLease, ReactiveFailure, ReactiveStreamStore,
    StreamSnapshot, stream::StreamCore,
};
use core::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use futures_core::Stream;
use std::sync::{Arc, Mutex, Weak};

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

/// Terminal bridge failure, retaining a source failure or fallible predicate error.
#[derive(Debug)]
pub enum BridgeFailure<E, PredicateError> {
    /// Store entered an error snapshot.
    Store(Arc<ReactiveFailure<E>>),
    /// The consumer's predicate rejected processing with an error.
    Predicate(PredicateError),
}

type BridgeItem<T, E, PredicateError> = Result<Arc<T>, Arc<BridgeFailure<E, PredicateError>>>;
type FilteredSnapshot<T, E, PredicateError> = (
    Option<Arc<T>>,
    Option<Arc<BridgeFailure<E, PredicateError>>>,
);

struct Buffer<T, E, PredicateError> {
    latest: Option<Arc<T>>,
    failure: Option<Arc<BridgeFailure<E, PredicateError>>>,
    seen: Option<Weak<StreamSnapshot<T, E>>>,
    invocation: Arc<()>,
    waker: Option<Waker>,
    closed: bool,
}

impl<T, E, PredicateError> Buffer<T, E, PredicateError> {
    fn new() -> Self {
        Self {
            latest: None,
            failure: None,
            seen: None,
            invocation: Arc::new(()),
            waker: None,
            closed: false,
        }
    }

    fn begin(&mut self, snapshot: &Arc<StreamSnapshot<T, E>>, invocation: &Arc<()>) -> bool {
        let seen = self
            .seen
            .as_ref()
            .is_some_and(|seen| Weak::ptr_eq(seen, &Arc::downgrade(snapshot)));

        if self.closed || self.failure.is_some() || seen {
            return false;
        }

        self.seen = Some(Arc::downgrade(snapshot));
        self.invocation = invocation.clone();
        true
    }

    fn take(
        &mut self,
        next_waker: &mut Option<Waker>,
    ) -> (Option<BridgeItem<T, E, PredicateError>>, Option<Waker>) {
        let item = self
            .failure
            .as_ref()
            .map(|failure| Err(failure.clone()))
            .or_else(|| self.latest.take().map(Ok));
        let previous = if item.is_some() {
            self.waker.take()
        } else {
            core::mem::replace(&mut self.waker, next_waker.take())
        };

        (item, previous)
    }

    fn publish(
        &mut self,
        invocation: &Arc<()>,
        value: &mut Option<Arc<T>>,
        failure: &mut Option<Arc<BridgeFailure<E, PredicateError>>>,
    ) -> (Option<Arc<T>>, Option<Waker>) {
        if self.closed || self.failure.is_some() || !Arc::ptr_eq(&self.invocation, invocation) {
            return (None, None);
        }

        let previous = if failure.is_some() {
            self.failure = failure.take();
            self.latest.take()
        } else {
            core::mem::replace(&mut self.latest, value.take())
        };

        (previous, self.waker.take())
    }
}

/// A store observer with explicit latest-accepted-value coalescing.
///
/// Errors beat buffered data, even when the store subsequently recovers. Caller
/// cancellation ends cleanly. Drop unsubscribes while parked. This observer never
/// connects/resets/disposes the store; source end and store dispose do not end it.
#[must_use = "poll the observer or drop it to release its listener"]
pub struct StoreStream<T, E, PredicateError = Infallible, L: ?Sized = dyn Fn() + Send + Sync> {
    buffer: Arc<Mutex<Buffer<T, E, PredicateError>>>,
    lease: ListenerLease<L>,
    cancellation: Option<Cancelled<E>>,
    done: bool,
}

impl<T, E, PredicateError, L: ?Sized> StoreStream<T, E, PredicateError, L> {
    fn cancelled(&mut self, cx: &mut Context<'_>) -> bool {
        self.cancellation
            .as_mut()
            .is_some_and(|cancelled| Pin::new(cancelled).poll(cx).is_ready())
    }

    fn close(&mut self) {
        self.lease.unsubscribe();
        let retained = {
            let mut buffer = self.buffer.lock().expect("bridge buffer");
            buffer.closed = true;
            (
                buffer.latest.take(),
                buffer.failure.take(),
                buffer.waker.take(),
            )
        };
        drop(retained);
        self.done = true;
    }
}

impl<T, E, PredicateError, L: ?Sized> Stream for StoreStream<T, E, PredicateError, L> {
    type Item = Result<Arc<T>, Arc<BridgeFailure<E, PredicateError>>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if this.done {
            return Poll::Ready(None);
        }

        if this.cancelled(cx) {
            this.close();

            return Poll::Ready(None);
        }

        let mut next_waker = Some(cx.waker().clone());
        // A custom waker clone may reenter cancellation/store code.
        if this.cancelled(cx) {
            this.close();

            return Poll::Ready(None);
        }

        let (item, previous_waker) = this
            .buffer
            .lock()
            .expect("bridge buffer")
            .take(&mut next_waker);
        drop(previous_waker);

        if matches!(item, Some(Err(_))) {
            this.close();
        }

        item.map_or(Poll::Pending, |item| Poll::Ready(Some(item)))
    }
}

impl<T, E, PredicateError, L: ?Sized> Drop for StoreStream<T, E, PredicateError, L> {
    fn drop(&mut self) {
        self.close();
    }
}

/// Predicate execution is outside the buffer lock because it may synchronously
/// reset/reconnect and recursively notify this same observer.
fn observe<T, E, PredicateError, Fut, D, R, L, G>(
    core: &Weak<StreamCore<T, E, Fut, D, R, L>>,
    buffer: &Weak<Mutex<Buffer<T, E, PredicateError>>>,
    predicate: &G,
) where
    L: Fn() + ?Sized,
    G: Fn(&T) -> Result<bool, PredicateError>,
{
    let (Some(core), Some(buffer)) = (core.upgrade(), buffer.upgrade()) else {
        return;
    };
    let snapshot = core.snapshot();

    observe_captured(&core, &buffer, snapshot, predicate);
}

fn observe_captured<T, E, PredicateError, Fut, D, R, L, G>(
    core: &Arc<StreamCore<T, E, Fut, D, R, L>>,
    buffer: &Arc<Mutex<Buffer<T, E, PredicateError>>>,
    snapshot: Arc<StreamSnapshot<T, E>>,
    predicate: &G,
) where
    L: Fn() + ?Sized,
    G: Fn(&T) -> Result<bool, PredicateError>,
{
    // A predicate may reset/reconnect synchronously. This identity prevents an
    // outer callback from replacing the newer accepted value after it returns.
    let invocation = Arc::new(());

    // Lock order is Core -> buffer, exclusively for Weak/Arc<()> metadata.
    // No buffered value, error, waker or callback is replaced or dropped here.
    let admitted = core.admit_current(&snapshot, || {
        buffer
            .lock()
            .expect("bridge buffer")
            .begin(&snapshot, &invocation)
    });

    if !admitted {
        return;
    }

    let (mut value, mut failure) = filter_snapshot(&snapshot, predicate);

    if value.is_none() && failure.is_none() {
        return;
    }

    let (previous, waker) =
        buffer
            .lock()
            .expect("bridge buffer")
            .publish(&invocation, &mut value, &mut failure);
    drop(previous);

    if let Some(waker) = waker {
        waker.wake();
    }
}

fn filter_snapshot<T, E, PredicateError, G>(
    snapshot: &StreamSnapshot<T, E>,
    predicate: &G,
) -> FilteredSnapshot<T, E, PredicateError>
where
    G: Fn(&T) -> Result<bool, PredicateError>,
{
    match snapshot {
        StreamSnapshot::Loaded { data } => match predicate(data) {
            Ok(true) => (Some(data.clone()), None),
            Ok(false) => (None, None),
            Err(error) => (None, Some(Arc::new(BridgeFailure::Predicate(error)))),
        },

        StreamSnapshot::Error { error, .. } => {
            (None, Some(Arc::new(BridgeFailure::Store(error.clone()))))
        }
        _ => (None, None),
    }
}

impl<T, E, F, Fut, D, R> ReactiveStreamStore<T, E, F, Fut, D, R>
where
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    Fut: Send + 'static,
    D: Send + 'static,
    R: Send + 'static,
{
    /// Observes current and subsequent loaded values with latest-wins buffering.
    pub fn observe(&self, cancellation: Option<CancellationToken<E>>) -> StoreStream<T, E> {
        self.observe_filtered(cancellation, |_| Ok::<_, Infallible>(true))
    }

    /// Evaluates a fallible predicate on every loaded notification, even unpolled.
    pub fn observe_filtered<PredicateError, G>(
        &self,
        cancellation: Option<CancellationToken<E>>,
        predicate: G,
    ) -> StoreStream<T, E, PredicateError>
    where
        PredicateError: Send + Sync + 'static,
        G: Fn(&T) -> Result<bool, PredicateError> + Send + Sync + 'static,
    {
        let buffer = Arc::new(Mutex::new(Buffer::new()));

        let weak_buffer = Arc::downgrade(&buffer);
        let weak_core = Arc::downgrade(&self.core);
        let callback: Arc<dyn Fn() + Send + Sync> =
            Arc::new(move || observe(&weak_core, &weak_buffer, &predicate));

        let lease = self.subscribe(callback.clone());
        callback();

        StoreStream {
            buffer,
            lease,
            cancellation: cancellation.map(|token| token.cancelled()),
            done: false,
        }
    }
}

impl<T, E, F, Fut, D, R> ReactiveStreamStore<T, E, F, Fut, D, R, dyn Fn()>
where
    T: 'static,
    E: 'static,
    Fut: 'static,
    D: 'static,
    R: 'static,
{
    /// Observes values without imposing `Send` on sources, callbacks or payloads.
    pub fn observe(
        &self,
        cancellation: Option<CancellationToken<E>>,
    ) -> StoreStream<T, E, Infallible, dyn Fn()> {
        self.observe_filtered(cancellation, |_| Ok::<_, Infallible>(true))
    }

    /// Applies a local fallible predicate before replacing the accepted-value slot.
    pub fn observe_filtered<PredicateError, G>(
        &self,
        cancellation: Option<CancellationToken<E>>,
        predicate: G,
    ) -> StoreStream<T, E, PredicateError, dyn Fn()>
    where
        PredicateError: 'static,
        G: Fn(&T) -> Result<bool, PredicateError> + 'static,
    {
        let buffer = Arc::new(Mutex::new(Buffer::new()));
        let weak_buffer = Arc::downgrade(&buffer);
        let weak_core = Arc::downgrade(&self.core);
        let callback: Arc<dyn Fn()> =
            Arc::new(move || observe(&weak_core, &weak_buffer, &predicate));

        let lease = self.subscribe(callback.clone());
        callback();

        StoreStream {
            buffer,
            lease,
            cancellation: cancellation.map(|token| token.cancelled()),
            done: false,
        }
    }
}
