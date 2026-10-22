//! Explicit budgets for one caller-owned persistent WebSocket.
use crate::{ConfigError, HeaderMap, TlsRoots};
use std::{
    fmt,
    time::{Duration, Instant},
};
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

const DEFAULT_MESSAGE: usize = 1024 * 1024;
const DEFAULT_READ_BUFFER: usize = 8192;
const DEFAULT_REQUEST: usize = 16_384;
const MAX_FRAME_HEADER: usize = 14;

/// Caller-selected authentication, trust and finite socket budgets.
#[derive(Clone)]
pub struct WsConfig {
    /// Explicit application/authentication headers. Protocol and cookie headers reject.
    pub headers: HeaderMap,
    /// Total DNS, TCP, TLS and WebSocket handshake deadline.
    pub connect_timeout: Duration,
    /// Deadline for each send, receive or complete graceful close.
    pub operation_timeout: Duration,
    /// Largest incoming frame and outgoing text payload.
    pub max_frame_bytes: usize,
    /// Largest reassembled incoming text message.
    pub max_message_bytes: usize,
    /// Read buffer allocation. Must be positive and at most the frame budget.
    pub read_buffer_bytes: usize,
    /// Maximum retained encoded write bytes, including frame headers.
    pub max_write_buffer_bytes: usize,
    /// Bound for endpoint and explicitly supplied header bytes together.
    pub max_request_bytes: usize,
    /// Explicit roots. Ordinary certificate and hostname verification stays enabled.
    pub tls: TlsRoots,
}

impl Default for WsConfig {
    fn default() -> Self {
        Self {
            headers: HeaderMap::new(),
            connect_timeout: Duration::from_secs(10),
            operation_timeout: Duration::from_secs(30),
            max_frame_bytes: DEFAULT_MESSAGE,
            max_message_bytes: DEFAULT_MESSAGE,
            read_buffer_bytes: DEFAULT_READ_BUFFER,
            max_write_buffer_bytes: DEFAULT_MESSAGE + MAX_FRAME_HEADER,
            max_request_bytes: DEFAULT_REQUEST,
            tls: TlsRoots::WebPki,
        }
    }
}

impl fmt::Debug for WsConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WsConfig")
            .field("max_frame_bytes", &self.max_frame_bytes)
            .field("max_message_bytes", &self.max_message_bytes)
            .field("max_write_buffer_bytes", &self.max_write_buffer_bytes)
            .finish_non_exhaustive()
    }
}

impl WsConfig {
    pub(crate) fn validate(&self, endpoint: &str) -> Result<(), ConfigError> {
        deadline(self.connect_timeout)?;
        deadline(self.operation_timeout)?;
        self.validate_buffers()?;
        self.validate_headers(endpoint)
    }

    fn validate_buffers(&self) -> Result<(), ConfigError> {
        if self.read_buffer_bytes == 0
            || self.read_buffer_bytes > self.max_frame_bytes
            || self.max_message_bytes == 0
            || self.max_frame_bytes == 0
            || self
                .max_frame_bytes
                .checked_add(MAX_FRAME_HEADER)
                .is_none_or(|size| size > self.max_write_buffer_bytes)
        {
            return Err(ConfigError("invalid WebSocket buffer budgets"));
        }

        Ok(())
    }

    fn validate_headers(&self, endpoint: &str) -> Result<(), ConfigError> {
        let mut remaining = self
            .max_request_bytes
            .checked_sub(endpoint.len())
            .ok_or(ConfigError("WebSocket request budget exceeded"))?;

        for (name, value) in &self.headers {
            if reserved(name.as_str()) {
                return Err(ConfigError("reserved WebSocket header override"));
            }

            remaining = remaining
                .checked_sub(name.as_str().len())
                .and_then(|remaining| remaining.checked_sub(value.len()))
                .ok_or(ConfigError("WebSocket request budget exceeded"))?;
        }

        Ok(())
    }

    pub(crate) fn protocol(&self) -> WebSocketConfig {
        WebSocketConfig::default()
            .read_buffer_size(self.read_buffer_bytes)
            .write_buffer_size(0)
            .max_write_buffer_size(self.max_write_buffer_bytes)
            .max_frame_size(Some(self.max_frame_bytes))
            .max_message_size(Some(self.max_message_bytes))
    }
}

fn deadline(duration: Duration) -> Result<(), ConfigError> {
    if duration.is_zero() || Instant::now().checked_add(duration).is_none() {
        return Err(ConfigError("invalid WebSocket deadline"));
    }

    Ok(())
}

fn reserved(name: &str) -> bool {
    name.starts_with("sec-websocket-")
        || [
            "host",
            "connection",
            "upgrade",
            "content-length",
            "transfer-encoding",
            "proxy-authorization",
            "proxy-connection",
            "cookie",
            "trailer",
            "expect",
        ]
        .contains(&name)
}
