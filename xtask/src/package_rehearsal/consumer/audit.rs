//! Confirms Cargo selected registry archives and their exact reviewed bytes.

use super::fixture::DEPENDENCIES;
use super::{
    super::{Config, archives, model},
    bytes,
};
use anyhow::{Context, Result, ensure};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Serialize)]
struct Audit {
    package: String,
    version: String,
    package_id: String,
    source: String,
    manifest: PathBuf,
    checksum: String,
    members: Option<usize>,
}

/// Audits every resolved SDK package in the consumer metadata.
pub(super) fn run(
    config: &Config,
    plan: &model::Plan,
    archives: &[archives::ReviewedArchive],
    metadata: &serde_json::Value,
    directory: &Path,
) -> Result<usize> {
    let lock = model::parse_lockfile(&fs::read_to_string(directory.join("Cargo.lock"))?)?;
    let lock: BTreeMap<_, _> = lock
        .into_iter()
        .map(|package| ((package.name.clone(), package.version.clone()), package))
        .collect();
    let packages = metadata
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .context("consumer metadata has no packages")?;
    require_direct(packages, plan)?;
    let home = config.cargo_home.join("registry/src").canonicalize()?;
    let mut audits = Vec::new();

    for target in &plan.targets {
        let Some(package) = resolved(packages, target) else {
            continue;
        };
        audits.push(audit(config, target, package, &lock, archives, &home)?);
    }

    for name in DEPENDENCIES {
        ensure!(
            audits.iter().any(|audit| audit.package == name),
            "consumer audit lacks required package {name}"
        );
    }

    fs::write(
        config.evidence.join("consumer-audit.json"),
        serde_json::to_vec_pretty(&audits)?,
    )?;
    Ok(audits.len())
}

/// Requires every direct external dependency to resolve at the planned version.
fn require_direct(packages: &[serde_json::Value], plan: &model::Plan) -> Result<()> {
    for name in DEPENDENCIES {
        let target = plan.require(name)?;
        ensure!(
            resolved(packages, target).is_some(),
            "consumer metadata lacks required package {name} {}",
            target.version
        );
    }

    Ok(())
}

/// Finds one exact resolved package identity.
fn resolved<'a>(
    packages: &'a [serde_json::Value],
    target: &model::Target,
) -> Option<&'a serde_json::Value> {
    packages.iter().find(|package| {
        package.get("name").and_then(serde_json::Value::as_str) == Some(target.name.as_str())
            && package.get("version").and_then(serde_json::Value::as_str)
                == Some(target.version.as_str())
    })
}

/// Binds one resolved package to its archive checksum and reviewed member bytes.
fn audit(
    config: &Config,
    target: &model::Target,
    package: &serde_json::Value,
    lock: &BTreeMap<(String, String), model::LockedPackage>,
    archives: &[archives::ReviewedArchive],
    home: &Path,
) -> Result<Audit> {
    let source = registry_source(package, target)?;
    let manifest = manifest(package, home)?;
    let archive = archive(archives, target)?;
    let locked = lock
        .get(&(target.name.clone(), target.version.clone()))
        .with_context(|| format!("consumer lock lacks {}", target.name))?;
    ensure!(
        locked.checksum.as_deref() == Some(archive.sha256.as_str()),
        "{} lock checksum differs from reviewed archive",
        target.name
    );
    let members = members(config, target, archive, &manifest)?;

    Ok(Audit {
        package: target.name.clone(),
        version: target.version.clone(),
        package_id: package
            .get("id")
            .and_then(serde_json::Value::as_str)
            .context("package has no id")?
            .to_owned(),
        source: source.to_owned(),
        manifest,
        checksum: archive.sha256.clone(),
        members,
    })
}

/// Requires an actual registry source rather than a path package.
fn registry_source<'a>(package: &'a serde_json::Value, target: &model::Target) -> Result<&'a str> {
    let source = package
        .get("source")
        .and_then(serde_json::Value::as_str)
        .with_context(|| format!("{} resolved from a path", target.name))?;
    ensure!(
        source.starts_with("registry+"),
        "{} resolved outside a registry",
        target.name
    );
    Ok(source)
}

/// Finds the exact reviewed archive for one target.
fn archive<'a>(
    archives: &'a [archives::ReviewedArchive],
    target: &model::Target,
) -> Result<&'a archives::ReviewedArchive> {
    archives
        .iter()
        .find(|archive| archive.name == target.name)
        .with_context(|| format!("missing archive for {}", target.name))
}

/// Rechecks reviewed member bytes only for an archive with a fresh review.
fn members(
    config: &Config,
    target: &model::Target,
    archive: &archives::ReviewedArchive,
    manifest: &Path,
) -> Result<Option<usize>> {
    match archive.review {
        archives::Review::FreshAnalysis => Ok(Some(bytes::compare(
            config,
            &target.name,
            archive,
            manifest,
        )?)),
        archives::Review::GateUnsupported => Ok(None),
    }
}

/// Requires the unpacked manifest to live under the fresh Cargo home.
fn manifest(package: &serde_json::Value, home: &Path) -> Result<PathBuf> {
    let manifest = PathBuf::from(
        package
            .get("manifest_path")
            .and_then(serde_json::Value::as_str)
            .context("package has no manifest path")?,
    )
    .canonicalize()?;
    ensure!(
        manifest.starts_with(home),
        "package resolved outside the fresh Cargo home"
    );

    Ok(manifest)
}
