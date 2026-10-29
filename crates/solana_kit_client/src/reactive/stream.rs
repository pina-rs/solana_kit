//! Connection windows with recoverable errors and explicit driver ownership.
use super::{
    CancellationReason, CancellationToken, Cancelled, ListenerLease, ReactiveFailure,
    shared::{Core, Operation},
};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_core::Stream;
use std::sync::{Arc, Weak};

/// One connection's data and optional-error lanes.
///
/// Flatten errors from an error-channel adapter into `Some(error)`. `None` items
/// represent Dart's ignored null error values; end of stream remains distinct.
pub struct ReactiveStreamConnection<D, R> {
    /// Data values or recoverable data-lane failures.
    pub data: D,
    /// Recoverable errors; null-equivalent items are ignored.
    pub errors: R,
}

/// Complete stream-store lifecycle, retaining cached fields while reconnecting.
#[derive(Debug)]
pub enum StreamSnapshot<T, E> {
    /// Not connected, or reset.
    Idle,
    /// Connecting; previous data/error remain visible.
    Loading {
        /// Last received data.
        data: Option<Arc<T>>,
        /// Last received error.
        error: Option<Arc<ReactiveFailure<E>>>,
    },
    /// Latest data, with no active error.
    Loaded {
        /// Received value, including a nullable `T` if appropriate.
        data: Arc<T>,
    },
    /// First error since the last loaded value or connect.
    Error {
        /// Last received data.
        data: Option<Arc<T>>,
        /// Original source/cancellation failure.
        error: Arc<ReactiveFailure<E>>,
    },
}

impl<T, E> StreamSnapshot<T, E> {
    fn starting(&self, failure: Option<&Arc<ReactiveFailure<E>>>) -> (Self, bool) {
        let data = self.data().cloned();

        let snapshot = match failure {
            Some(error) => Self::Error {
                data,
                error: error.clone(),
            },
            None => Self::Loading {
                data,
                error: self.error().cloned(),
            },
        };

        (snapshot, true)
    }

    /// Latest data, also retained through refresh and failure.
    pub fn data(&self) -> Option<&Arc<T>> {
        match self {
            Self::Idle => None,
            Self::Loading { data, .. } | Self::Error { data, .. } => data.as_ref(),
            Self::Loaded { data } => Some(data),
        }
    }

    /// Latest error, cleared when data next arrives or reset runs.
    pub fn error(&self) -> Option<&Arc<ReactiveFailure<E>>> {
        match self {
            Self::Loading { error, .. } => error.as_ref(),
            Self::Error { error, .. } => Some(error),
            _ => None,
        }
    }
}

pub(super) enum Work<Fut, D, R> {
    Opening(Pin<Box<Fut>>),
    Connected(Connection<D, R>),
}

pub(super) struct Connection<D, R> {
    data: Pin<Box<D>>,
    errors: Pin<Box<R>>,
    data_done: bool,
    errors_done: bool,
}

impl<D, R> Connection<D, R> {
    fn new(connection: ReactiveStreamConnection<D, R>) -> Self {
        Self {
            data: Box::pin(connection.data),
            errors: Box::pin(connection.errors),
            data_done: false,
            errors_done: false,
        }
    }

    fn finished(&self) -> bool {
        self.data_done && self.errors_done
    }
}

