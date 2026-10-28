//! Independent exact-version normal/build policies for public RPC and Jupiter clients.

use crate::process::{cargo, success};
use anyhow::{Result, ensure};
use std::collections::BTreeSet;

const CLIENT: &str = include_str!("../policy/client.txt");
const JUPITER: &str = concat!(
    include_str!("../policy/client.txt"),
    "solana_kit_jupiter v0.1.0\n"
);

fn dependencies(tree: &str, policy: &str) -> Result<()> {
    let actual: BTreeSet<_> = tree
        .lines()
        .map(|line| {
            line.split_once(" (")
                .map_or(line, |(package, _)| package)
                .trim()
        })
        .filter(|line| !line.is_empty())
        .collect();
    let expected: BTreeSet<_> = policy.lines().collect();
    ensure!(
        actual == expected,
        "unexpected client dependencies: {actual:?}"
    );
    println!("selected public client closure: {actual:?}");
    Ok(())
}

/// Queries the real graph separately on native and each supported Wasm target.
pub(crate) fn run() -> Result<()> {
    for (package, policy) in [
        ("solana_kit_client", CLIENT),
        ("solana_kit_jupiter", JUPITER),
    ] {
        graphs(package, policy)?;
        let forbidden = format!("{policy}tokio v1.54.1\n");
        let rejected =
            dependencies(&forbidden, policy).expect_err("client graph must reject Tokio");
        ensure!(
            rejected.to_string().contains("tokio"),
            "wrong client rejection"
        );
        println!("{package} dependency negative control rejected: {rejected}");
    }

    Ok(())
}

fn graphs(package: &str, policy: &str) -> Result<()> {
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
            package,
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

        let label = format!("{package}-dependencies-{}", target.unwrap_or("native"));
        let output = success(&mut command, &label)?;
        dependencies(&String::from_utf8(output.stdout)?, policy)?;
    }

    Ok(())
}

#[test]
fn current_client_and_jupiter_graphs_reject_tokio_on_every_target() {
    run().expect("native, browser and both WASI graphs plus negative controls must pass");
}
