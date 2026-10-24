//! Deterministic cryptography without entropy sources or host imports.

mod pda;

use core::fmt;

use ed25519_dalek::Signer;
use sha2::{Digest, Sha256};

use crate::{Address, Hash, Signature};

pub use pda::{
    MAX_SEED_LENGTH, MAX_SEEDS, PdaError, ProgramDerivedAddress, create_address_with_seed,
    create_program_address, find_program_address, is_on_curve,
};

/// Computes SHA-256 over the concatenation of the supplied byte slices.
#[must_use]
pub fn hash(slices: &[&[u8]]) -> Hash {
    let mut hasher = Sha256::new();

    for slice in slices {
        hasher.update(slice);
    }

    Hash::from_bytes(hasher.finalize().into())
}

/// A deterministic Ed25519 signer that zeroizes its owned secret on drop.
///
/// This type intentionally has no secret-export or `Debug` implementation.
/// Supplying a seed requires no host entropy; protecting and erasing the caller's
/// seed storage remains the caller's responsibility.
pub struct Keypair(ed25519_dalek::SigningKey);

impl Keypair {
    /// Derives an Ed25519 keypair from a caller-owned 32-byte seed.
    #[must_use]
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Self(ed25519_dalek::SigningKey::from_bytes(seed))
    }

    /// Imports a caller-owned 64-byte keypair: seed followed by Ed25519 public key.
    ///
    /// The public half must exactly match the key derived from the first 32 bytes.
    /// The input is unchanged; protecting and erasing it remains the caller's
    /// responsibility. The owned signer has the same zeroizing, non-exporting
    /// contract as [`Self::from_seed`].
    ///
    /// # Errors
    /// Returns [`KeypairImportError::InvalidLength`] unless the input is exactly
    /// 64 bytes, or [`KeypairImportError::PublicKeyMismatch`] if the public half
    /// is invalid or differs from the seed-derived public key.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KeypairImportError> {
        let bytes: &[u8; 64] = bytes
            .try_into()
            .map_err(|_| KeypairImportError::InvalidLength {
                actual: bytes.len(),
            })?;

        ed25519_dalek::SigningKey::from_keypair_bytes(bytes)
            .map(Self)
            .map_err(|_| KeypairImportError::PublicKeyMismatch)
    }

    /// Returns the public verification key as a Kit address.
    #[must_use]
    pub fn address(&self) -> Address {
        Address::from_bytes(self.0.verifying_key().to_bytes())
    }

    /// Signs these exact bytes using deterministic, unprehashed Ed25519.
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> Signature {
        Signature::from_bytes(self.0.sign(message).to_bytes())
    }
}

/// A rejected 64-byte keypair import, without any input key material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeypairImportError {
    /// The input was not exactly 64 bytes, including the 32-byte seed and public key.
    InvalidLength {
        /// The supplied byte count, without any input bytes.
        actual: usize,
    },
    /// The public half is invalid or differs from the seed-derived public key.
    PublicKeyMismatch,
}

impl fmt::Display for KeypairImportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { actual } => {
                write!(
                    formatter,
                    "expected 64 Ed25519 keypair bytes, received {actual}"
                )
            }
            Self::PublicKeyMismatch => {
                formatter.write_str("Ed25519 public key does not match the seed")
            }
        }
    }
}

impl core::error::Error for KeypairImportError {}

/// The address is not a valid key, or strict Ed25519 verification failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VerificationError;

impl fmt::Display for VerificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid Ed25519 key, signature, or message")
    }
}

impl core::error::Error for VerificationError {}

/// Verifies unprehashed Ed25519 while rejecting weak keys and malleable signatures.
///
/// # Errors
/// Returns [`VerificationError`] for an invalid public key, weak curve point,
/// noncanonical signature, or any signature/message mismatch.
pub fn verify(
    address: &Address,
    message: &[u8],
    signature: &Signature,
) -> Result<(), VerificationError> {
    let key = ed25519_dalek::VerifyingKey::from_bytes(address.as_bytes())
        .map_err(|_| VerificationError)?;
    let signature = ed25519_dalek::Signature::from_bytes(signature.as_bytes());

    key.verify_strict(message, &signature)
        .map_err(|_| VerificationError)
}
