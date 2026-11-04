//! Writes the exact final evidence record and its release blockers.

use super::{Config, archives, consumer, evidence, model, publish, registry, source};
use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct Summary<'a> {
    source_tree: String,
    workspace: PathBuf,
    head: Option<String>,
    plan: &'a model::Plan,
    archives: &'a [archives::ReviewedArchive],
    registry: &'a registry::RegistryEvidence,
    consumer: &'a consumer::ConsumerEvidence,
    publish: Option<&'a publish::PublishEvidence>,
    release_blockers: Vec<String>,
}

/// Writes `summary.json` and returns the blockers for the final console line.
pub(super) fn write(
    config: &Config,
    workspace: &Path,
    plan: &model::Plan,
    archives: &[archives::ReviewedArchive],
    registry: &registry::RegistryEvidence,
    consumer: &consumer::ConsumerEvidence,
    publish: Option<&publish::PublishEvidence>,
) -> Result<Vec<String>> {
    let blockers = blockers(plan, publish);
    let summary = Summary {
        source_tree: source::digest(workspace)?,
        workspace: workspace.to_owned(),
        head: source::revision(&config.workspace)?,
        plan,
        archives,
        registry,
        consumer,
        publish,
        release_blockers: blockers.clone(),
    };
    evidence::json(&config.evidence.join("summary.json"), &summary)?;
    Ok(blockers)
}

/// Describes excluded packages, private manifests and dry-run refusal classes.
fn blockers(plan: &model::Plan, publish: Option<&publish::PublishEvidence>) -> Vec<String> {
    let mut blockers = Vec::new();
    excluded(plan, &mut blockers);
    private(plan, &mut blockers);
    dry_run(publish, &mut blockers);

    blockers
}

/// Preserves each excluded package's exact dependency source.
fn excluded(plan: &model::Plan, blockers: &mut Vec<String>) {
    for package in &plan.excluded {
        blockers.push(format!(
            "{} {}: {} ({})",
            package.name,
            package.version,
            package.reason,
            package
                .dependency_source
                .as_deref()
                .unwrap_or("no dependency source")
        ));
    }
}

/// Counts Cargo's explicit publication refusals.
fn private(plan: &model::Plan, blockers: &mut Vec<String>) {
    let disabled = plan
        .targets
        .iter()
        .filter(|target| matches!(&target.publication, model::Publication::PublishDisabled))
        .count();

    if disabled > 0 {
        blockers.push(format!(
            "{disabled} included crates retain publish = false; no registry publication is possible"
        ));
    }
}

/// Collapses identical dry-run refusal classes into one line per class.
fn dry_run(publish: Option<&publish::PublishEvidence>, blockers: &mut Vec<String>) {
    let Some(publish) = publish else {
        return;
    };

    let mut classes = std::collections::BTreeMap::new();

    for outcome in &publish.outcomes {
        if let Some(blocker) = &outcome.blocker {
            *classes
                .entry((outcome.result.clone(), blocker.clone()))
                .or_insert(0_usize) += 1;
        }
    }

    for ((result, blocker), count) in classes {
        blockers.push(format!(
            "{count} cargo publish --dry-run package(s) {result}: {blocker}"
        ));
    }

    if let Some(blocker) = &publish.direct_package.blocker {
        blockers.push(format!(
            "solana_kit_client without the local registry: {blocker}"
        ));
    }
}
