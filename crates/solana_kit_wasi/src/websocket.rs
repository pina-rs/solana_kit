//! Bounded text sockets with application-owned Preview2 readiness.

use crate::{
    Endpoint, Reactor,
    socket_io::{Stream, Tcp, wait},
};
use rustls::{
    ClientConfig, ClientConnection, RootCertStore,
    pki_types::{CertificateDer, ServerName},
};
use std::{io::ErrorKind, sync::Arc, time::Duration};
use tungstenite::{
    Error, HandshakeError, Message, WebSocket, client::IntoClientRequest,
    handshake::client::ClientHandshake, protocol::WebSocketConfig,
};
use wasip2::{clocks::monotonic_clock, sockets::network::ErrorCode};

const DEFAULT_MESSAGE_BYTES: usize = 1_048_576;
const FRAME_HEADER_BYTES: usize = 14;
const READ_BUFFER_BYTES: usize = 8192;
const REQUEST_BYTES: usize = 16_384;
const CONNECT_SECONDS: u64 = 10;
const OPERATION_SECONDS: u64 = 30;
const SECURE_PORT: u16 = 443;
const PLAIN_PORT: u16 = 80;

#[path = "socket_poll.rs"]
mod polling;
use polling::Pending;

/// Explicit bounds and trust anchors. No ambient credentials or roots are loaded.
#[derive(Clone, Debug)]
pub struct SocketConfig {
    /// Total DNS, TCP, TLS and HTTP upgrade timeout.
    pub connect_timeout: Duration,
    /// Deadline for one send, receive or graceful close.
    pub operation_timeout: Duration,
    /// Maximum incoming frame and outgoing text size.
    pub max_frame_bytes: usize,
    /// Maximum incoming reassembled message size.
    pub max_message_bytes: usize,
    /// Maximum encoded bytes retained for writing, including frame headers.
    pub max_write_buffer_bytes: usize,
    /// DER-encoded CA certificates. Secure endpoints require at least one.
    pub root_certificates: Vec<Vec<u8>>,
}

impl Default for SocketConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(CONNECT_SECONDS),
            operation_timeout: Duration::from_secs(OPERATION_SECONDS),
            max_frame_bytes: DEFAULT_MESSAGE_BYTES,
            max_message_bytes: DEFAULT_MESSAGE_BYTES,
            max_write_buffer_bytes: DEFAULT_MESSAGE_BYTES + FRAME_HEADER_BYTES,
            root_certificates: Vec::new(),
        }
    }
}

/// A redacted failure retaining typed host, TLS and protocol causes.
#[derive(Debug)]
pub enum SocketError {
    /// Invalid endpoint, bounds, timeout or trust material.
    Configuration(&'static str),
    /// Host denied or failed DNS/TCP work, including absent capabilities.
    Host(ErrorCode),
    /// DNS returned no address.
    NoAddress,
    /// The finite operation deadline expired.
    Deadline,
    /// The shared readiness registry has no admission capacity.
    ReadinessLimit,
    /// Normal certificate, hostname or TLS validation failed.
    Tls(rustls::Error),
    /// A codec or transport operation failed. Peer bodies are not retained.
    Protocol,
    /// I/O failed outside a typed WASI socket operation.
    Io,
    /// An incoming/outgoing frame, message or write buffer exceeded its bound.
    Limit,
    /// A binary message arrived on this text-only socket.
    Binary,
    /// TCP/TLS ended without the required WebSocket close handshake.
    AbruptClose,
    /// The owner already ended or an operation was cancelled.
    Closed,
}

impl std::fmt::Display for SocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WASI WebSocket failure: {self:?}")
    }
}
impl std::error::Error for SocketError {}

/// One local socket. The caller drives the supplied Reactor; no task is spawned.
///
/// Dropping this owner unregisters readiness and closes its actual resources.
/// Poll methods borrow transiently, allowing sends during a pending receive.
/// Cancelling receive retains the bounded parser. Cancelling send is uncertain:
/// call shutdown, never automatically retry. No reconnect or unsubscribe is implied.
pub struct WasiWebSocket {
    socket: Option<WebSocket<Stream>>,
    reactor: Reactor,
    config: SocketConfig,
    reading: Pending,
    writing: Pending,
    closing: bool,
}

impl Drop for WasiWebSocket {
    fn drop(&mut self) {
        // WASI child pollables must close before their parent socket resources.
        self.shutdown();
    }
}

