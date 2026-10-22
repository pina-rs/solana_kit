//! Compiler collation from the pinned Dart address comparator.

use alloc::{string::ToString, vec::Vec};

use crate::Address;

/// Orders base58 digits first, then each lowercase letter before its uppercase.
///
/// Address text contains only base58 ASCII. These per-character ranks preserve
/// the reference comparator's prefix-length tie-break without a host collator.
pub(super) fn address_key(address: Address) -> Vec<u8> {
    address
        .to_string()
        .bytes()
        .map(|byte| {
            if byte.is_ascii_digit() {
                byte - b'0'
            } else {
                (byte.to_ascii_lowercase() - b'a') * 2 + 10 + u8::from(byte.is_ascii_uppercase())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_base58_character_uses_pinned_collation() {
        let alphabet = "123456789aAbBcCdDeEfFgGhHijJkKLmMnNopPqQrRsStTuUvVwWxXyYzZ";
        let mut addresses: Vec<Address> = alphabet
            .chars()
            .map(|character| {
                alloc::format!("1111111111111111111111111111111{character}")
                    .parse()
                    .unwrap()
            })
            .collect();
        let expected = addresses.clone();
        addresses.reverse();
        addresses.sort_by_key(|address| address_key(*address));

        assert_eq!(addresses, expected);
    }

    #[test]
    fn equal_and_valid_prefix_addresses_match_reference() {
        let short: Address = "5111111111111111111111111111111111111111111"
            .parse()
            .unwrap();
        let long: Address = "51111111111111111111111111111111111111111111"
            .parse()
            .unwrap();

        assert!(address_key(short) < address_key(long));
        assert_eq!(address_key(short), address_key(short));
    }
}
