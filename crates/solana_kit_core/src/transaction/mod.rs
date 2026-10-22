//! Immutable legacy/v0 messages, strict wire decoding, and ordered signatures.
//!
//! Compilation requires explicit blockhash expiry metadata. Decoding preserves
//! received bytes and records unknown expiry; the block height is never on wire.
//! A decoded transaction is structurally validated, not cryptographically trusted.

mod compile;
mod wire;

use alloc::vec::Vec;
use core::fmt;

use crate::{Address, Hash, Instruction, Signature};

/// Maximum legacy/v0 transaction packet size in bytes.
pub const PACKET_SIZE: usize = 1232;

/// Supported message envelopes. v1 requires its distinct future implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Version {
    /// Original unprefixed message.
    Legacy,
    /// Version zero with address table lookups.
    V0,
}

/// Blockhash expiry information supplied by the builder or received on wire.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lifetime {
    /// Caller-provided recent blockhash and last valid block height.
    Blockhash {
        /// Hash placed in the signed message.
        blockhash: Hash,
        /// Expiry height; metadata only, absent from the signed wire.
        last_valid_block_height: u64,
    },
    /// Wire token whose expiry or nonce interpretation has not been established.
    Unresolved(Hash),
}

impl Lifetime {
    /// Returns the exact signed lifetime token.
    #[must_use]
    pub const fn token(self) -> Hash {
        match self {
            Self::Blockhash { blockhash, .. } | Self::Unresolved(blockhash) => blockhash,
        }
    }
}

/// A caller-owned snapshot of one address lookup table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LookupTable {
    /// Address of the lookup table account.
    pub address: Address,
    /// Addresses in on-chain index order (at most 256).
    pub addresses: Vec<Address>,
}

/// Signer and readonly partition counts from a validated message header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    /// Number of initial static accounts requiring signatures.
    pub required_signatures: u8,
    /// Number of readonly accounts at the end of the signer partition.
    pub readonly_signers: u8,
    /// Number of readonly accounts at the end of the unsigned static partition.
    pub readonly_unsigned: u8,
}

/// One immutable message instruction; account positions retain duplicates.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledInstruction {
    pub(super) program_index: u8,
    pub(super) accounts: Vec<u8>,
    pub(super) data: Vec<u8>,
}

impl CompiledInstruction {
    /// Static index of the invoked program.
    #[must_use]
    pub const fn program_index(&self) -> u8 {
        self.program_index
    }
    /// Ordered account indices, including duplicates.
    #[must_use]
    pub fn accounts(&self) -> &[u8] {
        &self.accounts
    }
    /// Program-specific data.
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/// One immutable v0 lookup descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lookup {
    pub(super) address: Address,
    pub(super) writable: Vec<u8>,
    pub(super) readonly: Vec<u8>,
}

impl Lookup {
    /// Address of the lookup table account.
    #[must_use]
    pub const fn address(&self) -> Address {
        self.address
    }
    /// Lookup indices loaded with write access.
    #[must_use]
    pub fn writable_indices(&self) -> &[u8] {
        &self.writable
    }
    /// Lookup indices loaded with read access.
    #[must_use]
    pub fn readonly_indices(&self) -> &[u8] {
        &self.readonly
    }
}

/// A validated message whose wire bytes and account partitions are frozen.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    version: Version,
    header: Header,
    static_accounts: Vec<Address>,
    instructions: Vec<CompiledInstruction>,
    lookups: Vec<Lookup>,
    lifetime: Lifetime,
    bytes: Vec<u8>,
}

impl Message {
    /// Compiles instructions using deterministic Kit account ordering.
    ///
    /// # Errors
    /// Rejects unresolved lifetime, invalid program privileges, table conflicts,
    /// oversized account/index counts, and a message exceeding packet capacity.
    pub fn compile(
        version: Version,
        payer: Address,
        lifetime: Lifetime,
        instructions: &[Instruction],
        tables: &[LookupTable],
    ) -> Result<Self, Error> {
        compile::compile(version, payer, lifetime, instructions, tables)
    }

    /// Validates an entire external message without applying builder restrictions.
    ///
    /// # Errors
    /// Rejects unsupported versions, noncanonical lengths, invalid partitions or
    /// indices, truncation, trailing bytes, and excessive packet size.
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        wire::decode(bytes)
    }

    /// Exact frozen bytes signed by every required signer.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Supported wire version.
    #[must_use]
    pub const fn version(&self) -> Version {
        self.version
    }
    /// Validated partition counts, returned by value.
    #[must_use]
    pub const fn header(&self) -> Header {
        self.header
    }
    /// Ordered static addresses, starting with the fee payer.
    #[must_use]
    pub fn static_accounts(&self) -> &[Address] {
        &self.static_accounts
    }
    /// Required signer addresses in signature-slot order.
    #[must_use]
    pub fn signers(&self) -> &[Address] {
        &self.static_accounts[..usize::from(self.header.required_signatures)]
    }
    /// Compiled instructions in execution order.
    #[must_use]
    pub fn instructions(&self) -> &[CompiledInstruction] {
        &self.instructions
    }
    /// Address table descriptors in wire order.
    #[must_use]
    pub fn lookups(&self) -> &[Lookup] {
        &self.lookups
    }
    /// Explicit local expiry metadata, or an unresolved external token.
    #[must_use]
    pub const fn lifetime(&self) -> Lifetime {
        self.lifetime
    }
}

