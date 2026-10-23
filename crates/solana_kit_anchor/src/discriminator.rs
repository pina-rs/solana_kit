//! Anchor namespaces reuse the audited, portable SHA-256 implementation.

use sha2::{Digest, Sha256};

/// Size of the standard Anchor sighash prefix.
pub const DISCRIMINATOR_LENGTH: usize = 8;

/// Hashes byte-typed input; invalid integer bytes cannot be represented.
pub fn sha256(input: &[u8]) -> [u8; 32] {
    Sha256::digest(input).into()
}

/// Hashes `namespace:name` without allocating the joined text.
pub fn anchor_sighash(namespace: &str, name: &str) -> [u8; DISCRIMINATOR_LENGTH] {
    let mut hash = Sha256::new();
    hash.update(namespace.as_bytes());
    hash.update(b":");
    hash.update(name.as_bytes());
    let digest = hash.finalize();
    let mut result = [0; DISCRIMINATOR_LENGTH];
    result.copy_from_slice(&digest[..DISCRIMINATOR_LENGTH]);
    result
}

/// Computes a `global` instruction sighash.
pub fn instruction_discriminator(name: &str) -> [u8; DISCRIMINATOR_LENGTH] {
    anchor_sighash("global", name)
}

/// Computes an `account` sighash.
pub fn account_discriminator(name: &str) -> [u8; DISCRIMINATOR_LENGTH] {
    anchor_sighash("account", name)
}

/// Computes an `event` sighash.
pub fn event_discriminator(name: &str) -> [u8; DISCRIMINATOR_LENGTH] {
    anchor_sighash("event", name)
}

/// Checks any explicit discriminator length, including the empty prefix.
pub fn has_discriminator(data: &[u8], discriminator: &[u8]) -> bool {
    data.starts_with(discriminator)
}
