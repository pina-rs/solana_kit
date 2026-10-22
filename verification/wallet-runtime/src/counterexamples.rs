use crate::common::*;
use solana_kit_wallet::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub fn insertion() {
    let registry = WalletRegistry::new();
    let (_subscription, log) = events(&registry);
    let existing = Probe::new("incumbent");
    let first = registry.register(existing.clone()).unwrap();
    let nested = Rc::new(RefCell::new(None));
    let owned = nested.clone();
    let copy = registry.clone();
    let armed = Cell::new(true);
    *existing.hook.borrow_mut() = Some(Rc::new(move || {
        if armed.replace(false) {
            *owned.borrow_mut() = Some(copy.register(Probe::new("duplicate")).unwrap());
        }
    }));
    let outer = registry.register(Probe::new("duplicate")).unwrap();
    existing.clear();
    let names: Vec<_> = registry
        .wallets()
        .iter()
        .map(|wallet| wallet.name().to_owned())
        .collect();
    assert_eq!(names, ["incumbent", "duplicate"]);
    assert_eq!(*log.borrow(), [RegistryEventKind::Registered; 2]);
    drop(outer);
    assert_eq!(registry.wallets().len(), 2, "duplicate guard is inert");
    nested.borrow_mut().take();
    drop(first);
    assert_eq!(log.borrow().len(), 4);
}

pub fn removal() {
    let registry = WalletRegistry::new();
    let (_subscription, log) = events(&registry);
    let existing = Probe::new("duplicate");
    let identity: Rc<dyn Wallet> = existing.clone();
    let stale = registry.register(identity.clone()).unwrap();
    let copy = registry.clone();
    *existing.hook.borrow_mut() = Some(Rc::new(move || copy.unregister(&identity)));
    let incoming: Rc<dyn Wallet> = Probe::new("duplicate");
    let guard = registry.register(incoming.clone()).unwrap();
    existing.clear();
    assert!(
        contains(&registry, &incoming),
        "incoming must survive removal of stale name match"
    );
    assert_eq!(registry.wallets().len(), 1);
    assert_eq!(
        *log.borrow(),
        [
            RegistryEventKind::Registered,
            RegistryEventKind::Unregistered,
            RegistryEventKind::Registered
        ]
    );
    drop(stale);
    assert!(contains(&registry, &incoming));
    drop(guard);
    assert!(registry.wallets().is_empty());
}

pub fn identity() {
    let registry = WalletRegistry::new();
    let (_subscription, log) = events(&registry);
    let same = Probe::new("alpha");
    let first = registry.register(same.clone()).unwrap();
    let weak = Rc::downgrade(&same);
    let alternate = Cell::new(false);
    *same.hook.borrow_mut() = Some(Rc::new(move || {
        weak.upgrade()
            .unwrap()
            .name
            .set(if alternate.replace(true) {
                "beta"
            } else {
                "alpha"
            });
    }));
    let duplicate = registry.register(same.clone()).unwrap();
    same.clear();
    assert_eq!(registry.wallets().len(), 1);
    assert_eq!(*log.borrow(), [RegistryEventKind::Registered]);
    drop(duplicate);
    assert_eq!(registry.wallets().len(), 1);
    drop(first);
    assert!(registry.wallets().is_empty());
}
