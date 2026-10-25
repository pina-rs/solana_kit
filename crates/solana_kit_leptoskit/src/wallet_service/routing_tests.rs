//! Real interpreter callbacks submit follow-up requests before the service yields.
use super::tests::{Sandbox, interpreter};
use super::*;
use leptoskit_bridge::{Bytes, Request};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Wake, Waker};

const CHAIN: &[u8] = br#"(module
 (import "leptoskit_plugins_v1" "resolve" (func $resolve (param i32 i32) (result i64)))
 (import "leptoskit_plugins_v1" "submit" (func $submit (param i32 i32 i32 i32) (result i64)))
 (import "leptoskit_plugins_v1" "result_len" (func $result (param i64) (result i64)))
 (import "leptoskit_plugins_v1" "take_result" (func $take (param i64 i32 i32) (result i64)))
 (memory (export "memory") 1)
 (data (i32.const 0) "solana-wallet-session")
 (data (i32.const 32) "foreign")
 (global $wallet (mut i32) (i32.const 0))
 (global $foreign (mut i32) (i32.const 0))
 (global $call (mut i64) (i64.const 0))
 (global $count (mut i32) (i32.const 0))
 (func (export "leptoskit_main")
  (global.set $wallet (i32.wrap_i64 (call $resolve (i32.const 0) (i32.const 21))))
  (global.set $foreign (i32.wrap_i64 (call $resolve (i32.const 32) (i32.const 7))))
  (global.set $call (call $submit (global.get $wallet) (i32.const 1) (i32.const 0) (i32.const 0))))
 (func (export "leptoskit_plugins_ready"))
 (func (export "leptoskit_event") (param i64 i32 i32 i32) (result i32) (i32.const 0))
 (func (export "leptoskit_poll")
  (if (i32.and (i32.lt_u (global.get $count) (i32.const 96))
      (i64.ne (call $result (global.get $call)) (i64.const -1)))
   (then
    (drop (call $take (global.get $call) (i32.const 128) (i32.const 1024)))
    (i32.store (i32.const 64) (global.get $count))
    (drop (call $submit (global.get $foreign) (i32.const 0) (i32.const 64) (i32.const 4)))
    (global.set $count (i32.add (global.get $count) (i32.const 1)))
    (global.set $call (call $submit (global.get $wallet) (i32.const 2) (i32.const 0) (i32.const 0)))))))"#;
const VIEWPORT_WIDTH: f32 = 320.0;
const VIEWPORT_HEIGHT: f32 = 480.0;

#[derive(Default)]
struct Wakes(AtomicUsize);

impl Wake for Wakes {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

fn foreign(host: &mut WasmHost<HeadlessBackend>) -> PluginId {
    let plugin = host.plugins().register("foreign", [MethodId(0)]).unwrap();
    host.plugins().grant(plugin, MethodId(0), true).unwrap();
    plugin
}

fn complete_connect(host: &mut WasmHost<HeadlessBackend>, service: &mut WalletService) {
    host.start(VIEWPORT_WIDTH, VIEWPORT_HEIGHT).unwrap();
    let connect = host.plugins().next_request().unwrap();
    assert_eq!(connect.method, MethodId(wire::CONNECT_MOBILE));
    let epoch = service.shared.epoch();
    service
        .pending
        .insert(connect.id, (epoch, wire::CONNECT_MOBILE));
    // Synthetic native completion isolates routing; actual MWA peer proof is separate.
    service
        .shared
        .complete(worker::Completion {
            call: connect.id,
            epoch,
            result: Ok(vec![]),
        })
        .unwrap();
}

#[tokio::test]
async fn completion_routes_real_guest_authorize_and_retains_quantum_overflow() {
    let sandbox = Sandbox::new();
    let mut host = interpreter(CHAIN);
    let plugin = foreign(&mut host);
    let mut service = WalletService::install(&mut host, &sandbox.0).unwrap();
    complete_connect(&mut host, &mut service);
    let wakes = Arc::new(Wakes::default());
    let waker = Waker::from(wakes.clone());
    let mut context = Context::from_waker(&waker);
    let mut requests = Vec::new();

    for turn in 0..8 {
        let work = service.poll(&mut host, &mut context).unwrap();
        assert!(work.cancelled.is_empty());
        requests.extend(work.requests);
        assert!(
            host.plugins().next_request().is_none(),
            "raw request escaped on turn {turn}"
        );

        if service.queued.requests.is_empty() {
            break;
        }

        assert!(wakes.0.load(Ordering::Relaxed) > 0);
    }

    assert_eq!(requests.len(), 96);

    for (index, request) in requests.iter().enumerate() {
        assert_eq!(request.plugin, plugin);
        assert_eq!(request.payload.as_ref(), &(index as u32).to_le_bytes());
    }

    assert!(service.queued.requests.is_empty());
    Box::new(service).close().await.unwrap();
}

#[tokio::test]
async fn deferred_owned_cancellation_is_consumed_without_forwarding_or_admission() {
    let sandbox = Sandbox::new();
    let mut host = interpreter(super::tests::WASM);
    let plugin = foreign(&mut host);
    let mut service = WalletService::install(&mut host, &sandbox.0).unwrap();
    let own = host
        .plugins()
        .submit(service.plugin, MethodId(wire::AUTHORIZE), Bytes::default())
        .unwrap();
    let request = host.plugins().next_request().unwrap();
    service.queued.requests.push(request);
    host.plugins().cancel(own).unwrap();

    let other = host
        .plugins()
        .submit(plugin, MethodId(0), Bytes::from_static(b"retained"))
        .unwrap();
    let incoming = ServiceWork {
        requests: vec![],
        cancelled: vec![],
    };
    let waker = futures_util::task::noop_waker();
    let mut context = Context::from_waker(&waker);
    let routed = service.route(&mut host, &mut context, incoming).unwrap();

    assert!(routed.cancelled.is_empty());
    assert_eq!(
        routed
            .requests
            .iter()
            .map(|request: &Request| request.id)
            .collect::<Vec<_>>(),
        [other]
    );
    assert!(service.pending.is_empty());
    assert!(service.drained.is_empty());
    assert_eq!(service.calls, 0);
    Box::new(service).close().await.unwrap();
}
