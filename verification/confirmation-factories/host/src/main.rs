//! Runs the actual pinned production WasmHost and its configured interpreter.
use leptoskit_wasm::{WasmError, WasmHost};
const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 480.0;
const MAX_FUEL: u64 = 1_000_000_000;
const EXPECTED: &str = "confirmation-factory-contracts 11/11";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os().nth(1).ok_or("expected guest path")?;
    let bytes = std::fs::read(path)?;
    let mut exhausted = WasmHost::headless(&bytes)?;
    exhausted.set_fuel_budget(1);
    assert!(matches!(
        exhausted.start(WIDTH, HEIGHT),
        Err(WasmError::OutOfFuel)
    ));
    assert_eq!(exhausted.commits(), 0);

    let mut host = WasmHost::headless(&bytes)?;
    host.set_fuel_budget(MAX_FUEL);
    host.start(WIDTH, HEIGHT)?;
    assert_eq!(host.commits(), 1);
    let root = host.surface().root().ok_or("missing report root")?;
    assert_eq!(host.surface().props(root).text.as_deref(), Some(EXPECTED));
    println!(
        "actual pinned WasmHost confirmation-factory-contracts 11/11; one-fuel trap before commit"
    );
    Ok(())
}
