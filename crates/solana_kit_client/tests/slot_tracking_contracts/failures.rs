//! Failures remain observable without terminating the other source.

use super::*;
use solana_kit_client::reactive::slot_tracking::SlotTrackingStream;

pub fn errors_are_nonterminal_and_mapping_advances_watermark() {
    source_errors();
    initial_mapper_failure();
    subscription_mapper_failure();
}

fn source_errors() {
    let (initial, initial_control) = initial::<i32>();
    let (subscription, subscription_control) = subscription::<i32>();
    *initial_control.state.borrow_mut() = Some(Err(Failure::Initial));
    subscription_control
        .queue
        .borrow_mut()
        .push_back(Err(Failure::Subscription));
    let mut stream = SlotTrackingStream::new(initial, subscription, Ok, Ok);
    assert_eq!(
        next(&mut stream),
        Poll::Ready(Some(Err(Failure::Subscription)))
    );
    assert_eq!(next(&mut stream), Poll::Ready(Some(Err(Failure::Initial))));

    subscription_control
        .queue
        .borrow_mut()
        .push_back(Ok(response(0, 1)));
    assert_eq!(value(&mut stream), response(0, 1));
    subscription_control.closed.set(true);
    assert_eq!(next(&mut stream), Poll::Ready(None));
}

fn initial_mapper_failure() {
    let (initial, initial_control) = initial::<i32>();
    let (subscription, subscription_control) = subscription::<i32>();
    *initial_control.state.borrow_mut() = Some(Ok(response(2, 1)));
    let mut stream = SlotTrackingStream::new(
        initial,
        subscription,
        |_| Err::<i32, _>(Failure::InitialMapper),
        Ok,
    );
    assert_eq!(
        next(&mut stream),
        Poll::Ready(Some(Err(Failure::InitialMapper)))
    );
    subscription_control
        .queue
        .borrow_mut()
        .extend([Ok(response(1, 1)), Ok(response(2, 2))]);
    assert!(next(&mut stream).is_pending());
    assert_eq!(value(&mut stream), response(2, 2));
}

fn subscription_mapper_failure() {
    let (initial, initial_control) = initial::<i32>();
    let (subscription, subscription_control) = subscription::<i32>();
    *initial_control.state.borrow_mut() = Some(Ok(response(1, 1)));
    subscription_control
        .queue
        .borrow_mut()
        .push_back(Ok(response(2, -1)));
    let maps = Rc::new(Cell::new(0));
    let mapped = maps.clone();
    let mut stream = SlotTrackingStream::new(initial, subscription, Ok, move |value| {
        mapped.set(mapped.get() + 1);
        if value < 0 {
            Err(Failure::SubscriptionMapper)
        } else {
            Ok(value)
        }
    });
    assert_eq!(
        next(&mut stream),
        Poll::Ready(Some(Err(Failure::SubscriptionMapper)))
    );
    assert_eq!(maps.get(), 1);

    subscription_control
        .queue
        .borrow_mut()
        .extend([Ok(response(1, -1)), Ok(response(2, 2))]);
    assert!(next(&mut stream).is_pending());
    assert_eq!(maps.get(), 1);
    assert_eq!(value(&mut stream), response(2, 2));
    assert_eq!(maps.get(), 2);
}
