//! Builds an external consumer against the exact local-registry archives.

use super::{Config, model, success};
use anyhow::{Context, Result};
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

mod audit;
mod bytes;
mod fixture;
mod msrv;
mod negative;
mod wasm;

#[derive(Serialize)]
pub(super) struct ConsumerEvidence {
    pub destination: PathBuf,
    pub tests: usize,
    pub selected_tests: usize,
    pub audits: usize,
    pub msrv: Option<String>,
    pub wasm: Vec<String>,
    pub negative_missing: negative::Negative,
    pub negative_corrupt: negative::Negative,
}

/// Writes and executes the positive, negative, Wasm and MSRV consumers.
pub(super) fn run(
    config: &Config,
    plan: &model::Plan,
    archives: &[super::archives::ReviewedArchive],
    registry: &super::registry::RegistryEvidence,
) -> Result<ConsumerEvidence> {
    let directory = &config.consumer;
    fs::create_dir(directory)?;
    fixture::write(plan, directory, &registry.destination)?;
    lock(config, directory)?;
    let metadata = metadata(config, directory, "consumer-default-metadata")?;
    let audits = audit::run(config, plan, archives, &metadata, directory)?;
    let default = test(config, directory, &[], "consumer-default")?;
    let selected = test(
        config,
        directory,
        &["--features", "selected"],
        "consumer-selected",
    )?;
    let msrv = toolchain(config, directory)?;
    let wasm = wasm(config, plan)?;
    let negative_missing = negative(
        config,
        registry,
        "solana_kit_core",
        &[
            "no matching package",
            "failed to download",
            "failed to read",
            "No such file",
            "not found",
        ],
        "missing",
    )?;
    let negative_corrupt = negative(
        config,
        registry,
        "solana_kit_client",
        &["checksum", "failed to verify"],
        "corrupt",
    )?;

    Ok(ConsumerEvidence {
        destination: directory.clone(),
        tests: default,
        selected_tests: selected,
        audits,
        msrv,
        wasm,
        negative_missing,
        negative_corrupt,
    })
}

/// Generates the one external lockfile before metadata and tests reuse it.
fn lock(config: &Config, directory: &Path) -> Result<()> {
    let target = config.target.join("consumer-lock");
    let mut command = cargo(directory);
    command.args(["generate-lockfile", "--offline"]);
    command.env("CARGO_TARGET_DIR", &target);
    success(config, &mut command, "consumer-lockfile")?;
    Ok(())
}

/// Queries the exact resolved package identities through Cargo metadata.
fn metadata(config: &Config, directory: &Path, label: &str) -> Result<serde_json::Value> {
    let mut command = cargo(directory);
    command.args(["metadata", "--format-version", "1", "--locked", "--offline"]);
    command.env("CARGO_TARGET_DIR", config.target.join("consumer-metadata"));
    let output = success(config, &mut command, label)?;
    let path = config.evidence.join(format!("{label}.json"));
    fs::write(&path, &output.stdout)?;
    serde_json::from_slice(&output.stdout).context("parse consumer metadata")
}

/// Runs one complete test configuration and returns its executed test count.
fn test(config: &Config, directory: &Path, arguments: &[&str], label: &str) -> Result<usize> {
    let mut command = cargo(directory);
    command.args(["test", "--locked", "--offline"]);
    command.args(arguments);
    command.env("CARGO_TARGET_DIR", config.target.join(label));
    let output = success(config, &mut command, label)?;
    Ok(passed(&output.stdout))
}

/// Runs the declared MSRV when the configuration supplies one.
fn toolchain(config: &Config, directory: &Path) -> Result<Option<String>> {
    let Some(toolchain) = &config.msrv else {
        return Ok(None);
    };

    msrv::run(config, directory, toolchain)?;
    Ok(Some(toolchain.clone()))
}

/// Runs every configured Wasm target and returns the completed target names.
fn wasm(config: &Config, plan: &model::Plan) -> Result<Vec<String>> {
    let mut completed = Vec::new();

    for target in &config.wasm_targets {
        wasm::run(config, plan, target)?;
        completed.push(target.clone());
    }

    Ok(completed)
}

/// Runs one fresh negative registry with its own cache and target namespace.
fn negative(
    config: &Config,
    registry: &super::registry::RegistryEvidence,
    package: &str,
    expected: &[&str],
    suffix: &str,
) -> Result<negative::Negative> {
    negative::run(
        config,
        registry,
        &config.evidence.join(format!("consumer-negative-{suffix}")),
        package,
        expected,
        &format!("consumer-negative-{suffix}-home"),
        &format!("consumer-negative-{suffix}"),
    )
}

/// Sums Cargo's executed `test result` rows without counting ignored tests.
fn passed(stdout: &[u8]) -> usize {
    String::from_utf8_lossy(stdout)
        .lines()
        .filter_map(|line| line.strip_prefix("test result: ok. "))
        .filter_map(|line| line.split_once(" passed").map(|(count, _)| count))
        .filter_map(|count| count.parse::<usize>().ok())
        .sum()
}

/// Uses the selected Cargo binary without forwarding any caller flags.
fn cargo(directory: &Path) -> Command {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(directory);
    command
}

#[cfg(test)]
mod tests {
    use super::passed;

    #[test]
    fn sums_only_executed_test_rows() {
        let stdout = b"test result: ok. 7 passed; 0 failed; 1 ignored\nrunning 0 tests\ntest result: ok. 0 passed; 0 failed;\n";
        assert_eq!(passed(stdout), 7);
    }
}
