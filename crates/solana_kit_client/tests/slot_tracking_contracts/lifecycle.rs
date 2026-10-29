//! Completion, local ownership, and stale-value progress remain caller controlled.

use super::*;
use futures_core::stream::FusedStream;
use solana_kit_client::reactive::slot_tracking::SlotTrackingStream;
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::Wake,
};

pub fn completion_orders_and_queued_initial() {
    subscription_finishes_first();
    initial_finishes_first();
}

fn subscription_finishes_first() {
    let (initial, initial_control) = initial::<i32>();
    let (subscription, subscription_control) = subscription::<i32>();
    subscription_control.closed.set(true);
    let mut stream = SlotTrackingStream::new(initial, subscription, Ok, Ok);
    assert!(next(&mut stream).is_pending());
    assert!(!stream.is_terminated());
    *initial_control.state.borrow_mut() = Some(Ok(response(1, 10)));
    assert_eq!(value(&mut stream), response(1, 10));
    assert!(stream.is_terminated());

    assert_eq!(next(&mut stream), Poll::Ready(None));
    assert_eq!(next(&mut stream), Poll::Ready(None));
    assert_eq!(initial_control.polls.get(), 2);
    assert_eq!(subscription_control.polls.get(), 1);
}

fn initial_finishes_first() {
    let (initial, initial_control) = initial::<i32>();
    let (subscription, subscription_control) = subscription::<i32>();
    let queued = Contextual {
        context: RpcContext {
            slot: 0,
            api_version: Presence::Missing,
        },
        value: 10,
    };
    *initial_control.state.borrow_mut() = Some(Ok(queued));
    let mut stream = SlotTrackingStream::new(initial, subscription, Ok, Ok);
    let output = value(&mut stream);
    assert_eq!(output.context.api_version, Presence::Missing);
    assert_eq!(output.context.slot, 0);
    assert_eq!(output.value, 10);
    assert!(!stream.is_terminated());

    subscription_control
        .queue
        .borrow_mut()
        .push_back(Ok(response(1, 2)));
    assert_eq!(value(&mut stream), response(1, 2));
    subscription_control.closed.set(true);
    assert_eq!(next(&mut stream), Poll::Ready(None));
    assert!(stream.is_terminated());
    assert_eq!(initial_control.polls.get(), 1);
}

pub fn drop_local_payloads_and_stale_wakeup() {
    unpolled_and_parked_drop(false);
    unpolled_and_parked_drop(true);
    local_buffer_drop();
    stale_wakeup();
}

fn unpolled_and_parked_drop(poll: bool) {
    let (initial, initial_control) = initial::<i32>();
    let (subscription, subscription_control) = subscription::<i32>();
    let mut stream = SlotTrackingStream::new(initial, subscription, Ok, Ok);

    if poll {
        assert!(next(&mut stream).is_pending());
        assert_eq!(initial_control.polls.get(), 1);
        assert_eq!(subscription_control.polls.get(), 1);
    }

    drop(stream);
    assert_eq!(initial_control.drops.get(), 1);
    assert_eq!(subscription_control.drops.get(), 1);
}

fn local_buffer_drop() {
    let payload = Rc::new(Cell::new(10));
    let (initial, initial_control) = initial::<Rc<Cell<i32>>>();
    let (subscription, subscription_control) = subscription::<Rc<Cell<i32>>>();
    *initial_control.state.borrow_mut() = Some(Ok(response(1, payload.clone())));
    subscription_control
        .queue
        .borrow_mut()
        .push_back(Ok(response(1, payload.clone())));
    let mapper_calls = Rc::new(Cell::new(0));
    let calls = mapper_calls.clone();
    let mut stream = SlotTrackingStream::new(
        initial,
        subscription,
        move |value| {
            calls.set(calls.get() + 1);
            Ok(value)
        },
        Ok,
    );
    let current = value(&mut stream);
    assert!(Rc::ptr_eq(&current.value, &payload));
    assert_eq!(mapper_calls.get(), 1);

    assert_eq!(Rc::strong_count(&payload), 3);
    drop(current);
    assert_eq!(Rc::strong_count(&payload), 2);
    drop(stream);
    assert_eq!(Rc::strong_count(&payload), 1);
    assert_eq!(initial_control.drops.get(), 1);
    assert_eq!(subscription_control.drops.get(), 1);
}

#[derive(Default)]
struct WakeCounter(AtomicUsize);

impl Wake for WakeCounter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn stale_wakeup() {
    let (initial, _) = initial::<i32>();
    let (subscription, control) = subscription::<i32>();
    control
        .queue
        .borrow_mut()
        .extend([Ok(response(u64::MAX, 1)), Ok(response(0, 2))]);
    let mut stream = SlotTrackingStream::new(initial, subscription, Ok, Ok);
    assert_eq!(value(&mut stream), response(u64::MAX, 1));
    let wake_counter = Arc::new(WakeCounter::default());
    let waker = Waker::from(wake_counter.clone());
    let mut context = Context::from_waker(&waker);
    assert!(Pin::new(&mut stream).poll_next(&mut context).is_pending());
    assert_eq!(wake_counter.0.load(Ordering::SeqCst), 1);

    control
        .queue
        .borrow_mut()
        .push_back(Ok(response(u64::MAX, 3)));
    assert_eq!(value(&mut stream), response(u64::MAX, 3));
}
