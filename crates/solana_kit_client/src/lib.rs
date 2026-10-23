#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

/// Explicit portable transaction confirmation strategies.
pub mod confirmation;
mod errors;
/// Validated transaction accounts and ordered outer/inner instruction traces.
pub mod introspection;
mod json;

/// Caller-owned instruction plan execution.
pub mod plans;

pub mod methods;
/// Caller-owned bounded subscription protocol.
pub mod subscriptions;

mod validate;
mod wire;

use std::future::Future;
use std::sync::atomic::{AtomicU32, Ordering};

pub use errors::{Error, RemoteError, TransportError, TransportFailureKind};
pub use json::{Limits, Presence};
use methods::Method;
use serde_json::value::RawValue;

/// Owned JSON request and receive budget for one exchange.
#[derive(Debug)]
pub struct Request {
    /// Exact UTF-8 JSON body to POST with application/json content type.
    pub body: Vec<u8>,
    /// Maximum response body allocation; adapters enforce this while receiving.
    pub max_reply_bytes: usize,
}

/// Owned HTTP status and complete bounded body from one exchange.
#[derive(Debug)]
pub struct Reply {
    /// HTTP response status, checked before JSON decoding.
    pub status: u16,
    /// Exact response body, including non-success responses.
    pub body: Vec<u8>,
}

/// Executor-neutral HTTP adapter with concrete, possibly local futures.
///
/// The adapter owns endpoint, caller authentication, header limits and deadlines.
/// It must bound the body during reception, preserve failures, and cancel owned
/// work when its exchange is dropped. Drivers require explicit cancel-on-drop
/// ownership; dropping a Tokio JoinHandle merely detaches it. Do not retry calls
/// automatically. Separate clients sharing a multiplexed connection require
/// separate adapter correlation namespaces because IDs are unique per client.
pub trait Transport {
    /// One exchange; native implementations may retain Send without requiring it
    /// of browser or interpreted-guest implementations.
    type Exchange<'a>: Future<Output = Result<Reply, TransportError>> + 'a
    where
        Self: 'a;

    /// Posts one owned request and receives its bounded response.
    fn exchange(&self, request: Request) -> Self::Exchange<'_>;
}

/// Protocol and request-ID owner for one transport namespace.
///
/// IDs range from 1 through u32::MAX - 1, remain exactly representable in browser
/// numbers, and never wrap. Exhaustion requires constructing a fresh namespace.
/// No runtime, task, ambient credentials or retry policy is started by this type.
pub struct Client<T> {
    transport: T,
    limits: Limits,
    next_id: AtomicU32,
}

impl<T: Transport> Client<T> {
    /// Creates one independently correlated client with explicit parsing budgets.
    pub const fn new(transport: T, limits: Limits) -> Self {
        Self {
            transport,
            limits,
            next_id: AtomicU32::new(1),
        }
    }

    /// Executes a supported typed operation through the concrete adapter future.
    ///
    /// Errors distinguish transport, HTTP, framing, correlation, remote protocol
    /// and typed response failures. Absence is retained by the method result.
    pub async fn call<R: Method>(&self, method: R) -> Result<R::Response, Error> {
        let id = self.reserve_id()?;
        let request = json::request(id, &method, self.limits)?;
        let reply = self.transport.exchange(request).await?;
        let result: Box<RawValue> = json::response(id, reply, self.limits)?;
        method.decode(result.get())
    }

    fn reserve_id(&self) -> Result<u64, Error> {
        let mut current = self.next_id.load(Ordering::Relaxed);

        loop {
            let next = current.checked_add(1).ok_or(Error::IdExhausted)?;

            match self.next_id.compare_exchange_weak(
                current,
                next,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(id) => return Ok(u64::from(id)),
                Err(actual) => current = actual,
            }
        }
    }
}

#[cfg(test)]
mod id_tests;
#[cfg(test)]
mod protocol_tests;
