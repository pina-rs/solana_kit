//! Observe actual local flush completion without assuming cross-socket arrival order.
use solana_kit_client::{Request, Transport, subscriptions::*};
use solana_kit_core::Signature;
use solana_kit_native::{NativeHttp, NativeSubscriptionSocket, WsError};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll},
};

/// Retains the real HTTP adapter's Send future and checks each signature separately.
pub(super) struct ObservedHttp {
    pub inner: NativeHttp,
    pub flushed: Arc<AtomicUsize>,
}

impl Transport for ObservedHttp {
    type Exchange<'a> = <NativeHttp as Transport>::Exchange<'a>;

    /// Checks the local send boundary before creating the actual HTTP exchange.
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        assert_registration_flushed(&request, self.flushed.load(Ordering::SeqCst));
        self.inner.exchange(request)
    }
}

/// Records a signature only after the actual native socket flush succeeds.
pub(super) struct ObservedSocket {
    inner: NativeSubscriptionSocket,
    flushed: Arc<AtomicUsize>,
    pending: usize,
}

impl ObservedSocket {
    /// Shares only the two test signatures' completed-flush bits with HTTP.
    pub fn new(inner: NativeSubscriptionSocket, flushed: Arc<AtomicUsize>) -> Self {
        Self {
            inner,
            flushed,
            pending: 0,
        }
    }
}

impl TextSocket for ObservedSocket {
    type Error = WsError;

    /// Admission alone does not establish a completed network write.
    fn start_send(&mut self, text: &str) -> Result<(), WsError> {
        let request: serde_json::Value = serde_json::from_str(text).unwrap();
        self.inner.start_send(text)?;
        self.pending = if request["method"] == "signatureSubscribe" {
            signature_bit(&request["params"][0])
        } else {
            0
        };

        Ok(())
    }

    /// Pending and failed flushes never make a signature eligible for HTTP.
    fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), WsError>> {
        core::task::ready!(self.inner.poll_flush(cx))?;
        self.flushed.fetch_or(self.pending, Ordering::SeqCst);
        self.pending = 0;

        Poll::Ready(Ok(()))
    }

    /// Preserves actual incoming frames and the adapter's finite I/O budgets.
    fn poll_receive(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<ReceivedFrame>, WsError>> {
        self.inner.poll_receive(cx)
    }

    /// Preserves the real socket's acknowledged close behavior.
    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), WsError>> {
        self.inner.poll_close(cx)
    }
}

/// Keeps the two concurrent confirmation registrations distinct.
fn signature_bit(value: &serde_json::Value) -> usize {
    let signature = value.as_str().unwrap();

    if signature == super::SIGNATURE {
        return 1;
    }

    assert_eq!(signature, Signature::from_bytes([1; 64]).to_string());
    2
}

/// Checks the promised local order rather than the peer's scheduling order.
fn assert_registration_flushed(request: &Request, flushed: usize) {
    let body: serde_json::Value = serde_json::from_slice(&request.body).unwrap();

    if body["method"] == "getSignatureStatuses" {
        let signatures = body["params"][0].as_array().unwrap();
        assert_eq!(signatures.len(), 1);
        assert_ne!(
            flushed & signature_bit(&signatures[0]),
            0,
            "HTTP before signature flush"
        );
    }
}

/// Another watch's completed flush cannot satisfy the current signature's guard.
#[test]
fn http_rejects_missing_or_other_signature_flush() {
    let request = Request {
        body: serde_json::to_vec(&serde_json::json!({
            "method":"getSignatureStatuses", "params":[[super::SIGNATURE]]
        }))
        .unwrap(),
        max_reply_bytes: 1024,
    };

    for flushed in [0, 2] {
        assert!(
            std::panic::catch_unwind(|| assert_registration_flushed(&request, flushed)).is_err()
        );
    }

    assert_registration_flushed(&request, 1);
    assert_registration_flushed(&request, 3);
}
