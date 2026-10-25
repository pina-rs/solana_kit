//! Actual Session dispatch, Pending wake and separately awaited close contracts.
use serde::Serialize;
use solana_kit_client::{Error, methods::Method, subscriptions::*};
use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

const EXCHANGE_DEADLINE: u64 = 20;
const FRAME_BYTES: usize = 128;
const NOTIFICATION_VALUE: u64 = 9;

#[derive(Serialize)]
struct Binary([u8; 0]);
impl Method for Binary {
    type Response = u64;
    const NAME: &'static str = "binarySubscribe";
    fn decode(&self, _: &str) -> Result<u64, Error> {
        Err(Error::Envelope)
    }
}
impl ProviderMethod for Binary {
    const UNSUBSCRIBE: &'static str = "binaryUnsubscribe";
    const NOTIFICATION: &'static str = "binaryNotification";
    const NOTIFICATION_KIND: NotificationKind = NotificationKind::Binary("test-u64-v1");
    fn decode_binary(&self, bytes: &[u8]) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(
            bytes.try_into().map_err(|_| Error::Envelope)?,
        ))
    }
}

#[derive(Serialize)]
struct OtherBinary([u8; 0]);
impl Method for OtherBinary {
    type Response = u64;
    const NAME: &'static str = Binary::NAME;
    fn decode(&self, _: &str) -> Result<u64, Error> {
        Err(Error::Envelope)
    }
}
impl ProviderMethod for OtherBinary {
    const UNSUBSCRIBE: &'static str = Binary::UNSUBSCRIBE;
    const NOTIFICATION: &'static str = Binary::NOTIFICATION;
    const NOTIFICATION_KIND: NotificationKind = Binary::NOTIFICATION_KIND;
}

