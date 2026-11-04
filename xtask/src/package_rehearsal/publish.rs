//! Records offline publish-dry-run blockers without contacting a registry or uploading bytes.

use super::{Config, model, run_command};
use anyhow::Result;
use serde::Serialize;
use std::{path::Path, process::Command};

#[derive(Serialize)]
pub(super) struct PublishEvidence {
    pub outcomes: Vec<Outcome>,
    pub direct_package: Outcome,
}

#[derive(Serialize)]
pub(super) struct Outcome {
    pub package: String,
    pub status: String,
    pub result: String,
    pub blocker: Option<String>,
}

/// Dry-runs every package and retains the exact real registry prerequisite.
pub(super) fn run(
    config: &Config,
    plan: &model::Plan,
    workspace: &Path,
) -> Result<PublishEvidence> {
    let mut outcomes = Vec::new();

    for target in &plan.targets {
        outcomes.push(publish_one(config, target, workspace)?);
    }

    Ok(PublishEvidence {
        outcomes,
        direct_package: direct(config, workspace)?,
    })
}

/// Runs the dry-run for one exact package and classifies its refusal.
fn publish_one(config: &Config, target: &model::Target, workspace: &Path) -> Result<Outcome> {
    let mut command = cargo(workspace);
    command.args([
        "publish",
        "--dry-run",
        "--offline",
        "--no-verify",
        "-p",
        &target.name,
    ]);
    command.env("CARGO_TARGET_DIR", config.target.join("publish"));
    let output = run_command(config, &mut command, &format!("publish-{}", target.name))?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let (result, blocker) = classify(&stderr, output.status.success());

    Ok(Outcome {
        package: target.name.clone(),
        status: output.status.to_string(),
        result,
        blocker,
    })
}

/// Packages one dependent without a local registry to expose its missing versions.
fn direct(config: &Config, workspace: &Path) -> Result<Outcome> {
    let mut command = cargo(workspace);
    command.args([
        "package",
        "--locked",
        "--offline",
        "--no-verify",
        "-p",
        "solana_kit_client",
    ]);
    command.env("CARGO_TARGET_DIR", config.target.join("publish-direct"));
    let output = run_command(
        config,
        &mut command,
        "package-client-without-local-registry",
    )?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let (result, blocker) = if output.status.success() {
        ("passed".into(), None)
    } else if stderr.contains("no matching package named") {
        ("missing-registry-dependency".into(), Some(missing(&stderr)))
    } else {
        ("failed".into(), Some(missing(&stderr)))
    };

    Ok(Outcome {
        package: "solana_kit_client".into(),
        status: output.status.to_string(),
        result,
        blocker,
    })
}

/// Selects the dependency line that proves the first absent registry package.
fn missing(stderr: &str) -> String {
    stderr
        .lines()
        .find(|line| line.contains("no matching package"))
        .or_else(|| stderr.lines().find(|line| line.starts_with("error:")))
        .unwrap_or("package failed without the intended diagnosis")
        .trim()
        .to_owned()
}

/// Separates policy refusal from the offline registry API requirement.
fn classify(stderr: &str, success: bool) -> (String, Option<String>) {
    if success {
        return ("passed".into(), None);
    }

    if stderr.contains("cannot be published") || stderr.contains("package.publish") {
        return (
            "publish-disabled".into(),
            Some("Cargo.toml retains publish = false".into()),
        );
    }

    if stderr.contains("attempting to make an HTTP request")
        || stderr.contains("--offline was specified")
    {
        return (
            "registry-api-offline".into(),
            Some("offline dry-run requires registry API metadata".into()),
        );
    }

    let summary: Vec<_> = stderr
        .lines()
        .filter(|line| !line.trim().is_empty())
        .take(3)
        .map(|line| line.trim().to_owned())
        .collect();

    ("failed".into(), Some(summary.join("; ")))
}

/// Uses the selected Cargo binary without forwarding any caller flags.
fn cargo(directory: &Path) -> Command {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(directory);
    command
}

#[cfg(test)]
mod tests {
    use super::classify;

    #[test]
    fn classifies_policy_api_and_success_apart() {
        assert_eq!(classify("", true).0, "passed");
        assert_eq!(
            classify("error: `x` cannot be published.", false).0,
            "publish-disabled"
        );
        assert_eq!(
            classify(
                "error: attempting to make an HTTP request, but --offline was specified",
                false
            )
            .0,
            "registry-api-offline"
        );
        assert_eq!(classify("error: other", false).0, "failed");
    }
}
