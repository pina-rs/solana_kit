//! Command execution and raw evidence capture shared by the runtime journeys.

use anyhow::{Context, Result, ensure};
use std::process::{Command, Output};

/// Uses Cargo from the selected toolchain rather than guessing a PATH executable.
pub(crate) fn cargo() -> Command {
    Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
}

/// Saves both streams and the actual exit status before checking expectations.
pub(crate) fn capture(command: &mut Command, label: &str) -> Result<Output> {
    println!("running {command:?}");
    let output = command
        .output()
        .with_context(|| format!("execute {command:?}"))?;
    let directory = std::path::Path::new("target/runtime-evidence");
    std::fs::create_dir_all(directory)?;

    std::fs::write(directory.join(format!("{label}.stdout")), &output.stdout)?;
    std::fs::write(directory.join(format!("{label}.stderr")), &output.stderr)?;
    std::fs::write(
        directory.join(format!("{label}.status")),
        output.status.to_string(),
    )?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
    println!("{label}: {}", output.status);
    Ok(output)
}

/// A missing command or nonzero process is never converted into a skipped pass.
pub(crate) fn success(command: &mut Command, label: &str) -> Result<Output> {
    let output = capture(command, label)?;
    ensure!(output.status.success(), "{label} failed: {}", output.status);
    Ok(output)
}
