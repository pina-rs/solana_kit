//! Polls both actual inputs unlocked and commits only the current connection.
use super::{StoreCore, Work};
use crate::{
    methods::Contextual,
    reactive::{CancellationReason, Cancelled, ReactiveFailure, StreamSnapshot, shared::Operation},
};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_core::Stream;
use std::sync::{Arc, Weak};

type CurrentCoreResult<T, E, F, S, L> =
    Result<Arc<StoreCore<T, E, F, S, L>>, Arc<ReactiveFailure<E>>>;

/// Sole poll owner of both actual children; source completion leaves a quiet window.
#[must_use = "poll or explicitly spawn the driver; dropping it cancels both children"]
pub struct SlotStoreDriver<
    T,
    E,
    F,
    S,
    InitialMapper,
    SubscriptionMapper,
    L: Fn() + ?Sized = dyn Fn() + Send + Sync,
> {
    pub(super) operation: Arc<Operation<Work<F, S>, E>>,
    pub(super) core: Weak<StoreCore<T, E, F, S, L>>,
    pub(super) initial_mapper: Arc<InitialMapper>,
    pub(super) subscription_mapper: Arc<SubscriptionMapper>,
    pub(super) cancellation: Cancelled<E>,
    pub(super) immediate: Option<Arc<ReactiveFailure<E>>>,
    pub(super) done: bool,
}

pub(super) fn failure_from_reason<E>(
    reason: Arc<CancellationReason<E>>,
) -> Arc<ReactiveFailure<E>> {
    match &*reason {
        CancellationReason::Failure(failure) => failure.clone(),
        _ => Arc::new(ReactiveFailure::Cancelled(reason)),
    }
}

impl<T, E, F, S, InitialMapper, SubscriptionMapper, L: Fn() + ?Sized>
    SlotStoreDriver<T, E, F, S, InitialMapper, SubscriptionMapper, L>
{
    fn fail(
        &mut self,
        failure: Arc<ReactiveFailure<E>>,
        work: Option<Work<F, S>>,
    ) -> Arc<ReactiveFailure<E>> {
        self.operation
            .cancel(Arc::new(CancellationReason::Failure(failure.clone())));
        drop(work);

        if let Some(core) = self.core.upgrade() {
            let data = core.snapshot().data().cloned();
            core.commit(
                &self.operation.identity,
                StreamSnapshot::Error {
                    data,
                    error: failure.clone(),
                },
                |_| true,
            );
        }

        self.done = true;
        failure
    }

    fn stopped(&mut self, cx: &mut Context<'_>) -> Option<Arc<ReactiveFailure<E>>> {
        if let Some(failure) = self.immediate.take() {
            self.done = true;

            return Some(failure);
        }

        match Pin::new(&mut self.cancellation).poll(cx) {
            Poll::Ready(reason) => Some(self.fail(failure_from_reason(reason), None)),
            Poll::Pending => None,
        }
    }

    fn interrupted(&self) -> Option<Arc<ReactiveFailure<E>>> {
        self.operation.signal.reason().map(failure_from_reason)
    }

    fn current_core(&self) -> CurrentCoreResult<T, E, F, S, L> {
        if let Some(failure) = self.interrupted() {
            return Err(failure);
        }

        let Some(core) = self.core.upgrade() else {
            return Err(failure_from_reason(Arc::new(CancellationReason::Disposed)));
        };

        if !core.current(&self.operation.identity) {
            return Err(failure_from_reason(Arc::new(
                CancellationReason::Superseded,
            )));
        }

        Ok(core)
    }

    /// Mapping must finish before publication because it can fail or reenter the
    /// store; cached Context remains the only committed slot watermark.
    fn receive<Input>(
        &self,
        result: Result<Contextual<Input>, E>,
        mapper: &impl Fn(Input) -> Result<T, E>,
    ) -> Result<(), Arc<ReactiveFailure<E>>> {
        let core = self.current_core()?;
        let response = result.map_err(|error| Arc::new(ReactiveFailure::Source(error)))?;
        let previous = core.snapshot();

        if let Some(data) = previous
            .data()
            .filter(|data| response.context.slot < data.context.slot)
        {
            core.commit(
                &self.operation.identity,
                StreamSnapshot::Loaded { data: data.clone() },
                |state| matches!(state, StreamSnapshot::Loading { .. }),
            );

            return Ok(());
        }

        let Contextual { context, value } = response;
        let mapped = mapper(value);

        drop(self.current_core()?);
        let value = mapped.map_err(|error| Arc::new(ReactiveFailure::Source(error)))?;
        core.commit(
            &self.operation.identity,
            StreamSnapshot::Loaded {
                data: Arc::new(Contextual { context, value }),
            },
            |_| true,
        );

        Ok(())
    }
}

