//! Packages each included SDK crate in dependency order and retains reviewed archive bytes.

use super::{Config, model, success};
use crate::package_review;
use anyhow::{Result, ensure};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Serialize)]
pub(super) struct ReviewedArchive {
    pub name: String,
    pub version: String,
    pub archive: PathBuf,
    pub sha256: String,
    pub review: Review,
    pub members: usize,
    pub readability: Option<f64>,
    pub complexity: Option<f64>,
    pub limitation: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum Review {
    FreshAnalysis,
    GateUnsupported,
}

/// One archive's review result before it is bound into the evidence record.
struct Reviewed {
    review: Review,
    members: usize,
    readability: Option<f64>,
    complexity: Option<f64>,
    limitation: Option<String>,
}

/// Builds actual `.crate` files and runs the maintained archive gate on each.
pub(super) fn run(
    config: &Config,
    plan: &model::Plan,
    evidence: &Path,
) -> Result<Vec<ReviewedArchive>> {
    let archives = evidence.join("archives");
    let reviews = evidence.join("reviews");
    fs::create_dir(&archives)?;
    fs::create_dir(&reviews)?;
    let mut results = Vec::new();

    for target in &plan.targets {
        results.push(build(config, target, &archives, &reviews)?);
    }

    Ok(results)
}

/// Creates, copies and reviews one exact target archive.
fn build(
    config: &Config,
    target: &model::Target,
    archives: &Path,
    reviews: &Path,
) -> Result<ReviewedArchive> {
    let filename = format!("{}-{}.crate", target.name, target.version);
    let packaged = package(config, target, &filename)?;
    let archive = archives.join(&filename);
    fs::copy(&packaged, &archive)?;
    let sha256 = digest(&archive)?;
    let reviewed = review(config, target, &archive, &reviews.join(&target.name))?;

    Ok(ReviewedArchive {
        name: target.name.clone(),
        version: target.version.clone(),
        archive,
        sha256,
        review: reviewed.review,
        members: reviewed.members,
        readability: reviewed.readability,
        complexity: reviewed.complexity,
        limitation: reviewed.limitation,
    })
}

/// Executes Cargo for one package and returns its actual archive path.
fn package(config: &Config, target: &model::Target, filename: &str) -> Result<PathBuf> {
    let packaged = config.target.join("package").join(filename);
    let _ = fs::remove_file(&packaged);
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(&config.workspace).args([
        "package",
        "--locked",
        "--offline",
        "--exclude-lockfile",
        "--no-verify",
        "-p",
        &target.name,
    ]);
    command.env("CARGO_TARGET_DIR", &config.target);
    success(config, &mut command, &format!("package-{}", target.name))?;
    ensure!(
        fs::symlink_metadata(&packaged)?.is_file(),
        "Cargo did not create the exact archive: {filename}"
    );

    Ok(packaged)
}

/// Runs the maintained gate or records only an explicit archive-form limitation.
fn review(
    config: &Config,
    target: &model::Target,
    archive: &Path,
    receipt: &Path,
) -> Result<Reviewed> {
    fs::create_dir(receipt)?;
    let review_config = receipt.join("config.json");
    fs::write(
        &review_config,
        serde_json::to_vec_pretty(&serde_json::json!({
            "archive": archive,
            "source": target.source,
            "receipt": receipt.join("receipt"),
            "engine": config.engine,
            "engine_sha256": config.engine_sha256,
            "policy": config.policy,
            "gzip": config.gzip,
            "gzip_sha256": config.gzip_sha256,
        }))?,
    )?;

    match package_review::run(&review_config) {
        Ok(()) => fresh(receipt),
        Err(error) if unsupported(&error) => Ok(Reviewed {
            review: Review::GateUnsupported,
            members: 0,
            readability: None,
            complexity: None,
            limitation: Some(error.to_string()),
        }),
        Err(error) => Err(error),
    }
}

/// Reads the actual receipt left by a successful fresh review.
fn fresh(receipt: &Path) -> Result<Reviewed> {
    let receipt = receipt.join("receipt");
    let members = receipt.join("members.json");
    let quality = receipt.join("quality.json");

    Ok(Reviewed {
        review: Review::FreshAnalysis,
        members: serde_json::from_slice::<Vec<serde_json::Value>>(&fs::read(members)?)?.len(),
        readability: Some(score(&quality, "readability")?),
        complexity: Some(score(&quality, "complexity")?),
        limitation: None,
    })
}

/// Recognizes only explicit maintained-gate form limitations, never a low score.
fn unsupported(error: &anyhow::Error) -> bool {
    let message = error.to_string();

    message.contains("unsupported archive filename")
        || message.contains("GNU long-name extension records")
}

/// Reads one aggregate score from the retained analyzer report.
fn score(path: &Path, category: &str) -> Result<f64> {
    let report: serde_json::Value = serde_json::from_slice(&fs::read(path)?)?;

    report
        .get(category)
        .and_then(|value| value.get("value"))
        .and_then(serde_json::Value::as_f64)
        .filter(|value| value.is_finite())
        .ok_or_else(|| anyhow::anyhow!("archive review lacks a numeric {category} score"))
}

/// Hashes actual archive bytes after copying them into owned evidence custody.
fn digest(path: &Path) -> Result<String> {
    ensure!(
        fs::symlink_metadata(path)?.is_file(),
        "archive is not regular"
    );

    Ok(format!("{:x}", Sha256::digest(fs::read(path)?)))
}
