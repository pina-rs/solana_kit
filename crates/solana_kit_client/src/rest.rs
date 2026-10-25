//! Single-exchange REST contracts for off-chain application clients.
//!
//! Destination identity is supplied by each provider. Adapters map that identity
//! to approved roots and caller-owned authentication. Request data cannot select
//! an origin, redirect policy, runtime or retry policy.

mod admission;
mod target;

use crate::{Reply, TransportError};
use std::{fmt, future::Future};

pub use admission::{Budgets, JsonError};
pub use target::request_target;

/// HTTP methods used by typed provider operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// Read a resource.
    Get,
    /// Submit once.
    Post,
    /// Replace a resource.
    Put,
    /// Update selected fields.
    Patch,
    /// Delete a resource.
    Delete,
}

impl Method {
    /// Exact HTTP token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
        }
    }
}

/// Body presence and media type cannot disagree.
pub enum Body {
    /// No body and no media type.
    Absent,
    /// Exact JSON bytes. A null document remains present.
    Json(Vec<u8>),
    /// Exact application/x-www-form-urlencoded bytes.
    Form(Vec<u8>),
}

impl Body {
    /// Borrows the body bytes without copying.
    pub fn as_deref(&self) -> Option<&[u8]> {
        match self {
            Self::Absent => None,
            Self::Json(bytes) | Self::Form(bytes) => Some(bytes),
        }
    }

    /// Returns the media type derived from the body representation.
    pub const fn content_type(&self) -> Option<&'static str> {
        match self {
            Self::Absent => None,
            Self::Json(_) => Some("application/json"),
            Self::Form(_) => Some("application/x-www-form-urlencoded"),
        }
    }

    /// Transfers present bytes, or the empty absent representation.
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Absent => Vec::new(),
            Self::Json(bytes) | Self::Form(bytes) => bytes,
        }
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Body")
            .field("content_type", &self.content_type())
            .finish_non_exhaustive()
    }
}

/// One owned request to a provider-defined approved destination.
pub struct Request<D> {
    /// Explicit approved destination identity, independent of request data.
    pub service: D,
    /// Exact HTTP method.
    pub method: Method,
    /// Ordered decoded segments, individually encoded by the adapter.
    pub path_segments: Vec<String>,
    /// Ordered decoded pairs. Duplicate keys remain distinct.
    pub query: Vec<(String, String)>,
    /// Explicit absent, JSON or form representation.
    pub body: Body,
    /// Maximum reply allocation enforced while receiving.
    pub max_reply_bytes: usize,
}

impl<D> fmt::Debug for Request<D> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RestRequest")
            .field("method", &self.method)
            .field("max_reply_bytes", &self.max_reply_bytes)
            .finish_non_exhaustive()
    }
}

/// A single HTTP exchange using a provider-specific destination identity.
///
/// Implementations own root, headers, TLS and deadlines, deny redirects, bound
/// receiving before copies and send once. Dropping the future cancels its owned
/// work. A task-owning host retains capacity until actual termination is joined.
/// Credentials and request/response values must stay out of Debug and errors.
pub trait Transport<D> {
    /// Concrete future. Local Rc-bearing futures need no universal Send bound.
    type Exchange<'a>: Future<Output = Result<Reply, TransportError>> + 'a
    where
        Self: 'a;

    /// Performs one exchange with no automatic retry or detached work.
    fn exchange(&self, request: Request<D>) -> Self::Exchange<'_>;
}

/// Retains an original adapter failure behind a redacted diagnostic boundary.
///
/// The category is unchanged. Explicit `Error::source` traversal retains the
/// original error; ordinary Debug and Display exclude origin and request values.
pub fn redact_error(error: TransportError) -> TransportError {
    TransportError::with_source(error.kind, RedactedCause(error))
}

struct RedactedCause(TransportError);

impl fmt::Debug for RedactedCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RestCause")
    }
}

impl fmt::Display for RedactedCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("REST adapter failure")
    }
}

impl std::error::Error for RedactedCause {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
