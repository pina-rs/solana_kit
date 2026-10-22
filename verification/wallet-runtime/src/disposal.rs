use crate::common::*;
use solana_kit_wallet::*;
use std::{cell::Cell, rc::Rc};

struct Capture(WalletRegistry, Rc<Cell<usize>>);

impl Drop for Capture {
    fn drop(&mut self) {
        assert!(self.0.wallets().is_empty());
        self.0.dispose();
        self.1.set(self.1.get() + 1);
    }
}

pub fn run() {
    disposed_destructors();
    identity_before_revision();
    unnotified_metadata();
}

fn disposed_destructors() {
    let registry = WalletRegistry::new();
    let drops = Rc::new(Cell::new(0));
    let wallet = Probe::new("owned");
    let copy = registry.clone();
    let count = drops.clone();
    *wallet.on_drop.borrow_mut() = Some(Rc::new(move || {
        assert!(copy.wallets().is_empty());
        copy.dispose();
        count.set(count.get() + 1);
    }));
    let guard = registry.register(wallet).unwrap();
    let capture = Capture(registry.clone(), drops.clone());
    let listener = registry
        .on_event(move |_| {
            let _ = &capture;
        })
        .unwrap();
    registry.dispose();
    assert_eq!(drops.get(), 2);
    drop(guard);
    drop(listener);

    rejected_arguments(&registry, &drops);
}

fn rejected_arguments(registry: &WalletRegistry, drops: &Rc<Cell<usize>>) {
    let wallet = Probe::new("rejected");
    let copy = registry.clone();
    let count = drops.clone();
    *wallet.on_drop.borrow_mut() = Some(Rc::new(move || {
        assert!(copy.wallets().is_empty());
        copy.dispose();
        count.set(count.get() + 1);
    }));
    assert!(matches!(
        registry.register(wallet),
        Err(RegistryError::Disposed)
    ));
    assert_eq!(drops.get(), 3);
    let capture = Capture(registry.clone(), drops.clone());
    assert!(matches!(
        registry.on_event(move |_| {
            let _ = &capture;
        }),
        Err(RegistryError::Disposed)
    ));
    assert_eq!(drops.get(), 4);
}

fn identity_before_revision() {
    let registry = WalletRegistry::new();
    let (_listener, log) = events(&registry);
    let same = Probe::new("same");
    let owner = registry.register(same.clone()).unwrap();
    let weak = Rc::downgrade(&same);
    let copy = registry.clone();
    *same.hook.borrow_mut() = Some(Rc::new(move || {
        let identity: Rc<dyn Wallet> = weak.upgrade().unwrap();
        copy.changed(&identity);
    }));
    let duplicate = registry.register(same.clone()).unwrap();
    same.clear();
    assert_eq!(registry.wallets().len(), 1);
    assert_eq!(log.borrow().len(), 3);
    drop(duplicate);
    assert_eq!(registry.wallets().len(), 1);
    drop(owner);
}

fn unnotified_metadata() {
    let registry = WalletRegistry::new();
    let earlier = Probe::new("earlier");
    let earlier_owner = registry.register(earlier.clone()).unwrap();
    let trigger = Probe::new("trigger");
    let trigger_owner = registry.register(trigger.clone()).unwrap();
    let renamed = earlier.clone();
    *trigger.hook.borrow_mut() = Some(Rc::new(move || renamed.name.set("incoming")));
    let outer = registry.register(Probe::new("incoming")).unwrap();
    trigger.clear();
    assert_eq!(registry.wallets().len(), 3);
    assert_eq!(
        registry
            .wallets()
            .iter()
            .filter(|wallet| wallet.name() == "incoming")
            .count(),
        2
    );
    drop(outer);
    drop(trigger_owner);
    drop(earlier_owner);
}
