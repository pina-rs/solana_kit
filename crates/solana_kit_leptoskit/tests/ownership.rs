//! Retained tokens must not retain callbacks, payload owners, or host authority.
#![cfg(feature = "host")]
use leptoskit_bridge::{Bridge, Bytes, CallId, MethodId, Plugin};
use leptoskit_bridge_ffi::app::App;
use leptoskit_native::HeadlessBackend;
use leptoskit_wasm::WasmHost;
use solana_kit_leptoskit::{
    HostError as Error, HostLimits as Limits, HttpCompletion as Completion, HttpDriver as Driver,
};
use solana_kit_native::NativeHttp;
use std::{
    rc::Rc,
    sync::Arc,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::oneshot,
};

const WASM: &[u8] = br#"(module
 (memory (export "memory") 1)
 (func (export "leptoskit_main"))
 (func (export "leptoskit_plugins_ready"))
 (func (export "leptoskit_event") (param i64 i32 i32 i32) (result i32) i32.const 0))"#;

fn host() -> WasmHost<HeadlessBackend> {
    WasmHost::headless(WASM).unwrap()
}

fn limits() -> Limits {
    Limits {
        jobs: 1,
        request_bytes: 16,
        reply_bytes: 16,
    }
}

struct DropOwner(Arc<AtomicUsize>);

impl Drop for DropOwner {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

impl AsRef<[u8]> for DropOwner {
    fn as_ref(&self) -> &[u8] {
        b"payload"
    }
}

impl Plugin for DropOwner {
    fn name(&self) -> &'static str {
        "drop-observer"
    }

    fn methods(&self) -> &'static [MethodId] {
        &[MethodId(0)]
    }

    fn invoke(&mut self, _: MethodId, payload: Bytes) -> Result<Bytes, leptoskit_bridge::Error> {
        Ok(payload)
    }
}

#[tokio::test]
async fn retained_tokens_release_actual_app_callback_before_worker_finishes() {
    let dropped = Arc::new(AtomicUsize::new(0));
    let mut app = App::new(WASM).unwrap();
    app.install(Box::new(DropOwner(Arc::clone(&dropped))))
        .unwrap();
    let local = Rc::new(AtomicUsize::new(0));
    let captured = Rc::clone(&local);
    let identities = app.with_host(move |host| {
        captured.fetch_add(1, Ordering::SeqCst);
        (host.identity(), host.plugins().identity())
    });
    let (release, gate) = oneshot::channel();
    let retained = identities.clone();
    let worker = tokio::spawn(async move {
        gate.await.unwrap();
        retained
    });

    app.with_host(|host| assert_eq!(identities.0, host.identity()));
    assert_eq!(local.load(Ordering::SeqCst), 1);
    drop(app);
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
    release.send(()).unwrap();
    assert_eq!(worker.await.unwrap(), identities);
}

#[test]
fn retained_bridge_identity_releases_queued_payload_owner() {
    let dropped = Arc::new(AtomicUsize::new(0));
    let mut bridge = Bridge::default();
    let plugin = bridge.register("payload", [MethodId(0)]).unwrap();
    bridge.grant(plugin, MethodId(0), true).unwrap();
    let identity = bridge.identity();
    bridge
        .submit(
            plugin,
            MethodId(0),
            Bytes::from_owner(DropOwner(Arc::clone(&dropped))),
        )
        .unwrap();
    assert_eq!(dropped.load(Ordering::SeqCst), 0);
    drop(bridge);
    assert_eq!(dropped.load(Ordering::SeqCst), 1);

    for _ in 0..512 {
        assert_ne!(identity, Bridge::default().identity());
    }
}

#[tokio::test]
async fn repeated_transplant_and_restore_checks_both_axes_without_poisoning_owner() {
    let mut original = host();
    let host_identity = original.identity();
    let bridge_identity = original.plugins().identity();
    let mut driver = Driver::new(&mut original, limits()).unwrap();

    for _ in 0..64 {
        let mut neighbor = host();
        std::mem::swap(original.plugins(), neighbor.plugins());
        assert_eq!(bridge_identity, neighbor.plugins().identity());
        assert_eq!(host_identity, original.identity());
        assert!(matches!(driver.drain(&mut original), Err(Error::Owner)));
        assert!(matches!(driver.drain(&mut neighbor), Err(Error::Owner)));
        std::mem::swap(original.plugins(), neighbor.plugins());
        assert!(driver.drain(&mut original).unwrap().requests.is_empty());
    }

    driver.shutdown().await.unwrap();
}

async fn gated_reply(
    listener: TcpListener,
    ready: oneshot::Sender<()>,
    gate: oneshot::Receiver<()>,
) {
    let (mut socket, _) = listener.accept().await.unwrap();
    let mut headers = Vec::new();

    while !headers.ends_with(b"\r\n\r\n") {
        headers.push(socket.read_u8().await.unwrap());
        assert!(headers.len() < 1024);
    }

    let mut body = [0; 2];
    socket.read_exact(&mut body).await.unwrap();
    assert_eq!(body, *b"{}");
    ready.send(()).unwrap();
    gate.await.unwrap();
    socket
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\nokay")
        .await
        .unwrap();
    socket.shutdown().await.unwrap();
}

async fn join_completion(mut driver: Driver) -> (Driver, Completion) {
    let completion = tokio::time::timeout(Duration::from_secs(3), driver.next_completion())
        .await
        .unwrap()
        .unwrap();
    (driver, completion)
}

#[tokio::test]
async fn pending_send_wait_outlives_app_drop_but_cannot_authorize_transplanted_successor() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/", listener.local_addr().unwrap());
    let (accepted, observed) = oneshot::channel();
    let (release, gate) = oneshot::channel();
    let peer = tokio::spawn(gated_reply(listener, accepted, gate));
    let mut app = App::new(WASM).unwrap();
    let mut driver = app.with_host(|host| Driver::new(host, limits())).unwrap();
    let call: CallId = app.with_host(|host| {
        let service = NativeHttp::new(&endpoint, Default::default()).unwrap();
        let plugin = driver.install(host, "delayed", service, true).unwrap();
        host.plugins()
            .submit(plugin, MethodId(0), Bytes::from_static(b"\x10\0\0\0{}"))
            .unwrap()
    });
    app.with_host(|host| driver.drain(host)).unwrap();
    observed.await.unwrap();
    let waiting = tokio::spawn(join_completion(driver));
    let bridge = app.with_host(|host| std::mem::take(host.plugins()));
    let retained_host = app.with_host(|host| host.identity());
    app.with_host(|host| assert_eq!(host.commits(), 0));
    drop(app);
    release.send(()).unwrap();
    let (mut driver, completion) = waiting.await.unwrap();
    peer.await.unwrap();
    assert_eq!(driver.retained_jobs(), 0);
    let mut successor = host();
    *successor.plugins() = bridge;
    assert_ne!(retained_host, successor.identity());
    assert!(successor.plugins().result(call).unwrap().is_none());
    assert!(matches!(
        driver.deliver(&mut successor, completion),
        Err(Error::Owner)
    ));
    assert!(successor.plugins().result(call).unwrap().is_none());
    driver.shutdown().await.unwrap();
}
