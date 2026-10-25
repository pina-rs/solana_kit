//! Application-owned local executor and singleton plugin event receiver.

use futures::channel::oneshot;
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use leptoskit_bridge::guest::Events;
use leptoskit_bridge::{Error, PluginId};
use std::cell::RefCell;

const TRACE_LIMIT: usize = 32;
const CANCEL_EVENT: u32 = 7;

thread_local! {
    static POOL: RefCell<LocalPool> = RefCell::new(LocalPool::new());
    static TRACE: RefCell<Vec<&'static str>> = const { RefCell::new(Vec::new()) };
}

#[link(wasm_import_module = "leptoskit")]
unsafe extern "C" {
    fn node_create(id: u64, kind: i32);
    fn node_set_props(id: u64, ptr: *const u8, len: usize);
    fn commit();
}

/// Records bounded observable progress, with no host-specific logging import.
pub(super) fn record(event: &'static str) {
    TRACE.with_borrow_mut(|trace| {
        assert!(trace.len() < TRACE_LIMIT, "bounded trace capacity");
        trace.push(event);
    });
}

/// Publishes after executor borrows end; the real renderer copies the live bytes.
pub(super) fn publish() {
    let props = TRACE.with_borrow(|trace| {
        serde_json::json!({"text": serde_json::to_string(trace).expect("trace JSON")}).to_string()
    });

    // Safety: node 1 exists and the synchronous renderer copies this live slice.
    unsafe {
        node_set_props(1, props.as_ptr(), props.len());
        commit();
    }
}

/// The application routes its sole event receiver; the capability client has none.
async fn route(mut events: Events, cancellation: oneshot::Sender<()>) {
    loop {
        let event = events.recv().await.expect("bounded host event");

        match (event.plugin, event.kind) {
            (PluginId(7), 0) => record("unrelated-routed"),
            (PluginId(6), CANCEL_EVENT) => {
                assert!(event.payload.is_empty());
                record("cancel-routed");
                cancellation.send(()).expect("quiet operation still owned");
                return;
            }
            _ => panic!("unrouted application event"),
        }
    }
}

/// Pumps only runnable local futures when the host explicitly enters the guest.
#[unsafe(no_mangle)]
pub extern "C" fn leptoskit_poll() {
    POOL.with_borrow_mut(LocalPool::run_until_stalled);
    publish();
}

/// Negotiates the renderer independently from plugin protocol version one.
#[unsafe(no_mangle)]
pub extern "C" fn leptoskit_abi_version() -> u32 {
    1
}

/// Starts a local client journey and the application's independent event router.
#[unsafe(no_mangle)]
pub extern "C" fn leptoskit_main() {
    // Safety: this creates an owned text node through the pinned renderer ABI.
    unsafe { node_create(1, 1) };
    assert!(
        matches!(solana_kit_leptoskit::GuestHttp::new("missing"), Err(error) if error.kind == solana_kit_client::TransportFailureKind::Unavailable)
    );
    record("unavailable");
    record("started");
    let events = Events::take().expect("application owns singleton receiver");
    assert!(matches!(Events::take(), Err(Error::Busy)));
    record("singleton-owned");
    let (sender, receiver) = oneshot::channel();

    let spawner = POOL.with_borrow(LocalPool::spawner);
    spawner
        .spawn_local(route(events, sender))
        .expect("local router");
    spawner
        .spawn_local(crate::journey::run(receiver))
        .expect("local journey");
    leptoskit_poll();
}

/// This application has no renderer input handlers.
#[unsafe(no_mangle)]
pub extern "C" fn leptoskit_event(_: u64, _: i32, _: u32, _: u32) -> i32 {
    0
}
