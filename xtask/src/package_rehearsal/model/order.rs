//! Orders included SDK packages and maps their publication state.

use super::{Package, Publication, Target, blocking::LocalEdges};
use anyhow::{Context, Result, bail};
use std::collections::{BTreeMap, BTreeSet};

/// Builds ordered targets with only included local dependency names.
pub(super) fn targets(
    packages: &BTreeMap<String, &Package>,
    local: &LocalEdges,
    included: &BTreeSet<String>,
    order: Vec<String>,
) -> Result<Vec<Target>> {
    let mut targets = Vec::new();

    for name in order {
        let package = packages[&name];
        targets.push(Target {
            name: name.clone(),
            version: package.version.clone(),
            manifest_path: package.manifest_path.clone(),
            source: package
                .manifest_path
                .parent()
                .context("manifest has no parent")?
                .to_owned(),
            publication: publication(package),
            dependencies: local[&name]
                .iter()
                .filter(|dependency| included.contains(*dependency))
                .cloned()
                .collect(),
        });
    }

    Ok(targets)
}

/// Maps Cargo's `publish` field to an explicit publication decision.
fn publication(package: &Package) -> Publication {
    match &package.publish {
        None => Publication::Publishable,
        Some(registries) if registries.is_empty() => Publication::PublishDisabled,
        Some(registries) => Publication::Restricted(registries.clone()),
    }
}

/// Produces a deterministic dependency-first order or reports a cycle.
pub(super) fn dependency_order(
    local: &LocalEdges,
    included: &BTreeSet<String>,
) -> Result<Vec<String>> {
    let mut indegree = indegrees(local, included);
    let dependents = dependents(local, included);
    let mut ready: BTreeSet<_> = indegree
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(name, _)| name.clone())
        .collect();
    let mut order = Vec::new();

    while let Some(name) = ready.pop_first() {
        order.push(name.clone());

        for dependent in dependents.get(&name).into_iter().flatten() {
            let count = indegree
                .get_mut(dependent)
                .context("dependent package has no indegree")?;
            *count -= 1;

            if *count == 0 {
                ready.insert(dependent.clone());
            }
        }
    }

    if order.len() != included.len() {
        bail!("SDK package dependency cycle");
    }

    Ok(order)
}

/// Counts included dependencies for every package.
fn indegrees(local: &LocalEdges, included: &BTreeSet<String>) -> BTreeMap<String, usize> {
    included
        .iter()
        .map(|name| {
            let count = local[name]
                .iter()
                .filter(|dependency| included.contains(*dependency))
                .count();
            (name.clone(), count)
        })
        .collect()
}

/// Inverts included dependency edges into dependents.
fn dependents(local: &LocalEdges, included: &BTreeSet<String>) -> BTreeMap<String, Vec<String>> {
    let mut dependents: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for name in included {
        for dependency in &local[name] {
            if included.contains(dependency) {
                dependents
                    .entry(dependency.clone())
                    .or_default()
                    .push(name.clone());
            }
        }
    }

    dependents
}

#[cfg(test)]
mod tests {
    use super::{dependency_order, publication};
    use crate::package_rehearsal::model::{Package, Publication};
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn publication_preserves_private_and_restricted_manifests() {
        let package = |publish| Package {
            id: "id".into(),
            name: "package".into(),
            version: "0.1.0".into(),
            manifest_path: "Cargo.toml".into(),
            publish,
            dependencies: vec![],
        };
        assert!(matches!(
            publication(&package(None)),
            Publication::Publishable
        ));
        assert!(matches!(
            publication(&package(Some(vec![]))),
            Publication::PublishDisabled
        ));
        assert!(matches!(
            publication(&package(Some(vec!["private".into()]))),
            Publication::Restricted(registries) if registries == ["private"]
        ));
    }

    #[test]
    fn orders_local_dependencies_before_dependents() {
        let local = BTreeMap::from([
            ("core".to_owned(), vec![]),
            ("client".to_owned(), vec!["core".to_owned()]),
            ("wallet".to_owned(), vec!["core".to_owned()]),
        ]);
        let included = local.keys().cloned().collect::<BTreeSet<_>>();
        assert_eq!(
            dependency_order(&local, &included).unwrap(),
            ["core", "client", "wallet"]
        );
    }
}
