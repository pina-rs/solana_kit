//! Real loopback protocol, TLS and caller-owned lifecycle checks.
mod websocket_support;
use futures_util::{SinkExt, StreamExt};
use solana_kit_native::{NativeWebSocket, WsConfig, WsError};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Wake, Waker},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::timeout,
};
use tokio_tungstenite::tungstenite::Message;
use websocket_support::*;

const DEADLINE_MS: u64 = 50;
const QUIET_MS: u64 = 15;

#[tokio::test]
async fn endpoint_and_config_failures_are_redacted_before_io() {
    for value in [
        "http://localhost/",
        "ws://name:secret@localhost/",
        "ws://localhost/#secret",
        "ws://localhost:0/",
        "ws://localhost:65536/",
        "ws://localhost/ bad",
        "ws:///path",
    ] {
        let error = NativeWebSocket::connect(value, WsConfig::default())
            .await
            .unwrap_err();
        assert!(matches!(error, WsError::Config(_)));
        assert!(!format!("{error:?}").contains("secret"));
    }
    let mut config = WsConfig::default();
    config
        .headers
        .insert("sec-websocket-key", "secret".parse().unwrap());
    assert!(matches!(
        NativeWebSocket::connect("ws://localhost/", config).await,
        Err(WsError::Config(_))
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn actual_multithreaded_send_ping_pong_and_graceful_close() {
    let listener = websocket_support::listener().await;
    let endpoint = websocket_support::endpoint(&listener, false, "127.0.0.1");
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
        assert_eq!(
            socket.next().await.unwrap().unwrap(),
            Message::Text("request".into())
        );
        socket.send(Message::Ping(vec![3, 7].into())).await.unwrap();
        socket.send(Message::Text("answer".into())).await.unwrap();
        assert_eq!(
            socket.next().await.unwrap().unwrap(),
            Message::Pong(vec![3, 7].into())
        );
        assert!(matches!(
            socket.next().await.unwrap().unwrap(),
            Message::Close(_)
        ));
        socket.flush().await.unwrap();
    });
    tokio::spawn(async move {
        let mut socket = NativeWebSocket::connect(&endpoint, WsConfig::default())
            .await
            .unwrap();
        socket.send("request").await.unwrap();
        assert_eq!(socket.receive().await.unwrap().as_deref(), Some("answer"));
        socket.close().await.unwrap();
    })
    .await
    .unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn trusted_wss_wrong_ca_and_wrong_name_use_real_tls() {
    for (host, roots, succeeds) in [
        ("localhost", true, true),
        ("localhost", false, false),
        ("127.0.0.1", true, false),
    ] {
        tls_case(host, roots, succeeds).await;
    }
}

#[tokio::test]
async fn plaintext_forbidden_and_redirect_handshakes_fail_without_retry() {
    for reply in [
        b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n".as_slice(),
        b"HTTP/1.1 302 Found\r\nLocation: ws://invalid/\r\n\r\n",
        b"not HTTP\r\n\r\n",
    ] {
        rejection(reply).await;
    }
    let listener = websocket_support::listener().await;
    let endpoint = websocket_support::endpoint(&listener, true, "localhost");
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut hello = [0; 5];
        stream.read_exact(&mut hello).await.unwrap();
        assert_eq!(hello[0], 22);
        stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await.unwrap();
    });
    assert_eq!(
        NativeWebSocket::connect(&endpoint, WsConfig::default())
            .await
            .unwrap_err(),
        WsError::Tls
    );
    server.await.unwrap();
}

#[tokio::test]
async fn oversized_fragmented_malformed_and_binary_frames_are_terminal() {
    for (wire, expected) in [
        (vec![0x81, 65], WsError::Limit),
        (
            [
                vec![0x01, 40],
                vec![b'a'; 40],
                vec![0x80, 40],
                vec![b'b'; 40],
            ]
            .concat(),
            WsError::Limit,
        ),
        (vec![0xc1, 0], WsError::Protocol),
        (vec![0x81, 1, 0xff], WsError::Protocol),
        (vec![0x82, 1, 7], WsError::Binary),
    ] {
        raw_frame(wire, expected).await;
    }
}

