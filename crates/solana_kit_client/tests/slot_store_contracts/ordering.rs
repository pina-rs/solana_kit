use super::support::{Fixture, poll, response};
use super::support::{Mapped, Store};
use core::{
    pin::Pin,
    task::{Context, Waker},
};
use futures_core::Stream;
use solana_kit_client::methods::Contextual;
use solana_kit_client::reactive::{ReactiveFailure, StreamSnapshot};
use std::{cell::Cell, rc::Rc, sync::Arc, task::Poll};

#[cfg_attr(test, test)]
pub fn subscription_initial_order_equal_slots_and_complete_context() {
    const HIGH: u64 = 20;

    let fixture = Fixture::new();

    assert!(matches!(&*fixture.store.state(), StreamSnapshot::Idle));

    assert!(fixture.initial.borrow().is_empty());

    assert!(fixture.subscription.borrow().is_empty());
    let mut driver = fixture.store.connect();
    fixture.initial(0).push(Ok(response(10, 1)));
    fixture.subscription(0).push(Ok(response(HIGH, 2)));

    assert!(poll(&mut driver).is_pending());
    let snapshot = fixture.store.state();
    let data = snapshot.data().unwrap();

    assert_eq!(data.context, response(HIGH, 0).context);

    assert_eq!(data.value.origin, "subscription");

    assert_eq!(data.value.value, 2);

    assert!(data.value.marker.get());
    let mut observer = fixture.store.observe(None);
    let mut context = Context::from_waker(Waker::noop());
    let Poll::Ready(Some(Ok(observed))) = Pin::new(&mut observer).poll_next(&mut context) else {
        panic!("local composed observer did not seed data")
    };

    assert!(Arc::ptr_eq(&observed, data));

    assert!(fixture.initial_maps.borrow().is_empty());

    assert_eq!(*fixture.subscription_maps.borrow(), [2]);
    fixture.subscription(0).push(Ok(response(HIGH, 3)));

    assert!(poll(&mut driver).is_pending());

    assert_eq!(fixture.store.state().data().unwrap().value.value, 3);

    assert_eq!(*fixture.subscription_maps.borrow(), [2, 3]);
}

#[cfg_attr(test, test)]
pub fn reconnect_stale_completion_reuses_retained_arc_exactly_once() {
    const STALE_INITIAL: u64 = 99;
    const STALE_SUBSCRIPTION: u64 = 98;
    const NEWER: u64 = 101;

    let fixture = failed_window();
    let retained = fixture.store.state().data().unwrap().clone();
    let previous_error = fixture.store.state().error().unwrap().clone();
    let mut next = fixture.store.connect();

    assert!(Arc::ptr_eq(
        fixture.store.state().data().unwrap(),
        &retained
    ));

    assert!(Arc::ptr_eq(
        fixture.store.state().error().unwrap(),
        &previous_error
    ));
    let notifications = Rc::new(Cell::new(0));
    let observed = notifications.clone();
    let _lease = fixture
        .store
        .subscribe_local(move || observed.set(observed.get() + 1));
    fixture.initial(1).push(Ok(response(STALE_INITIAL, 2)));

    assert!(poll(&mut next).is_pending());

    assert_loaded_retained(&fixture.store, &retained);

    assert_eq!(notifications.get(), 1);
    fixture
        .subscription(1)
        .push(Ok(response(STALE_SUBSCRIPTION, 3)));

    assert!(poll(&mut next).is_pending());

    assert_eq!(notifications.get(), 1);

    assert_eq!(*fixture.initial_maps.borrow(), [1]);

    assert!(fixture.subscription_maps.borrow().is_empty());
    fixture.subscription(1).push(Ok(response(NEWER, 4)));

    assert!(poll(&mut next).is_pending());

    assert_eq!(
        fixture.store.state().data().unwrap().context,
        response(NEWER, 0).context
    );

    assert_eq!(notifications.get(), 2);

    assert!(matches!(&*previous_error, ReactiveFailure::Source(_)));
}

fn assert_loaded_retained(store: &Store, retained: &Arc<Contextual<Mapped>>) {
    let snapshot = store.state();

    assert!(matches!(&*snapshot, StreamSnapshot::Loaded { .. }));

    assert!(Arc::ptr_eq(snapshot.data().unwrap(), retained));

    assert!(snapshot.error().is_none());
}

fn failed_window() -> Fixture {
    let fixture = Fixture::new();
    let mut driver = fixture.store.connect();
    fixture.initial(0).push(Ok(response(100, 1)));

    assert!(poll(&mut driver).is_pending());
    fixture
        .subscription(0)
        .push(Err(super::support::Failure("first window failed")));

    assert!(matches!(poll(&mut driver), Poll::Ready(Err(_))));
    fixture
}
