//! Caller-owned single-threaded readiness registry. No network task is spawned.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::task::{Context, Poll, Wake, Waker};
use wasip2::clocks::monotonic_clock;
use wasip2::io::poll::{Pollable, poll};

#[derive(Default)]
struct PollWake(AtomicBool);

impl Wake for PollWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.store(true, Ordering::Release);
    }
}

impl PollWake {
    fn drive(&self, reactor: &Reactor) -> Result<(), DriveError> {
        if self.0.swap(false, Ordering::AcqRel) {
            return Ok(());
        }

        reactor.drive()
    }
}

/// Caller-owned local readiness driver. Clones share one registration owner.
#[derive(Clone, Default)]
pub struct Reactor(Rc<RefCell<Registry>>);

#[derive(Default)]
struct Registry {
    next: u64,
    waits: BTreeMap<u64, Registration>,
}

struct Registration {
    signal: Pollable,
    timer: Pollable,
    waker: Option<Waker>,
}

pub(crate) struct Wait {
    reactor: Reactor,
    id: u64,
}

impl Wait {
    #[cfg(feature = "websocket")]
    pub(crate) fn register(&mut self, context: &Context<'_>) {
        let waker = context.waker().clone();
        let previous = self
            .reactor
            .0
            .borrow_mut()
            .waits
            .get_mut(&self.id)
            .expect("live registration")
            .waker
            .replace(waker);
        drop(previous);
    }
}

impl Future for Wait {
    type Output = Result<(), ()>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = context.waker().clone();
        let mut registry = self.reactor.0.borrow_mut();
        let entry = registry.waits.get_mut(&self.id).expect("live registration");

        let result = if entry.timer.ready() {
            Err(())
        } else if entry.signal.ready() {
            Ok(())
        } else {
            let previous = entry.waker.replace(waker);
            drop(registry);
            drop(previous);

            return Poll::Pending;
        };

        let completed = registry.waits.remove(&self.id);
        drop(registry);
        drop(completed);

        Poll::Ready(result)
    }
}

impl Drop for Wait {
    fn drop(&mut self) {
        let cancelled = self.reactor.0.borrow_mut().waits.remove(&self.id);
        drop(cancelled);
    }
}

impl Reactor {
    pub(crate) fn wait(&self, signal: Pollable, deadline: u64) -> Result<Wait, ()> {
        let mut registry = self.0.borrow_mut();
        let id = registry.next;

        if registry.waits.len() >= 1024 {
            return Err(());
        }

        registry.next = id.checked_add(1).ok_or(())?;
        registry.waits.insert(
            id,
            Registration {
                signal,
                timer: monotonic_clock::subscribe_instant(deadline),
                waker: None,
            },
        );
        Ok(Wait {
            reactor: self.clone(),
            id,
        })
    }

    /// Returns currently retained readiness registrations, including deadlines.
    pub fn registrations(&self) -> usize {
        self.0.borrow().waits.len()
    }

    /// Explicitly blocks the caller's driver, never a Future::poll method.
    ///
    /// # Errors
    /// Returns NoReadiness if no polled WASI future can make progress.
    pub fn drive(&self) -> Result<(), DriveError> {
        let awakened = {
            let registry = self.0.borrow();
            let entries: Vec<_> = registry
                .waits
                .iter()
                .filter(|(_, entry)| entry.waker.is_some())
                .collect();

            if entries.is_empty() {
                return Err(DriveError::NoReadiness);
            }

            let signals: Vec<_> = entries
                .iter()
                .flat_map(|(_, entry)| [&entry.signal, &entry.timer])
                .collect();
            let mut ids: Vec<_> = poll(&signals)
                .into_iter()
                .map(|index| *entries[index as usize / 2].0)
                .collect();
            ids.sort_unstable();
            ids.dedup();
            ids
        };
        let wakers: Vec<_> = {
            let mut registry = self.0.borrow_mut();
            awakened
                .into_iter()
                .filter_map(|id| {
                    registry
                        .waits
                        .get_mut(&id)
                        .and_then(|entry| entry.waker.take())
                })
                .collect()
        };

        for waker in wakers {
            waker.wake();
        }

        Ok(())
    }

    /// Drives cooperative local polls before blocking on this reactor's host readiness.
    ///
    /// # Errors
    /// Returns NoReadiness when Pending work neither requests another poll nor
    /// registers a WASI signal.
    pub fn block_on<F: Future>(&self, future: F) -> Result<F::Output, DriveError> {
        let wake = Arc::new(PollWake::default());
        let waker = Waker::from(wake.clone());
        let mut context = Context::from_waker(&waker);
        let mut future = std::pin::pin!(future);

        loop {
            wake.0.store(false, Ordering::Release);

            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return Ok(output),
                Poll::Pending => wake.drive(self)?,
            }
        }
    }
}

/// An explicit driver misuse or unsupported readiness source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriveError {
    /// No pending WASI readiness registration exists.
    NoReadiness,
}
impl std::fmt::Display for DriveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WASI driver has no pending readiness")
    }
}
impl std::error::Error for DriveError {}
