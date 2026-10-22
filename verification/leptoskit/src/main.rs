//! Executes a real Rust guest in the pinned upstream LeptosKit headless host.
#![forbid(unsafe_code)]

use anyhow::{Context, Result, ensure};
use leptoskit_wasm::{EventKind, WasmError, WasmHost};
use serde_json::Value;
use solana_kit_conformance::{EXPECTED_CASE_COUNT, FixtureMode, SUITE_ID, run};

const VIEWPORT_WIDTH: f32 = 320.0;
const VIEWPORT_HEIGHT: f32 = 480.0;
// This is a pooled allowance for the whole entry, including lazy translation.
// The measured 77-case baseline needs 995m fuel; 15m per case retains headroom.
const FUEL_ALLOWANCE_PER_CASE: u64 = 15_000_000;

/// Requires the real interpreter to reject an insufficient finite allowance.
fn reject_insufficient_fuel(wasm: &[u8]) -> Result<()> {
    let mut host = WasmHost::headless(wasm)?;
    host.set_fuel_budget(1);
    ensure!(
        matches!(
            host.start(VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
            Err(WasmError::OutOfFuel)
        ),
        "insufficient fuel did not trap"
    );
    ensure!(host.commits() == 0, "exhausted guest committed a report");
    println!("actual LeptosKit WasmHost: insufficient fuel trapped before commit");
    Ok(())
}

/// Checks the full guest report against the same suite's native report.
fn validate(text: &str, mode: FixtureMode) -> Result<()> {
    let actual: Value = serde_json::from_str(text).context("decode guest report")?;
    let expected = run(mode);
    let cases = actual["cases"].as_array().context("missing guest cases")?;

    ensure!(actual["suite_id"] == SUITE_ID, "unexpected guest suite");
    ensure!(
        cases.len() == EXPECTED_CASE_COUNT,
        "incomplete guest report"
    );
    ensure!(
        actual["passed"] == expected.passed_count(),
        "incorrect pass count"
    );
    ensure!(
        actual["success"] == expected.is_success(),
        "incorrect success state"
    );

    for (actual, expected) in cases.iter().zip(expected.cases) {
        ensure!(actual["name"] == expected.name, "unexpected case name");
        ensure!(
            actual["error"] == serde_json::to_value(expected.outcome.err())?,
            "wrong outcome"
        );
    }

    println!("{text}");
    Ok(())
}

/// Loads the required artifact, starts the actual host, then dispatches corruption.
fn main() -> Result<()> {
    let path = std::env::args_os()
        .nth(1)
        .context("usage: host <Rust guest.wasm>")?;
    let wasm = std::fs::read(path).context("read compiled Rust guest")?;
    let fuel_budget = FUEL_ALLOWANCE_PER_CASE
        .checked_mul(u64::try_from(EXPECTED_CASE_COUNT)?)
        .context("suite fuel allowance overflow")?;

    reject_insufficient_fuel(&wasm)?;
    let mut host = WasmHost::headless(&wasm).context("instantiate actual LeptosKit WasmHost")?;

    host.set_fuel_budget(fuel_budget);
    host.start(VIEWPORT_WIDTH, VIEWPORT_HEIGHT)
        .context("start canonical guest")?;
    ensure!(
        host.commits() == 1,
        "canonical guest did not commit exactly once"
    );

    let root = host.surface().root().context("missing report node")?;
    let text = host
        .surface()
        .props(root)
        .text
        .as_deref()
        .context("missing canonical report")?;
    validate(text, FixtureMode::Canonical)?;

    host.dispatch_wire(1, EventKind::Press, None)
        .context("dispatch corruption event")?;
    ensure!(host.commits() == 2, "corruption event did not commit");

    let text = host
        .surface()
        .props(root)
        .text
        .as_deref()
        .context("missing corrupted report")?;
    validate(text, FixtureMode::CorruptHash)?;
    println!(
        "actual LeptosKit WasmHost: canonical {EXPECTED_CASE_COUNT}/{EXPECTED_CASE_COUNT}; corruption {}/{EXPECTED_CASE_COUNT} observed",
        EXPECTED_CASE_COUNT - 1
    );
    Ok(())
}
