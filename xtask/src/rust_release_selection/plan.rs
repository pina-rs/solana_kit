//! Validates the reviewed plan against configuration, manifests and batches.

use super::inputs::{self, Decision, Package, Plan};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// Counts each reviewed decision without exposing the full table twice.
#[derive(Serialize)]
pub(super) struct PlanSummary {
    pub(super) packages: usize,
    pub(super) public: usize,
    pub(super) blocked_upstream: usize,
    pub(super) private_fixture: usize,
    pub(super) private_deferred: usize,
    pub(super) reserved: usize,
}

/// Summarizes the reviewed plan decisions.
pub(super) fn summary(plan: &Plan) -> PlanSummary {
    let mut counts = PlanSummary {
        packages: plan.packages.len(),
        public: 0,
        blocked_upstream: 0,
        private_fixture: 0,
        private_deferred: 0,
        reserved: 0,
    };

    for entry in &plan.packages {
        match entry.decision {
            Decision::Public => counts.public += 1,
            Decision::BlockedUpstream => counts.blocked_upstream += 1,
            Decision::PrivateFixture => counts.private_fixture += 1,
            Decision::PrivateDeferred => counts.private_deferred += 1,
            Decision::Reserved => counts.reserved += 1,
        }
    }

    counts
}

/// Rejects duplicate, omitted, invented and reason-free reviewed packages.
pub(super) fn scope_blockers(
    plan: &Plan,
    configured: &BTreeSet<String>,
    packages: &BTreeMap<&str, &Package>,
    blockers: &mut Vec<String>,
) {
    let mut seen = BTreeSet::new();

    for entry in &plan.packages {
        collect_entry_blockers(entry, configured, packages, &mut seen, blockers);
    }

    for package in configured {
        if !seen.contains(package.as_str()) {
            blockers.push(format!(
                "configured Cargo package {package} is omitted from the reviewed plan"
            ));
        }
    }
}

/// Rejects one plan entry that is duplicated, invented, absent or unreasoned.
fn collect_entry_blockers<'a>(
    entry: &'a inputs::PlanPackage,
    configured: &BTreeSet<String>,
    packages: &BTreeMap<&str, &Package>,
    seen: &mut BTreeSet<&'a str>,
    blockers: &mut Vec<String>,
) {
    if entry.reason.trim().is_empty() {
        blockers.push(format!(
            "reviewed plan entry {} has no recorded reason",
            entry.package
        ));
    }

    if !seen.insert(entry.package.as_str()) {
        blockers.push(format!(
            "reviewed plan contains duplicate package {}",
            entry.package
        ));
    }

    if !configured.contains(&entry.package) {
        blockers.push(format!(
            "reviewed plan contains invented package {}",
            entry.package
        ));
    }

    if !packages.contains_key(entry.package.as_str()) {
        blockers.push(format!(
            "reviewed package {} is absent from Cargo metadata",
            entry.package
        ));
    }
}

/// Requires every reviewed version to match the actual workspace manifest.
pub(super) fn version_blockers(
    plan: &Plan,
    packages: &BTreeMap<&str, &Package>,
    blockers: &mut Vec<String>,
) {
    for entry in &plan.packages {
        let Some(package) = packages.get(entry.package.as_str()) else {
            continue;
        };

        if package.version != entry.version {
            blockers.push(format!(
                "reviewed version for {} is {}, workspace manifest has {}",
                entry.package, entry.version, package.version
            ));
        }
    }
}

/// Requires each manifest's publication state and sources to match its decision.
pub(super) fn manifest_blockers(
    plan: &Plan,
    packages: &BTreeMap<&str, &Package>,
    public: &BTreeSet<String>,
    blockers: &mut Vec<String>,
) {
    for entry in &plan.packages {
        let Some(package) = packages.get(entry.package.as_str()) else {
            continue;
        };

        if let Some(blocker) = decision_mismatch(entry, package) {
            blockers.push(blocker);
        }

        if entry.decision != Decision::BlockedUpstream {
            push_git_blockers(&entry.package, package, blockers);
        }

        if entry.decision == Decision::Public {
            push_local_blockers(&entry.package, package, packages, public, blockers);
        }
    }
}

/// Returns the first manifest decision mismatch for one package.
fn decision_mismatch(entry: &inputs::PlanPackage, package: &Package) -> Option<String> {
    let publish = inputs::cargo_publish(package);

    match entry.decision {
        Decision::Reserved => None,
        Decision::Public if publish == "allowed" => None,
        Decision::Public => Some(format!(
            "public package {} has Cargo publication {publish}",
            entry.package
        )),
        Decision::PrivateFixture | Decision::PrivateDeferred if publish == "disabled" => None,
        Decision::PrivateFixture | Decision::PrivateDeferred => Some(format!(
            "private package {} has Cargo publication {publish}",
            entry.package
        )),
        Decision::BlockedUpstream if inputs::has_git_dependency(package) => None,
        Decision::BlockedUpstream => Some(format!(
            "blocked package {} has no unpublished Git dependency",
            entry.package
        )),
    }
}

/// Rejects any non-dev Git dependency outside the explicitly blocked package.
fn push_git_blockers(package: &str, manifest: &Package, blockers: &mut Vec<String>) {
    for dependency in inputs::git_dependencies(manifest) {
        blockers.push(format!(
            "package {package} has unpublished Git dependency {}",
            dependency.name
        ));
    }
}

/// Rejects public packages that depend on workspace packages outside the public set.
fn push_local_blockers(
    package: &str,
    manifest: &Package,
    packages: &BTreeMap<&str, &Package>,
    public: &BTreeSet<String>,
    blockers: &mut Vec<String>,
) {
    for dependency in inputs::local_dependencies(manifest, packages) {
        if !public.contains(&dependency.name) {
            blockers.push(format!(
                "public package {package} depends on non-public package {}",
                dependency.name
            ));
        }
    }
}

/// Requires the reviewed batches to partition the public selection exactly.
pub(super) fn batch_blockers(plan: &Plan, public: &BTreeSet<String>, blockers: &mut Vec<String>) {
    let mut batched = BTreeSet::new();

    for batch in &plan.batches {
        collect_batch_blockers(batch, public, &mut batched, blockers);
    }

    for package in public {
        if !batched.contains(package) {
            blockers.push(format!("reviewed batches omit public package {package}"));
        }
    }
}

/// Rejects one empty, duplicated or non-public batch entry.
fn collect_batch_blockers(
    batch: &inputs::PlanBatch,
    public: &BTreeSet<String>,
    batched: &mut BTreeSet<String>,
    blockers: &mut Vec<String>,
) {
    if batch.packages.is_empty() {
        blockers.push(format!("reviewed batch {} is empty", batch.index));
    }

    for package in &batch.packages {
        if !batched.insert(package.clone()) {
            blockers.push(format!(
                "reviewed batches contain duplicate package {package}"
            ));
        }

        if !public.contains(package) {
            blockers.push(format!(
                "reviewed batches include non-public package {package}"
            ));
        }
    }
}
