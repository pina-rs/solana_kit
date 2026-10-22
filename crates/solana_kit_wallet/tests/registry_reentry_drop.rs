mod reentry;
use reentry::*;
use solana_kit_wallet::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

struct DropWallet {
    inner: Rc<HookWallet>,
    action: Box<dyn Fn()>,
}

impl Drop for DropWallet {
    fn drop(&mut self) {
        (self.action)();
    }
}

impl Wallet for DropWallet {
    fn name(&self) -> &str {
        self.inner.name()
    }
    fn version(&self) -> &str {
        self.inner.version()
    }
    fn icon(&self) -> &WalletIcon {
        self.inner.icon()
    }
    fn chains(&self) -> &[String] {
        self.inner.chains()
    }
    fn features(&self) -> &WalletFeatures {
        self.inner.features()
    }
    fn accounts(&self) -> &[WalletAccount] {
        self.inner.accounts()
    }
}

#[test]
fn invalidated_snapshot_drops_wallet_outside_borrow_before_retry() {
    let registry = WalletRegistry::new();
    let (_listener, events) = events(&registry);
    let inner = HookWallet::new("existing");
    let nested = Rc::new(RefCell::new(None::<Registration>));
    let retained = nested.clone();
    let copy = registry.clone();
    let drops = Rc::new(Cell::new(0));
    let count = drops.clone();
    let wallet = Rc::new(DropWallet {
        inner: inner.clone(),
        action: Box::new(move || {
            count.set(count.get() + 1);
            assert!(copy.wallets().is_empty());
            *retained.borrow_mut() = Some(copy.register(HookWallet::new("collision")).unwrap());
        }),
    });
    let old = registry.register(wallet.clone()).unwrap();
    let weak = Rc::downgrade(&wallet);
    let copy = registry.clone();

    inner.hook(move || {
        let identity: Rc<dyn Wallet> = weak.upgrade().unwrap();
        copy.unregister(&identity);
    });
    drop(wallet);
    let outer = registry.register(HookWallet::new("collision")).unwrap();
    assert_eq!(drops.get(), 1);
    assert_eq!(registry.wallets().len(), 1);
    assert_eq!(
        *events.borrow(),
        [
            RegistryEventKind::Registered,
            RegistryEventKind::Unregistered,
            RegistryEventKind::Registered
        ]
    );
    inner.clear();
    drop(old);
    drop(outer);
    assert_eq!(registry.wallets().len(), 1);
    nested.borrow_mut().take();
    assert_eq!(events.borrow().len(), 4);
}

#[test]
fn disposal_drops_owned_wallet_captures_outside_borrow() {
    let registry = WalletRegistry::new();
    let inner = HookWallet::new("existing");
    let copy = registry.clone();
    let drops = Rc::new(Cell::new(0));
    let count = drops.clone();
    let wallet = Rc::new(DropWallet {
        inner,
        action: Box::new(move || {
            count.set(count.get() + 1);
            assert!(copy.wallets().is_empty());
            assert!(matches!(
                copy.on_event(|_| {}),
                Err(RegistryError::Disposed)
            ));
            copy.dispose();
        }),
    });
    let guard = registry.register(wallet).unwrap();

    registry.dispose();
    assert_eq!(drops.get(), 1);
    drop(guard);
    assert_eq!(drops.get(), 1);
}
