//! Preserves native Cargo executables before another mode can overwrite their bytes.

mod archive;
mod stream;

use anyhow::{Context, Result, ensure};
use archive::{FileHash, Manifest, Producer};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    source: PathBuf,
    source_files: Vec<PathBuf>,
    target: PathBuf,
    archive: PathBuf,
    toolchain: PathBuf,
    expected_release: String,
    args: Vec<String>,
    required_tests: bool,
}

/// Loads one explicit capture contract without interpreting shell text.
pub(crate) fn run(path: &Path) -> Result<()> {
    let config: Config = serde_json::from_slice(&fs::read(path)?)?;
    capture(config)
}

/// Preflights old bytes before launching even a producer-version subprocess.
fn capture(config: Config) -> Result<()> {
    config.validate()?;
    fs::create_dir_all(&config.target)?;
    let target = config.target.canonicalize()?;
    let _lock = TargetLock::acquire(&target)?;
    let history = target.join(".artifact-capture-history.json");
    let mut prior: Vec<PathBuf> = if history.exists() {
        serde_json::from_slice(&fs::read(&history)?)?
    } else {
        Vec::new()
    };

    for archive in &prior {
        archive::verify(archive)?;
    }

    fs::create_dir(&config.archive).context("archive must be a fresh nonexistent directory")?;
    let destination = config.archive.canonicalize()?;
    ensure!(
        !destination.starts_with(&target),
        "archive must be outside owned target"
    );
    prior.push(destination.clone());
    archive::atomic_json(&history, &prior)?;

    // Register before the child: interrupted captures block future cache reuse.
    let result = execute(&config, &target, &destination);

    if let Err(error) = &result {
        fs::write(
            destination.join("capture-error.txt"),
            format!("{error:#}\n"),
        )?;
    }

    result
}

impl Config {
    /// Restricts mutations to one selected Cargo build/test and one owned target.
    fn validate(&self) -> Result<()> {
        ensure!(
            self.source.is_absolute()
                && self.target.is_absolute()
                && self.archive.is_absolute()
                && self.toolchain.is_absolute(),
            "paths must be absolute"
        );
        ensure!(
            !self.source_files.is_empty(),
            "source_files must identify captured source"
        );
        ensure!(
            matches!(
                self.args.first().map(String::as_str),
                Some("test" | "build")
            ),
            "args must start with test or build"
        );

        for argument in &self.args {
            ensure!(
                !argument.starts_with("--target-dir")
                    && !argument.starts_with("--message-format")
                    && !argument.starts_with("--config")
                    && argument != "--",
                "target, message format, config and trailing test arguments are capture-owned"
            );
        }

        ensure!(!self.archive.exists(), "refusing archive overwrite");
        Ok(())
    }
}

/// Retains the command and producer identities before streaming Cargo output.
fn execute(config: &Config, target: &Path, destination: &Path) -> Result<()> {
    let producers = ["cargo", "rustc", "rustdoc"]
        .into_iter()
        .map(|name| Producer::inspect(&config.toolchain.join(name), &config.expected_release))
        .collect::<Result<Vec<_>>>()?;
    let source_files = config
        .source_files
        .iter()
        .map(|path| FileHash::inspect(&config.source.join(path)))
        .collect::<Result<Vec<_>>>()?;
    let mut manifest = Manifest {
        source: config.source.clone(),
        source_files,
        producers,
        command: config.args.clone(),
        target: target.to_owned(),
        artifacts: Vec::new(),
        cargo_status: None,
        complete: false,
    };
    archive::atomic_json(&destination.join("manifest.json"), &manifest)?;
    run_cargo(config, target, destination, &mut manifest)
}

/// Finalizes retained evidence before propagating the actual Cargo exit status.
fn run_cargo(
    config: &Config,
    target: &Path,
    destination: &Path,
    manifest: &mut Manifest,
) -> Result<()> {
    let mut command = Command::new(&manifest.producers[0].file.path);
    command
        .args(&config.args)
        .args([
            "--offline",
            "--locked",
            "--jobs",
            "2",
            "--message-format=json",
            "--verbose",
        ])
        .arg("--target-dir")
        .arg(target)
        .current_dir(&config.source)
        .env("RUSTC", &manifest.producers[1].file.path)
        .env("RUSTDOC", &manifest.producers[2].file.path)
        .env_remove("RUSTC_WRAPPER")
        .env_remove("RUSTC_WORKSPACE_WRAPPER")
        .stdout(Stdio::piped())
        .stderr(fs::File::create(destination.join("cargo.stderr"))?);
    fs::write(destination.join("command.txt"), format!("{command:?}\n"))?;
    let mut child = command.spawn().context("start selected Cargo")?;
    let stdout = child.stdout.take().context("Cargo stdout pipe")?;
    let parsed = stream::capture(stdout, target, destination, manifest);
    let status = child.wait()?;
    manifest.cargo_status = status.code();
    fs::write(destination.join("cargo.status"), status.to_string())?;
    archive::atomic_json(&destination.join("manifest.json"), &manifest)?;
    parsed?;
    archive::list_tests(destination, manifest)?;
    ensure!(
        !config.required_tests
            || manifest
                .artifacts
                .iter()
                .any(|artifact| artifact.tests.is_some_and(|count| count > 0)),
        "required test inventory is empty"
    );
    manifest.complete = true;
    archive::atomic_json(&destination.join("manifest.json"), &manifest)?;
    archive::verify(destination)?;
    ensure!(
        status.success(),
        "Cargo failed: {status}; captured bytes retained"
    );
    Ok(())
}

struct TargetLock(PathBuf);

impl TargetLock {
    /// Serializes access without accepting a stale interrupted owner.
    fn acquire(target: &Path) -> Result<Self> {
        let path = target.join(".artifact-capture-active");
        fs::create_dir(&path).context("target already has an active or interrupted capture")?;
        Ok(Self(path))
    }
}

impl Drop for TargetLock {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir(&self.0) {
            eprintln!("retain target lock after cleanup failure: {error}");
        }
    }
}

#[cfg(test)]
mod tests;
