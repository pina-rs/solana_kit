use super::{
    WalletFailure,
    interop::*,
    metadata::{self, Snapshot},
};
use js_sys::{Array, Function, Object};
use solana_kit_wallet::{
    Wallet, WalletAccount, WalletErrorCode, WalletFeature, WalletFeatures, WalletIcon,
};
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::{Rc, Weak},
};
use wasm_bindgen::{JsValue, closure::Closure};

const MAX_EVENT_REPLACEMENTS: usize = 8;
type Listener = Rc<dyn Fn()>;

/// One concrete advertised JavaScript feature and its exact contract version.
#[derive(Clone)]
pub struct BrowserFeature {
    pub(super) raw: JsValue,
    pub(super) version: String,
}

impl WalletFeature for BrowserFeature {
    fn version(&self) -> &str {
        &self.version
    }
}

struct EventsOwner {
    off: Function,
    _callback: JsValue,
}

impl EventsOwner {
    fn close(self) -> Result<(), WalletFailure> {
        self.off
            .call0(&JsValue::UNDEFINED)
            .map_err(WalletFailure::js)?;
        Ok(())
    }
}

struct Inner {
    version: String,
    name: String,
    icon: WalletIcon,
    snapshot: RefCell<Snapshot>,
    revision: RefCell<Rc<()>>,
    listeners: RefCell<BTreeMap<u64, Listener>>,
    next_listener: Cell<u64>,
    events: RefCell<Option<EventsOwner>>,
    installing: Cell<bool>,
    pending_events: Cell<bool>,
    active: Cell<bool>,
    failures: Rc<RefCell<Vec<WalletFailure>>>,
}

/// A live Wallet Standard wallet with owned immutable metadata snapshots.
#[derive(Clone)]
pub struct BrowserWallet(Rc<Inner>);

impl BrowserWallet {
    pub(super) fn read(
        raw: &JsValue,
        failures: Rc<RefCell<Vec<WalletFailure>>>,
    ) -> Result<Self, WalletFailure> {
        object(raw.clone())?;
        let version = string(raw, "version")?;

        if version != "1.0.0" {
            return Err(WalletFailure::new(
                WalletErrorCode::UnsupportedFeature,
                "unsupported Wallet Standard version",
            ));
        }

        let icon = WalletIcon::parse(string(raw, "icon")?)
            .map_err(|error| WalletFailure::caused("invalid wallet icon", error))?;
        let wallet = Self(Rc::new(Inner {
            version,
            name: string(raw, "name")?,
            icon,
            snapshot: RefCell::new(Snapshot::read(raw)?),
            revision: RefCell::new(Rc::new(())),
            listeners: RefCell::new(BTreeMap::new()),
            next_listener: Cell::new(0),
            events: RefCell::new(None),
            installing: Cell::new(false),
            pending_events: Cell::new(false),
            active: Cell::new(true),
            failures,
        }));
        Ok(wallet)
    }

    /// Whether this registration is still live.
    pub fn is_available(&self) -> bool {
        self.0.active.get()
    }