/// An immutable message paired with exactly one optional slot per signer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transaction {
    message: Message,
    signatures: Vec<Option<Signature>>,
}

impl Transaction {
    /// Creates an unsigned transaction, allocating slots in message signer order.
    #[must_use]
    pub fn new(message: Message) -> Self {
        let signatures = alloc::vec![None; message.signers().len()];
        Self {
            message,
            signatures,
        }
    }

    /// Decodes a complete packet, preserving unsigned zero signature slots.
    ///
    /// # Errors
    /// Rejects excessive size, malformed messages, or signature-count mismatch.
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        wire::decode_transaction(bytes)
    }

    /// The frozen message used for signing.
    #[must_use]
    pub const fn message(&self) -> &Message {
        &self.message
    }
    /// Ordered slots; absent signatures encode as 64 zero bytes.
    #[must_use]
    pub fn signatures(&self) -> &[Option<Signature>] {
        &self.signatures
    }

    /// Returns a copy with one signature installed for an exact message snapshot.
    ///
    /// This checks identity and slot ownership, not cryptographic validity. Enable
    /// `crypto` and call `verify` before trusting externally supplied signatures.
    ///
    /// # Errors
    /// Rejects changed message bytes, unknown signers, and the reserved all-zero
    /// unsigned signature representation.
    pub fn with_signature(
        &self,
        signer: Address,
        signed_bytes: &[u8],
        signature: Signature,
    ) -> Result<Self, Error> {
        if signed_bytes != self.message.bytes() {
            return Err(Error::ChangedMessage);
        }

        if signature.as_bytes() == &[0; 64] {
            return Err(Error::InvalidSignature);
        }

        if !self.message.signers().contains(&signer) {
            return Err(Error::UnknownSigner);
        }

        let mut result = self.clone();

        // Received wire can repeat a signer address. One signature over these
        // identical bytes authorizes every slot owned by that same key.
        for (address, slot) in self.message.signers().iter().zip(&mut result.signatures) {
            if *address == signer {
                *slot = Some(signature);
            }
        }

        Ok(result)
    }

    /// Signs the exact immutable message using the deterministic local keypair.
    ///
    /// # Errors
    /// Rejects a keypair that does not own a required signature slot.
    #[cfg(feature = "crypto")]
    pub fn sign(&self, keypair: &crate::crypto::Keypair) -> Result<Self, Error> {
        self.with_signature(
            keypair.address(),
            self.message.bytes(),
            keypair.sign(self.message.bytes()),
        )
    }

    /// Checks every required signature against these exact message bytes.
    ///
    /// # Errors
    /// Rejects missing signatures, invalid keys, or signature/message mismatches.
    #[cfg(feature = "crypto")]
    pub fn verify(&self) -> Result<(), Error> {
        for (address, signature) in self.message.signers().iter().zip(&self.signatures) {
            let signature = signature.as_ref().ok_or(Error::MissingSignature)?;
            crate::crypto::verify(address, self.message.bytes(), signature)
                .map_err(|_| Error::InvalidSignature)?;
        }

        Ok(())
    }

    /// Encodes a packet, optionally requiring every signature slot to be filled.
    ///
    /// This is structural encoding; use `verify` for cryptographic authenticity.
    ///
    /// # Errors
    /// Rejects missing signatures when required, or an oversized packet.
    pub fn encode(&self, require_all_signatures: bool) -> Result<Vec<u8>, Error> {
        wire::encode_transaction(self, require_all_signatures)
    }
}

/// A structural, compilation, or signature failure at the transaction boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Input ends before a complete field.
    Truncated,
    /// Input contains bytes after the complete message.
    TrailingBytes,
    /// A length uses an overlong or overflowing short-u16 representation.
    InvalidLength,
    /// Version is not legacy or v0.
    UnsupportedVersion,
    /// Header partitions are inconsistent or lack a writable payer.
    InvalidHeader,
    /// An index or account count exceeds protocol bounds.
    InvalidIndex,
    /// The complete encoded transaction would exceed 1232 bytes.
    PacketTooLarge,
    /// Local compilation requires explicit known blockhash expiry metadata.
    UnresolvedLifetime,
    /// An invoked program is writable or is also the payer.
    WritableProgram,
    /// Lookup snapshots disagree, duplicate a table, or exceed 256 entries.
    InvalidLookup,
    /// Signer does not own a required slot.
    UnknownSigner,
    /// Signature was created for other message bytes.
    ChangedMessage,
    /// At least one required slot is unsigned.
    MissingSignature,
    /// Cryptographic signature validation failed, or zero was supplied as signed.
    InvalidSignature,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "transaction error: {self:?}")
    }
}
impl core::error::Error for Error {}

#[cfg(test)]
mod tests;
