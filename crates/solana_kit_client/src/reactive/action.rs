//! Eager dispatch with caller-owned future polling and stale-result suppression.
use super::{
    CancellationReason, CancellationToken, Cancelled, ListenerLease, ReactiveFailure,
    shared::{Core, Operation},
};
use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use std::sync::{Arc, Weak};

/// An action's complete observable lifecycle, including cached refresh fields.
#[derive(Debug)]
pub enum ActionSnapshot<T, E> {
    /// No dispatch has run since construction/reset.
    Idle,
    /// Args dispatch is pending; prior data and error remain visible.
    Running {
        /// Last successful result.
        result: Option<Arc<T>>,
        /// Last failure, retained until success.
        error: Option<Arc<ReactiveFailure<E>>>,
    },
    /// Current dispatch completed successfully.
    Success {
        /// The current result.
        result: Arc<T>,
    },
    /// Current dispatch failed while retaining its preceding result.
    Error {
        /// Last successful result.
        result: Option<Arc<T>>,
        /// Original failure or caller cancellation.
        error: Arc<ReactiveFailure<E>>,
    },
}

impl<T, E> ActionSnapshot<T, E> {
    fn starting(&self, failure: Option<&Arc<ReactiveFailure<E>>>) -> (Self, bool) {
        let result = self.result().cloned();

        if let Some(error) = failure {
            return (
                Self::Error {
                    result,
                    error: error.clone(),
                },
                true,
            );
        }

        let next = Self::Running {
            result,
            error: self.error().cloned(),
        };
        (next, !matches!(self, Self::Running { .. }))
    }

    /// Last successful result, including during refresh/failure.
    pub fn result(&self) -> Option<&Arc<T>> {
        match self {
            Self::Idle => None,
            Self::Running { result, .. } | Self::Error { result, .. } => result.as_ref(),
            Self::Success { result } => Some(result),
        }
    }

    /// Last error, cleared by reset or successful completion.
    pub fn error(&self) -> Option<&Arc<ReactiveFailure<E>>> {
        match self {
            Self::Running { error, .. } => error.as_ref(),
            Self::Error { error, .. } => Some(error),
            _ => None,
        }
    }
}

type ActionCore<T, E, Fut, L> = Core<ActionSnapshot<T, E>, Pin<Box<Fut>>, E, L>;

/// Args store with an inferred concrete action factory and future.
///
/// No `Send` bound is imposed on actions. Choose the local alias for callbacks
/// capturing `Rc`/`RefCell`; native auto traits follow all actual stored inputs.
pub struct ReactiveActionStore<Args, T, E, F, Fut, L: ?Sized = dyn Fn() + Send + Sync> {
    factory: Arc<F>,
    core: Arc<ActionCore<T, E, Fut, L>>,
    arguments: PhantomData<fn(Args)>,
}

/// Action store permitting local listener captures as well as local futures.
pub type LocalReactiveActionStore<Args, T, E, F, Fut> =
    ReactiveActionStore<Args, T, E, F, Fut, dyn Fn()>;

impl<Args, T, E, F, Fut> ReactiveActionStore<Args, T, E, F, Fut>
where
    F: Fn(CancellationToken<E>, Args) -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    /// Creates an idle store with shared listener callbacks.
    pub fn new(factory: F) -> Self {
        Self::create(factory)
    }
}

impl<Args, T, E, F, Fut> LocalReactiveActionStore<Args, T, E, F, Fut>
where
    F: Fn(CancellationToken<E>, Args) -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    /// Creates an idle store whose callbacks may capture local state.
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

impl<Args, T, E, F, Fut, L: Fn() + ?Sized> ReactiveActionStore<Args, T, E, F, Fut, L> {
    fn create(factory: F) -> Self {
        Self {
            factory: Arc::new(factory),
            core: Arc::new(Core::new(ActionSnapshot::Idle)),
            arguments: PhantomData,
        }
    }

    /// Reads an immutable shared snapshot without cloning user values.
    pub fn state(&self) -> Arc<ActionSnapshot<T, E>> {
        self.core.snapshot()
    }

    /// Registers a callback until the returned lease is dropped/unsubscribed.
    pub fn subscribe(&self, callback: Arc<L>) -> ListenerLease<L> {
        self.core.subscribe(callback)
    }

    /// Cancels current work and clears cached state; idle reset is quiet.
    pub fn reset(&self) {
        self.core.reset(ActionSnapshot::Idle, |state| {
            !matches!(state, ActionSnapshot::Idle)
        });
    }

    /// Permanently cancels work and clears listeners, preserving readable state.
    pub fn dispose(&self) {
        self.core.dispose();
    }
}

