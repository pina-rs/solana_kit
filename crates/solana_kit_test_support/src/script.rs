//! Finite deterministic transport scripts with ordered request capture.

use crate::{
    lock::lock,
    pending::{PendingExchange, PendingFuture, PendingState},
};
use solana_kit_client::{Reply, Request, Transport, TransportError, TransportFailureKind};
use std::{
    collections::VecDeque,
    fmt,
    future::{Future, Ready, ready},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

/// One request observed by [`ScriptedTransport`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapturedRequest {
    /// Correlation ID parsed from the exact request body.
    pub id: u64,
    /// JSON-RPC method name parsed from the exact request body.
    pub method: String,
    /// Exact request body bytes sent by the client.
    pub body: Vec<u8>,
    /// Receive budget requested by the exchange.
    pub max_reply_bytes: usize,
}

/// A finite transport that returns one scripted reply per exchange.
///
/// Clones share one script and request log, so a test can retain a handle while
/// the client owns another. Every exchange consumes exactly one step; there is no
/// retry, timer, runtime, network access or global state. A request after the
/// script ends produces a clear `Unavailable` transport failure with an
/// [`UnexpectedRequest`] source.
#[derive(Clone)]
pub struct ScriptedTransport {
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    steps: VecDeque<ScriptStep>,
    requests: Vec<CapturedRequest>,
    scripted: usize,
}

impl ScriptedTransport {
    /// Creates a finite script in exact exchange order.
    #[must_use]
    pub fn new(steps: impl IntoIterator<Item = ScriptStep>) -> Self {
        let steps: VecDeque<_> = steps.into_iter().collect();
        let inner = Inner {
            scripted: steps.len(),
            steps,
            requests: Vec::new(),
        };

        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    /// Returns every captured request in exchange order.
    #[must_use]
    pub fn requests(&self) -> Vec<CapturedRequest> {
        lock(&self.inner).requests.clone()
    }

    /// Returns the number of unconsumed scripted replies.
    #[must_use]
    pub fn remaining(&self) -> usize {
        lock(&self.inner).steps.len()
    }
}

impl Default for ScriptedTransport {
    fn default() -> Self {
        Self::new([])
    }
}

impl fmt::Debug for ScriptedTransport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = lock(&self.inner);

        formatter
            .debug_struct("ScriptedTransport")
            .field("remaining", &inner.steps.len())
            .field("requests", &inner.requests.len())
            .finish()
    }
}

impl Transport for ScriptedTransport {
    type Exchange<'a> = Exchange;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let captured = capture(request);
        let (step, scripted) = {
            let mut inner = lock(&self.inner);
            let step = inner.steps.pop_front();
            let scripted = inner.scripted;
            inner.requests.push(captured.clone());
            (step, scripted)
        };

        match step {
            Some(step) => step.exchange(captured.id),
            None => Exchange(ExchangeInner::Ready(ready(Err(exhausted(
                captured, scripted,
            ))))),
        }
    }
}

/// One scripted exchange outcome.
#[derive(Debug)]
pub struct ScriptStep(Step);

#[derive(Debug)]
enum Step {
    Success(Vec<u8>),
    Remote {
        code: i64,
        message: String,
        data: Option<Vec<u8>>,
    },
    Raw {
        status: u16,
        body: Vec<u8>,
    },
    Failure(TransportError),
    Pending(Arc<Mutex<PendingState>>),
}

impl ScriptStep {
    /// Correlates a success envelope around exact result JSON bytes.
    #[must_use]
    pub fn success(result: impl Into<Vec<u8>>) -> Self {
        Self(Step::Success(result.into()))
    }

    /// Correlates a remote JSON-RPC error without a `data` member.
    #[must_use]
    pub fn remote_error(code: i64, message: impl Into<String>) -> Self {
        Self(Step::Remote {
            code,
            message: message.into(),
            data: None,
        })
    }

    /// Correlates a remote JSON-RPC error retaining exact `data` JSON bytes.
    #[must_use]
    pub fn remote_error_with_data(
        code: i64,
        message: impl Into<String>,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self(Step::Remote {
            code,
            message: message.into(),
            data: Some(data.into()),
        })
    }

