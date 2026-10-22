//! Verifies the public retained-report command fails despite a passing aggregate.

use std::process::Command;

#[test]
fn retained_report_cannot_hide_a_failing_file() {
    let report = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/style-file-below-floor.json"
    );
    let output = Command::new(env!("CARGO_BIN_EXE_solana_kit_xtask"))
        .args(["style-report", report])
        .output()
        .expect("execute style report validator");
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 diagnostics");
    assert!(!output.status.success());
    assert!(stderr.contains("generated.rs: complexity 88.7"), "{stderr}");
}
