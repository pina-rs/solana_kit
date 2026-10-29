//! Dropping estimation cancels the suspended transport exchange and its owned request.

use super::{WIRE, assert_frame, read};
use solana_kit_client::{Client, Limits, Reply, Request, Transport, TransportError};
use solana_kit_helius::transactions::get_compute_units;
use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

struct PendingRpc {
    calls: Rc<Cell<usize>>,
    dropped: Rc<Cell<usize>>,
}

struct Exchange {
    request: Request,
    dropped: Rc<Cell<usize>>,
}

impl Transport for PendingRpc {
    type Exchange<'a> = Exchange;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        self.calls.set(self.calls.get() + 1);
        Exchange {
            request,
            dropped: Rc::clone(&self.dropped),
        }
    }
}

impl Future for Exchange {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        assert_frame(&self.request);
        Poll::Pending
    }
}

impl Drop for Exchange {
    fn drop(&mut self) {
        self.dropped.set(self.dropped.get() + 1);
    }
}

pub(super) fn dropped_exchange() {
    let calls = Rc::new(Cell::new(0));
    let dropped = Rc::new(Cell::new(0));
    let client = Client::new(
        PendingRpc {
            calls: Rc::clone(&calls),
            dropped: Rc::clone(&dropped),
        },
        Limits::default(),
    );
    let mut future = Box::pin(get_compute_units(&client, WIRE, read()));
    assert_eq!(calls.get(), 0);
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert_eq!(calls.get(), 1);
    assert_eq!(dropped.get(), 0);
    drop(future);
    assert_eq!(dropped.get(), 1);
}
