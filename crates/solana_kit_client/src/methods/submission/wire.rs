//! Immutable submission bytes retain the identity established before transport.

use solana_kit_core::{
    Signature,
    transaction::{Error, Transaction},
};

/// Structurally valid transaction bytes with every signature slot populated.
///
/// Construction preserves the exact legacy/v0/v1 wire and retains its primary
/// signature. It does not verify Ed25519 signatures or establish node honesty.
#[derive(Debug, Clone, Copy)]
pub struct SignedWire<'a> {
    bytes: &'a [u8],
    signature: Signature,
}

impl<'a> SignedWire<'a> {
    /// Validates complete signed wire before a submission operation can exist.
    ///
    /// # Errors
    /// Rejects malformed, unsupported or partially signed transactions.
    pub fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        let transaction = Transaction::decode(bytes)?;
        let signature = transaction
            .signatures()
            .first()
            .copied()
            .flatten()
            .ok_or(Error::MissingSignature)?;

        if transaction.signatures().iter().any(Option::is_none) {
            return Err(Error::MissingSignature);
        }

        Ok(Self { bytes, signature })
    }

    /// Exact caller-provided bytes; submission never re-encodes the transaction.
    #[must_use]
    pub const fn bytes(self) -> &'a [u8] {
        self.bytes
    }

    /// The primary signature derived from these bytes before network I/O.
    #[must_use]
    pub const fn signature(self) -> Signature {
        self.signature
    }
}
