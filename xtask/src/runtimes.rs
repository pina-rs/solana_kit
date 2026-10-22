//! Real native, Chrome, WASI, and upstream LeptosKit executions.

use crate::process::{capture, cargo, success};
use anyhow::{Context, Result, ensure};
use std::process::Command;

/// Requires the exact shared-suite output, including every named case.
fn suite(command: &mut Command, label: &str, corrupt: bool) -> Result<()> {
    let expected = if corrupt {
        include_str!("../../verification/conformance/evidence/native-corruption.txt")
    } else {
        include_str!("../../verification/conformance/evidence/native-success.txt")
    };

    let output = capture(command, label)?;
    let expected_code = i32::from(corrupt);

    ensure!(
        output.status.code() == Some(expected_code),
        "wrong {label} exit status"
    );
    ensure!(
        String::from_utf8(output.stdout)?.replace("\r\n", "\n") == expected,
        "wrong {label} report"
    );
    Ok(())
}

/// Executes both CLI modes before any Wasm execution is claimed.
pub(crate) fn native() -> Result<()> {
    suite(
        cargo().args(["run", "--locked", "--quiet", "-p", "solana_kit_conformance"]),
        "native-success",
        false,
    )?;
    suite(
        cargo().args([
            "run",
            "--locked",
            "--quiet",
            "-p",
            "solana_kit_conformance",
            "--",
            "--corrupt-fixture",
        ]),
        "native-corruption",
        true,
    )
}

/// Runs two actual browser tests; the second fails if corruption is unobserved.
pub(crate) fn browser() -> Result<()> {
    let chrome = std::env::var_os("CHROME").context("set CHROME to the Chrome binary")?;
    let driver =
        std::env::var_os("CHROMEDRIVER").context("set CHROMEDRIVER to the matching driver")?;
    success(Command::new(chrome).arg("--version"), "chrome-version")?;
    success(
        Command::new(driver).arg("--version"),
        "chromedriver-version",
    )?;
    success(
        cargo()
            .args([
                "test",
                "--locked",
                "-p",
                "solana_kit_browser_conformance",
                "--target",
                "wasm32-unknown-unknown",
                "--",
                "--nocapture",
            ])
            .env("WASM_BINDGEN_USE_BROWSER", "1"),
        "browser",
    )?;
    Ok(())
}

/// Executes both Rust WASI command artifacts in exactly Wasmtime 50.0.0.
pub(crate) fn wasi() -> Result<()> {
    let host = std::env::var_os("WASMTIME").context("set WASMTIME to Wasmtime 50.0.0")?;
    let version = success(Command::new(&host).arg("--version"), "wasmtime-version")?;
    ensure!(
        String::from_utf8(version.stdout)?.starts_with("wasmtime 50.0.0 "),
        "wrong Wasmtime version"
    );
    success(
        cargo().args([
            "build",
            "--locked",
            "-p",
            "solana_kit_conformance",
            "--target",
            "wasm32-wasip1",
            "--target",
            "wasm32-wasip2",
        ]),
        "wasi-build",
    )?;

    for target in ["wasm32-wasip1", "wasm32-wasip2"] {
        let artifact = format!("target/{target}/debug/solana_kit_conformance.wasm");
        suite(
            Command::new(&host).args(["run", &artifact]),
            &format!("{target}-success"),
            false,
        )?;
        suite(
            Command::new(&host).args(["run", &artifact, "--corrupt-fixture"]),
            &format!("{target}-corruption"),
            true,
        )?;
    }

    Ok(())
}

/// Builds a real Rust guest and runs the immutable upstream host workspace.
pub(crate) fn leptoskit() -> Result<()> {
    success(
        cargo().args([
            "build",
            "--locked",
            "--release",
            "-p",
            "solana_kit_conformance_guest",
            "--target",
            "wasm32-unknown-unknown",
        ]),
        "guest-build",
    )?;
    crate::gates::guest()?;
    success(
        cargo().args([
            "run",
            "--locked",
            "--manifest-path",
            "verification/leptoskit/Cargo.toml",
            "--",
            "target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm",
        ]),
        "leptoskit",
    )?;
    Ok(())
}
