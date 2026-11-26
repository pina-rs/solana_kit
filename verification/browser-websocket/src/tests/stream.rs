use super::connect;
use core::ops::ControlFlow;
use futures::{StreamExt, channel::oneshot, stream::FusedStream};
use solana_kit_browser::{BrowserClock, BrowserSubscriptionSocket, BrowserWebSocket};
use solana_kit_client::subscriptions::*;
use std::{cell::Cell, rc::Rc};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_test::wasm_bindgen_test;

const GENERATION: u64 = 7;
const ACK_DEADLINE: u64 = 1_000;
const QUIET_ACK_DEADLINE: u64 = 100;
const CLEANUP_TIMEOUT: u64 = 20;
const SIGNATURE: &str = concat!(
    "11111111111111111111111111111111",
    "11111111111111111111111111111111"
);

fn typed_session(socket: BrowserWebSocket) -> Session<BrowserSubscriptionSocket> {
    Session::new(
        BrowserSubscriptionSocket::from(socket),
        GENERATION,
        Budget::default(),
    )
}

#[wasm_bindgen_test]
async fn stream_one_shot_ends_cleanly_over_actual_browser_socket() {
    let clock = BrowserClock::new().unwrap();
    let mut session = typed_session(connect("/family").await);
    let handle = session
        .subscribe(
            SignatureNotifications(SIGNATURE.parse().unwrap(), None),
            clock.now() + ACK_DEADLINE,
        )
        .unwrap();
    let mut stream = session.notifications(&handle, &clock, CLEANUP_TIMEOUT);
    let value = stream.next().await.unwrap().unwrap();
    assert!(matches!(value.value, SignatureInfo::Processed { .. }));
    assert!(stream.next().await.is_none());
    assert!(stream.is_terminated());
    drop(stream);
    session.forget(handle).unwrap();
    session
        .shutdown(&clock, clock.now() + ACK_DEADLINE)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn callback_break_and_explicit_unsubscribe_over_actual_browser_socket() {
    let clock = BrowserClock::new().unwrap();
    let mut session = typed_session(connect("/family").await);
    let handle = session.subscribe(Root, clock.now() + ACK_DEADLINE).unwrap();
    let seen = Rc::new(Cell::new(0));
    let observed = seen.clone();
    session
        .run_callback(&handle, &clock, CLEANUP_TIMEOUT, move |value| {
            observed.set(value);
            ControlFlow::Break(())
        })
        .await
        .unwrap();
    assert_eq!(seen.get(), u64::MAX);
    session
        .unsubscribe(handle, &clock, clock.now() + ACK_DEADLINE)
        .await
        .unwrap();
    session
        .shutdown(&clock, clock.now() + ACK_DEADLINE)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn quiet_missing_acknowledgement_yields_one_failure_then_fuses() {
    let clock = BrowserClock::new().unwrap();
    let mut session = typed_session(connect("/quiet-subscription").await);
    let handle = session
        .subscribe(Root, clock.now() + QUIET_ACK_DEADLINE)
        .unwrap();
    let mut stream = session.notifications(&handle, &clock, CLEANUP_TIMEOUT);
    let item = stream.next().await.unwrap();
    assert!(
        matches!(item, Err(SessionError::Protocol(Failure::Deadline))),
        "{item:?}"
    );
    assert!(stream.next().await.is_none());
    assert!(stream.is_terminated());
    drop(stream);
    session.forget(handle).unwrap();
    session
        .shutdown(&clock, clock.now() + ACK_DEADLINE)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn owned_session_and_callback_run_in_browser_local_task() {
    let socket = connect("/family").await;
    let (sender, receiver) = oneshot::channel();

    spawn_local(async move {
        let clock = BrowserClock::new().unwrap();
        let mut session = typed_session(socket);
        let handle = session.subscribe(Root, clock.now() + ACK_DEADLINE).unwrap();
        let local = Rc::new(Cell::new(0));
        let observed = local.clone();
        let callback = session
            .run_callback(&handle, &clock, CLEANUP_TIMEOUT, move |value| {
                observed.set(value);
                ControlFlow::Break(())
            })
            .await
            .map(|()| local.get());
        let unsubscribe = session
            .unsubscribe(handle, &clock, clock.now() + ACK_DEADLINE)
            .await
            .is_ok();
        let shutdown = session
            .shutdown(&clock, clock.now() + ACK_DEADLINE)
            .await
            .is_ok();
        let _ = sender.send((callback, unsubscribe, shutdown));
    });

    let (callback, unsubscribe, shutdown) = receiver.await.unwrap();
    assert_eq!(callback.unwrap(), u64::MAX);
    assert!(unsubscribe);
    assert!(shutdown);
}
