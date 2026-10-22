use solana_kit_core::Address;

use crate::Signer;

/// One caller-owned logical capability borrowing a signer backend.
///
/// Reuse a reference to this handle to share its identity intentionally. A new
/// handle has a distinct identity even when it borrows the same backend. Two
/// distinct handles claiming one address conflict in a transaction pipeline.
/// Identity is separate from the Solana address and from backend object storage.
///
/// The backend reference makes this handle nonzero-sized, including for zero-sized
/// backends. Its storage identifies the capability only while the handle is
/// borrowed. No allocation, global registry, or entropy is required.
pub struct SignerCapability<'a, S> {
    pub(crate) signer: &'a S,
}

impl<'a, S> SignerCapability<'a, S> {
    /// Creates a distinct logical capability for this backend.
    #[must_use]
    pub fn new(signer: &'a S) -> Self {
        Self { signer }
    }
}

impl<S: Signer> Signer for SignerCapability<'_, S> {
    fn address(&self) -> Address {
        self.signer.address()
    }
}
