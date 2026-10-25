use super::{BrowserWallet, WalletFailure, interop::*, session::WalletSubscription};
use js_sys::{Array, Function, Object};
use solana_kit_wallet::{Registration, Wallet, WalletRegistry};
use std::{
    cell::{Cell, RefCell},
    rc::{Rc, Weak},
};
use wasm_bindgen::{JsCast, JsValue, closure::Closure, prelude::wasm_bindgen};
use web_sys::{CustomEvent, CustomEventInit, Event, Window};

const REGISTER: &str = "wallet-standard:register-wallet";
const READY: &str = "wallet-standard:app-ready";

#[wasm_bindgen(module = "/src/wallet/registration.js")]
extern "C" {
    fn registration(callback: &Function) -> Function;
}

struct Entry {
    raw: JsValue,
    wallet: Rc<BrowserWallet>,
    registration: Registration,
    _changes: WalletSubscription,
}

impl Entry {
    fn close(mut self) -> Result<(), WalletFailure> {
        self.registration.unregister();
        self.wallet.close()
    }
}

struct Inner {
    registry: WalletRegistry,
    entries: RefCell<Vec<Entry>>,
    failures: Rc<RefCell<Vec<WalletFailure>>>,
    closed: Cell<bool>,
}

impl Inner {
    fn record(&self, failure: WalletFailure) -> JsValue {
        let exception = failure.exception();
        self.failures.borrow_mut().push(failure);
        exception
    }

    fn register(self: &Rc<Self>, values: Array) -> Result<JsValue, WalletFailure> {
        if self.closed.get() {
            return Err(WalletFailure::disconnected());
        }

        let accepted = self.validate(values)?;
        let mut registered = Vec::new();

        for (raw, wallet) in accepted {
            match self.register_one(raw, wallet.clone()) {
                Ok(true) => registered.push(wallet),
                Ok(false) => {}
                Err(error) => {
                    self.unregister(&registered);

                    return Err(error);
                }
            }
        }

        Ok(self.unregister_callback(registered))
    }

    fn validate(&self, values: Array) -> Result<Vec<(JsValue, Rc<BrowserWallet>)>, WalletFailure> {
        let mut accepted: Vec<(JsValue, Rc<BrowserWallet>)> = Vec::new();

        // Complete validation precedes registration so bad batches do not leave partial membership.
        for raw in values.iter() {
            if self.contains(&raw) || accepted.iter().any(|(other, _)| Object::is(other, &raw)) {
                continue;
            }

            let wallet = Rc::new(BrowserWallet::read(&raw, self.failures.clone())?);
            accepted.push((raw, wallet));
        }

        Ok(accepted)
    }

    fn contains(&self, raw: &JsValue) -> bool {
        self.entries
            .borrow()
            .iter()
            .any(|entry| Object::is(&entry.raw, raw))
    }

    fn register_one(
        self: &Rc<Self>,
        raw: JsValue,
        wallet: Rc<BrowserWallet>,
    ) -> Result<bool, WalletFailure> {
        if self.closed.get() {
            return Err(WalletFailure::disconnected());
        }
        if self.contains(&raw) {
            return Ok(false);
        }

        wallet.reconcile_events()?;
        let changes = self.watch(&wallet)?;
        let erased: Rc<dyn Wallet> = wallet.clone();
        let guard = self
            .registry
            .register(erased.clone())
            .map_err(|error| WalletFailure::caused("wallet registration failed", error))?;
        let member = self
            .registry
            .wallets()
            .iter()
            .any(|entry| Rc::ptr_eq(entry, &erased));

        if self.closed.get() || !member {
            drop(guard);
            wallet.close()?;

            return Ok(false);
        }

        self.entries.borrow_mut().push(Entry {
            raw,
            wallet,
            registration: guard,
            _changes: changes,
        });
        Ok(true)
    }

    fn watch(
        self: &Rc<Self>,
        wallet: &Rc<BrowserWallet>,
    ) -> Result<WalletSubscription, WalletFailure> {
        let weak = Rc::downgrade(self);
        let changed = Rc::downgrade(wallet);
        wallet.listen(move || {
            let (Some(inner), Some(wallet)) = (weak.upgrade(), changed.upgrade()) else {
                return;
            };
            let erased: Rc<dyn Wallet> = wallet;
            inner.registry.changed(&erased);
        })
    }

    fn unregister_callback(self: &Rc<Self>, registered: Vec<Rc<BrowserWallet>>) -> JsValue {
        let weak = Rc::downgrade(self);
        Closure::<dyn FnMut()>::new(move || {
            if let Some(inner) = weak.upgrade() {
                inner.unregister(&registered);
            }
        })
        .into_js_value()
    }

