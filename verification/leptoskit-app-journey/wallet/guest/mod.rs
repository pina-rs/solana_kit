//! Installed by prepare.rs into a workspace made by the public generator.
mod flow;
mod trace;

use futures::{executor::LocalPool, task::LocalSpawnExt};
use leptoskit::native::Element;
use solana_kit_leptoskit::wallet_ui::{local_wallet_channel, local_wallet_panel};
use std::cell::RefCell;

type AppError = Box<dyn std::error::Error>;

thread_local! {
    static POOL: RefCell<LocalPool> = RefCell::new(LocalPool::new());
    static VIEW: RefCell<Option<Element>> = const { RefCell::new(None) };
    static FAILURE: RefCell<Option<AppError>> = const { RefCell::new(None) };
}

/// Retains the actual failure for the app's observer; no peer data is logged.
fn failed(error: impl Into<AppError>) {
    FAILURE.with_borrow_mut(|failure| *failure = Some(error.into()));
}

fn show(element: Element, status: &str) {
    let element = element.child(Element::text(status).id("wallet_fixture.status"));
    let element = Element::scroll_view()
        .id("wallet_fixture.scroll")
        .aria_label("Solana wallet application")
        .class("w-full h-full")
        .child(element);
    VIEW.with_borrow_mut(|view| *view = Some(element));
}

/// Uses the generated app's public renderer and its existing ABI exports.
pub fn main() {
    let (commands, actions) = local_wallet_channel();
    show(
        local_wallet_panel(None, false, false, &commands),
        "Choose a wallet",
    );
    leptoskit::guest::mount(|| VIEW.with_borrow(|view| view.clone().expect("wallet view mounted")));
    let events = leptoskit_bridge::guest::Events::take().expect("one application event owner");
    POOL.with_borrow(LocalPool::spawner)
        .spawn_local(flow::run(events, actions, commands))
        .expect("local wallet task");
    poll();
}

/// Runs local work after real native/plugin wake or a dispatched UI action.
pub fn poll() {
    POOL.with_borrow_mut(LocalPool::run_until_stalled);
    trace::poll();
    leptoskit::guest::render();
}

/// Records only the node and handled state of an actual native press.
pub fn event(node: i64, kind: i32, result: i32) {
    if kind == 0 {
        trace::press(node, result);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn leptoskit_poll() {
    poll();
}
