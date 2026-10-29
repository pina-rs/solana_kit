use super::support::{Control, Failure, Fixture, poll, response};
use solana_kit_client::reactive::{CancellationReason, ReactiveFailure};
use std::{sync::Arc, task::Poll};

fn shared_failure(control: &Control, expected: &Arc<ReactiveFailure<Failure>>) {
    let reason = control.signal.reason().expect("cancelled child");
    let CancellationReason::Failure(failure) = &*reason else {
        panic!("child lost original failure")
    };

    assert!(Arc::ptr_eq(failure, expected));
}

#[cfg_attr(test, test)]
/// Both failure orders share a body because neither may clone or replace the
/// first error while cancelling the second child.
pub fn first_failure_is_shared_without_cloning_and_ends_both_children() {
    for subscription_fails in [true, false] {
        let fixture = Fixture::new();
        let mut driver = fixture.store.connect();

        if subscription_fails {
            fixture
                .subscription(0)
                .push(Err(Failure("subscription first")));
        } else {
            fixture.initial(0).push(Err(Failure("initial first")));
        }

        let Poll::Ready(Err(failure)) = poll(&mut driver) else {
            panic!("first failure was not terminal")
        };

        assert!(Arc::ptr_eq(
            fixture.store.state().error().unwrap(),
            &failure
        ));
        shared_failure(&fixture.initial(0), &failure);
        shared_failure(&fixture.subscription(0), &failure);

        assert_eq!(fixture.initial(0).drops(), 1);

        assert_eq!(fixture.subscription(0).drops(), 1);
        fixture.initial(0).push(Ok(response(100, 100)));
        fixture.subscription(0).push(Err(Failure("late failure")));

        assert!(Arc::ptr_eq(
            fixture.store.state().error().unwrap(),
            &failure
        ));

        assert!(fixture.store.state().data().is_none());
        if subscription_fails {
            assert_eq!(fixture.initial(0).polls(), 0);
        }
    }
}

#[cfg_attr(test, test)]
pub fn failed_mapping_does_not_advance_the_retained_watermark() {
    const CURRENT: u64 = 20;
    const FAILED: u64 = 30;
    const RECOVERED: u64 = 25;

    let fixture = Fixture::new();
    let mut first = fixture.store.connect();
    fixture.initial(0).push(Ok(response(CURRENT, 1)));

    assert!(poll(&mut first).is_pending());
    let mut mapping = fixture.store.connect();
    fixture.fail_value.set(Some(2));
    fixture.initial(1).push(Ok(response(FAILED, 2)));
    let Poll::Ready(Err(failure)) = poll(&mut mapping) else {
        panic!("mapper failure lost")
    };

    assert!(matches!(&*failure, ReactiveFailure::Source(error) if error.0 == "mapper failed"));

    assert_eq!(fixture.store.state().data().unwrap().context.slot, CURRENT);
    shared_failure(&fixture.initial(1), &failure);
    shared_failure(&fixture.subscription(1), &failure);
    fixture.fail_value.set(None);
    let mut recovered = fixture.store.connect();
    fixture.initial(2).push(Ok(response(RECOVERED, 3)));

    assert!(poll(&mut recovered).is_pending());

    assert_eq!(
        fixture.store.state().data().unwrap().context.slot,
        RECOVERED
    );

    assert!(fixture.store.state().error().is_none());
}
