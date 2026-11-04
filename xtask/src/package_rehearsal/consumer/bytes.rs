//! Compares unpacked registry source headers with reviewed archive members.

use super::super::{Config, archives};
use anyhow::{Context, Result, ensure};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

/// Requires every reviewed member to match the unpacked registry source header.
pub(super) fn compare(
    config: &Config,
    name: &str,
    archive: &archives::ReviewedArchive,
    manifest: &Path,
) -> Result<usize> {
    let receipt = config.evidence.join("reviews").join(name).join("receipt");
    let members: Vec<serde_json::Value> =
        serde_json::from_slice(&fs::read(receipt.join("members.json"))?)?;
    let root = manifest.parent().context("manifest has no parent")?;

    for member in &members {
        let relative = member
            .get("path")
            .and_then(serde_json::Value::as_str)
            .context("archive member has no path")?;
        let expected = member
            .get("sha256")
            .and_then(serde_json::Value::as_str)
            .context("archive member has no digest")?;
        let path = root.join(relative);
        ensure!(
            fs::symlink_metadata(&path)?.is_file(),
            "unpacked member is not regular: {relative}"
        );
        ensure!(
            format!("{:x}", Sha256::digest(fs::read(&path)?)) == expected,
            "unpacked member differs from reviewed archive: {name}/{relative}"
        );
    }

    ensure!(
        fs::read(receipt.join("archive.sha256"))?
            .iter()
            .copied()
            .take_while(|byte| *byte != b'\n')
            .eq(archive.sha256.bytes()),
        "reviewed archive digest differs"
    );

    Ok(members.len())
}
