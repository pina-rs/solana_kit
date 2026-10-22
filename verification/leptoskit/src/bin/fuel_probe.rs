//! Controlled fuel boundaries through the unchanged pinned WasmHost.
#![forbid(unsafe_code)]

use anyhow::{Context, Result, bail};
use leptoskit_wasm::{EventKind, WasmHost};

/// Executes one fresh host so a trapped or translated instance cannot bias a trial.
fn main() -> Result<()> {
    let arguments: Vec<_> = std::env::args().collect();
    let path = arguments
        .get(1)
        .context("usage: fuel_probe <guest> <fuel> [selector]")?;
    let budget: u64 = arguments.get(2).context("missing fuel")?.parse()?;
    let selector: Option<i64> = arguments.get(3).map(|value| value.parse()).transpose()?;
    let bytes = std::fs::read(path)?;
    let mut host = WasmHost::headless(&bytes)?;
    host.set_fuel_budget(budget);

    let result = host.start(320.0, 480.0).and_then(|()| {
        if let Some(selector) = selector {
            host.dispatch_wire(selector, EventKind::Press, None)?;
        }

        Ok(())
    });

    if let Err(error) = result {
        println!(
            "fuel={budget} selector={selector:?} commits={} error={error}",
            host.commits()
        );
        bail!(error);
    }

    let root = host.surface().root().context("missing report node")?;
    let text = host
        .surface()
        .props(root)
        .text
        .as_deref()
        .context("missing report")?;
    println!(
        "fuel={budget} selector={selector:?} commits={}",
        host.commits()
    );
    println!("{text}");
    Ok(())
}
