//! Completion-time arbitration is shared by portal, storage and upload operations.

use solana_kit_client::subscriptions::{Clock, ClockError};
use std::{
    future::{Future, poll_fn},
    pin::pin,
    task::Poll,
};

pub(crate) enum Error {
    Clock(ClockError),
    Elapsed,
}

/// Both host polls and ready decoding can consume time; an elapsed deadline wins ties.
pub(crate) async fn within<F: Future, C: Clock>(
    operation: F,
    clock: &C,
    deadline: u64,
) -> Result<F::Output, Error> {
    if clock.now() >= deadline {
        return Err(Error::Elapsed);
    }

    let mut operation = pin!(operation);
    let mut timer = pin!(clock.sleep_until(deadline));
    poll_fn(|cx| {
        if clock.now() >= deadline {
            return Poll::Ready(Err(Error::Elapsed));
        }

        let timeout = timer.as_mut().poll(cx);

        if clock.now() >= deadline {
            return Poll::Ready(Err(Error::Elapsed));
        }

        match timeout {
            Poll::Ready(Ok(())) => return Poll::Ready(Err(Error::Elapsed)),
            Poll::Ready(Err(error)) => return Poll::Ready(Err(Error::Clock(error))),
            Poll::Pending => {}
        }

        let result = operation.as_mut().poll(cx);

        if clock.now() >= deadline {
            return Poll::Ready(Err(Error::Elapsed));
        }

        result.map(Ok)
    })
    .await
}