impl<T, E, D: Stream<Item = Result<T, E>>, R: Stream<Item = Option<E>>> Connection<D, R> {
    fn poll_data(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<T, E>>> {
        if self.data_done {
            return Poll::Pending;
        }

        let outcome = self.data.as_mut().poll_next(cx);
        self.data_done = matches!(outcome, Poll::Ready(None));
        outcome
    }

    fn poll_error(&mut self, cx: &mut Context<'_>) -> Poll<Option<Option<E>>> {
        if self.errors_done {
            return Poll::Pending;
        }

        let outcome = self.errors.as_mut().poll_next(cx);
        self.errors_done = matches!(outcome, Poll::Ready(None));
        outcome
    }
}

pub(super) type StreamCore<T, E, Fut, D, R, L> = Core<StreamSnapshot<T, E>, Work<Fut, D, R>, E, L>;

/// Caller-driven streaming store with a fresh concrete factory per connection.
pub struct ReactiveStreamStore<T, E, F, Fut, D, R, L: ?Sized = dyn Fn() + Send + Sync> {
    factory: Arc<F>,
    pub(super) core: Arc<StreamCore<T, E, Fut, D, R, L>>,
}

/// Stream store whose listeners and source work may retain local `Rc` values.
pub type LocalReactiveStreamStore<T, E, F, Fut, D, R> =
    ReactiveStreamStore<T, E, F, Fut, D, R, dyn Fn()>;

impl<T, E, F, Fut, D, R> ReactiveStreamStore<T, E, F, Fut, D, R>
where
    F: Fn(CancellationToken<E>) -> Fut,
    Fut: Future<Output = Result<ReactiveStreamConnection<D, R>, E>>,
    D: Stream<Item = Result<T, E>>,
    R: Stream<Item = Option<E>>,
{
    /// Creates an idle store with shared listener callbacks.
    pub fn new(factory: F) -> Self {
        Self::create(factory)
    }
}

impl<T, E, F, Fut, D, R> LocalReactiveStreamStore<T, E, F, Fut, D, R>
where
    F: Fn(CancellationToken<E>) -> Fut,
    Fut: Future<Output = Result<ReactiveStreamConnection<D, R>, E>>,
    D: Stream<Item = Result<T, E>>,
    R: Stream<Item = Option<E>>,
{
    /// Creates an idle store accepting local listener captures.
    pub fn new_local(factory: F) -> Self {
        Self::create(factory)
    }

    /// Registers a local callback without requiring shared-thread ownership.
    pub fn subscribe_local<G>(&self, callback: G) -> ListenerLease<dyn Fn()>
    where
        G: Fn() + 'static,
    {
        self.subscribe(Arc::new(callback))
    }
}

impl<T, E, F, Fut, D, R, L: Fn() + ?Sized> ReactiveStreamStore<T, E, F, Fut, D, R, L> {
    fn create(factory: F) -> Self {
        Self {
            factory: Arc::new(factory),
            core: Arc::new(Core::new(StreamSnapshot::Idle)),
        }
    }

    /// Reads immutable state without cloning user data/errors.
    pub fn state(&self) -> Arc<StreamSnapshot<T, E>> {
        self.core.snapshot()
    }

    /// Registers an independent observer owned by the returned lease.
    pub fn subscribe(&self, callback: Arc<L>) -> ListenerLease<L> {
        self.core.subscribe(callback)
    }

    /// Cancels current work and publishes idle, even if already idle.
    pub fn reset(&self) {
        self.core.reset(StreamSnapshot::Idle, |_| true);
    }

