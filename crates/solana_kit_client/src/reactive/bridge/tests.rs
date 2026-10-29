//! Native scheduling control for the capture-to-admission observation race.
use super::observe_captured;
use crate::reactive::{ReactiveStreamConnection, ReactiveStreamStore, StoreStream};
use core::convert::Infallible;
use futures::{Stream, channel::mpsc, stream};
use std::{
    future::{Future, ready},
    pin::Pin,
    sync::{Arc, Barrier, Mutex, Weak},
    task::{Context, Poll, Waker},
    thread,
};

const OLD: i32 = 1;
const NEW: i32 = 2;

fn poll<F: Future + Unpin>(future: &mut F) -> Poll<F::Output> {
    Pin::new(future).poll(&mut Context::from_waker(Waker::noop()))
}

fn assert_latest(bridge: &mut StoreStream<i32, ()>, expected: i32) {
    let mut context = Context::from_waker(Waker::noop());
    let Poll::Ready(Some(Ok(value))) = Pin::new(&mut *bridge).poll_next(&mut context) else {
        panic!("newer buffered observation was lost")
    };

    assert_eq!(*value, expected);

    assert!(Pin::new(bridge).poll_next(&mut context).is_pending());
}

#[test]
fn delayed_old_capture_cannot_replace_a_newer_buffered_observation() {
    let (mut sender, receiver) = mpsc::channel(1);
    let receiver = Mutex::new(Some(receiver));
    let store = ReactiveStreamStore::new(move |_| {
        ready(Ok::<_, ()>(ReactiveStreamConnection {
            data: receiver.lock().unwrap().take().unwrap(),
            errors: stream::empty::<Option<()>>(),
        }))
    });
    let mut driver = store.connect();

    assert!(poll(&mut driver).is_pending());
    sender.try_send(Ok(OLD)).unwrap();

    assert!(poll(&mut driver).is_pending());
    let mut bridge = store.observe(None);
    let core = store.core.clone();
    let buffer = bridge.buffer.clone();
    let captured = Arc::new(Barrier::new(2));
    let resume = Arc::new(Barrier::new(2));
    let captured_worker = captured.clone();
    let resume_worker = resume.clone();

    // A pauses at the exact old-snapshot boundary. B's real source poll and
    // listener must finish buffering NEW before A can attempt admission.
    let delayed = thread::spawn(move || {
        let old_snapshot = core.snapshot();
        captured_worker.wait();
        resume_worker.wait();
        observe_captured(&core, &buffer, old_snapshot, &|_: &i32| {
            Ok::<_, Infallible>(true)
        });
    });
    captured.wait();
    sender.try_send(Ok(NEW)).unwrap();

    assert!(poll(&mut driver).is_pending());
    let newer_invocation = bridge.buffer.lock().unwrap().invocation.clone();
    resume.wait();
    delayed.join().unwrap();
    let final_invocation = bridge.buffer.lock().unwrap().invocation.clone();

    assert!(Arc::ptr_eq(&newer_invocation, &final_invocation));
    let final_seen = bridge.buffer.lock().unwrap().seen.clone().unwrap();

    assert!(Weak::ptr_eq(&final_seen, &Arc::downgrade(&store.state())));
    assert_latest(&mut bridge, NEW);
}