#[tokio::test]
async fn handshake_and_receive_deadlines_release_actual_peers() {
    let listener = websocket_support::listener().await;
    let endpoint = websocket_support::endpoint(&listener, false, "127.0.0.1");
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        read_headers(&mut stream).await;
        assert_eq!(eof(&mut stream).await, 0);
    });
    let config = WsConfig {
        connect_timeout: Duration::from_millis(DEADLINE_MS),
        ..WsConfig::default()
    };
    assert_eq!(
        NativeWebSocket::connect(&endpoint, config)
            .await
            .unwrap_err(),
        WsError::Deadline
    );
    server.await.unwrap();
    let (mut socket, peer) = quiet(WsConfig {
        operation_timeout: Duration::from_millis(DEADLINE_MS),
        ..WsConfig::default()
    })
    .await;
    assert_eq!(socket.receive().await.unwrap_err(), WsError::Deadline);

    assert_eq!(socket.receive().await.unwrap_err(), WsError::Closed);
    peer.await.unwrap();
}

struct WakeCount(AtomicUsize);
impl Wake for WakeCount {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[tokio::test]
async fn cancelled_receive_retains_owner_and_owner_drop_releases_repeated_peers() {
    for _ in 0..16 {
        let (mut socket, peer) = quiet(WsConfig::default()).await;
        let wakes = Arc::new(WakeCount(AtomicUsize::new(0)));
        let waker = Waker::from(wakes.clone());
        let mut receive = Box::pin(socket.receive());
        assert!(
            std::future::poll_fn(|_| std::task::Poll::Ready(
                receive
                    .as_mut()
                    .poll(&mut std::task::Context::from_waker(&waker))
            ))
            .await
            .is_pending()
        );
        drop(receive);
        tokio::time::sleep(Duration::from_millis(QUIET_MS)).await;

        assert_eq!(wakes.0.load(Ordering::SeqCst), 0);
        assert!(!peer.is_finished());
        drop(socket);
        timeout(Duration::from_secs(1), peer)
            .await
            .unwrap()
            .unwrap();
    }
}

/// Distinguishes a valid peer close frame from an unannounced TCP EOF.
#[tokio::test]
async fn abrupt_close_is_distinct_from_graceful_close() {
    for graceful in [false, true] {
        let listener = websocket_support::listener().await;
        let endpoint = websocket_support::endpoint(&listener, false, "127.0.0.1");
        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut peer = tokio_tungstenite::accept_async(stream).await.unwrap();
            if graceful {
                peer.send(Message::Close(None)).await.unwrap();
            }
        });
        let mut socket = NativeWebSocket::connect(&endpoint, WsConfig::default())
            .await
            .unwrap();

        if graceful {
            assert_eq!(socket.receive().await.unwrap(), None);
        } else {
            assert_eq!(socket.receive().await.unwrap_err(), WsError::AbruptClose);
        }

        server.await.unwrap();
    }
}

#[tokio::test]
async fn explicit_authentication_and_outgoing_boundary_keep_caller_policy() {
    let listener = websocket_support::listener().await;
    let endpoint = websocket_support::endpoint(&listener, false, "127.0.0.1");
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut peer = tokio_tungstenite::accept_hdr_async(stream, Authentication)
            .await
            .unwrap();
        assert_eq!(
            peer.next().await.unwrap().unwrap(),
            Message::Text("a".repeat(64).into())
        );
        assert!(matches!(
            peer.next().await.unwrap().unwrap(),
            Message::Close(_)
        ));
        peer.flush().await.unwrap();
    });
    let mut config = WsConfig {
        max_frame_bytes: 64,
        max_message_bytes: 64,
        read_buffer_bytes: 16,
        max_write_buffer_bytes: 78,
        ..WsConfig::default()
    };
    config
        .headers
        .insert("authorization", "Bearer fixture-token".parse().unwrap());
    assert!(!format!("{config:?}").contains("fixture-token"));
    let mut socket = NativeWebSocket::connect(&endpoint, config).await.unwrap();
    assert!(!format!("{socket:?}").contains("fixture-token"));

    assert_eq!(
        socket.send(&"a".repeat(65)).await.unwrap_err(),
        WsError::Limit
    );
    socket.send(&"a".repeat(64)).await.unwrap();
    socket.close().await.unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn cancelled_connect_and_consuming_close_drop_owned_sockets() {
    let listener = websocket_support::listener().await;
    let endpoint = websocket_support::endpoint(&listener, false, "127.0.0.1");
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        read_headers(&mut stream).await;
        assert_eq!(eof(&mut stream).await, 0);
    });
    assert!(
        timeout(
            Duration::from_millis(DEADLINE_MS),
            NativeWebSocket::connect(&endpoint, WsConfig::default())
        )
        .await
        .is_err()
    );
    server.await.unwrap();
    let listener = websocket_support::listener().await;
    let endpoint = websocket_support::endpoint(&listener, false, "127.0.0.1");
    let peer = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut peer = tokio_tungstenite::accept_async(stream).await.unwrap();
        assert!(matches!(
            peer.next().await.unwrap().unwrap(),
            Message::Close(_)
        ));
        assert_eq!(eof(peer.get_mut()).await, 0);
    });

    let socket = NativeWebSocket::connect(&endpoint, WsConfig::default())
        .await
        .unwrap();
    let mut closing = Box::pin(socket.close());
    assert!(
        std::future::poll_fn(|context| std::task::Poll::Ready(closing.as_mut().poll(context)))
            .await
            .is_pending()
    );

    drop(closing);
    peer.await.unwrap();
}