    /// Cancels the connection and clears listeners, preserving readable state.
    pub fn dispose(&self) {
        self.core.dispose();
    }
}

impl<T, E, F, Fut, D, R, L> ReactiveStreamStore<T, E, F, Fut, D, R, L>
where
    F: Fn(CancellationToken<E>) -> Fut,
    Fut: Future<Output = Result<ReactiveStreamConnection<D, R>, E>>,
    D: Stream<Item = Result<T, E>>,
    R: Stream<Item = Option<E>>,
    L: Fn() + ?Sized,
{
    /// Eagerly publishes loading and returns the connection's sole driver.
    pub fn connect(&self) -> StreamDriver<T, E, Fut, D, R, L> {
        self.start(None)
    }

    /// Opens a connection with additional caller-owned cancellation.
    pub fn connect_with_signal(
        &self,
        caller: CancellationToken<E>,
    ) -> StreamDriver<T, E, Fut, D, R, L> {
        self.start(Some(caller))
    }

    fn start(&self, caller: Option<CancellationToken<E>>) -> StreamDriver<T, E, Fut, D, R, L> {
        let operation = Arc::new(Operation::new(caller));
        let immediate = operation
            .signal
            .reason()
            .map(|reason| Arc::new(ReactiveFailure::Cancelled(reason)));
        let previous = self
            .core
            .begin(&operation, |state| state.starting(immediate.as_ref()));
        let disposed = previous.is_none();
        let immediate = disposed
            .then(|| Arc::new(ReactiveFailure::Disposed))
            .or(immediate);
        let driver = StreamDriver {
            cancellation: operation.signal.cancelled(),
            immediate,
            operation,
            core: Arc::downgrade(&self.core),
            done: false,
        };

        if !disposed
            && self.core.current(&driver.operation.identity)
            && !driver.operation.signal.is_cancelled()
        {
            let future = (self.factory)(driver.operation.signal.clone());
            driver.operation.restore(Work::Opening(Box::pin(future)));
        }

        driver
    }
}

impl<T, E, F, Fut, D, R, L: ?Sized> Clone for ReactiveStreamStore<T, E, F, Fut, D, R, L> {
    fn clone(&self) -> Self {
        Self {
            factory: self.factory.clone(),
            core: self.core.clone(),
        }
    }
}

/// Sole connection poll owner; Drop destroys its factory future and both lanes.
///
/// Each poll reads at most one item per lane, data before errors when both are
/// ready. Errors are recoverable. Ending both lanes preserves the last snapshot.
#[must_use = "poll or explicitly spawn the connection; dropping it cancels work"]
pub struct StreamDriver<T, E, Fut, D, R, L: Fn() + ?Sized = dyn Fn() + Send + Sync> {
    operation: Arc<Operation<Work<Fut, D, R>, E>>,
    core: Weak<StreamCore<T, E, Fut, D, R, L>>,
    cancellation: Cancelled<E>,
    immediate: Option<Arc<ReactiveFailure<E>>>,
    done: bool,
}

impl<T, E, Fut, D, R, L: Fn() + ?Sized> StreamDriver<T, E, Fut, D, R, L> {
    fn error(&self, failure: Arc<ReactiveFailure<E>>, first_only: bool) {
        let Some(core) = self.core.upgrade() else {
            return;
        };
        let data = core.snapshot().data().cloned();
        let accept: fn(&StreamSnapshot<T, E>) -> bool = if first_only {
            |state| !matches!(state, StreamSnapshot::Error { .. })
        } else {
            |_| true
        };

        core.commit(
            &self.operation.identity,
            StreamSnapshot::Error {
                data,
                error: failure,
            },
            accept,
        );
    }

