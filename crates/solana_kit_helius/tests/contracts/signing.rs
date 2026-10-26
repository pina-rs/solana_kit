//! Optional authentication signing bodies shared by native and portable registries.

use solana_kit_core::crypto::{Keypair, verify};
use solana_kit_helius::auth::{VERIFICATION_TEXT, auth_message, sign_auth_message};

/// The exact default JSON and caller-owned timestamp determine the authenticated bytes.
pub fn default_message() {
    let keypair = Keypair::from_seed(&[7; 32]);
    let signed = sign_auth_message(&keypair, None, 1234).expect("local signing");
    assert_eq!(
        signed.message,
        format!(r#"{{"message":"{VERIFICATION_TEXT}","timestamp":1234}}"#)
    );
    assert!(
        verify(
            &keypair.address(),
            signed.message.as_bytes(),
            &signed.signature
        )
        .is_ok()
    );
    assert!(verify(&keypair.address(), b"different-message", &signed.signature).is_err());
    assert_ne!(
        auth_message(1234).expect("first timestamp"),
        auth_message(1235).expect("second timestamp")
    );
}

/// A caller-provided message is not reformatted or given a hidden timestamp.
pub fn caller_message() {
    let keypair = Keypair::from_seed(&[7; 32]);
    let signed = sign_auth_message(&keypair, Some("custom\0message".into()), u64::MAX)
        .expect("custom signing");
    assert_eq!(signed.message, "custom\0message");
    assert!(
        verify(
            &keypair.address(),
            signed.message.as_bytes(),
            &signed.signature
        )
        .is_ok()
    );
}
