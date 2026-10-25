//! Observable application-owned progress and cleanup contracts.
use solana_kit_client::subscriptions::*;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};
const GENERATION: u64 = 7;
const SUBSCRIBE_DEADLINE: u64 = 50;
const CLEANUP_DEADLINE: u64 = 20;
const AFTER_DEADLINE: u64 = 11;
const ROOT_VALUE: u64 = 99;

#[derive(Default)]
struct State {
    sent: Vec<String>,
    incoming: VecDeque<String>,
    closed: usize,
    eof: bool,
}
struct Socket(Arc<Mutex<State>>);
impl TextSocket for Socket {
    type Error = ();
    fn start_send(&mut self, text: &str) -> Result<(), ()> {
        self.0.lock().unwrap().sent.push(text.into());
        Ok(())
    }
    fn poll_flush(&mut self, _: &mut Context<'_>) -> Poll<Result<(), ()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_receive(&mut self, _: &mut Context<'_>) -> Poll<Result<Option<ReceivedFrame>, ()>> {
        let mut state = self.0.lock().unwrap();

        if let Some(frame) = state.incoming.pop_front() {
            return Poll::Ready(Ok(Some(ReceivedFrame::Text(frame))));
        }

        if state.eof {
            return Poll::Ready(Ok(None));
        }

        Poll::Pending
    }
    fn poll_close(&mut self, _: &mut Context<'_>) -> Poll<Result<(), ()>> {
        self.0.lock().unwrap().closed += 1;
        Poll::Ready(Ok(()))
    }
}
fn pair() -> (Session<Socket>, Arc<Mutex<State>>) {
    let state = Arc::new(Mutex::new(State::default()));
    (
        Session::new(Socket(state.clone()), GENERATION, Budget::default()),
        state,
    )
}
fn push(state: &Arc<Mutex<State>>, frame: &str) {
    state.lock().unwrap().incoming.push_back(frame.into());
}
fn ack(state: &Arc<Mutex<State>>, id: u32, value: &str) {
    push(
        state,
        &format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{value}}}"#),
    );
}
#[test]
fn quiet_receive_does_not_prevent_dynamic_registration_or_unsubscribe() {
    let (mut owner, state) = pair();
    let root = owner.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
    let mut cx = Context::from_waker(Waker::noop());
    assert!(owner.poll_next(&root, &mut cx, 0, 100).is_pending());
    assert_eq!(state.lock().unwrap().sent.len(), 1);
    let slot = owner.subscribe(Slot, SUBSCRIBE_DEADLINE).unwrap();
    ack(&state, 1, "70");
    ack(&state, 2, "71");

    assert!(owner.poll_next(&slot, &mut cx, 1, 100).is_pending());
    owner.cancel(&root, 100).unwrap();

    assert!(owner.poll_next(&slot, &mut cx, 2, 100).is_pending());
    assert!(state.lock().unwrap().sent[2].contains("rootUnsubscribe"));
    ack(&state, 3, "true");
    assert!(matches!(
        owner.poll_next(&root, &mut cx, 3, 100),
        Poll::Ready(Ok(Drain::Closed))
    ));
    owner.forget(root).unwrap();
    push(
        &state,
        r#"{"jsonrpc":"2.0","method":"slotNotification","params":{"subscription":71,"result":{"slot":9,"parent":8,"root":7}}}"#,
    );
    let Poll::Ready(Ok(Drain::Notification(value))) = owner.poll_next(&slot, &mut cx, 4, 100)
    else {
        panic!("typed slot notification missing");
    };
    assert_eq!(value.slot, 9);
    assert_eq!(state.lock().unwrap().closed, 0);
}
#[test]
fn cancel_before_subscribe_ack_still_awaits_true_remote_cleanup() {
    let (mut owner, state) = pair();
    let handle = owner.subscribe(Root, 10).unwrap();
    owner.cancel(&handle, CLEANUP_DEADLINE).unwrap();
    let mut cx = Context::from_waker(Waker::noop());
    assert!(
        owner
            .poll_next(&handle, &mut cx, 0, CLEANUP_DEADLINE)
            .is_pending()
    );
    ack(&state, 1, "4");
    assert!(
        owner
            .poll_next(&handle, &mut cx, 1, CLEANUP_DEADLINE)
            .is_pending()
    );
    assert!(
        owner
            .poll_next(&handle, &mut cx, 2, CLEANUP_DEADLINE)
            .is_pending()
    );

    assert!(state.lock().unwrap().sent[1].contains("rootUnsubscribe"));
    ack(&state, 2, "false");
    assert!(matches!(
        owner.poll_next(&handle, &mut cx, 3, CLEANUP_DEADLINE),
        Poll::Ready(Err(SessionError::Protocol(Failure::UnsubscribeFalse)))
    ));
}
#[test]
fn early_notification_missing_ack_and_peer_eof_remain_observable() {
    let (mut owner, state) = pair();
    let handle = owner.subscribe(Root, 10).unwrap();
    push(
        &state,
        r#"{"jsonrpc":"2.0","method":"rootNotification","params":{"subscription":4,"result":99}}"#,
    );
    ack(&state, 1, "4");
    let mut cx = Context::from_waker(Waker::noop());
    assert!(matches!(
        owner.poll_next(&handle, &mut cx, 0, CLEANUP_DEADLINE),
        Poll::Ready(Ok(Drain::Notification(ROOT_VALUE)))
    ));
    let pending = owner.subscribe(Slot, 10).unwrap();
    assert!(matches!(
        owner.poll_next(&pending, &mut cx, 10, CLEANUP_DEADLINE),
        Poll::Ready(Err(SessionError::Protocol(Failure::Deadline)))
    ));

    state.lock().unwrap().eof = true;
    assert!(matches!(
        owner.poll_next(&handle, &mut cx, AFTER_DEADLINE, CLEANUP_DEADLINE),
        Poll::Ready(Err(SessionError::Eof))
    ));
}
