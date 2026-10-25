//! Separate optional-factory graph policy. The default client policy is unchanged.
use std::{collections::BTreeSet, path::PathBuf, process::Command};

fn check(tree: &str) -> Result<(), String> {
    let actual: BTreeSet<_> = tree
        .lines()
        .map(|line| {
            line.split_once(" (")
                .map_or(line, |(package, _)| package)
                .trim()
        })
        .filter(|line| !line.is_empty())
        .collect();
    let expected: BTreeSet<_> = include_str!("../../normal-build.txt").lines().collect();

    if actual != expected {
        return Err(format!("unexpected optional confirmation graph {actual:?}"));
    }

    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo =
        PathBuf::from(std::env::var_os("SOLANA_KIT_CARGO").ok_or("explicit Cargo required")?);
    assert!(cargo.is_absolute());
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;

    for target in [
        "aarch64-apple-darwin",
        "wasm32-unknown-unknown",
        "wasm32-wasip1",
        "wasm32-wasip2",
    ] {
        let output = Command::new(&cargo)
            .current_dir(&root)
            .args([
                "tree",
                "--offline",
                "--locked",
                "-p",
                "solana_kit_client",
                "--no-default-features",
                "--features",
                "confirmation-factories",
                "-e",
                "normal,build",
                "--prefix",
                "none",
                "--format",
                "{p}",
                "--no-dedupe",
                "--target",
                target,
            ])
            .output()?;
        assert!(output.status.success());
        check(&String::from_utf8(output.stdout)?)?;
        println!("optional confirmation normal/build closure accepted on {target}");
    }

    let forbidden = concat!(include_str!("../../normal-build.txt"), "tokio v1.54.1\n");
    assert!(check(forbidden).unwrap_err().contains("tokio"));
    println!("optional closure rejects Tokio; default policy remains unchanged");
    Ok(())
}
