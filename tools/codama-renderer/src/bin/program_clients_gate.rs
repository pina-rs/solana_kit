//! Enforces each client's actual normal/build closure on every portable target.

use std::{collections::BTreeSet, error::Error, path::Path, process::Command};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const CLIENTS: [&str; 11] = [
    "solana_kit_address_lookup_table",
    "solana_kit_config",
    "solana_kit_stake",
    "solana_kit_loader",
    "solana_kit_account_compression",
    "solana_kit_mpl_core",
    "solana_kit_mpl_token_metadata",
    "solana_kit_mpl_bubblegum",
    "solana_kit_attestation_service",
    "solana_kit_squads",
    "solana_kit_subscriptions",
];
const PDA_CLIENTS: [&str; 7] = [
    "solana_kit_address_lookup_table",
    "solana_kit_mpl_core",
    "solana_kit_mpl_token_metadata",
    "solana_kit_mpl_bubblegum",
    "solana_kit_attestation_service",
    "solana_kit_squads",
    "solana_kit_subscriptions",
];
const TARGETS: [Option<&str>; 4] = [
    None,
    Some("wasm32-unknown-unknown"),
    Some("wasm32-wasip1"),
    Some("wasm32-wasip2"),
];

/// Keeps each client's closure separate so another client's dependency cannot leak in.
fn check(tree: &str, client: &str) -> Result<()> {
    let actual: BTreeSet<_> = tree
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .collect();
    let mut expected = BTreeSet::from([client, "solana_kit_core", "bs58"]);

    if client == "solana_kit_stake" {
        expected.insert("solana_kit_system");
    }

    if actual != expected {
        return Err(
            format!("unexpected {client} dependencies: {actual:?}; expected {expected:?}").into(),
        );
    }

    Ok(())
}

/// PDA derivation forwards only the already audited optional Core cryptography.
fn check_pda(tree: &str, client: &str) -> Result<()> {
    let actual: BTreeSet<_> = tree
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .collect();
    let mut allowed: BTreeSet<_> = include_str!("../../../../xtask/policy/crypto.txt")
        .lines()
        .collect();
    allowed.insert(client);
    let required = [
        client,
        "solana_kit_core",
        "bs58",
        "curve25519-dalek",
        "sha2",
    ];

    if !actual.is_subset(&allowed) || required.iter().any(|name| !actual.contains(name)) {
        return Err(format!("unexpected {client} PDA dependencies: {actual:?}").into());
    }

    Ok(())
}

/// Preserve the actual target-filtered Cargo result before applying SDK policy.
fn query(
    root: &Path,
    client: &str,
    target: Option<&str>,
    edges: &str,
    pda: bool,
) -> Result<String> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| env!("CARGO").into());
    let mut command = Command::new(cargo);
    command.current_dir(root).args([
        "tree",
        "--offline",
        "--locked",
        "-p",
        client,
        "--no-default-features",
        "-e",
        edges,
        "--prefix",
        "none",
        "--format",
        "{p}",
        "--no-dedupe",
    ]);

    if let Some(target) = target {
        command.args(["--target", target]);
    }

    if pda {
        command.args(["--features", "pda"]);
    }

    let output = command.output()?;

    if !output.status.success() {
        return Err(format!(
            "Cargo graph failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

/// Each real result and its injected Tokio variant use the identical checker.
fn graph(root: &Path, client: &str, target: Option<&str>, edges: &str, pda: bool) -> Result<()> {
    let tree = query(root, client, target, edges, pda)?;
    let verify = |tree: &str| {
        if pda {
            check_pda(tree, client)
        } else {
            check(tree, client)
        }
    };

    verify(&tree)?;
    let contaminated = format!("{tree}\ntokio v1.54.1\n");
    let error = verify(&contaminated).expect_err("host dependency must reject");
    assert!(
        error.to_string().contains("tokio"),
        "wrong dependency rejection"
    );
    println!(
        "{client} {} {edges} pda={pda}: approved closure; Tokio negative rejected",
        target.unwrap_or("native")
    );

    Ok(())
}

/// Applies the same exact policy and negative control to every maintained live query.
fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    for target in TARGETS {
        for client in CLIENTS {
            views(&root, client, target, false)?;
        }

        for client in PDA_CLIENTS {
            views(&root, client, target, true)?;
        }
    }

    Ok(())
}

/// Normal and normal-plus-build closures are independent Cargo queries.
fn views(root: &Path, client: &str, target: Option<&str>, pda: bool) -> Result<()> {
    for edges in ["normal", "normal,build"] {
        graph(root, client, target, edges, pda)?;
    }

    Ok(())
}
