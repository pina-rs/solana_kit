//! Matches the observed readiness selection and release publications exactly.

use super::inputs::{Plan, PlanPackage, Publication, Selected};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// One expected or observed package version.
#[derive(Serialize)]
pub(super) struct PackageVersion {
    pub(super) package: String,
    pub(super) version: String,
}
/// The readiness selection compared with the public plan set.
#[derive(Serialize)]
pub(super) struct SelectionReport {
    pub(super) status: &'static str,
    pub(super) expected: Vec<PackageVersion>,
    pub(super) observed: Vec<PackageVersion>,
    pub(super) missing: Vec<String>,
    pub(super) unexpected: Vec<String>,
    pub(super) wrong_versions: Vec<String>,
    pub(super) duplicates: Vec<String>,
    pub(super) wrong_targets: Vec<String>,
    pub(super) not_ready: Vec<String>,
}

/// The release record publications compared with the public plan set.
#[derive(Serialize)]
pub(super) struct ReleaseReport {
    pub(super) status: &'static str,
    pub(super) publications: usize,
    pub(super) expected_publications: usize,
    pub(super) missing: Vec<String>,
    pub(super) unexpected: Vec<String>,
    pub(super) wrong_versions: Vec<String>,
    pub(super) duplicates: Vec<String>,
    pub(super) wrong_targets: Vec<String>,
}

/// Computes both observed reports from the readiness rows and record publications.
pub(super) fn observed_reports(
    selected: &[Selected],
    publications: &[Publication],
    public: &BTreeSet<String>,
    plan: &Plan,
) -> (SelectionReport, ReleaseReport) {
    let reviewed = reviewed_map(plan);
    let selection = selection_report(selected, public, &reviewed);
    let release = release_report(publications, public, &reviewed);
    (selection, release)
}

/// Maps the reviewed plan by exact package name.
fn reviewed_map(plan: &Plan) -> BTreeMap<&str, &PlanPackage> {
    plan.packages
        .iter()
        .map(|entry| (entry.package.as_str(), entry))
        .collect()
}

/// Matches the observed readiness selection exactly, including versions and targets.
fn selection_report(
    selected: &[Selected],
    public: &BTreeSet<String>,
    reviewed: &BTreeMap<&str, &PlanPackage>,
) -> SelectionReport {
    let observed: Vec<_> = selected
        .iter()
        .map(|row| PackageVersion {
            package: row.package.clone(),
            version: row.version.clone(),
        })
        .collect();
    let expected: Vec<_> = public
        .iter()
        .map(|package| PackageVersion {
            package: package.clone(),
            version: reviewed[package.as_str()].version.clone(),
        })
        .collect();
    let mut mismatches = SelectionReport {
        status: "partial",
        expected,
        observed,
        missing: missing_packages(public, selected),
        unexpected: Vec::new(),
        wrong_versions: Vec::new(),
        duplicates: duplicate_names(selected),
        wrong_targets: Vec::new(),
        not_ready: Vec::new(),
    };

    for row in selected {
        collect_selection_mismatch(row, public, reviewed, &mut mismatches);
    }

    mismatches.status = selection_status(selected, &mismatches);
    mismatches
}

/// Collects one selection row's version, target and readiness mismatches.
fn collect_selection_mismatch(
    row: &Selected,
    public: &BTreeSet<String>,
    reviewed: &BTreeMap<&str, &PlanPackage>,
    report: &mut SelectionReport,
) {
    if !public.contains(&row.package) {
        report.unexpected.push(row.package.clone());
        return;
    }

    let expected_version = &reviewed[row.package.as_str()].version;

    if row.version != *expected_version {
        report.wrong_versions.push(format!(
            "{} {} (expected {expected_version})",
            row.package, row.version
        ));
    }

    if !correct_target(row) {
        report.wrong_targets.push(row.package.clone());
    } else if row.status != "ready" {
        report
            .not_ready
            .push(format!("{}: {}", row.package, row.status));
    }
}

