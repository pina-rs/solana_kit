#![cfg(feature = "confirmation-factories")]
//! Local pending futures, one-shot submission and isolated shared-owner cleanup.
use solana_kit_client::{
    Client, Error, Limits, Reply, Request, Transport, TransportError, TransportFailureKind,
    confirmation::{
        FactoryConfig, FactoryFailure, Failure, Policy, confirm_signature_shared,
        submit_and_confirm,
    },
    methods::submission::SendConfig,
    subscriptions::{
        Budget, Clock, ClockError, Drain, ReceivedFrame, Root, Session, SessionError, TextSocket,
    },
};
use solana_kit_core::{
    Address, Hash, Signature,
    transaction::{Lifetime, Message, Transaction, Version},
};
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    future::{Future, pending, ready},
    num::NonZeroU64,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};
const GENERATION: u64 = 7;
const ROOT_VALUE: u64 = 99;
const LATE_ACK: u64 = 11;

#[derive(Default)]
struct State {
    frames: VecDeque<String>,
    sent: Vec<String>,
    closed: usize,
    false_ack: bool,
    missing_ack: bool,
    late_ack: Option<u64>,
    eof: bool,
}
struct Socket(Rc<RefCell<State>>);
impl TextSocket for Socket {
    type Error = std::convert::Infallible;
    fn start_send(&mut self, text: &str) -> Result<(), Self::Error> {
        let request: serde_json::Value = serde_json::from_str(text).unwrap();
        let mut state = self.0.borrow_mut();
        state.sent.push(text.into());
        let method = request["method"].as_str().unwrap();
        let result = if method.ends_with("Unsubscribe") {
            serde_json::json!(!state.false_ack)
        } else {
            serde_json::json!(request["id"].as_u64().unwrap() + 10)
        };

        if state.missing_ack {
            return Ok(());
        }

        if state.late_ack.is_some() && method.ends_with("Subscribe") {
            state.late_ack = request["id"].as_u64();

            return Ok(());
        }

        state.frames.push_back(
            serde_json::json!({"jsonrpc":"2.0","id":request["id"],"result":result}).to_string(),
        );
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

        if let Some(frame) = state.frames.pop_front() {
            return Poll::Ready(Ok(Some(ReceivedFrame::Text(frame))));
        }

        if state.eof {
            return Poll::Ready(Ok(None));
        }

        Poll::Pending
    }
    fn poll_close(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.borrow_mut().closed += 1;
        Poll::Ready(Ok(()))
    }
}
struct ClockState(Rc<Cell<u64>>);
struct Sleep {
    now: Rc<Cell<u64>>,
    deadline: u64,
    finished: bool,
}
impl Future for Sleep {
    type Output = Result<(), ClockError>;
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        assert!(!self.finished, "completed sleep must never be repolled");

        if self.now.get() >= self.deadline {
            self.finished = true;
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }
}
impl Clock for ClockState {
    type Sleep<'a> = Sleep;
    fn now(&self) -> u64 {
        self.0.get()
    }
    fn sleep_until(&self, deadline: u64) -> Sleep {
        Sleep {
            now: self.0.clone(),
            deadline,
            finished: false,
        }
    }
}
struct LocalRpc {
    marker: Rc<Cell<usize>>,
    uncertain: bool,
}
struct Exchange {
    marker: Rc<Cell<usize>>,
    request: Request,
    pending: bool,
    uncertain: bool,
}
impl Exchange {
    fn submit(&self) -> Result<serde_json::Value, TransportError> {
        self.marker.set(self.marker.get() + 1);

        if self.uncertain {
            return Err(TransportError::with_source(
                TransportFailureKind::Io,
                std::io::Error::other("uncertain submission marker"),
            ));
        }

        Ok(serde_json::json!(
            Signature::from_bytes([7; 64]).to_string()
        ))
    }
}
impl Future for Exchange {
    type Output = Result<Reply, TransportError>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.pending {
            self.pending = true;
            cx.waker().wake_by_ref();

            return Poll::Pending;
        }

        let request: serde_json::Value = serde_json::from_slice(&self.request.body).unwrap();

