//! Executes the SDK package/release rehearsal against reviewed archives and a registry consumer.

use anyhow::{Context, Result};
use std::{fs, path::Path};

mod archives;
mod command;
mod config;
mod consumer;
mod evidence;
mod model;
mod publish;
mod registry;
mod source;
mod summary;

pub(crate) use command::{run_command, success};
pub(crate) use config::Config;

/// Rehearses archive creation, review, offline registry resolution and an external consumer.
pub(crate) fn run(path: &Path) -> Result<()> {
    let config: Config = serde_json::from_slice(&fs::read(path)?)?;
    config::validate(&config)?;
    fs::create_dir(&config.evidence).context("evidence must be a new directory")?;
    fs::create_dir_all(config.evidence.join("commands"))?;
    fs::create_dir_all(&config.target)?;
    let workspace = config.workspace.canonicalize()?;
    let plan = prepare(&config, &workspace)?;
    let archives = archives::run(&config, &plan, &config.evidence)?;
    let lockfile = model::closure_lockfile(&workspace.join("Cargo.lock"), &plan, &config.evidence)?;
    let registry = registry::run(&config, &plan, &archives, &lockfile)?;
    let consumer = consumer::run(&config, &plan, &archives, &registry)?;
    let publish = publish(&config, &plan, &workspace)?;
    let blockers = summary::write(
        &config,
        &workspace,
        &plan,
        &archives,
        &registry,
        &consumer,
        publish.as_ref(),
    )?;
    print(&plan, &archives, &registry, &consumer, &blockers);

    Ok(())
}

/// Reads Cargo metadata and writes the exact included/excluded package plan.
fn prepare(config: &Config, workspace: &Path) -> Result<model::Plan> {
    let metadata = model::metadata(config, workspace, &config.evidence)?;
    let plan = model::plan(&metadata, workspace)?;
    evidence::json(&config.evidence.join("packages.json"), &plan)?;
    Ok(plan)
}

/// Executes the publish dry run only when the caller did not disable it.
fn publish(
    config: &Config,
    plan: &model::Plan,
    workspace: &Path,
) -> Result<Option<publish::PublishEvidence>> {
    if !config.publish_dry_run {
        return Ok(None);
    }

    publish::run(config, plan, workspace).map(Some)
}

/// Prints the final evidence summary after all evidence files are written.
fn print(
    plan: &model::Plan,
    archives: &[archives::ReviewedArchive],
    registry: &registry::RegistryEvidence,
    consumer: &consumer::ConsumerEvidence,
    blockers: &[String],
) {
    if !plan.excluded.is_empty() {
        println!(
            "rehearsal excludes: {}",
            plan.excluded
                .iter()
                .map(|package| format!("{} ({})", package.name, package.reason))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let fresh = archives
        .iter()
        .filter(|archive| matches!(&archive.review, archives::Review::FreshAnalysis))
        .count();
    println!(
        "package-rehearsal: {} archives ({fresh} fresh reviews); {} registry packages; consumer {} default and {} selected tests; negative controls passed; {} release blocker(s)",
        archives.len(),
        registry.packages,
        consumer.tests,
        consumer.selected_tests,
        blockers.len()
    );

    for blocker in blockers {
        println!("blocker: {blocker}");
    }
}
