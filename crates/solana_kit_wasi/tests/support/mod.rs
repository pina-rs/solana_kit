use crate::reactor::Reactor;
use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{RawWaker, RawWakerVTable, Waker};

thread_local! {
    static OBSERVER: RefCell<Option<Reactor>> = const { RefCell::new(None) };
}

#[derive(Default)]
pub struct Probe {
    clones: AtomicUsize,
    drops: AtomicUsize,
    wakes: AtomicUsize,
}

impl Probe {
    pub fn new(reactor: &Reactor) -> Arc<Self> {
        OBSERVER.with(|observer| *observer.borrow_mut() = Some(reactor.clone()));
        Arc::new(Self::default())
    }

    pub fn waker(self: &Arc<Self>) -> Waker {
        // Each raw pointer owns one Arc and the vtable preserves that ownership.
        unsafe { Waker::from_raw(raw(self.clone())) }
    }

    pub fn clones(&self) -> usize {
        self.clones.load(Ordering::SeqCst)
    }

    pub fn drops(&self) -> usize {
        self.drops.load(Ordering::SeqCst)
    }

    pub fn wakes(&self) -> usize {
        self.wakes.load(Ordering::SeqCst)
    }
}

fn inspect() {
    OBSERVER.with(|observer| {
        observer
            .borrow()
            .as_ref()
            .expect("observer")
            .registrations();
    });
}

fn raw(probe: Arc<Probe>) -> RawWaker {
    RawWaker::new(Arc::into_raw(probe).cast(), &VTABLE)
}

unsafe fn clone(data: *const ()) -> RawWaker {
    // The borrowed Arc remains owned by the original RawWaker.
    let probe = ManuallyDrop::new(unsafe { Arc::<Probe>::from_raw(data.cast()) });
    inspect();
    probe.clones.fetch_add(1, Ordering::SeqCst);
    raw(Arc::clone(&probe))
}

unsafe fn wake(data: *const ()) {
    // Wake consumes exactly the Arc owned by this RawWaker.
    let probe = unsafe { Arc::<Probe>::from_raw(data.cast()) };
    inspect();
    probe.wakes.fetch_add(1, Ordering::SeqCst);
}

unsafe fn wake_by_ref(data: *const ()) {
    // A borrowed wake leaves the RawWaker's Arc alive.
    let probe = ManuallyDrop::new(unsafe { Arc::<Probe>::from_raw(data.cast()) });
    inspect();
    probe.wakes.fetch_add(1, Ordering::SeqCst);
}

unsafe fn drop_waker(data: *const ()) {
    // Drop consumes exactly the Arc owned by this RawWaker.
    let probe = unsafe { Arc::<Probe>::from_raw(data.cast()) };
    inspect();
    probe.drops.fetch_add(1, Ordering::SeqCst);
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop_waker);