/// Returns expected public packages that readiness omitted.
fn missing_packages(public: &BTreeSet<String>, selected: &[Selected]) -> Vec<String> {
    let observed: BTreeSet<&str> = selected.iter().map(|row| row.package.as_str()).collect();
    public
        .iter()
        .filter(|package| !observed.contains(package.as_str()))
        .cloned()
        .collect()
}

/// Returns duplicate package names in first-seen order.
fn duplicate_names(selected: &[Selected]) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut duplicates = Vec::new();

    for row in selected {
        if !seen.insert(row.package.as_str()) {
            duplicates.push(row.package.clone());
        }
    }

    duplicates
}

/// Whether a selected package targets the Rust crates.io channel.
fn correct_target(row: &Selected) -> bool {
    row.ecosystem.as_deref() == Some("cargo") && row.registry.as_deref() == Some("crates_io")
}

/// Distinguishes exact, empty, and partial selections.
fn selection_status(selected: &[Selected], report: &SelectionReport) -> &'static str {
    if selected.is_empty() {
        return "empty";
    }

    let exact = report.missing.is_empty()
        && report.unexpected.is_empty()
        && report.wrong_versions.is_empty()
        && report.duplicates.is_empty()
        && report.wrong_targets.is_empty()
        && report.not_ready.is_empty();

    if exact { "exact" } else { "partial" }
}

/// Matches the release record's publication entries to the public set exactly.
fn release_report(
    publications: &[Publication],
    public: &BTreeSet<String>,
    reviewed: &BTreeMap<&str, &PlanPackage>,
) -> ReleaseReport {
    let mut by_package: BTreeMap<&str, &Publication> = BTreeMap::new();
    let mut duplicates = Vec::new();

    for publication in publications {
        if by_package
            .insert(publication.package.as_str(), publication)
            .is_some()
        {
            duplicates.push(publication.package.clone());
        }
    }

    let mut report = ReleaseReport {
        status: "partial",
        publications: publications.len(),
        expected_publications: public.len(),
        missing: Vec::new(),
        unexpected: Vec::new(),
        wrong_versions: Vec::new(),
        duplicates,
        wrong_targets: Vec::new(),
    };

    for package in public {
        collect_publication_mismatch(
            package,
            by_package.get(package.as_str()).copied(),
            reviewed,
            &mut report,
        );
    }

    for name in by_package.keys() {
        if !public.contains(*name) {
            report.unexpected.push((*name).to_owned());
        }
    }

    report.status = release_status(publications, &report);
    report
}

/// Collects one publication's version and target mismatches.
fn collect_publication_mismatch(
    package: &str,
    publication: Option<&Publication>,
    reviewed: &BTreeMap<&str, &PlanPackage>,
    report: &mut ReleaseReport,
) {
    let Some(publication) = publication else {
        report.missing.push(package.to_owned());
        return;
    };

    let expected_version = &reviewed[package].version;

    if publication.version != *expected_version {
        report.wrong_versions.push(format!(
            "{package} {} (expected {expected_version})",
            publication.version
        ));
    }

    if publication.ecosystem != "cargo" {
        report
            .wrong_targets
            .push(format!("{package} ecosystem {}", publication.ecosystem));
    }

    if publication.registry != "crates_io" {
        report
            .wrong_targets
            .push(format!("{package} registry {}", publication.registry));
    }

    if publication.mode != "builtin" {
        report
            .wrong_targets
            .push(format!("{package} mode {}", publication.mode));
    }
}

/// Distinguishes exact, empty, and partial publication sets.
fn release_status(publications: &[Publication], report: &ReleaseReport) -> &'static str {
    if publications.is_empty() {
        return "empty";
    }

    let exact = report.missing.is_empty()
        && report.unexpected.is_empty()
        && report.wrong_versions.is_empty()
        && report.duplicates.is_empty()
        && report.wrong_targets.is_empty();

    if exact { "exact" } else { "partial" }
}
