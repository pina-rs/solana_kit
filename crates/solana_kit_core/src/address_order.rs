//! Locale-independent address text collation from the pinned client contract.

use alloc::{string::ToString, vec::Vec};
use core::cmp::Ordering;

use crate::Address;

/// Compares canonical address text with digits first and lowercase before the
/// matching uppercase letter. Prefixes sort before their longer extensions.
///
/// This is distinct from `Address`'s byte-wise `Ord`. It allocates the two
/// canonical strings and requires no locale, JavaScript, or host collator.
#[must_use]
pub fn compare_addresses(left: &Address, right: &Address) -> Ordering {
    left.to_string()
        .bytes()
        .map(rank)
        .cmp(right.to_string().bytes().map(rank))
}

pub(crate) fn sort_key(address: Address) -> Vec<u8> {
    address.to_string().bytes().map(rank).collect()
}

fn rank(byte: u8) -> u8 {
    if byte.is_ascii_digit() {
        return byte - b'0';
    }

    (byte.to_ascii_lowercase() - b'a') * 2 + 10 + u8::from(byte.is_ascii_uppercase())
}
