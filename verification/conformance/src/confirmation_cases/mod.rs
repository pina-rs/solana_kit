//! Bounded public confirmation policy, ownership, and submission fixtures.

mod lifecycle;
mod state;
mod submission;

use core::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

pub(crate) fn cases() -> [crate::CaseResult; 3] {
    [
        crate::case("confirmation-policy-ordering", state::run()),
        crate::case("confirmation-pending-cleanup", lifecycle::run()),
        crate::case("confirmation-exact-submission", submission::run()),
    ]
}

fn finish<F: Future>(future: F) -> Result<F::Output, &'static str> {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());

    for _ in 0..8 {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return Ok(value);
        }
    }

    Err("confirmation exceeded bounded fixture polling")
}
