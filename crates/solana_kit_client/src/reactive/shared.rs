//! Storage holds weak work/listener edges; user code always runs unlocked.
use super::{CancellationReason, CancellationSource, CancellationToken};
use std::sync::{
    Arc, Mutex, Weak,
    atomic::{AtomicBool, Ordering},
};

/// A source failure, cancellation, or attempted use of a disposed store.
#[derive(Debug)]
pub enum ReactiveFailure<E> {
    /// Original typed source failure.
    Source(E),
    /// Explicit cancellation with its original reason.
    Cancelled(Arc<CancellationReason<E>>),
    /// Dispatch/connect was attempted after disposal.
    Disposed,
}

struct Registration<L: ?Sized> {
    active: AtomicBool,
    callback: Arc<L>,
}

/// Owns one independent listener registration; Drop unsubscribes immediately.
///
/// Keep this lease for as long as notifications are wanted. A callback may capture
/// its store strongly because the store retains only a weak registration edge.
pub struct ListenerLease<L: ?Sized> {
    registration: Arc<Registration<L>>,
}

impl<L: ?Sized> ListenerLease<L> {
    /// Stops this registration idempotently, including a captured pending callback.
    pub fn unsubscribe(&self) {
        self.registration.active.store(false, Ordering::Release);
    }
}

impl<L: ?Sized> Drop for ListenerLease<L> {
    fn drop(&mut self) {
        self.unsubscribe();
    }
}

pub(super) struct Operation<W, E> {
    pub identity: Arc<()>,
    pub source: CancellationSource<E>,
    pub signal: CancellationToken<E>,
    work: Mutex<Option<W>>,
}

impl<W, E> Operation<W, E> {
    pub fn new(caller: Option<CancellationToken<E>>) -> Self {
        let source = CancellationSource::new();
        let signal = source.token().with_caller(caller);
        Self {
            identity: Arc::new(()),
            source,
            signal,
            work: Mutex::new(None),
        }
    }

    pub fn take(&self) -> Option<W> {
        self.work.lock().expect("operation work").take()
    }

    pub fn restore(&self, work: W) {
        let mut retained = Some(work);
        {
            let mut slot = self.work.lock().expect("operation work");

            if !self.signal.is_cancelled() {
                debug_assert!(slot.is_none());
                core::mem::swap(&mut *slot, &mut retained);
            }
        }
        drop(retained);
    }

    pub fn cancel(&self, reason: Arc<CancellationReason<E>>) {
        self.source.cancel_with(reason);
        let work = self.take();
        drop(work);
    }
}

struct Active<W, E> {
    identity: Arc<()>,
    operation: Weak<Operation<W, E>>,
    source: CancellationSource<E>,
}

impl<W, E> Active<W, E> {
    fn cancel(self, reason: CancellationReason<E>) {
        if let Some(operation) = self.operation.upgrade() {
            operation.cancel(Arc::new(reason));
        } else {
            // Retain completed-operation token semantics without retaining work.
            self.source.cancel_with(Arc::new(reason));
        }
    }
}

struct State<Snapshot, W, E, L: ?Sized> {
    snapshot: Arc<Snapshot>,
    active: Option<Active<W, E>>,
    listeners: Vec<Weak<Registration<L>>>,
    disposed: bool,
}

type Listeners<L> = Vec<Weak<Registration<L>>>;
type Change<Snapshot, W, E, L> = (Arc<Snapshot>, Option<Active<W, E>>, Listeners<L>);

impl<Snapshot, W, E, L: ?Sized> State<Snapshot, W, E, L> {
    // Only built-in snapshot constructors reach this private transaction. They
    // borrow their captures so unlocking always precedes user-owned destruction.
    fn begin(
        &mut self,
        operation: &Arc<Operation<W, E>>,
        next: impl FnOnce(&Snapshot) -> (Snapshot, bool),
    ) -> Option<Change<Snapshot, W, E, L>> {
        if self.disposed {
            return None;
        }

        let (snapshot, should_notify) = next(&self.snapshot);
        let previous = core::mem::replace(&mut self.snapshot, Arc::new(snapshot));
        let active = self.active.replace(Active {
            identity: operation.identity.clone(),
            operation: Arc::downgrade(operation),
            source: operation.source.clone(),
        });
        let listeners = if should_notify {
            self.listeners.clone()
        } else {
            Vec::new()
        };
        Some((previous, active, listeners))
    }

    fn current(&self, identity: &Arc<()>) -> bool {
        !self.disposed
            && self
                .active
                .as_ref()
                .is_some_and(|active| Arc::ptr_eq(&active.identity, identity))
    }

    fn commit(
        &mut self,
        identity: &Arc<()>,
        next: &mut Option<Arc<Snapshot>>,
        accept: fn(&Snapshot) -> bool,
    ) -> Option<(Arc<Snapshot>, Listeners<L>)> {
        if !self.current(identity) || !accept(&self.snapshot) {
            return None;
        }

        let previous = core::mem::replace(&mut self.snapshot, next.take().expect("snapshot"));
        Some((previous, self.listeners.clone()))
    }