        let result = match request["method"].as_str().unwrap() {
            "sendTransaction" => self.submit()?,
            "getSignatureStatuses" => serde_json::json!({"context":{"slot":10},"value":[{
                "slot":10,"confirmations":1,"err":null,"status":{"Ok":null},"confirmationStatus":"confirmed"}]}),
            _ => panic!("unexpected operation"),
        };

        Poll::Ready(Ok(Reply {
            status: 200,
            body: serde_json::to_vec(&serde_json::json!({
            "jsonrpc":"2.0","id":request["id"],"result":result}))
            .unwrap(),
        }))
    }
}
impl Transport for LocalRpc {
    type Exchange<'a> = Exchange;
    fn exchange(&self, request: Request) -> Exchange {
        Exchange {
            marker: self.marker.clone(),
            request,
            pending: false,
            uncertain: self.uncertain,
        }
    }
}
fn config() -> FactoryConfig {
    FactoryConfig {
        policy: Policy::default(),
        deadline: 100,
        poll_interval: NonZeroU64::new(1).unwrap(),
        cleanup_millis: NonZeroU64::new(10).unwrap(),
    }
}
fn lifetime() -> Lifetime {
    Lifetime::Blockhash {
        blockhash: Hash::from_bytes([0; 32]),
        last_valid_block_height: 50,
    }
}
fn transaction() -> Transaction {
    let payer = Address::from_bytes([1; 32]);
    let message = Message::compile(Version::Legacy, payer, lifetime(), &[], &[]).unwrap();
    Transaction::new(message.clone())
        .with_signature(payer, message.bytes(), Signature::from_bytes([7; 64]))
        .unwrap()
}
fn context() -> Context<'static> {
    Context::from_waker(Waker::noop())
}
fn owner(state: &Rc<RefCell<State>>) -> RefCell<Session<Socket>> {
    RefCell::new(Session::new(
        Socket(state.clone()),
        GENERATION,
        Budget::default(),
    ))
}
fn client(marker: Rc<Cell<usize>>, uncertain: bool) -> Client<LocalRpc> {
    Client::new(LocalRpc { marker, uncertain }, Limits::default())
}

#[cfg_attr(test, test)]
pub fn pending_local_rc_operation_releases_owner_and_preserves_unrelated_root() {
    let state = Rc::new(RefCell::new(State::default()));
    let owner = owner(&state);
    let root = owner.borrow_mut().subscribe(Root, 100).unwrap();
    state.borrow_mut().frames.push_back(
        r#"{"jsonrpc":"2.0","method":"rootNotification","params":{"subscription":11,"result":99}}"#
            .into(),
    );
    let marker = Rc::new(Cell::new(0));
    let client = client(marker.clone(), false);
    let clock = ClockState(Rc::new(Cell::new(0)));
    let mut operation = Box::pin(confirm_signature_shared(
        &client,
        &owner,
        &clock,
        Signature::from_bytes([7; 64]),
        lifetime(),
        config(),
        pending(),
    ));

    assert!(operation.as_mut().poll(&mut context()).is_pending());
    assert!(
        owner.try_borrow_mut().is_ok(),
        "no RefCell borrow across await"
    );
    assert!(
        Rc::strong_count(&marker) > 2,
        "local Rc retained in pending exchange"
    );
    assert!(matches!(
        operation.as_mut().poll(&mut context()),
        Poll::Ready(Ok(()))
    ));
    assert!(matches!(
        owner.borrow_mut().poll_next(&root, &mut context(), 0, 100),
        Poll::Ready(Ok(Drain::Notification(ROOT_VALUE)))
    ));
    assert_eq!(state.borrow().closed, 0);
    assert_eq!(state.borrow().sent.len(), 3);
}

