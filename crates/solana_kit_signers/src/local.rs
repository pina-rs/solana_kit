//! Local adapters supporting Rc-backed futures across suspension.
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
type Operation<'a, T, Failure> = Pin<Box<dyn Future<Output = Result<T, Failure>> + 'a>>;
trait PartialOperation<Failure> {
    fn partial<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, SignatureOutput, Failure>;
}
trait ModifyOperation<Failure> {
    fn modify<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, Transaction, Failure>;
}
trait SendOperation<Failure> {
    fn send<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, Signature, Failure>;
}
struct Adapter<'a, P>(&'a SignerCapability<'a, P>);
impl<Failure, P: TransactionPartialSigner<Failure>> PartialOperation<Failure> for Adapter<'_, P> {
    fn partial<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, SignatureOutput, Failure> {
        Box::pin(self.0.signer.sign_transaction(tx))
    }
}
impl<Failure, P: TransactionModifyingSigner<Failure>> ModifyOperation<Failure> for Adapter<'_, P> {
    fn modify<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, Transaction, Failure> {
        Box::pin(self.0.signer.modify_transaction(tx))
    }
}
impl<Failure, P: TransactionSendingSigner<Failure>> SendOperation<Failure> for Adapter<'_, P> {
    fn send<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, Signature, Failure> {
        Box::pin(self.0.signer.send_transaction(tx))
    }
}
/// An explicit supported-interface declaration borrowing one logical capability.
///
/// Only declared interfaces are eligible for role selection. Constructors require
/// the corresponding backend traits; absent interfaces have no backend adapter.
pub struct Registration<'a, Failure = Error> {
    info: Descriptor,
    partial: Option<Box<dyn PartialOperation<Failure> + 'a>>,
    modify: Option<Box<dyn ModifyOperation<Failure> + 'a>>,
    send: Option<Box<dyn SendOperation<Failure> + 'a>>,
}
impl<'a, Failure: 'a> Registration<'a, Failure> {
    /// Joins two registrations of heterogeneous backend types in input order.
    pub fn and(self, other: Self) -> Vec<Self> {
        vec![self, other]
    }

    /// Declares exactly the `partial` interfaces for this logical handle.
    pub fn partial<P: TransactionPartialSigner<Failure>>(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self {
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
    pub fn modifying<P: TransactionModifyingSigner<Failure>>(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self {
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
    pub fn sending<P: TransactionSendingSigner<Failure>>(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self {
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
    pub fn partial_modifying<
        P: TransactionPartialSigner<Failure> + TransactionModifyingSigner<Failure>,
    >(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self {
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
    pub fn partial_sending<
        P: TransactionPartialSigner<Failure> + TransactionSendingSigner<Failure>,
    >(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self {
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
    pub fn modifying_sending<
        P: TransactionModifyingSigner<Failure> + TransactionSendingSigner<Failure>,
    >(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self {
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
        P: TransactionPartialSigner<Failure>
            + TransactionModifyingSigner<Failure>
            + TransactionSendingSigner<Failure>,
    >(
        capability: &'a SignerCapability<'a, P>,
    ) -> Self {
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

impl<'a, Failure> From<Registration<'a, Failure>> for Vec<Registration<'a, Failure>> {
    fn from(registration: Registration<'a, Failure>) -> Self {
        vec![registration]
    }
}

include!("composition_runtime.rs");

struct MappedPartial<'a, P, Source, Failure> {
    capability: &'a SignerCapability<'a, P>,
    map: fn(Source) -> Failure,
}

impl<P, Source, Failure> PartialOperation<Failure> for MappedPartial<'_, P, Source, Failure>
where
    P: TransactionPartialSigner<Source>,
{
    fn partial<'a>(&'a self, tx: &'a Transaction) -> Operation<'a, SignatureOutput, Failure> {
        Box::pin(async move {
            self.capability
                .signer
                .sign_transaction(tx)
                .await
                .map_err(self.map)
        })
    }
}

impl<'a, Failure: 'a> Registration<'a, Failure> {
    /// Includes a backend with another concrete failure type in this composition.
    /// The caller chooses a lossless error owner; no backend error is discarded.
    pub fn partial_with<P, Source: 'a>(
        capability: &'a SignerCapability<'a, P>,
        map_error: fn(Source) -> Failure,
    ) -> Self
    where
        P: TransactionPartialSigner<Source>,
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
            partial: Some(Box::new(MappedPartial {
                capability,
                map: map_error,
            })),
            modify: None,
            send: None,
        }
    }
}
