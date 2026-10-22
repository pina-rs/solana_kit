//! Executes the official System constructor in actual pinned LeptosKit WasmHost.
#![forbid(unsafe_code)]

use anyhow::{Context, Result, ensure};
use leptoskit_wasm::{EventKind, WasmHost};

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 480.0;
const FUEL_BUDGET: u64 = 1_000_000_000;
const SUCCESS: &str = "solana-kit-system-create-account-v1: Ok(())";
const CORRUPTION: &str = concat!(
    "solana-kit-system-create-account-v1: ",
    "Err(\"CreateAccount serialized bytes mismatch\")"
);

/// Requires the real committed report instead of accepting host startup alone.
fn validate(text: Option<&str>, expected: &str) -> Result<()> {
    let text = text.context("missing report text")?;
    ensure!(
        text == expected,
        "unexpected System constructor report: {text}"
    );

    println!("{text}");
    Ok(())
}

/// Starts the canonical guest and reexecutes its corrupt oracle by a host event.
fn main() -> Result<()> {
    let path = std::env::args_os()
        .nth(1)
        .context("usage: host <System guest.wasm>")?;
    let wasm = std::fs::read(path).context("read Rust guest")?;
    let mut host = WasmHost::headless(&wasm).context("instantiate actual LeptosKit host")?;
    host.set_fuel_budget(FUEL_BUDGET);
    host.start(WIDTH, HEIGHT).context("start System guest")?;
    ensure!(
        host.commits() == 1,
        "canonical guest did not commit exactly once"
    );
    let root = host.surface().root().context("missing report node")?;
    validate(host.surface().props(root).text.as_deref(), SUCCESS)?;

    host.dispatch_wire(1, EventKind::Press, None)
        .context("dispatch corruption event")?;
    ensure!(
        host.commits() == 2,
        "corruption event did not commit exactly once"
    );
    validate(host.surface().props(root).text.as_deref(), CORRUPTION)
}
