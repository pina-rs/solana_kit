use super::{
    BrowserWallet, WalletDiscovery, WalletFailure, WalletSigner, operation::Token,
    session::WalletSubscription,
};
use solana_kit_wallet::{
    RegistryEventKind, RegistrySubscription, Wallet, WalletAccount, WalletErrorCode,
};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    future::Future,
    pin::Pin,
    rc::{Rc, Weak},
};

/// Local operation future; browser state is intentionally not Send.
pub type WalletFuture<T> = Pin<Box<dyn Future<Output = Result<T, WalletFailure>>>>;
type Listener = Rc<dyn Fn(&WalletState)>;

/// Valid connection states, including the account required by Connected.
#[derive(Clone)]
pub enum Connection {
    /// No current authorization.
    Disconnected,
    /// Waiting for this wallet's authorization result.
    Connecting(Rc<BrowserWallet>),
    /// This wallet currently authorizes this exact account snapshot.
    Connected {
        /// Current wallet identity.
        wallet: Rc<BrowserWallet>,
        /// Selected account.
        account: WalletAccount,
    },
    /// The owning controller has been disposed.
    Disposed,
}

impl Connection {
    fn wallet(&self) -> Option<&Rc<BrowserWallet>> {
        match self {
            Self::Connecting(wallet) | Self::Connected { wallet, .. } => Some(wallet),
            Self::Disconnected | Self::Disposed => None,
        }
    }
}

/// Immutable state snapshot delivered outside controller borrows.
#[derive(Clone)]
pub struct WalletState {
    /// Connection and its current authorization.
    pub connection: Connection,
    /// Whether a signing or disconnect prompt is outstanding.
    pub operating: bool,
    /// Last operation failure, retaining an inspectable cause.
    pub error: Option<WalletFailure>,
}

pub(super) struct Inner {
    pub discovery: Rc<WalletDiscovery>,
    pub chain: String,
    pub state: RefCell<WalletState>,
    pub authorization: RefCell<Rc<()>>,
    operation: RefCell<Option<Rc<Token>>>,
    wallet_subscription: RefCell<Option<WalletSubscription>>,
    registry_subscription: RefCell<Option<RegistrySubscription>>,
    listeners: RefCell<BTreeMap<u64, Listener>>,
    next_listener: RefCell<u64>,
}

impl Inner {
    pub fn current(&self, token: &Rc<Token>) -> bool {
        self.operation
            .borrow()
            .as_ref()
            .is_some_and(|current| Rc::ptr_eq(current, token))
            && token.valid()
    }

    pub fn notify(&self) {
        let state = self.state.borrow().clone();
        let listeners: Vec<_> = self.listeners.borrow().values().cloned().collect();

        for listener in listeners {
            listener(&state);
        }
    }

    fn invalidate(&self) -> Option<Rc<Token>> {
        let previous = self.operation.borrow_mut().take();
        *self.authorization.borrow_mut() = Rc::new(());

        previous
    }

    fn disconnected(&self, error: Option<WalletFailure>) {
        let previous = self.invalidate();
        *self.state.borrow_mut() = WalletState {
            connection: Connection::Disconnected,
            operating: false,
            error,
        };
        let subscription = self.wallet_subscription.borrow_mut().take();
        drop(subscription);
        cancel(previous);
        self.notify();
    }

    fn wallet_changed(&self) {
        let connection = self.state.borrow().connection.clone();
        let Some(wallet) = connection.wallet() else {
            return;
        };

        if !wallet.is_available() || !wallet.chains().contains(&self.chain) {
            self.disconnected(None);

            return;
        }

        let Connection::Connected { wallet, account } = connection else {
            // Wallets commonly publish accounts synchronously before connect resolves.
            return;
        };
        let accounts = authorized(&wallet, &self.chain);
        let selected = accounts
            .iter()
            .find(|next| next.public_key() == account.public_key())
            .or_else(|| accounts.first())
            .cloned();
        let previous = self.invalidate();
        let next = selected
            .map(|account| Connection::Connected { wallet, account })
            .unwrap_or(Connection::Disconnected);
        let mut state = self.state.borrow_mut();
        state.connection = next;

        state.operating = false;
        drop(state);
        cancel(previous);
        self.notify();
    }

    pub fn begin(self: &Rc<Self>) -> Result<OperationGuard, WalletFailure> {
        if self.operation.borrow().is_some() {
            return Err(WalletFailure::new(
                WalletErrorCode::InvalidRequest,
                "a wallet operation is already outstanding",
            ));
        }

        let token = Token::new();
        *self.operation.borrow_mut() = Some(token.clone());
        self.state.borrow_mut().operating = true;
        self.notify();
        Ok(OperationGuard {
            inner: Rc::downgrade(self),
            token,
            connecting: false,
            finished: false,
        })
    }
}

/// Application-owned controller. Futures and signers retain only weak authority.
/// Dropping this owner invalidates and wakes all pending caller operations.
pub struct WalletController(Rc<Inner>);

