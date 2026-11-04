//! Checks the portable package closure on selected WebAssembly targets.

use super::super::{Config, model, success};
use super::fixture::source;
use anyhow::Result;
use std::{fs, path::Path, process::Command};

/// Checks Core, Client, Signers and Wallet on each configured target.
pub(super) fn run(config: &Config, plan: &model::Plan, target: &str) -> Result<()> {
    let directory = config.evidence.join("wasm-consumer");
    fs::create_dir_all(directory.join("src"))?;
    fs::create_dir_all(directory.join(".cargo"))?;
    fs::write(directory.join("Cargo.toml"), manifest(plan)?)?;
    fs::write(
        directory.join("src/lib.rs"),
        "//! Portable package closure compiled on a WebAssembly target.\n#![forbid(unsafe_code)]\n\nuse solana_kit_core::Address;\n\npub fn address() -> Address {\n    Address::from_bytes([1; 32])\n}\n",
    )?;
    fs::write(
        directory.join(".cargo/config.toml"),
        source(&config.registry)?,
    )?;
    success(
        config,
        cargo(&directory).args(["generate-lockfile", "--offline"]),
        &format!("wasm-{target}-lock"),
    )?;
    let mut command = cargo(&directory);
    command.args(["check", "--locked", "--offline", "--target", target]);
    command.env(
        "CARGO_TARGET_DIR",
        config.target.join(format!("consumer-wasm-{target}")),
    );
    success(config, &mut command, &format!("wasm-{target}"))?;

    Ok(())
}

/// Builds the portable exact-version dependency manifest.
fn manifest(plan: &model::Plan) -> Result<String> {
    let mut manifest = String::from(
        "[package]\nname = \"solana-kit-wasm-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[workspace]\n\n[dependencies]\n",
    );

    for name in [
        "solana_kit_core",
        "solana_kit_client",
        "solana_kit_signers",
        "solana_kit_wallet",
    ] {
        let package = plan.require(name)?;
        manifest.push_str(&format!(
            "{name} = {{ version = \"={}\" }}\n",
            package.version
        ));
    }

    Ok(manifest)
}

/// Uses the selected Cargo binary without forwarding any caller flags.
fn cargo(directory: &Path) -> Command {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(directory);
    command
}
