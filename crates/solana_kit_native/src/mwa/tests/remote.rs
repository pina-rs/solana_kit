//! Remote reflector staging, encoding, deadline and cancellation evidence.
use super::reflector::*;
use super::*;
use crate::mwa::{remote::decode_reflector_id, transport::Encoding};
use solana_kit_wallet::mwa::{self, ProtocolVersion, ReflectorId, SessionKey};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::oneshot,
    task::JoinHandle,
};
use tokio_tungstenite::tungstenite::Message;

const SERVER_DEADLINE: Duration = Duration::from_secs(3);
const QUIET_PERIOD: Duration = Duration::from_millis(100);
const PEER_DRAIN: Duration = Duration::from_millis(500);
const CONNECT_STALL: Duration = Duration::from_secs(1);
const STALL: Duration = Duration::from_millis(50);
const SMALL_FRAME: usize = 256;
const SMALL_READ_BUFFER: usize = 64;
const SMALL_WRITE_BUFFER: usize = 270;
const BASE64_FRAME: usize = 150;
const BASE64_WRITE_BUFFER: usize = 164;
const MAX_HEADER_BYTES: usize = 16_384;
const EXPECTED_HELLO_BYTES: usize = 129;
const MAX_SAFE_ID: &[u8] = &[0x07, 0x1f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
const UNSAFE_ID: &[u8] = &[0x07, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const WIDE_ID: &[u8] = &[0x09, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
const MALFORMED_IDS: [&[u8]; 6] = [
    &[],
    &[0x80],
    &[0x00],
    &[0x05, 0x01, 0x02],
    &[0xff; 10],
    WIDE_ID,
];

fn protocol(encoding: Encoding) -> Option<&'static str> {
    match encoding {
        Encoding::Binary => Some(mwa::WEBSOCKET_BINARY),
        Encoding::Base64 => Some(mwa::WEBSOCKET_BASE64),
    }
}

async fn connect(
    authority: String,
    config: MwaConfig,
) -> Result<PendingRemoteAssociation, MwaError> {
    RemoteReflector::new(authority)
        .unwrap()
        .connect(NativeEntropy::default(), config)
        .await
}

async fn finish(server: JoinHandle<()>) {
    tokio::time::timeout(SERVER_DEADLINE, server)
        .await
        .unwrap()
        .unwrap();
}

fn remote_uri(uri: &str) -> (mwa::PublicPoint, String, ReflectorId) {
    match mwa::parse_association_uri(uri).unwrap() {
        mwa::Association::Remote {
            key,
            protocol,
            host,
            id,
        } => {
            assert_eq!(protocol, "v1");

            (key, host, id)
        }
        mwa::Association::Local { .. } => panic!("expected a remote association URI"),
    }
}

async fn exchange_hello(
    peer: &mut Peer,
    encoding: Encoding,
    association: mwa::PublicPoint,
) -> SessionKey {
    let hello = receive(peer, encoding).await;
    let public = mwa::verify_hello_request(&hello, &association).unwrap();
    let mut scalar = [0; 32];
    scalar[31] = 3;
    let agreement = mwa::AgreementKey::from_scalar(&scalar).unwrap();
    let key = agreement.session_key(&public, &association);
    let mut response = agreement.public_key().to_bytes().to_vec();
    response.extend(mwa::encrypt_message("{\"v\":1}", 1, &[0; 12], &key).unwrap());
    send(peer, encoding, response).await;

    key
}

async fn serve_calls(peer: &mut Peer, encoding: Encoding, key: SessionKey) {
    let mut session = mwa::Session::new(key, 2, 1);
    let request = receive(peer, encoding).await;
    let request: Value = serde_json::from_str(&session.decrypt(&request).unwrap()).unwrap();
    assert_eq!(request["method"], "get_capabilities");
    assert_eq!(request["id"], 1);
    send_empty(peer, encoding).await;
    let reply = serde_json::json!({
        "jsonrpc": "2.0",
        "id": request["id"],
        "result": {
            "max_transactions_per_request": 1,
            "max_messages_per_request": 1,
            "supported_transaction_versions": ["legacy"],
            "features": ["solana:signTransactions"],
        },
    });
    send(
        peer,
        encoding,
        session.encrypt(&reply.to_string(), &[1; 12]).unwrap(),
    )
    .await;

    let request = receive(peer, encoding).await;
    let request: Value = serde_json::from_str(&session.decrypt(&request).unwrap()).unwrap();
    assert_eq!(request["method"], "sign_messages");
    assert_eq!(request["id"], 2);
    let rejection = serde_json::json!({
        "jsonrpc": "2.0",
        "id": request["id"],
        "error": {"code": -1, "message": "declined"},
    });
    send(
        peer,
        encoding,
        session.encrypt(&rejection.to_string(), &[2; 12]).unwrap(),
    )
    .await;

    match peer.next().await.unwrap().unwrap() {
        Message::Close(_) => {}
        message => panic!("expected close, received {message:?}"),
    }

    peer.flush().await.unwrap();
}

async fn connect_error(encoding: Encoding, message: Message) -> MwaError {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, protocol(encoding), false).await;
        send_message(&mut peer, message).await;
        wait_for_eof(&mut peer).await;
    });

    let error = match connect(authority, mwa_config()).await {
        Err(error) => error,
        Ok(_) => panic!("unexpected reflector association"),
    };

    finish(server).await;

    error
}

