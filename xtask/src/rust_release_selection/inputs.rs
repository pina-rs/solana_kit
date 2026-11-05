//! Input contracts for the Rust release selection guard.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// One reviewed package decision in the Rust release channel.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(super) enum Decision {
    /// Part of the reviewed first-publish set.
    Public,
    /// Implemented, but an unreleased upstream registry package is required.
    BlockedUpstream,
    /// Deliberately private test machinery that must never be published.
    PrivateFixture,
    /// Private until a separate release review includes it.
    PrivateDeferred,
    /// A `0.0.0` reservation that is not a release target.
    Reserved,
}

/// The reviewed release plan; a data artifact, never a readiness stamp.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Plan {
    pub(super) schema_version: u8,
    pub(super) kind: String,
    pub(super) scope: Option<String>,
    pub(super) packages: Vec<PlanPackage>,
    pub(super) batches: Vec<PlanBatch>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PlanPackage {
    pub(super) package: String,
    pub(super) version: String,
    pub(super) decision: Decision,
    pub(super) reason: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PlanBatch {
    pub(super) index: usize,
    pub(super) packages: Vec<String>,
}

/// The subset of Cargo metadata used by this guard.
#[derive(Deserialize)]
pub(super) struct Metadata {
    pub(super) packages: Vec<Package>,
}

#[derive(Deserialize)]
pub(super) struct Package {
    pub(super) name: String,
    pub(super) version: String,
    pub(super) publish: Option<Vec<String>>,
    #[serde(default)]
    pub(super) dependencies: Vec<Dependency>,
}

#[derive(Clone, Deserialize)]
pub(super) struct Dependency {
    pub(super) name: String,
    #[serde(default)]
    pub(super) source: Option<String>,
    #[serde(default)]
    pub(super) kind: Option<String>,
}

/// The committed release record fields that select registry publications.
#[derive(Deserialize)]
pub(super) struct ReleaseRecord {
    pub(super) kind: String,
    #[serde(default)]
    pub(super) package_publications: Vec<Publication>,
}

#[derive(Clone, Deserialize)]
pub(super) struct Publication {
    pub(super) package: String,
    pub(super) ecosystem: String,
    pub(super) registry: String,
    pub(super) version: String,
    pub(super) mode: String,
}

/// The observed `monochange publish readiness` result.
#[derive(Deserialize)]
pub(super) struct Readiness {
    pub(super) kind: String,
    #[serde(default)]
    pub(super) record_commit: Option<String>,
    #[serde(default)]
    pub(super) packages: Vec<Selected>,
}

#[derive(Clone, Deserialize)]
pub(super) struct Selected {
    pub(super) package: String,
    #[serde(default)]
    pub(super) ecosystem: Option<String>,
    #[serde(default)]
    pub(super) registry: Option<String>,
    pub(super) version: String,
    pub(super) status: String,
}

/// Maps metadata by exact package name.
pub(super) fn package_map(metadata: &Metadata) -> BTreeMap<&str, &Package> {
    metadata
        .packages
        .iter()
        .map(|package| (package.name.as_str(), package))
        .collect()
}

/// Returns the reviewed first-publish package names.
pub(super) fn public_names(plan: &Plan) -> BTreeSet<String> {
    plan.packages
        .iter()
        .filter(|entry| entry.decision == Decision::Public)
        .map(|entry| entry.package.clone())
        .collect()
}

/// Maps Cargo's publication field to an explicit manifest decision.
pub(super) fn cargo_publish(package: &Package) -> &'static str {
    match &package.publish {
        None => "allowed",
        Some(registries) if registries.is_empty() => "disabled",
        Some(_) => "restricted",
    }
}

/// Whether a package has at least one non-dev Git dependency.
pub(super) fn has_git_dependency(package: &Package) -> bool {
    package.dependencies.iter().any(non_dev_git)
}

/// The non-dev Git dependencies of one manifest.
pub(super) fn git_dependencies(package: &Package) -> Vec<&Dependency> {
    package
        .dependencies
        .iter()
        .filter(|dependency| non_dev_git(dependency))
        .collect()
}

/// The non-dev dependencies that resolve inside the workspace.
pub(super) fn local_dependencies<'a>(
    package: &'a Package,
    packages: &BTreeMap<&str, &Package>,
) -> Vec<&'a Dependency> {
    package
        .dependencies
        .iter()
        .filter(|dependency| dependency.kind.as_deref() != Some("dev"))
        .filter(|dependency| dependency.source.is_none())
        .filter(|dependency| packages.contains_key(dependency.name.as_str()))
        .collect()
}

/// Whether one dependency is a non-dev Git checkout.
fn non_dev_git(dependency: &Dependency) -> bool {
    dependency.kind.as_deref() != Some("dev") && is_git_source(dependency.source.as_deref())
}

/// Whether a dependency source is a Git checkout.
fn is_git_source(source: Option<&str>) -> bool {
    source.is_some_and(|source| source.starts_with("git+"))
}
