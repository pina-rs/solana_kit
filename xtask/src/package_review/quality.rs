//! Requires finite per-file and aggregate floors for the complete archive partition.

use super::{Config, Member, output, regular};
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};

const FLOOR: f64 = 90.0;
const EXTENSIONS: &str = concat!(
    "c h cpp cc cxx hpp hh hxx cs java js mjs cjs kt kts mozjs py pyi rs ts mts cts tsx ",
    "dart go swift rb podspec php scala sc sh bash zsh lua ex exs hs nix yaml yml ",
    "properties md markdown",
);

#[derive(Deserialize)]
struct Score {
    value: f64,
}

#[derive(Deserialize)]
struct File {
    path: String,
    readability: Score,
    complexity: Score,
}

#[derive(Deserialize)]
struct Report {
    files: Vec<File>,
    skipped: Vec<String>,
    readability: Score,
    complexity: Score,
}

/// Uses byte-identical language-suffixed mirrors for source templates.
pub(super) fn analysis_path(path: &str) -> Option<String> {
    let mut candidate = path;

    while let Some((stem, extension)) = candidate.rsplit_once('.') {
        if EXTENSIONS
            .split_whitespace()
            .any(|known| known.eq_ignore_ascii_case(extension))
        {
            return Some(candidate.to_owned());
        }

        if !matches!(extension, "txt" | "in" | "tmpl" | "template") {
            return None;
        }

        candidate = stem;
    }

    None
}

/// Includes every actual member, even unsupported metadata, in the analyzer invocation.
pub(super) fn review(config: &Config, members: &[Member]) -> Result<()> {
    let report = config.receipt.join("quality.json");

    if let Some(existing) = &config.report {
        fs::write(&report, regular(existing)?)?;
    } else {
        analyze(config, members)?;
    }

    validate(&regular(&report)?, members)?;
    Ok(())
}

/// Records the effective policy and raw failed output before checking the status.
fn analyze(config: &Config, members: &[Member]) -> Result<()> {
    let effective = output(
        Command::new(&config.engine)
            .arg("config")
            .arg("--config")
            .arg(&config.policy)
            .args(["--format", "toml"])
            .current_dir(&config.receipt),
    )?;
    fs::write(config.receipt.join("effective-monostyle.toml"), effective)?;
    let mut command = Command::new(&config.engine);
    command.current_dir(&config.receipt).arg("check");
    command.arg("--config").arg(&config.policy);
    command.args([
        "--no-ignore",
        "--include-generated",
        "--fail-under",
        "90",
        "--format",
        "json",
        "--no-color",
        "--output",
        "quality.json",
        "--",
    ]);

    for member in members {
        let path = match &member.analysis {
            Some(path) => Path::new("analysis").join(path),
            None => Path::new("artifact").join(&member.path),
        };
        command.arg(path);
    }

    let result = command
        .output()
        .context("execute complete archive quality")?;
    fs::write(config.receipt.join("quality.stdout"), result.stdout)?;
    fs::write(config.receipt.join("quality.stderr"), result.stderr)?;
    ensure!(
        result.status.success(),
        "archive analyzer failed; inspect retained quality output"
    );
    Ok(())
}

/// Treats omitted, skipped, duplicate and unrelated rows as failures independent of averages.
fn validate(bytes: &[u8], members: &[Member]) -> Result<()> {
    let report: Report = serde_json::from_slice(bytes)?;
    floor("repository", &report.readability, &report.complexity)?;
    let expected: BTreeMap<_, _> = members
        .iter()
        .map(|member| {
            let path = match &member.analysis {
                Some(path) => format!("analysis/{path}"),
                None => format!("artifact/{}", member.path),
            };
            (path, member.analysis.is_some())
        })
        .collect();
    ensure!(
        expected.len() == members.len(),
        "conflicting analysis mirror paths"
    );
    let mut covered = BTreeSet::new();

    for file in report.files {
        let path = file.path.strip_prefix("./").unwrap_or(&file.path);
        ensure!(
            expected.get(path) == Some(&true),
            "unexpected scored member: {path}"
        );
        ensure!(
            covered.insert(path.to_owned()),
            "duplicate score row: {path}"
        );
        floor(path, &file.readability, &file.complexity)?;
    }

    for path in report.skipped {
        let path = path.strip_prefix("./").unwrap_or(&path);
        ensure!(
            expected.get(path) == Some(&false),
            "supported member was skipped: {path}"
        );
        ensure!(
            covered.insert(path.to_owned()),
            "duplicate skipped row: {path}"
        );
    }

    ensure!(
        covered == expected.into_keys().collect(),
        "score report does not cover exact archive membership"
    );
    Ok(())
}

/// Each category has its own floor; a high average cannot conceal a low file score.
fn floor(path: &str, readability: &Score, complexity: &Score) -> Result<()> {
    for score in [readability.value, complexity.value] {
        ensure!(
            score.is_finite() && (FLOOR..=100.0).contains(&score),
            "{path}: score {score} is outside {FLOOR}..=100"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{analysis_path, validate};
    use crate::package_review::Member;
    use serde_json::json;

    #[test]
    fn recognizes_source_templates_without_scoring_metadata() {
        assert_eq!(analysis_path("helper.rs.txt"), Some("helper.rs".into()));
        assert_eq!(analysis_path("README.md"), Some("README.md".into()));
        assert_eq!(analysis_path("Cargo.toml.orig"), None);
    }

    #[test]
    fn rejects_missing_low_duplicate_and_skipped_supported_rows() {
        let member = Member {
            path: "CHANGELOG.md".into(),
            kind: "regular",
            size: 0,
            sha256: String::new(),
            analysis: Some("CHANGELOG.md".into()),
            source: None,
        };
        let members = [member];
        let score = json!({"value": 100.0});
        let file =
            json!({"path": "analysis/CHANGELOG.md", "readability": score, "complexity": score});
        let report =
            json!({"files": [file], "skipped": [], "readability": score, "complexity": score});
        assert!(validate(&serde_json::to_vec(&report).unwrap(), &members).is_ok());
        let mut missing = report.clone();
        missing["files"] = json!([]);
        assert!(validate(&serde_json::to_vec(&missing).unwrap(), &members).is_err());
        let mut low = report.clone();
        low["files"][0]["readability"]["value"] = json!(89.0);
        assert!(validate(&serde_json::to_vec(&low).unwrap(), &members).is_err());
        let mut duplicate = report.clone();
        duplicate["files"] = json!([file, file]);
        assert!(validate(&serde_json::to_vec(&duplicate).unwrap(), &members).is_err());
        let mut skipped = missing;
        skipped["skipped"] = json!(["analysis/CHANGELOG.md"]);
        assert!(validate(&serde_json::to_vec(&skipped).unwrap(), &members).is_err());
    }
}
