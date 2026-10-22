//! Pure capability contracts: explicit completion, wakeups, errors and teardown.

use futures::channel::oneshot;
use futures::task::{ArcWake, waker};
use solana_kit_async_boundaries::cancellable;
use solana_kit_boundary_contract::{Clock, Entropy};
use std::cell::{Cell, RefCell};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Waker};

#[derive(Default)]
struct WakeCount(AtomicUsize);

impl ArcWake for WakeCount {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.0.fetch_add(1, Ordering::SeqCst);
    }
}

struct InjectedEntropy(RefCell<Option<oneshot::Receiver<Vec<u8>>>>);

struct Fill<'a> {
    receiver: oneshot::Receiver<Vec<u8>>,
    output: &'a mut [u8],
}

impl Future for Fill<'_> {
    type Output = Result<(), &'static str>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let bytes = match Pin::new(&mut self.receiver).poll(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Err(_)) => return Poll::Ready(Err("entropy unavailable")),
            Poll::Ready(Ok(bytes)) => bytes,
        };

        if bytes.len() != self.output.len() {
            return Poll::Ready(Err("entropy length mismatch"));
        }

        self.output.copy_from_slice(&bytes);

        Poll::Ready(Ok(()))
    }
}

impl Entropy for InjectedEntropy {
    type Error = &'static str;
    type Fill<'a> = Fill<'a>;

    fn fill<'a>(&'a self, bytes: &'a mut [u8]) -> Self::Fill<'a> {
        Fill {
            receiver: self.0.borrow_mut().take().expect("one fill per fixture"),
            output: bytes,
        }
    }
}

#[test]
fn entropy_pending_wakes_then_fills_without_ambient_rng() {
    let (sender, receiver) = oneshot::channel();
    let entropy = InjectedEntropy(RefCell::new(Some(receiver)));
    let mut bytes = [0; 4];
    let wakes = Arc::new(WakeCount::default());
    let waker = waker(Arc::clone(&wakes));
    let mut cx = Context::from_waker(&waker);

    {
        let mut fill = std::pin::pin!(entropy.fill(&mut bytes));
        assert_eq!(fill.as_mut().poll(&mut cx), Poll::Pending);
        sender.send(vec![1, 2, 3, 4]).expect("live entropy request");
        assert!(wakes.0.load(Ordering::SeqCst) > 0);
        assert_eq!(fill.as_mut().poll(&mut cx), Poll::Ready(Ok(())));
    }

    assert_eq!(bytes, [1, 2, 3, 4]);
}

#[test]
fn missing_entropy_is_an_error_and_preserves_output() {
    let (sender, receiver) = oneshot::channel();
    let entropy = InjectedEntropy(RefCell::new(Some(receiver)));
    let mut bytes = [9; 4];
    drop(sender);

    assert_eq!(
        futures::executor::block_on(entropy.fill(&mut bytes)),
        Err("entropy unavailable")
    );
    assert_eq!(bytes, [9; 4]);
}

#[test]
fn short_entropy_reply_is_rejected_without_partial_write() {
    let (sender, receiver) = oneshot::channel();
    let entropy = InjectedEntropy(RefCell::new(Some(receiver)));
    let mut bytes = [9; 4];
    sender.send(vec![1]).expect("live entropy request");

    assert_eq!(
        futures::executor::block_on(entropy.fill(&mut bytes)),
        Err("entropy length mismatch")
    );
    assert_eq!(bytes, [9; 4]);
}

struct DeniedClock;

impl Clock for DeniedClock {
    type Error = &'static str;
    type Sleep<'a> = std::future::Ready<Result<(), Self::Error>>;

    fn sleep_until(&self, _tick: u64) -> Self::Sleep<'_> {
        std::future::ready(Err("clock capability denied"))
    }
}

#[test]
fn unavailable_clock_reports_capability_failure() {
    assert_eq!(
        futures::executor::block_on(DeniedClock.sleep_until(1)),
        Err("clock capability denied")
    );
}

#[derive(Default)]
struct ClockState {
    tick: u64,
    waker: Option<Waker>,
}

#[derive(Default)]
struct ManualClock(Rc<RefCell<ClockState>>);

impl ManualClock {
    fn advance(&self, tick: u64) {
        let mut state = self.0.borrow_mut();
        state.tick = tick;

        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

struct Sleep {
    state: Rc<RefCell<ClockState>>,
    deadline: u64,
}

impl Future for Sleep {
    type Output = Result<(), ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.borrow_mut();

        if state.tick >= self.deadline {
            return Poll::Ready(Ok(()));
        }

        state.waker = Some(cx.waker().clone());

        Poll::Pending
    }
}

impl Clock for ManualClock {
    type Error = ();
    type Sleep<'a> = Sleep;

    fn sleep_until(&self, tick: u64) -> Self::Sleep<'_> {
        Sleep {
            state: Rc::clone(&self.0),
            deadline: tick,
        }
    }
}

#[test]
fn only_injected_clock_progress_completes_timer() {
    const DEADLINE: u64 = 10;
    let clock = ManualClock::default();
    let mut sleep = std::pin::pin!(clock.sleep_until(DEADLINE));
    let wakes = Arc::new(WakeCount::default());
    let waker = waker(Arc::clone(&wakes));
    let mut cx = Context::from_waker(&waker);

    assert_eq!(sleep.as_mut().poll(&mut cx), Poll::Pending);
    assert_eq!(sleep.as_mut().poll(&mut cx), Poll::Pending);
    clock.advance(DEADLINE - 1);
    assert_eq!(sleep.as_mut().poll(&mut cx), Poll::Pending);
    clock.advance(DEADLINE);
    assert!(wakes.0.load(Ordering::SeqCst) > 0);
    assert_eq!(sleep.as_mut().poll(&mut cx), Poll::Ready(Ok(())));
}

#[test]
fn cancellation_wakes_quiet_operation_and_rejects_late_completion() {
    let (complete, reply) = oneshot::channel::<()>();
    let (cancel, cancellation) = oneshot::channel();
    let completed = Rc::new(Cell::new(false));
    let observed = Rc::clone(&completed);
    let operation = async move {
        reply.await.expect("completion only before cancellation");
        observed.set(true);
    };

    let mut operation = std::pin::pin!(cancellable(operation, cancellation));
    let wakes = Arc::new(WakeCount::default());
    let waker = waker(Arc::clone(&wakes));
    let mut cx = Context::from_waker(&waker);

    assert_eq!(operation.as_mut().poll(&mut cx), Poll::Pending);
    cancel.send(()).expect("live cancellation receiver");
    assert!(wakes.0.load(Ordering::SeqCst) > 0);
    assert_eq!(operation.as_mut().poll(&mut cx), Poll::Ready(None));
    assert!(complete.send(()).is_err());
    assert!(!completed.get());
}
