#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

/// Complete pinned instruction, event, header, privilege and error contracts.
pub mod generated;
mod tree;

pub use generated::*;
pub use tree::*;

use solana_kit_core::{AccountMeta, AccountRole, Address};

/// Canonical SPL Noop program used by the mutating instruction wrappers.
pub const NOOP_PROGRAM: Address = Address::from_bytes([
    11, 188, 15, 192, 187, 71, 202, 47, 116, 196, 17, 46, 148, 171, 19, 207, 163, 198, 52, 229,
    220, 23, 234, 203, 3, 205, 26, 35, 205, 126, 120, 124,
]);

/// Ordered roles for initialization, append, replace and insert-or-append.
/// Append readonly proof nodes after these three entries when required.
pub fn modification_accounts(tree: Address, authority: Address) -> [AccountMeta; 3] {
    [
        AccountMeta {
            address: tree,
            role: AccountRole::Writable,
        },
        AccountMeta {
            address: authority,
            role: AccountRole::ReadonlySigner,
        },
        AccountMeta {
            address: NOOP_PROGRAM,
            role: AccountRole::Readonly,
        },
    ]
}
