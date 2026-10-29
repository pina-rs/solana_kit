//! Reuses the same bounded observer buffer and cancellation priority as other stores.
use super::SlotTrackingStore;
use crate::{
    methods::Contextual,
    reactive::{
        CancellationToken, StoreStream,
        bridge::{local_observer, shared_observer},
    },
};
use core::convert::Infallible;

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
where
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
    F: Send + 'static,
    S: Send + 'static,
{
    /// Observes current and subsequent accepted responses without owning connection lifetime.
    pub fn observe(
        &self,
        cancellation: Option<CancellationToken<E>>,
    ) -> StoreStream<Contextual<T>, E> {
        self.observe_filtered(cancellation, |_| Ok::<_, Infallible>(true))
    }

    /// Filters complete contextual values before latest-accepted-value buffering.
    pub fn observe_filtered<PredicateError, G>(
        &self,
        cancellation: Option<CancellationToken<E>>,
        predicate: G,
    ) -> StoreStream<Contextual<T>, E, PredicateError>
    where
        PredicateError: Send + Sync + 'static,
        G: Fn(&Contextual<T>) -> Result<bool, PredicateError> + Send + Sync + 'static,
    {
        shared_observer(&self.core, cancellation, predicate)
    }
}

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
        dyn Fn(),
    >
where
    T: 'static,
    E: 'static,
    F: 'static,
    S: 'static,
{
    /// Observes local payloads and sources without requiring Send or Clone.
    pub fn observe(
        &self,
        cancellation: Option<CancellationToken<E>>,
    ) -> StoreStream<Contextual<T>, E, Infallible, dyn Fn()> {
        self.observe_filtered(cancellation, |_| Ok::<_, Infallible>(true))
    }

    /// Applies a local fallible predicate while sharing the standard observer lifecycle.
    pub fn observe_filtered<PredicateError, G>(
        &self,
        cancellation: Option<CancellationToken<E>>,
        predicate: G,
    ) -> StoreStream<Contextual<T>, E, PredicateError, dyn Fn()>
    where
        PredicateError: 'static,
        G: Fn(&Contextual<T>) -> Result<bool, PredicateError> + 'static,
    {
        local_observer(&self.core, cancellation, predicate)
    }
}
