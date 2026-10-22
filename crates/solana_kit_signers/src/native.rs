//! Explicit Send adapters for native multi-thread executors.
use crate::{
    Error, Role, SignatureOutput, Signer, SignerCapability, TransactionModifyingSigner,
    TransactionPartialSigner, TransactionSendingSigner,
    composition::{Descriptor, Interfaces, MAX_ITEMS, select_roles, validate_registrations},
};
use alloc::{boxed::Box, vec, vec::Vec};
use core::{
    future::{Future, poll_fn},
    pin::Pin,
};
use solana_kit_core::{Address, Signature, transaction::Transaction};
type Operation<'a, T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send + 'a>>;
trait PartialOperation {
    fn partial<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, SignatureOutput>;
}
trait ModifyOperation {
    fn modify<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, Transaction>;
}
trait SendOperation {
    fn send<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, Signature>;
}
struct Adapter<'a, P>(&'a SignerCapability<'a, P>);
impl<P: TransactionPartialSigner + Sync> PartialOperation for Adapter<'_, P>
where
    for<'t> <P as TransactionPartialSigner>::Sign<'t>: Send,
{
    fn partial<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, SignatureOutput> {
        Box::pin(self.0.signer.sign_transaction(tx))
    }
}
impl<P: TransactionModifyingSigner + Sync> ModifyOperation for Adapter<'_, P>
where
    for<'t> <P as TransactionModifyingSigner>::Modify<'t>: Send,
{
    fn modify<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, Transaction> {
        Box::pin(self.0.signer.modify_transaction(tx))
    }
}
impl<P: TransactionSendingSigner + Sync> SendOperation for Adapter<'_, P>
where
    for<'t> <P as TransactionSendingSigner>::Send<'t>: Send,
{
    fn send<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, Signature> {
        Box::pin(self.0.signer.send_transaction(tx))
    }
}
/// An explicit supported-interface declaration borrowing one logical capability.
///
/// Only declared interfaces are eligible for role selection. Constructors require
/// the corresponding backend traits; absent interfaces have no backend adapter.
pub struct Registration<'a> {
    info: Descriptor,
    partial: Option<Box<dyn PartialOperation + Send + Sync + 'a>>,
    modify: Option<Box<dyn ModifyOperation + Send + Sync + 'a>>,
    send: Option<Box<dyn SendOperation + Send + Sync + 'a>>,
}
impl<'a> Registration<'a> {
    /// Joins two registrations of heterogeneous backend types in input order.
    pub fn and(self, other: Self) -> Vec<Self> {
        vec![self, other]
    }

    /// Declares exactly the `partial` interfaces for this logical handle.
    pub fn partial<P: TransactionPartialSigner + Sync>(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self
    where
        for<'t> <P as TransactionPartialSigner>::Sign<'t>: Send,
    {
        Self {
            info: Descriptor {
                address: capability.address(),
                identity: capability as *const SignerCapability<'_, P> as usize,
                interfaces: Interfaces {
                    partial: true,
                    modifying: false,
                    sending: false,
                },
            },
            partial: Some(Box::new(Adapter(capability))),
            modify: None,
            send: None,
        }
    }

    /// Declares exactly the `modifying` interfaces for this logical handle.
    pub fn modifying<P: TransactionModifyingSigner + Sync>(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self
    where
        for<'t> <P as TransactionModifyingSigner>::Modify<'t>: Send,
    {
        Self {
            info: Descriptor {
                address: capability.address(),
                identity: capability as *const SignerCapability<'_, P> as usize,
                interfaces: Interfaces {
                    partial: false,
                    modifying: true,
                    sending: false,
                },
            },
            partial: None,
            modify: Some(Box::new(Adapter(capability))),
            send: None,
        }
    }

    /// Declares exactly the `sending` interfaces for this logical handle.
    pub fn sending<P: TransactionSendingSigner + Sync>(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self
    where
        for<'t> <P as TransactionSendingSigner>::Send<'t>: Send,
    {
        Self {
            info: Descriptor {
                address: capability.address(),
                identity: capability as *const SignerCapability<'_, P> as usize,
                interfaces: Interfaces {
                    partial: false,
                    modifying: false,
                    sending: true,
                },
            },
            partial: None,
            modify: None,
            send: Some(Box::new(Adapter(capability))),
        }
    }

    /// Declares exactly the `partial_modifying` interfaces for this logical handle.
    pub fn partial_modifying<P: TransactionPartialSigner + TransactionModifyingSigner + Sync>(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self
    where
        for<'t> <P as TransactionPartialSigner>::Sign<'t>: Send,
        for<'t> <P as TransactionModifyingSigner>::Modify<'t>: Send,
    {
        Self {
            info: Descriptor {
                address: capability.address(),
                identity: capability as *const SignerCapability<'_, P> as usize,
                interfaces: Interfaces {
                    partial: true,
                    modifying: true,
                    sending: false,
                },
            },
            partial: Some(Box::new(Adapter(capability))),
            modify: Some(Box::new(Adapter(capability))),
            send: None,
        }
    }

    /// Declares exactly the `partial_sending` interfaces for this logical handle.
    pub fn partial_sending<P: TransactionPartialSigner + TransactionSendingSigner + Sync>(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self
    where
        for<'t> <P as TransactionPartialSigner>::Sign<'t>: Send,
        for<'t> <P as TransactionSendingSigner>::Send<'t>: Send,
    {
        Self {
            info: Descriptor {
                address: capability.address(),
                identity: capability as *const SignerCapability<'_, P> as usize,
                interfaces: Interfaces {
                    partial: true,
                    modifying: false,
                    sending: true,
                },
            },
            partial: Some(Box::new(Adapter(capability))),
            modify: None,
            send: Some(Box::new(Adapter(capability))),
        }
    }

    /// Declares exactly the `modifying_sending` interfaces for this logical handle.
    pub fn modifying_sending<P: TransactionModifyingSigner + TransactionSendingSigner + Sync>(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self
    where
        for<'t> <P as TransactionModifyingSigner>::Modify<'t>: Send,
        for<'t> <P as TransactionSendingSigner>::Send<'t>: Send,
    {
        Self {
            info: Descriptor {
                address: capability.address(),
                identity: capability as *const SignerCapability<'_, P> as usize,
                interfaces: Interfaces {
                    partial: false,
                    modifying: true,
                    sending: true,
                },
            },
            partial: None,
            modify: Some(Box::new(Adapter(capability))),
            send: Some(Box::new(Adapter(capability))),
        }
    }

    /// Declares exactly the `composite` interfaces for this logical handle.
    pub fn composite<
        P: TransactionPartialSigner + TransactionModifyingSigner + TransactionSendingSigner + Sync,
    >(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self
    where
        for<'t> <P as TransactionPartialSigner>::Sign<'t>: Send,
        for<'t> <P as TransactionModifyingSigner>::Modify<'t>: Send,
        for<'t> <P as TransactionSendingSigner>::Send<'t>: Send,
    {
        Self {
            info: Descriptor {
                address: capability.address(),
                identity: capability as *const SignerCapability<'_, P> as usize,
                interfaces: Interfaces {
                    partial: true,
                    modifying: true,
                    sending: true,
                },
            },
            partial: Some(Box::new(Adapter(capability))),
            modify: Some(Box::new(Adapter(capability))),
            send: Some(Box::new(Adapter(capability))),
        }
    }
}

impl<'a> From<Registration<'a>> for Vec<Registration<'a>> {
    fn from(registration: Registration<'a>) -> Self {
        vec![registration]
    }
}

include!("composition_runtime.rs");
