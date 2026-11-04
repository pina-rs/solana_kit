//! Separates local package edges from dependencies outside the workspace.

use super::Package;
use anyhow::{Result, ensure};
use std::{collections::BTreeMap, path::Path};

pub(super) type LocalEdges = BTreeMap<String, Vec<String>>;
pub(super) type Blockers = BTreeMap<String, (String, Option<String>)>;

/// Separates local package edges from blockers outside the workspace crate set.
pub(super) fn local_edges(
    packages: &BTreeMap<String, &Package>,
    crates: &Path,
) -> Result<(LocalEdges, Blockers)> {
    let mut local = BTreeMap::new();
    let mut blocked = BTreeMap::new();

    for (name, package) in packages {
        let dependencies = local_dependencies(name, package, packages, crates, &mut blocked)?;
        local.insert(name.clone(), dependencies);
    }

    Ok((local, blocked))
}

/// Records one package's blockers while returning only local, non-dev dependencies.
fn local_dependencies(
    name: &str,
    package: &Package,
    packages: &BTreeMap<String, &Package>,
    crates: &Path,
    blocked: &mut Blockers,
) -> Result<Vec<String>> {
    let mut dependencies = Vec::new();

    for dependency in &package.dependencies {
        if dependency.kind.as_deref() == Some("dev") {
            continue;
        }

        match &dependency.source {
            Some(source) if source.starts_with("git+") => {
                blocked.insert(
                    name.to_owned(),
                    ("unpublished Git prerequisite".into(), Some(source.clone())),
                );
            }

            Some(_) => {}
            None => local_dependency(
                name,
                dependency,
                packages,
                crates,
                blocked,
                &mut dependencies,
            )?,
        }
    }

    dependencies.sort();
    dependencies.dedup();
    Ok(dependencies)
}

/// Classifies one path dependency as local or an external blocker.
fn local_dependency(
    name: &str,
    dependency: &super::Dependency,
    packages: &BTreeMap<String, &Package>,
    crates: &Path,
    blocked: &mut Blockers,
    dependencies: &mut Vec<String>,
) -> Result<()> {
    let Some(path) = &dependency.path else {
        return Ok(());
    };

    if path.canonicalize()?.starts_with(crates) {
        ensure!(
            packages.contains_key(&dependency.name),
            "unknown local dependency {}",
            dependency.name
        );
        dependencies.push(dependency.name.clone());
        return Ok(());
    }

    blocked.insert(
        name.to_owned(),
        (
            "path dependency outside the workspace crate set".into(),
            Some(path.to_string_lossy().into_owned()),
        ),
    );
    Ok(())
}

/// Marks every dependent of an excluded package as excluded too.
pub(super) fn propagate(local: &LocalEdges, blocked: &mut Blockers) {
    loop {
        if !propagate_once(local, blocked) {
            return;
        }
    }
}

/// Adds every newly excluded dependent and reports whether the set changed.
fn propagate_once(local: &LocalEdges, blocked: &mut Blockers) -> bool {
    let mut added = false;

    for (name, dependencies) in local {
        if blocked.contains_key(name) {
            continue;
        }

        if let Some(dependency) = dependencies
            .iter()
            .find(|dependency| blocked.contains_key(*dependency))
        {
            blocked.insert(
                name.clone(),
                (format!("depends on excluded package `{dependency}`"), None),
            );
            added = true;
        }
    }

    added
}
