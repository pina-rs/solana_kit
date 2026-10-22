//! Separate closed System program graph policy; core policies remain unchanged.

use crate::process::{cargo, success};
use anyhow::{Result, ensure};
use std::collections::BTreeSet;

/// Program data requires exactly its own crate, Kit core and base58.
fn dependencies(tree: &str) -> Result<()> {
    let actual: BTreeSet<_> = tree
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .collect();
    let expected = BTreeSet::from(["solana_kit_system", "solana_kit_core", "bs58"]);
    ensure!(
        actual == expected,
        "unexpected System dependencies: {actual:?}"
    );
    println!("selected System closure: {actual:?}");
    Ok(())
}

/// Executes each target query independently and rejects the real host-dependency fixture.
pub(crate) fn run() -> Result<()> {
    for target in [
        None,
        Some("wasm32-unknown-unknown"),
        Some("wasm32-wasip1"),
        Some("wasm32-wasip2"),
    ] {
        let mut command = cargo();
        command.args([
            "tree",
            "--locked",
            "-p",
            "solana_kit_system",
            "--no-default-features",
            "-e",
            "normal,build",
            "--prefix",
            "none",
            "--format",
            "{p}",
            "--no-dedupe",
        ]);

        if let Some(target) = target {
            command.args(["--target", target]);
        }

        let label = format!("system-dependencies-{}", target.unwrap_or("native"));
        let output = success(&mut command, &label)?;
        dependencies(&String::from_utf8(output.stdout)?)?;
    }

    let forbidden = concat!(
        "solana_kit_system v0.1.0\n",
        include_str!("../fixtures/forbidden-dependency.txt")
    );
    let rejected = dependencies(forbidden).expect_err("System graph must reject Tokio");
    ensure!(
        rejected.to_string().contains("tokio"),
        "wrong System rejection"
    );
    println!("System dependency negative control rejected: {rejected}");
    Ok(())
}