    pub(super) fn same(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    pub(super) fn revision(&self) -> Rc<()> {
        self.0.revision.borrow().clone()
    }

    pub(super) fn feature(&self, identifier: &str) -> Result<BrowserFeature, WalletFailure> {
        if !self.is_available() {
            return Err(WalletFailure::disconnected());
        }

        let feature = self
            .features()
            .feature::<BrowserFeature>(identifier)
            .cloned()
            .ok_or_else(|| {
                WalletFailure::new(
                    WalletErrorCode::UnsupportedFeature,
                    "wallet capability is unavailable",
                )
            })?;

        if feature.version != "1.0.0" {
            return Err(WalletFailure::new(
                WalletErrorCode::UnsupportedFeature,
                "wallet capability version is unsupported",
            ));
        }

        Ok(feature)
    }

    pub(super) fn raw_account(&self, account: &WalletAccount) -> Result<JsValue, WalletFailure> {
        self.0
            .snapshot
            .borrow()
            .accounts
            .iter()
            .find(|entry| &entry.metadata == account)
            .map(|entry| entry.raw.clone())
            .ok_or_else(WalletFailure::disconnected)
    }

    pub(super) async fn connect(&self, silent: bool) -> Result<(), WalletFailure> {
        let feature = self.feature(solana_kit_wallet::CONNECT)?;
        let revision = self.revision();
        let input = Object::new();
        set(&input, "silent", &JsValue::from_bool(silent))?;
        let args = Array::of1(&input);
        let output = object(promise(&feature.raw, "connect", &args).await?)?;
        let accounts = metadata::accounts(get(&output, "accounts")?)?;

        if !self.is_available() {
            return Err(WalletFailure::disconnected());
        }

        if !Rc::ptr_eq(&revision, &self.revision()) {
            let current: Vec<_> = self.accounts();
            let returned: Vec<_> = accounts
                .iter()
                .map(|entry| entry.metadata.clone())
                .collect();

            if current != returned {
                return Err(WalletFailure::disconnected());
            }

            return Ok(());
        }

        self.0.snapshot.borrow_mut().accounts = accounts;
        self.changed();
        Ok(())
    }

    pub(super) async fn disconnect(&self) -> Result<(), WalletFailure> {
        let feature = self.feature(solana_kit_wallet::DISCONNECT)?;
        promise(&feature.raw, "disconnect", &Array::new()).await?;
        Ok(())
    }

    pub(super) fn listen(
        &self,
        listener: impl Fn() + 'static,
    ) -> Result<WalletSubscription, WalletFailure> {
        let id = self
            .0
            .next_listener
            .get()
            .checked_add(1)
            .ok_or_else(WalletFailure::invalid)?;
        self.0.next_listener.set(id);
        self.0.listeners.borrow_mut().insert(id, Rc::new(listener));
        Ok(WalletSubscription {
            inner: Rc::downgrade(&self.0),
            id,
        })
    }

    pub(super) fn close(&self) -> Result<(), WalletFailure> {
        if self.0.active.replace(false) {
            self.changed();
        }
        let owner = self.0.events.borrow_mut().take();

        if let Some(owner) = owner {
            owner.close()?;
        }

        Ok(())
    }

    fn changed(&self) {
        *self.0.revision.borrow_mut() = Rc::new(());
        let listeners: Vec<_> = self.0.listeners.borrow().values().cloned().collect();

        for listener in listeners {
            listener();
        }
    }

    fn handle_change(&self, value: JsValue) -> Result<(), WalletFailure> {
        let value = object(value)?;
        let revision = self.revision();
        let previous = self.0.snapshot.borrow().clone();
        let replace_events = has(&value, "features")?;
        let next = previous.changed(&value)?;

        if !Rc::ptr_eq(&revision, &self.revision()) {
            return Err(WalletFailure::disconnected());
        }

        *self.0.snapshot.borrow_mut() = next;
        self.changed();

        if replace_events {
            self.reconcile_events()?;
        }

        Ok(())
    }

    pub(super) fn reconcile_events(&self) -> Result<(), WalletFailure> {
        self.0.pending_events.set(true);

        if self.0.installing.replace(true) {
            return Ok(());
        }

        let result = self.replace_events();
        self.0.installing.set(false);
        result
    }

    // A synchronous on() callback may replace features; bounded reconciliation avoids stale owners.
    fn replace_events(&self) -> Result<(), WalletFailure> {
        for _ in 0..MAX_EVENT_REPLACEMENTS {
            if !self.0.pending_events.replace(false) {
                return Ok(());
            }

            let previous = self.0.events.borrow_mut().take();

            if let Some(previous) = previous {
                previous.close()?;
            }

            if !self.is_available() {
                return Ok(());
            }

            let advertised = self.features();
            let Some(feature) = advertised.feature::<BrowserFeature>(solana_kit_wallet::EVENTS)
            else {
                return Ok(());
            };
            let feature = feature.clone();
            self.install_events(&feature)?;
        }

        Err(WalletFailure::new(
            WalletErrorCode::InvalidResponse,
            "wallet repeatedly replaced event capabilities during subscription",
        ))
    }

    fn receive_change(&self, value: JsValue) {
        if !self.is_available() {
            return;
        }

        if let Err(error) = self.handle_change(value) {
            self.0.failures.borrow_mut().push(error);
            self.0.active.set(false);
            self.changed();
        }
    }

    fn install_events(&self, feature: &BrowserFeature) -> Result<(), WalletFailure> {
        if feature.version != "1.0.0" {
            return Err(WalletFailure::new(
                WalletErrorCode::UnsupportedFeature,
                "wallet events version is unsupported",
            ));
        }

        let weak = Rc::downgrade(&self.0);
        let callback = Closure::<dyn FnMut(JsValue)>::new(move |value| {
            let Some(inner) = weak.upgrade() else { return };
            let wallet = BrowserWallet(inner);
            wallet.receive_change(value);
        })
        .into_js_value();
        let off = function(call(
            &feature.raw,
            "on",
            &Array::of2(&"change".into(), &callback),
        )?)?;
        let owner = EventsOwner {
            off,
            _callback: callback,
        };

        if !self.is_available() || self.0.pending_events.get() {
            owner.close()?;
        } else {
            *self.0.events.borrow_mut() = Some(owner);
        }

        Ok(())
    }
}

impl Wallet for BrowserWallet {
    fn version(&self) -> &str {
        &self.0.version
    }
    fn name(&self) -> &str {
        &self.0.name
    }
    fn icon(&self) -> &WalletIcon {
        &self.0.icon
    }
    fn chains(&self) -> Vec<String> {
        self.0.snapshot.borrow().chains.clone()
    }
    fn features(&self) -> WalletFeatures {
        self.0.snapshot.borrow().features.clone()
    }
    fn accounts(&self) -> Vec<WalletAccount> {
        self.0
            .snapshot
            .borrow()
            .accounts
            .iter()
            .map(|entry| entry.metadata.clone())
            .collect()
    }
}

pub(super) struct WalletSubscription {
    inner: Weak<Inner>,
    id: u64,
}

impl Drop for WalletSubscription {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.upgrade() {
            let removed = inner.listeners.borrow_mut().remove(&self.id);
            drop(removed);
        }
    }
}

impl Drop for BrowserWallet {
    fn drop(&mut self) {
        if Rc::strong_count(&self.0) == 1 {
            if let Err(error) = self.close() {
                self.0.failures.borrow_mut().push(error);
            }
        }
    }
}
