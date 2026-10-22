use crate::common::*;
use solana_kit_wallet::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

struct Capture(Box<dyn Fn()>);

impl Drop for Capture {
    fn drop(&mut self) {
        (self.0)();
    }
}

#[derive(Clone, Copy, Debug)]
enum Mutation {
    Insert,
    Remove,
    Change,
    Dispose,
    Listeners,
    DropGuard,
    DropCapture,
}

struct Context {
    registry: WalletRegistry,
    first: Rc<Probe>,
    owner: Rc<RefCell<Option<Registration>>>,
    nested: Rc<RefCell<Option<Registration>>>,
}

impl Context {
    fn insert(&self) {
        *self.nested.borrow_mut() = Some(self.registry.register(Probe::new("fresh")).unwrap());
    }

    fn mutate(&self, mutation: Mutation) {
        let identity: Rc<dyn Wallet> = self.first.clone();

        match mutation {
            Mutation::Insert => self.insert(),
            Mutation::Remove => self.registry.unregister(&identity),
            Mutation::Change => {
                self.first.name.set("fresh");
                self.registry.changed(&identity);
            }
            Mutation::Dispose => self.registry.dispose(),
            Mutation::Listeners => drop(self.registry.on_event(|_| {}).unwrap()),
            Mutation::DropGuard => {
                let guard = self.owner.borrow_mut().take();
                drop(guard);
            }
            Mutation::DropCapture => self.drop_capture(),
        }
    }

    fn drop_capture(&self) {
        let registry = self.registry.clone();
        let nested = self.nested.clone();
        let capture = Capture(Box::new(move || {
            assert_eq!(registry.wallets().len(), 2);
            *nested.borrow_mut() = Some(registry.register(Probe::new("fresh")).unwrap());
        }));
        let listener = self
            .registry
            .on_event(move |_| {
                let _ = &capture;
            })
            .unwrap();
        drop(listener);
    }
}

pub fn run() {
    for target in 0..3 {
        for mutation in [
            Mutation::Insert,
            Mutation::Remove,
            Mutation::Change,
            Mutation::Dispose,
            Mutation::Listeners,
            Mutation::DropGuard,
            Mutation::DropCapture,
        ] {
            scenario(target, mutation);
        }
    }
}

fn scenario(target: usize, mutation: Mutation) {
    let registry = WalletRegistry::new();
    let (_listener, log) = events(&registry);
    let first = Probe::new("left");
    let owner = Rc::new(RefCell::new(Some(
        registry.register(first.clone()).unwrap(),
    )));
    let last = Probe::new("right");
    let last_owner = registry.register(last.clone()).unwrap();
    let incoming = Probe::new("fresh");
    let probes = [&incoming, &first, &last];
    let context = Context {
        registry: registry.clone(),
        first: first.clone(),
        owner: owner.clone(),
        nested: Rc::new(RefCell::new(None)),
    };
    let retained = context.nested.clone();
    let armed = Cell::new(true);
    let prior_reads = probes[target].reads.get();
    *probes[target].hook.borrow_mut() = Some(Rc::new(move || {
        if armed.replace(false) {
            context.mutate(mutation);
        }
    }));
    let result = registry.register(incoming.clone());
    let reads = probes[target].reads.get() - prior_reads;
    probes[target].clear();
    verify(mutation, &registry, &incoming, &log, &result, reads);
    drop(result);
    retained.borrow_mut().take();
    owner.borrow_mut().take();
    drop(last_owner);
    assert!(registry.wallets().is_empty());
}

fn verify(
    mutation: Mutation,
    registry: &WalletRegistry,
    incoming: &Rc<Probe>,
    log: &EventLog,
    result: &Result<Registration, RegistryError>,
    reads: usize,
) {
    use RegistryEventKind::{Changed, Registered, Unregistered};
    let identity: Rc<dyn Wallet> = incoming.clone();

    match mutation {
        Mutation::Dispose => {
            assert!(matches!(result, Err(RegistryError::Disposed)));
            assert!(registry.wallets().is_empty());
            assert_eq!(*log.borrow(), [Registered, Registered]);
        }
        Mutation::Remove | Mutation::DropGuard => {
            assert!(result.is_ok());
            assert!(contains(registry, &identity));
            assert_eq!(
                *log.borrow(),
                [Registered, Registered, Unregistered, Registered]
            );
        }
        Mutation::Change => {
            assert!(result.is_ok());
            assert!(!contains(registry, &identity));
            assert_eq!(*log.borrow(), [Registered, Registered, Changed]);
        }
        Mutation::Listeners => {
            assert!(contains(registry, &identity));
            assert_eq!(
                reads, 1,
                "listener-only changes do not invalidate wallet observations"
            );
        }
        Mutation::Insert | Mutation::DropCapture => {
            assert!(!contains(registry, &identity));
            assert_eq!(registry.wallets().len(), 3);
            assert_eq!(*log.borrow(), [Registered; 3]);
        }
    }
}
