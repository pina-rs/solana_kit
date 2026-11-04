//! Repeats the external consumer under the declared MSRV toolchain.

use super::super::{Config, success};
use anyhow::Result;
use std::{path::Path, process::Command};

/// Runs default and selected features through rustup's named toolchain.
pub(super) fn run(config: &Config, directory: &Path, toolchain: &str) -> Result<()> {
    for (label, features) in [
        ("consumer-msrv-default", false),
        ("consumer-msrv-selected", true),
    ] {
        let mut command = Command::new("rustup");
        command.current_dir(directory).args([
            "run",
            toolchain,
            "cargo",
            "test",
            "--locked",
            "--offline",
        ]);
        command.env("CARGO_TARGET_DIR", config.target.join(label));

        if features {
            command.args(["--features", "selected"]);
        }

        success(config, &mut command, label)?;
    }

    Ok(())
}
