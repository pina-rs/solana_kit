//! A cancellable connection window combining initial and subscription values.
use crate::{
    methods::Contextual,
    reactive::{
        CancellationToken, ListenerLease, ReactiveFailure, StreamSnapshot,
        shared::{Core, Operation},
    },
};
use core::{future::Future, pin::Pin};
use futures_core::Stream;
use std::sync::Arc;

mod driver;
mod observe;
pub use driver::SlotStoreDriver;
use driver::failure_from_reason;

struct Work<F, S> {
    initial: Option<Pin<Box<F>>>,
    subscription: Option<Pin<Box<S>>>,
}

type StoreCore<T, E, F, S, L> = Core<StreamSnapshot<Contextual<T>, E>, Work<F, S>, E, L>;

/// Fresh caller-owned sources combined by slot within each connection window.
///
/// Retained data supplies the watermark across reconnects. Mappers run only for
/// eligible slots, before publishing a new watermark; failures end both sources.
pub struct SlotTrackingStore<
    T,
    E,
    InitialFactory,
    SubscriptionFactory,
    InitialMapper,
    SubscriptionMapper,
    F,
    S,
    L: ?Sized = dyn Fn() + Send + Sync,
> {
    initial_factory: Arc<InitialFactory>,
    subscription_factory: Arc<SubscriptionFactory>,
    initial_mapper: Arc<InitialMapper>,
    subscription_mapper: Arc<SubscriptionMapper>,
    core: Arc<StoreCore<T, E, F, S, L>>,
}

/// Composed slot store with local callbacks and local source futures/streams.
pub type LocalSlotTrackingStore<
    T,
    E,
    InitialFactory,
    SubscriptionFactory,
    InitialMapper,
    SubscriptionMapper,
    F,
    S,
> = SlotTrackingStore<
    T,
    E,
    InitialFactory,
    SubscriptionFactory,
    InitialMapper,
    SubscriptionMapper,
    F,
    S,
    dyn Fn(),
>;

impl<T, E, InitialFactory, SubscriptionFactory, InitialMapper, SubscriptionMapper, F, S>
    SlotTrackingStore<
        T,
        E,
        InitialFactory,
        SubscriptionFactory,
        InitialMapper,
        SubscriptionMapper,
        F,
        S,
    >
{
    /// Creates an idle store without invoking or polling either factory.
    pub fn new<Input, Update>(
        initial_factory: InitialFactory,
        subscription_factory: SubscriptionFactory,
        initial_mapper: InitialMapper,
        subscription_mapper: SubscriptionMapper,
    ) -> Self
    where
        InitialFactory: Fn(CancellationToken<E>) -> F,
        SubscriptionFactory: Fn(CancellationToken<E>) -> S,
        F: Future<Output = Result<Contextual<Input>, E>>,
        S: Stream<Item = Result<Contextual<Update>, E>>,
        InitialMapper: Fn(Input) -> Result<T, E>,
        SubscriptionMapper: Fn(Update) -> Result<T, E>,
    {
        Self::create(
            initial_factory,
            subscription_factory,
            initial_mapper,
            subscription_mapper,
        )
    }
}

impl<T, E, InitialFactory, SubscriptionFactory, InitialMapper, SubscriptionMapper, F, S>
    LocalSlotTrackingStore<
        T,
        E,
        InitialFactory,
        SubscriptionFactory,
        InitialMapper,
        SubscriptionMapper,
        F,
        S,
    >
{
    /// Creates an idle store accepting local factory, map and listener captures.
    pub fn new_local<Input, Update>(
        initial_factory: InitialFactory,
        subscription_factory: SubscriptionFactory,
        initial_mapper: InitialMapper,
        subscription_mapper: SubscriptionMapper,
    ) -> Self
    where
        InitialFactory: Fn(CancellationToken<E>) -> F,
        SubscriptionFactory: Fn(CancellationToken<E>) -> S,
        F: Future<Output = Result<Contextual<Input>, E>>,
        S: Stream<Item = Result<Contextual<Update>, E>>,
        InitialMapper: Fn(Input) -> Result<T, E>,
        SubscriptionMapper: Fn(Update) -> Result<T, E>,
    {
        Self::create(
            initial_factory,
            subscription_factory,
            initial_mapper,
            subscription_mapper,
        )
    }

    /// Registers a callback whose captures need no Send or Sync bound.
    pub fn subscribe_local<G: Fn() + 'static>(&self, callback: G) -> ListenerLease<dyn Fn()> {
        self.subscribe(Arc::new(callback))
    }
}

impl<
    T,
    E,
    InitialFactory,
    SubscriptionFactory,
    InitialMapper,
    SubscriptionMapper,
    F,
    S,
    L: Fn() + ?Sized,