#[cfg_attr(test, test)]
pub fn one_shot_submission_preserves_original_uncertain_error_and_cleanup() {
    for uncertain in [false, true] {
        let state = Rc::new(RefCell::new(State::default()));
        let owner = owner(&state);
        let marker = Rc::new(Cell::new(0));
        let client = client(marker.clone(), uncertain);
        let clock = ClockState(Rc::new(Cell::new(0)));
        let transaction = transaction();

        let result = futures::executor::block_on(submit_and_confirm(
            &client,
            &owner,
            &clock,
            &transaction,
            SendConfig::default(),
            config(),
            pending(),
        ));

        if uncertain {
            let error = result.unwrap_err();
            assert!(error.cleanup.is_empty());
            let failure = error.outcome.unwrap_err();
            let FactoryFailure::Confirmation(Failure::Rpc(Error::Transport(error))) = *failure
            else {
                panic!("original transport error");
            };
            assert_eq!(error.kind, TransportFailureKind::Io);

            assert_eq!(
                std::error::Error::source(&error).unwrap().to_string(),
                "uncertain submission marker"
            );
        } else {
            assert_eq!(result.unwrap(), Signature::from_bytes([7; 64]));
        }

        assert_eq!(marker.get(), 1, "submission must never retry");
        assert_eq!(state.borrow().sent.len(), 2);
        assert_eq!(state.borrow().closed, 0);
    }
}

#[cfg_attr(test, test)]
pub fn submission_guards_run_before_subscription_or_http() {
    let state = Rc::new(RefCell::new(State::default()));
    let owner = owner(&state);
    let marker = Rc::new(Cell::new(0));
    let client = client(marker.clone(), false);
    let clock = ClockState(Rc::new(Cell::new(0)));
    let signed = transaction();
    let unsigned = Transaction::new(signed.message().clone());
    let received = Transaction::decode(&signed.encode(true).unwrap()).unwrap();

    for transaction in [unsigned, received] {
        assert!(
            futures::executor::block_on(submit_and_confirm(
                &client,
                &owner,
                &clock,
                &transaction,
                SendConfig::default(),
                config(),
                pending()
            ))
            .is_err()
        );
    }

    assert_eq!(marker.get(), 0);
    assert!(state.borrow().sent.is_empty());
}

#[cfg_attr(test, test)]
pub fn cancellation_late_ack_false_ack_deadline_and_disconnect_are_explicit() {
    late_false_ack();
    missing_ack_and_disconnect();
    deadline_and_cancellation_priority();
    completed_acknowledgement_timer_is_not_repolled();
}

#[derive(Clone, Copy)]
enum ConstructionFault {
    Capacity,
    Range,
}
struct FaultClock {
    now: Rc<Cell<u64>>,
    construction: Option<ConstructionFault>,
    unavailable: Rc<Cell<bool>>,
}
struct FaultSleep {
    sleep: Sleep,
    construction: Option<ConstructionFault>,
    unavailable: Rc<Cell<bool>>,
    finished: bool,
}
impl Clock for FaultClock {
    type Sleep<'a> = FaultSleep;

    fn now(&self) -> u64 {
        self.now.get()
    }

    fn sleep_until(&self, deadline: u64) -> FaultSleep {
        FaultSleep {
            sleep: Sleep {
                now: self.now.clone(),
                deadline,
                finished: false,
            },
            construction: self.construction,
            unavailable: self.unavailable.clone(),
            finished: false,
        }
    }
}
impl Future for FaultSleep {
    type Output = Result<(), ClockError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        assert!(
            !self.finished,
            "completed fault sleep must never be repolled"
        );
        let failure = match self.construction.take() {
            Some(ConstructionFault::Capacity) => Some(ClockError::Capacity),
            Some(ConstructionFault::Range) => Some(ClockError::DeadlineRange),
            None if self.unavailable.get() => {
                Some(ClockError::Unavailable(TransportError::with_source(
                    TransportFailureKind::Io,
                    std::io::Error::other("confirmation clock source marker"),
                )))
            }
            None => None,
        };

        if let Some(error) = failure {
            self.finished = true;

            return Poll::Ready(Err(error));
        }

        let result = Pin::new(&mut self.sleep).poll(cx);
        self.finished = result.is_ready();
        result
    }
}

