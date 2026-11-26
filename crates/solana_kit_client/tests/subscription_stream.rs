//! Stream and callback consumption preserve one Session poll contract.
use core::ops::ControlFlow;
use futures::{Stream, StreamExt, executor::block_on, stream::FusedStream};
use solana_kit_client::subscriptions::*;
use solana_kit_core::Signature;
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

const GENERATION: u64 = 7;
const ACK_DEADLINE: u64 = 10;
const CLEANUP_TIMEOUT: u64 = 5;

#[derive(Default)]
struct State {
    sent: Vec<String>,
    incoming: VecDeque<String>,
}

struct Socket(Rc<RefCell<State>>);

impl TextSocket for Socket {
    type Error = std::convert::Infallible;

    fn start_send(&mut self, text: &str) -> Result<(), Self::Error> {
        self.0.borrow_mut().sent.push(text.into());
        Ok(())
    }

    fn poll_flush(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_receive(
        &mut self,
        _: &mut Context<'_>,
    ) -> Poll<Result<Option<ReceivedFrame>, Self::Error>> {
        let mut state = self.0.borrow_mut();

        match state.incoming.pop_front() {
            Some(frame) => Poll::Ready(Ok(Some(ReceivedFrame::Text(frame)))),
            None => Poll::Pending,
        }
    }

    fn poll_close(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

fn pair() -> (Session<Socket>, Rc<RefCell<State>>) {
    let state = Rc::new(RefCell::new(State::default()));
    (
        Session::new(Socket(state.clone()), GENERATION, Budget::default()),
        state,
    )
}

fn push(state: &Rc<RefCell<State>>, frame: &str) {
    state.borrow_mut().incoming.push_back(frame.into());
}

fn ack(state: &Rc<RefCell<State>>, id: u32, value: &str) {
    push(
        state,
        &format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{value}}}"#),
    );
}

fn notification(state: &Rc<RefCell<State>>, method: &str, server: u64, result: &str) {
    push(
        state,
        &format!(
            r#"{{"jsonrpc":"2.0","method":"{method}","params":{{"subscription":{server},"result":{result}}}}}"#
        ),
    );
}

fn signature_request() -> SignatureNotifications {
    SignatureNotifications(Signature::from_bytes([0; 64]), None)
}

struct Sleep;

impl Future for Sleep {
    type Output = Result<(), ClockError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

/// Records each requested deadline and never advances.
struct ManualClock {
    now: Rc<Cell<u64>>,
    sleeps: Rc<RefCell<Vec<u64>>>,
}

impl ManualClock {
    fn new() -> Self {
        Self {
            now: Rc::new(Cell::new(0)),
            sleeps: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl Clock for ManualClock {
    type Sleep<'a> = Sleep;

    fn now(&self) -> u64 {
        self.now.get()
    }

    fn sleep_until(&self, deadline: u64) -> Sleep {
        self.sleeps.borrow_mut().push(deadline);
        Sleep
    }
}

struct AdvanceSleep {
    now: Rc<Cell<u64>>,
    deadline: u64,
}

impl Future for AdvanceSleep {
    type Output = Result<(), ClockError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        this.now.set(this.deadline);
        Poll::Ready(Ok(()))
    }
}

/// Advances to the first requested deadline on its initial poll.
struct AdvanceClock {
    now: Rc<Cell<u64>>,
    sleeps: Rc<RefCell<Vec<u64>>>,
}

impl AdvanceClock {
    fn new() -> Self {
        Self {
            now: Rc::new(Cell::new(0)),
            sleeps: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl Clock for AdvanceClock {
    type Sleep<'a> = AdvanceSleep;

    fn now(&self) -> u64 {
        self.now.get()
    }

    fn sleep_until(&self, deadline: u64) -> AdvanceSleep {
        self.sleeps.borrow_mut().push(deadline);
        AdvanceSleep {
            now: self.now.clone(),
            deadline,
        }
    }
}

struct FailureSleep;

impl Future for FailureSleep {
    type Output = Result<(), ClockError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Err(ClockError::Capacity))
    }
}

struct FailureClock;

impl Clock for FailureClock {
    type Sleep<'a> = FailureSleep;

    fn now(&self) -> u64 {
        0
    }

    fn sleep_until(&self, _: u64) -> FailureSleep {
        FailureSleep
    }
}

#[test]
fn acknowledged_idle_stream_has_no_notification_lifetime_timer() {
    let (mut owner, state) = pair();
    let handle = owner.subscribe(Root, ACK_DEADLINE).unwrap();
    ack(&state, 1, "17");
    let clock = ManualClock::new();
    let mut stream = owner.notifications(&handle, &clock, CLEANUP_TIMEOUT);
    let mut context = Context::from_waker(Waker::noop());

    // The first poll acknowledges the subscription. A far-future tick then
    // proves the cleanup timeout does not become a notification deadline.
    assert!(Pin::new(&mut stream).poll_next(&mut context).is_pending());
    clock.now.set(500);
    assert!(Pin::new(&mut stream).poll_next(&mut context).is_pending());
    assert!(clock.sleeps.borrow().is_empty());
    assert!(!stream.is_terminated());

    notification(&state, "rootNotification", 17, "42");
    assert_eq!(block_on(stream.next()).unwrap().unwrap(), 42);
    assert!(!stream.is_terminated());
}

#[test]
fn one_shot_stream_yields_once_then_is_fused() {
    let (mut owner, state) = pair();
    let handle = owner.subscribe(signature_request(), ACK_DEADLINE).unwrap();
    notification(
        &state,
        "signatureNotification",
        17,
        r#"{"context":{"slot":1},"value":{"err":"AccountInUse"}}"#,
    );
    ack(&state, 1, "17");
    let clock = ManualClock::new();
    let mut stream = owner.notifications(&handle, &clock, CLEANUP_TIMEOUT);
    let value = block_on(stream.next()).unwrap().unwrap();
    assert!(matches!(value.value, SignatureInfo::Processed { .. }));
    assert!(block_on(stream.next()).is_none());
    assert!(stream.is_terminated());
    drop(stream);
    owner.forget(handle).unwrap();
}

#[test]
fn callback_continue_ends_after_one_shot_completion() {
    let (mut owner, state) = pair();
    let handle = owner.subscribe(signature_request(), ACK_DEADLINE).unwrap();
    notification(
        &state,
        "signatureNotification",
        17,
        r#"{"context":{"slot":1},"value":{"err":null}}"#,
    );
    ack(&state, 1, "17");
    let clock = ManualClock::new();
    let count = Rc::new(Cell::new(0));
    let observed = count.clone();
    let result = block_on(
        owner.run_callback(&handle, &clock, CLEANUP_TIMEOUT, move |_| {
            observed.set(observed.get() + 1);
            ControlFlow::Continue(())
        }),
    );
    assert!(result.is_ok());
    assert_eq!(count.get(), 1);
    owner.forget(handle).unwrap();
}

#[test]
fn acknowledgement_timer_drives_tick_and_failure_is_yielded_once() {
    let (mut owner, _) = pair();
    let handle = owner.subscribe(Root, ACK_DEADLINE).unwrap();
    let clock = AdvanceClock::new();
    let mut stream = owner.notifications(&handle, &clock, CLEANUP_TIMEOUT);
    let item = block_on(stream.next()).unwrap();
    assert!(matches!(
        item,
        Err(SessionError::Protocol(Failure::Deadline))
    ));
    assert!(block_on(stream.next()).is_none());
    assert!(stream.is_terminated());
    assert_eq!(&*clock.sleeps.borrow(), &[ACK_DEADLINE]);
    drop(stream);
    owner.forget(handle).unwrap();
}

#[test]
fn clock_failure_is_returned_once_and_terminates_the_session() {
    let (mut owner, _) = pair();
    let handle = owner.subscribe(Root, ACK_DEADLINE).unwrap();
    let clock = FailureClock;
    let mut stream = owner.notifications(&handle, &clock, CLEANUP_TIMEOUT);
    let item = block_on(stream.next()).unwrap();
    assert!(matches!(
        item,
        Err(SessionError::Clock(ClockError::Capacity))
    ));
    assert!(block_on(stream.next()).is_none());
    drop(stream);
    assert!(matches!(
        owner.subscribe(Root, ACK_DEADLINE),
        Err(SessionError::Protocol(Failure::Closed))
    ));
}

#[test]
fn callback_break_releases_the_borrow_for_explicit_unsubscribe() {
    let (mut owner, state) = pair();
    let handle = owner.subscribe(Root, ACK_DEADLINE).unwrap();
    ack(&state, 1, "17");
    notification(&state, "rootNotification", 17, "99");
    notification(&state, "rootNotification", 17, "100");
    let clock = ManualClock::new();
    let count = Rc::new(Cell::new(0));
    let observed = count.clone();
    let result = block_on(
        owner.run_callback(&handle, &clock, CLEANUP_TIMEOUT, move |_| {
            observed.set(observed.get() + 1);
            ControlFlow::Break(())
        }),
    );
    assert!(result.is_ok());
    assert_eq!(count.get(), 1);

    owner.cancel(&handle, ACK_DEADLINE).unwrap();
    block_on(owner.flush(&clock, ACK_DEADLINE)).unwrap();
    assert!(
        state
            .borrow()
            .sent
            .last()
            .unwrap()
            .contains("rootUnsubscribe")
    );
    ack(&state, 2, "true");
    block_on(owner.unsubscribe(handle, &clock, ACK_DEADLINE)).unwrap();
}

#[test]
fn cleanup_timeout_is_relative_to_the_driving_stream_tick() {
    let (mut owner, state) = pair();
    let retained = owner.subscribe(Root, ACK_DEADLINE).unwrap();
    let abandoned = owner.subscribe(Root, ACK_DEADLINE).unwrap();
    ack(&state, 1, "17");
    ack(&state, 2, "18");
    let clock = ManualClock::new();
    let mut context = Context::from_waker(Waker::noop());

    {
        let mut stream = owner.notifications(&retained, &clock, CLEANUP_TIMEOUT);
        assert!(Pin::new(&mut stream).poll_next(&mut context).is_pending());
    }
    drop(abandoned);
    clock.now.set(20);

    let mut stream = owner.notifications(&retained, &clock, CLEANUP_TIMEOUT);
    assert!(Pin::new(&mut stream).poll_next(&mut context).is_pending());
    assert_eq!(&*clock.sleeps.borrow(), &[25]);
    assert!(
        state
            .borrow()
            .sent
            .last()
            .unwrap()
            .contains("rootUnsubscribe")
    );
    assert!(state.borrow().sent.last().unwrap().contains("18"));

    ack(&state, 3, "true");
    assert!(Pin::new(&mut stream).poll_next(&mut context).is_pending());
    drop(stream);
    assert!(owner.next_deadline().is_none());
    assert!(!clock.sleeps.borrow().is_empty());
}