impl WasiWebSocket {
    /// Opens a strict ws/wss endpoint with explicit trust and host grants.
    ///
    /// # Errors
    /// Rejects invalid configuration before DNS. Returns host, TLS, upgrade,
    /// protocol or deadline errors without redirects or silent fallbacks.
    pub async fn connect(
        reactor: Reactor,
        endpoint: &str,
        config: SocketConfig,
    ) -> Result<Self, SocketError> {
        let socket = open(&reactor, endpoint, &config).await?;

        Ok(Self {
            socket: Some(socket),
            reactor,
            config,
            reading: Pending::default(),
            writing: Pending::default(),
            closing: false,
        })
    }

    /// Sends exactly one bounded text message and flushes its encoded bytes.
    ///
    /// # Errors
    /// Bounds reject before copying. A cancelled send is uncertain; shut down its owner.
    pub async fn send(&mut self, text: &str) -> Result<(), SocketError> {
        self.start_send(text)?;
        std::future::poll_fn(|context| self.poll_flush(context)).await
    }

    /// Receives one complete UTF8 text; processes bounded ping/pong/close frames.
    ///
    /// # Errors
    /// Rejects binary, malformed/oversized frames, abrupt EOF and deadline expiry.
    /// Cancelling receive retains this owner, its bounded parser and original deadline.
    pub async fn receive(&mut self) -> Result<Option<String>, SocketError> {
        std::future::poll_fn(|context| self.poll_receive(context)).await
    }

    /// Receives one complete bounded text or provider binary message.
    ///
    /// # Errors
    /// Preserves typed host, codec, payload-limit and finite deadline failures.
    /// Cancelling this wrapper retains the original parser and readiness slots.
    pub async fn receive_frame(
        &mut self,
    ) -> Result<Option<solana_kit_client::subscriptions::ReceivedFrame>, SocketError> {
        std::future::poll_fn(|context| self.poll_receive_frame(context)).await
    }

    /// Consumes the owner and awaits the peer close under one deadline.
    ///
    /// # Errors
    /// Reports malformed peer data, I/O or missing acknowledgement by the deadline.
    pub async fn close(mut self) -> Result<(), SocketError> {
        std::future::poll_fn(|context| self.poll_close(context)).await
    }
}

impl solana_kit_client::subscriptions::TextSocket for WasiWebSocket {
    type Error = SocketError;

    fn start_send(&mut self, text: &str) -> Result<(), Self::Error> {
        Self::start_send(self, text)
    }

    fn poll_flush(
        &mut self,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Self::poll_flush(self, context)
    }

