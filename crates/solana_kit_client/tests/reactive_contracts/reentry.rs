use super::{
    streams::source,
    support::{action, next, poll},
};
use solana_kit_client::reactive::{ActionSnapshot, StreamSnapshot};
use std::{cell::Cell, rc::Rc, task::Poll};

const UNOBSERVED: usize = usize::MAX;
const COMPLETED: i32 = 7;

#[cfg_attr(test, test)]
pub fn listeners_can_reset_and_unsubscribe_without_source_io_or_cycles() {
    let (store, calls) = action();
    let captured = store.clone();
    let lease = store.subscribe_local(move || {
        if matches!(&*captured.state(), ActionSnapshot::Running { .. }) {
            captured.reset();
        }
    });
    let mut driver = store.dispatch(());

    assert!(calls.borrow().is_empty());

    assert!(matches!(poll(&mut driver), Poll::Ready(Err(_))));
    lease.unsubscribe();
    lease.unsubscribe();
    let mut running = store.dispatch(());
    calls.borrow()[0].complete(Ok(1));

    assert!(matches!(poll(&mut running), Poll::Ready(Ok(_))));
    let weak_calls = Rc::downgrade(&calls);
    drop((calls, store, driver, running));

    assert!(weak_calls.upgrade().is_some());
    drop(lease);

    assert!(weak_calls.upgrade().is_none());
}

#[cfg_attr(test, test)]
pub fn future_poll_and_destructor_reentry_cannot_restore_or_commit_stale_work() {
    let (store, calls) = action();
    let mut driver = store.dispatch(());
    let control = calls.borrow()[0].clone();
    let during_poll = Rc::new(Cell::new(UNOBSERVED));
    let observed = during_poll.clone();
    let captured = store.clone();
    let tracked = control.clone();
    control.on_poll(move || {
        captured.reset();
        observed.set(tracked.drops());
    });

    assert!(matches!(poll(&mut driver), Poll::Ready(Err(_))));

    assert_eq!(during_poll.get(), 0);

    assert_eq!(control.drops(), 1);

    assert!(matches!(&*store.state(), ActionSnapshot::Idle));
    let mut completed = store.dispatch(());
    let control = calls.borrow()[1].clone();
    let captured = store.clone();
    control.on_drop(move || captured.reset());
    control.complete(Ok(COMPLETED));

    assert!(matches!(poll(&mut completed), Poll::Ready(Err(_))));

    assert!(matches!(&*store.state(), ActionSnapshot::Idle));

    assert_eq!(control.drops(), 1);
}

#[cfg_attr(test, test)]
pub fn stream_listeners_and_local_predicates_can_reset_reentrantly() {
    let (store, controls) = source();
    let captured = store.clone();
    let lease = store.subscribe_local(move || {
        if matches!(&*captured.state(), StreamSnapshot::Loading { .. }) {
            captured.reset();
        }
    });
    let mut prevented = store.connect();

    assert!(controls.borrow().is_empty());

    assert!(matches!(poll(&mut prevented), Poll::Ready(Err(_))));
    drop(lease);
    let mut driver = store.connect();

    assert!(poll(&mut driver).is_pending());
    controls.borrow()[0].0.push(Ok(7));

    assert!(poll(&mut driver).is_pending());
    let captured = store.clone();
    let first = Rc::new(Cell::new(true));
    let mut bridge = store.observe_filtered(None, move |_| {
        if first.replace(false) {
            captured.reset();
        }

        Ok::<_, ()>(true)
    });

    assert!(matches!(&*store.state(), StreamSnapshot::Idle));

    assert!(next(&mut bridge).is_pending());
    let mut replacement = store.connect();

    assert!(poll(&mut replacement).is_pending());
    controls.borrow()[1].0.push(Ok(8));

    assert!(poll(&mut replacement).is_pending());

    assert!(matches!(next(&mut bridge), Poll::Ready(Some(Ok(value))) if *value == 8));
}
