//! Intentional key generation and backup, separate from non-exporting signers.

use core::{fmt, future::Future};

use zeroize::Zeroizing;

use super::{Keypair, KeypairImportError};

const SEED_BYTES: usize = 32;
const KEYPAIR_BYTES: usize = 64;

/// Caller-owned secure entropy. Core never selects an operating system or host.
pub trait Entropy {
    /// The host's actual entropy failure.
    type Error;
    /// A host operation that may suspend without imposing a Send bound.
    type Fill<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Fills the complete output or fails. Partial output remains secret.
    fn fill<'a>(&'a mut self, output: &'a mut [u8]) -> Self::Fill<'a>;
}

/// A key created with an explicit backup/export capability.
///
/// This type has no Clone or serialization implementation. Converting it into a
/// normal [`Keypair`] consumes this capability; normal signers cannot be wrapped
/// to recover an export capability. The underlying signer zeroizes on drop.
pub struct ExportableKeypair(Keypair);

impl ExportableKeypair {
    /// Creates an exportable key from a caller-owned seed.
    /// The caller remains responsible for protecting its original seed.
    #[must_use]
    pub fn from_seed(seed: &[u8; SEED_BYTES]) -> Self {
        Self(Keypair::from_seed(seed))
    }

    /// Imports exactly seed followed by its matching public verification key.
    ///
    /// # Errors
    /// Returns the same length and public-half errors as [`Keypair::from_bytes`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KeypairImportError> {
        Keypair::from_bytes(bytes).map(Self)
    }

    /// Generates a key using the supplied host, zeroizing seed storage on every
    /// success, failure and cancellation path.
    ///
    /// # Errors
    /// Returns the host's original failure without creating a signer.
    pub async fn generate<E: Entropy>(entropy: &mut E) -> Result<Self, E::Error> {
        let mut seed = Zeroizing::new([0; SEED_BYTES]);
        entropy.fill(&mut seed[..]).await?;

        Ok(Self::from_seed(&seed))
    }

    /// Borrows ordinary signing capabilities without exposing secret bytes.
    #[must_use]
    pub fn signer(&self) -> &Keypair {
        &self.0
    }

    /// Permanently removes this owner's export capability.
    #[must_use]
    pub fn into_signer(self) -> Keypair {
        self.0
    }

    /// Explicitly exports seed followed by public key into zeroizing storage.
    /// Exported bytes remain secret and must never be logged or sent as metadata.
    #[must_use]
    pub fn backup(&self) -> KeypairBackup {
        KeypairBackup(Zeroizing::new(self.0.0.to_keypair_bytes()))
    }
}

/// Secret backup bytes with deterministic zeroization and redacted diagnostics.
///
/// There is deliberately no Deref, AsRef, Clone or serialization implementation.
pub struct KeypairBackup(Zeroizing<[u8; KEYPAIR_BYTES]>);

impl KeypairBackup {
    /// Deliberately borrows secret bytes for an explicit backup operation.
    #[must_use]
    pub fn expose_secret(&self) -> &[u8; KEYPAIR_BYTES] {
        &self.0
    }
}

impl fmt::Debug for KeypairBackup {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("KeypairBackup(REDACTED)")
    }
}

impl fmt::Debug for ExportableKeypair {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ExportableKeypair(REDACTED)")
    }
}
