//! Executes the complete Token-2022 guest in the actual LeptosKit interpreter.
use anyhow::{Context, Result, ensure};
use leptoskit_wasm::{WasmError, WasmHost};

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 480.0;
const TOKEN_FUEL: u64 = 8_505_000_000;

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
    host.set_fuel_budget(TOKEN_FUEL);
    host.start(WIDTH, HEIGHT)?;
    ensure!(host.commits() == 1, "guest did not commit exactly once");
    let root = host.surface().root().context("missing report node")?;
    ensure!(
        host.surface().props(root).text.as_deref() == Some("token-2022-public: 119/119"),
        "incomplete Token-2022 report"
    );
    println!("token-2022-public: 119/119; actual wasmi; one-fuel rejection");
    Ok(())
}
