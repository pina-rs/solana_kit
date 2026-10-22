//! Runs actual guest journeys and controls through the pinned real WasmHost.
#![forbid(unsafe_code)]

mod application;
mod imports;
mod journeys;

use anyhow::{Context, Result, ensure};
use journeys::Mode;

/// A missing artifact is an error; every negative runs the same observable gates.
fn main() -> Result<()> {
    let path = std::env::args_os()
        .nth(1)
        .context("usage: host <guest.wasm>")?;
    let wasm = std::fs::read(path).context("read required guest artifact")?;
    imports::verify(&wasm)?;
    journeys::run(&wasm, Mode::Canonical)?;
    journeys::replacement(&wasm)?;

    for (mode, required) in [
        (Mode::OmitBridge, "start actual guest"),
        (Mode::OmitWake, "expected entropy-complete"),
        (Mode::OmitCancel, "expected unload-pending"),
        (Mode::CorruptEntropy, "wake actual guest"),
    ] {
        let error = journeys::run(&wasm, mode).expect_err("negative control must fail");
        ensure!(
            error.to_string().contains(required),
            "wrong negative error: {error:#}"
        );
        println!("negative {mode:?}: required failure: {error:#}");
    }

    println!("actual LeptosKit async guest: all canonical and negative journeys passed");
    Ok(())
}