    /// Returns an exact HTTP status and body without rewriting correlation.
    #[must_use]
    pub fn raw(status: u16, body: impl Into<Vec<u8>>) -> Self {
        Self(Step::Raw {
            status,
            body: body.into(),
        })
    }

    /// Returns one typed transport failure.
    #[must_use]
    pub fn failure(error: TransportError) -> Self {
        Self(Step::Failure(error))
    }

    /// Creates one pending exchange and its control handle.
    ///
    /// The exchange registers a waker on first poll. The handle can wake that
    /// waker, complete the exchange, and observe delivery or cancellation.
    #[must_use]
    pub fn pending() -> (Self, PendingExchange) {
        let (state, control) = PendingExchange::channel();

        (Self(Step::Pending(state)), control)
    }

    fn exchange(self, id: u64) -> Exchange {
        match self.0 {
            Step::Success(result) => Exchange(ExchangeInner::Ready(ready(Ok(Reply {
                status: 200,
                body: success_body(id, result),
            })))),
            Step::Remote {
                code,
                message,
                data,
            } => Exchange(ExchangeInner::Ready(ready(Ok(Reply {
                status: 200,
                body: remote_body(id, code, message, data),
            })))),
            Step::Raw { status, body } => {
                Exchange(ExchangeInner::Ready(ready(Ok(Reply { status, body }))))
            }
            Step::Failure(error) => Exchange(ExchangeInner::Ready(ready(Err(error)))),
            Step::Pending(state) => Exchange(ExchangeInner::Pending(PendingFuture::new(state, id))),
        }
    }
}

/// Owned future returned by [`ScriptedTransport`].
pub struct Exchange(ExchangeInner);

enum ExchangeInner {
    Ready(Ready<Result<Reply, TransportError>>),
    Pending(PendingFuture),
}

impl Future for Exchange {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        match &mut self.get_mut().0 {
            ExchangeInner::Ready(future) => Pin::new(future).poll(context),
            ExchangeInner::Pending(future) => Pin::new(future).poll(context),
        }
    }
}

/// A request that exceeded a finite script.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnexpectedRequest {
    /// Unmatched request, including its exact body and parsed identity.
    pub request: CapturedRequest,
    /// Number of replies configured by the script.
    pub scripted: usize,
}

impl fmt::Display for UnexpectedRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "scripted transport has no reply for request {} ({}) after {} scripted exchanges",
            self.request.id, self.request.method, self.scripted
        )
    }
}

impl std::error::Error for UnexpectedRequest {}

fn capture(request: Request) -> CapturedRequest {
    let value: serde_json::Value = serde_json::from_slice(&request.body)
        .expect("scripted transport requires a client-framed JSON request");
    let id = value
        .get("id")
        .and_then(serde_json::Value::as_u64)
        .expect("scripted transport requires a request ID");
    let method = value
        .get("method")
        .and_then(serde_json::Value::as_str)
        .expect("scripted transport requires a method name")
        .to_owned();

    CapturedRequest {
        id,
        method,
        body: request.body,
        max_reply_bytes: request.max_reply_bytes,
    }
}

fn exhausted(request: CapturedRequest, scripted: usize) -> TransportError {
    TransportError::with_source(
        TransportFailureKind::Unavailable,
        UnexpectedRequest { request, scripted },
    )
}

pub(crate) fn success_body(id: u64, result: Vec<u8>) -> Vec<u8> {
    let mut body = envelope_prefix(id, "result");
    body.extend_from_slice(&result);
    body.push(b'}');
    body
}

pub(crate) fn remote_body(id: u64, code: i64, message: String, data: Option<Vec<u8>>) -> Vec<u8> {
    let message = serde_json::to_vec(&message).expect("serialize remote error message");
    let mut body =
        format!(r#"{{"jsonrpc":"2.0","id":{id},"error":{{"code":{code},"message":"#).into_bytes();
    body.extend_from_slice(&message);

    if let Some(data) = data {
        body.extend_from_slice(b",\"data\":");
        body.extend_from_slice(&data);
    }

    body.extend_from_slice(b"}}");
    body
}

fn envelope_prefix(id: u64, member: &str) -> Vec<u8> {
    format!(r#"{{"jsonrpc":"2.0","id":{id},"{member}":"#).into_bytes()
}
