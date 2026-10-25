#![cfg(feature = "confirmation-factories")]
//! Independent watches share one protocol owner without closing unrelated work.
use solana_kit_client::{
    confirmation::{ConfirmationWatch, FactoryConfig, Policy},
    subscriptions::{Budget, Drain, ReceivedFrame, Root, Session, TextSocket},
};
use solana_kit_core::{Hash, Signature, transaction::Lifetime};
use std::{
    collections::VecDeque,
    num::NonZeroU64,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};
const GENERATION: u64 = 7;
const REPLACEMENT_GENERATION: u64 = 8;
const FIRST_REMOTE: u32 = 11;
const SECOND_REMOTE: u32 = 12;
const ROOT_REMOTE: u32 = 13;
const CLEANUP_DEADLINE: u64 = 20;
const ROOT_VALUE: u64 = 99;

#[derive(Default)]
struct State {
    incoming: VecDeque<String>,
    sent: Vec<String>,
    closed: usize,
}
struct Socket(Arc<Mutex<State>>);
impl TextSocket for Socket {
    type Error = core::convert::Infallible;
    fn start_send(&mut self, text: &str) -> Result<(), Self::Error> {
        self.0.lock().unwrap().sent.push(text.into());
        Ok(())
    }
    fn poll_flush(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn poll_receive(
        &mut self,
        _: &mut Context<'_>,
    ) -> Poll<Result<Option<ReceivedFrame>, Self::Error>> {
        self.0
            .lock()
            .unwrap()
            .incoming
            .pop_front()
            .map_or(Poll::Pending, |frame| {
                Poll::Ready(Ok(Some(ReceivedFrame::Text(frame))))
            })
    }
    fn poll_close(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.lock().unwrap().closed += 1;
        Poll::Ready(Ok(()))
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
fn register(owner: &mut Session<Socket>, byte: u8) -> ConfirmationWatch {
    ConfirmationWatch::register(
        owner,
        Signature::from_bytes([byte; 64]),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([0; 32]),
            last_valid_block_height: 50,
        },
        config(),
    )
    .unwrap()
}
fn push(state: &Arc<Mutex<State>>, frame: String) {
    state.lock().unwrap().incoming.push_back(frame);
}
fn ack(state: &Arc<Mutex<State>>, id: u32, result: u32) {
    push(
        state,
        format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#),
    );
}
fn signature(state: &Arc<Mutex<State>>, subscription: u32, value: &str) {
    push(
        state,
        format!(
            r#"{{"jsonrpc":"2.0","method":"signatureNotification","params":{{"subscription":{subscription},"result":{{"context":{{"slot":20}},"value":{value}}}}}}}"#
        ),
    );
}
#[cfg_attr(test, test)]
pub fn independent_confirmation_watches_retain_other_notifications_and_socket() {
    let state = Arc::new(Mutex::new(State::default()));
    let mut owner = Session::new(Socket(state.clone()), GENERATION, Budget::default());
    let mut first = register(&mut owner, 1);
    let mut second = register(&mut owner, 2);
    let root = owner.subscribe(Root, 100).unwrap();
    let mut cx = Context::from_waker(Waker::noop());
    assert!(owner.poll_flush(&mut cx).is_ready());

    ack(&state, 1, FIRST_REMOTE);
    ack(&state, 2, SECOND_REMOTE);
    ack(&state, 3, ROOT_REMOTE);
    signature(&state, SECOND_REMOTE, r#"{"err":null}"#);
    signature(&state, FIRST_REMOTE, r#"{"err":null}"#);

    assert!(matches!(
        first.poll_notification(&mut owner, &mut cx, 1),
        Poll::Ready(Ok(()))
    ));
    assert!(matches!(
        second.poll_notification(&mut owner, &mut cx, 1),
        Poll::Ready(Ok(()))
    ));
    assert!(matches!(
        first.poll_cleanup(&mut owner, &mut cx, 1, CLEANUP_DEADLINE),
        Poll::Ready(Ok(()))
    ));
    assert!(matches!(
        second.poll_cleanup(&mut owner, &mut cx, 1, CLEANUP_DEADLINE),
        Poll::Ready(Ok(()))
    ));
    push(
        &state,
        r#"{"jsonrpc":"2.0","method":"rootNotification","params":{"subscription":13,"result":99}}"#
            .into(),
    );
    assert!(matches!(
        owner.poll_next(&root, &mut cx, 2, CLEANUP_DEADLINE),
        Poll::Ready(Ok(Drain::Notification(ROOT_VALUE)))
    ));
    assert_eq!(state.lock().unwrap().closed, 0);
    assert_eq!(state.lock().unwrap().sent.len(), 3);
}
#[cfg_attr(test, test)]
pub fn receipt_notification_rearms_poll_before_later_execution_notification() {
    let state = Arc::new(Mutex::new(State::default()));
    let mut owner = Session::new(
        Socket(state.clone()),
        REPLACEMENT_GENERATION,
        Budget::default(),
    );
    let mut watch = register(&mut owner, 3);
    let mut cx = Context::from_waker(Waker::noop());
    ack(&state, 1, FIRST_REMOTE);
    signature(&state, FIRST_REMOTE, r#""receivedSignature""#);
    signature(&state, FIRST_REMOTE, r#"{"err":null}"#);

    assert!(watch.poll_notification(&mut owner, &mut cx, 1).is_pending());
    assert!(matches!(
        watch.poll_notification(&mut owner, &mut cx, 1),
        Poll::Ready(Ok(()))
    ));
}
