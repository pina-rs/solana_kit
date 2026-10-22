use crate::Wallet;
use alloc::{
    collections::BTreeMap,
    rc::{Rc, Weak},
    string::ToString,
    vec::Vec,
};
use core::{cell::RefCell, fmt};

/// Registry mutation category.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistryEventKind {
    /// Wallet became available.
    Registered,
    /// Wallet became unavailable.
    Unregistered,
    /// Registered wallet properties changed.
    Changed,
}

/// A synchronous registry event retaining the affected wallet.
#[derive(Clone)]
pub struct RegistryEvent {
    /// Mutation category.
    pub kind: RegistryEventKind,
    /// Caller-supplied wallet identity.
    pub wallet: Rc<dyn Wallet>,
}

/// Registration or listener creation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistryError {
    /// The registry was disposed.
    Disposed,
    /// The finite identity space was exhausted.
    IdentityExhausted,
    /// All 32 name-observation passes were invalidated by reentrant mutations.
    /// No registration or event was created by this call; nested effects remain.
    UnstableRegistration,
}

type Listener = Rc<dyn Fn(&RegistryEvent)>;
const REGISTRATION_ATTEMPTS: usize = 32;

enum RegistrationDecision {
    Retry,
    Duplicate,
    Registered(u64),
}

#[derive(Default)]
struct State {
    wallets: BTreeMap<u64, Rc<dyn Wallet>>,
    listeners: BTreeMap<u64, Listener>,
    next: u64,
    disposed: bool,
    revision: Rc<()>,
}

impl State {
    // This commit step calls no wallet code and independently deduplicates identity.
    fn register(
        &mut self,
        wallet: &Rc<dyn Wallet>,
        revision: &Rc<()>,
        duplicate: bool,
    ) -> Result<RegistrationDecision, RegistryError> {
        if self.disposed {
            return Err(RegistryError::Disposed);
        }

        if self
            .wallets
            .values()
            .any(|existing| Rc::ptr_eq(existing, wallet))
        {
            return Ok(RegistrationDecision::Duplicate);
        }

        if !Rc::ptr_eq(revision, &self.revision) {
            return Ok(RegistrationDecision::Retry);
        }

        if duplicate {
            return Ok(RegistrationDecision::Duplicate);
        }

        let id = self.allocate()?;
        self.wallets.insert(id, wallet.clone());
        self.revision = Rc::new(());
        Ok(RegistrationDecision::Registered(id))
    }

    fn allocate(&mut self) -> Result<u64, RegistryError> {
        if self.disposed {
            return Err(RegistryError::Disposed);
        }

        self.next = self
            .next
            .checked_add(1)
            .ok_or(RegistryError::IdentityExhausted)?;
        Ok(self.next)
    }
}

/// Caller-owned passive registry. Clones share the same local registry.
/// No singleton, discovery bridge, runtime, or background task is installed.
#[derive(Clone, Default)]
pub struct WalletRegistry(Rc<RefCell<State>>);

