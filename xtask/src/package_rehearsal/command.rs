//! Runs child commands with one controlled Cargo environment and retained output.

use super::Config;
use anyhow::{Context, Result, ensure};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

/// Adds only a path to the Cargo home; `env_remove` cannot clear an inherited parent.
fn sanitize(config: &Config, command: &mut Command) {
    command
        .env("CARGO_HOME", &config.cargo_home)
        .env("CARGO_INCREMENTAL", "0")
        .env("CARGO_BUILD_JOBS", "2")
        .env("CARGO_NET_OFFLINE", "true")
        .env("CARGO_HTTP_USER_AGENT", "build")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_BUILD_TARGET")
        .env_remove("RUSTC_WRAPPER")
        .env_remove("RUSTC_WORKSPACE_WRAPPER")
        .env_remove("CARGO_REGISTRY_TOKEN");

    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("CARGO_REGISTRIES_") {
            command.env_remove(key);
        }
    }
}

/// Saves both streams and the actual status before checking the expected result.
pub(crate) fn run_command(config: &Config, command: &mut Command, label: &str) -> Result<Output> {
    execute(config, command, label, None)
}

/// Runs a negative control with its own fresh Cargo home instead of the positive one.
pub(crate) fn with_home(
    config: &Config,
    command: &mut Command,
    home: &Path,
    label: &str,
) -> Result<Output> {
    execute(config, command, label, Some(home))
}

/// Applies the controlled environment, then a caller-specific Cargo home when needed.
fn execute(
    config: &Config,
    command: &mut Command,
    label: &str,
    home: Option<&Path>,
) -> Result<Output> {
    sanitize(config, command);

    if let Some(home) = home {
        command.env("CARGO_HOME", home);
    }

    println!("running {command:?}");
    let output = command
        .output()
        .with_context(|| format!("execute {command:?}"))?;
    let directory = config.evidence.join("commands");
    fs::create_dir_all(&directory)?;
    fs::write(directory.join(format!("{label}.stdout")), &output.stdout)?;
    fs::write(directory.join(format!("{label}.stderr")), &output.stderr)?;
    fs::write(
        directory.join(format!("{label}.status")),
        output.status.to_string(),
    )?;
    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
    println!("{label}: {}", output.status);
    Ok(output)
}

/// A missing command or nonzero process is never converted into a skipped pass.
pub(crate) fn success(config: &Config, command: &mut Command, label: &str) -> Result<Output> {
    let output = run_command(config, command, label)?;
    ensure!(
        output.status.success(),
        "{label} failed with {}; inspect retained output",
        output.status
    );
    Ok(output)
}
