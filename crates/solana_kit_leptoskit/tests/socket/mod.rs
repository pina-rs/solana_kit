//! Shared actual interpreter and bounded socket fixtures.
mod lifecycle;
mod ownership;
use futures_util::{SinkExt, StreamExt};
use leptoskit_bridge::{Bytes, CallId, MethodId, PluginId};
use leptoskit_native::HeadlessBackend;
use leptoskit_wasm::WasmHost;
use solana_kit_leptoskit::{HostLimits, HttpDriver, SocketCompletion, SocketDriver, SocketLimits};
use solana_kit_native::WsConfig;
use std::time::Duration;
use tokio::{net::TcpListener, sync::oneshot, task::JoinHandle};
use tokio_tungstenite::{accept_async, tungstenite::Message};

type Host = WasmHost<HeadlessBackend>;
const WAIT: Duration = Duration::from_secs(3);

fn host() -> Host {
    let wasm = wat::parse_str(
        r#"(module (memory (export "memory") 1)
        (func (export "leptoskit_main"))
        (func (export "leptoskit_plugins_ready"))
        (func (export "leptoskit_event") (param i64 i32 i32 i32) (result i32) i32.const 0))"#,
    )
    .unwrap();
    WasmHost::headless(&wasm).unwrap()
}

fn driver(host: &mut Host) -> SocketDriver {
    SocketDriver::new(
        host,
        SocketLimits {
            sockets: 1,
            messages: 2,
            frame_bytes: 64,
        },
    )
    .unwrap()
}

fn install(host: &mut Host, driver: &mut SocketDriver, endpoint: String) -> PluginId {
    let config = WsConfig {
        max_frame_bytes: 64,
        max_message_bytes: 64,
        read_buffer_bytes: 64,
        max_write_buffer_bytes: 78,
        ..Default::default()
    };
    driver
        .install(host, "socket", endpoint, config, true)
        .unwrap()
}

fn route(host: &mut Host, driver: &mut SocketDriver) {
    let mut http = HttpDriver::new(
        host,
        HostLimits {
            jobs: 1,
            request_bytes: 64,
            reply_bytes: 64,
        },
    )
    .unwrap();
    let routed = http.drain(host).unwrap();
    let routed = driver.route(host, routed).unwrap();
    assert!(routed.requests.is_empty() && routed.cancelled.is_empty());
}

async fn next(driver: &mut SocketDriver) -> SocketCompletion {
    tokio::time::timeout(WAIT, driver.next_completion())
        .await
        .unwrap()
        .unwrap()
}

fn submit(host: &mut Host, plugin: PluginId, method: u32, payload: Vec<u8>) -> CallId {
    host.plugins()
        .submit(plugin, MethodId(method), payload.into())
        .unwrap()
}

fn command(lease: CallId, body: &[u8]) -> Vec<u8> {
    let mut payload = lease.0.to_le_bytes().to_vec();
    payload.push(0);
    payload.extend_from_slice(body);
    payload
}

async fn fixture(text: bool) -> (String, JoinHandle<()>, oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("ws://{}/", listener.local_addr().unwrap());
    let (sender, receiver) = oneshot::channel();
    let peer = tokio::spawn(async move {
        let mut socket = accept_async(listener.accept().await.unwrap().0)
            .await
            .unwrap();

        if text {
            receiver.await.unwrap();
            socket.send(Message::Text("frame".into())).await.unwrap();
        }

        let incoming = socket.next().await;
        assert!(incoming.is_none() || incoming.is_some_and(|frame| frame.is_err()));
    });
    (endpoint, peer, sender)
}

async fn open(host: &mut Host, driver: &mut SocketDriver, plugin: PluginId) -> CallId {
    let lease = submit(host, plugin, 1, Vec::new());
    route(host, driver);
    let completion = next(driver).await;
    driver.deliver(host, completion).unwrap();
    let event = host.plugins().next_event().unwrap();
    assert_eq!(event.payload.len(), 9);
    assert_eq!(event.payload[8], 1);
    assert!(host.plugins().result(lease).unwrap().is_none());

    lease
}

async fn joined(peer: JoinHandle<()>) {
    tokio::time::timeout(WAIT, peer).await.unwrap().unwrap();
}
