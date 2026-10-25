//! Runs generated client contracts through the production LeptosKit interpreter.
#![forbid(unsafe_code)]

use leptoskit_wasm::{WasmError, WasmHost};

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 480.0;
const FUEL_BUDGET: u64 = 250_000_000;
const EXPECTED: &str = "generated-client-contracts:8";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .ok_or("usage: packed_counter_host <guest.wasm>")?;
    let wasm = std::fs::read(path)?;
    reject_insufficient_fuel(&wasm)?;
    let mut host = WasmHost::headless(&wasm)?;
    host.set_fuel_budget(FUEL_BUDGET);
    host.start(WIDTH, HEIGHT)?;
    assert_eq!(host.commits(), 1);
    let root = host.surface().root().ok_or("guest omitted report node")?;
    let props = host.surface().props(root);
    assert_eq!(props.text.as_deref(), Some(EXPECTED));
    println!("actual LeptosKit WasmHost: {EXPECTED}; one-fuel trap before commit");

    Ok(())
}

/// The guest must execute its assertions before committing any successful report.
fn reject_insufficient_fuel(wasm: &[u8]) -> Result<(), WasmError> {
    let mut host = WasmHost::headless(wasm)?;
    host.set_fuel_budget(1);
    assert!(matches!(
        host.start(WIDTH, HEIGHT),
        Err(WasmError::OutOfFuel)
    ));
    assert_eq!(host.commits(), 0);

    Ok(())
}
