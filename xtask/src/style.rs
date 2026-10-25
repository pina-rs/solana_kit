//! Enforces repository and individual file floors from the pinned monostyle report.

use crate::process;
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use std::{collections::BTreeSet, path::Path, process::Command};

mod coverage;

const VERSION: &str = "monostyle 0.3.27";
const FLOOR: f64 = 90.0;
const REPORT: &str = "target/style-evidence/report.json";

#[derive(Deserialize)]
struct Score {
    value: f64,
}

#[derive(Deserialize)]
struct FileReport {
    path: String,
    readability: Score,
    complexity: Score,
    total_lines: usize,
    code_lines: usize,
}

#[derive(Deserialize)]
struct Report {
    files: Vec<FileReport>,
    readability: Score,
    complexity: Score,
    total_lines: usize,
    code_lines: usize,
}

/// Checks the installed version before executing a fresh report with generated files included.
pub(crate) fn run(root: Option<&Path>) -> Result<()> {
    if let Some(directory) = root {
        std::env::set_current_dir(directory).context("enter style repository")?;
    }

    let version = process::success(Command::new("monostyle").arg("--version"), "style-version")?;
    require_version(&version.stdout)?;
    let inventory = coverage::inventory()?;
    std::fs::create_dir_all("target/style-evidence")?;
    process::success(
        Command::new("monostyle")
            .args([
                "check",
                "--include-generated",
                "--fail-under",
                "90",
                "--format",
                "json",
                "--output",
                REPORT,
                "--no-color",
                "--",
            ])
            .args(&inventory),
        "style-check",
    )?;

    let report = read_report(Path::new(REPORT))?;
    coverage::check(&report, &inventory)?;
    print_report(&report);

    Ok(())
}

/// Allows independent validation of a retained report without rerunning the analyzer.
pub(crate) fn check_report(path: &Path) -> Result<()> {
    let report = read_report(path)?;
    print_report(&report);

    Ok(())
}

/// Reads complete JSON; missing fields and malformed or trailing data are errors.
fn read_report(path: &Path) -> Result<Report> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    validate(&bytes).with_context(|| format!("validate {}", path.display()))
}

/// Reports success only after every applicable score and coverage check passes.
fn print_report(report: &Report) {
    println!(
        "style: {} files; repository readability {:.2}, complexity {:.2}; every category >= {FLOOR}",
        report.files.len(),
        report.readability.value,
        report.complexity.value,
    );
}

/// Rejects every other analyzer before inventory discovery or report generation.
fn require_version(output: &[u8]) -> Result<()> {
    ensure!(
        std::str::from_utf8(output)?.trim() == VERSION,
        "style gate requires {VERSION}"
    );

    Ok(())
}

/// Required cells mirror the pinned analyzer's ProjectReport and FileReport score objects.
fn validate(bytes: &[u8]) -> Result<Report> {
    let report: Report = serde_json::from_slice(bytes).context("parse monostyle JSON report")?;
    let mut paths = BTreeSet::new();
    ensure!(
        !report.files.is_empty(),
        "monostyle report has no scored files"
    );
    check_scores("repository", &report.readability, &report.complexity)?;

    for file in &report.files {
        ensure!(!file.path.is_empty(), "scored file has an empty path");
        ensure!(
            paths.insert(&file.path),
            "duplicate scored file: {}",
            file.path
        );
        check_scores(&file.path, &file.readability, &file.complexity)?;
    }

    ensure!(
        report
            .files
            .iter()
            .try_fold(0usize, |total, file| total.checked_add(file.total_lines))
            .context("scored file total_lines overflow")?
            == report.total_lines,
        "scored file total_lines do not match repository total"
    );
    ensure!(
        report
            .files
            .iter()
            .try_fold(0usize, |total, file| total.checked_add(file.code_lines))
            .context("scored file code_lines overflow")?
            == report.code_lines,
        "scored file code_lines do not match repository total"
    );

    Ok(report)
}

/// Both categories have independent floors; averaging cannot conceal a failing category.
fn check_scores(path: &str, readability: &Score, complexity: &Score) -> Result<()> {
    for (category, score) in [("readability", readability), ("complexity", complexity)] {
        ensure!(
            score.value.is_finite() && (FLOOR..=100.0).contains(&score.value),
            "{path}: {category} {} is outside {FLOOR}..=100",
            score.value
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests;
