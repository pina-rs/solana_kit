//! Black-box checks that the executable reports its own package metadata.
//!
//! These commands are inert: they print a version or help and start no workflow.

use std::{fs, path::Path, process::Command};

const BINARY: &str = env!("CARGO_BIN_EXE_solana-kit-publisher");

#[test]
fn version_flag_reports_the_binary_package_version() {
    let version = package_version();

    assert_eq!(
        output(&["--version"]),
        format!("solana-kit-publisher {version}\n")
    );
}

#[test]
fn help_heading_reports_the_binary_package_version() {
    let version = package_version();
    let help = output(&["--help"]);
    let heading = help.lines().next().expect("help must have a heading");

    assert_eq!(heading, format!("solana-kit-publisher {version}"));
}

fn output(arguments: &[&str]) -> String {
    let output = Command::new(BINARY)
        .args(arguments)
        .output()
        .expect("run the publisher executable");

    assert!(
        output.status.success(),
        "{} failed: {}",
        arguments.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "{} wrote to stderr: {}",
        arguments.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("publisher output must be UTF-8")
}

/// Reads the version from this binary package's authored manifest, not from the
/// compile-time environment the executable itself uses.
fn package_version() -> String {
    let manifest = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .expect("read the binary package manifest");
    let mut in_package = false;

    for line in manifest.lines() {
        let line = line.trim();

        if line.starts_with('[') {
            in_package = line == "[package]";

            continue;
        }

        if in_package && let Some(value) = line.strip_prefix("version = ") {
            return value.trim_matches('"').to_owned();
        }
    }

    panic!("the binary package manifest has no package version");
}
