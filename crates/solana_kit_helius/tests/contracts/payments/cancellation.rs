//! Dropping preparation releases the actual pending exchange at either checkout stage.

use super::*;
use solana_kit_client::{Reply, TransportError};
use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

struct Paused {
    calls: Rc<Cell<usize>>,
    pending_drops: Rc<Cell<usize>>,
    pause_at: usize,
}

struct Exchange {
    response: Option<Reply>,
    pending: bool,
    pending_drops: Rc<Cell<usize>>,
    _request: ServiceRequest,
}

impl Future for Exchange {
    type Output = Result<Reply, TransportError>;

    fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        match self.response.take() {
            Some(reply) => Poll::Ready(Ok(reply)),
            None => Poll::Pending,
        }
    }
}

impl Drop for Exchange {
    fn drop(&mut self) {
        if self.pending {
            self.pending_drops.set(self.pending_drops.get() + 1);
        }
    }
}

impl ServiceTransport<Service> for Paused {
    type Exchange<'a> = Exchange;

    fn exchange(&self, request: ServiceRequest) -> Self::Exchange<'_> {
        let next = self.calls.get() + 1;
        self.calls.set(next);
        let pending = next == self.pause_at;

        Exchange {
            response: (!pending).then(|| reply(PREVIEW)),
            pending,
            pending_drops: Rc::clone(&self.pending_drops),
            _request: request,
        }
    }
}

/// Both preview and initialization are genuinely suspended before cancellation.
pub fn dropped_preparation() {
    for pause_at in [1, 2] {
        let calls = Rc::new(Cell::new(0));
        let pending_drops = Rc::new(Cell::new(0));
        let client = ServiceClient::new(
            Paused {
                calls: Rc::clone(&calls),
                pending_drops: Rc::clone(&pending_drops),
                pause_at,
            },
            Budgets::default(),
        );
        let mut future = Box::pin(create_payment(&client, request()));
        let mut context = Context::from_waker(futures::task::noop_waker_ref());
        assert!(future.as_mut().poll(&mut context).is_pending());
        assert_eq!(calls.get(), pause_at);
        assert_eq!(pending_drops.get(), 0);
        drop(future);
        assert_eq!(pending_drops.get(), 1);
        assert_eq!(calls.get(), pause_at);
    }
}
