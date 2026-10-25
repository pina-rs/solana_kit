#![cfg(all(feature = "keys", target_arch = "wasm32", target_os = "unknown"))]

use solana_kit_browser::keys::BrowserEntropy;
use solana_kit_core::crypto::{Entropy, ExportableKeypair, Keypair};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn browser_generation_roundtrips_through_an_explicit_backup() {
    let mut entropy = BrowserEntropy::new().expect("real browser Web Crypto");
    let keypair = ExportableKeypair::generate(&mut entropy)
        .await
        .expect("host entropy");
    let backup = keypair.backup();
    let imported = Keypair::from_bytes(backup.expose_secret()).expect("validated backup");

    assert_eq!(imported.address(), keypair.signer().address());
    assert_eq!(
        imported.sign(b"browser caller"),
        keypair.signer().sign(b"browser caller")
    );
}

#[wasm_bindgen_test]
async fn entropy_larger_than_one_host_call_uses_bounded_chunks() {
    const OUTPUT_BYTES: usize = 65_537;
    let mut entropy = BrowserEntropy::new().expect("real browser Web Crypto");
    let mut bytes = vec![0; OUTPUT_BYTES];
    entropy
        .fill(&mut bytes)
        .await
        .expect("multiple legal Crypto calls");

    assert!(bytes.iter().any(|byte| *byte != 0));
}
