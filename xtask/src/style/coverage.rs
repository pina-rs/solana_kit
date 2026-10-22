//! Git supplies authored paths explicitly, bypassing analyzer directory exclusions.

use super::Report;
use anyhow::{Context, Result, ensure};
use std::{collections::BTreeSet, path::Component, path::Path, process::Command};

/// Includes tracked files even when ignored, plus new nonignored files; never walks caches.
pub(super) fn inventory() -> Result<BTreeSet<String>> {
    let output = Command::new("git")
        .args([
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ])
        .output()
        .context("execute Git style inventory")?;
    ensure!(output.status.success(), "Git style inventory failed");
    let paths = std::str::from_utf8(&output.stdout).context("Git paths must be UTF-8")?;
    let mut inventory = BTreeSet::new();

    for path in paths.split_terminator('\0') {
        let path = normalize(path)?;

        if is_cache(&path) {
            continue;
        }

        ensure!(inventory.insert(path.clone()), "duplicate Git path: {path}");
        let metadata = std::fs::symlink_metadata(&path)
            .with_context(|| format!("inspect authored file: {path}"))?;
        ensure!(
            metadata.is_file(),
            "authored path is not a regular file: {path}"
        );
    }

    ensure!(!inventory.is_empty(), "Git style inventory is empty");
    Ok(inventory)
}

/// Every Rust input must have a score; scores for unknown or aliased paths are errors.
pub(super) fn check(report: &Report, inventory: &BTreeSet<String>) -> Result<()> {
    let mut scored = BTreeSet::new();

    for file in &report.files {
        let path = normalize(&file.path)?;
        ensure!(inventory.contains(&path), "unknown scored path: {path}");
        ensure!(scored.insert(path.clone()), "duplicate scored path: {path}");
    }

    for path in inventory.iter().filter(|path| path.ends_with(".rs")) {
        ensure!(
            scored.contains(path),
            "authored Rust file was not scored: {path}"
        );
    }

    Ok(())
}

/// Accepts the analyzer's optional leading ./ without accepting traversal or absolute paths.
fn normalize(path: &str) -> Result<String> {
    let mut parts = Vec::new();

    for component in Path::new(path).components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => parts.push(part.to_str().context("path must be UTF-8")?),
            _ => anyhow::bail!("style path must stay within the repository: {path}"),
        }
    }

    ensure!(!parts.is_empty(), "style path is empty");
    Ok(parts.join("/"))
}

/// Build artifacts and local execution receipts are outside the authored source inventory.
fn is_cache(path: &str) -> bool {
    path.split('/').any(|part| part == "target") || path.starts_with(".monochange/local/")
}