    fn cancelled(&mut self, reason: Arc<CancellationReason<E>>) -> Arc<ReactiveFailure<E>> {
        let failure = Arc::new(ReactiveFailure::Cancelled(reason.clone()));
        self.operation.cancel(reason);
        self.error(failure.clone(), false);
        self.done = true;
        failure
    }
}

impl<T, E, Fut, D, R, L> StreamDriver<T, E, Fut, D, R, L>
where
    Fut: Future<Output = Result<ReactiveStreamConnection<D, R>, E>>,
    D: Stream<Item = Result<T, E>>,
    R: Stream<Item = Option<E>>,
    L: Fn() + ?Sized,
{
    fn stopped(&mut self, cx: &mut Context<'_>) -> Option<Result<(), Arc<ReactiveFailure<E>>>> {
        if let Some(error) = self.immediate.take() {
            self.done = true;

            return Some(Err(error));
        }

        match Pin::new(&mut self.cancellation).poll(cx) {
            Poll::Ready(reason) => Some(Err(self.cancelled(reason))),
            Poll::Pending => None,
        }
    }

    fn pending(
        &mut self,
        work: Work<Fut, D, R>,
        progressed: bool,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Arc<ReactiveFailure<E>>>> {
        self.operation.restore(work);

        if let Some(reason) = self.operation.signal.reason() {
            return Poll::Ready(Err(self.cancelled(reason)));
        }

        if progressed {
            cx.waker().wake_by_ref();
        }

        Poll::Pending
    }

    fn opened(
        &mut self,
        result: Result<ReactiveStreamConnection<D, R>, E>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Arc<ReactiveFailure<E>>>> {
        if let Some(reason) = self.operation.signal.reason() {
            return Poll::Ready(Err(self.cancelled(reason)));
        }

        match result {
            Ok(connection) => self.pending(Work::Connected(Connection::new(connection)), true, cx),
            Err(error) => {
                let error = Arc::new(ReactiveFailure::Source(error));
                self.error(error.clone(), false);
                self.done = true;
                Poll::Ready(Err(error))
            }
        }
    }

    fn poll_open(
        &mut self,
        mut future: Pin<Box<Fut>>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Arc<ReactiveFailure<E>>>> {
        match future.as_mut().poll(cx) {
            Poll::Pending => self.pending(Work::Opening(future), false, cx),
            Poll::Ready(result) => {
                drop(future);
                self.opened(result, cx)
            }
        }
    }

    fn loaded(&self, value: T) {
        if let Some(core) = self.core.upgrade() {
            core.commit(
                &self.operation.identity,
                StreamSnapshot::Loaded {
                    data: Arc::new(value),
                },
                |_| true,
            );
        }
    }

    fn data(&self, outcome: Poll<Option<Result<T, E>>>) -> bool {
        match outcome {
            Poll::Ready(Some(Ok(value))) => {
                self.loaded(value);
                true
            }
            Poll::Ready(Some(Err(error))) => {
                self.error(Arc::new(ReactiveFailure::Source(error)), true);
                true
            }
            _ => false,
        }
    }

    fn error_item(&self, outcome: Poll<Option<Option<E>>>) -> bool {
        match outcome {
            Poll::Ready(Some(Some(error))) => {
                self.error(Arc::new(ReactiveFailure::Source(error)), true);
                true
            }
            Poll::Ready(Some(None)) => true,
            _ => false,
        }
    }

    /// Every user-code boundary rechecks cancellation because a source poll or
    /// listener can synchronously reset/replace this connection window.
    fn poll_connected(
        &mut self,
        mut connection: Connection<D, R>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Arc<ReactiveFailure<E>>>> {
        // Source polls and listeners may cancel synchronously; every boundary
        // must reject stale ready events before advancing or restoring work.
        let data = connection.poll_data(cx);

        if let Some(reason) = self.operation.signal.reason() {
            drop(connection);

            return Poll::Ready(Err(self.cancelled(reason)));
        }

        let data_progressed = self.data(data);

        if let Some(reason) = self.operation.signal.reason() {
            drop(connection);

            return Poll::Ready(Err(self.cancelled(reason)));
        }

        let error = connection.poll_error(cx);

        if let Some(reason) = self.operation.signal.reason() {
            drop(connection);

            return Poll::Ready(Err(self.cancelled(reason)));
        }

        let error_progressed = self.error_item(error);

        if !connection.finished() {
            return self.pending(
                Work::Connected(connection),
                data_progressed || error_progressed,
                cx,
            );
        }

        drop(connection);

        if let Some(reason) = self.operation.signal.reason() {
            return Poll::Ready(Err(self.cancelled(reason)));
        }

        self.done = true;
        Poll::Ready(Ok(()))
    }
}

impl<T, E, Fut, D, R, L> Future for StreamDriver<T, E, Fut, D, R, L>
where
    Fut: Future<Output = Result<ReactiveStreamConnection<D, R>, E>>,
    D: Stream<Item = Result<T, E>>,
    R: Stream<Item = Option<E>>,
    L: Fn() + ?Sized,
{
    type Output = Result<(), Arc<ReactiveFailure<E>>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        assert!(!this.done, "completed connection polled again");

        if let Some(result) = this.stopped(cx) {
            return Poll::Ready(result);
        }

        match this.operation.take() {
            Some(Work::Opening(future)) => this.poll_open(future, cx),
            Some(Work::Connected(connection)) => this.poll_connected(connection, cx),
            None => Poll::Pending,
        }
    }
}

impl<T, E, Fut, D, R, L: Fn() + ?Sized> Drop for StreamDriver<T, E, Fut, D, R, L> {
    fn drop(&mut self) {
        if !self.done {
            let reason = self
                .operation
                .signal
                .reason()
                .unwrap_or_else(|| Arc::new(CancellationReason::Dropped));
            self.cancelled(reason);
        }
    }
}
