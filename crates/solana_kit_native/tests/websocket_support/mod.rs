use futures_util::{SinkExt, StreamExt};
use solana_kit_native::{NativeWebSocket, TlsRoots, WsConfig, WsError};
use std::{sync::Arc, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task::JoinHandle,
    time::timeout,
};
use tokio_rustls::{
    TlsAcceptor,
    rustls::{
        self, ServerConfig,
        pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
    },
};
use tokio_tungstenite::tungstenite::Message;

pub async fn listener() -> TcpListener {
    TcpListener::bind("127.0.0.1:0").await.unwrap()
}
pub fn endpoint(listener: &TcpListener, secure: bool, host: &str) -> String {
    format!(
        "{}://{host}:{}/rpc",
        if secure { "wss" } else { "ws" },
        listener.local_addr().unwrap().port()
    )
}
pub async fn eof(stream: &mut (impl AsyncRead + Unpin)) -> usize {
    timeout(Duration::from_secs(2), stream.read(&mut [0]))
        .await
        .unwrap()
        .unwrap()
}
pub async fn read_headers(stream: &mut TcpStream) {
    let mut bytes = Vec::new();

    while !bytes.ends_with(b"\r\n\r\n") {
        let mut byte = [0];
        stream.read_exact(&mut byte).await.unwrap();
        bytes.push(byte[0]);
        assert!(bytes.len() < 16_384);
    }
}
pub fn acceptor() -> TlsAcceptor {
    let config =
        ServerConfig::builder_with_provider(rustls::crypto::ring::default_provider().into())
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(
                vec![CertificateDer::from(
                    include_bytes!("../fixtures/server.der").to_vec(),
                )],
                PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(
                    include_bytes!("../fixtures/server-key.der").to_vec(),
                )),
            )
            .unwrap();
    TlsAcceptor::from(Arc::new(config))
}
pub fn config() -> WsConfig {
    WsConfig {
        tls: TlsRoots::Certificates(vec![include_bytes!("../fixtures/ca.der").to_vec()]),
        ..WsConfig::default()
    }
}

pub async fn tls_case(host: &str, roots: bool, succeeds: bool) {
    let listener = listener().await;
    let endpoint = endpoint(&listener, true, host);
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let stream = acceptor().accept(stream).await;
        if !succeeds {
            assert!(stream.is_err());
            return;
        }

        let mut peer = tokio_tungstenite::accept_async(stream.unwrap())
            .await
            .unwrap();
        peer.send(Message::Text("trusted".into())).await.unwrap();
        assert!(matches!(
            peer.next().await.unwrap().unwrap(),
            Message::Close(_)
        ));
        peer.flush().await.unwrap();
    });
    let mut policy = config();

    if !roots {
        policy.tls =
            TlsRoots::Certificates(vec![include_bytes!("../fixtures/wrong-ca.der").to_vec()]);
    }

    let result = NativeWebSocket::connect(&endpoint, policy).await;

    if succeeds {
        let mut socket = result.unwrap();
        assert_eq!(socket.receive().await.unwrap().as_deref(), Some("trusted"));
        socket.close().await.unwrap();
    } else {
        assert_eq!(result.unwrap_err(), WsError::Tls);
    }

    server.await.unwrap();
}
pub async fn rejection(reply: &'static [u8]) {
    let listener = listener().await;
    let endpoint = endpoint(&listener, false, "127.0.0.1");
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        read_headers(&mut stream).await;
        stream.write_all(reply).await.unwrap();
    });
    assert!(matches!(
        NativeWebSocket::connect(&endpoint, WsConfig::default()).await,
        Err(WsError::Handshake | WsError::Protocol)
    ));
    server.await.unwrap();
}
pub async fn raw_frame(wire: Vec<u8>, expected: WsError) {
    let listener = listener().await;
    let endpoint = endpoint(&listener, false, "127.0.0.1");
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut peer = tokio_tungstenite::accept_async(stream).await.unwrap();
        peer.get_mut().write_all(&wire).await.unwrap();
        assert_eq!(eof(peer.get_mut()).await, 0);
    });
    let policy = WsConfig {
        max_frame_bytes: 64,
        max_message_bytes: 64,
        read_buffer_bytes: 16,
        max_write_buffer_bytes: 78,
        ..WsConfig::default()
    };
    let mut socket = NativeWebSocket::connect(&endpoint, policy).await.unwrap();
    assert_eq!(socket.receive().await.unwrap_err(), expected);
    assert_eq!(socket.receive().await.unwrap_err(), WsError::Closed);
    server.await.unwrap();
}
pub async fn quiet(config: WsConfig) -> (NativeWebSocket, JoinHandle<()>) {
    let listener = listener().await;
    let endpoint = endpoint(&listener, false, "127.0.0.1");
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut peer = tokio_tungstenite::accept_async(stream).await.unwrap();
        assert_eq!(eof(peer.get_mut()).await, 0);
    });
    (
        NativeWebSocket::connect(&endpoint, config).await.unwrap(),
        server,
    )
}

pub struct Authentication;
impl tokio_tungstenite::tungstenite::handshake::server::Callback for Authentication {
    fn on_request(
        self,
        request: &tokio_tungstenite::tungstenite::handshake::server::Request,
        response: tokio_tungstenite::tungstenite::handshake::server::Response,
    ) -> Result<
        tokio_tungstenite::tungstenite::handshake::server::Response,
        tokio_tungstenite::tungstenite::handshake::server::ErrorResponse,
    > {
        assert_eq!(request.uri().path(), "/rpc");
        assert_eq!(request.headers()["authorization"], "Bearer fixture-token");
        assert!(!request.headers().contains_key("cookie"));
        Ok(response)
    }
}
