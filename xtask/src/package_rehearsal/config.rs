//! Authenticates the absolute paths and tool identities of one rehearsal.

use anyhow::{Result, ensure};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Owned, absolute paths and exact tool identities for one rehearsal.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    pub workspace: PathBuf,
    pub evidence: PathBuf,
    pub target: PathBuf,
    pub registry: PathBuf,
    pub consumer: PathBuf,
    pub cargo_home: PathBuf,
    pub cache: PathBuf,
    pub sparse_index: PathBuf,
    pub engine: PathBuf,
    pub engine_sha256: String,
    pub policy: PathBuf,
    pub gzip: PathBuf,
    pub gzip_sha256: String,
    pub msrv: Option<String>,
    #[serde(default)]
    pub wasm_targets: Vec<String>,
    #[serde(default = "enabled")]
    pub publish_dry_run: bool,
}

/// Defaults a newly added rehearsal stage to enabled.
fn enabled() -> bool {
    true
}

/// Authenticates every input before the first packaging or consumer side effect.
pub(super) fn validate(config: &Config) -> Result<()> {
    let paths = [
        &config.workspace,
        &config.evidence,
        &config.target,
        &config.registry,
        &config.consumer,
        &config.cargo_home,
        &config.cache,
        &config.sparse_index,
        &config.engine,
        &config.policy,
        &config.gzip,
    ];
    ensure!(
        paths.iter().all(|path| path.is_absolute()),
        "rehearsal paths must be absolute"
    );
    outside(config)?;
    fresh(config)?;
    tools(config)?;
    ensure!(
        fs::read(&config.policy)? == include_bytes!("../../../monostyle.toml"),
        "rehearsal policy differs from the maintained SDK policy"
    );

    Ok(())
}

/// Keeps every generated tree outside the source checkout.
fn outside(config: &Config) -> Result<()> {
    let workspace = config.workspace.canonicalize()?;

    for path in [
        &config.evidence,
        &config.target,
        &config.registry,
        &config.consumer,
        &config.cargo_home,
        &config.cache,
        &config.sparse_index,
    ] {
        ensure!(
            !path.starts_with(&workspace),
            "rehearsal data must stay outside the source tree: {}",
            path.display()
        );
    }

    Ok(())
}

/// Requires new output namespaces and an existing credential-free Cargo home.
fn fresh(config: &Config) -> Result<()> {
    ensure!(
        !config.evidence.exists(),
        "evidence directory must be fresh"
    );
    ensure!(
        !config.registry.exists(),
        "registry destination must be fresh"
    );
    ensure!(
        !config.consumer.exists(),
        "consumer directory must be fresh"
    );
    ensure!(config.cargo_home.is_dir(), "Cargo home must already exist");
    ensure!(
        !config.cargo_home.join("credentials.toml").exists()
            && !config.cargo_home.join("credentials").exists(),
        "Cargo home must not contain registry credentials"
    );
    ensure!(config.cache.is_dir(), "crate cache must be a directory");
    ensure!(
        config.sparse_index.is_dir(),
        "sparse index must be a directory"
    );

    Ok(())
}

/// Requires the admitted analyzer, gzip and policy bytes.
fn tools(config: &Config) -> Result<()> {
    authenticate(&config.engine, &config.engine_sha256)?;
    authenticate(&config.gzip, &config.gzip_sha256)?;
    Ok(())
}

/// Requires literal regular tool bytes instead of trusting PATH lookup.
fn authenticate(path: &Path, expected: &str) -> Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    ensure!(metadata.is_file(), "tool is not a regular file");
    ensure!(
        format!("{:x}", Sha256::digest(fs::read(path)?)) == expected,
        "tool bytes differ: {}",
        path.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Config, validate};

    fn config() -> Config {
        serde_json::from_str(
            r#"{"workspace":"/workspace","evidence":"/evidence","target":"/target",
            "registry":"/registry","consumer":"/consumer","cargo_home":"/cargo",
            "cache":"/cache","sparse_index":"/index","engine":"/engine",
            "engine_sha256":"00","policy":"/policy","gzip":"/gzip",
            "gzip_sha256":"00","msrv":null,"wasm_targets":[]}"#,
        )
        .unwrap()
    }

    #[test]
    fn publish_dry_run_defaults_on_and_unknown_fields_fail() {
        assert!(config().publish_dry_run);
        assert!(
            serde_json::from_str::<Config>(
                r#"{"workspace":"/workspace","evidence":"/evidence","target":"/target",
                "registry":"/registry","consumer":"/consumer","cargo_home":"/cargo",
                "cache":"/cache","sparse_index":"/index","engine":"/engine",
                "engine_sha256":"00","policy":"/policy","gzip":"/gzip",
                "gzip_sha256":"00","msrv":null,"wasm_targets":[],"unexpected":true}"#
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_relative_and_in_repository_paths_before_side_effects() {
        let mut config = config();
        assert!(validate(&config).is_err());
        config.workspace = "/workspace".into();
        config.evidence = "/workspace/evidence".into();
        assert!(validate(&config).is_err());
    }
}
