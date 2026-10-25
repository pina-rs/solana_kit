//! Actual wallet completion resumes a guest into registered native HTTP work.
use super::WalletService;
use super::tests::{Sandbox, WASM, interpreter};
use crate::{HostLimits, HttpDriver, Routed, SocketDriver, SocketLimits};
use futures_util::StreamExt;
use leptoskit_bridge::{Bytes, CallId, Error, MethodId, PluginId, Request};
use leptoskit_bridge_ffi::app::{AppService, ServiceClose, ServiceWork};
use leptoskit_native::HeadlessBackend;
use leptoskit_wasm::WasmHost;
use solana_kit_core::{Address, Signature, crypto::verify};
use solana_kit_native::{NativeHttp, WsConfig};
use std::{
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::oneshot,
};

const WAIT: Duration = Duration::from_secs(3);
const TURNS: usize = 32;
const CHAIN_COUNT: usize = 96;

const DONE_REPORT: u32 = 5;
const SOCKET_REPORT: u32 = 6;
const FRAME_BYTES: usize = 64;
const WRITE_BUFFER_BYTES: usize = 78;
const VIEWPORT: (f32, f32) = (320.0, 480.0);
const MESSAGE: &[u8] = b"reviewed bytes";

const CHAIN: &[u8] = br#"(module
 (import "leptoskit_plugins_v1" "resolve"
  (func $resolve (param i32 i32) (result i64)))
 (import "leptoskit_plugins_v1" "submit"
  (func $submit (param i32 i32 i32 i32) (result i64)))
 (import "leptoskit_plugins_v1" "result_len" (func $result (param i64) (result i64)))
 (import "leptoskit_plugins_v1" "take_result"
  (func $take (param i64 i32 i32) (result i64)))
 (import "leptoskit_plugins_v1" "event_info" (func $info (param i32) (result i32)))
 (import "leptoskit_plugins_v1" "take_event"
  (func $event (param i32 i32) (result i64)))
 (memory (export "memory") 1)
 (data (i32.const 0) "solana-wallet-session")
 (data (i32.const 32) "composite-http")
 (data (i32.const 48) "platform")
 (data (i32.const 64) "\40\00\00\00{}")
 (data (i32.const 80) "reviewed bytes")
 (data (i32.const 112) "composite-socket")
 (global $wallet (mut i32) (i32.const 0))
 (global $http (mut i32) (i32.const 0))
 (global $platform (mut i32) (i32.const 0))
 (global $socket (mut i32) (i32.const 0))
 (global $call (mut i64) (i64.const 0))
 (global $stage (mut i32) (i32.const 0))
 (global $count (mut i32) (i32.const 0))
 (func $report (param $kind i32) (param $ptr i32) (param $len i32)
  (drop (call $submit (global.get $platform) (local.get $kind)
   (local.get $ptr) (local.get $len))))
 (func $ready (result i32)
  (i64.ne (call $result (global.get $call)) (i64.const -1)))
 (func (export "leptoskit_main")
  (global.set $wallet (i32.wrap_i64 (call $resolve (i32.const 0) (i32.const 21))))
  (global.set $http (i32.wrap_i64 (call $resolve (i32.const 32) (i32.const 14))))
  (global.set $platform (i32.wrap_i64 (call $resolve (i32.const 48) (i32.const 8))))
  (global.set $socket (i32.wrap_i64 (call $resolve (i32.const 112) (i32.const 16))))
  (global.set $call
   (call $submit (global.get $wallet) (i32.const 8) (i32.const 0) (i32.const 0))))
 (func (export "leptoskit_plugins_ready"))
 (func (export "leptoskit_event") (param i64 i32 i32 i32) (result i32) (i32.const 0))
 (func (export "leptoskit_poll") (local $length i32)
  (if (i32.eq (global.get $stage) (i32.const 0))
   (then
    (if (call $info (i32.const 128))
     (then
      (if (i32.ne (i32.load (i32.const 128)) (global.get $wallet)) (then unreachable))
      (local.set $length (i32.wrap_i64 (call $event (i32.const 256) (i32.const 128))))
      (call $report (i32.const 1) (i32.const 264)
       (i32.sub (local.get $length) (i32.const 8)))
      (global.set $stage (i32.const 1))
      (global.set $call
       (call $submit (global.get $wallet) (i32.const 10) (i32.const 80) (i32.const 14)))))))
  (if (i32.and (i32.eq (global.get $stage) (i32.const 1)) (call $ready))
   (then
    (local.set $length
     (i32.wrap_i64 (call $take (global.get $call) (i32.const 512) (i32.const 128))))
    (call $report (i32.const 2) (i32.const 512) (local.get $length))
    (global.set $stage (i32.const 2))
    (global.set $call
     (call $submit (global.get $http) (i32.const 0) (i32.const 64) (i32.const 6)))))
  (if (i32.and (i32.eq (global.get $stage) (i32.const 2)) (call $ready))
   (then
    (local.set $length
     (i32.wrap_i64 (call $take (global.get $call) (i32.const 768) (i32.const 128))))
    (call $report (i32.const 3) (i32.const 768) (local.get $length))
    (global.set $stage (i32.const 10))
    (global.set $call
     (call $submit (global.get $socket) (i32.const 1) (i32.const 0) (i32.const 0)))))
  (if (i32.eq (global.get $stage) (i32.const 10))
   (then
    (if (call $info (i32.const 128))
     (then
      (if (i32.ne (i32.load (i32.const 128)) (global.get $socket)) (then unreachable))
      (local.set $length (i32.wrap_i64 (call $event (i32.const 256) (i32.const 128))))
      (call $report (i32.const 6) (i32.const 256) (local.get $length))
      (global.set $stage (i32.const 11))))))
  (if (i32.eq (global.get $stage) (i32.const 11))
   (then
    (global.set $stage (i32.const 3))
    (global.set $call
     (call $submit (global.get $http) (i32.const 0) (i32.const 0) (i32.const 0)))))
  (if (i32.and (i32.eq (global.get $stage) (i32.const 3)) (call $ready))
   (then
    (drop (call $take (global.get $call) (i32.const 768) (i32.const 128)))
    (i32.store (i32.const 96) (global.get $count))
    (call $report (i32.const 4) (i32.const 96) (i32.const 4))
    (global.set $count (i32.add (global.get $count) (i32.const 1)))
    (if (i32.lt_u (global.get $count) (i32.const 96))
     (then
      (global.set $call
       (call $submit (global.get $http) (i32.const 0) (i32.const 0) (i32.const 0))))
     (else
      (global.set $stage (i32.const 4))
      (call $report (i32.const 5) (i32.const 0) (i32.const 0))))))))"#;

