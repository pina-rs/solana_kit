//! Guards the reviewed Rust release selection; it never claims channel readiness.
//!
//! `monochange publish readiness` correctly reports `ready` when a repository
//! selects zero packages. That upstream result is not evidence that the
//! project's reviewed release selection is prepared. This command compares the
//! reviewed plan in `docs/planning/rust-release-plan.json` with the configured
//! Cargo packages, the actual workspace metadata, the committed release record
//! and the observed readiness selection, and reports only whether the
//! selection is exact. Exit 0 qualifies the selection alone: artifact
//! verification, registry ownership, trusted-publishing setup and publication
//! authorization stay separate external prerequisites.

mod inputs;
mod observed;
mod plan;
#[cfg(test)]
mod tests;

use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use {
    inputs::Metadata,
    observed::observed_reports,
    plan::{
        batch_blockers, manifest_blockers, scope_blockers, summary as plan_summary,
        version_blockers,
    },
};

/// The narrow release decision this guard is allowed to report.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Status {
    SelectionExact,
    SelectionMismatch,
}

impl Status {
    fn word(self) -> &'static str {
        match self {
            Self::SelectionExact => "selection-exact",
            Self::SelectionMismatch => "selection-mismatch",
        }
    }
}

/// Absolute input and output paths for one selection check.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    plan: PathBuf,
    monochange: PathBuf,
    release_record: PathBuf,
    readiness: PathBuf,
    record_commit: Option<String>,
    output: Option<PathBuf>,
}

/// The complete machine-readable selection report.
#[derive(Serialize)]
struct Report {
    schema_version: u8,
    kind: &'static str,
    status: Status,
    plan: plan::PlanSummary,
    selection: observed::SelectionReport,
    release_record: observed::ReleaseReport,
    batches: Vec<inputs::PlanBatch>,
    blockers: Vec<String>,
}

/// Runs one selection check; exit 0 means the selection alone is exact.
pub(crate) fn run(path: &Path) -> Result<()> {
    let config: Config = serde_json::from_slice(&fs::read(path)?)
        .with_context(|| format!("parse selection config {}", path.display()))?;
    validate_paths(&config, path)?;
    let plan: inputs::Plan = read_json(&config.plan)?;
    let configured = configured_packages(&config.monochange)?;
    let metadata = workspace_metadata()?;
    let release: inputs::ReleaseRecord = read_json(&config.release_record)?;
    let readiness: inputs::Readiness = read_json(&config.readiness)?;
    let report = evaluate(
        &plan,
        &configured,
        &metadata,
        &release,
        &readiness,
        config.record_commit.as_deref(),
    );

    if let Some(output) = &config.output {
        write_report(output, &report)?;
    }

    print_report(&report);

    if report.status == Status::SelectionMismatch {
        bail!(
            "rust release selection is {}: {} blocker(s)",
            report.status.word(),
            report.blockers.len()
        );
    }

    Ok(())
}

/// Requires explicit absolute paths so no output lands beside source by accident.
fn validate_paths(config: &Config, config_path: &Path) -> Result<()> {
    let inputs = [
        &config.plan,
        &config.monochange,
        &config.release_record,
        &config.readiness,
    ];

    for path in inputs {
        ensure!(path.is_absolute(), "selection paths must be absolute");
    }

    if let Some(output) = &config.output {
        ensure!(output.is_absolute(), "selection paths must be absolute");

        if output.exists() {
            let output = output.canonicalize()?;

            for input in inputs
                .into_iter()
                .map(PathBuf::as_path)
                .chain([config_path])
            {
                ensure!(
                    output != input.canonicalize()?,
                    "selection output would overwrite an input"
                );
            }
        }
    }

    Ok(())
}

/// Reads one JSON input with a path-bearing diagnostic.
fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    serde_json::from_slice(&fs::read(path)?).with_context(|| format!("parse {}", path.display()))
}

/// Reads the configured Cargo package ids from `monochange.toml`.
fn configured_packages(path: &Path) -> Result<BTreeSet<String>> {
    let value: toml::Value = toml::from_str(&fs::read_to_string(path)?)?;
    let default_type = value
        .get("defaults")
        .and_then(|defaults| defaults.get("package_type"))
        .and_then(toml::Value::as_str)
        .unwrap_or("cargo");
    let packages = value
        .get("package")
        .and_then(toml::Value::as_table)
        .context("monochange.toml has no [package.*] tables")?;
    let mut names = BTreeSet::new();

    for (name, package) in packages {
        let package_type = package
            .get("type")
            .and_then(toml::Value::as_str)
            .unwrap_or(default_type);

        if package_type == "cargo" {
            names.insert(name.clone());
        }
    }

    ensure!(
        !names.is_empty(),
        "monochange.toml configures no Cargo packages"
    );
    Ok(names)
}

/// Reads workspace-only Cargo metadata without network or dependency checkouts.
fn workspace_metadata() -> Result<Metadata> {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.args([
        "metadata",
        "--format-version",
        "1",
        "--no-deps",
        "--locked",
        "--offline",
    ]);
    let output = command.output().context("execute cargo metadata")?;
    ensure!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).context("parse Cargo metadata")
}

