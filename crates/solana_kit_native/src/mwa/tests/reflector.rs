//! Owned loopback TLS WebSocket reflector fixture for remote association tests.
use super::*;
use crate::{TlsRoots, WsConfig, mwa::transport::Encoding};
use base64::{Engine, engine::general_purpose::STANDARD};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    time::{Duration, timeout},
};
use tokio_rustls::{
    TlsAcceptor,
    rustls::{
        self, ServerConfig,
        pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
    },
};
use tokio_tungstenite::{
    WebSocketStream,
    tungstenite::{
        Message,
        handshake::server::{Callback, ErrorResponse, Request as Upgrade, Response as Accepted},
        http::HeaderValue,
    },
};

const ROOT: &[u8] = include_bytes!("../../../tests/fixtures/ca.der");
const CERT: &[u8] = include_bytes!("../../../tests/fixtures/server.der");
const KEY: &[u8] = include_bytes!("../../../tests/fixtures/server-key.der");
const WRONG_ROOT: &[u8] = include_bytes!("../../../tests/fixtures/wrong-ca.der");
const OFFER: &str = "com.solana.mobilewalletadapter.v1, com.solana.mobilewalletadapter.v1.base64";

/// Independent length-prefixed reflector ID fixture with a literal URI oracle.
pub(super) const ID_BYTES: [u8; 4] = [0x03, 0x01, 0x02, 0x03];

/// The TLS-protected server side of one reflector fixture.
pub(super) type Peer = WebSocketStream<tokio_rustls::server::TlsStream<TcpStream>>;

pub(super) fn acceptor() -> TlsAcceptor {
    let config =
        ServerConfig::builder_with_provider(rustls::crypto::ring::default_provider().into())
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(
                vec![CertificateDer::from(CERT.to_vec())],
                PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(KEY.to_vec())),
            )
            .unwrap();

    TlsAcceptor::from(Arc::new(config))
}

pub(super) fn roots() -> TlsRoots {
    TlsRoots::Certificates(vec![ROOT.to_vec()])
}

pub(super) fn wrong_roots() -> TlsRoots {
    TlsRoots::Certificates(vec![WRONG_ROOT.to_vec()])
}

pub(super) fn mwa_config() -> MwaConfig {
    MwaConfig {
        socket: WsConfig {
            tls: roots(),
            connect_timeout: Duration::from_secs(2),
            operation_timeout: Duration::from_secs(2),
            ..WsConfig::default()
        },
        join_timeout: Duration::from_secs(2),
        ..MwaConfig::default()
    }
}

/// Binds a loopback listener and returns the certificate's exact authority.
pub(super) async fn reflector() -> (TcpListener, String) {
    // Match the client's localhost address family; a refused IPv6 attempt can
    // consume the connection deadline on Windows before falling back to IPv4.
    let listener = TcpListener::bind("localhost:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    (listener, format!("localhost:{port}"))
}

/// Response selection for the fixture handshake.
pub(super) struct Select {
    pub(super) protocol: Option<&'static str>,
    pub(super) extension: bool,
}

impl Callback for Select {
    fn on_request(
        self,
        request: &Upgrade,
        mut response: Accepted,
    ) -> Result<Accepted, ErrorResponse> {
        assert_request(request);

        if let Some(protocol) = self.protocol {
            response
                .headers_mut()
                .insert("sec-websocket-protocol", HeaderValue::from_static(protocol));
        }

        if self.extension {
            response.headers_mut().insert(
                "sec-websocket-extensions",
                HeaderValue::from_static("permessage-deflate"),
            );
        }

        Ok(response)
    }
}

/// Responds with two separate protocol header lines even though both were offered.
pub(super) struct DuplicateProtocols;

impl Callback for DuplicateProtocols {
    fn on_request(
        self,
        request: &Upgrade,
        mut response: Accepted,
    ) -> Result<Accepted, ErrorResponse> {
        assert_request(request);
        response.headers_mut().append(
            "sec-websocket-protocol",
            HeaderValue::from_static(mwa::WEBSOCKET_BINARY),
        );
        response.headers_mut().append(
            "sec-websocket-protocol",
            HeaderValue::from_static(mwa::WEBSOCKET_BASE64),
        );

        Ok(response)
    }
}

fn assert_request(request: &Upgrade) {
    assert_eq!(request.uri().path(), "/reflect");
    assert!(request.uri().query().is_none());
    assert_eq!(request.headers()["sec-websocket-protocol"], OFFER);
    assert!(
        request.headers()["host"]
            .to_str()
            .unwrap()
            .starts_with("localhost:")
    );
    assert!(!request.headers().contains_key("authorization"));
    assert!(!request.headers().contains_key("cookie"));
}

pub(super) async fn accept(
    listener: TcpListener,
    protocol: Option<&'static str>,
    extension: bool,
) -> Peer {
    let (stream, _) = listener.accept().await.unwrap();
    let stream = acceptor().accept(stream).await.unwrap();

    tokio_tungstenite::accept_hdr_async(
        stream,
        Select {
            protocol,
            extension,
        },
    )
    .await
    .unwrap()
}

pub(super) fn wire(encoding: Encoding, bytes: Vec<u8>) -> Message {
    match encoding {
        Encoding::Binary => Message::Binary(bytes.into()),
        Encoding::Base64 => Message::Text(STANDARD.encode(bytes).into()),
    }
}

pub(super) async fn send(peer: &mut Peer, encoding: Encoding, bytes: Vec<u8>) {
    peer.send(wire(encoding, bytes)).await.unwrap();
}

pub(super) async fn send_id(peer: &mut Peer, encoding: Encoding) {
    send(peer, encoding, ID_BYTES.to_vec()).await;
}

pub(super) async fn send_empty(peer: &mut Peer, encoding: Encoding) {
    send(peer, encoding, Vec::new()).await;
}

pub(super) async fn send_message(peer: &mut Peer, message: Message) {
    peer.send(message).await.unwrap();
}

pub(super) async fn receive(peer: &mut Peer, encoding: Encoding) -> Vec<u8> {
    match (encoding, peer.next().await.unwrap().unwrap()) {
        (Encoding::Binary, Message::Binary(bytes)) => bytes.to_vec(),
        (Encoding::Base64, Message::Text(text)) => STANDARD.decode(text.as_bytes()).unwrap(),
        (encoding, message) => panic!("expected {encoding:?} frame, received {message:?}"),
    }
}

/// Asserts that no data frame arrives within the finite quiet window.
pub(super) async fn quiet(peer: &mut Peer, duration: Duration) {
    if let Ok(Some(Ok(message))) = timeout(duration, peer.next()).await {
        panic!("unexpected frame before join: {message:?}");
    }
}

/// Waits for a graceful close, abrupt EOF, or reset within the finite fixture bound.
pub(super) async fn wait_for_eof(peer: &mut Peer) {
    match timeout(Duration::from_secs(2), peer.next()).await {
        Ok(None) | Ok(Some(Ok(Message::Close(_)))) | Ok(Some(Err(_))) => {}
        Ok(Some(Ok(message))) => panic!("unexpected frame before close: {message:?}"),
        Err(_) => panic!("peer did not close within the fixture bound"),
    }
}
