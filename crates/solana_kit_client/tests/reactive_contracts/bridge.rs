use super::{
    streams::source,
    support::{next, poll},
};
use futures_core::Stream;
use solana_kit_client::reactive::{BridgeFailure, CancellationSource, StreamSnapshot};
use std::{
    cell::Cell,
    pin::Pin,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

#[cfg_attr(test, test)]
/// The rejected last value and unpolled burst share a fixture because both must
/// retain the newest accepted value without preserving intermediate history.
pub fn bridge_seeds_and_coalesces_only_accepted_values() {
    let (store, controls) = source();
    let mut driver = store.connect();

    assert!(poll(&mut driver).is_pending());
    controls.borrow()[0].0.push(Ok(2));

    assert!(poll(&mut driver).is_pending());
    let mut bridge = store.observe_filtered(None, |value| Ok::<_, &'static str>(value % 2 == 0));

    assert!(matches!(next(&mut bridge), Poll::Ready(Some(Ok(value))) if *value == 2));

    controls.borrow()[0].0.push(Ok(4));
    controls.borrow()[0].0.push(Ok(6));
    controls.borrow()[0].0.push(Ok(7));

    assert!(poll(&mut driver).is_pending());
    assert!(poll(&mut driver).is_pending());
    assert!(poll(&mut driver).is_pending());

    assert!(matches!(next(&mut bridge), Poll::Ready(Some(Ok(value))) if *value == 6));

    assert!(next(&mut bridge).is_pending());

    // Keeping the observer unpolled makes every intermediate accepted value
    // obsolete; the final pull must still return the newest accepted value.
    for value in 0..128 {
        controls.borrow()[0].0.push(Ok(value));

        assert!(poll(&mut driver).is_pending());
    }

    assert!(matches!(next(&mut bridge), Poll::Ready(Some(Ok(value))) if *value == 126));
    drop(bridge);

    assert!(matches!(&*store.state(), StreamSnapshot::Loaded { .. }));

    assert_eq!(controls.borrow()[0].0.drops(), 0);
}

#[cfg_attr(test, test)]
pub fn bridge_retains_error_ahead_of_buffered_data_and_later_recovery() {
    let (store, controls) = source();
    let mut driver = store.connect();

    assert!(poll(&mut driver).is_pending());
    let mut bridge = store.observe(None);
    controls.borrow()[0].0.push(Ok(1));

    assert!(poll(&mut driver).is_pending());
    controls.borrow()[0].1.push(Some("terminal observer error"));

    assert!(poll(&mut driver).is_pending());
    controls.borrow()[0].0.push(Ok(2));

    assert!(poll(&mut driver).is_pending());

    let Poll::Ready(Some(Err(error))) = next(&mut bridge) else {
        panic!("buffered error lost")
    };

    assert!(matches!(&*error, BridgeFailure::Store(_)));

    assert!(matches!(next(&mut bridge), Poll::Ready(None)));

    assert!(matches!(&*store.state(), StreamSnapshot::Loaded { .. }));
    let mut seeded_error = store.observe(None);
    controls.borrow()[0].1.push(Some("second error"));

    assert!(poll(&mut driver).is_pending());

    assert!(matches!(next(&mut seeded_error), Poll::Ready(Some(Err(_)))));
}

#[cfg_attr(test, test)]
pub fn fallible_local_predicate_and_drop_release_the_listener() {
    let (store, controls) = source();
    let mut driver = store.connect();

    assert!(poll(&mut driver).is_pending());
    let calls = Rc::new(Cell::new(0));
    let observed = calls.clone();
    let mut bridge = store.observe_filtered(None, move |_| {
        observed.set(observed.get() + 1);
        Err::<bool, _>("predicate failed")
    });
    controls.borrow()[0].0.push(Ok(1));

    assert!(poll(&mut driver).is_pending());

    let Poll::Ready(Some(Err(error))) = next(&mut bridge) else {
        panic!("predicate error lost")
    };

    assert!(matches!(
        &*error,
        BridgeFailure::Predicate("predicate failed")
    ));
    controls.borrow()[0].0.push(Ok(2));

    assert!(poll(&mut driver).is_pending());

    assert_eq!(calls.get(), 1);
    let observed = calls.clone();
    let bridge = store.observe_filtered(None, move |_| {
        observed.set(observed.get() + 1);
        Ok::<_, ()>(true)
    });

    assert_eq!(calls.get(), 2);
    drop(bridge);
    controls.borrow()[0].0.push(Ok(3));

    assert!(poll(&mut driver).is_pending());

    assert_eq!(calls.get(), 2);
}

struct WakeCount(AtomicUsize);

impl Wake for WakeCount {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[cfg_attr(test, test)]
pub fn cancellation_wakes_parked_bridge_and_wins_initial_snapshot() {
    let (store, controls) = source();
    let mut driver = store.connect();

    assert!(poll(&mut driver).is_pending());
    controls.borrow()[0].0.push(Ok(1));

    assert!(poll(&mut driver).is_pending());
    let caller = CancellationSource::new();
    let mut initial = store.observe(Some(caller.token()));
    caller.cancel(None);

    assert!(matches!(next(&mut initial), Poll::Ready(None)));
    let caller = CancellationSource::new();
    store.reset();
    let mut bridge = store.observe(Some(caller.token()));
    let counter = Arc::new(WakeCount(AtomicUsize::new(0)));
    let waker = Waker::from(counter.clone());

    assert!(
        Pin::new(&mut bridge)
            .poll_next(&mut Context::from_waker(&waker))
            .is_pending()
    );
    store.dispose();

    assert!(
        Pin::new(&mut bridge)
            .poll_next(&mut Context::from_waker(&waker))
            .is_pending()
    );
    caller.cancel(None);

    assert!(counter.0.load(Ordering::SeqCst) > 0);

    assert!(matches!(next(&mut bridge), Poll::Ready(None)));
}

const DATA_VALUE: i32 = 7;
const ERROR_VALUE: i32 = 8;

struct LocalValue(Rc<Cell<i32>>);
struct LocalError(Rc<Cell<i32>>);

#[cfg_attr(test, test)]
pub fn local_payloads_and_errors_require_neither_send_nor_clone() {
    use super::support::Queue;
    use solana_kit_client::reactive::{
        LocalReactiveStreamStore, ReactiveFailure, ReactiveStreamConnection,
    };
    use std::{cell::RefCell, future::ready};

    let (data, data_control) = Queue::new();
    let (errors, error_control) = Queue::new();
    let connection = Rc::new(RefCell::new(Some(ReactiveStreamConnection {
        data,
        errors,
    })));
    let store = LocalReactiveStreamStore::new_local(move |_| {
        ready(Ok::<_, LocalError>(connection.borrow_mut().take().unwrap()))
    });
    let mut driver = store.connect();

    assert!(poll(&mut driver).is_pending());
    let payload = Rc::new(Cell::new(DATA_VALUE));
    data_control.push(Ok(LocalValue(payload.clone())));

    assert!(poll(&mut driver).is_pending());
    let mut bridge = store.observe(None);

    assert!(
        matches!(next(&mut bridge), Poll::Ready(Some(Ok(value))) if Rc::ptr_eq(&value.0, &payload))
    );
    let cause = Rc::new(Cell::new(ERROR_VALUE));
    error_control.push(Some(LocalError(cause.clone())));

    assert!(poll(&mut driver).is_pending());

    let snapshot = store.state();
    let ReactiveFailure::Source(error) = &**snapshot.error().unwrap() else {
        panic!("local cause lost")
    };

    assert!(Rc::ptr_eq(&error.0, &cause));
}
