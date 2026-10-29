use super::support::{Failure, Fixture, poll, response};
use solana_kit_client::reactive::{CancellationSource, ReactiveFailure, StreamSnapshot};
use std::{sync::Arc, task::Poll};

#[cfg_attr(test, test)]
pub fn fresh_sources_reset_dispose_drop_and_quiet_completion() {
    let fixture = Fixture::new();
    let mut old = fixture.store.connect();
    let mut driver = fixture.store.connect();

    assert_eq!(fixture.initial.borrow().len(), 2);

    assert_eq!(fixture.subscription.borrow().len(), 2);

    assert!(fixture.initial(0).signal.is_cancelled());

    assert_eq!(fixture.subscription(0).drops(), 1);

    assert!(matches!(poll(&mut old), Poll::Ready(Err(_))));
    fixture.initial(1).push(Ok(response(100, 100)));
    fixture.subscription(1).finish();

    assert!(poll(&mut driver).is_pending());

    assert!(poll(&mut driver).is_pending());

    assert!(!fixture.initial(1).signal.is_cancelled());
    fixture.store.reset();

    assert!(matches!(&*fixture.store.state(), StreamSnapshot::Idle));

    assert!(fixture.initial(1).signal.is_cancelled());
    let mut reset = fixture.store.connect();
    fixture.initial(2).push(Ok(response(1, 1)));

    assert!(poll(&mut reset).is_pending());

    assert_eq!(fixture.store.state().data().unwrap().context.slot, 1);
    drop(reset);

    assert_eq!(fixture.subscription(2).drops(), 1);
    let disposed = fixture.store.connect();
    fixture.store.dispose();
    fixture.store.dispose();

    assert_eq!(fixture.initial(3).drops(), 1);

    assert_eq!(fixture.subscription(3).drops(), 1);
    drop(disposed);
    let mut rejected = fixture.store.connect();

    let Poll::Ready(Err(error)) = poll(&mut rejected) else {
        panic!("disposed store started sources")
    };

    assert!(matches!(&*error, ReactiveFailure::Disposed));

    assert_eq!(fixture.initial.borrow().len(), 4);
}

#[cfg_attr(test, test)]
pub fn caller_cancel_prevents_initial_io_and_wins_ready_results() {
    let fixture = Fixture::new();
    let caller = CancellationSource::new();
    caller.cancel(Some(Failure("already cancelled")));
    let mut rejected = fixture.store.connect_with_signal(caller.token());

    assert!(fixture.initial.borrow().is_empty());

    assert!(fixture.subscription.borrow().is_empty());

    assert!(matches!(poll(&mut rejected), Poll::Ready(Err(_))));
    let caller = CancellationSource::new();
    let mut driver = fixture.store.connect_with_signal(caller.token());
    fixture.initial(0).push(Ok(response(10, 10)));
    caller.cancel(Some(Failure("caller stopped")));
    let Poll::Ready(Err(failure)) = poll(&mut driver) else {
        panic!("caller cancellation lost")
    };

    assert!(Arc::ptr_eq(
        fixture.store.state().error().unwrap(),
        &failure
    ));

    assert_eq!(fixture.initial(0).polls(), 0);

    assert_eq!(fixture.subscription(0).polls(), 0);

    assert_eq!(fixture.initial(0).drops(), 1);

    assert_eq!(fixture.subscription(0).drops(), 1);
    let mut recovered = fixture.store.connect();
    fixture.initial(1).push(Ok(response(1, 1)));

    assert!(poll(&mut recovered).is_pending());

    assert_eq!(fixture.store.state().data().unwrap().context.slot, 1);
}
