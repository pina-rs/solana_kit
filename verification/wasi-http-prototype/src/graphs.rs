//! Closed adapter graph checks keep Preview2 capabilities out of other targets.

use std::collections::BTreeSet;
use std::process::Command;

pub fn check() {
    for target in [
        None,
        Some("wasm32-unknown-unknown"),
        Some("wasm32-wasip1"),
        Some("wasm32-wasip2"),
    ] {
        let mut command = Command::new("cargo");
        command.current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        command.args([
            "+1.99.1",
            "tree",
            "--locked",
            "--offline",
            "-p",
            "solana_kit_wasi",
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

        let output = command.output().expect("cargo tree");
        assert!(output.status.success(), "graph query failed");
        let tree = String::from_utf8(output.stdout).expect("UTF8");
        let preview2 = target == Some("wasm32-wasip2");
        let expected = expected(preview2);
        assert!(matches_policy(&tree, &expected));
        assert!(!matches_policy(
            &format!("{tree}\ntokio v1.54.1\n"),
            &expected
        ));
        println!(
            "GRAPH {} exact={} Tokio negative rejected",
            target.unwrap_or("native"),
            expected.len()
        );
    }
}

fn expected(preview2: bool) -> BTreeSet<String> {
    let mut expected: BTreeSet<_> = include_str!("../../../xtask/policy/client.txt")
        .lines()
        .map(str::to_owned)
        .collect();
    expected.insert("solana_kit_wasi v0.1.0".to_owned());

    if preview2 {
        expected.extend(
            [
                "wasip2 v1.0.4+wasi-0.2.12",
                "wit-bindgen v0.57.1",
                "bitflags v2.13.2",
            ]
            .map(str::to_owned),
        );
    }

    expected
}

fn matches_policy(tree: &str, expected: &BTreeSet<String>) -> bool {
    let actual: BTreeSet<_> = tree
        .lines()
        .map(|line| {
            line.split_once(" (")
                .map_or(line, |(package, _)| package)
                .trim()
                .to_owned()
        })
        .filter(|line| !line.is_empty())
        .collect();

    &actual == expected
}
