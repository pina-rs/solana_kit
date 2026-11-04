//! Explicit validator discovery and pinned version verification.

use crate::node::NodeError;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const BINARY_NAME: &str = "solana-test-validator";
const BINARY_OVERRIDE: &str = "SOLANA_TEST_VALIDATOR";
const EXPECTED_VERSION_PREFIX: &str = "4.2.";

/// Prefers the explicit override, then the installed release, then PATH.
///
/// A compatible 4.2 candidate wins even when an unrelated newer binary appears
/// earlier on PATH; an explicit override is never silently replaced.
pub(crate) fn locate_validator() -> Result<(PathBuf, String), NodeError> {
    let override_path = env::var_os(BINARY_OVERRIDE).map(PathBuf::from);
    let candidates = candidates(&override_path)?;
    let mut wrong = Vec::new();

    for candidate in &candidates {
        let Some(path) = explicit(candidate) else {
            continue;
        };

        match checked_version(&path) {
            Ok(version) => return Ok((path, version)),

            Err(NodeError::WrongBinary(version)) if override_path.is_some() => {
                return Err(NodeError::WrongBinary(version));
            }

            Err(NodeError::WrongBinary(version)) => {
                wrong.push(format!("{}: {version}", path.display()));
            }
            Err(error) => return Err(error),
        }
    }

    if wrong.is_empty() {
        return Err(NodeError::MissingBinary(searched(&candidates)));
    }

    Err(NodeError::WrongBinary(wrong.join("; ")))
}

fn candidates(override_path: &Option<PathBuf>) -> Result<Vec<PathBuf>, NodeError> {
    if let Some(path) = override_path {
        return override_candidate(path);
    }

    let mut candidates = Vec::new();

    if let Some(home) = env::var_os("HOME") {
        candidates.push(
            PathBuf::from(home)
                .join(".local/share/solana/install/active_release/bin")
                .join(BINARY_NAME),
        );
    }

    if let Some(paths) = env::var_os("PATH") {
        candidates.extend(env::split_paths(&paths).map(|directory| directory.join(BINARY_NAME)));
    }

    candidates.dedup();

    Ok(candidates)
}

fn override_candidate(path: &Path) -> Result<Vec<PathBuf>, NodeError> {
    if explicit(path).is_none() {
        return Err(NodeError::MissingBinary(format!(
            "{BINARY_OVERRIDE}={} is not an executable file",
            path.display()
        )));
    }

    Ok(vec![path.to_path_buf()])
}

fn searched(candidates: &[PathBuf]) -> String {
    candidates
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn explicit(path: &Path) -> Option<PathBuf> {
    let metadata = fs::metadata(path).ok()?;

    if !metadata.is_file() {
        return None;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        if metadata.permissions().mode() & 0o111 == 0 {
            return None;
        }
    }

    Some(path.to_path_buf())
}

fn checked_version(binary: &Path) -> Result<String, NodeError> {
    let output = Command::new(binary)
        .arg("--version")
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .output()?;

    if !output.status.success() {
        return Err(NodeError::WrongBinary(format!(
            "{} exited with {}",
            binary.display(),
            output.status
        )));
    }

    let text = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let version = text
        .strip_prefix("solana-test-validator ")
        .and_then(|rest| rest.split_whitespace().next())
        .ok_or_else(|| NodeError::WrongBinary(text.clone()))?;

    if !version.starts_with(EXPECTED_VERSION_PREFIX) {
        return Err(NodeError::WrongBinary(text));
    }

    Ok(text)
}
