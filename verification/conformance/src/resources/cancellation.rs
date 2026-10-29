//! Dropping a suspended estimator drops the exact owned adapter exchange.

use super::*;
use crate::require;
use alloc::sync::Arc;
use core::{
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
};
use solana_kit_client::resources::{EstimateConfig, estimate, estimate_and_set_with_client};

struct PendingTransport {
    active: Arc<AtomicUsize>,
    dropped: Arc<AtomicUsize>,
}

struct Exchange {
    active: Arc<AtomicUsize>,
    dropped: Arc<AtomicUsize>,
}

impl Transport for PendingTransport {
    type Exchange<'a> = Exchange;

    fn exchange(&self, _: Request) -> Self::Exchange<'_> {
        self.active.fetch_add(1, Ordering::SeqCst);
        Exchange {
            active: self.active.clone(),
            dropped: self.dropped.clone(),
        }
    }
}

impl Future for Exchange {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Drop for Exchange {
    fn drop(&mut self) {
        self.active.fetch_sub(1, Ordering::SeqCst);
        self.dropped.fetch_add(1, Ordering::SeqCst);
    }
}

pub(super) fn run() -> Result<(), &'static str> {
    let active = Arc::new(AtomicUsize::new(0));
    let dropped = Arc::new(AtomicUsize::new(0));
    let client = Client::new(
        PendingTransport {
            active: active.clone(),
            dropped: dropped.clone(),
        },
        Limits::default(),
    );
    let message = draft(VersionContext::Legacy);
    let mut future = alloc::boxed::Box::pin(estimate(&client, &message, EstimateConfig::default()));
    require(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending(),
        "pending simulation completed",
    )?;
    require(
        active.load(Ordering::SeqCst) == 1 && dropped.load(Ordering::SeqCst) == 0,
        "simulation exchange ownership missing",
    )?;
    drop(future);
    require(
        active.load(Ordering::SeqCst) == 0 && dropped.load(Ordering::SeqCst) == 1,
        "dropped estimator detached simulation",
    )?;
    let mut future = alloc::boxed::Box::pin(estimate_and_set_with_client(
        &client,
        message,
        EstimateConfig::default(),
    ));
    require(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending(),
        "pending tuning completed",
    )?;
    drop(future);
    require(
        active.load(Ordering::SeqCst) == 0 && dropped.load(Ordering::SeqCst) == 2,
        "dropped tuning detached simulation",
    )?;
    native_send(&client)
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn native_send(client: &Client<PendingTransport>) -> Result<(), &'static str> {
    fn assert_send<T: Send>(_: T) {}
    let message = draft(VersionContext::Legacy);
    assert_send(estimate(client, &message, EstimateConfig::default()));
    assert_send(estimate_and_set_with_client(
        client,
        message,
        EstimateConfig::default(),
    ));

    Ok(())
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn native_send(_: &Client<PendingTransport>) -> Result<(), &'static str> {
    Ok(())
}
