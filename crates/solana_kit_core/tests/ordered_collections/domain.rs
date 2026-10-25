use alloc::{string::String, vec};
use solana_kit_core::collections::{OrderedMap, OrderedSet};

const FIRST_KEY_TAG: u8 = 20;
const DUPLICATE_KEY_TAG: u8 = 99;
const FIRST_REPLACEMENT: u8 = 7;
const MUTATED_VALUE: u8 = 11;

/// Equality deliberately ignores an object's tag, exposing first-object retention.
#[derive(Debug)]
struct Identity {
    key: u8,
    tag: u8,
}

impl PartialEq for Identity {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Identity {}

fn identity(key: u8, tag: u8) -> Identity {
    Identity { key, tag }
}

/// Duplicate updates retain the first actual key, not only its equal value.
pub fn map_replacements() {
    let entries = vec![
        (identity(2, FIRST_KEY_TAG), 3),
        (identity(1, 10), 4),
        (identity(2, DUPLICATE_KEY_TAG), FIRST_REPLACEMENT),
    ];
    let allocation = entries.as_ptr();
    let mut map = OrderedMap::from_entries(entries);
    assert_eq!(map.as_slice().as_ptr(), allocation);

    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice()[0].0.tag, FIRST_KEY_TAG);
    assert_eq!(map.as_slice()[1].0.key, 1);
    assert_eq!(map.get(&identity(2, 0)), Some(&FIRST_REPLACEMENT));

    assert_eq!(
        map.try_insert(identity(2, 88), 9).unwrap(),
        Some(FIRST_REPLACEMENT)
    );
    assert_eq!(map.as_slice()[0].0.tag, FIRST_KEY_TAG);
    *map.get_mut(&identity(1, 0)).unwrap() = MUTATED_VALUE;

    assert_eq!(map.remove(&identity(2, 0)), Some(9));
    assert_eq!(map.try_insert(identity(2, 30), 12).unwrap(), None);
    assert_eq!(map.as_slice()[0].0.key, 1);
    assert_eq!(map.as_slice()[1].0.tag, 30);
    assert_eq!((&map).into_iter().count(), 2);
    assert_eq!(
        map.into_iter()
            .map(|entry| entry.1)
            .collect::<alloc::vec::Vec<_>>(),
        vec![MUTATED_VALUE, 12]
    );

    map_value_equality();
    borrowed_keys();
}

fn borrowed_keys() {
    let mut map = OrderedMap::from_entries(vec![(String::from("wallet"), 1)]);
    assert_eq!(map.get("wallet"), Some(&1));
    *map.get_mut("wallet").unwrap() = 2;
    assert_eq!(map.remove("wallet"), Some(2));
    assert!(map.is_empty());

    let mut set = OrderedSet::from_values(vec![String::from("wallet")]);
    assert!(set.contains("wallet"));
    assert!(set.remove("wallet"));
    assert!(set.is_empty());
}

fn map_value_equality() {
    let first = OrderedMap::from_entries(vec![(2, 20), (1, 10)]);
    let second = OrderedMap::from_entries(vec![(1, 10), (2, 20)]);
    let empty: OrderedMap<Identity, Identity> = OrderedMap::default();
    assert_eq!(first, second);
    assert_ne!(first.as_slice(), second.as_slice());
    assert_eq!(first.into_entries(), vec![(2, 20), (1, 10)]);
    assert!(empty.is_empty());
}

/// Sets retain first actual values, while equality uses unordered membership.
pub fn set_membership() {
    let values = vec![identity(2, 20), identity(1, 10), identity(2, 99)];
    let allocation = values.as_ptr();
    let mut set = OrderedSet::from_values(values);
    assert_eq!(set.as_slice().as_ptr(), allocation);

    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice()[0].tag, 20);
    assert!(!set.try_insert(identity(2, 88)).unwrap());
    assert!(set.contains(&identity(2, 0)));

    assert!(!set.remove(&identity(3, 0)));
    assert!(set.remove(&identity(2, 0)));
    assert!(set.try_insert(identity(2, 30)).unwrap());
    assert_eq!(set.as_slice()[0].key, 1);
    assert_eq!(set.as_slice()[1].tag, 30);
    assert_eq!((&set).into_iter().count(), 2);

    assert_eq!(
        set.into_iter()
            .map(|value| value.key)
            .collect::<alloc::vec::Vec<_>>(),
        vec![1, 2]
    );

    let first = OrderedSet::from_values(vec![2, 1, 2]);
    let second = OrderedSet::from_values(vec![1, 2]);
    let empty: OrderedSet<Identity> = OrderedSet::default();
    assert_eq!(first, second);
    assert_ne!(first.as_slice(), second.as_slice());
    assert_eq!(first.into_values(), vec![2, 1]);
    assert!(empty.is_empty());
}
