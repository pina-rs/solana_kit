//! Derives SDK package roots, publication state and dependency order from Cargo metadata.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

mod blocking;
mod lockfile;
mod order;
mod planning;
mod workspace;

pub(super) use lockfile::{LockedPackage, closure_lockfile, parse_lockfile};
pub(super) use planning::plan;

/// Exact Cargo workspace metadata used by the rehearsal.
#[derive(Deserialize, Serialize)]
pub(super) struct Metadata {
    pub packages: Vec<Package>,
    pub workspace_members: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub(super) struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf,
    pub publish: Option<Vec<String>>,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

#[derive(Deserialize, Serialize)]
pub(super) struct Dependency {
    pub name: String,
    pub source: Option<String>,
    pub path: Option<PathBuf>,
    pub kind: Option<String>,
}

/// How an SDK package may be published after the external prerequisites are met.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum Publication {
    Publishable,
    PublishDisabled,
    Restricted(Vec<String>),
}

/// One included package in dependency-first order.
#[derive(Serialize)]
pub(super) struct Target {
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf,
    pub source: PathBuf,
    pub publication: Publication,
    pub dependencies: Vec<String>,
}

/// A workspace package that cannot enter the local archive rehearsal.
#[derive(Serialize)]
pub(super) struct Excluded {
    pub name: String,
    pub version: String,
    pub reason: String,
    pub dependency_source: Option<String>,
}

#[derive(Serialize)]
pub(super) struct Plan {
    pub roots: Vec<PathBuf>,
    pub targets: Vec<Target>,
    pub excluded: Vec<Excluded>,
}

impl Plan {
    /// Returns one included target by exact metadata name.
    pub(super) fn target(&self, name: &str) -> Option<&Target> {
        self.targets.iter().find(|target| target.name == name)
    }

    /// Requires a consumer dependency to have been included and ordered.
    pub(super) fn require(&self, name: &str) -> Result<&Target> {
        self.target(name)
            .with_context(|| format!("consumer package is not in the rehearsal: {name}"))
    }
}

/// Reads workspace-only metadata, avoiding network and git dependency checkouts.
pub(super) fn metadata(
    config: &super::Config,
    workspace: &Path,
    evidence: &Path,
) -> Result<Metadata> {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(workspace).args([
        "metadata",
        "--format-version",
        "1",
        "--no-deps",
        "--locked",
        "--offline",
    ]);
    let output = super::success(config, &mut command, "workspace-metadata")?;
    fs::write(evidence.join("metadata.json"), &output.stdout)?;
    serde_json::from_slice(&output.stdout).context("parse Cargo metadata")
}
