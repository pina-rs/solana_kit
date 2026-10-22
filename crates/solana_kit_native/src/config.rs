//! Explicit resource policy, independent of endpoint and TLS material.
use crate::TlsRoots;
use hyper::http::HeaderMap;
use std::{
    error::Error,
    fmt,
    time::{Duration, Instant},
};

const DEFAULT_CONNECT: Duration = Duration::from_secs(10);
const DEFAULT_EXCHANGE: Duration = Duration::from_secs(30);
const DEFAULT_HEADER_BYTES: usize = 16_384;
const DEFAULT_HEADER_COUNT: usize = 64;
const MIN_READ_BUFFER: usize = 8192;

/// Invalid caller configuration; diagnostics exclude supplied secret values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigError(pub(crate) &'static str);

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl Error for ConfigError {}

/// Per-transport resource policy, supplied explicitly to [`crate::NativeHttp::new`].
#[derive(Clone)]
pub struct HttpConfig {
    /// Caller authentication and application headers; reserved framing headers reject.
    pub headers: HeaderMap,
    /// DNS, TCP and TLS handshake deadline within the overall exchange deadline.
    pub connect_timeout: Duration,
    /// Total deadline through complete response body reception, starting on poll.
    pub exchange_timeout: Duration,
    /// Parser limit for response status line plus headers, and chunk trailers.
    pub max_header_bytes: usize,
    /// Parser limit for response header fields.
    pub max_headers: usize,
    /// Maximum Hyper read buffer; must be at least 8192 and cover the header limit.
    pub max_read_buffer: usize,
    /// Caller-selected roots; certificate/name verification cannot be disabled.
    pub tls: TlsRoots,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            headers: HeaderMap::new(),
            connect_timeout: DEFAULT_CONNECT,
            exchange_timeout: DEFAULT_EXCHANGE,
            max_header_bytes: DEFAULT_HEADER_BYTES,
            max_headers: DEFAULT_HEADER_COUNT,
            max_read_buffer: DEFAULT_HEADER_BYTES,
            tls: TlsRoots::WebPki,
        }
    }
}

impl fmt::Debug for HttpConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HttpConfig")
            .field("connect_timeout", &self.connect_timeout)
            .field("exchange_timeout", &self.exchange_timeout)
            .field("max_header_bytes", &self.max_header_bytes)
            .field("max_headers", &self.max_headers)
            .field("max_read_buffer", &self.max_read_buffer)
            .finish_non_exhaustive()
    }
}

impl HttpConfig {
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        validate_timeout(self.connect_timeout)?;
        validate_timeout(self.exchange_timeout)?;
        validate_parser(self)?;

        if self.headers.keys().any(|name| reserved(name.as_str())) {
            return Err(ConfigError("reserved HTTP header override"));
        }

        Ok(())
    }
}

fn validate_timeout(timeout: Duration) -> Result<(), ConfigError> {
    if timeout.is_zero() || Instant::now().checked_add(timeout).is_none() {
        return Err(ConfigError("timeouts must be positive and representable"));
    }

    Ok(())
}

fn validate_parser(config: &HttpConfig) -> Result<(), ConfigError> {
    if config.max_header_bytes == 0
        || config.max_headers == 0
        || config.max_read_buffer < MIN_READ_BUFFER
        || config.max_header_bytes > config.max_read_buffer
    {
        return Err(ConfigError("invalid HTTP parser budgets"));
    }

    Ok(())
}

fn reserved(name: &str) -> bool {
    const RESERVED: &[&str] = &[
        "host",
        "content-length",
        "content-type",
        "transfer-encoding",
        "connection",
        "upgrade",
        "trailer",
        "expect",
        "proxy-authorization",
        "proxy-connection",
        "keep-alive",
        "te",
    ];
    RESERVED.contains(&name)
}
