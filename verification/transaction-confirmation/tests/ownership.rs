//! Actual Pending/wake/drop and bounded explicit close through generic callers.
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures::{
    Stream,
    channel::mpsc::{UnboundedReceiver, unbounded},
    task::LocalSpawnExt,
};
use solana_kit_client::confirmation::{
    Confirmation, Event, Failure, Observer, Policy, WaitError, wait,
};
use solana_kit_core::{Hash, Signature, transaction::Lifetime};
use std::{
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

#[derive(Default)]
struct Counts {
    polls: AtomicUsize,
    receives_dropped: AtomicUsize,
    closes: AtomicUsize,
    drops: AtomicUsize,
}
struct Channel {
    receiver: UnboundedReceiver<Event>,
    counts: Arc<Counts>,
    close_fails: bool,
}
struct Receive<'a> {
    channel: &'a mut Channel,
}
impl Future for Receive<'_> {
    type Output = Result<Event, Failure>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.channel.counts.polls.fetch_add(1, Ordering::SeqCst);
        Pin::new(&mut self.channel.receiver)
            .poll_next(cx)
            .map(|value| value.ok_or(Failure::ObserverEnded))
    }
}
impl Drop for Receive<'_> {
    fn drop(&mut self) {
        self.channel
            .counts
            .receives_dropped
            .fetch_add(1, Ordering::SeqCst);
    }
}
struct Close {
    counts: Arc<Counts>,
    pending: bool,
    fail: bool,
}
impl Future for Close {
    type Output = Result<(), Failure>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        assert!(self.counts.receives_dropped.load(Ordering::SeqCst) > 0);

        if self.pending {
            self.pending = false;
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        self.counts.closes.fetch_add(1, Ordering::SeqCst);
        Poll::Ready(if self.fail {
            Err(Failure::ObserverEnded)
        } else {
            Ok(())
        })
    }
}
impl Observer for Channel {
    type Next<'a> = Receive<'a>;
    type Close<'a> = Close;
    fn next(&mut self) -> Self::Next<'_> {
        Receive { channel: self }
    }
    fn close(&mut self) -> Self::Close<'_> {
        Close {
            counts: self.counts.clone(),
            pending: true,
            fail: self.close_fails,
        }
    }
}
impl Drop for Channel {
    fn drop(&mut self) {
        self.counts.drops.fetch_add(1, Ordering::SeqCst);
    }
}
fn confirmation() -> Confirmation {
    Confirmation::new(
        Signature::from_bytes([7; 64]),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([1; 32]),
            last_valid_block_height: 100,
        },
        Policy::default(),
    )
    .unwrap()
}
async fn generic_wait<O: Observer, C: Future<Output = ()>>(
    observer: O,
    cancellation: C,
) -> Result<(), WaitError> {
    wait(confirmation(), observer, cancellation).await
}
fn channel(
    fail: bool,
) -> (
    futures::channel::mpsc::UnboundedSender<Event>,
    Channel,
    Arc<Counts>,
) {
    let (sender, receiver) = unbounded();
    let counts = Arc::new(Counts::default());
    let observer = Channel {
        receiver,
        counts: counts.clone(),
        close_fails: fail,
    };
    (sender, observer, counts)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_send_future_wakes_and_awaits_close() {
    let (sender, observer, counts) = channel(false);
    let task = tokio::spawn(generic_wait(observer, core::future::pending()));

    for _ in 0..10_000 {
        if counts.polls.load(Ordering::SeqCst) > 0 {
            break;
        }
        tokio::task::yield_now().await;
    }

    assert!(counts.polls.load(Ordering::SeqCst) > 0);
    assert!(!task.is_finished());
    sender.unbounded_send(Event::Timeout).unwrap();
    assert!(matches!(
        task.await.unwrap(),
        Err(WaitError::Outcome(Failure::Timeout))
    ));
    assert_eq!(counts.closes.load(Ordering::SeqCst), 1);
    assert_eq!(counts.drops.load(Ordering::SeqCst), 1);
    assert!(sender.unbounded_send(Event::Timeout).is_err());
}

struct Local {
    channel: Channel,
    marker: Rc<()>,
}
struct LocalReceive<'a> {
    receive: Receive<'a>,
    marker: Rc<()>,
}
impl Future for LocalReceive<'_> {
    type Output = Result<Event, Failure>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        assert!(Rc::strong_count(&self.marker) >= 2);
        Pin::new(&mut self.receive).poll(cx)
    }
}
impl Observer for Local {
    type Next<'a> = LocalReceive<'a>;
    type Close<'a> = Close;
    fn next(&mut self) -> Self::Next<'_> {
        LocalReceive {
            receive: self.channel.next(),
            marker: self.marker.clone(),
        }
    }
    fn close(&mut self) -> Self::Close<'_> {
        self.channel.close()
    }
}

#[test]
fn local_rc_survives_pending_and_quiet_cancellation_closes() {
    let (sender, channel, counts) = channel(false);
    let (cancel, cancelled) = futures::channel::oneshot::channel();
    let mut pool = futures::executor::LocalPool::new();
    let task = pool
        .spawner()
        .spawn_local_with_handle(generic_wait(
            Local {
                channel,
                marker: Rc::new(()),
            },
            async {
                let _ = cancelled.await;
            },
        ))
        .unwrap();
    pool.run_until_stalled();
    assert!(counts.polls.load(Ordering::SeqCst) > 0);
    assert_eq!(counts.closes.load(Ordering::SeqCst), 0);
    cancel.send(()).unwrap();
    assert!(matches!(
        pool.run_until(task),
        Err(WaitError::Outcome(Failure::Cancelled))
    ));
    assert_eq!(counts.closes.load(Ordering::SeqCst), 1);
    assert!(sender.unbounded_send(Event::Timeout).is_err());
}

#[test]
fn cleanup_failure_retains_prior_outcome_and_empty_input_fails() {
    let (sender, observer, counts) = channel(true);
    sender.unbounded_send(Event::Timeout).unwrap();
    let result = futures::executor::block_on(generic_wait(observer, core::future::pending()));
    assert!(matches!(
        result,
        Err(WaitError::Cleanup {
            outcome: Err(Failure::Timeout),
            source,
        }) if matches!(*source, Failure::ObserverEnded)
    ));
    assert_eq!(counts.closes.load(Ordering::SeqCst), 1);

    let (sender, observer, _) = channel(false);
    drop(sender);
    let result = futures::executor::block_on(generic_wait(observer, core::future::pending()));
    assert!(matches!(
        result,
        Err(WaitError::Outcome(Failure::ObserverEnded))
    ));
}

#[test]
fn dropping_wait_cancels_locally_without_claiming_awaited_cleanup() {
    let (_sender, observer, counts) = channel(false);
    let mut task = Box::pin(generic_wait(observer, core::future::pending()));
    let waker = futures::task::noop_waker();
    let mut context = Context::from_waker(&waker);
    assert!(task.as_mut().poll(&mut context).is_pending());
    drop(task);
    assert_eq!(counts.drops.load(Ordering::SeqCst), 1);
    assert_eq!(counts.closes.load(Ordering::SeqCst), 0);
    assert_eq!(counts.receives_dropped.load(Ordering::SeqCst), 1);
}

#[test]
fn cancellation_wins_simultaneous_timeout() {
    let (sender, observer, _) = channel(false);
    sender.unbounded_send(Event::Timeout).unwrap();
    let result = futures::executor::block_on(generic_wait(observer, core::future::ready(())));
    assert!(matches!(
        result,
        Err(WaitError::Outcome(Failure::Cancelled))
    ));
}
