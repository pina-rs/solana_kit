//! Explicit native message adapters retaining Send associated futures.
use alloc::{boxed::Box, vec, vec::Vec};
use core::{
    future::{Future, poll_fn},
    pin::Pin,
    task::Poll,
};
use solana_kit_core::Address;

use super::{
    MAX_MESSAGES, MessageError, MessageModifyingSigner, MessagePartialSigner,
    MessageSignatureOutput, SignableMessage, validation,
};
use crate::{Signer, SignerCapability};

type Operation<'a, T> = Pin<Box<dyn Future<Output = Result<Vec<T>, MessageError>> + Send + 'a>>;

trait PartialOperation {
    fn sign<'a>(&'a self, messages: &'a [SignableMessage])
    -> Operation<'a, MessageSignatureOutput>;
}
trait ModifyOperation {
    fn modify<'a>(&'a self, messages: &'a [SignableMessage]) -> Operation<'a, SignableMessage>;
}
struct Adapter<'a, P>(&'a SignerCapability<'a, P>);

impl<P: MessagePartialSigner + Sync> PartialOperation for Adapter<'_, P>
where
    for<'t> P::Sign<'t>: Send,
{
    fn sign<'a>(
        &'a self,
        messages: &'a [SignableMessage],
    ) -> Operation<'a, MessageSignatureOutput> {
        Box::pin(self.0.signer.sign_messages(messages))
    }
}
impl<P: MessageModifyingSigner + Sync> ModifyOperation for Adapter<'_, P>
where
    for<'t> P::Modify<'t>: Send,
{
    fn modify<'a>(&'a self, messages: &'a [SignableMessage]) -> Operation<'a, SignableMessage> {
        Box::pin(self.0.signer.modify_messages(messages))
    }
}

/// One explicit message interface borrowing a caller-owned logical capability.
///
/// Register a composite backend in exactly one role. Distinct handles claiming an
/// address conflict, including handles borrowing coincident zero-sized backends.
pub struct Registration<'a> {
    address: Address,
    identity: usize,
    partial: Option<Box<dyn PartialOperation + Send + Sync + 'a>>,
    modify: Option<Box<dyn ModifyOperation + Send + Sync + 'a>>,
}

impl<'a> Registration<'a> {
    /// Declares the whole-slice partial-signing interface.
    pub fn partial<P: MessagePartialSigner + Sync>(capability: &'a SignerCapability<'a, P>) -> Self
    where
        for<'t> P::Sign<'t>: Send,
    {
        Self {
            address: capability.address(),
            identity: capability as *const SignerCapability<'_, P> as usize,
            partial: Some(Box::new(Adapter(capability))),
            modify: None,
        }
    }

    /// Declares the whole-slice modifying interface.
    pub fn modifying<P: MessageModifyingSigner + Sync>(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self
    where
        for<'t> P::Modify<'t>: Send,
    {
        Self {
            address: capability.address(),
            identity: capability as *const SignerCapability<'_, P> as usize,
            partial: None,
            modify: Some(Box::new(Adapter(capability))),
        }
    }
}

include!("runtime.rs");
