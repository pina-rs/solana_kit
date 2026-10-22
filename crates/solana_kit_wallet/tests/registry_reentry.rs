mod reentry;
use reentry::*;
use solana_kit_wallet::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[test]
fn getter_inserting_a_collision_invalidates_outer_name_decision() {
    let registry = WalletRegistry::new();
    let (_listener, events) = events(&registry);
    let existing = HookWallet::new("existing");
    let original = registry.register(existing.clone()).unwrap();
    let nested = Rc::new(RefCell::new(None::<Registration>));
    let retained = nested.clone();
    let copy = registry.clone();
    let armed = Cell::new(true);
    existing.hook(move || {
        if armed.replace(false) {
            *retained.borrow_mut() = Some(copy.register(HookWallet::new("collision")).unwrap());
        }
    });
    let outer = registry.register(HookWallet::new("collision")).unwrap();
    let names: Vec<_> = registry
        .wallets()
        .iter()
        .map(|wallet| wallet.name().to_owned())
        .collect();
    assert_eq!(names, ["existing", "collision"]);
    assert_eq!(*events.borrow(), [RegistryEventKind::Registered; 2]);
    drop(outer);
    assert_eq!(registry.wallets().len(), 2);
    existing.clear();
    nested.borrow_mut().take();
    drop(original);
    assert!(registry.wallets().is_empty());
    assert_eq!(events.borrow().len(), 4);
}

#[test]
fn getter_removing_matching_identity_does_not_discard_incoming_wallet() {
    let registry = WalletRegistry::new();
    let (_listener, events) = events(&registry);
    let existing = HookWallet::new("collision");
    let identity: Rc<dyn Wallet> = existing.clone();
    let old = registry.register(identity.clone()).unwrap();
    let copy = registry.clone();
    existing.hook(move || copy.unregister(&identity));
    let incoming: Rc<dyn Wallet> = HookWallet::new("collision");
    let current = registry.register(incoming.clone()).unwrap();
    let wallets = registry.wallets();
    assert_eq!(wallets.len(), 1);
    assert!(Rc::ptr_eq(&wallets[0], &incoming));
    assert_eq!(
        *events.borrow(),
        [
            RegistryEventKind::Registered,
            RegistryEventKind::Unregistered,
            RegistryEventKind::Registered
        ]
    );
    existing.clear();
    drop(old);
    assert_eq!(registry.wallets().len(), 1);
    drop(current);
    assert_eq!(events.borrow().len(), 4);
}

#[test]
fn identity_deduplication_does_not_depend_on_matching_name_reads() {
    let registry = WalletRegistry::new();
    let (_listener, events) = events(&registry);
    let same = HookWallet::new("first");
    let first = registry.register(same.clone()).unwrap();
    let weak = Rc::downgrade(&same);
    let read = Cell::new(false);
    same.hook(move || {
        weak.upgrade().unwrap().name.set(if read.replace(true) {
            "second"
        } else {
            "first"
        })
    });
    let duplicate = registry.register(same.clone()).unwrap();
    assert_eq!(registry.wallets().len(), 1);
    assert_eq!(*events.borrow(), [RegistryEventKind::Registered]);
    same.clear();
    drop(duplicate);
    assert_eq!(registry.wallets().len(), 1);
    drop(first);
    assert_eq!(events.borrow().len(), 2);
}

#[test]
fn identity_is_rechecked_when_incoming_getter_registers_the_same_rc() {
    let registry = WalletRegistry::new();
    let (_listener, events) = events(&registry);
    let same = HookWallet::new("same");
    let weak = Rc::downgrade(&same);
    let nested = Rc::new(RefCell::new(None::<Registration>));
    let retained = nested.clone();
    let copy = registry.clone();
    let armed = Cell::new(true);
    same.hook(move || {
        if armed.replace(false) {
            *retained.borrow_mut() = Some(copy.register(weak.upgrade().unwrap()).unwrap());
        }
    });
    let outer = registry.register(same.clone()).unwrap();
    assert_eq!(registry.wallets().len(), 1);
    assert_eq!(*events.borrow(), [RegistryEventKind::Registered]);
    same.clear();
    drop(outer);
    assert_eq!(registry.wallets().len(), 1);
    nested.borrow_mut().take();
    assert!(registry.wallets().is_empty());
    assert_eq!(events.borrow().len(), 2);
}
