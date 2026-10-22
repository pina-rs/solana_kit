use alloc::vec::Vec;

use solana_kit_core::{Address, Signature, transaction::Transaction};

use crate::Error;

/// A signature together with its owner and the exact immutable bytes authorized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignatureOutput {
    address: Address,
    bytes: Vec<u8>,
    signature: Signature,
}

impl SignatureOutput {
    /// Records backend output. Acceptance validates identity and snapshot.
    #[must_use]
    pub fn new(address: Address, bytes: &[u8], signature: Signature) -> Self {
        Self {
            address,
            bytes: bytes.into(),
            signature,
        }
    }

    /// Address that authorized these bytes.
    #[must_use]
    pub const fn address(&self) -> Address {
        self.address
    }

    /// Exact bytes authorized by the backend.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Signature supplied by the backend.
    #[must_use]
    pub const fn signature(&self) -> Signature {
        self.signature
    }

    /// Checks a bare-message response against the caller's expected owner and bytes.
    ///
    /// With `crypto`, also performs strict Ed25519 verification.
    ///
    /// # Errors
    /// Rejects foreign owners, changed bytes, zero signatures, and invalid crypto.
    pub fn validate(&self, owner: Address, bytes: &[u8]) -> Result<(), Error> {
        if owner != self.address {
            return Err(Error::ForeignSigner);
        }

        if self.bytes != bytes {
            return Err(Error::ChangedSnapshot);
        }

        if self.signature.as_bytes() == &[0; 64] {
            return Err(solana_kit_core::transaction::Error::InvalidSignature.into());
        }

        #[cfg(feature = "crypto")]
        solana_kit_core::crypto::verify(&owner, bytes, &self.signature)
            .map_err(|_| solana_kit_core::transaction::Error::InvalidSignature)?;

        Ok(())
    }

    /// Installs a validated response in its address-owned transaction slot.
    ///
    /// # Errors
    /// Rejects foreign owners, changed bytes, unknown slots, and invalid signatures.
    pub fn apply(&self, owner: Address, transaction: &Transaction) -> Result<Transaction, Error> {
        self.validate(owner, transaction.message().bytes())?;
        Ok(transaction.with_signature(owner, &self.bytes, self.signature)?)
    }
}
