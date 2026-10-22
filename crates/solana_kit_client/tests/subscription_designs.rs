//! Executable ownership comparison. Both retain a notification before its ack.
use std::collections::{BTreeMap, VecDeque};

#[test]
fn enum_dispatch_prototype_requires_family_variants_at_every_boundary() {
    enum Event {
        Root(u64),
        Slot { slot: u64 },
    }
    let mut early = BTreeMap::from([(7_u64, VecDeque::from([Event::Root(99)]))]);
    let acknowledged = 7;
    let events = early.remove(&acknowledged).unwrap();
    assert!(matches!(events.front(), Some(Event::Root(99))));
    let slot = Event::Slot { slot: 100 };
    assert!(matches!(slot, Event::Slot { slot: 100 }));
    assert!(early.is_empty());
}

#[test]
fn typed_handle_prototype_keeps_one_registry_and_typed_results() {
    struct Handle<T> {
        server: u64,
        parse: fn(&str) -> T,
    }
    let handle = Handle {
        server: 7,
        parse: |raw: &str| raw.parse::<u64>().unwrap(),
    };
    let mut early = BTreeMap::from([(7_u64, VecDeque::from(["99"]))]);
    let raw = early.get_mut(&handle.server).unwrap().pop_front().unwrap();
    let value: u64 = (handle.parse)(raw);
    assert_eq!(value, 99);
    assert!(early[&handle.server].is_empty());
}