/// The test app owns one dispatcher, one retained batch and all shutdown futures.
struct Composite {
    http: HttpDriver,
    sockets: SocketDriver,
    wallet: WalletService,
    queued: ServiceWork,
    yields: usize,
}

impl Composite {
    fn new(host: &mut WasmHost<HeadlessBackend>, sandbox: &Sandbox) -> Self {
        Self {
            http: HttpDriver::new(
                host,
                HostLimits {
                    jobs: 1,
                    request_bytes: 64,
                    reply_bytes: 64,
                },
            )
            .unwrap(),
            sockets: SocketDriver::new(
                host,
                SocketLimits {
                    sockets: 1,
                    messages: 2,
                    frame_bytes: 64,
                },
            )
            .unwrap(),
            wallet: WalletService::install(host, &sandbox.0).unwrap(),
            queued: ServiceWork::default(),
            yields: 0,
        }
    }

    fn completions(&mut self, host: &mut WasmHost<HeadlessBackend>, context: &mut Context<'_>) {
        let http = {
            let completion = self.http.next_completion();
            std::pin::pin!(completion).poll(context)
        };

        if let Poll::Ready(completion) = http {
            self.http.deliver(host, completion.unwrap()).unwrap();
        }

        let socket = {
            let completion = self.sockets.next_completion();
            std::pin::pin!(completion).poll(context)
        };

        if let Poll::Ready(completion) = socket {
            self.sockets.deliver(host, completion.unwrap()).unwrap();
        }
    }
}

fn append(output: &mut ServiceWork, incoming: ServiceWork) {
    output.requests.extend(incoming.requests);
    output.cancelled.extend(incoming.cancelled);
}

fn drain(host: &mut WasmHost<HeadlessBackend>) -> ServiceWork {
    ServiceWork {
        cancelled: std::iter::from_fn(|| host.plugins().next_cancelled()).collect(),
        requests: std::iter::from_fn(|| host.plugins().next_request()).collect(),
    }
}

impl AppService for Composite {
    fn poll(
        &mut self,
        host: &mut WasmHost<HeadlessBackend>,
        context: &mut Context<'_>,
    ) -> Result<ServiceWork, Error> {
        let mut incoming = std::mem::take(&mut self.queued);
        append(&mut incoming, drain(host));
        let mut platform = ServiceWork::default();

        for _ in 0..TURNS {
            let routed = self
                .http
                .route(
                    host,
                    Routed {
                        requests: incoming.requests,
                        cancelled: incoming.cancelled,
                        guest_woken: false,
                    },
                )
                .unwrap();
            let routed = self.sockets.route(host, routed).unwrap();
            let work = self.wallet.route(
                host,
                context,
                ServiceWork {
                    requests: routed.requests,
                    cancelled: routed.cancelled,
                },
            )?;
            append(&mut platform, work);
            self.completions(host, context);
            incoming = drain(host);

            if incoming.requests.is_empty() && incoming.cancelled.is_empty() {
                return Ok(platform);
            }
        }

        self.queued = incoming;
        self.yields += 1;
        context.waker().wake_by_ref();
        Ok(platform)
    }

