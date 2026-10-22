//! Negative control for upstream raw-wire logging, isolated in its own process.
use futures_util::{SinkExt, StreamExt};
use solana_kit_native::{NativeWebSocket, WsConfig};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio_tungstenite::tungstenite::Message;

static HEADER_EXPOSED: AtomicBool = AtomicBool::new(false);
static QUERY_EXPOSED: AtomicBool = AtomicBool::new(false);
struct Capture;
impl log::Log for Capture {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }
    fn log(&self, record: &log::Record<'_>) {
        let text = record.args().to_string();

        if text.contains("Bearer trace-fixture-secret") {
            HEADER_EXPOSED.store(true, Ordering::SeqCst);
        }

        if text.contains("query-fixture-secret") {
            QUERY_EXPOSED.store(true, Ordering::SeqCst);
        }
    }
    fn flush(&self) {}
}

#[tokio::test]
async fn sensitive_headers_do_not_redact_upstream_raw_wire_trace() {
    log::set_logger(&Capture).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!(
        "ws://{}/?token=query-fixture-secret",
        listener.local_addr().unwrap()
    );
    let peer = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut peer = tokio_tungstenite::accept_hdr_async(stream, Authentication)
            .await
            .unwrap();
        assert!(matches!(
            peer.next().await.unwrap().unwrap(),
            Message::Close(_)
        ));
        peer.flush().await.unwrap();
    });
    let mut config = WsConfig::default();
    config.headers.insert(
        "authorization",
        "Bearer trace-fixture-secret".parse().unwrap(),
    );
    let original = config.headers["authorization"].is_sensitive();

    let socket = NativeWebSocket::connect(&endpoint, config.clone())
        .await
        .unwrap();
    assert_eq!(config.headers["authorization"].is_sensitive(), original);
    socket.close().await.unwrap();
    peer.await.unwrap();
    log::set_max_level(log::LevelFilter::Off);
    assert!(HEADER_EXPOSED.load(Ordering::SeqCst));
    assert!(QUERY_EXPOSED.load(Ordering::SeqCst));
}

struct Authentication;
impl tokio_tungstenite::tungstenite::handshake::server::Callback for Authentication {
    fn on_request(
        self,
        request: &tokio_tungstenite::tungstenite::handshake::server::Request,
        response: tokio_tungstenite::tungstenite::handshake::server::Response,
    ) -> Result<
        tokio_tungstenite::tungstenite::handshake::server::Response,
        tokio_tungstenite::tungstenite::handshake::server::ErrorResponse,
    > {
        assert_eq!(
            request.headers()["authorization"],
            "Bearer trace-fixture-secret"
        );
        Ok(response)
    }
}