async fn selected_reflector_session(encoding: Encoding) {
    let (listener, authority) = reflector().await;
    let (key_tx, key_rx) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, protocol(encoding), false).await;
        send_id(&mut peer, encoding).await;
        let association = key_rx.await.unwrap();

        // The association URI exists and the pending owner is live, but no HELLO
        // may precede the reflector's empty wallet-join frame.
        quiet(&mut peer, QUIET_PERIOD).await;
        send_empty(&mut peer, encoding).await;
        let key = exchange_hello(&mut peer, encoding, association).await;
        serve_calls(&mut peer, encoding, key).await;
    });

    let pending = connect(authority.clone(), mwa_config()).await.unwrap();
    let (key, host, id) = remote_uri(pending.uri());
    assert_eq!(host, authority);
    assert_eq!(id.value(), 0x01_02_03);
    assert_eq!(pending.id(), id);
    let expected = format!(
        "solana-wallet:/v1/associate/remote?association={}&reflector={}&id=AQID&v=v1",
        key.association_token(),
        authority.replace(':', "%3A"),
    );
    assert_eq!(pending.uri(), expected);
    assert_eq!(format!("{pending:?}"), "PendingRemoteAssociation { .. }");
    key_tx.send(key).unwrap();

    let mut wallet = tokio::time::timeout(SERVER_DEADLINE, pending.join())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(wallet.properties().protocol_version, ProtocolVersion::V1);

    match wallet
        .call(Method::GetCapabilities, Map::new())
        .await
        .unwrap()
    {
        Response::Result(result) => {
            assert_eq!(result["max_transactions_per_request"], 1);
        }
        Response::Error(_) => panic!("expected a capability result"),
    }

    match wallet.call(Method::SignMessages, Map::new()).await.unwrap() {
        Response::Error(error) => assert_eq!(error.code, -1),
        Response::Result(_) => panic!("expected a wallet rejection"),
    }

    wallet.close().await.unwrap();
    finish(server).await;
}

fn small_config() -> MwaConfig {
    let mut config = mwa_config();
    config.socket.max_frame_bytes = SMALL_FRAME;
    config.socket.read_buffer_bytes = SMALL_READ_BUFFER;
    config.socket.max_write_buffer_bytes = SMALL_WRITE_BUFFER;

    config
}

