//! Portable message proof, deliberately unregistered in the shared runtime suite.
extern crate alloc;

/// Fixed distinct-message batch used by private runtime workload accounting.
pub const ACCEPTED_MESSAGES: usize = 256;

use alloc::{vec, vec::Vec};
use solana_kit_core::crypto::Keypair;
use solana_kit_signers::{
    SignerCapability,
    message::{
        MessageError, SignableMessage,
        local::{Composition, Registration},
    },
};

/// Preserves arbitrary bytes, UTF-8 and caller-owned signature snapshots.
#[must_use]
pub fn message_snapshots() -> bool {
    let key = Keypair::from_seed(&[7; 32]);
    let message = SignableMessage::with_signatures(
        b"exact bare bytes",
        [(key.address(), key.sign(b"exact bare bytes"))],
    );
    SignableMessage::from_text("é").content() == [195, 169]
        && SignableMessage::new([0, 255, 128]).content() == [0, 255, 128]
        && message.is_ok_and(|m| m.signatures()[&key.address()] == key.sign(m.content()))
}

/// Executes exact 256-message bulk success and rejects 257 before signing.
pub async fn message_batches() -> bool {
    let key = Keypair::from_seed(&[7; 32]);
    let handle = SignerCapability::new(&key);
    let Ok(pipeline) = Composition::new(vec![Registration::partial(&handle)]) else {
        return false;
    };
    let messages: Vec<_> = (0..ACCEPTED_MESSAGES)
        .map(|n| SignableMessage::new([n as u8]))
        .collect();

    let Ok(results) = pipeline.sign(&messages).await else {
        return false;
    };
    results.len() == ACCEPTED_MESSAGES
        && results.iter().zip(&messages).all(|(result, input)| {
            result.content() == input.content()
                && result.signatures().get(&key.address()) == Some(&key.sign(input.content()))
        })
        && pipeline.sign(&vec![SignableMessage::new([]); 257]).await
            == Err(MessageError::TooManyMessages)
}
