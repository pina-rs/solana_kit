//! Clock failure never grants a replay or masquerades as an elapsed deadline.
use solana_kit_client::{TransportError, TransportFailureKind, subscriptions::*};
use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

const EXCHANGE_DEADLINE: u64 = 20;
const CLOSED_DEADLINE: u64 = 30;
const SHUTDOWN_DEADLINE: u64 = 40;
const RETAINED_LOCAL_VALUE: i32 = 7;
const TIMER_CONSTRUCTION_CEILING: usize = 5;

#[derive(Default)]
struct State {
    sends: usize,
    closes: usize,
    drops: usize,
    receive_waker: Option<Waker>,
}
struct Socket(Rc<RefCell<State>>);
impl Drop for Socket {
    fn drop(&mut self) {
        self.0.borrow_mut().drops += 1;
    }
}
impl TextSocket for Socket {
    type Error = std::convert::Infallible;
    fn start_send(&mut self, _: &str) -> Result<(), Self::Error> {
        self.0.borrow_mut().sends += 1;
        Ok(())
    }
    fn poll_flush(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn poll_receive(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<ReceivedFrame>, Self::Error>> {
        self.0.borrow_mut().receive_waker = Some(cx.waker().clone());
        Poll::Pending
    }
    fn poll_close(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.borrow_mut().closes += 1;
        Poll::Ready(Ok(()))
    }
}
#[derive(Clone, Copy)]
enum Mode {
    Capacity,
    Range,
    PollFailure,
    Never,
    Advance,
}
struct Timer {
    mode: Mode,
    now: Rc<Cell<u64>>,
    constructions: Rc<Cell<usize>>,
}
struct Sleep {
    mode: Mode,
    now: Rc<Cell<u64>>,
    deadline: u64,
    pending: bool,
}
impl Clock for Timer {
    type Sleep<'a> = Sleep;
    fn now(&self) -> u64 {
        self.now.get()
    }
    fn sleep_until(&self, deadline: u64) -> Sleep {
        let count = self.constructions.get() + 1;
        assert!(
            count < TIMER_CONSTRUCTION_CEILING,
            "expired acknowledgement timer starved protocol polling"
        );
        self.constructions.set(count);
        Sleep {
            mode: self.mode,
            now: self.now.clone(),
            deadline,
            pending: false,
        }
    }
}
impl Future for Sleep {
    type Output = Result<(), ClockError>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match this.mode {
            Mode::Capacity => Poll::Ready(Err(ClockError::Capacity)),
            Mode::Range => Poll::Ready(Err(ClockError::DeadlineRange)),
            Mode::Never => Poll::Pending,

            Mode::Advance if this.deadline <= this.now.get() => Poll::Ready(Ok(())),

            Mode::PollFailure | Mode::Advance if !this.pending => {
                this.pending = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }

            Mode::PollFailure => {
                Poll::Ready(Err(ClockError::Unavailable(TransportError::with_source(
                    TransportFailureKind::Unavailable,
                    std::io::Error::other("clock disconnected"),
                ))))
            }
            Mode::Advance => {
                this.now.set(this.deadline);
                Poll::Ready(Ok(()))
            }
        }
    }
}
fn timer(mode: Mode) -> Timer {
    Timer {
        mode,
        now: Rc::new(Cell::new(0)),
        constructions: Rc::new(Cell::new(0)),
    }
}
fn owner() -> (Session<Socket>, Rc<RefCell<State>>) {
    let state = Rc::new(RefCell::default());
    (
        Session::new(Socket(state.clone()), 1, Budget::default()),
        state,
    )
}

#[test]
fn construction_failures_precede_send_and_retire_the_owner() {
    for mode in [Mode::Capacity, Mode::Range] {
        let (mut owner, state) = owner();
        let _handle = owner.subscribe(Root, 10).unwrap();
        let result = futures::executor::block_on(owner.flush(&timer(mode), EXCHANGE_DEADLINE));

        match mode {
            Mode::Capacity => assert!(matches!(
                result,
                Err(SessionError::Clock(ClockError::Capacity))
            )),
            Mode::Range => assert!(matches!(
                result,
                Err(SessionError::Clock(ClockError::DeadlineRange))
            )),
            _ => unreachable!(),
        }

        assert_eq!(state.borrow().sends, 0);
        assert!(matches!(
            owner.subscribe(Root, CLOSED_DEADLINE),
            Err(SessionError::Protocol(Failure::Closed))
        ));
        futures::executor::block_on(owner.shutdown(&timer(Mode::Never), SHUTDOWN_DEADLINE))
            .unwrap();

        assert_eq!(state.borrow().closes, 1);
        assert_eq!(state.borrow().drops, 1);
    }
}

#[test]
fn pending_clock_failure_retains_cause_without_replaying_an_uncertain_send() {
    use std::error::Error as _;
    let (mut owner, state) = owner();
    let handle = owner.subscribe(Root, 10).unwrap();
    let local = Rc::new(RETAINED_LOCAL_VALUE);
    let result = futures::executor::block_on(async {
        let result = owner
            .next(&handle, &timer(Mode::PollFailure), EXCHANGE_DEADLINE)
            .await;
        assert_eq!(*local, RETAINED_LOCAL_VALUE);
        result
    });
    let Err(SessionError::Clock(ClockError::Unavailable(error))) = result else {
        panic!("clock failure must remain distinct from timeout");
    };
    assert_eq!(error.source().unwrap().to_string(), "clock disconnected");
    assert_eq!(state.borrow().sends, 1);

    assert!(state.borrow().receive_waker.is_some());
    assert!(matches!(
        owner
            .flush(&timer(Mode::Never), CLOSED_DEADLINE)
            .now_or_never(),
        Some(Err(SessionError::Protocol(Failure::Closed)))
    ));
    futures::executor::block_on(owner.shutdown(&timer(Mode::Never), SHUTDOWN_DEADLINE)).unwrap();
    assert_eq!(state.borrow().sends, 1);
    assert_eq!(state.borrow().drops, 1);
}

#[test]
fn elapsed_acknowledgement_timer_allows_the_protocol_owner_to_expire() {
    let (mut owner, state) = owner();
    let handle = owner.subscribe(Root, 10).unwrap();
    let result =
        futures::executor::block_on(owner.next(&handle, &timer(Mode::Advance), EXCHANGE_DEADLINE));
    assert!(matches!(
        result,
        Err(SessionError::Protocol(Failure::Deadline))
    ));
    assert_eq!(state.borrow().sends, 1);
}

#[test]
fn failed_shutdown_clock_still_drops_the_actual_socket_owner() {
    let (owner, state) = owner();
    let result =
        futures::executor::block_on(owner.shutdown(&timer(Mode::Range), EXCHANGE_DEADLINE));
    assert!(matches!(
        result,
        Err(SessionError::Clock(ClockError::DeadlineRange))
    ));
    assert_eq!(state.borrow().closes, 0);
    assert_eq!(state.borrow().drops, 1);
    fn send<T: Send>() {}
    send::<SessionError<std::convert::Infallible>>();
    send::<ClockError>();
}

use futures::FutureExt;
