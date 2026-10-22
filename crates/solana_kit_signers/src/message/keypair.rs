use alloc::vec::Vec;
use core::future::{Ready, ready};
use solana_kit_core::crypto::Keypair;

use super::{
    MAX_MESSAGES, MessageError, MessagePartialSigner, MessageSignatureOutput, SignableMessage,
};

impl MessagePartialSigner for Keypair {
    type Sign<'a> = Ready<Result<Vec<MessageSignatureOutput>, MessageError>>;

    fn sign_messages<'a>(&'a self, messages: &'a [SignableMessage]) -> Self::Sign<'a> {
        if messages.len() > MAX_MESSAGES {
            return ready(Err(MessageError::TooManyMessages));
        }

        ready(
            messages
                .iter()
                .map(|message| {
                    MessageSignatureOutput::new(
                        message.content(),
                        [(self.address(), self.sign(message.content()))],
                    )
                })
                .collect(),
        )
    }
}
