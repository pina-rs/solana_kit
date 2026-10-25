//! Typed Client calls retain Rc through actual host-driven Pending and wake.
use crate::application::record;
use futures::{
    channel::oneshot,
    future::{Either, poll_fn, select},
};
use solana_kit_client::methods::Balance;
use solana_kit_client::{Client, Error, Limits, TransportFailureKind};
use solana_kit_core::Address;
use solana_kit_leptoskit::GuestHttp;
use std::{future::Future, rc::Rc, task::Poll};

const LOCAL_SENTINEL: u8 = 42;

async fn balance(service: &str) -> Result<u64, Error> {
    let local = Rc::new(LOCAL_SENTINEL);
    let transport = GuestHttp::new(service).expect("installed named service");
    let client = Client::new(
        transport,
        Limits {
            bytes: 1024,
            depth: 16,
        },
    );
    let mut call = Box::pin(client.call(Balance(Address::from_bytes([0; 32]), None)));
    poll_fn(|context| {
        assert!(call.as_mut().poll(context).is_pending());
        record("pending");
        Poll::Ready(())
    })
    .await;
    let result = call.await;
    assert_eq!(*local, LOCAL_SENTINEL);
    record("resumed-local-rc");

    result.map(|balance| balance.value)
}

/// Exercises failures after actual suspension so typed errors cannot be stubbed.
pub(super) async fn run(cancellation: oneshot::Receiver<()>) {
    let denied = GuestHttp::new("denied").expect("installed denied service");
    let client = Client::new(denied, Limits::default());
    assert!(
        matches!(client.call(Balance(Address::from_bytes([0; 32]), None)).await,
        Err(Error::Transport(error)) if error.kind == TransportFailureKind::Denied)
    );
    record("denied");
    assert_eq!(
        balance("secure").await.expect("actual HTTPS balance"),
        u64::MAX
    );
    record("https-max");
    assert!(matches!(balance("status").await, Err(Error::Status(503))));
    record("status-rejected");

    assert!(matches!(balance("large").await,
        Err(Error::Transport(error)) if error.kind == TransportFailureKind::BodyLimit));
    record("body-bounded");
    assert!(matches!(balance("deadline").await,
        Err(Error::Transport(error)) if error.kind == TransportFailureKind::Deadline));
    record("quiet-deadline");
    crate::rest::run().await;
    let quiet = Box::pin(balance("quiet"));

    match select(quiet, cancellation).await {
        Either::Right((Ok(()), future)) => drop(future),
        _ => panic!("quiet operation must be cancelled by application event"),
    }

    record("quiet-cancelled");
    let _never = balance("quiet").await;
    panic!("pending unload work cannot complete");
}
