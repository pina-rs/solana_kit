//! Native value contracts replace Dart's Rust-like Option object wrappers.

use alloc::format;
use alloc::vec::Vec;
extern crate std;

use core::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const VALUE: u16 = 513;

pub(super) fn native_variants_preserve_presence_nested_absence_and_value_extraction() {
    const PRESENT: Option<u16> = Some(VALUE);
    const ABSENT: Option<u16> = None;
    const NESTED_ABSENT: Option<Option<u16>> = Some(None);
    assert!(PRESENT.is_some());
    assert!(!PRESENT.is_none());
    assert!(ABSENT.is_none());
    assert!(!ABSENT.is_some());
    assert_ne!(NESTED_ABSENT, None);

    assert_eq!(NESTED_ABSENT, Some(ABSENT));
    let Some(value) = PRESENT else {
        panic!("present value lost")
    };
    assert_eq!(value, VALUE);
    assert_eq!(PRESENT, Some(VALUE));
    assert_eq!(ABSENT, None::<u16>);
    assert_ne!(PRESENT, Some(VALUE + 1));
}

pub(super) fn nullable_adaptation_is_identity_without_collapsing_nested_values() {
    fn nullable<T>(value: Option<T>) -> Option<T> {
        value
    }
    assert_eq!(nullable(Some(VALUE)), Some(VALUE));
    assert_eq!(nullable(None::<u16>), None);
    assert_eq!(nullable(Some(None::<u16>)), Some(None));
    let input = [Some(Some(VALUE)), Some(None), None];
    let flattened: Vec<_> = input.into_iter().map(Option::flatten).collect();
    assert_eq!(flattened, [Some(VALUE), None, None]);
}

pub(super) fn native_fallback_runs_only_for_absence() {
    let calls = Cell::new(0);
    let fallback = || {
        calls.set(calls.get() + 1);
        VALUE
    };
    let present = Some(7);
    let absent = None::<u16>;
    assert_eq!(resolve(present, fallback), 7);
    assert_eq!(calls.get(), 0);
    assert_eq!(resolve(absent, fallback), VALUE);
    assert_eq!(calls.get(), 1);
}

fn resolve(value: Option<u16>, fallback: impl FnOnce() -> u16) -> u16 {
    value.unwrap_or_else(fallback)
}

pub(super) fn native_equality_hash_and_numeric_debug_have_explicit_bounds() {
    fn hash(value: Option<u16>) -> u64 {
        let mut state = DefaultHasher::new();
        value.hash(&mut state);
        state.finish()
    }
    assert_eq!(hash(Some(VALUE)), hash(Some(VALUE)));
    assert_eq!(hash(None), hash(None));
    assert_eq!(format!("{:?}", Some(VALUE)), "Some(513)");
    assert_eq!(format!("{:?}", None::<u16>), "None");
}
