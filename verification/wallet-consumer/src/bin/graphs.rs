use std::{error::Error, process::Command};

fn check(graph: &str) -> Result<(), String> {
    for line in graph.lines() {
        let name = line.split_whitespace().next().ok_or("empty graph line")?;

        if !matches!(name, "solana_kit_wallet" | "base64") {
            return Err(format!("forbidden normal/build dependency {name}"));
        }
    }

    if !graph.contains("solana_kit_wallet") || !graph.contains("base64") {
        return Err("missing expected graph packages".into());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--offline",
            "-p",
            "solana_kit_wallet",
            "--target",
            "all",
            "--edges",
            "normal,build",
            "--prefix",
            "none",
            "--format",
            "{p}",
        ])
        .output()?;

    if !output.status.success() {
        return Err("cargo tree failed".into());
    }

    let graph = String::from_utf8(output.stdout)?;
    check(&graph)?;

    for dependency in ["tokio", "getrandom", "solana-sdk", "web-sys", "reqwest"] {
        let contaminated = format!("{graph}\n{dependency} v1.0.0\n");

        if check(&contaminated).is_ok() {
            return Err("dependency negative control escaped".into());
        }
    }

    println!(
        "Default graph has only wallet/base64; five host/RNG/Solana negative controls rejected"
    );
    Ok(())
}
