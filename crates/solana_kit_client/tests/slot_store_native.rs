#![cfg(all(feature = "reactive", not(target_arch = "wasm32")))]
//! Constructor inference, conditional Send and reentrant native wakeup controls.
use core::{
    future::{Future, pending, ready},
    pin::Pin,
    task::{Context as TaskContext, Poll, Waker},
};
use futures::{Stream, stream};
use solana_kit_client::{
    Presence,
    methods::{Context, Contextual},
    reactive::{
        CancellationSource, StreamSnapshot,
        slot_tracking::{LocalSlotTrackingStore, SlotTrackingStore},
    },
};
use std::{
    cell::Cell,
    rc::Rc,
    sync::{Arc, Mutex},
    task::Wake,
};

fn response<T>(value: T) -> Contextual<T> {
    Contextual {
        context: Context {
            slot: 1,
            api_version: Presence::Missing,
        },
        value,
    }
}

fn require_send<T: Send>(_: &T) {}

#[test]
fn idle_constructor_infers_distinct_source_and_mapped_types() {
    let store = SlotTrackingStore::new(
        |_| ready(Ok::<_, ()>(response(1_i32))),
        |_| stream::pending::<Result<Contextual<i64>, ()>>(),
        |value: i32| Ok::<_, ()>(i64::from(value)),
        Ok::<i64, ()>,
    );
    assert!(matches!(&*store.state(), StreamSnapshot::Idle));
    let calls = Rc::new(Cell::new(0));
    let observed = calls.clone();
    let local = LocalSlotTrackingStore::new_local(
        move |_| {
            observed.set(observed.get() + 1);
            ready(Ok::<_, ()>(response(1_i32)))
        },
        |_| stream::pending::<Result<Contextual<i64>, ()>>(),
        |value: i32| Ok::<_, ()>(i64::from(value)),
        Ok::<i64, ()>,
    );
    assert!(matches!(&*local.state(), StreamSnapshot::Idle));
    assert_eq!(calls.get(), 0);
}

#[test]
fn native_store_driver_and_observer_send_follow_concrete_inputs() {
    let store = SlotTrackingStore::new(
        |_| ready(Ok::<_, ()>(response(1_i32))),
        |_| stream::pending::<Result<Contextual<i32>, ()>>(),
        Ok::<i32, ()>,
        Ok::<i32, ()>,
    );
    let mut driver = store.connect();
    let mut observer = store.observe(None);
    require_send(&store);
    require_send(&driver);
    require_send(&observer);
    let mut context = TaskContext::from_waker(Waker::noop());
    assert!(Pin::new(&mut driver).poll(&mut context).is_pending());
    let Poll::Ready(Some(Ok(value))) = Pin::new(&mut observer).poll_next(&mut context) else {
        panic!("native observer did not receive initial result")
    };
    assert_eq!(value.value, 1);
}

struct ResetOnWake(Mutex<Option<Box<dyn FnOnce() + Send>>>);

impl Wake for ResetOnWake {
    fn wake(self: Arc<Self>) {
        let callback = self.0.lock().unwrap().take();

        if let Some(callback) = callback {
            callback();
        }
    }
}

#[test]
fn cancellation_waker_can_reset_the_store_without_a_lock_cycle() {
    let store = SlotTrackingStore::new(
        |_| pending::<Result<Contextual<i32>, ()>>(),
        |_| stream::pending::<Result<Contextual<i32>, ()>>(),
        Ok::<i32, ()>,
        Ok::<i32, ()>,
    );
    let caller = CancellationSource::new();
    let mut driver = store.connect_with_signal(caller.token());
    let captured = store.clone();
    let wake = Arc::new(ResetOnWake(Mutex::new(Some(Box::new(move || {
        captured.reset()
    })))));
    let waker = Waker::from(wake);
    assert!(
        Pin::new(&mut driver)
            .poll(&mut TaskContext::from_waker(&waker))
            .is_pending()
    );
    caller.cancel(None);
    assert!(matches!(&*store.state(), StreamSnapshot::Idle));
    assert!(matches!(
        Pin::new(&mut driver).poll(&mut TaskContext::from_waker(Waker::noop())),
        Poll::Ready(Err(_))
    ));
}
