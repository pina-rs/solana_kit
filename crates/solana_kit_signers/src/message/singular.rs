use alloc::{boxed::Box, vec::Vec};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use solana_kit_core::Address;

use super::{
    MAX_MESSAGES, MessageError, MessagePartialSigner, MessageSignatureOutput, SignableMessage,
};
use crate::{MessageSigner, Signer};

/// Explicit ordered adapter from singular signing to the bulk interface.
///
/// This adapter deliberately invokes the singular backend once per message.
/// Implement MessagePartialSigner directly for a genuine backend-native bulk call.
pub struct SingularMessageSigner<S>(pub S);

impl<S: Signer> Signer for SingularMessageSigner<S> {
    fn address(&self) -> Address {
        self.0.address()
    }
}

impl<S: MessageSigner> MessagePartialSigner for SingularMessageSigner<S> {
    type Sign<'a>
        = SingularBatch<'a, S>
    where
        Self: 'a;

    fn sign_messages<'a>(&'a self, messages: &'a [SignableMessage]) -> Self::Sign<'a> {
        SingularBatch {
            signer: &self.0,
            messages,
            pending: None,
            outputs: Vec::with_capacity(messages.len().min(MAX_MESSAGES)),
        }
    }
}

/// Ordered singular operations preserving their concrete associated-future auto traits.
pub struct SingularBatch<'a, S: MessageSigner> {
    signer: &'a S,
    messages: &'a [SignableMessage],
    pending: Option<Pin<Box<S::Sign<'a>>>>,
    outputs: Vec<MessageSignatureOutput>,
}

impl<S: MessageSigner> Future for SingularBatch<'_, S> {
    type Output = Result<Vec<MessageSignatureOutput>, MessageError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.messages.len() > MAX_MESSAGES {
            return Poll::Ready(Err(MessageError::TooManyMessages));
        }

        while let Some(message) = this.messages.get(this.outputs.len()) {
            let pending = this
                .pending
                .get_or_insert_with(|| Box::pin(this.signer.sign_message(message.content())));
            let output =
                core::task::ready!(pending.as_mut().poll(context)).map_err(MessageError::Signer)?;
            this.pending = None;
            this.outputs.push(MessageSignatureOutput::new(
                output.bytes(),
                [(output.address(), output.signature())],
            )?);
        }

        Poll::Ready(Ok(core::mem::take(&mut this.outputs)))
    }
}
