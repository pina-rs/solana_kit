//! Private off-chain RPC design pilot. Never use in on-chain programs.
#![forbid(unsafe_code)]

pub mod alternatives;
pub mod json;
pub mod methods;

use std::future::Future;

use json::{Error, Limits};
use methods::Method;

/// An adapter receives owned bytes and must bound its receive buffer to this limit.
#[derive(Debug)]
pub struct Request {
    pub body: Vec<u8>,
    pub max_reply_bytes: usize,
}

/// Status handling precedes JSON decoding; non-success cannot masquerade as a result.
#[derive(Debug)]
pub struct Reply {
    pub status: u16,
    pub body: Vec<u8>,
}

/// Prototype host failures. The public adapter contract must retain detailed sources.
#[derive(Debug, PartialEq, Eq)]
pub enum TransportError {
    Denied,
    Io,
    BodyLimit,
}

/// Concrete futures retain native Send or guest-local Rc as appropriate.
pub trait Transport {
    type Exchange<'a>: Future<Output = Result<Reply, TransportError>> + 'a
    where
        Self: 'a;

    fn exchange(&self, request: Request) -> Self::Exchange<'_>;
}

/// One protocol owner shared by every concrete transport.
pub struct Client<T> {
    transport: T,
    limits: Limits,
}

impl<T: Transport> Client<T> {
    pub const fn new(transport: T, limits: Limits) -> Self {
        Self { transport, limits }
    }

    /// Executes typed decoding after the actual associated exchange completes.
    pub async fn call<R: Method>(&self, id: u64, method: R) -> Result<R::Response, Error> {
        let request = json::request(id, &method, self.limits)?;
        let reply = self.transport.exchange(request).await?;
        json::response(id, reply, self.limits)
    }
}