impl WalletRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Passive initialization has no discovery source and performs no work.
    pub fn initialize(&self) {}
    /// Returns an owned snapshot in registration order.
    pub fn wallets(&self) -> Vec<Rc<dyn Wallet>> {
        self.0.borrow().wallets.values().cloned().collect()
    }

    /// Registers once by Rc identity and observed current name. Duplicate guards
    /// are inert. Dropping a successful guard unregisters only that registration,
    /// even after reentry.
    ///
    /// Name getters run outside registry borrows. Membership changes and
    /// [`Self::changed`] invalidate the observation pass. Up to 32 passes permit
    /// finite nested operations; persistent mutation returns
    /// [`RegistryError::UnstableRegistration`] without inserting this wallet.
    /// Names mutated without notification cannot provide an atomic snapshot:
    /// deduplication compares the names observed during the accepted pass.
    pub fn register(&self, wallet: Rc<dyn Wallet>) -> Result<Registration, RegistryError> {
        if self.0.borrow().disposed {
            return Err(RegistryError::Disposed);
        }

        for _ in 0..REGISTRATION_ATTEMPTS {
            let revision = self.0.borrow().revision.clone();
            let wallets = self.wallets();
            let name = wallet.name().to_string();
            let duplicate = wallets.iter().any(|existing| existing.name() == name);
            let decision = self
                .0
                .borrow_mut()
                .register(&wallet, &revision, duplicate)?;

            let id = match decision {
                RegistrationDecision::Retry => continue,
                RegistrationDecision::Duplicate => {
                    return Ok(Registration {
                        state: Weak::new(),
                        id: None,
                    });
                }
                RegistrationDecision::Registered(id) => id,
            };

            emit(
                &self.0,
                RegistryEvent {
                    kind: RegistryEventKind::Registered,
                    wallet,
                },
            );

            return Ok(Registration {
                state: Rc::downgrade(&self.0),
                id: Some(id),
            });
        }

        Err(RegistryError::UnstableRegistration)
    }

    /// Removes the identical wallet when present. A same-name wallet differs.
    pub fn unregister(&self, wallet: &Rc<dyn Wallet>) {
        let id = self
            .0
            .borrow()
            .wallets
            .iter()
            .find(|(_, existing)| Rc::ptr_eq(existing, wallet))
            .map(|(id, _)| *id);

        if let Some(id) = id {
            remove(&self.0, id);
        }
    }

    /// Emits a change only for an identical currently registered wallet.
    pub fn changed(&self, wallet: &Rc<dyn Wallet>) {
        let present = {
            let mut state = self.0.borrow_mut();
            let present = state
                .wallets
                .values()
                .any(|existing| Rc::ptr_eq(existing, wallet));

            if present {
                state.revision = Rc::new(());
            }

            present
        };

        if present {
            emit(
                &self.0,
                RegistryEvent {
                    kind: RegistryEventKind::Changed,
                    wallet: wallet.clone(),
                },
            );
        }
    }

    /// Subscribes synchronously. Callbacks execute without shared mutable borrows.
    /// A dispatch snapshots listeners. Removal affects later dispatches.
    pub fn on_event(
        &self,
        listener: impl Fn(&RegistryEvent) + 'static,
    ) -> Result<RegistrySubscription, RegistryError> {
        let mut state = self.0.borrow_mut();
        let id = state.allocate()?;
        state.listeners.insert(id, Rc::new(listener));
        Ok(RegistrySubscription {
            state: Rc::downgrade(&self.0),
            id: Some(id),
        })
    }

    /// Idempotently releases wallets and listeners without mutation events.
    pub fn dispose(&self) {
        let released = {
            let mut state = self.0.borrow_mut();
            state.disposed = true;
            state.revision = Rc::new(());
            (
                core::mem::take(&mut state.wallets),
                core::mem::take(&mut state.listeners),
            )
        };
        drop(released);
    }
}

fn emit(state: &Rc<RefCell<State>>, event: RegistryEvent) {
    let listeners: Vec<_> = state.borrow().listeners.values().cloned().collect();

    for listener in listeners {
        listener(&event);
    }
}

fn remove(state: &Rc<RefCell<State>>, id: u64) {
    let wallet = {
        let mut state = state.borrow_mut();
        let wallet = state.wallets.remove(&id);

        if wallet.is_some() {
            state.revision = Rc::new(());
        }

        wallet
    };

    if let Some(wallet) = wallet {
        emit(
            state,
            RegistryEvent {
                kind: RegistryEventKind::Unregistered,
                wallet,
            },
        );
    }
}

/// An idempotent registration owner. Drop unregisters without retaining registry
/// ownership. Keep this guard alive for as long as the wallet should be present.
pub struct Registration {
    state: Weak<RefCell<State>>,
    id: Option<u64>,
}

impl Registration {
    /// Removes this registration once. Inert duplicate guards do nothing.
    pub fn unregister(&mut self) {
        if let (Some(id), Some(state)) = (self.id.take(), self.state.upgrade()) {
            remove(&state, id);
        }
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        self.unregister();
    }
}

/// Listener owner. Drop removes the listener without retaining the registry.
pub struct RegistrySubscription {
    state: Weak<RefCell<State>>,
    id: Option<u64>,
}

impl RegistrySubscription {
    /// Idempotently removes the listener.
    pub fn unsubscribe(&mut self) {
        if let (Some(id), Some(state)) = (self.id.take(), self.state.upgrade()) {
            let removed = state.borrow_mut().listeners.remove(&id);
            drop(removed);
        }
    }
}

impl Drop for RegistrySubscription {
    fn drop(&mut self) {
        self.unsubscribe();
    }
}

impl fmt::Display for RegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl core::error::Error for RegistryError {}
