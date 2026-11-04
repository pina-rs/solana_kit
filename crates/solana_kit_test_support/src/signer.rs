//! Optional deterministic keypair fixture for signer-adjacent tests.

pub use solana_kit_core::crypto::Keypair;

/// Fixed seed for reproducible tests.
///
/// This key is public fixture material. Never use it with real funds or on a
/// production network.
pub const TEST_SEED: [u8; 32] = [0x5a; 32];

/// Creates the deterministic test keypair.
#[must_use]
pub fn test_keypair() -> Keypair {
    Keypair::from_seed(&TEST_SEED)
}

/// Creates a deterministic keypair from a caller-owned seed.
#[must_use]
pub fn keypair(seed: &[u8; 32]) -> Keypair {
    Keypair::from_seed(seed)
}
