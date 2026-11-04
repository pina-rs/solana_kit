//! Authenticates the absolute paths and tool identities of one rehearsal.

use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Component, Path, PathBuf},
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
    ensure!(workspace.is_dir(), "workspace must be a directory");

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
            !resolved(path)?.starts_with(&workspace),
            "rehearsal data must stay outside the source tree: {}",
            path.display()
        );
    }

    let outputs = [
        &config.evidence,
        &config.target,
        &config.registry,
        &config.consumer,
        &config.cargo_home,
    ]
    .into_iter()
    .map(|path| resolved(path))
    .collect::<Result<Vec<_>>>()?;

    for (index, path) in outputs.iter().enumerate() {
        ensure!(
            outputs[index + 1..]
                .iter()
                .all(|other| { !path.starts_with(other) && !other.starts_with(path) }),
            "rehearsal output namespaces must not overlap"
        );
    }

    Ok(())
}

/// Resolves existing ancestors before appending any not-yet-created components.
fn resolved(path: &Path) -> Result<PathBuf> {
    ensure!(
        !path
            .components()
            .any(|part| matches!(part, Component::ParentDir)),
        "rehearsal paths must not contain parent traversal"
    );
    let mut ancestor = path;
    let mut missing = Vec::new();

    loop {
        match fs::symlink_metadata(ancestor) {
            Ok(_) => break,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                missing.push(ancestor.file_name().ok_or(error)?);
                ancestor = ancestor.parent().context("rehearsal path has no parent")?;
            }
            Err(error) => return Err(error.into()),
        }
    }

    let mut path = ancestor.canonicalize()?;

    for part in missing.into_iter().rev() {
        path.push(part);
    }

    Ok(path)
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
    for name in [
        "credentials.toml",
        "credentials",
        "config.toml",
        "config",
        "registry/src",
    ] {
        match fs::symlink_metadata(config.cargo_home.join(name)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
            Ok(_) => anyhow::bail!("Cargo home must not contain {name}"),
        }
    }
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
    use super::{Config, fresh, outside, validate};

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

    #[cfg(unix)]
    #[test]
    fn output_paths_cannot_alias_the_source_or_each_other() {
        use std::{fs, os::unix::fs::symlink};

        let directory =
            std::env::temp_dir().join(format!("rehearsal-paths-{}", std::process::id()));
        fs::create_dir(&directory).unwrap();
        let workspace = directory.join("workspace");
        fs::create_dir(&workspace).unwrap();
        let mut config = config();
        config.workspace = workspace.clone();
        config.evidence = directory.join("evidence");
        config.target = directory.join("target");
        config.registry = directory.join("registry");
        config.consumer = directory.join("consumer");
        config.cargo_home = directory.join("cargo");
        config.cache = directory.join("cache");
        config.sparse_index = directory.join("index");
        outside(&config).unwrap();
        config.evidence = workspace.join("new-evidence");
        let inside = outside(&config);
        let alias = directory.join("alias");
        symlink(&workspace, &alias).unwrap();
        config.evidence = alias.join("new-evidence");
        let escaped = outside(&config);
        config.evidence = directory.join("nested/../workspace/evidence");
        let traversal = outside(&config);
        config.evidence = config.target.join("evidence");
        let overlap = outside(&config);
        fs::remove_dir_all(&directory).unwrap();
        assert!(inside.is_err(), "source child was accepted");
        assert!(escaped.is_err(), "source alias was accepted");
        assert!(traversal.is_err(), "parent traversal was accepted");
        assert!(overlap.is_err(), "overlapping outputs were accepted");
    }

    #[test]
    fn cargo_home_admits_caches_but_not_configuration_or_extracted_sources() {
        use std::fs;

        let directory = std::env::temp_dir().join(format!("rehearsal-home-{}", std::process::id()));
        fs::create_dir(&directory).unwrap();
        let mut config = config();
        config.evidence = directory.join("evidence");
        config.registry = directory.join("registry");
        config.consumer = directory.join("consumer");
        config.cargo_home = directory.join("home");
        config.cache = config.cargo_home.join("registry/cache");
        config.sparse_index = config.cargo_home.join("registry/index");
        fs::create_dir_all(&config.cache).unwrap();
        fs::create_dir_all(&config.sparse_index).unwrap();
        fresh(&config).unwrap();
        let mut rejected = Vec::new();

        for name in ["config.toml", "credentials.toml"] {
            let path = config.cargo_home.join(name);
            fs::write(&path, b"fixture").unwrap();
            rejected.push(fresh(&config).is_err());
            fs::remove_file(path).unwrap();
        }

        fs::create_dir(config.cargo_home.join("registry/src")).unwrap();
        rejected.push(fresh(&config).is_err());
        fs::remove_dir_all(directory).unwrap();
        assert!(rejected.into_iter().all(|value| value));
    }
}
