//! Only authenticated detached signatures can produce a publication payload.

use super::{AttestationMessage, SIGNATURE_BYTES};
use crate::Error;
use solana_kit_core::{
    Address, Signature,
    codecs::{Base64, Decoder, Limits},
    crypto,
};

/// Verified `base64(signature || exact JSON)` with its signing identity retained.
#[derive(Debug)]
pub struct PublicationAttestation {
    message: AttestationMessage,
    signer: Address,
    payload: String,
}

impl AttestationMessage {
    /// Checks the signature against these exact bytes before forming the payload.
    pub fn attach_signature(
        self,
        signer: Address,
        signature: Signature,
    ) -> Result<PublicationAttestation, Error> {
        crypto::verify(&signer, &self.bytes, &signature)
            .map_err(|_| Error::InvalidSignature(signer))?;
        let mut signed = Vec::new();
        let size = self.bytes.len() + SIGNATURE_BYTES;
        signed
            .try_reserve_exact(size)
            .map_err(|_| Error::Codec(solana_kit_core::codecs::Error::AllocationFailed))?;
        signed.extend_from_slice(signature.as_bytes());
        signed.extend_from_slice(&self.bytes);
        let items = self.limits.max_items - self.bytes.len() - size;
        let payload = Base64
            .decode(&signed, Limits::new(self.limits.max_bytes, items))
            .map_err(Error::Codec)?;

        Ok(PublicationAttestation {
            message: self,
            signer,
            payload,
        })
    }
}

impl PublicationAttestation {
    /// The verified signature followed by the exact JSON, encoded with standard padding.
    #[must_use]
    pub fn payload(&self) -> &str {
        &self.payload
    }

    /// Public identity whose detached signature was verified.
    #[must_use]
    pub const fn signer(&self) -> Address {
        self.signer
    }

    /// Immutable source bytes and request identifier.
    #[must_use]
    pub const fn message(&self) -> &AttestationMessage {
        &self.message
    }
}
