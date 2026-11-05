//! Exact third-party normal/build policies for public RPC and Jupiter clients.

use crate::process::{cargo, success};
use anyhow::{Result, ensure};
use std::collections::BTreeSet;

const CLIENT: &str = include_str!("../policy/client.txt");
const JUPITER: &str = concat!(include_str!("../policy/client.txt"), "solana_kit_jupiter\n");

fn dependencies(tree: &str, policy: &str) -> Result<()> {
    let actual: BTreeSet<_> = tree
        .lines()
        .map(|line| {
            line.split_once(" (")
                .map_or(line, |(package, _)| package)
                .trim()
        })
        .map(policy_identity)
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

/// Workspace versions are governed by Cargo.lock; external versions stay exact here.
fn policy_identity(package: &str) -> &str {
    match package.split_once(" v") {
        Some((name @ ("solana_kit_client" | "solana_kit_core" | "solana_kit_jupiter"), _)) => name,
        _ => package,
    }
}

#[test]
fn workspace_release_versions_preserve_the_same_external_policy() {
    dependencies(
        "solana_kit_client v9.8.7\nsolana_kit_core v0.2.0\n",
        "solana_kit_client\nsolana_kit_core\n",
    )
    .expect("workspace version changes preserve the dependency policy");

    assert!(dependencies("bs58 v0.6.0\n", "bs58 v0.5.1\n").is_err());
    assert!(dependencies("solana_kit_new v0.1.0\n", "solana_kit_core\n").is_err());
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
