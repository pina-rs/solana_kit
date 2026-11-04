//! Resolves the included SDK set and dependency-first order.

use super::{Excluded, Metadata, Plan, blocking, order, workspace};
use anyhow::{Result, ensure};
use std::{collections::BTreeSet, path::Path};

/// Selects only `crates/*` workspace members and resolves their local dependency order.
pub(crate) fn plan(metadata: &Metadata, workspace: &Path) -> Result<Plan> {
    let workspace = workspace.canonicalize()?;
    let crates = workspace.join("crates").canonicalize()?;
    let packages = workspace::packages(metadata, &crates)?;
    ensure!(packages.contains_key("solana_kit_core"), "Core is missing");
    let (local, mut blocked) = blocking::local_edges(&packages, &crates)?;
    blocking::propagate(&local, &mut blocked);
    let included = included(&packages, &blocked);
    let order = order::dependency_order(&local, &included)?;
    let targets = order::targets(&packages, &local, &included, order)?;
    let excluded = excluded(&packages, &blocked);

    Ok(Plan {
        roots: packages
            .values()
            .map(|package| package.manifest_path.clone())
            .collect(),
        targets,
        excluded,
    })
}

/// Returns package names without an external prerequisite.
fn included(
    packages: &std::collections::BTreeMap<String, &super::Package>,
    blocked: &blocking::Blockers,
) -> BTreeSet<String> {
    packages
        .keys()
        .filter(|name| !blocked.contains_key(*name))
        .cloned()
        .collect()
}

/// Retains exact exclusion reasons for evidence.
fn excluded(
    packages: &std::collections::BTreeMap<String, &super::Package>,
    blocked: &blocking::Blockers,
) -> Vec<Excluded> {
    packages
        .keys()
        .filter_map(|name| {
            blocked.get(name).map(|(reason, source)| Excluded {
                name: name.clone(),
                version: packages[name].version.clone(),
                reason: reason.clone(),
                dependency_source: source.clone(),
            })
        })
        .collect()
}
