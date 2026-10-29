//! Slot ordering, complete contexts, initial polling, and bounded pending values.

use super::*;
use solana_kit_client::reactive::slot_tracking::SlotTrackingStream;
use std::{borrow::ToOwned, format};

pub fn ordered_values_and_contexts() {
    let (initial, initial_control) = initial::<&str>();
    let (subscription, subscription_control) = subscription::<i32>();
    *initial_control.state.borrow_mut() = Some(Ok(response(1, "stale")));
    subscription_control
        .queue
        .borrow_mut()
        .push_back(Ok(response(2, 1)));
    let initial_maps = Rc::new(Cell::new(0));
    let mapped = initial_maps.clone();
    let mut stream = SlotTrackingStream::new(
        initial,
        subscription,
        move |value| {
            mapped.set(mapped.get() + 1);
            Ok(value.to_owned())
        },
        |value| Ok(format!("subscription:{value}")),
    );
    assert_eq!(value(&mut stream), response(2, "subscription:1".to_owned()));
    assert_eq!(initial_maps.get(), 0);

    subscription_control.queue.borrow_mut().extend([
        Ok(response(2, 2)),
        Ok(response(1, 0)),
        Ok(response(3, 3)),
    ]);
    assert_eq!(value(&mut stream), response(2, "subscription:2".to_owned()));
    assert!(next(&mut stream).is_pending());
    assert_eq!(value(&mut stream), response(3, "subscription:3".to_owned()));
    subscription_control.closed.set(true);
    assert_eq!(next(&mut stream), Poll::Ready(None));
    assert_eq!(initial_control.polls.get(), 1);
    assert_eq!(initial_control.drops.get(), 1);
}

pub fn first_round_and_equal_slot_drain() {
    let (initial, initial_control) = initial::<i32>();
    let (subscription, subscription_control) = subscription::<i32>();
    *initial_control.state.borrow_mut() = Some(Ok(response(1, 1)));
    subscription_control
        .queue
        .borrow_mut()
        .push_back(Ok(response(1, 2)));
    let mut stream = SlotTrackingStream::new(initial, subscription, Ok, Ok);
    assert_eq!(initial_control.polls.get(), 0);
    assert_eq!(subscription_control.polls.get(), 0);

    assert_eq!(value(&mut stream), response(1, 2));
    assert_eq!(initial_control.polls.get(), 1);
    assert_eq!(subscription_control.polls.get(), 1);
    subscription_control
        .queue
        .borrow_mut()
        .push_back(Ok(response(2, 3)));
    assert_eq!(value(&mut stream), response(1, 1));
    assert_eq!(subscription_control.polls.get(), 1);
    assert_eq!(value(&mut stream), response(2, 3));
    assert_eq!(subscription_control.polls.get(), 2);
}