/// Compares the reviewed plan with every selection boundary; no network, no credentials.
fn evaluate(
    reviewed_plan: &inputs::Plan,
    configured: &BTreeSet<String>,
    metadata: &Metadata,
    release: &inputs::ReleaseRecord,
    readiness: &inputs::Readiness,
    record_commit: Option<&str>,
) -> Report {
    let packages = inputs::package_map(metadata);
    let public = inputs::public_names(reviewed_plan);
    let mut blockers = identity_blockers(reviewed_plan, release, readiness, record_commit);

    if public.is_empty() {
        blockers.push("reviewed plan has no public packages".into());
    }

    scope_blockers(reviewed_plan, configured, &packages, &mut blockers);
    version_blockers(reviewed_plan, &packages, &mut blockers);
    manifest_blockers(reviewed_plan, &packages, &public, &mut blockers);
    batch_blockers(reviewed_plan, &public, &mut blockers);
    let (selection, release_record) = observed_reports(
        &readiness.packages,
        &release.package_publications,
        &public,
        reviewed_plan,
    );
    blockers.extend(selection_blockers(&selection));
    blockers.extend(publication_blockers(&release_record));
    blockers.sort();
    blockers.dedup();

    let status = if blockers.is_empty() {
        Status::SelectionExact
    } else {
        Status::SelectionMismatch
    };

    Report {
        schema_version: 1,
        kind: "solanaKit.rustReleaseSelection",
        status,
        plan: plan_summary(reviewed_plan),
        selection,
        release_record,
        batches: reviewed_plan.batches.clone(),
        blockers,
    }
}

/// Binds each input to the monochange record identity it claims.
fn identity_blockers(
    plan: &inputs::Plan,
    release: &inputs::ReleaseRecord,
    readiness: &inputs::Readiness,
    record_commit: Option<&str>,
) -> Vec<String> {
    let mut blockers = Vec::new();

    if plan.schema_version != 1 {
        blockers.push(format!(
            "reviewed plan schema_version is {}, expected 1",
            plan.schema_version
        ));
    }

    if plan
        .scope
        .as_deref()
        .is_none_or(|scope| scope.trim().is_empty())
    {
        blockers.push("reviewed plan has no scope statement".into());
    }

    if plan.kind != "solanaKit.rustReleasePlan" {
        blockers.push(format!(
            "reviewed plan kind is {}, expected solanaKit.rustReleasePlan",
            plan.kind
        ));
    }

    if release.kind != "monochange.releaseRecord" {
        blockers.push(format!(
            "release record kind is {}, expected monochange.releaseRecord",
            release.kind
        ));
    }

    if readiness.kind != "monochange.publishReadiness" {
        blockers.push(format!(
            "readiness kind is {}, expected monochange.publishReadiness",
            readiness.kind
        ));
    }

    if let Some(expected) = record_commit
        && readiness.record_commit.as_deref() != Some(expected)
    {
        blockers.push(format!(
            "readiness record commit {:?} does not match the configured release record commit {expected}",
            readiness.record_commit
        ));
    }

    blockers
}

/// Turns every selection mismatch into one printable blocker line.
fn selection_blockers(selection: &observed::SelectionReport) -> Vec<String> {
    let mut blockers = Vec::new();

    if selection.observed.is_empty() {
        blockers.push(
            "readiness selection is empty; zero selected packages is not a release selection"
                .into(),
        );
    }

    for package in &selection.duplicates {
        blockers.push(format!(
            "readiness selection contains duplicate package {package}"
        ));
    }

    for package in &selection.unexpected {
        blockers.push(format!(
            "readiness selection contains unexpected package {package}"
        ));
    }

    for package in &selection.missing {
        blockers.push(format!(
            "readiness selection omits expected package {package}"
        ));
    }

    for version in &selection.wrong_versions {
        blockers.push(format!("readiness selection has wrong version {version}"));
    }

    for package in &selection.wrong_targets {
        blockers.push(format!(
            "readiness selection has wrong registry target for {package}"
        ));
    }

    for package in &selection.not_ready {
        blockers.push(format!("readiness selection is not ready for {package}"));
    }

    blockers
}

/// Turns every release-record mismatch into one printable blocker line.
fn publication_blockers(release: &observed::ReleaseReport) -> Vec<String> {
    let mut blockers = Vec::new();

    if release.publications == 0 {
        blockers.push("release record contains no package publications".into());
    }

    for package in &release.duplicates {
        blockers.push(format!(
            "release record contains duplicate publication for {package}"
        ));
    }

    for package in &release.missing {
        blockers.push(format!(
            "release record has no publication entry for public package {package}"
        ));
    }

    for package in &release.unexpected {
        blockers.push(format!(
            "release record publishes non-public package {package}"
        ));
    }

    for version in &release.wrong_versions {
        blockers.push(format!("release publication version mismatch: {version}"));
    }

    for target in &release.wrong_targets {
        blockers.push(format!("release publication has wrong target {target}"));
    }

    blockers
}

/// Writes the machine-readable selection report with a final newline.
fn write_report(path: &Path, report: &Report) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(report)?;
    bytes.push(b'\n');
    fs::write(path, bytes).context("write selection report")?;
    Ok(())
}

/// Prints the narrow selection decision and every blocker.
fn print_report(report: &Report) {
    println!(
        "rust-release-selection: {}",
        report.status.word().to_uppercase()
    );
    println!(
        "plan: {} packages ({} public, {} blocked-upstream, {} private-fixture, {} private-deferred, {} reserved)",
        report.plan.packages,
        report.plan.public,
        report.plan.blocked_upstream,
        report.plan.private_fixture,
        report.plan.private_deferred,
        report.plan.reserved
    );
    println!(
        "readiness selection: {} ({} expected, {} observed)",
        report.selection.status,
        report.selection.expected.len(),
        report.selection.observed.len()
    );
    println!(
        "release record publications: {} ({} expected)",
        report.release_record.publications, report.release_record.expected_publications
    );
    println!("batches: {}", report.batches.len());

    for blocker in &report.blockers {
        println!("blocker: {blocker}");
    }
}
