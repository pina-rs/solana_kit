//! Exact ordered attestation JSON; entropy and signing belong to the caller.

use crate::Error;
use solana_kit_core::{Hash, codecs::Limits};

const REQUEST_ID_DIGITS: usize = 32;
pub(super) const SIGNATURE_BYTES: usize = 64;

/// A caller-issued 32-digit decimal identifier. This type provides no uniqueness guarantee.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestId(String);

impl RequestId {
    /// Validates length and ASCII digits before allocating the owned identifier.
    pub fn parse(value: &str) -> Result<Self, Error> {
        if value.len() != REQUEST_ID_DIGITS || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Error::InvalidRequestId);
        }

        Ok(Self(value.to_owned()))
    }

    /// The exact validated digits, including leading zeroes.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Frozen bytes for one caller-approved publication attestation.
#[derive(Debug)]
pub struct AttestationMessage {
    bytes: Vec<u8>,
    request_id: RequestId,
    #[cfg(feature = "crypto")]
    limits: Limits,
}

impl AttestationMessage {
    /// Freezes `slot_number`, `blockhash`, then `request_unique_id` with no whitespace.
    ///
    /// Both the eventual base64 output and its binary input must fit `max_bytes`.
    /// `max_items` covers JSON bytes, signature-prefixed bytes and base64 characters
    /// together. Public scalar formatting uses at most 64 temporary characters.
    pub fn new(
        slot: u64,
        blockhash: Hash,
        request_id: RequestId,
        limits: Limits,
    ) -> Result<Self, Error> {
        let slot = slot.to_string();
        let blockhash = blockhash.to_string();
        let length = b"{\"slot_number\":,\"blockhash\":\"\",\"request_unique_id\":\"\"}".len()
            + slot.len()
            + blockhash.len()
            + request_id.as_str().len();
        let signed_length = length + SIGNATURE_BYTES;
        let encoded_length = signed_length.div_ceil(3) * 4;

        if encoded_length > limits.max_bytes
            || length + signed_length + encoded_length > limits.max_items
        {
            return Err(Error::LimitExceeded);
        }

        let bytes = format!(
            "{{\"slot_number\":{slot},\"blockhash\":\"{blockhash}\",\"request_unique_id\":\"{}\"}}",
            request_id.as_str()
        )
        .into_bytes();

        Ok(Self {
            bytes,
            request_id,
            #[cfg(feature = "crypto")]
            limits,
        })
    }

    /// Exact bare Ed25519 message bytes; no off-chain-message envelope is added.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The caller-issued identifier bound into these immutable bytes.
    #[must_use]
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }
}

#[cfg(feature = "crypto")]
mod signed;
#[cfg(feature = "crypto")]
pub use signed::PublicationAttestation;
