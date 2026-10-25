//! Cargo source identity must match across generated paths and SDK Git pins.
use super::Result;
use std::{fmt::Write, path::Path, process::Command};

const PACKAGES: [&str; 4] = [
    "leptoskit_bridge",
    "leptoskit_bridge_ffi",
    "leptoskit_native",
    "leptoskit_wasm",
];

/// Git and path dependencies with identical bytes still have different Rust
/// identities; bind both to the exact admitted clean public framework tree.
pub(super) fn patches(framework: &Path, expected: &str) -> Result<String> {
    let output = Command::new("/usr/bin/git")
        .arg("-C")
        .arg(framework)
        .args(["rev-parse", "HEAD"])
        .output()?;
    let head = std::str::from_utf8(&output.stdout)?.trim();

    if !output.status.success() || head != expected || head.len() != 40 {
        return Err("framework source differs from the explicitly admitted commit".into());
    }

    let status = Command::new("/usr/bin/git")
        .arg("-C")
        .arg(framework)
        .args(["status", "--porcelain=v1", "--untracked-files=no"])
        .output()?;

    if !status.status.success() || !status.stdout.is_empty() {
        return Err("framework source must be the clean admitted public tree".into());
    }

    let mut patch = String::new();

    for source in ["crates-io", "\"https://github.com/ifiokjr/leptoskit\""] {
        writeln!(patch, "\n[patch.{source}]")?;

        for package in PACKAGES {
            let directory = framework.join("crates").join(package);

            if !directory.join("Cargo.toml").is_file() {
                return Err("admitted framework is missing a required public package".into());
            }

            writeln!(patch, "{package} = {{ path = {directory:?} }}")?;
        }
    }

    Ok(patch)
}

pub(super) fn workspace(contents: &str, patch: &str) -> Result<String> {
    if contents.contains(patch) {
        return Ok(contents.to_owned());
    }

    if contents.contains("[patch.") {
        return Err("review existing Cargo source patches before adapting this fixture".into());
    }

    Ok(format!("{contents}{patch}"))
}