fn base64_budget_config() -> MwaConfig {
    let mut config = mwa_config();
    config.socket.max_frame_bytes = BASE64_FRAME;
    config.socket.read_buffer_bytes = SMALL_READ_BUFFER;
    config.socket.max_write_buffer_bytes = BASE64_WRITE_BUFFER;

    config
}

async fn negotiation_failure(selected: Option<&'static str>, extension: bool) {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let stream = acceptor().accept(stream).await.unwrap();
        let result = tokio_tungstenite::accept_hdr_async(
            stream,
            Select {
                protocol: selected,
                extension,
            },
        )
        .await;

        if let Ok(mut peer) = result {
            let _ = tokio::time::timeout(PEER_DRAIN, peer.next()).await;
        }
    });
    assert!(connect(authority, mwa_config()).await.is_err());

    finish(server).await;
}

async fn duplicate_protocol_failure() {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let stream = acceptor().accept(stream).await.unwrap();
        let result = tokio_tungstenite::accept_hdr_async(stream, DuplicateProtocols).await;

        if let Ok(mut peer) = result {
            let _ = tokio::time::timeout(PEER_DRAIN, peer.next()).await;
        }
    });
    assert!(connect(authority, mwa_config()).await.is_err());

    finish(server).await;
}

#[tokio::test]
async fn remote_binary_reflector_stages_id_uri_join_and_encrypted_calls() {
    selected_reflector_session(Encoding::Binary).await;
}

#[tokio::test]
async fn remote_base64_reflector_stages_id_uri_join_and_encrypted_calls() {
    selected_reflector_session(Encoding::Base64).await;
}

#[test]
fn reflector_authorities_require_an_exact_bare_tls_authority() {
    for authority in [
        "reflector.example",
        "reflector.example:443",
        "127.0.0.1",
        "[::1]:443",
    ] {
        let reflector = RemoteReflector::new(authority).unwrap();
        assert_eq!(reflector.authority(), authority);
        assert_eq!(format!("{reflector:?}"), "RemoteReflector { .. }");
    }

    for authority in [
        "",
        "user@reflector.example",
        "reflector.example/reflect",
        "reflector.example?query=1",
        "reflector.example#fragment",
        "ws://reflector.example",
        "wss://reflector.example",
        "reflector.example:0",
        "reflector.example:0443",
        "reflector.example:65536",
        "local host",
        "[0:0:0:0:0:0:0:1]:443",
    ] {
        assert!(
            RemoteReflector::new(authority).is_err(),
            "accepted {authority}"
        );
    }
}

#[tokio::test]
async fn remote_configuration_rejects_headers_and_nonpositive_join_deadlines_before_io() {
    let (listener, authority) = reflector().await;
    let mut config = mwa_config();
    config
        .socket
        .headers
        .insert("authorization", "Bearer personal".parse().unwrap());
    assert!(matches!(
        connect(authority, config).await,
        Err(MwaError::Handshake)
    ));
    drop(listener);

    let (listener, authority) = reflector().await;
    let mut config = mwa_config();
    config.join_timeout = Duration::ZERO;
    assert!(matches!(
        connect(authority, config).await,
        Err(MwaError::Config(_))
    ));
    drop(listener);

    let (listener, authority) = reflector().await;
    let mut config = mwa_config();
    config.max_requests = 0;
    assert!(matches!(
        connect(authority, config).await,
        Err(MwaError::Capacity)
    ));
    drop(listener);
}

#[tokio::test]
async fn remote_tls_requires_selected_roots_and_exact_destination_name() {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let _ = acceptor().accept(stream).await;
    });
    let mut config = mwa_config();
    config.socket.tls = wrong_roots();
    assert!(matches!(
        connect(authority, config).await,
        Err(MwaError::Transport(crate::WsError::Tls))
    ));
    finish(server).await;

    // This case deliberately connects by IPv4, which is absent from the SAN.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let _ = acceptor().accept(stream).await;
    });
    assert!(matches!(
        connect(format!("127.0.0.1:{port}"), mwa_config()).await,
        Err(MwaError::Transport(crate::WsError::Tls))
    ));
    finish(server).await;
}

