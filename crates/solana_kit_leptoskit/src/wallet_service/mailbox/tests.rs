//! Real bounded-channel capacity wakes the retained replacement operation.
use super::*;
use leptoskit_bridge::{CallId, MethodId, PluginId, Request};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::task::{Wake, Waker};
use tokio::sync::mpsc;

#[derive(Default)]
struct Wakes(AtomicUsize);

impl Wake for Wakes {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

fn command(call: u64) -> Command {
    Command {
        request: Request {
            id: CallId(call),
            plugin: PluginId(1),
            method: MethodId(0),
            payload: Vec::new().into(),
        },
        epoch: call,
    }
}

#[test]
fn stale_queue_capacity_wakes_and_delivers_exactly_one_replacement() {
    let (sender, mut receiver) = mpsc::channel(1);
    let mut mailbox = Mailbox::new(sender);
    let wakes = Arc::new(Wakes::default());
    let waker = Waker::from(wakes.clone());
    let mut context = Context::from_waker(&waker);
    mailbox.send(command(1)).unwrap();
    mailbox.send(command(2)).unwrap();
    assert_eq!(mailbox.send(command(3)), Err(Error::Busy));
    mailbox.flush(&mut context).unwrap();
    assert_eq!(receiver.try_recv().unwrap().request.id, CallId(1));
    assert!(wakes.0.load(Ordering::Relaxed) > 0);
    mailbox.flush(&mut context).unwrap();
    assert_eq!(receiver.try_recv().unwrap().request.id, CallId(2));
    assert!(matches!(
        receiver.try_recv(),
        Err(mpsc::error::TryRecvError::Empty)
    ));
}

#[test]
fn cancellation_discards_a_capacity_waiter_before_admitting_its_successor() {
    let (sender, mut receiver) = mpsc::channel(1);
    let mut mailbox = Mailbox::new(sender);
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    mailbox.send(command(1)).unwrap();
    mailbox.send(command(2)).unwrap();
    mailbox.flush(&mut context).unwrap();
    mailbox.clear();
    mailbox.send(command(3)).unwrap();
    mailbox.flush(&mut context).unwrap();
    assert_eq!(receiver.try_recv().unwrap().request.id, CallId(1));
    mailbox.flush(&mut context).unwrap();
    assert_eq!(receiver.try_recv().unwrap().request.id, CallId(3));
    assert!(matches!(
        receiver.try_recv(),
        Err(mpsc::error::TryRecvError::Empty)
    ));
}

#[test]
fn a_closed_worker_reports_native_failure_instead_of_backpressure() {
    let (sender, receiver) = mpsc::channel(1);
    drop(receiver);
    assert_eq!(Mailbox::new(sender).send(command(1)), Err(Error::Native));
}
