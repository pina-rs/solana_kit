//! Reviews actual Cargo archives without assuming a package-specific member list.

use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

mod archive;
mod quality;

const MAX_BYTES: u64 = 32 * 1024 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    archive: PathBuf,
    source: PathBuf,
    receipt: PathBuf,
    engine: PathBuf,
    engine_sha256: String,
    policy: PathBuf,
    gzip: PathBuf,
    gzip_sha256: String,
    report: Option<PathBuf>,
}

#[derive(Serialize)]
struct Member {
    path: String,
    kind: &'static str,
    size: usize,
    sha256: String,
    analysis: Option<String>,
    source: Option<String>,
}

/// A fresh receipt binds complete membership, reviewed source and exact artifact bytes.
pub(crate) fn run(path: &Path) -> Result<()> {
    review(path, false)
}

/// Validates report structure and coverage only; it never certifies fresh source quality.
pub(crate) fn check_report(path: &Path) -> Result<()> {
    review(path, true)
}

/// Separates a fresh analysis from the deliberately narrower retained-report check.
fn review(path: &Path, report_only: bool) -> Result<()> {
    let config = configuration(path, report_only)?;
    let original = regular(&config.archive)?;
    let digest = sha256(&original);
    fs::create_dir(&config.receipt).context("receipt must be a new directory")?;

    let members = archive::review(&config)?;
    fs::write(
        config.receipt.join("members.json"),
        serde_json::to_vec_pretty(&members)?,
    )?;
    fs::write(config.receipt.join("archive.sha256"), format!("{digest}\n"))?;
    quality::review(&config, &members)?;
    archive::compare_source(&config, &members)?;
    ensure!(
        regular(&config.archive)? == original,
        "archive changed during review"
    );

    let mode = if report_only {
        "retained-report-only"
    } else {
        "fresh-analysis"
    };

    fs::write(config.receipt.join("mode.txt"), mode)?;
    println!(
        "package-review: {mode}; {} regular members; SHA256 {digest}",
        members.len()
    );

    Ok(())
}

/// Authenticates paths, tools and policy before creating a review receipt.
fn configuration(path: &Path, report_only: bool) -> Result<Config> {
    let config: Config = serde_json::from_slice(&regular(path)?)?;
    ensure!(
        config.report.is_some() == report_only,
        "fresh review requires actual analysis; package-report requires a retained report"
    );
    ensure!(
        [
            &config.archive,
            &config.source,
            &config.receipt,
            &config.engine,
            &config.policy,
            &config.gzip
        ]
        .iter()
        .all(|path| path.is_absolute()),
        "review paths must be absolute"
    );

    ensure!(
        fs::symlink_metadata(&config.source)?.is_dir(),
        "source root must be a real directory"
    );
    authenticate(&config.engine, &config.engine_sha256)?;
    authenticate(&config.gzip, &config.gzip_sha256)?;
    ensure!(
        regular(&config.policy)? == include_bytes!("../../monostyle.toml"),
        "SDK policy differs"
    );

    Ok(config)
}

/// Limits retained input bytes and rejects symlinks before reading them.
fn regular(path: &Path) -> Result<Vec<u8>> {
    let metadata =
        fs::symlink_metadata(path).with_context(|| format!("inspect {}", path.display()))?;
    ensure!(
        metadata.is_file() && metadata.len() <= MAX_BYTES,
        "expected bounded regular file: {}",
        path.display()
    );
    Ok(fs::read(path)?)
}

/// Requires literal regular tool paths and the admitted executable bytes.
fn authenticate(path: &Path, expected: &str) -> Result<()> {
    ensure!(path.is_absolute(), "tool path must be absolute");
    ensure!(
        sha256(&regular(path)?) == expected,
        "tool bytes differ: {}",
        path.display()
    );
    Ok(())
}

/// Computes a digest of actual bytes, independently of filesystem metadata.
fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Preserves a failed child diagnostic rather than accepting partial output.
fn output(command: &mut Command) -> Result<Vec<u8>> {
    let output = command.output().context("execute archive review tool")?;
    ensure!(
        output.status.success(),
        "review child failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(output.stdout)
}
