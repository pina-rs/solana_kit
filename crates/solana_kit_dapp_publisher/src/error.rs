//! Failures retain typed public facts without exposing transaction or payload bytes.

use solana_kit_core::{Address, codecs, transaction};
use std::fmt;

/// Failure to construct an attestation or establish publication transaction policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// A request identifier must contain exactly 32 ASCII decimal digits.
    InvalidRequestId,
    /// A caller-selected byte or aggregate allocation limit was exceeded.
    LimitExceeded,
    /// A bounded byte/text allocation or metadata codec failed.
    Codec(codecs::Error),
    /// The transaction wire or completed signature set is invalid.
    Transaction(transaction::Error),
    /// A present or newly supplied signature fails strict Ed25519 verification.
    InvalidSignature(Address),
    /// A supplied signature differs from an already occupied slot for this signer.
    OccupiedSignatureMismatch(Address),
    /// The message's lifetime token differs from the caller's expected blockhash.
    BlockhashMismatch,
    /// The first static account differs from the expected fee payer.
    PayerMismatch,
    /// The proposed local signer owns no required signature slot.
    SignerNotRequired,
    /// Required signers differ from the policy's exact address set.
    SignerSetMismatch,
    /// Lookup accounts require authenticated snapshots, unsupported in this slice.
    UnresolvedLookups,
    /// An instruction invokes a program outside the canonical publication allowlist.
    ProgramNotAllowed(Address),
    /// No instruction invokes the Token Metadata program.
    MissingMetadataInstruction,
    /// The first Token Metadata Create has no matching collection key.
    CollectionMismatch,
    /// The release mint's required existing signature is absent.
    MissingMintSignature,
    /// An expected collection-verification account is absent.
    MissingAccount(Address),
    /// Canonical metadata or master-edition PDA derivation failed.
    #[cfg(feature = "crypto")]
    Pda(solana_kit_core::crypto::PdaError),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Codec(cause) => write!(formatter, "publication codec: {cause}"),
            Self::Transaction(cause) => write!(formatter, "publication transaction: {cause}"),
            Self::InvalidSignature(address) => write!(formatter, "invalid signature for {address}"),
            Self::OccupiedSignatureMismatch(address) => {
                write!(
                    formatter,
                    "signature slot is already occupied for {address}"
                )
            }
            Self::ProgramNotAllowed(address) => write!(formatter, "unexpected program {address}"),
            Self::MissingAccount(address) => write!(formatter, "missing account {address}"),
            #[cfg(feature = "crypto")]
            Self::Pda(cause) => write!(formatter, "publication PDA: {cause}"),
            Self::InvalidRequestId => {
                formatter.write_str("expected 32 ASCII decimal request-id digits")
            }
            Self::LimitExceeded => formatter.write_str("publication resource limit exceeded"),
            Self::BlockhashMismatch => formatter.write_str("publication blockhash mismatch"),
            Self::PayerMismatch => formatter.write_str("publication fee payer mismatch"),
            Self::SignerNotRequired => formatter.write_str("publication signer is not required"),
            Self::SignerSetMismatch => formatter.write_str("publication signer set mismatch"),
            Self::UnresolvedLookups => {
                formatter.write_str("publication lookup accounts are unresolved")
            }
            Self::MissingMetadataInstruction => {
                formatter.write_str("missing Token Metadata instruction")
            }
            Self::CollectionMismatch => formatter.write_str("publication collection mismatch"),
            Self::MissingMintSignature => {
                formatter.write_str("missing existing release mint signature")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Codec(cause) => Some(cause),
            Self::Transaction(cause) => Some(cause),
            #[cfg(feature = "crypto")]
            Self::Pda(cause) => Some(cause),
            _ => None,
        }
    }
}
