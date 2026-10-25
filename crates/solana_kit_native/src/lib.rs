#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod config;
mod endpoint;
mod exchange;
mod rest;
mod tls;

pub use config::{ConfigError, HttpConfig};
pub use hyper::http::{HeaderMap, HeaderName, HeaderValue};
pub use rest::NativeRest;
pub use tls::TlsRoots;

use endpoint::Endpoint;
use solana_kit_client::{Reply, Request, Transport, TransportError};
use std::{
    fmt,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio_rustls::rustls::ClientConfig;

/// Native HTTP/1 transport; each exchange exclusively owns one connection.
///
/// No runtime, pool, background driver, retry, redirect, proxy, cookie jar or
/// ambient credential lookup is installed. Dropping a polled exchange drops its
/// owned socket/TLS/HTTP work. Tokio's OS hostname resolution may finish after
/// cancellation; it cannot send an HTTP request after the exchange is dropped.
pub struct NativeHttp {
    endpoint: Endpoint,
    config: HttpConfig,
    tls: Arc<ClientConfig>,
}

impl NativeHttp {
    /// Validates an explicit absolute HTTP(S) endpoint and caller-owned policy.
    ///
    /// Credentials belong in `config.headers`; URL userinfo and fragments reject.
    /// Configuration failures never contain the supplied endpoint or headers.
    pub fn new(endpoint: &str, config: HttpConfig) -> Result<Self, ConfigError> {
        let endpoint = Endpoint::parse(endpoint)?;
        config.validate()?;
        let tls = config.tls.build()?;

        Ok(Self {
            endpoint,
            config,
            tls: Arc::new(tls),
        })
    }
}

impl fmt::Debug for NativeHttp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("NativeHttp").finish_non_exhaustive()
    }
}

/// Owned, cancel-on-drop native exchange retaining `Send` through generic clients.
///
/// Stable Rust cannot name an async-block GAT future. This wrapper boxes that
/// future once with an explicit `Send` bound; it never starts a detached task.
pub struct NativeExchange<'a> {
    future: Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + Send + 'a>>,
}

impl Future for NativeExchange<'_> {
    type Output = Result<Reply, TransportError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        self.future.as_mut().poll(context)
    }
}

impl Transport for NativeHttp {
    type Exchange<'a> = NativeExchange<'a>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        NativeExchange {
            future: Box::pin(exchange::execute(self, request)),
        }
    }
}

mod websocket;
mod websocket_config;
mod websocket_connect;
pub use websocket::{NativeSubscriptionSocket, NativeWebSocket, WsError};
pub use websocket_config::WsConfig;
