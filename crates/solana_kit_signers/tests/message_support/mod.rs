use solana_kit_core::{Address, Signature};
use solana_kit_signers::{
    Signer,
    message::{MessageError, MessagePartialSigner, MessageSignatureOutput, SignableMessage},
};
use std::{
    cell::Cell,
    future::{Ready, ready},
};

pub fn address(seed: u8) -> Address {
    #[cfg(feature = "crypto")]
    {
        solana_kit_core::crypto::Keypair::from_seed(&[seed; 32]).address()
    }
    #[cfg(not(feature = "crypto"))]
    {
        Address::from_bytes([seed; 32])
    }
}

pub fn signature(seed: u8, bytes: &[u8]) -> Signature {
    #[cfg(feature = "crypto")]
    {
        solana_kit_core::crypto::Keypair::from_seed(&[seed; 32]).sign(bytes)
    }
    #[cfg(not(feature = "crypto"))]
    {
        let mut signature = [seed.max(1); 64];
        signature[0] = bytes.iter().fold(seed, |sum, byte| sum.wrapping_add(*byte));
        Signature::from_bytes(signature)
    }
}

pub fn output(seed: u8, message: &SignableMessage) -> MessageSignatureOutput {
    MessageSignatureOutput::new(
        message.content(),
        [(address(seed), signature(seed, message.content()))],
    )
    .unwrap()
}

#[derive(Clone, Copy)]
pub enum Behavior {
    Good,
    Short,
    Long,
    Foreign,
    Changed,
    Missing,
    Zero,
    Invalid,
    Reject,
}

pub struct Backend {
    pub seed: u8,
    pub behavior: Behavior,
    pub calls: Cell<usize>,
    pub items: Cell<usize>,
}

impl Backend {
    pub fn new(seed: u8, behavior: Behavior) -> Self {
        Self {
            seed,
            behavior,
            calls: Cell::new(0),
            items: Cell::new(0),
        }
    }
}

impl Signer for Backend {
    fn address(&self) -> Address {
        address(self.seed)
    }
}

impl MessagePartialSigner for Backend {
    type Sign<'a> = Ready<Result<Vec<MessageSignatureOutput>, MessageError>>;

    fn sign_messages<'a>(&'a self, messages: &'a [SignableMessage]) -> Self::Sign<'a> {
        self.calls.set(self.calls.get() + 1);
        self.items.set(self.items.get() + messages.len());
        let mut outputs: Vec<_> = messages.iter().map(|m| output(self.seed, m)).collect();

        match self.behavior {
            Behavior::Good => {}
            Behavior::Short => {
                outputs.pop();
            }
            Behavior::Long => {
                outputs.push(output(self.seed, &SignableMessage::new([])));
            }
            Behavior::Foreign => {
                outputs[0] = output(self.seed + 1, &messages[0]);
            }
            Behavior::Changed => {
                outputs[0] = output(self.seed, &SignableMessage::new(b"changed"));
            }
            Behavior::Missing => {
                outputs[0] = MessageSignatureOutput::new(messages[0].content(), []).unwrap();
            }
            Behavior::Zero => {
                outputs[0] = MessageSignatureOutput::new(
                    messages[0].content(),
                    [(self.address(), Signature::from_bytes([0; 64]))],
                )
                .unwrap();
            }
            Behavior::Invalid => {
                outputs[0] = MessageSignatureOutput::new(
                    messages[0].content(),
                    [(self.address(), Signature::from_bytes([42; 64]))],
                )
                .unwrap();
            }
            Behavior::Reject => return ready(Err(MessageError::Rejected)),
        }

        ready(Ok(outputs))
    }
}