#[cfg_attr(test, test)]
pub fn clock_construction_failures_prevent_submission_and_retain_failed_cleanup() {
    for fault in [ConstructionFault::Capacity, ConstructionFault::Range] {
        let state = Rc::new(RefCell::new(State::default()));
        let owner = owner(&state);
        let marker = Rc::new(Cell::new(0));
        let client = client(marker.clone(), false);
        let clock = FaultClock {
            now: Rc::new(Cell::new(0)),
            construction: Some(fault),
            unavailable: Rc::new(Cell::new(false)),
        };
        let transaction = transaction();
        let error = futures::executor::block_on(submit_and_confirm(
            &client,
            &owner,
            &clock,
            &transaction,
            SendConfig::default(),
            config(),
            pending(),
        ))
        .unwrap_err();

        assert!(matches!(
            (fault, *error.outcome.unwrap_err()),
            (
                ConstructionFault::Capacity,
                FactoryFailure::Session(SessionError::Clock(ClockError::Capacity))
            ) | (
                ConstructionFault::Range,
                FactoryFailure::Session(SessionError::Clock(ClockError::DeadlineRange))
            )
        ));
        assert!(matches!(
            (fault, error.cleanup.as_slice()),
            (
                ConstructionFault::Capacity,
                [SessionError::Clock(ClockError::Capacity)]
            ) | (
                ConstructionFault::Range,
                [SessionError::Clock(ClockError::DeadlineRange)]
            )
        ));
        assert_eq!(marker.get(), 0, "failed clock cannot authorize submission");
        assert!(state.borrow().sent.is_empty());
        assert_eq!(
            state.borrow().closed,
            0,
            "application retains socket shutdown"
        );
    }
}

