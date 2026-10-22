//! Positional optional signatures over exact validated message bytes.

use alloc::vec::Vec;

use crate::{
    Address, Signature,
    codecs::{Error as CodecError, Limits, Reader},
};

use super::{Error, Message, wire};

const SIGNATURE_BYTES: usize = 64;

/// An immutable message with an optional signature for each required address.
///
/// Duplicate V0 addresses are rejected because address-keyed source envelopes
/// cannot represent them without losing slots. All-zero wire slots mean absent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Envelope {
    message: Message,
    signatures: Vec<Option<Signature>>,
}

impl Envelope {
    /// Creates a distinct unsigned envelope, with all required slots absent.
    ///
    /// # Errors
    /// Rejects repeated V0 signatories or allocation failure.
    pub fn unsigned(message: Message) -> Result<Self, Error> {
        let mut signatures = Vec::new();

        for (index, address) in message.signatories().iter().enumerate() {
            if message.signatories()[..index].contains(address) {
                return Err(Error::InvalidSignatories);
            }
        }

        signatures
            .try_reserve_exact(message.signatories().len())
            .map_err(|_| CodecError::AllocationFailed)?;
        signatures.resize(message.signatories().len(), None);
        Ok(Self {
            message,
            signatures,
        })
    }

    /// Strictly decodes a bounded envelope and its complete message.
    ///
    /// # Errors
    /// Rejects malformed messages, zero or mismatched signature counts, truncated
    /// signatures, and exhausted allocation budgets before retaining wire bytes.
    pub fn decode(bytes: &[u8], limits: Limits) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes, limits)?;
        let count = usize::from(reader.take(1)?[0]);
        let signatures = reader.take(count * SIGNATURE_BYTES)?;
        reader.claim(count)?;
        let message = wire::read(&mut reader)?;
        reader.finish()?;

        if count == 0 || count != message.signatories().len() {
            return Err(Error::SignerMismatch);
        }

        let mut envelope = Self::unsigned(message)?;

        for (slot, bytes) in envelope
            .signatures
            .iter_mut()
            .zip(signatures.as_chunks::<SIGNATURE_BYTES>().0)
        {
            if bytes.iter().any(|byte| *byte != 0) {
                *slot = Some(Signature::from_bytes(*bytes));
            }
        }

        Ok(envelope)
    }

    /// Encodes ordered signatures followed by the exact immutable message bytes.
    ///
    /// # Errors
    /// Rejects output exceeding `max_bytes`, allocation failure, or missing slots
    /// when `require_complete` is true. Presence does not prove valid signatures.
    pub fn encode(&self, require_complete: bool, max_bytes: usize) -> Result<Vec<u8>, Error> {
        let size = self
            .message
            .bytes()
            .len()
            .checked_add(1 + self.signatures.len() * SIGNATURE_BYTES)
            .ok_or(CodecError::Overflow)?;

        if require_complete && !self.is_complete() {
            return Err(Error::MissingSignature);
        }

        if size > max_bytes {
            return Err(CodecError::LimitExceeded.into());
        }

        let count = wire::signatories_valid(self.message.signatories())?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(size)
            .map_err(|_| CodecError::AllocationFailed)?;
        bytes.push(count);

        for signature in &self.signatures {
            bytes.extend_from_slice(
                signature
                    .as_ref()
                    .map_or(&[0; 64], |value| value.as_bytes().as_slice()),
            );
        }

        bytes.extend_from_slice(self.message.bytes());
        Ok(bytes)
    }

    /// Borrows the immutable validated message.
    #[must_use]
    pub const fn message(&self) -> &Message {
        &self.message
    }

    /// Borrows optional signatures in message signatory order.
    #[must_use]
    pub fn signatures(&self) -> &[Option<Signature>] {
        &self.signatures
    }

    /// Reports presence in every required slot, without cryptographic verification.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.signatures.iter().all(Option::is_some)
    }

    /// Returns a new envelope with a supplied signature in its required slot.
    ///
    /// # Errors
    /// Rejects addresses absent from the message. An all-zero signature is absent.
    pub fn with_signature(&self, address: Address, signature: Signature) -> Result<Self, Error> {
        let index = self
            .message
            .signatories()
            .iter()
            .position(|item| *item == address)
            .ok_or(Error::SignerMismatch)?;
        let mut envelope = self.clone();
        envelope.signatures[index] = (signature.as_bytes() != &[0; 64]).then_some(signature);
        Ok(envelope)
    }

    /// Deterministically signs the exact message bytes and returns a new envelope.
    ///
    /// # Errors
    /// Rejects a key whose address is not required by the message.
    #[cfg(feature = "crypto")]
    pub fn sign(&self, keypair: &crate::crypto::Keypair) -> Result<Self, Error> {
        self.with_signature(keypair.address(), keypair.sign(self.message.bytes()))
    }

    /// Strictly verifies every required signature over the exact message bytes.
    ///
    /// # Errors
    /// Rejects missing signatures, weak keys, or any signature/message mismatch.
    #[cfg(feature = "crypto")]
    pub fn verify(&self) -> Result<(), Error> {
        for (address, signature) in self.message.signatories().iter().zip(&self.signatures) {
            let signature = signature.as_ref().ok_or(Error::MissingSignature)?;
            crate::crypto::verify(address, self.message.bytes(), signature)
                .map_err(|_| Error::InvalidSignature)?;
        }

        Ok(())
    }
}