    fn close(self: Box<Self>) -> ServiceClose {
        let Self {
            mut http,
            mut sockets,
            wallet,
            ..
        } = *self;
        let wallet = Box::new(wallet).close();
        Box::pin(async move {
            let (http, sockets, wallet) = tokio::join!(http.shutdown(), sockets.shutdown(), wallet);
            http.map_err(|_| Error::Native)?;
            sockets.map_err(|_| Error::Native)?;
            wallet
        })
    }
}

fn install_http(
    host: &mut WasmHost<HeadlessBackend>,
    app: &mut Composite,
    listener: &TcpListener,
) -> PluginId {
    let endpoint = format!("http://{}/", listener.local_addr().unwrap());
    let native = NativeHttp::new(&endpoint, Default::default()).unwrap();
    app.http
        .install(host, "composite-http", native, true)
        .unwrap()
}

fn platform(host: &mut WasmHost<HeadlessBackend>) -> PluginId {
    let methods = (1..=SOCKET_REPORT).map(MethodId);
    let plugin = host.plugins().register("platform", methods).unwrap();

    for method in 1..=SOCKET_REPORT {
        host.plugins()
            .grant(plugin, MethodId(method), true)
            .unwrap();
    }

    plugin
}

async fn peer(
    listener: TcpListener,
    received: oneshot::Sender<()>,
    release: oneshot::Receiver<()>,
) {
    let (mut socket, _) = listener.accept().await.unwrap();
    let mut headers = Vec::new();

    while !headers.ends_with(b"\r\n\r\n") {
        headers.push(socket.read_u8().await.unwrap());
        assert!(headers.len() < 4096);
    }

    let mut body = [0; 2];
    socket.read_exact(&mut body).await.unwrap();
    assert_eq!(&body, b"{}");
    received.send(()).unwrap();
    release.await.unwrap();
    socket
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\nokay")
        .await
        .unwrap();
    socket.shutdown().await.unwrap();
}

async fn socket_peer(
    host: &mut WasmHost<HeadlessBackend>,
    app: &mut Composite,
) -> tokio::task::JoinHandle<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("ws://{}/", listener.local_addr().unwrap());
    let config = WsConfig {
        max_frame_bytes: FRAME_BYTES,
        max_message_bytes: FRAME_BYTES,
        read_buffer_bytes: FRAME_BYTES,
        max_write_buffer_bytes: WRITE_BUFFER_BYTES,
        ..Default::default()
    };
    app.sockets
        .install(host, "composite-socket", endpoint, config, true)
        .unwrap();
    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
        let ended = socket.next().await;
        use tokio_tungstenite::tungstenite::{Error, Message, error::ProtocolError};

        match ended {
            None | Some(Ok(Message::Close(_))) | Some(Err(Error::ConnectionClosed)) => {}
            Some(Err(Error::Protocol(ProtocolError::ResetWithoutClosingHandshake))) => {}
            Some(Err(Error::Io(error)))
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::ConnectionReset
                        | std::io::ErrorKind::ConnectionAborted
                        | std::io::ErrorKind::BrokenPipe
                        | std::io::ErrorKind::UnexpectedEof
                ) => {}
            other => panic!("unexpected socket peer shutdown: {other:?}"),
        }
    })
}

struct CountingWake {
    count: Arc<AtomicUsize>,
    parent: Waker,
}

impl Wake for CountingWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.parent.wake_by_ref();
    }
}

fn turn(
    app: &mut Composite,
    host: &mut WasmHost<HeadlessBackend>,
    context: &mut Context<'_>,
    count: &Arc<AtomicUsize>,
    platform: PluginId,
) -> ServiceWork {
    let waker = Waker::from(Arc::new(CountingWake {
        count: Arc::clone(count),
        parent: context.waker().clone(),
    }));
    let before = count.load(Ordering::Relaxed);
    let yields = app.yields;
    let work = app.poll(host, &mut Context::from_waker(&waker)).unwrap();
    assert!(
        host.plugins().next_request().is_none(),
        "raw work escaped its owner"
    );
    assert!(
        work.requests
            .iter()
            .all(|request| request.plugin == platform),
        "SDK work bypassed its native owner"
    );

    if app.yields != yields {
        assert!(count.load(Ordering::Relaxed) > before);
        assert!(!app.queued.requests.is_empty() || !app.queued.cancelled.is_empty());
    }

    work
}