    fn unregister(&self, wallets: &[Rc<BrowserWallet>]) {
        let all = std::mem::take(&mut *self.entries.borrow_mut());
        let (removed, kept): (Vec<_>, Vec<_>) = all
            .into_iter()
            .partition(|entry| wallets.iter().any(|wallet| wallet.same(&entry.wallet)));
        *self.entries.borrow_mut() = kept;

        for entry in removed {
            if let Err(error) = entry.close() {
                self.failures.borrow_mut().push(error);
            }
        }
    }
}

/// Window-scoped discovery owner. No singleton or background task is installed.
///
/// `close` records cleanup errors; Drop performs the same best-effort cleanup.
/// Wallets retain the shared error journal so errors remain available through
/// `take_failures` while discovery is alive. Wallet promises cannot be aborted by
/// Wallet Standard; controller cancellation invalidates their caller interest.
pub struct WalletDiscovery {
    inner: Rc<Inner>,
    window: Window,
    listener: Option<Function>,
    _api: Object,
}

impl WalletDiscovery {
    /// Listens for future wallets and announces the standard app-ready handshake.
    pub fn start(registry: WalletRegistry) -> Result<Self, WalletFailure> {
        let window = web_sys::window().ok_or_else(|| {
            WalletFailure::new(
                solana_kit_wallet::WalletErrorCode::Unavailable,
                "wallet discovery requires a Window",
            )
        })?;
        let inner = Rc::new(Inner {
            registry,
            entries: RefCell::new(Vec::new()),
            failures: Rc::new(RefCell::new(Vec::new())),
            closed: Cell::new(false),
        });
        let api = Self::api(&inner)?;
        let listener = Self::listener(Rc::downgrade(&inner), api.clone())?;
        window
            .add_event_listener_with_callback(REGISTER, &listener)
            .map_err(WalletFailure::js)?;
        let mut discovery = Self {
            inner,
            window,
            listener: Some(listener),
            _api: api.clone(),
        };
        let init = CustomEventInit::new();
        init.set_detail(&api);
        let event =
            CustomEvent::new_with_event_init_dict(READY, &init).map_err(WalletFailure::js)?;

        if let Err(error) = discovery.window.dispatch_event(&event) {
            let failure = WalletFailure::js(error);
            discovery.close();

            return Err(failure);
        }

        Ok(discovery)
    }

    /// The caller-owned portable metadata registry.
    pub fn registry(&self) -> &WalletRegistry {
        &self.inner.registry
    }

    /// Current browser wallet identities, in registration order.
    pub fn wallets(&self) -> Vec<Rc<BrowserWallet>> {
        self.inner
            .entries
            .borrow()
            .iter()
            .map(|entry| entry.wallet.clone())
            .collect()
    }

    /// Takes original registration, change-event and cleanup failures.
    pub fn take_failures(&self) -> Vec<WalletFailure> {
        std::mem::take(&mut *self.inner.failures.borrow_mut())
    }

    /// Idempotently removes this owner's listener and wallet registrations.
    /// Errors are retained in `take_failures`, including unsubscribe exceptions.
    pub fn close(&mut self) {
        if self.inner.closed.replace(true) {
            return;
        }

        if let Some(listener) = self.listener.take() {
            if let Err(error) = self
                .window
                .remove_event_listener_with_callback(REGISTER, &listener)
            {
                self.inner
                    .failures
                    .borrow_mut()
                    .push(WalletFailure::js(error));
            }
        }

        let removed = std::mem::take(&mut *self.inner.entries.borrow_mut());

        for entry in removed {
            if let Err(error) = entry.close() {
                self.inner.failures.borrow_mut().push(error);
            }
        }
    }

    fn api(inner: &Rc<Inner>) -> Result<Object, WalletFailure> {
        let weak = Rc::downgrade(inner);
        let callback =
            Closure::<dyn FnMut(Array) -> Result<JsValue, JsValue>>::new(move |values| {
                let inner = weak
                    .upgrade()
                    .ok_or_else(|| WalletFailure::disconnected().exception())?;
                inner
                    .register(values)
                    .map_err(|failure| inner.record(failure))
            })
            .into_js_value();
        let api = Object::new();
        set(&api, "register", &registration(&function(callback)?))?;
        Ok(api)
    }

    fn listener(weak: Weak<Inner>, api: Object) -> Result<Function, WalletFailure> {
        let callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
            let Some(inner) = weak.upgrade().filter(|inner| !inner.closed.get()) else {
                return;
            };
            let Ok(event) = event.dyn_into::<CustomEvent>() else {
                return;
            };
            let result = function(event.detail()).and_then(|callback| {
                callback
                    .call1(&JsValue::UNDEFINED, &api)
                    .map_err(WalletFailure::js)
            });

            if let Err(error) = result {
                inner.failures.borrow_mut().push(error);
            }
        })
        .into_js_value();
        // Closure guarantees Function; this checked conversion cannot inspect wallet data.
        function(callback)
    }
}

impl Drop for WalletDiscovery {
    fn drop(&mut self) {
        self.close();
    }
}
