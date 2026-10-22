//! Private executor-independent contract experiment; not a published client API.

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

extern crate alloc;

use alloc::vec::Vec;
use core::future::Future;

/// Owned request data; an adapter must not retain a borrowed caller buffer.
#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    /// Correlation value assigned by the caller.
    pub id: u64,
    /// Host-neutral wire data.
    pub body: Vec<u8>,
}

/// Host-neutral response before client correlation validation.
#[derive(Debug, PartialEq, Eq)]
pub struct Reply {
    /// Echoed request correlation value.
    pub id: u64,
    /// Response data.
    pub body: Vec<u8>,
}

/// Adapter boundary whose concrete future preserves its own auto traits.
pub trait Transport {
    /// An observable transport failure.
    type Error;
    /// Owned operation: dropping it must stop locally owned work.
    type Exchange<'a>: Future<Output = Result<Reply, Self::Error>> + 'a
    where
        Self: 'a;

    /// Starts only when polled; no runtime or task is started by the client.
    fn exchange(&self, request: Request) -> Self::Exchange<'_>;
}

/// Protocol validation and transport errors stay distinguishable.
#[derive(Debug, PartialEq, Eq)]
pub enum CallError<E> {
    /// The adapter reported a failure.
    Transport(E),
    /// Response belongs to another request.
    Correlation,
    /// The reply did not encode exactly one little-endian u64.
    MalformedReply,
}

/// Minimal generic client exercising real response validation across adapters.
pub struct Client<T>(T);

impl<T: Transport> Client<T> {
    /// Owns an explicitly selected adapter.
    pub const fn new(transport: T) -> Self {
        Self(transport)
    }

    /// Executes an exchange and rejects a reply with another request's identity.
    pub async fn call(&self, request: Request) -> Result<u64, CallError<T::Error>> {
        let id = request.id;
        let reply = self
            .0
            .exchange(request)
            .await
            .map_err(CallError::Transport)?;

        if reply.id != id {
            return Err(CallError::Correlation);
        }

        let bytes = reply
            .body
            .try_into()
            .map_err(|_| CallError::MalformedReply)?;

        Ok(u64::from_le_bytes(bytes))
    }
}

/// Entropy is a capability even when host delivery is asynchronous.
pub trait Entropy {
    /// Host capability failure.
    type Error;
    /// No universal Send bound: a guest may retain local host state.
    type Fill<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Fills all bytes or reports failure; never substitutes zeroes.
    fn fill<'a>(&'a self, bytes: &'a mut [u8]) -> Self::Fill<'a>;
}

/// Monotonic timer capability; ticks are meaningful only to this clock.
pub trait Clock {
    /// Capability failure.
    type Error;
    /// Completion must be explicitly driven by the injected clock.
    type Sleep<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Suspends until the injected clock reaches the supplied tick.
    fn sleep_until(&self, tick: u64) -> Self::Sleep<'_>;
}
