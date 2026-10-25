use solana_kit_wallet::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub type Hook = Rc<dyn Fn()>;
pub type EventLog = Rc<RefCell<Vec<RegistryEventKind>>>;

pub struct Probe {
    pub name: Cell<&'static str>,
    pub hook: RefCell<Option<Hook>>,
    pub reads: Cell<usize>,
    pub on_drop: RefCell<Option<Hook>>,
    icon: WalletIcon,
    features: WalletFeatures,
}

impl Probe {
    /// Gives every scenario an independently mutable public Wallet backend.
    pub fn new(name: &'static str) -> Rc<Self> {
        Rc::new(Self {
            name: Cell::new(name),
            hook: RefCell::new(None),
            reads: Cell::new(0),
            on_drop: RefCell::new(None),
            icon: WalletIcon::parse("data:image/png;base64,").unwrap(),
            features: WalletFeatures::new(),
        })
    }

    pub fn clear(&self) {
        self.hook.borrow_mut().take();
    }
}

impl Drop for Probe {
    fn drop(&mut self) {
        if let Some(action) = self.on_drop.get_mut().take() {
            action();
        }
    }
}

impl Wallet for Probe {
    fn name(&self) -> &str {
        self.reads.set(self.reads.get() + 1);
        let hook = self.hook.borrow().clone();

        if let Some(hook) = hook {
            hook();
        }

        self.name.get()
    }
    fn version(&self) -> &str {
        "1.0.0"
    }
    fn icon(&self) -> &WalletIcon {
        &self.icon
    }
    fn chains(&self) -> Vec<String> {
        Vec::new()
    }
    fn features(&self) -> WalletFeatures {
        self.features.clone()
    }
    fn accounts(&self) -> Vec<WalletAccount> {
        Vec::new()
    }
}

pub fn events(registry: &WalletRegistry) -> (RegistrySubscription, EventLog) {
    let events = Rc::new(RefCell::new(Vec::new()));
    let output = events.clone();
    let subscription = registry
        .on_event(move |event| output.borrow_mut().push(event.kind))
        .unwrap();
    (subscription, events)
}

pub fn contains(registry: &WalletRegistry, wallet: &Rc<dyn Wallet>) -> bool {
    registry
        .wallets()
        .iter()
        .any(|current| Rc::ptr_eq(current, wallet))
}
