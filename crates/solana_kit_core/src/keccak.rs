//! Optional deterministic Keccak-256 for off-chain account-compression clients.

use sha3::{Digest, Keccak256};

use crate::Hash;

/// Hashes the concatenation of slices with Keccak's original 0x01 padding.
/// This intentionally differs from the SHA3-256 standardized padding.
#[must_use]
pub fn keccak256(slices: &[&[u8]]) -> Hash {
    let mut digest = Keccak256::new();

    for slice in slices {
        digest.update(slice);
    }

    Hash::from_bytes(digest.finalize().into())
}