    fn reset(
        &mut self,
        next: &mut Option<Arc<Snapshot>>,
        should_notify: fn(&Snapshot) -> bool,
    ) -> Option<Change<Snapshot, W, E, L>> {
        if self.disposed {
            return None;
        }

        let listeners = if should_notify(&self.snapshot) {
            self.listeners.clone()
        } else {
            Vec::new()
        };
        let previous = core::mem::replace(&mut self.snapshot, next.take().expect("snapshot"));
        Some((previous, self.active.take(), listeners))
    }
}

pub(super) struct Core<Snapshot, W, E, L: ?Sized> {
    state: Mutex<State<Snapshot, W, E, L>>,
}

impl<Snapshot, W, E, L: Fn() + ?Sized> Core<Snapshot, W, E, L> {
    pub fn new(snapshot: Snapshot) -> Self {
        Self {
            state: Mutex::new(State {
                snapshot: Arc::new(snapshot),
                active: None,
                listeners: Vec::new(),
                disposed: false,
            }),
        }
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        self.state.lock().expect("store state").snapshot.clone()
    }

    // Admission is built-in metadata work, never a user callback. Its sole
    // caller borrows its captures and takes only the bridge metadata mutex.
    pub fn admit_current(&self, snapshot: &Arc<Snapshot>, admit: impl FnOnce() -> bool) -> bool {
        let state = self.state.lock().expect("store state");

        if !Arc::ptr_eq(&state.snapshot, snapshot) {
            return false;
        }

        admit()
    }

    pub fn subscribe(&self, callback: Arc<L>) -> ListenerLease<L> {
        let registration = Arc::new(Registration {
            active: AtomicBool::new(true),
            callback,
        });
        let disposed = {
            let mut state = self.state.lock().expect("store state");
            state
                .listeners
                .retain(|listener| listener.strong_count() != 0);

            if !state.disposed {
                state.listeners.push(Arc::downgrade(&registration));
            }

            state.disposed
        };

        if disposed {
            registration.active.store(false, Ordering::Release);
        }

        ListenerLease { registration }
    }

    pub fn begin(
        &self,
        operation: &Arc<Operation<W, E>>,
        next: impl FnOnce(&Snapshot) -> (Snapshot, bool),
    ) -> Option<Arc<Snapshot>> {
        let change = self
            .state
            .lock()
            .expect("store state")
            .begin(operation, next);
        let (previous, active, listeners) = change?;

        if let Some(active) = active {
            active.cancel(CancellationReason::Superseded);
        }

        notify(listeners);
        Some(previous)
    }

    pub fn current(&self, identity: &Arc<()>) -> bool {
        self.state.lock().expect("store state").current(identity)
    }

    pub fn commit(
        &self,
        identity: &Arc<()>,
        snapshot: Snapshot,
        accept: fn(&Snapshot) -> bool,
    ) -> bool {
        let mut next = Some(Arc::new(snapshot));
        let change = self
            .state
            .lock()
            .expect("store state")
            .commit(identity, &mut next, accept);

        if let Some((previous, listeners)) = change {
            drop(previous);
            notify(listeners);

            return true;
        }

        false
    }

    pub fn reset(&self, snapshot: Snapshot, should_notify: fn(&Snapshot) -> bool) {
        let mut next = Some(Arc::new(snapshot));
        let change = self
            .state
            .lock()
            .expect("store state")
            .reset(&mut next, should_notify);
        let Some((previous, active, listeners)) = change else {
            return;
        };

        if let Some(active) = active {
            active.cancel(CancellationReason::Reset);
        }

        drop(previous);
        notify(listeners);
    }

    pub fn dispose(&self) {
        let (active, listeners) = {
            let mut state = self.state.lock().expect("store state");
            state.disposed = true;
            (state.active.take(), core::mem::take(&mut state.listeners))
        };
        deactivate(listeners);

        if let Some(active) = active {
            active.cancel(CancellationReason::Disposed);
        }
    }
}

impl<Snapshot, W, E, L: ?Sized> Drop for Core<Snapshot, W, E, L> {
    fn drop(&mut self) {
        let state = self.state.get_mut().expect("unlocked store destruction");
        let active = state.active.take();
        let listeners = core::mem::take(&mut state.listeners);
        deactivate(listeners);

        if let Some(active) = active {
            active.cancel(CancellationReason::Disposed);
        }
    }
}

fn notify<L: Fn() + ?Sized>(listeners: Vec<Weak<Registration<L>>>) {
    for listener in listeners
        .into_iter()
        .filter_map(|listener| listener.upgrade())
    {
        if listener.active.load(Ordering::Acquire) {
            (listener.callback)();
        }
    }
}

fn deactivate<L: ?Sized>(listeners: Vec<Weak<Registration<L>>>) {
    for listener in listeners
        .into_iter()
        .filter_map(|listener| listener.upgrade())
    {
        listener.active.store(false, Ordering::Release);
    }
}
