//! A clean external consumer tests the normalized packaged core without repo flags.

use crate::process::{cargo, success};
use anyhow::Result;
use std::{path::PathBuf, process::Command};

/// Packages the public core, extracts it outside the repository, and runs a consumer.
pub(crate) fn run() -> Result<()> {
    let root = std::env::current_dir()?;
    let directory: PathBuf =
        std::env::temp_dir().join(format!("solana-kit-consumer-{}", std::process::id()));
    std::fs::create_dir_all(directory.join("src"))?;
    success(
        cargo().args([
            "package",
            "--locked",
            "-p",
            "solana_kit_core",
            "--all-features",
        ]),
        "consumer-package",
    )?;
    success(
        Command::new("tar")
            .arg("-xzf")
            .arg(root.join("target/package/solana_kit_core-0.1.0.crate"))
            .arg("-C")
            .arg(&directory),
        "consumer-extract",
    )?;
    std::fs::write(
        directory.join("Cargo.toml"),
        include_str!("../consumer/Cargo.toml.txt"),
    )?;
    std::fs::write(
        directory.join("src/lib.rs"),
        include_str!("../consumer/lib.rs"),
    )?;
    std::fs::write(
        directory.join("src/codec_overrides.rs"),
        include_str!("../../crates/solana_kit_core/tests/codec_size/overrides.rs"),
    )?;
    success(
        cargo()
            .args(["test", "--manifest-path", "Cargo.toml"])
            .current_dir(&directory)
            .env_remove("RUSTFLAGS")
            .env_remove("CARGO_ENCODED_RUSTFLAGS")
            .env_remove("CARGO_BUILD_TARGET")
            .env_remove("RUSTC_WRAPPER")
            .env_remove("RUSTC_WORKSPACE_WRAPPER"),
        "consumer-test",
    )?;
    println!("clean consumer retained at {}", directory.display());
    Ok(())
}
