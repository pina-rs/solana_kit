//! Bounded off-chain application messages and immutable signing envelopes.
//!
//! These are application messages, not transaction wire formats. V0 limits apply
//! to content bytes only. V1 has no protocol content cap; callers supply budgets.
//! V1 content consumes the whole frame, so appended valid text is content.
//! Decoding budgets count the retained wire bytes, UTF-8 bytes, and address slots.

mod envelope;
mod message;
mod wire;

pub use envelope::Envelope;
pub use message::{ContentFormat, Format, Message};

use core::fmt;

/// A message, envelope, or signature failed validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// A bounded binary read, allocation, or text conversion failed.
    Codec(crate::codecs::Error),
    /// The signing domain or version discriminator is unsupported.
    InvalidDomainOrVersion,
    /// The content is empty or violates its V0 format.
    InvalidContent,
    /// Signatories are empty, excessive, duplicated, or incorrectly ordered.
    InvalidSignatories,
    /// A signature count or signer address does not match the message.
    SignerMismatch,
    /// At least one signature is missing.
    MissingSignature,
    /// Strict Ed25519 verification failed.
    InvalidSignature,
}

impl From<crate::codecs::Error> for Error {
    fn from(value: crate::codecs::Error) -> Self {
        Self::Codec(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "off-chain message error: {self:?}")
    }
}

impl core::error::Error for Error {}
