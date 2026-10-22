mod reentry;
use reentry::*;
use solana_kit_wallet::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[test]
fn changed_during_getter_invalidates_already_observed_names() {
    let registry = WalletRegistry::new();
    let (_listener, events) = events(&registry);
    let earlier = HookWallet::new("earlier");
    let first = registry.register(earlier.clone()).unwrap();
    let trigger = HookWallet::new("trigger");
    let second = registry.register(trigger.clone()).unwrap();
    let copy = registry.clone();
    let renamed = earlier.clone();
    let armed = Cell::new(true);
    trigger.hook(move || {
        if armed.replace(false) {
            renamed.name.set("collision");
            let identity: Rc<dyn Wallet> = renamed.clone();
            copy.changed(&identity);
        }
    });
    let incoming = registry.register(HookWallet::new("collision")).unwrap();
    assert_eq!(registry.wallets().len(), 2);
    assert_eq!(
        *events.borrow(),
        [
            RegistryEventKind::Registered,
            RegistryEventKind::Registered,
            RegistryEventKind::Changed
        ]
    );
    trigger.clear();
    drop(incoming);
    drop(first);
    drop(second);
    assert_eq!(events.borrow().len(), 5);
}

#[test]
fn persistent_mutation_fails_finitely_and_preserves_nested_events() {
    let registry = WalletRegistry::new();
    let (_listener, events) = events(&registry);
    let existing = HookWallet::new("existing");
    let first = registry.register(existing.clone()).unwrap();
    let weak = Rc::downgrade(&existing);
    let copy = registry.clone();
    let reads = Rc::new(Cell::new(0));
    let count = reads.clone();
    existing.hook(move || {
        count.set(count.get() + 1);
        let identity: Rc<dyn Wallet> = weak.upgrade().unwrap();
        copy.changed(&identity);
    });
    assert!(matches!(
        registry.register(HookWallet::new("incoming")),
        Err(RegistryError::UnstableRegistration)
    ));
    assert_eq!(reads.get(), 32);
    assert_eq!(registry.wallets().len(), 1);
    assert_eq!(events.borrow().len(), 33);
    assert!(
        events.borrow()[1..]
            .iter()
            .all(|kind| *kind == RegistryEventKind::Changed)
    );
    existing.clear();
    let incoming = registry.register(HookWallet::new("incoming")).unwrap();
    assert_eq!(registry.wallets().len(), 2);
    drop(first);
    drop(incoming);
    assert_eq!(events.borrow().len(), 36);
}

#[test]
fn finite_nested_mutations_complete_on_the_last_allowed_pass() {
    let registry = WalletRegistry::new();
    let existing = HookWallet::new("existing");
    let first = registry.register(existing.clone()).unwrap();
    let weak = Rc::downgrade(&existing);
    let copy = registry.clone();
    let reads = Rc::new(Cell::new(0));
    let count = reads.clone();
    existing.hook(move || {
        count.set(count.get() + 1);

        if count.get() < 32 {
            let identity: Rc<dyn Wallet> = weak.upgrade().unwrap();
            copy.changed(&identity);
        }
    });
    let incoming = registry.register(HookWallet::new("incoming")).unwrap();
    assert_eq!(reads.get(), 32);
    assert_eq!(registry.wallets().len(), 2);
    existing.clear();
    drop(first);
    drop(incoming);
}

#[test]
fn getter_disposal_rejects_outer_registration_without_an_event() {
    let registry = WalletRegistry::new();
    let (_listener, events) = events(&registry);
    let existing = HookWallet::new("existing");
    let first = registry.register(existing.clone()).unwrap();
    let copy = registry.clone();
    existing.hook(move || copy.dispose());
    assert!(matches!(
        registry.register(HookWallet::new("incoming")),
        Err(RegistryError::Disposed)
    ));
    assert!(registry.wallets().is_empty());
    assert_eq!(*events.borrow(), [RegistryEventKind::Registered]);
    existing.clear();
    drop(first);
}

#[test]
fn getter_dropping_nested_guard_releases_matching_entry_before_decision() {
    let registry = WalletRegistry::new();
    let (_listener, events) = events(&registry);
    let existing = HookWallet::new("collision");
    let owner = Rc::new(RefCell::new(Some(
        registry.register(existing.clone()).unwrap(),
    )));
    let released = owner.clone();
    existing.hook(move || {
        let registration = released.borrow_mut().take();
        drop(registration);
    });
    let incoming = registry.register(HookWallet::new("collision")).unwrap();
    assert_eq!(registry.wallets().len(), 1);
    assert_eq!(events.borrow().len(), 3);
    existing.clear();
    drop(incoming);
    assert_eq!(events.borrow().len(), 4);
}

struct RegisterOnDrop {
    registry: WalletRegistry,
    retained: Rc<RefCell<Option<Registration>>>,
    count: Rc<Cell<usize>>,
}

impl Drop for RegisterOnDrop {
    fn drop(&mut self) {
        self.count.set(self.count.get() + 1);
        *self.retained.borrow_mut() = Some(
            self.registry
                .register(HookWallet::new("collision"))
                .unwrap(),
        );
    }
}

#[test]
fn getter_unsubscribe_drops_capture_outside_borrow_and_revalidates() {
    let registry = WalletRegistry::new();
    let (_listener, events) = events(&registry);
    let existing = HookWallet::new("existing");
    let original = registry.register(existing.clone()).unwrap();
    let nested = Rc::new(RefCell::new(None));
    let count = Rc::new(Cell::new(0));
    let capture = RegisterOnDrop {
        registry: registry.clone(),
        retained: nested.clone(),
        count: count.clone(),
    };
    let subscription = registry
        .on_event(move |_| {
            let _ = &capture;
        })
        .unwrap();
    let owner = RefCell::new(Some(subscription));
    existing.hook(move || {
        let subscription = owner.borrow_mut().take();
        drop(subscription);
    });
    let outer = registry.register(HookWallet::new("collision")).unwrap();
    assert_eq!(count.get(), 1);
    assert_eq!(registry.wallets().len(), 2);
    assert_eq!(*events.borrow(), [RegistryEventKind::Registered; 2]);
    existing.clear();
    drop(outer);
    nested.borrow_mut().take();
    drop(original);
    assert_eq!(events.borrow().len(), 4);
}
