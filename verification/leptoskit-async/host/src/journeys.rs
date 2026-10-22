//! Observable actual-host journeys, including controls which must fail the gate.

use crate::application::Application;
use anyhow::{Context, Result, ensure};
use leptoskit_bridge::{Bytes, Error, MethodId, PluginId};

const TRACE: &[&str] = &[
    "unavailable",
    "denied",
    "singleton-owned",
    "entropy-pending",
    "entropy-complete",
    "short-pending",
    "short-rejected",
    "unavailable-pending",
    "unavailable-reply",
    "timer-pending",
    "timer-complete",
    "quiet-pending",
    "unrelated-routed",
    "cancel-routed",
    "quiet-cancelled",
    "unload-pending",
];
const ENTROPY: &[u8] = &[9, 8, 7, 6];
const DEADLINE: u64 = 10;
const CANCEL_EVENT: u32 = 7;

/// Controls remove precisely one required integration step from the real run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Mode {
    Canonical,
    OmitBridge,
    OmitWake,
    OmitCancel,
    CorruptEntropy,
}

/// Compares the complete guest trace, so missing or extra work cannot pass.
fn require(app: &Application, last: &str) -> Result<()> {
    let root = app
        .host
        .surface()
        .root()
        .context("missing guest report node")?;
    let text = app
        .host
        .surface()
        .props(root)
        .text
        .as_deref()
        .context("missing guest report")?;
    let actual: Vec<String> = serde_json::from_str(text)?;
    let end = TRACE
        .iter()
        .position(|event| *event == last)
        .context("unknown trace marker")?;
    ensure!(
        actual == TRACE[..=end],
        "expected {last}; actual guest trace: {actual:?}"
    );
    Ok(())
}

/// Completes entropy on later turns, advances injected time, then cancels quiet work.
fn entropy(app: &mut Application, mode: Mode) -> Result<()> {
    require(app, "entropy-pending")?;
    let entropy = app.request(0, &[4])?;
    let bytes = if mode == Mode::CorruptEntropy {
        &[0; 4]
    } else {
        ENTROPY
    };

    app.complete(
        entropy,
        Ok(Bytes::copy_from_slice(bytes)),
        mode != Mode::OmitWake,
    )?;

    if mode == Mode::OmitWake {
        require(app, "entropy-complete")?;
    }

    require(app, "short-pending")?;
    let short = app.request(0, &[4])?;
    app.complete(short, Ok(Bytes::from_static(&[9, 8])), true)?;
    require(app, "unavailable-pending")?;
    let unavailable = app.request(0, &[4])?;
    app.complete(unavailable, Err(Error::Unavailable), true)?;
    Ok(())
}

/// Completes entropy on later turns, advances injected time, then cancels quiet work.
pub(super) fn run(wasm: &[u8], mode: Mode) -> Result<()> {
    let mut app = Application::start(wasm, 1, mode != Mode::OmitBridge)?;
    entropy(&mut app, mode)?;
    require(&app, "timer-pending")?;
    app.advance(DEADLINE - 1)?;
    require(&app, "timer-pending")?;
    ensure!(
        app.advance(DEADLINE - 2).is_err(),
        "backward clock accepted"
    );
    app.advance(DEADLINE)?;
    require(&app, "quiet-pending")?;

    cancellation(&mut app, mode)?;
    let live = app.live.clone();
    drop(app);
    ensure!(live.get() == 0, "unload retained native work");
    println!("canonical: {TRACE:?}; cancelled/late/unload cleanup observed");
    Ok(())
}

/// A single event pump wakes quiet work; cancellation accounting precedes drain.
fn cancellation(app: &mut Application, mode: Mode) -> Result<()> {
    let quiet = app.request(2, &[])?;
    app.event(1, 0)?;

    if mode != Mode::OmitCancel {
        app.event(0, CANCEL_EVENT)?;
        let overflow = app
            .event(0, CANCEL_EVENT)
            .expect_err("full event queue must fail");
        ensure!(
            overflow.downcast_ref::<Error>() == Some(&Error::Busy),
            "wrong event overflow error"
        );
    }

    app.host.wake_plugins().context("wake actual guest")?;
    require(app, "unload-pending")?;
    ensure!(
        app.live.get() == 1,
        "native work disappeared before cancellation drain"
    );
    ensure!(
        app.host.plugins().check_call(PluginId(0), MethodId(0), 0) == Err(Error::Busy),
        "running cancellation did not retain its slot"
    );
    ensure!(
        app.host.plugins().complete(quiet.id, Ok(Bytes::new())) == Err(Error::InvalidCall),
        "late completion before cancellation drain was accepted"
    );
    app.drain()?;

    ensure!(
        app.cancellations == 1 && app.live.get() == 1,
        "native cancellation not dropped"
    );
    ensure!(
        app.host.plugins().complete(quiet.id, Ok(Bytes::new())) == Err(Error::InvalidCall),
        "late cancelled completion was accepted"
    );
    app.wake()?;
    require(app, "unload-pending")?;
    ensure!(
        app.host.plugins().buffered_bytes() == 0,
        "bridge retained bytes"
    );
    ensure!(
        app.host.plugins().next_cancelled().is_none(),
        "cancellation registration retained"
    );
    Ok(())
}

/// Exercises an unloaded pending request against a replacement with the same ID.
pub(super) fn replacement(wasm: &[u8]) -> Result<()> {
    let old = Application::start(wasm, 1, true)?;
    let old_token = old.request(0, &[4])?;
    let live = old.live.clone();
    drop(old);
    ensure!(live.get() == 0, "unload did not drop fixture resource");

    let mut new = Application::start(wasm, 2, true)?;
    let current = new.request(0, &[4])?;
    ensure!(old_token.id == current.id, "control did not reuse call ID");
    ensure!(
        !new.complete(old_token, Ok(Bytes::from_static(&[0; 4])), true)?,
        "old callback delivered"
    );
    require(&new, "entropy-pending")?;

    ensure!(new.live.get() == 1, "stale callback removed current work");
    new.complete(current, Ok(Bytes::copy_from_slice(ENTROPY)), true)?;
    require(&new, "short-pending")?;

    let mut unsafe_routing = Application::start(wasm, 2, true)?;
    unsafe_routing.request(0, &[4])?;
    unsafe_routing
        .host
        .plugins()
        .complete(old_token.id, Ok(Bytes::from_static(&[0; 4])))?;
    let failure = unsafe_routing
        .wake()
        .expect_err("numeric-only stale reply must corrupt guest");
    ensure!(
        failure.to_string().contains("wake actual guest"),
        "wrong replacement control error"
    );
    println!("replacement: reused ID rejected by generation; numeric-only stale reply traps guest");
    Ok(())
}
