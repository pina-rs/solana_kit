//! Closed import and normal/build dependency policies for the deterministic layer.

use crate::process::{cargo, success};
use anyhow::{Context, Result, bail, ensure};
use std::{collections::BTreeSet, path::Path};
use wasmparser::{Parser, Payload};

const GUEST: &str = "target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm";
const ALLOWED_IMPORTS: &[(&str, &str)] = &[
    ("leptoskit", "node_create"),
    ("leptoskit", "node_set_props"),
    ("leptoskit", "commit"),
];

/// Parses the compiled artifact; target cfg never substitutes for binary inspection.
fn imports(bytes: &[u8]) -> Result<()> {
    let mut found = BTreeSet::new();

    for payload in Parser::new(0).parse_all(bytes) {
        if let Payload::ImportSection(section) = payload? {
            for import in section {
                let import = import?;
                ensure!(
                    ALLOWED_IMPORTS.contains(&(import.module, import.name)),
                    "forbidden import {}::{}",
                    import.module,
                    import.name
                );
                found.insert((import.module, import.name));
            }
        }
    }

    ensure!(
        found == ALLOWED_IMPORTS.iter().copied().collect(),
        "missing expected guest imports"
    );
    println!("compiled guest imports: {found:?}");
    Ok(())
}

/// Checks the actual guest artifact and fails if it has not been built.
pub(crate) fn guest() -> Result<()> {
    imports(&std::fs::read(GUEST).context("build the Rust guest before checking imports")?)
}

/// Exposes the production import gate for externally generated negative artifacts.
pub(crate) fn check_import_file() -> Result<()> {
    let path = std::env::args_os()
        .nth(2)
        .context("usage: cargo xtask imports <guest.wasm>")?;
    imports(&std::fs::read(path)?)
}

/// Extracts all selected packages while ignoring local absolute path decorations.
fn packages(tree: &str) -> BTreeSet<&str> {
    tree.lines()
        .filter_map(|line| line.split_whitespace().next())
        .collect()
}

/// Uses an exact crate allowlist rather than a blacklist that misses new host APIs.
fn dependencies(tree: &str, allowed: &str) -> Result<()> {
    let actual = packages(tree);
    let allowed = packages(allowed);
    let forbidden: Vec<_> = actual.difference(&allowed).collect();

    ensure!(
        forbidden.is_empty(),
        "forbidden core dependencies: {forbidden:?}"
    );
    ensure!(
        actual.contains("solana_kit_core"),
        "tree omits the core package"
    );
    println!("selected core closure: {actual:?}");
    Ok(())
}

/// Allows the same dependency gate to be exercised with a forbidden tree fixture.
pub(crate) fn check_dependency_fixture() -> Result<()> {
    let path = std::env::args_os()
        .nth(2)
        .context("usage: cargo xtask dependency-fixture <tree.txt>")?;
    let tree = std::fs::read_to_string(path)?;
    dependencies(&tree, include_str!("../policy/crypto.txt"))
}

/// Queries selected normal/build closures on native and every promised Wasm target.
fn dependency_graphs() -> Result<()> {
    for target in [
        None,
        Some("wasm32-unknown-unknown"),
        Some("wasm32-wasip1"),
        Some("wasm32-wasip2"),
    ] {
        for features in ["", "crypto", "fixed-points", "crypto,fixed-points"] {
            dependency_graph(target, features, "normal,build")?;

            if features.contains("fixed-points") {
                dependency_graph(target, features, "normal")?;
            }
        }
    }

    Ok(())
}

/// Selects one feature/target closure without mixing host-only verification tools.
fn dependency_graph(target: Option<&str>, features: &str, edges: &str) -> Result<()> {
    let mut command = cargo();
    command.args([
        "tree",
        "--locked",
        "-p",
        "solana_kit_core",
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

    if !features.is_empty() {
        command.args(["--features", features]);
    }

    let label = format!(
        "dependencies-{}-{}-{edges}",
        target.unwrap_or("native"),
        features.replace(',', "-")
    );
    let output = success(&mut command, &label)?;
    let tree = String::from_utf8(output.stdout)?;
    let allowed = dependency_policy(features);

    if features.contains("fixed-points") {
        ensure!(
            packages(&tree).contains("autocfg") == (edges == "normal,build"),
            "autocfg must be build-only in the fixed-point closure"
        );
    }

    dependencies(&tree, &allowed)
}

/// Numeric build tooling is allowed only when the optional numeric graph is selected.
fn dependency_policy(features: &str) -> String {
    let mut allowed = if features.contains("crypto") {
        include_str!("../policy/crypto.txt").to_owned()
    } else {
        "solana_kit_core\nbs58\n".to_owned()
    };

    if features.contains("fixed-points") {
        allowed.push_str(include_str!("../policy/fixed-points.txt"));
    }

    allowed
}

/// Executes observable controls against exactly the same production checkers.
fn negative_controls() -> Result<()> {
    let directory = Path::new("target/runtime-evidence");
    let bytes = wat::parse_str(include_str!("../fixtures/forbidden-import.wat"))?;
    std::fs::create_dir_all(directory)?;
    std::fs::write(directory.join("forbidden-import.wasm"), &bytes)?;

    match imports(&bytes) {
        Err(error) if error.to_string().contains("forbidden import env::random") => {
            println!("import negative control rejected: {error}")
        }
        _ => bail!("forbidden import negative control was not rejected correctly"),
    }

    for features in ["", "crypto", "fixed-points", "crypto,fixed-points"] {
        match dependencies(
            include_str!("../fixtures/forbidden-dependency.txt"),
            &dependency_policy(features),
        ) {
            Err(error) if error.to_string().contains("tokio") => {
                println!("dependency negative control ({features}) rejected: {error}")
            }
            _ => bail!("forbidden dependency negative control was not rejected correctly"),
        }
    }

    Ok(())
}

/// Runs both real graph gates and their required failing fixtures.
pub(crate) fn run() -> Result<()> {
    guest()?;
    dependency_graphs()?;
    crate::system_gate::run()?;
    crate::client_gate::run()?;
    negative_controls()
}
