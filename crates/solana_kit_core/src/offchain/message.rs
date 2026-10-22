//! Validated message construction and immutable access.

use alloc::{string::String, vec::Vec};

use crate::{Address, codecs::Limits};

use super::{Error, wire};

/// The V0 content discriminator and its content-byte limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ContentFormat {
    /// Nonempty printable ASCII, bytes 0x20 through 0x7e, at most 1232 bytes.
    RestrictedAscii = 0,
    /// Nonempty UTF-8, at most 1232 bytes.
    Utf8 = 1,
    /// Nonempty UTF-8, at most 65535 bytes.
    Utf8Long = 2,
}

/// Version-specific fields. V1 cannot carry a V0 application domain or format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Format {
    /// V0 identifies the requesting application and content restrictions.
    V0 {
        /// A 32-byte application identifier, with the same encoding as an address.
        application_domain: Address,
        /// The content restriction encoded in the V0 preamble.
        content_format: ContentFormat,
    },
    /// V1 uses nonempty UTF-8 and sorted unique raw address bytes.
    V1,
}

/// A validated message owning the exact immutable bytes covered by signatures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub(super) bytes: Vec<u8>,
    pub(super) format: Format,
    pub(super) signatories: Vec<Address>,
    pub(super) content: String,
}

impl Message {
    /// Constructs V0, preserving signatory order as required by its wire format.
    ///
    /// # Errors
    /// Rejects invalid content, signatory counts, or the caller's byte budget.
    /// V0 message codecs permit repeated addresses; envelopes reject them.
    pub fn v0(
        application_domain: Address,
        content_format: ContentFormat,
        content: &str,
        signatories: &[Address],
        max_bytes: usize,
    ) -> Result<Self, Error> {
        wire::build(
            Format::V0 {
                application_domain,
                content_format,
            },
            content,
            signatories,
            max_bytes,
        )
    }

    /// Constructs V1 and sorts unique signatories by their raw 32-byte values.
    ///
    /// # Errors
    /// Rejects empty content, invalid signatories, or the caller's byte budget.
    pub fn v1(content: &str, signatories: &[Address], max_bytes: usize) -> Result<Self, Error> {
        wire::build(Format::V1, content, signatories, max_bytes)
    }

    /// Strictly decodes one complete bounded message, preserving its exact bytes.
    ///
    /// # Errors
    /// Rejects malformed domains, versions, formats, UTF-8, counts, V1 order,
    /// V0 lengths, trailing V0 data, and exhausted budgets before allocation.
    pub fn decode(bytes: &[u8], limits: Limits) -> Result<Self, Error> {
        let mut reader = crate::codecs::Reader::new(bytes, limits)?;
        let message = wire::read(&mut reader)?;
        reader.finish()?;
        Ok(message)
    }

    /// Borrows the exact bytes signed by every required signatory.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the version-specific domain and format metadata.
    #[must_use]
    pub const fn format(&self) -> Format {
        self.format
    }

    /// Borrows signatories in their exact wire order.
    #[must_use]
    pub fn signatories(&self) -> &[Address] {
        &self.signatories
    }

    /// Borrows the validated, nonempty UTF-8 content without normalization.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}