#[tokio::test]
async fn pending_receive_is_woken_by_actual_peer_data() {
    let listener = websocket_support::listener().await;
    let endpoint = websocket_support::endpoint(&listener, false, "127.0.0.1");
    let peer = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut peer = tokio_tungstenite::accept_async(stream).await.unwrap();
        tokio::time::sleep(Duration::from_millis(DEADLINE_MS)).await;
        peer.send(Message::Text("wake".into())).await.unwrap();
        assert!(matches!(
            peer.next().await.unwrap().unwrap(),
            Message::Close(_)
        ));
        peer.flush().await.unwrap();
    });
    let mut socket = NativeWebSocket::connect(&endpoint, WsConfig::default())
        .await
        .unwrap();
    let wakes = Arc::new(WakeCount(AtomicUsize::new(0)));
    let waker = Waker::from(wakes.clone());
    let mut receive = Box::pin(socket.receive());
    let first = std::future::poll_fn(|_| {
        std::task::Poll::Ready(
            receive
                .as_mut()
                .poll(&mut std::task::Context::from_waker(&waker)),
        )
    })
    .await;

    assert!(first.is_pending());
    tokio::time::sleep(Duration::from_millis(DEADLINE_MS * 2)).await;
    assert!(wakes.0.load(Ordering::SeqCst) > 0);
    assert_eq!(receive.await.unwrap().as_deref(), Some("wake"));
    socket.close().await.unwrap();
    peer.await.unwrap();
}

#[tokio::test]
async fn tls_handshake_and_consuming_close_deadlines_close_their_sockets() {
    let listener = websocket_support::listener().await;
    let endpoint = websocket_support::endpoint(&listener, true, "localhost");
    let peer = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut header = [0; 5];
        stream.read_exact(&mut header).await.unwrap();
        assert_eq!(header[0], 22);
        let length = usize::from(u16::from_be_bytes([header[3], header[4]]));
        stream.read_exact(&mut vec![0; length]).await.unwrap();
        assert_eq!(eof(&mut stream).await, 0);
    });
    let config = WsConfig {
        connect_timeout: Duration::from_millis(DEADLINE_MS),
        ..WsConfig::default()
    };
    assert_eq!(
        NativeWebSocket::connect(&endpoint, config)
            .await
            .unwrap_err(),
        WsError::Deadline
    );
    peer.await.unwrap();
    close_deadline().await;
}

async fn close_deadline() {
    let listener = websocket_support::listener().await;
    let endpoint = websocket_support::endpoint(&listener, false, "127.0.0.1");
    let peer = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut peer = tokio_tungstenite::accept_async(stream).await.unwrap();
        assert!(matches!(
            peer.next().await.unwrap().unwrap(),
            Message::Close(_)
        ));
        assert_eq!(eof(peer.get_mut()).await, 0);
    });
    let config = WsConfig {
        operation_timeout: Duration::from_millis(DEADLINE_MS),
        ..WsConfig::default()
    };
    let socket = NativeWebSocket::connect(&endpoint, config).await.unwrap();
    assert_eq!(socket.close().await.unwrap_err(), WsError::Deadline);
    peer.await.unwrap();
}
