//! Validates complete command lines before any verification side effects.

use anyhow::{Context, Result, bail, ensure};
use std::{ffi::OsString, path::PathBuf};

const USAGE: &str = "usage: cargo xtask <command> [path]";
const OFFLINE: &str = "xtask does not forward Cargo flags; use CARGO_NET_OFFLINE=true cargo xtask <command> for offline child Cargo commands";

pub(crate) enum Command {
    ArtifactCapture(PathBuf),
    Style(Option<PathBuf>),
    StyleReport(PathBuf),
    Native,
    Browser,
    Wasi,
    Leptoskit,
    Gates,
    DependencyGraphs,
    Consumer,
    Imports(PathBuf),
    DependencyFixture(PathBuf),
    PackageReview(PathBuf),
    PackageReport(PathBuf),
    RegistryFixture(PathBuf),
    All,
}

/// Resolves the command and its exact positional arity before dispatch.
pub(crate) fn parse(arguments: &[OsString]) -> Result<Command> {
    let command = arguments.first().context(USAGE)?;
    let command = command.to_str().context("command must be valid UTF-8")?;
    let paths = &arguments[1..];

    match command {
        "artifact-capture" => Ok(Command::ArtifactCapture(required_path(paths, command)?)),
        "style" => Ok(Command::Style(path(paths, command, false)?)),
        "style-report" => Ok(Command::StyleReport(required_path(paths, command)?)),
        "imports" => Ok(Command::Imports(required_path(paths, command)?)),
        "dependency-fixture" => Ok(Command::DependencyFixture(required_path(paths, command)?)),
        "package-review" => Ok(Command::PackageReview(required_path(paths, command)?)),
        "package-report" => Ok(Command::PackageReport(required_path(paths, command)?)),
        "registry-fixture" => Ok(Command::RegistryFixture(required_path(paths, command)?)),
        "native" => no_arguments(paths, command, Command::Native),
        "browser" => no_arguments(paths, command, Command::Browser),
        "wasi" => no_arguments(paths, command, Command::Wasi),
        "leptoskit" => no_arguments(paths, command, Command::Leptoskit),
        "gates" => no_arguments(paths, command, Command::Gates),
        "dependency-graphs" => no_arguments(paths, command, Command::DependencyGraphs),
        "consumer" => no_arguments(paths, command, Command::Consumer),
        "all" => no_arguments(paths, command, Command::All),
        _ => bail!("unknown verification command: {command}; {USAGE}"),
    }
}

/// Rejects all arguments for journeys that configure themselves through the environment.
fn no_arguments(paths: &[OsString], name: &str, command: Command) -> Result<Command> {
    ensure!(
        paths.is_empty(),
        "unexpected argument for {name}; usage: cargo xtask {name}; {OFFLINE}"
    );

    Ok(command)
}

/// Preserves the optional style root while rejecting options and extra positionals.
fn path(paths: &[OsString], name: &str, required: bool) -> Result<Option<PathBuf>> {
    ensure!(
        paths.len() <= 1
            && paths
                .iter()
                .all(|path| !path.to_string_lossy().starts_with('-')),
        "unexpected argument for {name}; usage: cargo xtask {name} {}; {OFFLINE}",
        if required { "<path>" } else { "[root]" }
    );

    Ok(paths.first().map(PathBuf::from))
}

/// Requires exactly one path for report and fixture commands.
fn required_path(paths: &[OsString], name: &str) -> Result<PathBuf> {
    path(paths, name, true)?.with_context(|| format!("usage: cargo xtask {name} <path>"))
}

#[cfg(test)]
mod tests;