impl<T, E, F, S, InitialMapper, SubscriptionMapper, L, Input, Update>
    SlotStoreDriver<T, E, F, S, InitialMapper, SubscriptionMapper, L>
where
    F: Future<Output = Result<Contextual<Input>, E>>,
    S: Stream<Item = Result<Contextual<Update>, E>>,
    InitialMapper: Fn(Input) -> Result<T, E>,
    SubscriptionMapper: Fn(Update) -> Result<T, E>,
    L: Fn() + ?Sized,
{
    fn poll_subscription(
        &self,
        work: &mut Work<F, S>,
        cx: &mut Context<'_>,
    ) -> Result<bool, Arc<ReactiveFailure<E>>> {
        let Some(subscription) = work.subscription.as_mut() else {
            return Ok(false);
        };
        let result = subscription.as_mut().poll_next(cx);

        if let Some(failure) = self.interrupted() {
            return Err(failure);
        }

        match result {
            Poll::Pending => Ok(false),
            Poll::Ready(None) => {
                drop(work.subscription.take());
                Ok(true)
            }
            Poll::Ready(Some(result)) => {
                self.receive(result, &*self.subscription_mapper)?;
                Ok(true)
            }
        }
    }

    fn poll_initial(
        &self,
        work: &mut Work<F, S>,
        cx: &mut Context<'_>,
    ) -> Result<bool, Arc<ReactiveFailure<E>>> {
        let Some(mut initial) = work.initial.take() else {
            return Ok(false);
        };

        match initial.as_mut().poll(cx) {
            Poll::Pending => {
                work.initial = Some(initial);
                Ok(false)
            }
            Poll::Ready(result) => {
                drop(initial);
                self.receive(result, &*self.initial_mapper)?;
                Ok(true)
            }
        }
    }

    /// Both sources share one failure boundary because the first failure must
    /// cancel/drop the other child before its next poll can publish anything.
    fn round(
        &mut self,
        mut work: Work<F, S>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Arc<ReactiveFailure<E>>>> {
        let progress = self
            .poll_subscription(&mut work, cx)
            .and_then(|subscription_progress| {
                if let Some(failure) = self.interrupted() {
                    return Err(failure);
                }

                self.poll_initial(&mut work, cx)
                    .map(|initial_progress| initial_progress || subscription_progress)
            });

        let progressed = match progress {
            Ok(progressed) => progressed,
            Err(failure) => return Poll::Ready(Err(self.fail(failure, Some(work)))),
        };

        self.operation.restore(work);

        if let Some(failure) = self.stopped(cx) {
            return Poll::Ready(Err(failure));
        }

        if progressed {
            cx.waker().wake_by_ref();
        }

        Poll::Pending
    }
}

impl<T, E, F, S, InitialMapper, SubscriptionMapper, L, Input, Update> Future
    for SlotStoreDriver<T, E, F, S, InitialMapper, SubscriptionMapper, L>
where
    F: Future<Output = Result<Contextual<Input>, E>>,
    S: Stream<Item = Result<Contextual<Update>, E>>,
    InitialMapper: Fn(Input) -> Result<T, E>,
    SubscriptionMapper: Fn(Update) -> Result<T, E>,
    L: Fn() + ?Sized,
{
    type Output = Result<(), Arc<ReactiveFailure<E>>>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        assert!(!this.done, "completed slot-store driver polled again");

        if let Some(failure) = this.stopped(cx) {
            return Poll::Ready(Err(failure));
        }

        match this.operation.take() {
            Some(work) => this.round(work, cx),
            None => Poll::Pending,
        }
    }
}

impl<T, E, F, S, InitialMapper, SubscriptionMapper, L: Fn() + ?Sized> Drop
    for SlotStoreDriver<T, E, F, S, InitialMapper, SubscriptionMapper, L>
{
    fn drop(&mut self) {
        if !self.done {
            let reason = self
                .operation
                .signal
                .reason()
                .unwrap_or_else(|| Arc::new(CancellationReason::Dropped));
            self.fail(failure_from_reason(reason), None);
        }
    }
}
