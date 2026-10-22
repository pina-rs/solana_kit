//! Immutable bare messages and whole-slice backend contracts.

use alloc::{collections::BTreeMap, vec::Vec};
use core::{fmt, future::Future};

use solana_kit_core::{Address, Signature};

use crate::Signer;

pub mod local;
pub mod native;
mod singular;
mod validation;

#[cfg(feature = "crypto")]
mod keypair;

pub use singular::{SingularBatch, SingularMessageSigner};

/// Maximum number of messages or registrations accepted before backend effects.
pub const MAX_MESSAGES: usize = 256;

/// An immutable address-to-signature dictionary owned by Kit.
pub type SignatureDictionary = BTreeMap<Address, Signature>;

/// Exact bare content and its current signatures, without an off-chain envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignableMessage {
    content: Vec<u8>,
    signatures: SignatureDictionary,
}

impl SignableMessage {
    /// Copies arbitrary bytes into an unsigned immutable snapshot. No size cap applies.
    #[must_use]
    pub fn new(content: impl AsRef<[u8]>) -> Self {
        Self {
            content: content.as_ref().into(),
            signatures: SignatureDictionary::new(),
        }
    }

    /// Copies exact UTF-8 bytes, without normalization or an automatic envelope.
    #[must_use]
    pub fn from_text(content: &str) -> Self {
        Self::new(content.as_bytes())
    }

    /// Copies bytes and records existing signatures without authenticating them.
    ///
    /// Authentication occurs at the pipeline boundary when `crypto` is enabled.
    ///
    /// # Errors
    /// Rejects duplicate addresses instead of silently overwriting their signatures.
    pub fn with_signatures(
        content: impl AsRef<[u8]>,
        signatures: impl IntoIterator<Item = (Address, Signature)>,
    ) -> Result<Self, MessageError> {
        let mut result = Self::new(content);

        for (address, signature) in signatures {
            if result.signatures.insert(address, signature).is_some() {
                return Err(MessageError::DuplicateSignature);
            }
        }

        Ok(result)
    }

    /// Exact immutable bare bytes authorized by signatures.
    #[must_use]
    pub fn content(&self) -> &[u8] {
        &self.content
    }

    /// Existing signatures, ordered by address for deterministic iteration.
    #[must_use]
    pub fn signatures(&self) -> &SignatureDictionary {
        &self.signatures
    }
}

/// One backend's signature dictionary and exact bytes authorized for one message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageSignatureOutput(SignableMessage);

impl MessageSignatureOutput {
    /// Records the exact snapshot and signatures claimed by a backend.
    ///
    /// # Errors
    /// Rejects duplicate addresses. Ownership and authentication are checked on merge.
    pub fn new(
        content: impl AsRef<[u8]>,
        signatures: impl IntoIterator<Item = (Address, Signature)>,
    ) -> Result<Self, MessageError> {
        Ok(Self(SignableMessage::with_signatures(content, signatures)?))
    }

    /// Exact bytes the backend claims to have authorized.
    #[must_use]
    pub fn content(&self) -> &[u8] {
        self.0.content()
    }

    /// The backend's immutable signature dictionary.
    #[must_use]
    pub fn signatures(&self) -> &SignatureDictionary {
        self.0.signatures()
    }
}

/// Signs a whole bounded slice and returns one signature dictionary per message.
///
/// Explicit registration rejects a value without this interface at compile time.
///
/// ```compile_fail
/// use solana_kit_core::Address;
/// use solana_kit_signers::{Signer, SignerCapability, message::local::Registration};
/// struct AddressOnly;
/// impl Signer for AddressOnly {
///     fn address(&self) -> Address { Address::from_bytes([1; 32]) }
/// }
/// let backend = AddressOnly;
/// let handle = SignerCapability::new(&backend);
/// let registration = Registration::partial(&handle);
/// ```
pub trait MessagePartialSigner: Signer {
    /// Backend future, retaining its actual local or native auto traits.
    type Sign<'a>: Future<Output = Result<Vec<MessageSignatureOutput>, MessageError>> + 'a
    where
        Self: 'a;

    /// Receives the actual whole input slice once, in order.
    fn sign_messages<'a>(&'a self, messages: &'a [SignableMessage]) -> Self::Sign<'a>;
}

/// May replace messages sequentially before partial signers see their final bytes.
///
/// ```compile_fail
/// use solana_kit_core::Address;
/// use solana_kit_signers::{Signer, SignerCapability, message::local::Registration};
/// struct AddressOnly;
/// impl Signer for AddressOnly {
///     fn address(&self) -> Address { Address::from_bytes([1; 32]) }
/// }
/// let backend = AddressOnly;
/// let handle = SignerCapability::new(&backend);
/// let registration = Registration::modifying(&handle);
/// ```
pub trait MessageModifyingSigner: Signer {
    /// Backend future, without a universal `Send` requirement.
    type Modify<'a>: Future<Output = Result<Vec<SignableMessage>, MessageError>> + 'a
    where
        Self: 'a;

    /// Receives the whole slice once and returns exactly one replacement per input.
    fn modify_messages<'a>(&'a self, messages: &'a [SignableMessage]) -> Self::Modify<'a>;
}

/// Precise failures at the message capability boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageError {
    /// The backend or user rejected authorization.
    Rejected,
    /// The caller or backend cancelled the operation.
    Cancelled,
    /// More than 256 message inputs were supplied.
    TooManyMessages,
    /// More than 256 registrations were supplied before deduplication.
    TooManySigners,
    /// Distinct logical handles claim one address, or declare conflicting interfaces.
    ConflictingSigner,
    /// The backend returned a different number of results than inputs.
    Cardinality,
    /// One dictionary contains an address twice.
    DuplicateSignature,
    /// A backend omitted its own signature.
    MissingSignature,
    /// A backend claimed authorization owned by another address.
    ForeignSigner,
    /// A partial signed changed bytes, or a modifier reused a stale signature.
    ChangedSnapshot,
    /// A zero signature, or an invalid signature with `crypto`, was supplied.
    InvalidSignature,
    /// An explicit singular adapter propagated an existing signer failure.
    Signer(crate::Error),
}

impl fmt::Display for MessageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "message signer error: {self:?}")
    }
}

impl core::error::Error for MessageError {}
