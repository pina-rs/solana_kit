//! Failures that stay distinct from transport and remote JSON-RPC errors.

use std::{error::Error as StdError, fmt};

/// A local Privacy Rings encoding, account, or decryption failure.
#[derive(Debug)]
pub enum Error {
    /// A batch, page, or account list violated a documented bound.
    Limit(&'static str),
    /// Two parts of one instruction described different settlement legs.
    SettlementMismatch,
    /// A deposit or withdrawal amount was zero.
    ZeroAmount,
    /// A base64 payload was not the documented standard alphabet.
    Base64,
    /// A viewing key, salt, or ciphertext had the wrong length or tag.
    Key,
    /// The ciphertext was not addressed to this viewing key.
    NotForViewer,
    /// A decrypted or proofless payload did not match the documented layout.
    Plaintext,
    /// An X25519 peer produced a non-contributory shared secret.
    NonContributory,
    /// PDA derivation failed for the supplied seeds.
    #[cfg(feature = "crypto")]
    Pda(solana_kit_core::crypto::PdaError),
    /// The remote prover refused the call or returned a proof this client will not use.
    Prover(ProverFailure),
}

/// A prover request that did not produce a usable proof.
///
/// Variants carry no URL, witness, or API key. An HTTP status is the only
/// remote detail retained.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProverFailure {
    /// The circuit name is empty or is not one path segment.
    Circuit,
    /// The endpoint URL is not usable, or a remote URL is not `https`.
    Config,
    /// A non-loopback prover was asked to take a cleartext witness.
    Cleartext,
    /// The prover returned a status this client does not retry.
    Http(u16),
    /// The job id was not a safe token.
    Job,
    /// Status polling exceeded the wait bound.
    Timeout,
    /// The body was not the documented JSON object.
    Response,
    /// The prover reported a failed job or a null proof.
    Server,
    /// The transport failed. The underlying message is discarded.
    Transport,
    /// No proving-key pin is known for this circuit.
    UnknownCircuit,
    /// The response omitted `provingKeySha256`.
    ProvingKeyMissing,
    /// The reported pin does not match the pinned key.
    ProvingKeyMismatch,
    /// A G1 point is not on the curve.
    ProofPoint,
    /// A proof field had the wrong shape.
    ProofParse,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "privacy rings client failed: {self:?}")
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        #[cfg(feature = "crypto")]
        if let Self::Pda(error) = self {
            return Some(error);
        }
        let _ = self;
        None
    }
}
