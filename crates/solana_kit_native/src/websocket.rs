//! Bounded native text WebSocket I/O for a separate caller-owned Connection.
use crate::{
    ConfigError, WsConfig,
    websocket_connect::{self, Socket},
};
use futures_util::{SinkExt, StreamExt};
use std::{error::Error, fmt};
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::{Error as SocketError, Message, error::ProtocolError};

/// Redacted socket failure. Peer content, endpoints and credentials are never retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsError {
    /// Invalid caller endpoint, budget or trust material.
    Config(ConfigError),
    /// Connect or operation deadline elapsed. The socket closes on an observed timeout.
    Deadline,
    /// Network I/O failed.
    Io,
    /// Certificate chain, destination name or TLS handshake failed.
    Tls,
    /// HTTP upgrade rejected or invalid. Redirects are never followed.
    Handshake,
    /// Malformed WebSocket protocol or text encoding.
    Protocol,
    /// Frame, message or write-buffer budget exceeded.
    Limit,
    /// Peer sent binary data on this text subscription socket.
    Binary,
    /// Peer ended TCP without a WebSocket close handshake.
    AbruptClose,
    /// This socket has already ended.
    Closed,
}

impl fmt::Display for WsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "WebSocket failure: {self:?}")
    }
}
impl Error for WsError {}

/// One persistent socket, with no owned tasks, runtime, pool, retry or reconnect.
///
/// Feed received text to a separate `subscriptions::Connection`. Its current-generation
/// ingest failure requires dropping this socket. The caller owns ticks and generations.
/// Dropping `connect` before completion closes its in-progress socket. Dropping a borrowed
/// `receive` keeps this owner and its bounded parser alive. Dropping a borrowed `send`
/// can leave a partially written message. Drop the owner after that cancellation and
/// never automatically retry an uncertain send. Dropping this owner closes its socket
/// immediately. Dropping consuming `close` also closes it. No remote unsubscribe is implied.
pub struct NativeWebSocket {
    socket: Option<Socket>,
    config: WsConfig,
}

impl fmt::Debug for NativeWebSocket {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeWebSocket")
            .finish_non_exhaustive()
    }
}

impl NativeWebSocket {
    /// Connects to an explicit ws/wss endpoint within one handshake deadline.
    ///
    /// Userinfo, fragments and unsupported forms reject. Headers are caller-owned.
    /// Tungstenite bounds handshake input at 65,536 bytes plus one bounded read chunk.
    /// Its fixed packet-count guard also applies. No ambient credentials are read.
    pub async fn connect(endpoint: &str, config: WsConfig) -> Result<Self, WsError> {
        config.validate(endpoint).map_err(WsError::Config)?;
        let parsed = websocket_connect::endpoint(endpoint).map_err(WsError::Config)?;
        let socket = timeout(
            config.connect_timeout,
            websocket_connect::open(endpoint, parsed, &config),
        )
        .await
        .map_err(|_| WsError::Deadline)??;
        Ok(Self {
            socket: Some(socket),
            config,
        })
    }

    /// Sends and flushes one text payload. Length is checked before copying.
    ///
    /// An observed I/O failure or deadline closes this owner. Cancelling the borrowed
    /// future retains the owner and may retain a bounded unsent or partial message.
    pub async fn send(&mut self, text: &str) -> Result<(), WsError> {
        if text.len() > self.config.max_frame_bytes {
            return Err(WsError::Limit);
        }

        let socket = self.socket.as_mut().ok_or(WsError::Closed)?;
        let result = timeout(
            self.config.operation_timeout,
            socket.send(Message::Text(text.into())),
        )
        .await
        .map_err(|_| WsError::Deadline)
        .and_then(|result| result.map_err(socket_error));
        self.finish(result)
    }

    /// Receives one complete text message. Ping/pong control frames are flushed.
    ///
    /// Graceful peer close returns `None` and releases the socket. Abrupt EOF is an
    /// error. A pending receive sleeps on Tokio I/O and is safe to cancel while retaining
    /// this owner. A completed timeout or error closes the socket.
    pub async fn receive(&mut self) -> Result<Option<String>, WsError> {
        let socket = self.socket.as_mut().ok_or(WsError::Closed)?;
        let result = timeout(self.config.operation_timeout, receive(socket))
            .await
            .map_err(|_| WsError::Deadline)
            .and_then(|result| result);

        if matches!(result, Ok(None)) {
            self.socket.take();
        }

        self.finish(result)
    }

    /// Consumes the owner, sends close and waits for peer close within one deadline.
    ///
    /// Cancelling this future drops its owned socket. Messages received while waiting
    /// for peer close are discarded within the same frame/message budgets.
    pub async fn close(mut self) -> Result<(), WsError> {
        let mut socket = self.socket.take().ok_or(WsError::Closed)?;
        timeout(self.config.operation_timeout, async {
            socket
                .send(Message::Close(None))
                .await
                .map_err(socket_error)?;
            while receive(&mut socket).await?.is_some() {}
            Ok(())
        })
        .await
        .map_err(|_| WsError::Deadline)?
    }

    fn finish<T>(&mut self, result: Result<T, WsError>) -> Result<T, WsError> {
        if result.is_err() {
            self.socket.take();
        }

        result
    }
}

async fn receive(socket: &mut Socket) -> Result<Option<String>, WsError> {
    loop {
        match socket.next().await {
            Some(Ok(Message::Text(text))) => return Ok(Some(text.to_string())),

            Some(Ok(Message::Close(_))) | None => match socket.flush().await {
                Ok(()) | Err(SocketError::ConnectionClosed) => return Ok(None),
                Err(error) => return Err(socket_error(error)),
            },

            Some(Ok(Message::Ping(_) | Message::Pong(_))) => {
                socket.flush().await.map_err(socket_error)?;
            }
            Some(Ok(Message::Binary(_))) => return Err(WsError::Binary),
            Some(Ok(Message::Frame(_))) => return Err(WsError::Protocol),
            Some(Err(SocketError::ConnectionClosed)) => return Ok(None),
            Some(Err(error)) => return Err(socket_error(error)),
        }
    }
}

pub(crate) fn socket_error(error: SocketError) -> WsError {
    match error {
        SocketError::Capacity(_) | SocketError::WriteBufferFull(_) | SocketError::AttackAttempt => {
            WsError::Limit
        }
        SocketError::Io(_) => WsError::Io,
        SocketError::Http(_) | SocketError::HttpFormat(_) | SocketError::Url(_) => {
            WsError::Handshake
        }
        SocketError::Protocol(ProtocolError::ResetWithoutClosingHandshake) => WsError::AbruptClose,
        SocketError::ConnectionClosed | SocketError::AlreadyClosed => WsError::Closed,
        _ => WsError::Protocol,
    }
}
