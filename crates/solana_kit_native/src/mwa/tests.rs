use super::*;
use futures_util::{SinkExt, StreamExt};
use solana_kit_wallet::mwa::{ProtocolVersion, encrypt_message, verify_hello_request};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::{
    Message,
    handshake::server::{Callback, ErrorResponse, Request as Upgrade, Response as Accepted},
    http::HeaderValue,
};

const FIRST_PORT: u16 = 49152;
const LAST_PORT: u16 = 65535;
const STALL_DEADLINE_MS: u64 = 50;

async fn association() -> (NativeAssociation, TcpListener) {
    let mut proposal = NativeAssociation::generate(NativeEntropy::default())
        .await
        .unwrap();

    for port in FIRST_PORT..=LAST_PORT {
        if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)).await {
            proposal.port = AssociationPort::new(u32::from(port)).unwrap();

            return (proposal, listener);
        }
    }

    panic!("no local private port available");
}

type PeerSocket = tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>;

/// Checks the real peer's binary subprotocol through Tungstenite's callback contract.
struct BinaryProtocol;

impl Callback for BinaryProtocol {
    fn on_request(
        self,
        request: &Upgrade,
        mut response: Accepted,
    ) -> Result<Accepted, ErrorResponse> {
        assert_eq!(request.uri().path(), mwa::LOCAL_WEBSOCKET_PATH);
        assert_eq!(
            request.headers()["sec-websocket-protocol"],
            mwa::WEBSOCKET_BINARY
        );
        response.headers_mut().insert(
            "sec-websocket-protocol",
            HeaderValue::from_static(mwa::WEBSOCKET_BINARY),
        );
        Ok(response)
    }
}

async fn handshake(listener: TcpListener, association: mwa::PublicPoint) -> (PeerSocket, Session) {
    let (stream, _) = listener.accept().await.unwrap();
    let mut socket = tokio_tungstenite::accept_hdr_async(stream, BinaryProtocol)
        .await
        .unwrap();
    let Message::Binary(hello) = socket.next().await.unwrap().unwrap() else {
        panic!("expected HELLO");
    };
    let first = verify_hello_request(&hello, &association).unwrap();
    socket
        .send(Message::Binary(Vec::new().into()))
        .await
        .unwrap();
    let Message::Binary(hello) = socket.next().await.unwrap().unwrap() else {
        panic!("expected fresh HELLO after APP_PING");
    };
    let public = verify_hello_request(&hello, &association).unwrap();
    assert_ne!(first, public);

    let mut scalar = [0; 32];
    scalar[31] = 3;
    let agreement = AgreementKey::from_scalar(&scalar).unwrap();
    let key = agreement.session_key(&public, &association);
    let mut hello = agreement.public_key().to_bytes().to_vec();
    hello.extend(encrypt_message("{\"v\":1}", 0, &[0; 12], &key).unwrap());
    socket.send(Message::Binary(hello.into())).await.unwrap();
    (socket, Session::new(key, 1, 1))
}

async fn peer(listener: TcpListener, association: mwa::PublicPoint) -> PeerSocket {
    let (mut socket, mut session) = handshake(listener, association).await;
    let Message::Binary(request) = socket.next().await.unwrap().unwrap() else {
        panic!("expected request");
    };
    let request: Value = serde_json::from_str(&session.decrypt(&request).unwrap()).unwrap();
    assert_eq!(request["method"], "get_capabilities");
    let reply = serde_json::json!({
        "jsonrpc": "2.0", "id": request["id"],
        "error": {"code": -1, "message": "declined"},
    });
    socket
        .send(Message::Binary(Vec::new().into()))
        .await
        .unwrap();
    socket
        .send(Message::Binary(
            session
                .encrypt(&reply.to_string(), &[1; 12])
                .unwrap()
                .into(),
        ))
        .await
        .unwrap();
    socket
}

#[tokio::test]
async fn real_binary_peer_negotiates_properties_and_preserves_wallet_rejection() {
    let (proposal, listener) = association().await;
    let public = proposal.association.public_key();
    let server = tokio::spawn(async move {
        let mut socket = peer(listener, public).await;
        assert!(matches!(
            socket.next().await.unwrap().unwrap(),
            Message::Close(_)
        ));
        socket.flush().await.unwrap();
    });
    let mut wallet = proposal.connect(MwaConfig::default()).await.unwrap();
    assert_eq!(wallet.properties().protocol_version, ProtocolVersion::V1);
    assert!(matches!(
        wallet
            .call(Method::GetCapabilities, Map::new())
            .await
            .unwrap(),
        Response::Error(_)
    ));
    wallet.close().await.unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn connect_deadline_drops_an_actual_stalled_peer() {
    use tokio::io::AsyncReadExt;
    let (proposal, listener) = association().await;
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut bytes = [0; 1024];

        loop {
            if socket.read(&mut bytes).await.unwrap() == 0 {
                break;
            }
        }
    });
    let mut config = MwaConfig::default();
    config.socket.connect_timeout = Duration::from_millis(STALL_DEADLINE_MS);
    assert!(matches!(
        proposal.connect(config).await,
        Err(MwaError::Deadline(_))
    ));
    tokio::time::timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn dropping_a_pending_signing_call_closes_the_session_and_actual_peer() {
    let (proposal, listener) = association().await;
    let public = proposal.association.public_key();
    let (started, received) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        let mut socket = peer(listener, public).await;
        assert!(matches!(
            socket.next().await.unwrap().unwrap(),
            Message::Binary(_)
        ));
        started.send(()).unwrap();
        let closed = socket.next().await;
        assert!(closed.is_none() || matches!(closed, Some(Err(_)) | Some(Ok(Message::Close(_)))));
    });
    let mut wallet = proposal.connect(MwaConfig::default()).await.unwrap();
    assert!(matches!(
        wallet
            .call(Method::GetCapabilities, Map::new())
            .await
            .unwrap(),
        Response::Error(_)
    ));

    {
        let call = wallet.call(Method::SignMessages, Map::new());
        tokio::pin!(call);
        tokio::select! {
            result = &mut call => panic!("signing unexpectedly completed: {}", result.is_ok()),
            result = received => result.unwrap(),
        }
    }

    assert!(matches!(
        wallet.call(Method::GetCapabilities, Map::new()).await,
        Err(MwaError::Closed)
    ));
    tokio::time::timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
}
