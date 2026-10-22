use crate::common::*;
use solana_kit_wallet::*;
use std::{cell::Cell, rc::Rc};

const PASSES: usize = 32;

pub fn run() {
    for candidate in [false, true] {
        attempt(candidate, PASSES - 1, true);
        attempt(candidate, PASSES, false);
        attempt(candidate, usize::MAX, false);
    }
}

fn attempt(candidate: bool, invalidations: usize, succeeds: bool) {
    let registry = WalletRegistry::new();
    let (_listener, log) = events(&registry);
    let existing = Probe::new("stable");
    let owner = registry.register(existing.clone()).unwrap();
    let incoming = Probe::new("incoming");
    let existing_count = Rc::strong_count(&existing);
    let incoming_count = Rc::strong_count(&incoming);
    let target = if candidate { &incoming } else { &existing };
    let reads = Rc::new(Cell::new(0));
    let peak = Rc::new(Cell::new(0));

    install(&registry, target, &existing, &reads, &peak, invalidations);
    let result = registry.register(incoming.clone());
    target.clear();

    assert_eq!(reads.get(), PASSES);
    assert!(
        peak.get() <= existing_count + 4,
        "snapshot references must not accumulate"
    );
    assert_eq!(Rc::strong_count(&existing), existing_count);
    assert_eq!(
        Rc::strong_count(&incoming),
        incoming_count + usize::from(succeeds)
    );

    inspect(result, succeeds, &registry, &incoming, &log);
    drop(owner);
    assert!(registry.wallets().is_empty());
}

fn install(
    registry: &WalletRegistry,
    target: &Rc<Probe>,
    existing: &Rc<Probe>,
    reads: &Rc<Cell<usize>>,
    peak: &Rc<Cell<usize>>,
    invalidations: usize,
) {
    let registry = registry.clone();
    let existing = Rc::downgrade(existing);
    let reads = reads.clone();
    let peak = peak.clone();
    *target.hook.borrow_mut() = Some(Rc::new(move || {
        reads.set(reads.get() + 1);
        let existing = existing.upgrade().unwrap();
        peak.set(peak.get().max(Rc::strong_count(&existing)));

        if reads.get() <= invalidations {
            let identity: Rc<dyn Wallet> = existing;
            registry.changed(&identity);
        }
    }));
}

fn inspect(
    result: Result<Registration, RegistryError>,
    succeeds: bool,
    registry: &WalletRegistry,
    incoming: &Rc<Probe>,
    log: &EventLog,
) {
    use RegistryEventKind::{Changed, Registered};

    if succeeds {
        let guard = result.unwrap();
        assert_eq!(registry.wallets().len(), 2);
        assert_eq!(log.borrow().len(), 33);
        assert_eq!(log.borrow()[32], Registered);
        assert!(log.borrow()[1..32].iter().all(|kind| *kind == Changed));
        drop(guard);

        return;
    }

    assert!(matches!(result, Err(RegistryError::UnstableRegistration)));
    assert_eq!(
        RegistryError::UnstableRegistration.to_string(),
        "UnstableRegistration"
    );
    assert_eq!(registry.wallets().len(), 1);
    assert_eq!(log.borrow().len(), 33);
    assert!(log.borrow()[1..].iter().all(|kind| *kind == Changed));

    let stable = registry.register(incoming.clone()).unwrap();
    assert_eq!(registry.wallets().len(), 2);
    assert_eq!(log.borrow().last(), Some(&Registered));
    drop(stable);
}
