//! Materializes the positive and negative local registries from reviewed archives.

use super::{Config, model};
use crate::registry_fixture;
use anyhow::{Context, Result, ensure};
use serde::Serialize;
use std::{fs, path::Path};

#[derive(Serialize)]
pub(super) struct RegistryEvidence {
    pub destination: std::path::PathBuf,
    pub packages: usize,
    pub negative_missing: std::path::PathBuf,
    pub negative_corrupt: std::path::PathBuf,
}

/// Builds one complete registry and two fresh negative namespaces.
pub(super) fn run(
    config: &Config,
    plan: &model::Plan,
    archives: &[super::archives::ReviewedArchive],
    lockfile: &Path,
) -> Result<RegistryEvidence> {
    let packages = install(
        config,
        plan,
        archives,
        lockfile,
        &config.registry,
        None,
        None,
    )?;
    let missing = config.evidence.join("registry-negative-missing");
    let corrupt = config.evidence.join("registry-negative-corrupt");
    install(
        config,
        plan,
        archives,
        lockfile,
        &missing,
        Some("solana_kit_core"),
        None,
    )?;
    install(
        config,
        plan,
        archives,
        lockfile,
        &corrupt,
        None,
        Some("solana_kit_client"),
    )?;

    Ok(RegistryEvidence {
        destination: config.registry.clone(),
        packages,
        negative_missing: missing,
        negative_corrupt: corrupt,
    })
}

/// Writes one explicit fixture configuration and executes the offline installer.
fn install(
    config: &Config,
    plan: &model::Plan,
    archives: &[super::archives::ReviewedArchive],
    lockfile: &Path,
    destination: &Path,
    omit: Option<&str>,
    corrupt: Option<&str>,
) -> Result<usize> {
    let fixture = write_config(config, plan, archives, lockfile, destination, omit, corrupt)?;
    registry_fixture::run(&fixture)?;
    verify(destination, archives)
}

/// Retains the exact fixture input as evidence before execution.
fn write_config(
    config: &Config,
    plan: &model::Plan,
    archives: &[super::archives::ReviewedArchive],
    lockfile: &Path,
    destination: &Path,
    omit: Option<&str>,
    corrupt: Option<&str>,
) -> Result<std::path::PathBuf> {
    let core = plan.require("solana_kit_core")?;
    let core_archive = archive(archives, core)?;
    let label = destination
        .file_name()
        .context("registry destination has no name")?
        .to_string_lossy()
        .into_owned();
    let fixture = config.evidence.join(format!("{label}.json"));
    fs::write(
        &fixture,
        serde_json::to_vec_pretty(&serde_json::json!({
            "archive": core_archive.archive,
            "lockfile": lockfile,
            "metadata": config.evidence.join("metadata.json"),
            "cache": config.cache,
            "sparse_index": config.sparse_index,
            "destination": destination,
            "package": core.name,
            "version": core.version,
            "omit_archive": omit,
            "corrupt_archive": corrupt,
            "additions": additions(archives, &core.name),
        }))?,
    )?;

    Ok(fixture)
}

/// Finds the reviewed Core archive used as the fixture's primary record.
fn archive<'a>(
    archives: &'a [super::archives::ReviewedArchive],
    core: &model::Target,
) -> Result<&'a super::archives::ReviewedArchive> {
    archives
        .iter()
        .find(|archive| archive.name == core.name)
        .context("missing Core archive")
}

/// Selects every other reviewed archive with its exact identity.
fn additions(archives: &[super::archives::ReviewedArchive], core: &str) -> Vec<serde_json::Value> {
    archives
        .iter()
        .filter(|archive| archive.name != core)
        .map(|archive| {
            serde_json::json!({
                "archive": archive.archive,
                "package": archive.name,
                "version": archive.version,
            })
        })
        .collect()
}

/// Requires every selected archive to be present in the installed registry.
fn verify(destination: &Path, archives: &[super::archives::ReviewedArchive]) -> Result<usize> {
    let inputs: Vec<serde_json::Value> =
        serde_json::from_slice(&fs::read(destination.join("inputs.json"))?)?;
    ensure!(!inputs.is_empty(), "registry fixture installed no packages");

    for archive in archives {
        ensure!(
            inputs.iter().any(|entry| {
                entry.get("name").and_then(serde_json::Value::as_str) == Some(archive.name.as_str())
                    && entry.get("version").and_then(serde_json::Value::as_str)
                        == Some(archive.version.as_str())
            }),
            "registry lacks selected SDK archive {} {}",
            archive.name,
            archive.version
        );
    }

    Ok(inputs.len())
}