    fn poll_receive(
        &mut self,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<Option<solana_kit_client::subscriptions::ReceivedFrame>, Self::Error>>
    {
        self.poll_receive_frame(context)
    }

    fn poll_close(
        &mut self,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Self::poll_close(self, context)
    }
}

async fn open(
    reactor: &Reactor,
    endpoint: &str,
    config: &SocketConfig,
) -> Result<WebSocket<Stream>, SocketError> {
    let parsed = parse(endpoint)?;
    let tls = configuration(config, parsed.secure)?;
    let deadline = deadline(config.connect_timeout)?;
    let request = endpoint
        .into_client_request()
        .map_err(|_| SocketError::Protocol)?;
    let stream = connect_stream(reactor, request.uri(), tls, deadline).await?;
    let handshake =
        ClientHandshake::start(stream, request, Some(protocol(config))).map_err(protocol_error)?;
    complete_handshake(handshake, reactor, deadline).await
}

async fn connect_stream(
    reactor: &Reactor,
    uri: &tungstenite::http::Uri,
    tls: Option<Arc<ClientConfig>>,
    deadline: u64,
) -> Result<Stream, SocketError> {
    let host = uri
        .host()
        .ok_or(SocketError::Configuration("missing host"))?;
    let host = host.trim_matches(['[', ']']);
    let port = uri.port_u16().unwrap_or(if tls.is_some() {
        SECURE_PORT
    } else {
        PLAIN_PORT
    });

    let tcp = Tcp::connect(host, port, reactor, deadline).await?;

    match tls {
        Some(config) => tls_connect(tcp, config, host, reactor, deadline).await,
        None => Ok(Stream::Plain(tcp)),
    }
}

async fn complete_handshake(
    mut handshake: tungstenite::handshake::MidHandshake<ClientHandshake<Stream>>,
    reactor: &Reactor,
    deadline: u64,
) -> Result<WebSocket<Stream>, SocketError> {
    loop {
        match handshake.handshake() {
            Ok((socket, _)) => return Ok(socket),
            Err(HandshakeError::Interrupted(pending)) => {
                wait(reactor, pending.get_ref().get_ref().signal(), deadline).await?;
                handshake = pending;
            }
            Err(HandshakeError::Failure(error)) => return Err(protocol_error(error)),
        }
    }
}

fn protocol(config: &SocketConfig) -> WebSocketConfig {
    WebSocketConfig::default()
        .read_buffer_size(config.max_frame_bytes.min(READ_BUFFER_BYTES))
        .write_buffer_size(0)
        .max_write_buffer_size(config.max_write_buffer_bytes)
        .max_frame_size(Some(config.max_frame_bytes))
        .max_message_size(Some(config.max_message_bytes))
}

fn parse(endpoint: &str) -> Result<Endpoint, SocketError> {
    if endpoint.len() > REQUEST_BYTES {
        return Err(SocketError::Limit);
    }

    let http = if let Some(rest) = endpoint.strip_prefix("wss://") {
        format!("https://{rest}")
    } else if let Some(rest) = endpoint.strip_prefix("ws://") {
        format!("http://{rest}")
    } else {
        return Err(SocketError::Configuration(
            "explicit ws or wss scheme required",
        ));
    };

    Endpoint::parse(&http).map_err(SocketError::Configuration)
}

/// Validate caller bounds before constructing TLS state or starting any host work.
fn configuration(
    config: &SocketConfig,
    secure: bool,
) -> Result<Option<Arc<ClientConfig>>, SocketError> {
    deadline(config.connect_timeout)?;
    deadline(config.operation_timeout)?;

    if config.max_frame_bytes == 0
        || config.max_message_bytes == 0
        || config
            .max_frame_bytes
            .checked_add(FRAME_HEADER_BYTES)
            .is_none_or(|size| size > config.max_write_buffer_bytes)
    {
        return Err(SocketError::Configuration("invalid socket buffer bounds"));
    }

    if !secure {
        return Ok(None);
    }

    if config.root_certificates.is_empty() {
        return Err(SocketError::Configuration(
            "wss requires explicit trust anchors",
        ));
    }

    let mut roots = RootCertStore::empty();

    for certificate in &config.root_certificates {
        roots
            .add(CertificateDer::from(certificate.clone()))
            .map_err(SocketError::Tls)?;
    }

    let config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    Ok(Some(Arc::new(config)))
}

fn deadline(duration: Duration) -> Result<u64, SocketError> {
    let nanos = u64::try_from(duration.as_nanos())
        .ok()
        .filter(|nanos| *nanos != 0)
        .ok_or(SocketError::Configuration("invalid timeout"))?;
    monotonic_clock::now()
        .checked_add(nanos)
        .ok_or(SocketError::Deadline)
}

async fn tls_connect(
    tcp: Tcp,
    config: Arc<ClientConfig>,
    host: &str,
    reactor: &Reactor,
    deadline: u64,
) -> Result<Stream, SocketError> {
    let name = ServerName::try_from(host.to_owned())
        .map_err(|_| SocketError::Configuration("invalid TLS name"))?;
    let connection = ClientConnection::new(config, name).map_err(SocketError::Tls)?;
    let mut stream = Box::new(rustls::StreamOwned::new(connection, tcp));

    while stream.conn.is_handshaking() {
        match stream.conn.complete_io(&mut stream.sock) {
            Ok(_) => {}

            Err(error) if error.kind() == ErrorKind::WouldBlock => {
                wait(reactor, stream.sock.signal(), deadline).await?
            }
            Err(error) => return Err(io_error(error)),
        }
    }

    Ok(Stream::Tls(stream))
}

fn would_block(error: Error) -> Result<(), SocketError> {
    match error {
        Error::Io(error) if error.kind() == ErrorKind::WouldBlock => Ok(()),
        error => Err(protocol_error(error)),
    }
}

fn io_error(error: std::io::Error) -> SocketError {
    if error.kind() == ErrorKind::UnexpectedEof {
        return SocketError::AbruptClose;
    }

    if let Some(tls) = error
        .get_ref()
        .and_then(|source| source.downcast_ref::<rustls::Error>())
    {
        return SocketError::Tls(tls.clone());
    }

    SocketError::Io
}

fn protocol_error(error: Error) -> SocketError {
    match error {
        Error::Capacity(_) | Error::WriteBufferFull(_) | Error::AttackAttempt => SocketError::Limit,
        Error::Io(error) => io_error(error),
        Error::Protocol(tungstenite::error::ProtocolError::ResetWithoutClosingHandshake) => {
            SocketError::AbruptClose
        }
        Error::ConnectionClosed | Error::AlreadyClosed => SocketError::Closed,
        _ => SocketError::Protocol,
    }
}
