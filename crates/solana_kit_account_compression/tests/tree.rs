//! Independent primary/Dart fixtures retain every pair and literal allocation size.
use solana_kit_account_compression::*;
use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::Error};

const FIRST_DEPTH: u32 = 3;
const FIRST_BUFFER: u32 = 8;
const EXCESS_CANOPY: u32 = 4;
const DEPTH_LIMIT: u32 = 32;
const EXPECTED_PAIRS: usize = 34;
const EXPECTED_HEADER: usize = 56;
const INVALID_CANOPIES: &[usize] = &[1, 31, 32, 63, 65, 96, usize::MAX];
const BUFFER_PROBES: &[u32] = &[0, 8, 16, 32, 64, 256, 512, 1024, 2048, 4096];

fn numbers(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|part| part.parse().unwrap())
        .collect()
}

#[cfg_attr(test, test)]
pub fn primary_allocation_vectors_and_inverse_canopies() {
    for line in include_str!("fixtures/allocation.tsv").lines() {
        let values = numbers(line);
        let (depth, buffer, canopy, bytes) = (values[0], values[1], values[2], values[3]);
        assert_eq!(
            account_size(depth, buffer, canopy),
            Ok(usize::try_from(bytes).unwrap())
        );
        assert_eq!(canopy_depth(canopy_size(canopy).unwrap()), Ok(canopy));
    }

    let pairs: Vec<_> = include_str!("fixtures/pairs.tsv")
        .lines()
        .map(|line| {
            let values = numbers(line);
            (values[0], values[1])
        })
        .collect();
    assert_eq!(HEADER_SIZE_V1, EXPECTED_HEADER);
    assert_eq!(VALID_DEPTH_BUFFER_PAIRS, pairs);
    assert_eq!(pairs.len(), EXPECTED_PAIRS);
    pair_matrix(&pairs);
}

fn pair_matrix(pairs: &[(u32, u32)]) {
    for depth in 0..=DEPTH_LIMIT {
        for &buffer in BUFFER_PROBES {
            let expected = pairs.contains(&(depth, buffer));
            assert_eq!(is_valid_depth_buffer_pair(depth, buffer), expected);
            assert_eq!(account_size(depth, buffer, 0).is_ok(), expected);
        }
    }
}

#[cfg_attr(test, test)]
pub fn invalid_and_overflowing_canopies_fail_explicitly() {
    assert_eq!(
        account_size(FIRST_DEPTH, FIRST_BUFFER, EXCESS_CANOPY),
        Err(Error::InvalidValue)
    );
    assert_eq!(
        account_size(u32::MAX, FIRST_BUFFER, 0),
        Err(Error::InvalidValue)
    );
    assert_eq!(canopy_size(u32::MAX), Err(Error::Overflow));
    assert_eq!(canopy_size(usize::BITS), Err(Error::Overflow));

    for &bytes in INVALID_CANOPIES {
        assert!(canopy_depth(bytes).is_err());
    }
}

#[cfg_attr(test, test)]
pub fn canonical_addresses_and_default_roles_are_source_grounded() {
    assert_eq!(
        PROGRAM.to_string(),
        "cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK"
    );
    assert_eq!(
        NOOP_PROGRAM.to_string(),
        "noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV"
    );

    let tree = Address::from_bytes([1; 32]);
    let authority = Address::from_bytes([2; 32]);
    assert_eq!(
        modification_accounts(tree, authority),
        [
            AccountMeta {
                address: tree,
                role: AccountRole::Writable
            },
            AccountMeta {
                address: authority,
                role: AccountRole::ReadonlySigner
            },
            AccountMeta {
                address: NOOP_PROGRAM,
                role: AccountRole::Readonly
            },
        ]
    );
}
