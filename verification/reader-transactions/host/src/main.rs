//! Executes the complete Reader control guest in the actual LeptosKit interpreter.
use anyhow::{Context, Result, ensure};
use leptoskit_wasm::{WasmError, WasmHost};

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 480.0;
const READER_FUEL: u64 = 100_000_000;

fn main() -> Result<()> {
    let path = std::env::args_os().nth(1).context("guest Wasm required")?;
    let bytes = std::fs::read(path)?;
    let mut exhausted = WasmHost::headless(&bytes)?;
    exhausted.set_fuel_budget(1);
    ensure!(
        matches!(exhausted.start(WIDTH, HEIGHT), Err(WasmError::OutOfFuel)),
        "one fuel must trap"
    );
    ensure!(exhausted.commits() == 0, "exhausted guest committed");

    let mut host = WasmHost::headless(&bytes)?;
    host.set_fuel_budget(READER_FUEL);
    host.start(WIDTH, HEIGHT)?;
    ensure!(host.commits() == 1, "guest did not commit exactly once");
    let root = host.surface().root().context("missing report node")?;
    ensure!(
        host.surface().props(root).text.as_deref() == Some("reader-transactions: 5/5"),
        "incomplete Reader report"
    );
    println!("reader-transactions: 5/5; actual wasmi; one-fuel rejection");
    Ok(())
}
