//! Exercises argument rejection through the executable before verification work begins.

use std::process::Command;

#[test]
fn retained_report_rejects_ignored_trailing_argument() {
    let report = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/style-file-below-floor.json"
    );
    let output = Command::new(env!("CARGO_BIN_EXE_solana_kit_xtask"))
        .args(["style-report", report, "--offline"])
        .env("CARGO_NET_OFFLINE", "true")
        .output()
        .expect("execute argument validator");
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 diagnostics");
    assert!(!output.status.success());
    assert!(stderr.contains("unexpected argument"), "{stderr}");
    assert!(stderr.contains("CARGO_NET_OFFLINE=true"), "{stderr}");
    assert!(output.stdout.is_empty());
}

#[test]
fn invalid_commands_stop_before_child_execution() {
    let cases: &[&[&str]] = &[
        &["consumer", "--offline"],
        &["consumer", "surplus"],
        &["native", "surplus"],
        &["browser", "surplus"],
        &["wasi", "surplus"],
        &["leptoskit", "surplus"],
        &["gates", "surplus"],
        &["dependency-graphs", "surplus"],
        &["all", "surplus"],
        &["style", "--offline"],
        &["style", ".", "surplus"],
        &["style-report", "report.json", "surplus"],
        &["imports", "guest.wasm", "surplus"],
        &["dependency-fixture", "tree.txt", "surplus"],
    ];

    for arguments in cases {
        let output = Command::new(env!("CARGO_BIN_EXE_solana_kit_xtask"))
            .args(*arguments)
            .env("CARGO_NET_OFFLINE", "true")
            .env("CARGO", "missing-cargo-must-never-execute")
            .output()
            .expect("execute argument validator");
        let stderr = String::from_utf8(output.stderr).expect("UTF-8 diagnostics");
        assert!(!output.status.success(), "{arguments:?}");
        assert!(
            stderr.contains("unexpected argument"),
            "{arguments:?}: {stderr}"
        );
        assert!(stderr.contains("usage:"), "{stderr}");
        assert!(
            output.stdout.is_empty(),
            "{arguments:?} started verification"
        );
    }
}
