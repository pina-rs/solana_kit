//! Makes an MSRV claim fail when Cargo actually invokes a different compiler.

use std::{env, process::Command};

fn main() {
    println!("cargo:rerun-if-env-changed=SDK_REHEARSAL_EXPECTED_RUSTC");
    let Some(expected) = env::var_os("SDK_REHEARSAL_EXPECTED_RUSTC") else {
        return;
    };
    let output = Command::new(env::var_os("RUSTC").expect("Cargo supplies RUSTC"))
        .arg("--version")
        .output()
        .expect("read actual compiler version");
    assert!(output.status.success(), "compiler version query failed");
    let actual = String::from_utf8(output.stdout).expect("compiler version is UTF-8");
    assert_eq!(actual.trim(), expected.to_str().unwrap(), "MSRV compiler mismatch");
}
