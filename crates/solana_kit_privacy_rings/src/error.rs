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