impl WalletController {
    /// Creates a controller for one validated Solana chain.
    pub fn new(discovery: Rc<WalletDiscovery>, chain: String) -> Result<Self, WalletFailure> {
        if !chain.starts_with("solana:") || !solana_kit_wallet::is_identifier(&chain) {
            return Err(WalletFailure::new(
                WalletErrorCode::InvalidRequest,
                "unsupported Solana chain",
            ));
        }

        let inner = Rc::new(Inner {
            discovery,
            chain,
            state: RefCell::new(WalletState {
                connection: Connection::Disconnected,
                operating: false,
                error: None,
            }),
            authorization: RefCell::new(Rc::new(())),
            operation: RefCell::new(None),
            wallet_subscription: RefCell::new(None),
            registry_subscription: RefCell::new(None),
            listeners: RefCell::new(BTreeMap::new()),
            next_listener: RefCell::new(0),
        });
        let weak = Rc::downgrade(&inner);
        let subscription = inner
            .discovery
            .registry()
            .on_event(move |event| {
                let Some(inner) = weak.upgrade() else { return };
                let selected = inner.state.borrow().connection.wallet().cloned();

                if event.kind == RegistryEventKind::Unregistered
                    && selected.is_some_and(|wallet| {
                        let erased: Rc<dyn Wallet> = wallet;
                        Rc::ptr_eq(&erased, &event.wallet)
                    })
                {
                    inner.disconnected(None);
                }
            })
            .map_err(|error| WalletFailure::caused("wallet registry is unavailable", error))?;
        *inner.registry_subscription.borrow_mut() = Some(subscription);
        Ok(Self(inner))
    }

    /// Current immutable state.
    pub fn state(&self) -> WalletState {
        self.0.state.borrow().clone()
    }

    /// Observes future state changes until the returned owner is dropped.
    pub fn on_change(
        &self,
        listener: impl Fn(&WalletState) + 'static,
    ) -> Result<ControllerSubscription, WalletFailure> {
        self.active()?;
        let id = self
            .0
            .next_listener
            .borrow()
            .checked_add(1)
            .ok_or_else(WalletFailure::invalid)?;
        *self.0.next_listener.borrow_mut() = id;
        self.0.listeners.borrow_mut().insert(id, Rc::new(listener));
        Ok(ControllerSubscription {
            inner: Rc::downgrade(&self.0),
            id,
        })
    }

    /// Starts authorization immediately. Dropping even an unpolled future resets Connecting.
    pub fn connect(
        &self,
        wallet: Rc<BrowserWallet>,
        silent: bool,
    ) -> Result<WalletFuture<()>, WalletFailure> {
        self.active()?;

        if !self
            .0
            .discovery
            .wallets()
            .iter()
            .any(|entry| entry.same(&wallet))
        {
            return Err(WalletFailure::disconnected());
        }

        if !wallet.chains().contains(&self.0.chain) {
            return Err(WalletFailure::new(
                WalletErrorCode::UnsupportedFeature,
                "wallet does not support the requested chain",
            ));
        }

        wallet.feature(solana_kit_wallet::CONNECT)?;
        wallet.feature(solana_kit_wallet::EVENTS)?;

        let previous_operation = self.0.invalidate();
        let previous = self.0.wallet_subscription.borrow_mut().take();
        drop(previous);
        self.0.state.borrow_mut().connection = Connection::Connecting(wallet.clone());
        let mut guard = self.0.begin()?;
        guard.connecting = true;
        cancel(previous_operation);
        let weak = Rc::downgrade(&self.0);
        let subscription = wallet.listen(move || {
            if let Some(inner) = weak.upgrade() {
                inner.wallet_changed();
            }
        })?;

        if !self.0.current(&guard.token) {
            return Err(WalletFailure::disconnected());
        }

        *self.0.wallet_subscription.borrow_mut() = Some(subscription);
        Ok(Box::pin(async move {
            let result =
                super::operation::Current::new(guard.token.clone(), wallet.connect(silent)).await;
            let result = result.and_then(|()| guard.connected(wallet));
            guard.finish(result)
        }))
    }

    /// Selects an exact currently authorized account, invalidating existing signers.
    pub fn select_account(&self, account: WalletAccount) -> Result<(), WalletFailure> {
        self.active()?;
        let connection = self.state().connection;
        let Connection::Connected { wallet, .. } = connection else {
            return Err(WalletFailure::disconnected());
        };

        if !authorized(&wallet, &self.0.chain).contains(&account) {
            return Err(WalletFailure::disconnected());
        }

        let previous = self.0.invalidate();
        self.0.state.borrow_mut().connection = Connection::Connected { wallet, account };
        self.0.state.borrow_mut().operating = false;
        cancel(previous);
        self.0.notify();
        Ok(())
    }

    /// Removes local authorization before awaiting the wallet's disconnect promise.
    pub fn disconnect(&self) -> Result<WalletFuture<()>, WalletFailure> {
        self.active()?;
        let wallet = self.state().connection.wallet().cloned();
        self.0.disconnected(None);
        let mut guard = self.0.begin()?;
        Ok(Box::pin(async move {
            let operation = disconnect_wallet(wallet);
            let result = super::operation::Current::new(guard.token.clone(), operation).await;
            guard.finish(result)
        }))
    }