/// Invalid policy must fail at either public entry point before a TCP accept.
async fn rejects_deadline_before_connection(config: MwaConfig) {
    let (listener, authority) = reflector().await;
    assert!(matches!(
        connect(authority, config.clone()).await,
        Err(MwaError::Config(_))
    ));
    assert!(
        tokio::time::timeout(Duration::from_millis(1), listener.accept())
            .await
            .is_err()
    );

    let (proposal, listener) = association().await;
    assert!(matches!(
        proposal.connect(config).await,
        Err(MwaError::Config(_))
    ));
    assert!(
        tokio::time::timeout(Duration::from_millis(1), listener.accept())
            .await
            .is_err()
    );
}

#[tokio::test]
async fn local_and_remote_deadlines_reject_zero_and_overflow_before_io() {
    for duration in [Duration::ZERO, Duration::MAX] {
        let mut configs = [mwa_config(), mwa_config(), mwa_config(), mwa_config()];
        configs[0].approval_timeout = duration;
        configs[1].join_timeout = duration;
        configs[2].socket.connect_timeout = duration;
        configs[3].socket.operation_timeout = duration;

        for config in configs {
            rejects_deadline_before_connection(config).await;
        }
    }
}

#[tokio::test]
async fn remote_reflector_rejects_http_redirects() {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut stream = acceptor().accept(stream).await.unwrap();
        let mut request = Vec::new();

        while !request.ends_with(b"\r\n\r\n") {
            let mut byte = [0];
            stream.read_exact(&mut byte).await.unwrap();
            request.push(byte[0]);
            assert!(request.len() < MAX_HEADER_BYTES);
        }

        stream
            .write_all(
                b"HTTP/1.1 302 Found\r\nLocation: wss://evil.example/reflect\r\nContent-Length: 0\r\n\r\n",
            )
            .await
            .unwrap();
    });
    assert!(matches!(
        connect(authority, mwa_config()).await,
        Err(MwaError::Transport(_))
    ));
    finish(server).await;
}

#[tokio::test]
async fn remote_reflector_requires_one_offered_subprotocol_and_no_extensions() {
    // Each case proves the reflector cannot omit selection, choose an unknown
    // protocol, provide two lines, or add an unrequested extension.
    for (selected, extension) in [
        (None, false),
        (Some("com.solana.mobilewalletadapter.v2"), false),
        (
            Some("com.solana.mobilewalletadapter.v1, com.solana.mobilewalletadapter.v1.base64"),
            false,
        ),
        (Some(mwa::WEBSOCKET_BINARY), true),
    ] {
        negotiation_failure(selected, extension).await;
    }

    duplicate_protocol_failure().await;
}