impl<Args, T, E, F, Fut, L> ReactiveActionStore<Args, T, E, F, Fut, L>
where
    F: Fn(CancellationToken<E>, Args) -> Fut,
    Fut: Future<Output = Result<T, E>>,
    L: Fn() + ?Sized,
{
    /// Eagerly starts dispatch; the returned future owns all ongoing work.
    pub fn dispatch(&self, args: Args) -> ActionDispatch<T, E, Fut, L> {
        self.start(args, None)
    }

    /// Dispatches with additional caller cancellation scoped to this operation.
    pub fn dispatch_with_signal(
        &self,
        args: Args,
        caller: CancellationToken<E>,
    ) -> ActionDispatch<T, E, Fut, L> {
        self.start(args, Some(caller))
    }

    fn start(
        &self,
        args: Args,
        caller: Option<CancellationToken<E>>,
    ) -> ActionDispatch<T, E, Fut, L> {
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
        let driver = ActionDispatch {
            cancellation: operation.signal.cancelled(),
            previous_result: previous.as_ref().and_then(|state| state.result().cloned()),
            immediate,
            operation,
            core: Arc::downgrade(&self.core),
            done: false,
        };

        // Harden Dart's action path to match its stream path: a running listener
        // that resets/replaces the store prevents unnecessary source I/O.
        if !disposed
            && self.core.current(&driver.operation.identity)
            && !driver.operation.signal.is_cancelled()
        {
            let future = (self.factory)(driver.operation.signal.clone(), args);
            driver.operation.restore(Box::pin(future));
        }

        driver
    }
}

impl<Args, T, E, F, Fut, L: ?Sized> Clone for ReactiveActionStore<Args, T, E, F, Fut, L> {
    fn clone(&self) -> Self {
        Self {
            factory: self.factory.clone(),
            core: self.core.clone(),
            arguments: PhantomData,
        }
    }
}

/// Sole owner of dispatch polling; dropping it cancels its actual owned future.
#[must_use = "poll or explicitly spawn the dispatch; dropping it cancels work"]
pub struct ActionDispatch<T, E, Fut, L: Fn() + ?Sized = dyn Fn() + Send + Sync> {
    operation: Arc<Operation<Pin<Box<Fut>>, E>>,
    core: Weak<ActionCore<T, E, Fut, L>>,
    cancellation: Cancelled<E>,
    previous_result: Option<Arc<T>>,
    immediate: Option<Arc<ReactiveFailure<E>>>,
    done: bool,
}

impl<T, E, Fut, L: Fn() + ?Sized> ActionDispatch<T, E, Fut, L> {
    fn cancelled(&mut self, reason: Arc<CancellationReason<E>>) -> Arc<ReactiveFailure<E>> {
        let failure = Arc::new(ReactiveFailure::Cancelled(reason.clone()));
        self.operation.cancel(reason);

        if let Some(core) = self.core.upgrade() {
            core.commit(
                &self.operation.identity,
                ActionSnapshot::Error {
                    result: self.previous_result.clone(),
                    error: failure.clone(),
                },
                |_| true,
            );
        }

        self.done = true;
        failure
    }
}

impl<T, E, Fut, L> ActionDispatch<T, E, Fut, L>
where
    Fut: Future<Output = Result<T, E>>,
    L: Fn() + ?Sized,
{
    fn stopped(&mut self, cx: &mut Context<'_>) -> Option<Result<Arc<T>, Arc<ReactiveFailure<E>>>> {
        if let Some(error) = self.immediate.take() {
            self.done = true;

            return Some(Err(error));
        }

        match Pin::new(&mut self.cancellation).poll(cx) {
            Poll::Ready(reason) => Some(Err(self.cancelled(reason))),
            Poll::Pending => None,
        }
    }

    /// Commit is the completion boundary because its listeners may immediately
    /// replace the dispatch after its successful result has already published.
    fn complete(&mut self, result: Result<T, E>) -> Poll<Result<Arc<T>, Arc<ReactiveFailure<E>>>> {
        if let Some(reason) = self.operation.signal.reason() {
            return Poll::Ready(Err(self.cancelled(reason)));
        }

        let result = result
            .map(Arc::new)
            .map_err(|error| Arc::new(ReactiveFailure::Source(error)));

        let committed = if let Some(core) = self.core.upgrade() {
            let snapshot = match &result {
                Ok(result) => ActionSnapshot::Success {
                    result: result.clone(),
                },
                Err(error) => ActionSnapshot::Error {
                    result: self.previous_result.clone(),
                    error: error.clone(),
                },
            };

            core.commit(&self.operation.identity, snapshot, |_| true)
        } else {
            false
        };

        self.done = true;

        if !committed {
            let reason = self
                .operation
                .signal
                .reason()
                .unwrap_or_else(|| Arc::new(CancellationReason::Disposed));

            return Poll::Ready(Err(Arc::new(ReactiveFailure::Cancelled(reason))));
        }

        Poll::Ready(result)
    }
}

impl<T, E, Fut, L> Future for ActionDispatch<T, E, Fut, L>
where
    Fut: Future<Output = Result<T, E>>,
    L: Fn() + ?Sized,
{
    type Output = Result<Arc<T>, Arc<ReactiveFailure<E>>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        assert!(!this.done, "completed dispatch polled again");

        if let Some(result) = this.stopped(cx) {
            return Poll::Ready(result);
        }

        let Some(mut future) = this.operation.take() else {
            return Poll::Pending;
        };

        match future.as_mut().poll(cx) {
            Poll::Ready(result) => {
                drop(future);
                this.complete(result)
            }
            Poll::Pending => {
                this.operation.restore(future);
                this.stopped(cx).map_or(Poll::Pending, Poll::Ready)
            }
        }
    }
}

impl<T, E, Fut, L: Fn() + ?Sized> Drop for ActionDispatch<T, E, Fut, L> {
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
