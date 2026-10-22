use futures::{FutureExt, poll};
use solana_kit_browser::{BrowserWebSocket, WsConfig, WsError};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
    time::Duration,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
mod families;
mod lifecycle;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen(inline_js = "
let original; let sockets = []; let fail = false;
let captured;
export function capture_terminal() { captured = sockets.at(-1); }
export function drop_terminal() { captured.dispatchEvent(new Event('error')); }
export function capture() {
  if (original) return;
  original = globalThis.WebSocket;
  globalThis.WebSocket = class extends original {
    constructor(...args) { if(fail) throw new Error('private marker'); super(...args); sockets.push(this); }
  };
}
export function handlers() { let s=sockets.at(-1); return [s.onopen,s.onmessage,s.onerror,s.onclose].filter(Boolean).length; }
export function fail_constructor(value) { fail=value; }
export function late_events() { let s=sockets.at(-1); s.dispatchEvent(new Event('error')); s.dispatchEvent(new MessageEvent('message',{data:'late private text'})); s.dispatchEvent(new Event('open')); }
export function blob_event() { sockets.at(-1).dispatchEvent(new MessageEvent('message',{data:new Blob(['binary'])})); }
export function closed() { let s=sockets.at(-1); return new Promise(resolve => { if(s.readyState === 3) resolve(); else s.addEventListener('close',resolve,{once:true}); }); }
export function message_event() { return new Promise(resolve => sockets.at(-1).addEventListener('message',resolve,{once:true})); }
export function close_before_open() {
  const current = globalThis.WebSocket;
  globalThis.WebSocket = class extends current {
    constructor(...args) {
      super(...args);
      queueMicrotask(() => this.dispatchEvent(new CloseEvent('close',{code:1006,wasClean:false})));
      globalThis.WebSocket = current;
    }
  };
}
")]
extern "C" {
    fn capture();
    fn capture_terminal();
    fn drop_terminal();
    fn handlers() -> usize;
    fn fail_constructor(value: bool);
    fn late_events();
    fn blob_event();
    fn closed() -> js_sys::Promise;
    fn message_event() -> js_sys::Promise;
    fn close_before_open();
}
fn endpoint(path: &str) -> String {
    format!("{}{path}", env!("SOLANA_KIT_WS_FIXTURE"))
}
fn config() -> WsConfig {
    WsConfig {
        operation_timeout: Duration::from_secs(2),
        ..Default::default()
    }
}
async fn connect(path: &str) -> BrowserWebSocket {
    capture();
    BrowserWebSocket::connect(&endpoint(path), config())
        .await
        .unwrap()
}
struct Counter(AtomicUsize);
impl Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}
struct Reentrant;
impl Wake for Reentrant {
    fn wake(self: Arc<Self>) {
        late_events();
    }
}
#[wasm_bindgen_test]
async fn reentrant_wake_and_close_before_open_preserve_terminal_cause() {
    let mut socket = connect("/echo").await;
    let message = message_event();
    socket.send("go").unwrap();
    let waker = Waker::from(Arc::new(Reentrant));
    let mut context = Context::from_waker(&waker);
    let mut receive = socket.receive().boxed_local();
    assert!(matches!(receive.as_mut().poll(&mut context), Poll::Pending));

    JsFuture::from(message).await.unwrap();
    assert_eq!(receive.await, Err(WsError::Browser));
    assert_eq!(handlers(), 0);
    late_events();
    assert_eq!(socket.receive().await, Err(WsError::Browser));
    close_before_open();

    assert!(matches!(
        BrowserWebSocket::connect(&endpoint("/early-close"), config()).await,
        Err(WsError::PeerClosed {
            code: 1006,
            clean: false
        })
    ));
    assert_eq!(handlers(), 0);
}
#[wasm_bindgen_test]
async fn real_pending_cancel_resume_wake_and_drop_cleanup() {
    let mut socket = connect("/echo").await;
    assert_eq!(handlers(), 4);
    let counter = Arc::new(Counter(AtomicUsize::new(0)));
    let waker = Waker::from(counter.clone());
    let mut context = Context::from_waker(&waker);

    {
        let mut receive = socket.receive().boxed_local();
        assert!(matches!(receive.as_mut().poll(&mut context), Poll::Pending));
    }

    assert_eq!(handlers(), 4);
    let message = message_event();
    socket.send("go").unwrap();
    let mut receive = socket.receive().boxed_local();

    assert!(matches!(receive.as_mut().poll(&mut context), Poll::Pending));
    JsFuture::from(message).await.unwrap();
    assert!(counter.0.load(Ordering::SeqCst) > 0);
    assert_eq!(receive.await.unwrap(), "resumed");

    assert_eq!(handlers(), 4);
    drop(socket);
    assert_eq!(handlers(), 0);
    late_events();
    JsFuture::from(closed()).await.unwrap();
}
#[wasm_bindgen_test]
async fn network_queue_overflow_is_terminal_and_discards_queued_messages() {
    capture();
    let mut limits = config();
    limits.queue_messages = 1;

    let mut socket = BrowserWebSocket::connect(&endpoint("/burst"), limits)
        .await
        .unwrap();
    let terminal = closed();
    socket.send("go").unwrap();

    JsFuture::from(terminal).await.unwrap();
    assert_eq!(socket.receive().await, Err(WsError::QueueLimit));
    assert_eq!(handlers(), 0);
    late_events();

    assert_eq!(socket.receive().await, Err(WsError::QueueLimit));
}
#[wasm_bindgen_test]
async fn binary_blob_oversize_and_malformed_protocol_are_observable() {
    let mut binary = connect("/binary").await;
    assert_eq!(binary.receive().await, Err(WsError::Binary));
    assert_eq!(handlers(), 0);

    let mut blob = connect("/quiet").await;
    blob_event();
    assert_eq!(blob.receive().await, Err(WsError::Binary));
    assert_eq!(handlers(), 0);

    let mut limits = config();
    limits.message_bytes = 64;
    let mut oversize = BrowserWebSocket::connect(&endpoint("/oversize"), limits)
        .await
        .unwrap();
    assert_eq!(oversize.receive().await, Err(WsError::MessageLimit));

    let mut malformed = connect("/malformed").await;
    let mut owner = solana_kit_client::subscriptions::Connection::new(7, Default::default());
    assert!(
        owner
            .ingest(7, malformed.receive().await.unwrap().as_bytes())
            .is_err()
    );
    malformed.close().await.unwrap();
}
#[wasm_bindgen_test]
async fn explicit_close_peer_close_deadline_constructor_and_handshake_errors() {
    connect("/quiet").await.close().await.unwrap();
    assert_eq!(handlers(), 0);
    late_events();

    let mut peer = connect("/peer-close").await;
    assert!(matches!(
        peer.receive().await,
        Err(WsError::PeerClosed { .. })
    ));
    assert_eq!(handlers(), 0);

    let mut limits = config();
    limits.operation_timeout = Duration::from_millis(50);
    let mut quiet = BrowserWebSocket::connect(&endpoint("/quiet"), limits)
        .await
        .unwrap();
    assert_eq!(quiet.receive().await, Err(WsError::Deadline));
    assert_eq!(handlers(), 0);

    fail_constructor(true);
    assert_eq!(
        BrowserWebSocket::connect(&endpoint("/quiet"), config())
            .await
            .unwrap_err(),
        WsError::Constructor
    );
    fail_constructor(false);

    assert_eq!(
        BrowserWebSocket::connect(&endpoint("/refused"), config())
            .await
            .unwrap_err(),
        WsError::Browser
    );
    assert_eq!(handlers(), 0);
    assert_eq!(
        BrowserWebSocket::connect(env!("SOLANA_KIT_WSS_FIXTURE"), config())
            .await
            .unwrap_err(),
        WsError::Browser
    );
}
#[wasm_bindgen_test]
async fn cancelled_connect_and_close_detach_every_handler() {
    capture();
    {
        let url = endpoint("/stall");
        let future = BrowserWebSocket::connect(&url, config());
        let mut future = std::pin::pin!(future);
        assert!(poll!(&mut future).is_pending());
        handshake_observed().await;
    }
    assert_eq!(handlers(), 0);

    let socket = connect("/quiet").await;
    {
        let mut future = socket.close().boxed_local();
        assert!(poll!(&mut future).is_pending());
    }

    assert_eq!(handlers(), 0);
    JsFuture::from(closed()).await.unwrap();
}
async fn handshake_observed() {
    use solana_kit_client::Transport;
    let url = endpoint("/state").replacen("ws://", "http://", 1);
    let adapter = solana_kit_browser::BrowserHttp::new(&url, Default::default()).unwrap();

    for _ in 0..100 {
        let reply = adapter
            .exchange(solana_kit_client::Request {
                body: Vec::new(),
                max_reply_bytes: 1,
            })
            .await
            .unwrap();

        if reply.body == b"1" {
            return;
        }
    }

    panic!("fixture did not observe cancelled connection's upgrade request");
}
#[wasm_bindgen_test]
async fn external_bounds_and_redaction() {
    for path in [
        "http://localhost",
        "ws://user:private@localhost",
        "ws://localhost/#secret",
        "ws://localhost:0",
        "ws://localhost/ space",
    ] {
        assert_eq!(
            BrowserWebSocket::connect(path, config()).await.unwrap_err(),
            WsError::Config
        );
    }
    for protocols in [
        vec!["private protocol".into()],
        vec!["repeat".into(), "repeat".into()],
    ] {
        let mut limits = config();
        limits.protocols = protocols;
        assert_eq!(
            BrowserWebSocket::connect(&endpoint("/quiet"), limits)
                .await
                .unwrap_err(),
            WsError::Config
        );
    }
    let mut limits = config();
    limits.protocols = vec!["private-marker".into()];
    assert!(!format!("{limits:?}").contains("private-marker"));
    limits.message_bytes = 4;
    limits.buffered_bytes = 4;

    let mut socket = BrowserWebSocket::connect(&endpoint("/send-limit"), limits)
        .await
        .unwrap();
    assert_eq!(socket.send("12345"), Err(WsError::MessageLimit));

    socket.send("1234").unwrap();
    assert_eq!(socket.send("x"), Err(WsError::Backpressure));
    assert!(!format!("{socket:?}").contains("quiet"));
    socket.close().await.unwrap();
}
#[wasm_bindgen_test]
async fn utf8_byte_boundary_and_invalid_budgets_are_checked() {
    capture();
    let mut limits = config();
    limits.message_bytes = 4;
    let mut socket = BrowserWebSocket::connect(&endpoint("/unicode"), limits.clone())
        .await
        .unwrap();
    assert_eq!(socket.receive().await.unwrap(), "💖");
    socket.close().await.unwrap();

    let mut socket = BrowserWebSocket::connect(&endpoint("/unicode-overflow"), limits)
        .await
        .unwrap();
    assert_eq!(socket.receive().await, Err(WsError::MessageLimit));

    let invalid = [
        WsConfig {
            queue_messages: 0,
            ..config()
        },
        WsConfig {
            message_bytes: 0,
            ..config()
        },
        WsConfig {
            queue_bytes: 1,
            ..config()
        },
        WsConfig {
            buffered_bytes: 1,
            ..config()
        },
        WsConfig {
            operation_timeout: Duration::ZERO,
            ..config()
        },
        WsConfig {
            operation_timeout: Duration::MAX,
            ..config()
        },
    ];

    for limits in invalid {
        assert_eq!(
            BrowserWebSocket::connect(&endpoint("/quiet"), limits)
                .await
                .unwrap_err(),
            WsError::Config
        );
    }
}
