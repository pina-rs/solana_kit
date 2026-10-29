//! Readable cancellation with weak waiter registrations and unlocked wakeups.
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use std::sync::{
    Arc, Mutex, Weak,
    atomic::{AtomicBool, Ordering},
};

/// Why an operation stopped, preserving an optional caller-owned cause.
#[derive(Debug)]
pub enum CancellationReason<E> {
    /// Cancellation requested through the caller's source.
    Caller(Option<Arc<E>>),
    /// A newer operation owns the store.
    Superseded,
    /// The store returned to idle.
    Reset,
    /// The store was permanently disposed.
    Disposed,
    /// The caller dropped an unfinished driver.
    Dropped,
}

struct Source<E> {
    cancelled: AtomicBool,
    reason: Mutex<Option<Arc<CancellationReason<E>>>>,
    waiters: Mutex<Vec<Weak<Mutex<Option<Waker>>>>>,
}

/// Owner of an idempotent cancellation signal.
pub struct CancellationSource<E>(Arc<Source<E>>);

impl<E> Clone for CancellationSource<E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<E> Default for CancellationSource<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> CancellationSource<E> {
    /// Creates an uncancelled source without starting any work.
    pub fn new() -> Self {
        Self(Arc::new(Source {
            cancelled: AtomicBool::new(false),
            reason: Mutex::new(None),
            waiters: Mutex::new(Vec::new()),
        }))
    }

    /// Returns a readable token that does not cancel when dropped.
    pub fn token(&self) -> CancellationToken<E> {
        CancellationToken {
            sources: vec![self.0.clone()],
        }
    }

    /// Cancels once and wakes registered waiters outside all locks.
    pub fn cancel(&self, reason: Option<E>) {
        self.cancel_with(Arc::new(CancellationReason::Caller(reason.map(Arc::new))));
    }

    pub(super) fn cancel_with(&self, reason: Arc<CancellationReason<E>>) {
        if !self.record_reason(&reason) {
            return;
        }

        self.0.cancelled.store(true, Ordering::Release);
        let waiters = {
            let mut waiters = self.0.waiters.lock().expect("cancellation waiters");
            core::mem::take(&mut *waiters)
        };

        for waiter in waiters.into_iter().filter_map(|waiter| waiter.upgrade()) {
            let waker = waiter.lock().expect("cancellation waker").take();

            if let Some(waker) = waker {
                waker.wake();
            }
        }
    }

    fn record_reason(&self, reason: &Arc<CancellationReason<E>>) -> bool {
        let mut current = self.0.reason.lock().expect("cancellation state");

        if current.is_some() {
            return false;
        }

        *current = Some(reason.clone());
        true
    }
}

/// Cloneable read-only signal, including a per-operation internal source.
pub struct CancellationToken<E> {
    sources: Vec<Arc<Source<E>>>,
}

impl<E> Clone for CancellationToken<E> {
    fn clone(&self) -> Self {
        Self {
            sources: self.sources.clone(),
        }
    }
}

impl<E> CancellationToken<E> {
    /// Whether any composed source has cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.sources
            .iter()
            .any(|source| source.cancelled.load(Ordering::Acquire))
    }

    /// The first reason; internal cancellation takes precedence over the caller.
    pub fn reason(&self) -> Option<Arc<CancellationReason<E>>> {
        self.sources
            .iter()
            .find_map(|source| source.reason.lock().expect("cancellation state").clone())
    }

    /// Creates an independently registered, wakeable cancellation future.
    pub fn cancelled(&self) -> Cancelled<E> {
        let waiters = self
            .sources
            .iter()
            .map(|source| {
                let waiter = Arc::new(Mutex::new(None));
                let mut waiters = source.waiters.lock().expect("cancellation waiters");
                waiters.retain(|waiter| waiter.strong_count() != 0);
                waiters.push(Arc::downgrade(&waiter));
                waiter
            })
            .collect();
        Cancelled {
            token: self.clone(),
            waiters,
        }
    }

    pub(super) fn with_caller(mut self, caller: Option<Self>) -> Self {
        if let Some(caller) = caller {
            self.sources.extend(caller.sources);
        }

        self
    }
}

/// A cancellation waiter; dropping it releases its retained executor wakers.
pub struct Cancelled<E> {
    token: CancellationToken<E>,
    waiters: Vec<Arc<Mutex<Option<Waker>>>>,
}

impl<E> Future for Cancelled<E> {
    type Output = Arc<CancellationReason<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(reason) = self.token.reason() {
            return Poll::Ready(reason);
        }

        for waiter in &self.waiters {
            let next = cx.waker().clone();
            let previous = waiter.lock().expect("cancellation waker").replace(next);
            drop(previous);
        }

        self.token.reason().map_or(Poll::Pending, Poll::Ready)
    }
}

impl<E> Drop for Cancelled<E> {
    fn drop(&mut self) {
        for waiter in &self.waiters {
            let previous = waiter.lock().expect("cancellation waker").take();
            drop(previous);
        }
    }
}
