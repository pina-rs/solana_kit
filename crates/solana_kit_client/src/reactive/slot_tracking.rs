//! Slot-aware stream merging and a caller-driven composed reactive store.
//!
//! `SlotTrackingStream` polls both owned inputs in the first round, subscription before initial.
//! Equal slots are accepted. Older values do not reach their mapper. Each input
//! has one pending output, drained before more source work, so buffering is bounded.
//!
//! Source and typed mapper errors remain stream items. A mapper failure advances
//! the watermark, matching the pinned Dart helper. Forwarding subscription mapper
//! errors is a deliberate hardening of Dart's uncaught listener exception path.
//! Rust panics are not converted to typed errors. No work runs until polled.
//! `SlotTrackingStore` adds fresh source factories, retained refresh state and
//! terminal shared-failure cancellation; its mapper advances the slot only on success.

use crate::methods::Contextual;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_core::{Stream, stream::FusedStream};

mod store;
pub use store::{LocalSlotTrackingStore, SlotStoreDriver, SlotTrackingStore};

/// A caller-polled merge with owned inputs and one pending result per source.
///
/// Inputs may use local values or produce Send work according to their concrete
/// types. Dropping this adapter drops both actual inputs; nothing is detached.
/// Each input is fused independently and termination waits for both and buffers.
#[must_use = "poll the stream or drop it to release both owned inputs"]
pub struct SlotTrackingStream<F, S, InitialMapper, SubscriptionMapper, T, E> {
    initial: Option<Pin<Box<F>>>,
    subscription: Option<Pin<Box<S>>>,
    initial_mapper: InitialMapper,
    subscription_mapper: SubscriptionMapper,
    initial_pending: Option<Result<Contextual<T>, E>>,
    subscription_pending: Option<Result<Contextual<T>, E>>,
    watermark: Option<u64>,
}

impl<F, S, InitialMapper, SubscriptionMapper, T, E>
    SlotTrackingStream<F, S, InitialMapper, SubscriptionMapper, T, E>
{
    /// Owns both sources without polling, spawning, or requiring Unpin or Send.
    ///
    /// Both mappers share the source error type. Callers can use their own error
    /// enum when the RPC, subscription, and mapping failures differ.
    pub fn new<I, U>(
        initial: F,
        subscription: S,
        initial_mapper: InitialMapper,
        subscription_mapper: SubscriptionMapper,
    ) -> Self
    where
        F: Future<Output = Result<Contextual<I>, E>>,
        S: Stream<Item = Result<Contextual<U>, E>>,
        InitialMapper: FnMut(I) -> Result<T, E>,
        SubscriptionMapper: FnMut(U) -> Result<T, E>,
    {
        Self {
            initial: Some(Box::pin(initial)),
            subscription: Some(Box::pin(subscription)),
            initial_mapper,
            subscription_mapper,
            initial_pending: None,
            subscription_pending: None,
            watermark: None,
        }
    }

    fn take_pending(&mut self) -> Option<Result<Contextual<T>, E>> {
        self.subscription_pending
            .take()
            .or_else(|| self.initial_pending.take())
    }

    fn finished(&self) -> bool {
        self.initial.is_none()
            && self.subscription.is_none()
            && self.initial_pending.is_none()
            && self.subscription_pending.is_none()
    }
}

// Only source pointees are structurally pinned. No pin projection exposes a
// mapper or buffered item, so moving those owned values cannot violate pinning.
impl<F, S, InitialMapper, SubscriptionMapper, T, E> Unpin
    for SlotTrackingStream<F, S, InitialMapper, SubscriptionMapper, T, E>
{
}

impl<F, S, InitialMapper, SubscriptionMapper, T, E>
    SlotTrackingStream<F, S, InitialMapper, SubscriptionMapper, T, E>
{
    fn poll_initial<I>(&mut self, cx: &mut Context<'_>) -> bool
    where
        F: Future<Output = Result<Contextual<I>, E>>,
        InitialMapper: FnMut(I) -> Result<T, E>,
    {
        let Some(initial) = self.initial.as_mut() else {
            return false;
        };
        let Poll::Ready(result) = initial.as_mut().poll(cx) else {
            return false;
        };
        self.initial = None;
        self.initial_pending = map_result(result, &mut self.initial_mapper, &mut self.watermark);

        true
    }

    fn poll_subscription<U>(&mut self, cx: &mut Context<'_>) -> bool
    where
        S: Stream<Item = Result<Contextual<U>, E>>,
        SubscriptionMapper: FnMut(U) -> Result<T, E>,
    {
        let Some(subscription) = self.subscription.as_mut() else {
            return false;
        };

        match subscription.as_mut().poll_next(cx) {
            Poll::Pending => false,
            Poll::Ready(None) => {
                self.subscription = None;
                true
            }
            Poll::Ready(Some(result)) => {
                self.subscription_pending =
                    map_result(result, &mut self.subscription_mapper, &mut self.watermark);

                true
            }
        }
    }
}

impl<F, S, InitialMapper, SubscriptionMapper, T, E, I, U> Stream
    for SlotTrackingStream<F, S, InitialMapper, SubscriptionMapper, T, E>
where
    F: Future<Output = Result<Contextual<I>, E>>,
    S: Stream<Item = Result<Contextual<U>, E>>,
    InitialMapper: FnMut(I) -> Result<T, E>,
    SubscriptionMapper: FnMut(U) -> Result<T, E>,
{
    type Item = Result<Contextual<T>, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if let Some(item) = this.take_pending() {
            return Poll::Ready(Some(item));
        }

        let subscription_progressed = this.poll_subscription(cx);
        let initial_progressed = this.poll_initial(cx);

        if let Some(item) = this.take_pending() {
            return Poll::Ready(Some(item));
        }

        if this.finished() {
            return Poll::Ready(None);
        }

        if subscription_progressed || initial_progressed {
            cx.waker().wake_by_ref();
        }

        Poll::Pending
    }
}

impl<F, S, InitialMapper, SubscriptionMapper, T, E, I, U> FusedStream
    for SlotTrackingStream<F, S, InitialMapper, SubscriptionMapper, T, E>
where
    F: Future<Output = Result<Contextual<I>, E>>,
    S: Stream<Item = Result<Contextual<U>, E>>,
    InitialMapper: FnMut(I) -> Result<T, E>,
    SubscriptionMapper: FnMut(U) -> Result<T, E>,
{
    fn is_terminated(&self) -> bool {
        self.finished()
    }
}

fn map_result<I, T, E>(
    result: Result<Contextual<I>, E>,
    mapper: &mut impl FnMut(I) -> Result<T, E>,
    watermark: &mut Option<u64>,
) -> Option<Result<Contextual<T>, E>> {
    let response = match result {
        Ok(response) => response,
        Err(error) => return Some(Err(error)),
    };

    if watermark.is_some_and(|slot| response.context.slot < slot) {
        return None;
    }

    *watermark = Some(response.context.slot);
    let Contextual { context, value } = response;

    Some(mapper(value).map(|value| Contextual { context, value }))
}
