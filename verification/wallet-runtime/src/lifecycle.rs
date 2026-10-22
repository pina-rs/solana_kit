use crate::common::*;
use solana_kit_wallet::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub fn run() {
    snapshots();
    destruction();
    identity_and_names();
    irrelevant_notifications();
}

fn snapshots() {
    use RegistryEventKind::{Changed, Registered};
    let registry = WalletRegistry::new();
    let log = Rc::new(RefCell::new(Vec::new()));
    let second = Rc::new(RefCell::new(None::<RegistrySubscription>));
    let third = Rc::new(RefCell::new(None::<RegistrySubscription>));
    let copy = registry.clone();
    let output = log.clone();
    let removed = second.clone();
    let added = third.clone();
    let first = registry
        .on_event(move |event| {
            output.borrow_mut().push((1, event.kind));

            if event.kind == Registered {
                let subscription = removed.borrow_mut().take();
                drop(subscription);
                let output = output.clone();
                *added.borrow_mut() = Some(
                    copy.on_event(move |event| output.borrow_mut().push((3, event.kind)))
                        .unwrap(),
                );
                copy.changed(&event.wallet);
            }
        })
        .unwrap();
    let output = log.clone();
    *second.borrow_mut() = Some(
        registry
            .on_event(move |event| output.borrow_mut().push((2, event.kind)))
            .unwrap(),
    );
    let guard = registry.register(Probe::new("event")).unwrap();
    assert_eq!(
        *log.borrow(),
        [(1, Registered), (1, Changed), (3, Changed), (2, Registered)]
    );
    drop(first);
    third.borrow_mut().take();
    drop(guard);
}

fn destruction() {
    let registry = WalletRegistry::new();
    let existing = Probe::new("old");
    let old = registry.register(existing.clone()).unwrap();
    let nested = Rc::new(RefCell::new(None));
    let retained = nested.clone();
    let copy = registry.clone();
    let drops = Rc::new(Cell::new(0));
    let count = drops.clone();
    *existing.on_drop.borrow_mut() = Some(Rc::new(move || {
        count.set(count.get() + 1);
        assert!(copy.wallets().is_empty());
        *retained.borrow_mut() = Some(copy.register(Probe::new("collision")).unwrap());
    }));
    let weak = Rc::downgrade(&existing);
    let copy = registry.clone();
    *existing.hook.borrow_mut() = Some(Rc::new(move || {
        let identity: Rc<dyn Wallet> = weak.upgrade().unwrap();
        copy.unregister(&identity);
    }));
    drop(existing);
    let outer = registry.register(Probe::new("collision")).unwrap();
    assert_eq!(drops.get(), 1);
    assert_eq!(registry.wallets().len(), 1);
    drop(old);
    drop(outer);
    assert_eq!(registry.wallets().len(), 1);
    nested.borrow_mut().take();
    assert!(registry.wallets().is_empty());
}

fn identity_and_names() {
    let registry = WalletRegistry::new();
    let first = Probe::new("old");
    let identity: Rc<dyn Wallet> = first.clone();
    let mut stale = registry.register(identity.clone()).unwrap();
    first.name.set("current");
    registry.changed(&identity);
    let old_name = registry.register(Probe::new("old")).unwrap();
    let equal_name = registry.register(Probe::new("current")).unwrap();
    drop(equal_name);
    assert_eq!(registry.wallets().len(), 2);
    registry.unregister(&identity);
    let fresh = registry.register(identity.clone()).unwrap();
    stale.unregister();
    drop(stale);
    assert!(contains(&registry, &identity));
    drop(old_name);
    drop(fresh);
    assert!(registry.wallets().is_empty());
}

fn irrelevant_notifications() {
    let registry = WalletRegistry::new();
    let outside: Rc<dyn Wallet> = Probe::new("outside");
    let incoming = Probe::new("incoming");
    let copy = registry.clone();
    *incoming.hook.borrow_mut() = Some(Rc::new(move || {
        copy.changed(&outside);
        copy.unregister(&outside);
    }));
    let guard = registry.register(incoming.clone()).unwrap();
    assert_eq!(incoming.reads.get(), 1);
    incoming.clear();
    drop(guard);
}