#[cfg_attr(test, test)]
pub fn poll_clock_failure_retains_original_cause_without_resending_signed_bytes() {
    let state = Rc::new(RefCell::new(State::default()));
    let owner = owner(&state);
    let marker = Rc::new(Cell::new(0));
    let client = client(marker.clone(), false);
    let unavailable = Rc::new(Cell::new(false));
    let clock = FaultClock {
        now: Rc::new(Cell::new(0)),
        construction: None,
        unavailable: unavailable.clone(),
    };
    let transaction = transaction();
    let mut operation = Box::pin(submit_and_confirm(
        &client,
        &owner,
        &clock,
        &transaction,
        SendConfig::default(),
        config(),
        pending(),
    ));

    assert!(operation.as_mut().poll(&mut context()).is_pending());
    assert!(operation.as_mut().poll(&mut context()).is_pending());
    assert_eq!(marker.get(), 1);
    assert!(owner.try_borrow_mut().is_ok());
    assert!(
        Rc::strong_count(&marker) > 2,
        "local Rc crosses the pending read"
    );
    unavailable.set(true);

    let Poll::Ready(Err(error)) = operation.as_mut().poll(&mut context()) else {
        panic!("poll-time clock failure must complete the operation");
    };
    let FactoryFailure::Session(SessionError::Clock(ClockError::Unavailable(cause))) =
        *error.outcome.unwrap_err()
    else {
        panic!("original clock failure must remain distinct from timeout");
    };
    assert_eq!(
        std::error::Error::source(&cause).unwrap().to_string(),
        "confirmation clock source marker"
    );
    assert!(matches!(
        error.cleanup.as_slice(),
        [SessionError::Clock(ClockError::Unavailable(_))]
    ));
    assert_eq!(
        marker.get(),
        1,
        "clock failure cannot retry signed submission"
    );
    assert_eq!(state.borrow().sent.len(), 1);
    assert_eq!(
        state.borrow().closed,
        0,
        "application retains socket shutdown"
    );
}
struct QuietRpc;
impl Transport for QuietRpc {
    type Exchange<'a> = std::future::Pending<Result<Reply, TransportError>>;
    fn exchange(&self, _: Request) -> Self::Exchange<'_> {
        pending()
    }
}
fn completed_acknowledgement_timer_is_not_repolled() {
    let state = Rc::new(RefCell::new(State {
        late_ack: Some(0),
        ..Default::default()
    }));
    let owner = owner(&state);
    let _root = owner.borrow_mut().subscribe(Root, 1).unwrap();
    let clock = ClockState(Rc::new(Cell::new(0)));
    let client = Client::new(QuietRpc, Limits::default());
    let mut operation = Box::pin(confirm_signature_shared(
        &client,
        &owner,
        &clock,
        Signature::from_bytes([7; 64]),
        lifetime(),
        config(),
        pending(),
    ));
    assert!(operation.as_mut().poll(&mut context()).is_pending());
    clock.0.set(2);

    assert!(operation.as_mut().poll(&mut context()).is_pending());
    assert!(operation.as_mut().poll(&mut context()).is_pending());
    clock.0.set(config().deadline);
    assert!(matches!(
        operation.as_mut().poll(&mut context()),
        Poll::Ready(Err(_))
    ));
}
fn deadline_and_cancellation_priority() {
    for cancelled in [false, true] {
        let state = Rc::new(RefCell::new(State::default()));
        let owner = owner(&state);
        let client = client(Rc::new(Cell::new(0)), false);
        let clock = ClockState(Rc::new(Cell::new(0)));
        let gate = Rc::new(Cell::new(false));
        let cancel = std::future::poll_fn(|_| {
            if gate.get() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        });
        let mut operation = Box::pin(confirm_signature_shared(
            &client,
            &owner,
            &clock,
            Signature::from_bytes([7; 64]),
            lifetime(),
            config(),
            cancel,
        ));

        assert!(operation.as_mut().poll(&mut context()).is_pending());
        gate.set(cancelled);
        clock.0.set(config().deadline);
        let Poll::Ready(Err(error)) = operation.as_mut().poll(&mut context()) else {
            panic!("deadline or cancellation");
        };
        let FactoryFailure::Confirmation(failure) = *error.outcome.unwrap_err() else {
            panic!("policy failure");
        };
        assert!(matches!(
            (cancelled, failure),
            (true, Failure::Cancelled) | (false, Failure::Timeout)
        ));

        assert!(error.cleanup.is_empty());
        assert_eq!(state.borrow().closed, 0);
    }
}
fn late_false_ack() {
    let state = Rc::new(RefCell::new(State {
        late_ack: Some(0),
        false_ack: true,
        ..Default::default()
    }));
    let owner = owner(&state);
    let client = client(Rc::new(Cell::new(0)), false);
    let clock = ClockState(Rc::new(Cell::new(0)));
    let mut operation = Box::pin(confirm_signature_shared(
        &client,
        &owner,
        &clock,
        Signature::from_bytes([7; 64]),
        lifetime(),
        config(),
        ready(()),
    ));
    assert!(operation.as_mut().poll(&mut context()).is_pending());
    assert!(owner.try_borrow_mut().is_ok());
    let id = state.borrow().late_ack.unwrap();

    state
        .borrow_mut()
        .frames
        .push_back(serde_json::json!({"jsonrpc":"2.0","id":id,"result":LATE_ACK}).to_string());

    let Poll::Ready(Err(error)) = operation.as_mut().poll(&mut context()) else {
        panic!("cancel cleanup");
    };
    assert!(matches!(
        *error.outcome.unwrap_err(),
        FactoryFailure::Confirmation(Failure::Cancelled)
    ));
    assert_eq!(
        error.cleanup.len(),
        1,
        "false remote acknowledgement retained"
    );
}
fn missing_ack_and_disconnect() {
    let client = client(Rc::new(Cell::new(0)), false);
    let clock = ClockState(Rc::new(Cell::new(0)));
    let state = Rc::new(RefCell::new(State {
        missing_ack: true,
        ..Default::default()
    }));
    let owner = self::owner(&state);
    let mut operation = Box::pin(confirm_signature_shared(
        &client,
        &owner,
        &clock,
        Signature::from_bytes([7; 64]),
        lifetime(),
        config(),
        ready(()),
    ));
    assert!(operation.as_mut().poll(&mut context()).is_pending());
    clock.0.set(LATE_ACK);

    let Poll::Ready(Err(error)) = operation.as_mut().poll(&mut context()) else {
        panic!("bounded missing ack");
    };
    assert!(matches!(error.cleanup.as_slice(), [SessionError::Deadline]));
    drop(operation);

    let state = Rc::new(RefCell::new(State {
        eof: true,
        ..Default::default()
    }));
    let owner = self::owner(&state);
    let mut operation = Box::pin(confirm_signature_shared(
        &client,
        &owner,
        &clock,
        Signature::from_bytes([7; 64]),
        lifetime(),
        config(),
        pending(),
    ));
    let Poll::Ready(Err(error)) = operation.as_mut().poll(&mut context()) else {
        panic!("disconnect");
    };
    assert!(matches!(
        *error.outcome.unwrap_err(),
        FactoryFailure::Session(SessionError::Eof)
    ));
}