#[derive(Default)]
struct State {
    incoming: VecDeque<ReceivedFrame>,
    sent: Vec<String>,
    joined: bool,
    closes: usize,
    receive_waker: Option<Waker>,
}
struct Socket(Rc<RefCell<State>>);
impl TextSocket for Socket {
    type Error = ();
    fn start_send(&mut self, text: &str) -> Result<(), ()> {
        self.0.borrow_mut().sent.push(text.to_owned());
        Ok(())
    }
    fn poll_flush(&mut self, _: &mut Context<'_>) -> Poll<Result<(), ()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_receive(&mut self, cx: &mut Context<'_>) -> Poll<Result<Option<ReceivedFrame>, ()>> {
        let mut state = self.0.borrow_mut();

        if let Some(frame) = state.incoming.pop_front() {
            return Poll::Ready(Ok(Some(frame)));
        }

        state.receive_waker = Some(cx.waker().clone());
        Poll::Pending
    }
    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), ()>> {
        let mut state = self.0.borrow_mut();
        state.closes += 1;

        if state.joined {
            Poll::Ready(Ok(()))
        } else {
            state.receive_waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
fn pair(budget: Budget) -> (Session<Socket>, Rc<RefCell<State>>) {
    let state = Rc::new(RefCell::new(State::default()));
    (Session::new(Socket(state.clone()), 1, budget), state)
}
fn ack(state: &Rc<RefCell<State>>, id: u32, value: &str) {
    state
        .borrow_mut()
        .incoming
        .push_back(ReceivedFrame::Text(format!(
            r#"{{"jsonrpc":"2.0","id":{id},"result":{value}}}"#
        )));
}

#[test]
fn session_forwards_preack_binary_without_delivering_it_to_json_consumers() {
    let (mut owner, state) = pair(Budget::default());
    let json = owner.subscribe(Root, 10).unwrap();
    let binary = owner.subscribe(Provider(Binary([])), 10).unwrap();
    state.borrow_mut().incoming.push_back(ReceivedFrame::Binary(
        NOTIFICATION_VALUE.to_le_bytes().to_vec(),
    ));
    ack(&state, 1, "7");
    ack(&state, 2, "8");
    let mut cx = Context::from_waker(Waker::noop());
    assert!(
        owner
            .poll_next(&json, &mut cx, 0, EXCHANGE_DEADLINE)
            .is_pending()
    );

    assert!(matches!(
        owner.poll_next(&binary, &mut cx, 0, EXCHANGE_DEADLINE),
        Poll::Ready(Ok(Drain::Notification(NOTIFICATION_VALUE)))
    ));
    assert_eq!(state.borrow().sent.len(), 2);
    owner.cancel(&binary, EXCHANGE_DEADLINE).unwrap();
    ack(&state, 3, "true");
    assert!(matches!(
        owner.poll_next(&binary, &mut cx, 1, EXCHANGE_DEADLINE),
        Poll::Ready(Ok(Drain::Closed))
    ));
    owner.forget(binary).unwrap();
    assert!(owner.poll_shutdown(&mut cx).is_pending());
    assert_eq!(state.borrow().closes, 1);

    state.borrow_mut().joined = true;
    assert!(matches!(owner.poll_shutdown(&mut cx), Poll::Ready(Ok(()))));
}

#[test]
fn session_enforces_binary_frame_caps_and_json_only_rejection() {
    let (mut owner, state) = pair(Budget::default());
    let json = owner.subscribe(Root, 10).unwrap();
    ack(&state, 1, "7");
    state
        .borrow_mut()
        .incoming
        .push_back(ReceivedFrame::Binary(vec![0; 8]));
    let mut cx = Context::from_waker(Waker::noop());
    assert!(matches!(
        owner.poll_next(&json, &mut cx, 0, EXCHANGE_DEADLINE),
        Poll::Ready(Err(SessionError::Protocol(Failure::Method)))
    ));

    let (mut owner, state) = pair(Budget {
        frame: solana_kit_client::Limits {
            bytes: FRAME_BYTES,
            ..Default::default()
        },
        ..Budget::default()
    });
    let binary = owner.subscribe(Provider(Binary([])), 10).unwrap();
    ack(&state, 1, "7");
    state
        .borrow_mut()
        .incoming
        .push_back(ReceivedFrame::Binary(vec![0; FRAME_BYTES + 1]));
    assert!(matches!(
        owner.poll_next(&binary, &mut cx, 0, EXCHANGE_DEADLINE),
        Poll::Ready(Err(SessionError::Protocol(Failure::Protocol(
            Error::BodyLimit
        ))))
    ));
}

#[test]
fn identical_binary_names_do_not_authorize_another_concrete_decoder() {
    let (mut owner, state) = pair(Budget::default());
    let first = owner.subscribe(Provider(Binary([])), 10).unwrap();
    assert!(matches!(
        owner.subscribe(Provider(OtherBinary([])), 10),
        Err(SessionError::Protocol(Failure::Method))
    ));
    ack(&state, 1, "7");
    state.borrow_mut().incoming.push_back(ReceivedFrame::Binary(
        NOTIFICATION_VALUE.to_le_bytes().to_vec(),
    ));
    let mut cx = Context::from_waker(Waker::noop());
    assert!(matches!(
        owner.poll_next(&first, &mut cx, 0, EXCHANGE_DEADLINE),
        Poll::Ready(Ok(Drain::Notification(NOTIFICATION_VALUE)))
    ));
}

struct Counter(AtomicUsize);
impl Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn idle_last_handle_drop_wakes_pending_session_and_preserves_joined_shutdown() {
    let (mut owner, state) = pair(Budget::default());
    let root = owner.subscribe(Root, 10).unwrap();
    let dropped = owner.subscribe(Provider(Binary([])), 10).unwrap();
    ack(&state, 1, "7");
    ack(&state, 2, "8");
    let counter = Arc::new(Counter(AtomicUsize::new(0)));
    let waker = Waker::from(counter.clone());
    let mut cx = Context::from_waker(&waker);

    assert!(
        owner
            .poll_next(&root, &mut cx, 0, EXCHANGE_DEADLINE)
            .is_pending()
    );
    assert_eq!(state.borrow().sent.len(), 2);
    let previous = counter.0.load(Ordering::SeqCst);
    drop(dropped);
    assert!(
        counter.0.load(Ordering::SeqCst) > previous,
        "idle drop wakes the real Session"
    );
    assert!(
        owner
            .poll_next(&root, &mut cx, 1, EXCHANGE_DEADLINE)
            .is_pending()
    );
    assert_eq!(state.borrow().sent.len(), 3);
    assert!(state.borrow().sent[2].contains("binaryUnsubscribe"));

    assert_eq!(owner.next_deadline(), Some(EXCHANGE_DEADLINE));
    ack(&state, 3, "true");
    assert!(
        owner
            .poll_next(&root, &mut cx, 2, EXCHANGE_DEADLINE)
            .is_pending()
    );
    assert_eq!(owner.next_deadline(), None);
    assert!(
        owner.poll_shutdown(&mut cx).is_pending(),
        "request stop is not joined close"
    );
    state.borrow_mut().joined = true;
    assert!(matches!(owner.poll_shutdown(&mut cx), Poll::Ready(Ok(()))));
}
