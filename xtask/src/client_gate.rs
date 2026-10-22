//! Independent exact-version normal/build policy for the public RPC client.

use crate::process::{cargo, success};
use anyhow::{Result, ensure};
use std::collections::BTreeSet;

fn dependencies(tree: &str) -> Result<()> {
    let actual: BTreeSet<_> = tree
        .lines()
        .map(|line| {
            line.split_once(" (")
                .map_or(line, |(package, _)| package)
                .trim()
        })
        .filter(|line| !line.is_empty())
        .collect();
    let expected: BTreeSet<_> = include_str!("../policy/client.txt").lines().collect();
    ensure!(
        actual == expected,
        "unexpected client dependencies: {actual:?}"
    );
    println!("selected public client closure: {actual:?}");
    Ok(())
}

/// Queries the real graph separately on native and each supported Wasm target.
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
            "solana_kit_client",
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

        let label = format!("client-dependencies-{}", target.unwrap_or("native"));
        let output = success(&mut command, &label)?;
        dependencies(&String::from_utf8(output.stdout)?)?;
    }

    let forbidden = concat!(include_str!("../policy/client.txt"), "tokio v1.54.1\n");
    let rejected = dependencies(forbidden).expect_err("client graph must reject Tokio");
    ensure!(
        rejected.to_string().contains("tokio"),
        "wrong client rejection"
    );
    println!("client dependency negative control rejected: {rejected}");
    Ok(())
}
