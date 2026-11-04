//! Hashes the exact locked source inputs independently of a Git checkout.

use anyhow::Result;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Hashes every SDK source file plus the workspace lock and release policy.
pub(super) fn digest(workspace: &Path) -> Result<String> {
    let mut files = Vec::new();
    collect(&workspace.join("crates"), &mut files)?;

    for name in [
        "Cargo.toml",
        "Cargo.lock",
        "monochange.toml",
        "rust-toolchain.toml",
    ] {
        files.push(workspace.join(name));
    }

    files.sort();
    let mut digest = Sha256::new();

    for path in files {
        let relative = path.strip_prefix(workspace).unwrap_or(&path);
        let bytes = fs::read(&path)?;
        digest.update(relative.to_string_lossy().as_bytes());
        digest.update((bytes.len() as u64).to_le_bytes());
        digest.update(bytes);
    }

    Ok(format!("{:x}", digest.finalize()))
}

/// Collects regular source files while rejecting aliases in the reviewed tree.
fn collect(directory: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    if !directory.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let kind = entry.file_type()?;

        if kind.is_dir() {
            collect(&entry.path(), files)?;
        } else if kind.is_file() {
            files.push(entry.path());
        } else {
            anyhow::bail!("source tree contains a non-regular entry");
        }
    }

    Ok(())
}

/// Returns Git's exact HEAD when the source snapshot is a checkout.
pub(super) fn revision(workspace: &Path) -> Result<Option<String>> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(workspace)
        .args(["rev-parse", "HEAD"])
        .output()?;

    if !output.status.success() {
        return Ok(None);
    }

    Ok(Some(String::from_utf8(output.stdout)?.trim().to_owned()))
}
