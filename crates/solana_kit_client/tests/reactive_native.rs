#![cfg(all(feature = "reactive", not(target_arch = "wasm32")))]
//! Native auto traits and adversarial RawWaker hooks, separate from safe portable cases.
use solana_kit_client::reactive::{
    ActionSnapshot, CancellationSource, CancellationToken, LocalReactiveActionStore,
    LocalReactiveStreamStore, ReactiveActionStore, ReactiveStreamConnection, ReactiveStreamStore,
    StreamSnapshot,
};
use std::{
    future::{Future, pending, ready},
    mem::ManuallyDrop,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

const RESULT: i32 = 7;

fn require_send<T: Send>(_: &T) {}

#[test]
fn idle_constructors_infer_factory_results_without_starting_work() {
    let action =
        ReactiveActionStore::new(|_: CancellationToken<()>, ()| ready(Ok::<i32, ()>(RESULT)));
    assert!(matches!(&*action.state(), ActionSnapshot::Idle));
    let local = LocalReactiveActionStore::new_local(|_: CancellationToken<()>, ()| {
        ready(Ok::<i32, ()>(RESULT))
    });
    assert!(matches!(&*local.state(), ActionSnapshot::Idle));

    let factory = |_: CancellationToken<()>| {
        ready(Ok::<_, ()>(ReactiveStreamConnection {
            data: futures::stream::empty::<Result<i32, ()>>(),
            errors: futures::stream::empty::<Option<()>>(),
        }))
    };
    let streams = ReactiveStreamStore::new(factory);
    assert!(matches!(&*streams.state(), StreamSnapshot::Idle));
    let local = LocalReactiveStreamStore::new_local(factory);
    assert!(matches!(&*local.state(), StreamSnapshot::Idle));
}

#[test]
fn native_driver_and_store_send_follow_concrete_inputs() {
    let store = ReactiveActionStore::new(|_, ()| ready(Ok::<_, ()>(RESULT)));
    let mut dispatch = store.dispatch(());
    require_send(&store);
    require_send(&dispatch);

    let mut context = Context::from_waker(Waker::noop());
    let Poll::Ready(Ok(value)) = Pin::new(&mut dispatch).poll(&mut context) else {
        panic!("native dispatch did not complete")
    };

    assert_eq!(*value, RESULT);
    let streams = ReactiveStreamStore::new(|_| {
        ready(Ok::<_, ()>(ReactiveStreamConnection {
            data: futures::stream::empty::<Result<i32, ()>>(),
            errors: futures::stream::empty::<Option<()>>(),
        }))
    });
    let connection = streams.connect();
    let observer = streams.observe(None);
    require_send(&streams);
    require_send(&connection);
    require_send(&observer);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Event {
    Clone,
    Drop,
    Wake,
}

type Callback = Box<dyn FnOnce() + Send>;

struct Hooks {
    callback: Mutex<Option<(Event, Callback)>>,
}

impl Hooks {
    fn arm(&self, event: Event, callback: impl FnOnce() + Send + 'static) {
        *self.callback.lock().unwrap() = Some((event, Box::new(callback)));
    }

    fn fire(&self, event: Event) {
        let callback = {
            let mut callback = self.callback.lock().unwrap();
            let matching = callback
                .as_ref()
                .is_some_and(|(expected, _)| *expected == event);
            matching.then(|| callback.take()).flatten()
        };

        if let Some((_, callback)) = callback {
            callback();
        }
    }
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, wake_raw, wake_ref_raw, drop_raw);

unsafe fn clone_raw(pointer: *const ()) -> RawWaker {
    // Safety: every raw handle owns one Arc<Hooks>, retained by into_raw below.
    let hooks = ManuallyDrop::new(unsafe { Arc::<Hooks>::from_raw(pointer.cast()) });
    hooks.fire(Event::Clone);
    let cloned = Arc::clone(&hooks);
    RawWaker::new(Arc::into_raw(cloned).cast(), &VTABLE)
}

unsafe fn wake_raw(pointer: *const ()) {
    // Safety: consuming wake takes exactly the Arc owned by this raw handle.
    let hooks = unsafe { Arc::<Hooks>::from_raw(pointer.cast()) };
    hooks.fire(Event::Wake);
}

unsafe fn wake_ref_raw(pointer: *const ()) {
    // Safety: wake-by-reference retains the raw handle's original strong count.
    let hooks = ManuallyDrop::new(unsafe { Arc::<Hooks>::from_raw(pointer.cast()) });
    hooks.fire(Event::Wake);
}

unsafe fn drop_raw(pointer: *const ()) {
    // Safety: drop consumes exactly the Arc owned by this raw handle.
    let hooks = unsafe { Arc::<Hooks>::from_raw(pointer.cast()) };
    hooks.fire(Event::Drop);
}

fn waker(hooks: &Arc<Hooks>) -> Waker {
    let raw = RawWaker::new(Arc::into_raw(hooks.clone()).cast(), &VTABLE);
    // Safety: the static vtable implements Arc ownership for all four operations.
    unsafe { Waker::from_raw(raw) }
}

#[test]
fn waker_clone_drop_and_wake_can_reenter_store_commands() {
    // One-shot hooks avoid recursive firing while the same store command is
    // entered during waiter registration, replacement, or cancellation wakeup.
    for event in [Event::Clone, Event::Drop, Event::Wake] {
        let store = ReactiveActionStore::new(|_, ()| pending::<Result<i32, ()>>());
        let caller = CancellationSource::new();
        let mut driver = store.dispatch_with_signal((), caller.token());
        let hooks = Arc::new(Hooks {
            callback: Mutex::new(None),
        });
        let custom = waker(&hooks);

        if event == Event::Clone {
            let captured = store.clone();
            hooks.arm(event, move || captured.reset());
        }

        let outcome = Pin::new(&mut driver).poll(&mut Context::from_waker(&custom));

        if event == Event::Clone {
            assert!(matches!(outcome, Poll::Ready(Err(_))));

            assert!(matches!(&*store.state(), ActionSnapshot::Idle));
            continue;
        }

        assert!(outcome.is_pending());
        let captured = store.clone();
        hooks.arm(event, move || captured.reset());

        if event == Event::Wake {
            caller.cancel(None);
        }

        assert!(matches!(
            Pin::new(&mut driver).poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(Err(_))
        ));

        assert!(matches!(&*store.state(), ActionSnapshot::Idle));
    }
}
