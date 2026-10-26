//! Local deterministic authentication signing with explicit caller time and key ownership.

use solana_kit_core::{Signature, crypto::Keypair};

/// Exact verification text from the pinned Helius signing contract.
pub const VERIFICATION_TEXT: &str =
    "Please sign this message to verify ownership of your wallet and connect to Helius.";

/// An authentication message and its Kit-owned Ed25519 signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedAuthMessage {
    /// Exact UTF-8 bytes signed, represented as an owned string.
    pub message: String,
    /// Deterministic signature over the exact message bytes.
    pub signature: Signature,
}

/// Builds the default authentication message using caller-supplied Unix milliseconds.
pub fn auth_message(timestamp: u64) -> Result<String, serde_json::Error> {
    #[derive(serde::Serialize)]
    struct Message {
        message: &'static str,
        timestamp: u64,
    }

    serde_json::to_string(&Message {
        message: VERIFICATION_TEXT,
        timestamp,
    })
}

/// Signs a caller message or the exact default message without clocks or secret export.
pub fn sign_auth_message(
    keypair: &Keypair,
    message: Option<String>,
    timestamp: u64,
) -> Result<SignedAuthMessage, serde_json::Error> {
    let message = match message {
        Some(message) => message,
        None => auth_message(timestamp)?,
    };

    let signature = keypair.sign(message.as_bytes());
    Ok(SignedAuthMessage { message, signature })
}