#[tokio::test]
async fn remote_reflector_rejects_malformed_truncated_wide_and_unsafe_ids() {
    for payload in MALFORMED_IDS {
        let error = connect_error(Encoding::Binary, wire(Encoding::Binary, payload.into())).await;
        assert!(
            matches!(error, MwaError::Reflector),
            "accepted ID payload {payload:?}"
        );
    }

    for payload in [
        UNSAFE_ID,
        &[0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    ] {
        let error = connect_error(Encoding::Binary, wire(Encoding::Binary, payload.into())).await;
        assert!(matches!(error, MwaError::Reflector));
    }

    let error = connect_error(Encoding::Base64, wire(Encoding::Base64, UNSAFE_ID.into())).await;
    assert!(matches!(error, MwaError::Reflector));
}

#[test]
fn reflector_id_decoder_preserves_source_framing_and_javascript_safe_bounds() {
    assert_eq!(
        decode_reflector_id(&[0x03, 0x01, 0x02, 0x03])
            .unwrap()
            .value(),
        0x01_02_03
    );
    assert_eq!(
        decode_reflector_id(MAX_SAFE_ID).unwrap().value(),
        9_007_199_254_740_991
    );
    assert!(decode_reflector_id(&[0x01, 0x00]).is_ok());
    assert!(decode_reflector_id(WIDE_ID).is_err());
}

#[tokio::test]
async fn remote_reflector_rejects_wrong_frame_types_and_invalid_base64() {
    let error = connect_error(Encoding::Binary, Message::Text("AwECAw==".into())).await;
    assert!(matches!(error, MwaError::Handshake));

    let error = connect_error(
        Encoding::Base64,
        Message::Binary(vec![0x03, 0x01, 0x02, 0x03].into()),
    )
    .await;
    assert!(matches!(error, MwaError::Handshake));

    let error = connect_error(Encoding::Base64, Message::Text("not base64!!".into())).await;
    assert!(matches!(error, MwaError::Handshake));
}

#[tokio::test]
async fn remote_reflector_rejects_an_oversized_id_frame() {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, Some(mwa::WEBSOCKET_BINARY), false).await;
        send_message(&mut peer, Message::Binary(vec![0xff; 300].into())).await;
        wait_for_eof(&mut peer).await;
    });

    let error = match connect(authority, small_config()).await {
        Err(error) => error,
        Ok(_) => panic!("accepted an oversized reflector frame"),
    };

    assert!(matches!(error, MwaError::Socket(_) | MwaError::Capacity));
    finish(server).await;
}

#[tokio::test]
async fn remote_base64_hello_must_fit_the_encoded_wire_budget() {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, Some(mwa::WEBSOCKET_BASE64), false).await;
        send_id(&mut peer, Encoding::Base64).await;
        send_empty(&mut peer, Encoding::Base64).await;
        wait_for_eof(&mut peer).await;
    });
    let pending = connect(authority, base64_budget_config()).await.unwrap();
    assert!(matches!(pending.join().await, Err(MwaError::Capacity)));
    finish(server).await;
}

#[tokio::test]
async fn remote_join_must_be_empty() {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, Some(mwa::WEBSOCKET_BINARY), false).await;
        send_id(&mut peer, Encoding::Binary).await;
        send(&mut peer, Encoding::Binary, vec![0x01]).await;
        wait_for_eof(&mut peer).await;
    });
    let pending = connect(authority, mwa_config()).await.unwrap();
    assert!(matches!(pending.join().await, Err(MwaError::Reflector)));
    finish(server).await;
}

#[tokio::test]
async fn remote_empty_hello_response_is_invalid() {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, Some(mwa::WEBSOCKET_BINARY), false).await;
        send_id(&mut peer, Encoding::Binary).await;
        send_empty(&mut peer, Encoding::Binary).await;
        assert_eq!(
            receive(&mut peer, Encoding::Binary).await.len(),
            EXPECTED_HELLO_BYTES
        );
        send_empty(&mut peer, Encoding::Binary).await;
        wait_for_eof(&mut peer).await;
    });
    let pending = connect(authority, mwa_config()).await.unwrap();
    assert!(matches!(pending.join().await, Err(MwaError::Reflector)));
    finish(server).await;
}

#[tokio::test]
async fn remote_connect_deadline_drops_a_stalled_reflector() {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, Some(mwa::WEBSOCKET_BINARY), false).await;
        wait_for_eof(&mut peer).await;
    });
    let mut config = mwa_config();
    config.socket.connect_timeout = CONNECT_STALL;
    assert!(matches!(
        connect(authority, config).await,
        Err(MwaError::Deadline(_))
    ));
    finish(server).await;
}

#[tokio::test]
async fn remote_join_deadline_drops_a_stalled_wallet_join() {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, Some(mwa::WEBSOCKET_BINARY), false).await;
        send_id(&mut peer, Encoding::Binary).await;
        wait_for_eof(&mut peer).await;
    });
    let mut config = mwa_config();
    config.join_timeout = STALL;
    let pending = connect(authority, config).await.unwrap();
    assert!(matches!(pending.join().await, Err(MwaError::Deadline(_))));
    finish(server).await;
}

