use super::support::{Fixture, poll, response};
use solana_kit_client::reactive::StreamSnapshot;
use std::{cell::Cell, rc::Rc, task::Poll};

#[cfg_attr(test, test)]
pub fn listener_poll_and_destructor_reentry_preserve_lifecycle_ownership() {
    let fixture = Fixture::new();
    let captured = fixture.store.clone();
    let lease = fixture.store.subscribe_local(move || {
        if matches!(&*captured.state(), StreamSnapshot::Loading { .. }) {
            captured.reset();
        }
    });
    let mut prevented = fixture.store.connect();

    assert!(fixture.initial.borrow().is_empty());

    assert!(fixture.subscription.borrow().is_empty());

    assert!(matches!(poll(&mut prevented), Poll::Ready(Err(_))));
    drop(lease);
    let mut polling = fixture.store.connect();
    let immediate = Rc::new(Cell::new(false));
    let observed = immediate.clone();
    let tracked = fixture.subscription(0);
    let captured = fixture.store.clone();
    fixture.subscription(0).on_poll(move || {
        captured.reset();
        observed.set(tracked.signal.is_cancelled());
    });

    assert!(matches!(poll(&mut polling), Poll::Ready(Err(_))));

    assert!(immediate.get());

    assert_eq!(fixture.subscription(0).drops(), 1);

    assert_eq!(fixture.initial(0).drops(), 1);
    let mut dropping = fixture.store.connect();
    let captured = fixture.store.clone();
    fixture.initial(1).on_drop(move || captured.reset());
    fixture.initial(1).push(Ok(response(10, 10)));

    assert!(matches!(poll(&mut dropping), Poll::Ready(Err(_))));

    assert!(matches!(&*fixture.store.state(), StreamSnapshot::Idle));

    assert!(fixture.initial_maps.borrow().is_empty());

    assert_eq!(fixture.subscription(1).drops(), 1);
}
