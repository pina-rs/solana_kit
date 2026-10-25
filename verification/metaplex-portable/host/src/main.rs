//! Executes the complete public client registry in the production LeptosKit WasmHost.
use anyhow::{Context, Result, ensure};
use leptoskit_wasm::{WasmError, WasmHost};
const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 480.0;
const FUEL: u64 = 25_000_000_000;

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
    host.set_fuel_budget(FUEL);

    if std::env::args().nth(2).as_deref() == Some("--expect-corrupt") {
        ensure!(host.start(WIDTH, HEIGHT).is_err(), "corrupt guest returned");
        ensure!(host.commits() == 0, "corrupt guest committed");
        println!("intentional contract corruption trapped before commit");

        return Ok(());
    }

    host.start(WIDTH, HEIGHT)?;
    ensure!(host.commits() == 1, "guest did not commit exactly once");
    let root = host.surface().root().context("missing report node")?;
    ensure!(
        host.surface().props(root).text.as_deref() == Some(include_str!("../../report.txt")),
        "incomplete report"
    );
    println!(
        "{}; actual wasmi; one-fuel rejection",
        include_str!("../../report.txt")
    );

    Ok(())
}