    /// Creates a typed Kit signer bound to this account and authorization identity.
    pub fn signer(&self) -> Result<WalletSigner, WalletFailure> {
        self.active()?;
        WalletSigner::new(&self.0)
    }

    /// Cancels local authorization and wakes pending callers without another wallet prompt.
    /// A wallet promise or submitted transaction may already have remote effects.
    pub fn cancel(&self) -> Result<(), WalletFailure> {
        self.active()?;
        self.0.disconnected(None);
        Ok(())
    }

    /// Clears the displayed operation error. Discovery cleanup errors use their own journal.
    pub fn clear_error(&self) -> Result<(), WalletFailure> {
        self.active()?;
        self.0.state.borrow_mut().error = None;
        self.0.notify();
        Ok(())
    }

    /// Cancels caller interest and releases all controller subscriptions idempotently.
    pub fn dispose(&self) {
        if matches!(self.state().connection, Connection::Disposed) {
            return;
        }

        let previous = self.0.invalidate();
        self.0.state.borrow_mut().connection = Connection::Disposed;
        self.0.state.borrow_mut().operating = false;
        let wallet = self.0.wallet_subscription.borrow_mut().take();
        let registry = self.0.registry_subscription.borrow_mut().take();
        drop((wallet, registry));
        cancel(previous);
        self.0.notify();

        let listeners = std::mem::take(&mut *self.0.listeners.borrow_mut());
        drop(listeners);
    }

    fn active(&self) -> Result<(), WalletFailure> {
        if matches!(self.state().connection, Connection::Disposed) {
            return Err(WalletFailure::disconnected());
        }

        Ok(())
    }
}

impl Drop for WalletController {
    fn drop(&mut self) {
        self.dispose();
    }
}

pub(super) fn authorized(wallet: &BrowserWallet, chain: &str) -> Vec<WalletAccount> {
    wallet
        .accounts()
        .into_iter()
        .filter(|account| account.chains().iter().any(|entry| entry == chain))
        .collect()
}

pub(super) struct OperationGuard {
    inner: Weak<Inner>,
    pub token: Rc<Token>,
    connecting: bool,
    finished: bool,
}

impl OperationGuard {
    fn connected(&self, wallet: Rc<BrowserWallet>) -> Result<(), WalletFailure> {
        let inner = self
            .inner
            .upgrade()
            .ok_or_else(WalletFailure::disconnected)?;

        if !inner.current(&self.token) || !wallet.chains().contains(&inner.chain) {
            return Err(WalletFailure::disconnected());
        }

        let account = authorized(&wallet, &inner.chain)
            .into_iter()
            .next()
            .ok_or_else(WalletFailure::invalid)?;
        *inner.authorization.borrow_mut() = Rc::new(());
        inner.state.borrow_mut().connection = Connection::Connected { wallet, account };
        Ok(())
    }

    // Finishing can synchronously notify code that replaces authorization; recheck before returning it.
    pub fn finish<T>(&mut self, result: Result<T, WalletFailure>) -> Result<T, WalletFailure> {
        let inner = self
            .inner
            .upgrade()
            .ok_or_else(WalletFailure::disconnected)?;

        if !inner.current(&self.token) {
            return Err(WalletFailure::disconnected());
        }

        if let Err(error) = &result {
            inner.state.borrow_mut().error = Some(error.clone());

            if self.connecting {
                inner.state.borrow_mut().connection = Connection::Disconnected;
            }
        }

        inner.operation.borrow_mut().take();
        inner.state.borrow_mut().operating = false;
        self.finished = true;
        let authorization = inner.authorization.borrow().clone();
        inner.notify();

        if result.is_ok() && !Rc::ptr_eq(&authorization, &inner.authorization.borrow()) {
            return Err(WalletFailure::disconnected());
        }

        result
    }
}

impl Drop for OperationGuard {
    fn drop(&mut self) {
        let Some(inner) = self
            .inner
            .upgrade()
            .filter(|inner| inner.current(&self.token))
        else {
            return;
        };

        if !self.finished {
            inner.operation.borrow_mut().take();
            let mut state = inner.state.borrow_mut();
            state.operating = false;

            if self.connecting {
                state.connection = Connection::Disconnected;
            }

            drop(state);
            self.token.cancel();
            inner.notify();
        }
    }
}

/// State-listener owner. Drop removes only this callback.
pub struct ControllerSubscription {
    inner: Weak<Inner>,
    id: u64,
}

impl Drop for ControllerSubscription {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.upgrade() {
            let removed = inner.listeners.borrow_mut().remove(&self.id);
            drop(removed);
        }
    }
}

fn cancel(token: Option<Rc<Token>>) {
    if let Some(token) = token {
        token.cancel();
    }
}

async fn disconnect_wallet(wallet: Option<Rc<BrowserWallet>>) -> Result<(), WalletFailure> {
    if let Some(wallet) = wallet {
        wallet.disconnect().await?;
    }

    Ok(())
}