>
    SlotTrackingStore<
        T,
        E,
        InitialFactory,
        SubscriptionFactory,
        InitialMapper,
        SubscriptionMapper,
        F,
        S,
        L,
    >
{
    fn create(
        initial_factory: InitialFactory,
        subscription_factory: SubscriptionFactory,
        initial_mapper: InitialMapper,
        subscription_mapper: SubscriptionMapper,
    ) -> Self {
        Self {
            initial_factory: Arc::new(initial_factory),
            subscription_factory: Arc::new(subscription_factory),
            initial_mapper: Arc::new(initial_mapper),
            subscription_mapper: Arc::new(subscription_mapper),
            core: Arc::new(Core::new(StreamSnapshot::Idle)),
        }
    }

    /// Returns shared state, retaining the complete RPC context with its value.
    pub fn state(&self) -> Arc<StreamSnapshot<Contextual<T>, E>> {
        self.core.snapshot()
    }

    /// Registers a listener owned solely by the returned lease.
    pub fn subscribe(&self, callback: Arc<L>) -> ListenerLease<L> {
        self.core.subscribe(callback)
    }

    /// Cancels both children and clears cached data and its slot watermark.
    pub fn reset(&self) {
        self.core.reset(StreamSnapshot::Idle, |_| true);
    }

    /// Cancels both children permanently, retaining the last readable snapshot.
    pub fn dispose(&self) {
        self.core.dispose();
    }
}

impl<
    T,
    E,
    InitialFactory,
    SubscriptionFactory,
    InitialMapper,
    SubscriptionMapper,
    F,
    S,
    L,
    Input,
    Update,
>
    SlotTrackingStore<
        T,
        E,
        InitialFactory,
        SubscriptionFactory,
        InitialMapper,
        SubscriptionMapper,
        F,
        S,
        L,
    >
where
    InitialFactory: Fn(CancellationToken<E>) -> F,
    SubscriptionFactory: Fn(CancellationToken<E>) -> S,
    F: Future<Output = Result<Contextual<Input>, E>>,
    S: Stream<Item = Result<Contextual<Update>, E>>,
    InitialMapper: Fn(Input) -> Result<T, E>,
    SubscriptionMapper: Fn(Update) -> Result<T, E>,
    L: Fn() + ?Sized,
{
    /// Eagerly publishes loading and creates fresh source owners; caller polls them.
    pub fn connect(&self) -> SlotStoreDriver<T, E, F, S, InitialMapper, SubscriptionMapper, L> {
        self.start(None)
    }

    /// Adds caller cancellation to both children for this connection only.
    pub fn connect_with_signal(
        &self,
        caller: CancellationToken<E>,
    ) -> SlotStoreDriver<T, E, F, S, InitialMapper, SubscriptionMapper, L> {
        self.start(Some(caller))
    }

    /// Loading listeners may replace this window because notification is synchronous;
    /// source factories therefore run only after current ownership is rechecked.
    fn start(
        &self,
        caller: Option<CancellationToken<E>>,
    ) -> SlotStoreDriver<T, E, F, S, InitialMapper, SubscriptionMapper, L> {
        let operation = Arc::new(Operation::new(caller));
        let immediate = operation.signal.reason().map(failure_from_reason);
        let previous = self
            .core
            .begin(&operation, |state| state.starting(immediate.as_ref()));
        let disposed = previous.is_none();
        let immediate = disposed
            .then(|| Arc::new(ReactiveFailure::Disposed))
            .or(immediate);
        let driver = SlotStoreDriver {
            cancellation: operation.signal.cancelled(),
            operation,
            initial_mapper: self.initial_mapper.clone(),
            subscription_mapper: self.subscription_mapper.clone(),
            core: Arc::downgrade(&self.core),
            immediate,
            done: false,
        };

        if !disposed
            && self.core.current(&driver.operation.identity)
            && !driver.operation.signal.is_cancelled()
        {
            let subscription = (self.subscription_factory)(driver.operation.signal.clone());
            let mut work = Work {
                subscription: Some(Box::pin(subscription)),
                initial: None,
            };

            if !driver.operation.signal.is_cancelled()
                && self.core.current(&driver.operation.identity)
            {
                work.initial = Some(Box::pin((self.initial_factory)(
                    driver.operation.signal.clone(),
                )));
            }

            driver.operation.restore(work);
        }

        driver
    }
}

impl<T, E, InitialFactory, SubscriptionFactory, InitialMapper, SubscriptionMapper, F, S, L: ?Sized>
    Clone
    for SlotTrackingStore<
        T,
        E,
        InitialFactory,
        SubscriptionFactory,
        InitialMapper,
        SubscriptionMapper,
        F,
        S,
        L,
    >
{
    fn clone(&self) -> Self {
        Self {
            initial_factory: self.initial_factory.clone(),
            subscription_factory: self.subscription_factory.clone(),
            initial_mapper: self.initial_mapper.clone(),
            subscription_mapper: self.subscription_mapper.clone(),
            core: self.core.clone(),
        }
    }
}
