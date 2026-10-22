//! Exercises the compiled artifact and novel missing/forbidden/ABI controls.

use super::verify;
use std::path::Path;

/// Missing the actual build artifact fails this test rather than skipping it.
#[test]
fn actual_rust_guest_has_exact_import_contract() {
    let artifact = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../target/wasm32-unknown-unknown/debug/solana_kit_leptoskit_async_guest.wasm");
    let wasm = std::fs::read(artifact).expect("build the required actual guest first");
    verify(&wasm).expect("actual compiled guest import contract");
}

/// An empty artifact cannot claim integration without the required bridge.
#[test]
fn missing_imports_fail() {
    let wasm = wat::parse_str("(module)").expect("valid negative fixture");
    let error = verify(&wasm).expect_err("missing imports must fail");
    assert!(error.to_string().contains("missing expected guest imports"));
}

/// Even an unfamiliar ambient import fails the closed binary policy.
#[test]
fn unknown_ambient_import_fails() {
    let wasm = wat::parse_str("(module (import \"env\" \"novel_entropy\" (func)))")
        .expect("valid negative fixture");
    let error = verify(&wasm).expect_err("ambient import must fail");
    assert!(
        error
            .to_string()
            .contains("forbidden compiled guest import")
    );
}

/// Matching names cannot conceal an incompatible compiled integer signature.
#[test]
fn wrong_bridge_signature_fails() {
    let wasm = wat::parse_str(
        "(module (import \"leptoskit_plugins_v1\" \"version\" (func (param i32) (result i32))))",
    )
    .expect("valid negative fixture");
    let error = verify(&wasm).expect_err("wrong signature must fail");
    assert!(error.to_string().contains("wrong bridge ABI signature"));
}
