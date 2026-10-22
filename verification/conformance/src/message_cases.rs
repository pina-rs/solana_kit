//! Accepted message assertions and their fixed interpreter workload metadata.

use core::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

/// Fixed distinct-message workload owned by the accepted assertion source.
pub const MESSAGE_COUNT: usize = signer_capability_consumer::ACCEPTED_MESSAGES;
/// Each message is signed by its backend, signed by its oracle, and verified.
pub const CRYPTO_CALLS_PER_MESSAGE: u64 = 3;
/// Two key derivations and the snapshot's two deterministic signing calls.
pub const FIXED_CRYPTO_CALLS: u64 = 4;

pub(crate) fn cases() -> [crate::CaseResult; 2] {
    [
        crate::case(
            "message-snapshots",
            crate::require(
                signer_capability_consumer::message_snapshots(),
                "message snapshot bytes and signatures",
            ),
        ),
        crate::case("message-batches", batches()),
    ]
}

fn batches() -> Result<(), &'static str> {
    let mut future = pin!(signer_capability_consumer::message_batches());
    let mut context = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(success) => crate::require(success, "full distinct message batch"),
        Poll::Pending => Err("deterministic message backend unexpectedly pending"),
    }
}