#[tokio::test]
async fn remote_hello_deadline_drops_a_stalled_wallet_hello() {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, Some(mwa::WEBSOCKET_BINARY), false).await;
        send_id(&mut peer, Encoding::Binary).await;
        send_empty(&mut peer, Encoding::Binary).await;
        assert_eq!(
            receive(&mut peer, Encoding::Binary).await.len(),
            EXPECTED_HELLO_BYTES
        );
        wait_for_eof(&mut peer).await;
    });
    let mut config = mwa_config();
    config.socket.operation_timeout = STALL;
    let pending = connect(authority, config).await.unwrap();
    assert!(matches!(pending.join().await, Err(MwaError::Deadline(_))));
    finish(server).await;
}

#[tokio::test]
async fn dropping_a_pending_remote_owner_closes_the_actual_reflector_peer() {
    let (listener, authority) = reflector().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, Some(mwa::WEBSOCKET_BINARY), false).await;
        send_id(&mut peer, Encoding::Binary).await;
        wait_for_eof(&mut peer).await;
    });
    let pending = connect(authority, mwa_config()).await.unwrap();
    assert!(
        pending
            .uri()
            .starts_with("solana-wallet:/v1/associate/remote?")
    );
    drop(pending);
    finish(server).await;
}

#[tokio::test]
async fn dropping_a_pending_join_future_closes_the_actual_reflector_peer() {
    let (listener, authority) = reflector().await;
    let (hello_tx, hello_rx) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, Some(mwa::WEBSOCKET_BINARY), false).await;
        send_id(&mut peer, Encoding::Binary).await;
        send_empty(&mut peer, Encoding::Binary).await;
        assert_eq!(
            receive(&mut peer, Encoding::Binary).await.len(),
            EXPECTED_HELLO_BYTES
        );
        hello_tx.send(()).unwrap();
        wait_for_eof(&mut peer).await;
    });
    let pending = connect(authority, mwa_config()).await.unwrap();

    {
        let join = pending.join();
        tokio::pin!(join);
        tokio::select! {
            result = &mut join => panic!("join unexpectedly completed: {}", result.is_ok()),
            result = hello_rx => result.unwrap(),
        }
    }

    finish(server).await;
}

#[tokio::test]
async fn dropping_a_borrowed_remote_call_closes_the_session_and_actual_peer() {
    let (listener, authority) = reflector().await;
    let (key_tx, key_rx) = oneshot::channel();
    let (request_tx, request_rx) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut peer = accept(listener, Some(mwa::WEBSOCKET_BINARY), false).await;
        send_id(&mut peer, Encoding::Binary).await;
        let association = key_rx.await.unwrap();
        send_empty(&mut peer, Encoding::Binary).await;
        let key = exchange_hello(&mut peer, Encoding::Binary, association).await;
        let mut session = mwa::Session::new(key, 2, 1);
        let request = receive(&mut peer, Encoding::Binary).await;
        let request: Value = serde_json::from_str(&session.decrypt(&request).unwrap()).unwrap();
        assert_eq!(request["method"], "sign_messages");
        request_tx.send(()).unwrap();
        wait_for_eof(&mut peer).await;
    });
    let pending = connect(authority, mwa_config()).await.unwrap();
    let (key, _, _) = remote_uri(pending.uri());
    key_tx.send(key).unwrap();
    let mut wallet = pending.join().await.unwrap();

    {
        let call = wallet.call(Method::SignMessages, Map::new());
        tokio::pin!(call);
        tokio::select! {
            result = &mut call => panic!("signing unexpectedly completed: {}", result.is_ok()),
            result = request_rx => result.unwrap(),
        }
    }

    assert!(matches!(
        wallet.call(Method::GetCapabilities, Map::new()).await,
        Err(MwaError::Closed)
    ));

    finish(server).await;
}
