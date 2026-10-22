//! Keep explicit JavaScript adapters outside portable client dependency graphs.
use std::{collections::BTreeSet, process::Command};
const CLIENT: &[&str] = &[
    "base64 v0.22.1",
    "bs58 v0.5.1",
    "itoa v1.0.18",
    "memchr v2.8.3",
    "proc-macro2 v1.0.107",
    "quote v1.0.47",
    "serde v1.0.229",
    "serde_core v1.0.229",
    "serde_derive v1.0.229",
    "serde_json v1.0.154",
    "syn v3.0.9",
    "unicode-ident v1.0.27",
    "zmij v1.0.23",
];

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let version = Command::new("rustc").arg("-vV").output()?;
    assert!(version.status.success());
    let version = String::from_utf8(version.stdout)?;
    let host = version
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .ok_or("missing compiler host")?;
    let targets = [
        host,
        "wasm32-unknown-unknown",
        "wasm32-wasip1",
        "wasm32-wasip2",
    ];

    for target in targets {
        audit("solana_kit_core", target, &["bs58 v0.5.1"])?;
        audit("solana_kit_client", target, CLIENT)?;
    }

    println!("Eight portable normal/build graphs match exact package/version allowlists.");
    Ok(())
}
fn audit(package: &str, target: &str, expected: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args([
            "+1.99.1",
            "tree",
            "-p",
            package,
            "-e",
            "normal,build",
            "--prefix",
            "none",
            "--format",
            "{p}",
            "--target",
            target,
            "--locked",
            "--offline",
        ])
        .output()?;

    if !output.status.success() {
        return Err("portable dependency query failed".into());
    }

    let graph = String::from_utf8(output.stdout)?;
    let actual = packages(&graph);
    let expected = expected.iter().map(|value| (*value).to_owned()).collect();

    if actual != expected {
        return Err(format!("unexpected dependency closure for {package}").into());
    }

    let corrupted = format!("{graph}\nweb-sys v0.3.105\n");
    assert_ne!(
        packages(&corrupted),
        expected,
        "JavaScript dependency negative control must fail"
    );
    println!("{package} {target} dependency isolation passed, including negative control");

    Ok(())
}
fn packages(graph: &str) -> BTreeSet<String> {
    graph
        .lines()
        .filter(|line| {
            !line.starts_with("solana_kit_core ") && !line.starts_with("solana_kit_client ")
        })
        .map(|line| {
            line.split_whitespace()
                .take(2)
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}
