//! Actual interpreter routing and pending native work use the generated-host scheduler.
use super::*;
use leptoskit_bridge::{Bytes, MethodId};
use std::{
    fs,
    net::TcpListener,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
    task::{Context, Wake, Waker},
    thread::{self, JoinHandle},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const WAIT: Duration = Duration::from_secs(5);
const WASM: &[u8] = br#"(module
 (memory (export "memory") 1)
 (func (export "leptoskit_main"))
 (func (export "leptoskit_plugins_ready"))
 (func (export "leptoskit_event") (param i64 i32 i32 i32) (result i32) i32.const 0))"#;
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct Sandbox(PathBuf);

impl Sandbox {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("solana-journey-{nonce}-{sequence}"));
        fs::create_dir(&path).unwrap();
        Self(path.canonicalize().unwrap())
    }

    fn config(&self, http: &str, socket: &str) {
        let json = serde_json::json!({"http":http,"socket":socket});
        fs::write(
            self.0.join("journey.json"),
            serde_json::to_vec(&json).unwrap(),
        )
        .unwrap();
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn interpreter() -> Host {
    Host::headless(&wat::parse_bytes(WASM).unwrap()).unwrap()
}

struct ThreadWake(thread::Thread);

impl Wake for ThreadWake {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// A real peer withholds its response, then observes cancellation as TCP EOF.
fn pending_peer(accepted: mpsc::Sender<()>) -> (u16, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let port = listener.local_addr().unwrap().port();
    let owner = thread::current();
    let peer = thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async move {
            tokio::time::timeout(WAIT, peer_eof(listener, accepted, owner))
                .await
                .expect("pending native peer deadline");
        });
    });
    (port, peer)
}

async fn peer_eof(listener: TcpListener, accepted: mpsc::Sender<()>, owner: thread::Thread) {
    use tokio::io::AsyncReadExt;
    let listener = tokio::net::TcpListener::from_std(listener).unwrap();
    let (mut stream, _) = listener.accept().await.unwrap();
    let mut bytes = [0; 4096];
    assert!(stream.read(&mut bytes).await.unwrap() > 0);
    accepted.send(()).unwrap();
    owner.unpark();

    while stream.read(&mut bytes).await.unwrap() != 0 {}
}

fn submit(host: &mut Host, name: &str, method: u32, bytes: &[u8]) -> leptoskit_bridge::CallId {
    let plugin = host.plugins().resolve(name).unwrap();
    host.plugins()
        .submit(plugin, MethodId(method), Bytes::from(bytes.to_vec()))
        .unwrap()
}

#[test]
fn activation_is_inert_until_the_generated_host_installs_the_service() {
    let sandbox = Sandbox::new();
    sandbox.config("http://127.0.0.1:8899", "ws://127.0.0.1:8900");
    let mut host = interpreter();
    let install = from_app_dirs(&sandbox.0, &sandbox.0).unwrap();
    assert!(host.plugins().resolve("solana-wallet-session").is_none());
    assert!(!sandbox.0.join("solana-wallet").exists());
    let service = install(&mut host).unwrap();

    for name in [
        "solana-wallet-session",
        "solana.rpc",
        "solana.socket",
        "journey.timer",
    ] {
        assert!(host.plugins().resolve(name).is_some());
    }

    futures::executor::block_on(service.close()).unwrap();
}

#[test]
/// Admit every real owner before closing. Peer notifications wake the native UI
/// thread; the deadline bounds a missing notification without a polling sleep.
fn close_joins_pending_http_socket_timer_and_wallet_work() {
    let sandbox = Sandbox::new();
    let (sender, accepted) = mpsc::channel();
    let (http, http_peer) = pending_peer(sender.clone());
    let (socket, socket_peer) = pending_peer(sender);
    sandbox.config(
        &format!("http://127.0.0.1:{http}"),
        &format!("ws://127.0.0.1:{socket}"),
    );

    let mut host = interpreter();
    let mut service = from_app_dirs(&sandbox.0, &sandbox.0).unwrap()(&mut host).unwrap();
    let platform = host.plugins().register("unrelated", [MethodId(1)]).unwrap();
    host.plugins().grant(platform, MethodId(1), true).unwrap();
    let unrelated = submit(&mut host, "unrelated", 1, b"preserve this work");

    let wallet = submit(&mut host, "solana-wallet-session", 8, &[]);
    submit(&mut host, "solana.socket", 1, &[]);
    submit(&mut host, "journey.timer", 1, &60_000_u64.to_le_bytes());
    submit(&mut host, "solana.rpc", 0, b"\x40\0\0\0{}");

    let waker = Waker::from(Arc::new(ThreadWake(thread::current())));
    let mut context = Context::from_waker(&waker);
    let deadline = Instant::now() + WAIT;
    let mut peers = 0;
    let mut opened = false;
    let mut returned = Vec::new();

    while peers != 2 || !opened || returned.is_empty() {
        let work = service.poll(&mut host, &mut context).unwrap();
        assert!(work.cancelled.is_empty());
        returned.extend(work.requests);
        peers += accepted.try_iter().count();

        while let Some(event) = host.plugins().next_event() {
            assert_eq!(&event.payload.as_ref()[..8], &wallet.0.to_le_bytes());
            opened = true;
        }

        if peers != 2 || !opened || returned.is_empty() {
            let remaining = deadline
                .checked_duration_since(Instant::now())
                .expect("native admission deadline");
            thread::park_timeout(remaining);
        }
    }

    assert_eq!(returned.len(), 1);
    assert_eq!(returned[0].id, unrelated);
    assert_eq!(returned[0].payload.as_ref(), b"preserve this work");
    futures::executor::block_on(service.close()).unwrap();
    http_peer.join().unwrap();
    socket_peer.join().unwrap();
}

#[test]
fn rejects_activation_inside_an_unrelated_tokio_runtime() {
    let sandbox = Sandbox::new();
    sandbox.config("http://127.0.0.1:8899", "ws://127.0.0.1:8900");
    let mut host = interpreter();
    let install = from_app_dirs(&sandbox.0, &sandbox.0).unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let _entered = runtime.enter();
    assert!(matches!(install(&mut host), Err(Error::InvalidArgument)));
    assert!(host.plugins().resolve("solana-wallet-session").is_none());
}

#[test]
fn malformed_and_oversized_files_fail_before_activation() {
    let sandbox = Sandbox::new();
    assert!(matches!(
        from_app_dirs(&sandbox.0, &sandbox.0),
        Err(Error::InvalidArgument)
    ));
    fs::write(sandbox.0.join("journey.json"), b"{").unwrap();
    assert!(matches!(
        from_app_dirs(&sandbox.0, &sandbox.0),
        Err(Error::InvalidArgument)
    ));
    fs::write(sandbox.0.join("journey.json"), [b' '; 4097]).unwrap();
    assert!(matches!(
        from_app_dirs(&sandbox.0, &sandbox.0),
        Err(Error::InvalidArgument)
    ));
}
