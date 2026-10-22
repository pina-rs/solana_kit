use solana_kit_wallet::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub struct HookWallet {
    pub name: Cell<&'static str>,
    hook: RefCell<Option<Rc<dyn Fn()>>>,
    icon: WalletIcon,
    features: WalletFeatures,
}

impl HookWallet {
    pub fn new(name: &'static str) -> Rc<Self> {
        Rc::new(Self {
            name: Cell::new(name),
            hook: RefCell::new(None),
            icon: WalletIcon::parse("data:image/png;base64,").unwrap(),
            features: WalletFeatures::new(),
        })
    }

    pub fn hook(&self, hook: impl Fn() + 'static) {
        *self.hook.borrow_mut() = Some(Rc::new(hook));
    }

    pub fn clear(&self) {
        self.hook.borrow_mut().take();
    }
}

impl Wallet for HookWallet {
    fn name(&self) -> &str {
        let hook = self.hook.borrow().clone();

        if let Some(hook) = hook {
            hook();
        }

        self.name.get()
    }
    fn version(&self) -> &str {
        WALLET_STANDARD_VERSION
    }
    fn icon(&self) -> &WalletIcon {
        &self.icon
    }
    fn chains(&self) -> &[String] {
        &[]
    }
    fn features(&self) -> &WalletFeatures {
        &self.features
    }
    fn accounts(&self) -> &[WalletAccount] {
        &[]
    }
}

pub fn events(
    registry: &WalletRegistry,
) -> (RegistrySubscription, Rc<RefCell<Vec<RegistryEventKind>>>) {
    let events = Rc::new(RefCell::new(Vec::new()));
    let output = events.clone();
    let listener = registry
        .on_event(move |event| output.borrow_mut().push(event.kind))
        .unwrap();
    (listener, events)
}
