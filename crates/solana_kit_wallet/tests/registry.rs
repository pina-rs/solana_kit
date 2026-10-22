mod common;
use common::*;
use solana_kit_wallet::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[test]
fn registry_deduplicates_names_and_identity_and_releases_owners() {
    let registry = WalletRegistry::new();
    registry.initialize();
    let events = Rc::new(RefCell::new(Vec::new()));
    let observed = events.clone();
    let mut subscription = registry
        .on_event(move |event| observed.borrow_mut().push(event.kind))
        .unwrap();
    let wallet = TestWallet::named("Test");
    let mut registration = registry.register(wallet.clone()).unwrap();
    let duplicate = registry.register(TestWallet::named("Test")).unwrap();

    drop(duplicate);
    assert_eq!(registry.wallets().len(), 1);
    registry.changed(&wallet);
    registry.changed(&TestWallet::named("Test"));
    let snapshot = registry.wallets();
    registry.unregister(&TestWallet::named("Test"));
    assert_eq!(registry.wallets().len(), 1);
    registration.unregister();

    registration.unregister();
    assert!(registry.wallets().is_empty());
    assert_eq!(snapshot.len(), 1);
    assert_eq!(
        *events.borrow(),
        [
            RegistryEventKind::Registered,
            RegistryEventKind::Changed,
            RegistryEventKind::Unregistered
        ]
    );
    subscription.unsubscribe();
    subscription.unsubscribe();
    let registration = registry.register(wallet).unwrap();
    drop(registration);

    assert_eq!(events.borrow().len(), 3);
}

#[test]
fn reentrant_unregister_does_not_hold_mutable_registry_borrow() {
    let registry = WalletRegistry::new();
    let owner = registry.clone();
    let _subscription = registry
        .on_event(move |event| {
            assert_eq!(
                owner.wallets().len(),
                usize::from(event.kind == RegistryEventKind::Registered)
            );
            if event.kind == RegistryEventKind::Registered {
                owner.unregister(&event.wallet);
            }
        })
        .unwrap();
    let guard = registry.register(TestWallet::named("Test")).unwrap();
    assert!(registry.wallets().is_empty());
    drop(guard);
}

#[test]
fn stale_guard_cannot_unregister_a_new_registration_of_the_same_wallet() {
    let registry = WalletRegistry::new();
    let wallet = TestWallet::named("Test");
    let old = registry.register(wallet.clone()).unwrap();
    registry.unregister(&wallet);
    let current = registry.register(wallet).unwrap();
    drop(old);
    assert_eq!(registry.wallets().len(), 1);
    drop(current);

    assert!(registry.wallets().is_empty());
}

#[test]
fn callback_can_remove_itself_and_dispose_registry() {
    let registry = WalletRegistry::new();
    let subscription = Rc::new(RefCell::new(None::<RegistrySubscription>));
    let captured = subscription.clone();
    let owner = registry.clone();
    *subscription.borrow_mut() = Some(
        registry
            .on_event(move |_| {
                captured.borrow_mut().take();
                owner.dispose();
            })
            .unwrap(),
    );
    let guard = registry.register(TestWallet::named("Test")).unwrap();
    assert!(subscription.borrow().is_none());
    assert!(registry.wallets().is_empty());
    assert!(matches!(
        registry.register(TestWallet::named("Test")),
        Err(RegistryError::Disposed)
    ));

    assert!(matches!(
        registry.on_event(|_| {}),
        Err(RegistryError::Disposed)
    ));
    registry.dispose();
    drop(guard);
}

#[test]
fn disposal_and_drop_preserve_caller_wallet_ownership() {
    let registry = WalletRegistry::new();
    let wallet = TestWallet::named("Test");
    let weak_wallet = Rc::downgrade(&wallet);
    let registration = registry.register(wallet.clone()).unwrap();
    assert_eq!(Rc::strong_count(&wallet), 2);
    registry.dispose();
    assert_eq!(Rc::strong_count(&wallet), 1);
    registry.changed(&wallet);

    registry.unregister(&wallet);
    drop(registry);
    drop(registration);
    assert!(weak_wallet.upgrade().is_some());
    drop(wallet);
    assert!(weak_wallet.upgrade().is_none());
}

#[test]
fn listeners_do_not_keep_registry_or_wallets_alive() {
    let registry = WalletRegistry::new();
    let wallet = TestWallet::named("Test");
    let weak_wallet = Rc::downgrade(&wallet);
    let registration = registry.register(wallet.clone()).unwrap();
    let subscription = registry.on_event(|_| {}).unwrap();
    drop(wallet);
    drop(registry);
    assert!(weak_wallet.upgrade().is_none());

    drop(subscription);
    drop(registration);
}

struct ReentrantName {
    inner: Rc<dyn Wallet>,
    registry: WalletRegistry,
    name: Cell<&'static str>,
}

impl Wallet for ReentrantName {
    fn version(&self) -> &str {
        self.inner.version()
    }
    fn name(&self) -> &str {
        let _snapshot = self.registry.wallets();
        self.name.get()
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
fn canonical_name_callback_runs_outside_registry_borrows() {
    let registry = WalletRegistry::new();
    let wallet = Rc::new(ReentrantName {
        inner: TestWallet::named("Reentrant"),
        registry: registry.clone(),
        name: Cell::new("Reentrant"),
    });
    let first = registry.register(wallet).unwrap();
    let second = registry.register(TestWallet::named("Other")).unwrap();
    assert_eq!(registry.wallets().len(), 2);
    drop(first);
    drop(second);
    assert!(registry.wallets().is_empty());
}

#[test]
fn name_deduplication_observes_the_current_backend_name() {
    let registry = WalletRegistry::new();
    let wallet = Rc::new(ReentrantName {
        inner: TestWallet::named("Original"),
        registry: registry.clone(),
        name: Cell::new("Original"),
    });
    let original = registry.register(wallet.clone()).unwrap();
    wallet.name.set("Renamed");
    let duplicate = registry.register(TestWallet::named("Renamed")).unwrap();
    assert_eq!(registry.wallets().len(), 1);
    drop(duplicate);
    let replacement = registry.register(TestWallet::named("Original")).unwrap();

    assert_eq!(registry.wallets().len(), 2);
    drop(original);
    drop(replacement);
}

struct ReadOnDrop(WalletRegistry);
impl Drop for ReadOnDrop {
    fn drop(&mut self) {
        assert!(self.0.wallets().is_empty());
    }
}

#[test]
fn listener_capture_destructor_can_reenter_after_unsubscribe() {
    let registry = WalletRegistry::new();
    let capture = ReadOnDrop(registry.clone());
    let listener = registry
        .on_event(move |_| {
            let _held = &capture;
        })
        .unwrap();
    drop(listener);
    registry.dispose();
}