fn assert_reports(requests: &[Request], plugin: PluginId) {
    assert_eq!(requests.len(), CHAIN_COUNT + 5);
    assert!(requests.iter().all(|request| request.plugin == plugin));
    let address: Address = std::str::from_utf8(requests[0].payload.as_ref())
        .unwrap()
        .parse()
        .unwrap();
    let signature: Signature = std::str::from_utf8(requests[1].payload.as_ref())
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(requests[0].method, MethodId(1));
    assert_eq!(requests[1].method, MethodId(2));
    verify(&address, MESSAGE, &signature).unwrap();
    assert_eq!(requests[2].method, MethodId(3));
    assert_eq!(requests[2].payload.as_ref(), b"\0\xc8\0okay");
    assert_eq!(requests[3].method, MethodId(SOCKET_REPORT));
    assert_eq!(requests[3].payload.len(), 9);
    assert_eq!(requests[3].payload.as_ref()[8], 1);

    for (index, request) in requests[4..4 + CHAIN_COUNT].iter().enumerate() {
        assert_eq!(request.method, MethodId(4));
        assert_eq!(request.payload.as_ref(), &(index as u32).to_le_bytes());
    }

    assert_eq!(requests.last().unwrap().method, MethodId(DONE_REPORT));
}

async fn drive_until(
    app: &mut Composite,
    host: &mut WasmHost<HeadlessBackend>,
    platform: PluginId,
    requests: &mut Vec<Request>,
    finished: impl Fn(&Composite, &[Request]) -> bool,
) {
    let wakes = Arc::new(AtomicUsize::new(0));
    tokio::time::timeout(
        WAIT,
        std::future::poll_fn(|context| {
            let work = turn(app, host, context, &wakes, platform);
            assert!(work.cancelled.is_empty());
            requests.extend(work.requests);

            if finished(app, requests) {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn wallet_signing_continuation_reaches_native_io_owners_and_preserves_platform_fifo() {
    let sandbox = Sandbox::new();
    let mut host = interpreter(CHAIN);
    let mut app = Composite::new(&mut host, &sandbox);
    let foreign = platform(&mut host);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    install_http(&mut host, &mut app, &listener);
    let socket = socket_peer(&mut host, &mut app).await;

    let (received, observed) = oneshot::channel();
    let (release, released) = oneshot::channel();
    let server = tokio::spawn(peer(listener, received, released));
    let mut requests = Vec::new();
    host.start(VIEWPORT.0, VIEWPORT.1).unwrap();

    drive_until(&mut app, &mut host, foreign, &mut requests, |app, _| {
        app.http.retained_jobs() == 1
    })
    .await;
    tokio::time::timeout(WAIT, observed).await.unwrap().unwrap();
    release.send(()).unwrap();

    drive_until(
        &mut app,
        &mut host,
        foreign,
        &mut requests,
        |_, requests| {
            requests
                .last()
                .is_some_and(|request| request.method == MethodId(DONE_REPORT))
        },
    )
    .await;
    assert_reports(&requests, foreign);
    assert!(app.yields > 0);
    assert_eq!(app.http.retained_jobs(), 0);
    assert_eq!(app.sockets.retained_sockets(), 1);
    tokio::time::timeout(WAIT, Box::new(app).close())
        .await
        .unwrap()
        .unwrap();
    tokio::time::timeout(WAIT, server).await.unwrap().unwrap();
    tokio::time::timeout(WAIT, socket).await.unwrap().unwrap();
}

#[tokio::test]
async fn cancelled_held_http_request_never_enters_native_work() {
    let sandbox = Sandbox::new();
    let mut host = interpreter(WASM);
    let mut app = Composite::new(&mut host, &sandbox);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let plugin = install_http(&mut host, &mut app, &listener);

    let call: CallId = host
        .plugins()
        .submit(plugin, MethodId(0), Bytes::from_static(b"\x40\0\0\0{}"))
        .unwrap();
    app.queued = drain(&mut host);
    assert_eq!(app.queued.requests.len(), 1);
    host.plugins().cancel(call).unwrap();

    let waker = Waker::noop();
    let work = app
        .poll(&mut host, &mut Context::from_waker(waker))
        .unwrap();
    assert!(work.requests.is_empty());
    assert_eq!(app.http.retained_jobs(), 0);
    assert!(matches!(
        host.plugins().result(call),
        Err(Error::InvalidCall)
    ));
    tokio::time::timeout(WAIT, Box::new(app).close())
        .await
        .unwrap()
        .unwrap();
}
