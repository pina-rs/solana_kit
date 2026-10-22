//! Executed async boundary proofs and a compile-negative spawn control.
//!
//! A local transport retains Rc through its pending exchange. Creating it inside
//! the spawned block rules out rejection merely from moving Rc into that block.
//!
//! ```compile_fail,E0277
//! use std::rc::Rc;
//! use solana_kit_async_boundaries::{LocalTransport, request};
//! use solana_kit_boundary_contract::Client;
//!
//! tokio::spawn(async {
//!     let client = Client::new(LocalTransport::new(Rc::new(7)));
//!     client.call(request()).await
//! });
//! ```

#![forbid(unsafe_code)]

use futures::channel::oneshot;
use solana_kit_boundary_contract::{Reply, Request, Transport};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

/// Fixed request used by both native and local generic clients.
pub fn request() -> Request {
    Request {
        id: 41,
        body: 314_u64.to_le_bytes().to_vec(),
    }
}

/// Local adapter deliberately holds Rc across a yielding await boundary.
pub struct LocalTransport(Rc<u64>);

impl LocalTransport {
    /// Selects explicitly supplied single-thread state.
    pub fn new(state: Rc<u64>) -> Self {
        Self(state)
    }
}

/// Concrete local future retains its borrow and Rc after its first Pending.
pub struct LocalExchange<'a> {
    state: &'a Rc<u64>,
    request: Option<Request>,
    yielded: bool,
}

impl Future for LocalExchange<'_> {
    type Output = Result<Reply, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.yielded {
            self.yielded = true;
            cx.waker().wake_by_ref();

            return Poll::Pending;
        }

        assert_eq!(**self.state, 7);
        let request = self
            .request
            .take()
            .expect("future is polled only before completion");

        Poll::Ready(Ok(Reply {
            id: request.id,
            body: request.body,
        }))
    }
}

impl Transport for LocalTransport {
    type Error = ();
    type Exchange<'a> = LocalExchange<'a>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        LocalExchange {
            state: &self.0,
            request: Some(request),
            yielded: false,
        }
    }
}

/// A task owner must abort explicitly: JoinHandle's own Drop detaches.
pub struct OwnedTask<T>(tokio::task::JoinHandle<T>);

impl<T> OwnedTask<T> {
    /// Takes responsibility for aborting all work retained by this task.
    pub fn new(handle: tokio::task::JoinHandle<T>) -> Self {
        Self(handle)
    }
}

impl<T> Drop for OwnedTask<T> {
    fn drop(&mut self) {
        self.0.abort();
    }
}

/// Races a quiet operation against an explicit cancellation wake source.
pub async fn cancellable<F: Future>(
    operation: F,
    cancel: oneshot::Receiver<()>,
) -> Option<F::Output> {
    let operation = Box::pin(operation);
    let cancel = Box::pin(cancel);

    match futures::future::select(operation, cancel).await {
        futures::future::Either::Left((result, _)) => Some(result),
        futures::future::Either::Right((_, _)) => None,
    }
}
