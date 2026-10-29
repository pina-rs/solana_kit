use super::support::{action, poll};
use solana_kit_client::reactive::{
    ActionSnapshot, CancellationReason, CancellationSource, ReactiveFailure,
};
use std::{cell::Cell, rc::Rc, sync::Arc, task::Poll};

#[cfg_attr(test, test)]
pub fn refresh_retains_result_and_error_until_success() {
    let (store, calls) = action();
    let mut first = store.dispatch(());
    calls.borrow()[0].complete(Ok(7));

    assert!(matches!(poll(&mut first), Poll::Ready(Ok(value)) if *value == 7));

    let mut failing = store.dispatch(());

    assert_eq!(store.state().result().map(|value| **value), Some(7));
    calls.borrow()[1].complete(Err("source failed"));

    assert!(matches!(poll(&mut failing), Poll::Ready(Err(_))));
    let previous_error = store.state().error().unwrap().clone();
    let mut refreshed = store.dispatch(());

    assert!(Arc::ptr_eq(store.state().error().unwrap(), &previous_error));
    calls.borrow()[2].complete(Ok(9));

    assert!(matches!(poll(&mut refreshed), Poll::Ready(Ok(value)) if *value == 9));

    assert!(store.state().error().is_none());

    assert_eq!(calls.borrow()[2].drops(), 1);
}

#[cfg_attr(test, test)]
pub fn supersession_and_reset_drop_work_before_late_producers_complete() {
    let (store, calls) = action();
    let notifications = Rc::new(Cell::new(0));
    let observed = notifications.clone();
    let _lease = store.subscribe_local(move || observed.set(observed.get() + 1));
    store.reset();

    assert_eq!(notifications.get(), 0);
    let mut obsolete = store.dispatch(());
    let mut winner = store.dispatch(());

    assert_eq!(notifications.get(), 1);

    assert_eq!(calls.borrow()[0].drops(), 1);
    calls.borrow()[0].complete(Err("late loser"));

    let Poll::Ready(Err(error)) = poll(&mut obsolete) else {
        panic!("obsolete dispatch did not cancel")
    };
    let ReactiveFailure::Cancelled(reason) = &*error else {
        panic!("missing cancellation reason")
    };

    assert!(matches!(&**reason, CancellationReason::Superseded));
    calls.borrow()[1].complete(Ok(11));

    assert!(matches!(poll(&mut winner), Poll::Ready(Ok(value)) if *value == 11));
    let mut pending = store.dispatch(());
    store.reset();

    assert_eq!(calls.borrow()[2].drops(), 1);
    calls.borrow()[2].complete(Ok(99));

    assert!(matches!(poll(&mut pending), Poll::Ready(Err(_))));

    assert!(matches!(&*store.state(), ActionSnapshot::Idle));
}

#[cfg_attr(test, test)]
pub fn caller_cancellation_wins_ready_completion_and_prevents_initial_io() {
    let (store, calls) = action();
    let caller = CancellationSource::new();
    caller.cancel(Some("already stopped"));
    let mut rejected = store.dispatch_with_signal((), caller.token());

    assert!(calls.borrow().is_empty());

    assert!(matches!(poll(&mut rejected), Poll::Ready(Err(_))));
    let caller = CancellationSource::new();
    let mut pending = store.dispatch_with_signal((), caller.token());
    calls.borrow()[0].complete(Ok(1));
    caller.cancel(Some("caller stopped"));

    let Poll::Ready(Err(error)) = poll(&mut pending) else {
        panic!("caller cancellation lost")
    };
    let ReactiveFailure::Cancelled(reason) = &*error else {
        panic!("missing cancellation reason")
    };
    let CancellationReason::Caller(Some(cause)) = &**reason else {
        panic!("original cause lost")
    };

    assert_eq!(**cause, "caller stopped");

    assert_eq!(calls.borrow()[0].polls(), 0);

    assert_eq!(calls.borrow()[0].drops(), 1);
    let mut recovered = store.dispatch(());
    calls.borrow()[1].complete(Ok(5));

    assert!(matches!(poll(&mut recovered), Poll::Ready(Ok(_))));
}

#[cfg_attr(test, test)]
pub fn dropped_and_disposed_drivers_release_actual_owned_work() {
    let (store, calls) = action();
    let pending = store.dispatch(());
    drop(pending);

    assert_eq!(calls.borrow()[0].drops(), 1);

    assert!(matches!(&*store.state(), ActionSnapshot::Error { .. }));
    let mut pending = store.dispatch(());
    store.dispose();
    store.dispose();

    assert_eq!(calls.borrow()[1].drops(), 1);

    assert!(matches!(poll(&mut pending), Poll::Ready(Err(_))));
    store.reset();
    let mut rejected = store.dispatch(());

    let Poll::Ready(Err(error)) = poll(&mut rejected) else {
        panic!("disposed dispatch ran")
    };

    assert!(matches!(&*error, ReactiveFailure::Disposed));

    assert_eq!(calls.borrow().len(), 2);
}
