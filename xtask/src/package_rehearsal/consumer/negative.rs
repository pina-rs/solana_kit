//! Builds independent missing-archive and corrupted-checksum consumer controls.

use super::super::{Config, command, registry};
use super::fixture::source;
use anyhow::{Context, Result, ensure};
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Serialize)]
pub(crate) struct Negative {
    pub package: String,
    pub status: String,
    pub matched: String,
    pub destination: PathBuf,
}

/// Copies a consumer and Cargo cache into an independent negative namespace.
pub(super) fn run(
    config: &Config,
    registry: &registry::RegistryEvidence,
    directory: &Path,
    package: &str,
    expected: &[&str],
    cargo_home_label: &str,
    label: &str,
) -> Result<Negative> {
    let registry = if package == "solana_kit_core" {
        &registry.negative_missing
    } else {
        &registry.negative_corrupt
    };
    let home = config.evidence.join(cargo_home_label);
    fs::create_dir(&home)?;
    link_tree(
        &config.cargo_home.join("registry/index"),
        &home.join("registry/index"),
    )?;
    link_tree(
        &config.cargo_home.join("registry/cache"),
        &home.join("registry/cache"),
    )?;
    copy_tree(&config.consumer, directory)?;
    fs::write(directory.join(".cargo/config.toml"), source(registry)?)?;
    let output = build(config, directory, &home, label)?;
    ensure!(!output.status.success(), "{label} unexpectedly compiled");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let matched = expected
        .iter()
        .find(|fragment| stderr.contains(*fragment))
        .with_context(|| format!("{label} failed without the intended diagnostic"))?;
    ensure!(
        stderr.contains(package),
        "{label} diagnostic does not name {package}"
    );

    Ok(Negative {
        package: package.to_owned(),
        status: output.status.to_string(),
        matched: (*matched).to_owned(),
        destination: directory.to_owned(),
    })
}

/// Executes the control with its fresh Cargo home and target directory.
fn build(
    config: &Config,
    directory: &Path,
    home: &Path,
    label: &str,
) -> Result<std::process::Output> {
    let mut command = cargo(directory);
    command.args(["test", "--locked", "--offline"]);
    command.env("CARGO_HOME", home);
    command.env(
        "CARGO_TARGET_DIR",
        config.target.join(format!("{label}-target")),
    );
    command::with_home(config, &mut command, home, &format!("{label}-build"))
}

/// Recursively hardlinks an immutable Cargo cache, falling back to a copy.
fn link_tree(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            link_tree(&entry.path(), &target)?;
        } else if fs::hard_link(entry.path(), &target).is_err() {
            fs::copy(entry.path(), &target)?;
        }
    }

    Ok(())
}

/// Copies only the consumer fixture files into a negative namespace.
fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let name = entry.file_name();

        if name == "target" {
            continue;
        }

        let target = destination.join(&name);

        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }

    Ok(())
}

/// Uses the selected Cargo binary without forwarding any caller flags.
fn cargo(directory: &Path) -> Command {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(directory);
    command
}
