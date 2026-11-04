//! Repeats the external consumer under the declared MSRV toolchain.

use super::super::{Config, success};
use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

/// Runs default and selected features through rustup's named toolchain.
pub(super) fn run(config: &Config, directory: &Path, toolchain: &str) -> Result<()> {
    let cargo = binary(config, toolchain, "cargo")?;
    let rustc = binary(config, toolchain, "rustc")?;
    let rustdoc = binary(config, toolchain, "rustdoc")?;
    let version = success(
        config,
        Command::new(&rustc).arg("--version"),
        "msrv-rustc-version",
    )?;
    let version = String::from_utf8(version.stdout)?;

    for (label, features) in [
        ("consumer-msrv-default", false),
        ("consumer-msrv-selected", true),
    ] {
        let mut command = Command::new(&cargo);
        command
            .current_dir(directory)
            .args(["test", "--locked", "--offline"]);
        command
            .env("CARGO", &cargo)
            .env("RUSTC", &rustc)
            .env("RUSTDOC", &rustdoc)
            .env("SDK_REHEARSAL_EXPECTED_RUSTC", version.trim());
        command.env("CARGO_TARGET_DIR", config.target.join(label));

        if features {
            command.args(["--features", "selected"]);
        }

        success(config, &mut command, label)?;
    }

    Ok(())
}

/// Resolves each executable independently of inherited compiler overrides.
fn binary(config: &Config, toolchain: &str, name: &str) -> Result<PathBuf> {
    let output = success(
        config,
        Command::new("rustup").args(["which", "--toolchain", toolchain, name]),
        &format!("msrv-{name}-path"),
    )?;
    Ok(PathBuf::from(String::from_utf8(output.stdout)?.trim()))
}

#[cfg(test)]
mod tests {
    /// Replays only the compiler-sensitive stage against an already audited consumer.
    #[test]
    #[ignore = "requires a completed consumer and SDK_REHEARSAL_CONFIG"]
    fn actual_msrv_consumer() {
        let path = std::env::var_os("SDK_REHEARSAL_CONFIG").expect("rehearsal config path");
        let config: super::Config = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        let toolchain = config.msrv.as_deref().expect("explicit MSRV toolchain");
        super::run(&config, &config.consumer, toolchain).unwrap();
    }
}
