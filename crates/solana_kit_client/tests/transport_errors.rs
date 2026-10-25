#![cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]

use solana_kit_client::{Error, TransportError, TransportFailureKind as Kind, rest};
use std::{
    error::Error as StdError,
    fmt,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

struct NativeCause(Arc<AtomicUsize>);

impl fmt::Debug for NativeCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("cause-private")
    }
}

impl fmt::Display for NativeCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("cause-private")
    }
}

impl StdError for NativeCause {}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_errors_and_pending_futures_keep_send_and_original_source() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TransportError>();
    assert_send_sync::<Error>();
    assert_send_sync::<solana_kit_client::subscriptions::ClockError>();

    let original = Arc::new(AtomicUsize::new(17));
    let failure = rest::redact_error(TransportError::with_source(
        Kind::Io,
        NativeCause(Arc::clone(&original)),
    ));
    let error = tokio::spawn(async move {
        let error = Error::from(failure);
        tokio::task::yield_now().await;
        error
    })
    .await
    .unwrap();
    assert!(!format!("{error:?} {error}").contains("cause-private"));

    let mut source: &(dyn StdError + 'static) = &error;

    while let Some(next) = source.source() {
        source = next;
    }

    let retained = source.downcast_ref::<NativeCause>().unwrap();
    assert!(Arc::ptr_eq(&original, &retained.0));
    original.store(29, Ordering::SeqCst);
    assert_eq!(retained.0.load(Ordering::SeqCst), 29);
    assert_eq!(source.to_string(), "cause-private");
}

#[test]
fn source_free_failures_keep_their_category_and_empty_chain() {
    for kind in [
        Kind::Unavailable,
        Kind::Denied,
        Kind::Deadline,
        Kind::Cancelled,
        Kind::Io,
        Kind::BodyLimit,
        Kind::HeaderLimit,
        Kind::InvalidResponse,
    ] {
        let error = TransportError::new(kind);
        assert_eq!(error.kind, kind);
        assert!(error.source().is_none());
    }
}
