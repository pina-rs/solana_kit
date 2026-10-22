//! Isolated deterministic signer cases, separate from the shared runtime suite.
#![no_std]
#![forbid(unsafe_code)]

use solana_kit_core::{Signature, crypto::Keypair};
use solana_kit_signers::{Error, SignatureOutput};

/// Accepts a deterministic signature over its exact byte snapshot.
#[must_use]
pub fn valid() -> bool {
    let key = Keypair::from_seed(&[7; 32]);
    let output = SignatureOutput::new(
        key.address(),
        b"portable signing",
        key.sign(b"portable signing"),
    );
    output.validate(key.address(), b"portable signing").is_ok()
}

/// Rejects the reserved unsigned zero-signature representation.
#[must_use]
pub fn malformed() -> bool {
    let key = Keypair::from_seed(&[7; 32]);
    let output = SignatureOutput::new(
        key.address(),
        b"portable signing",
        Signature::from_bytes([0; 64]),
    );
    output.validate(key.address(), b"portable signing").is_err()
}

/// Rejects the same signature against a different snapshot.
#[must_use]
pub fn tamper() -> bool {
    let key = Keypair::from_seed(&[7; 32]);
    let output = SignatureOutput::new(
        key.address(),
        b"portable signing",
        key.sign(b"portable signing"),
    );
    output.validate(key.address(), b"tampered") == Err(Error::ChangedSnapshot)
}
