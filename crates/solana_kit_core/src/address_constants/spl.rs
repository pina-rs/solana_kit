//! Pinned spl address literals for off-chain clients.

use crate::Address;

/// `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`.
pub const TOKEN_PROGRAM: Address = Address::from_bytes([
    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237,
    95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
]);

/// `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`.
pub const TOKEN2022_PROGRAM: Address = Address::from_bytes([
    6, 221, 246, 225, 238, 117, 143, 222, 24, 66, 93, 188, 228, 108, 205, 218, 182, 26, 252, 77,
    131, 185, 13, 39, 254, 189, 249, 40, 216, 161, 139, 252,
]);

/// `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`.
pub const ASSOCIATED_TOKEN_PROGRAM: Address = Address::from_bytes([
    140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131, 11, 90, 19, 153, 218,
    255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
]);

/// `Memo4c2pN8afCj432Lb7RMVKi9PbQnnW7ewFFaV3oAH`.
pub const MEMO_PROGRAM: Address = Address::from_bytes([
    5, 74, 83, 82, 40, 84, 205, 209, 3, 204, 46, 70, 49, 163, 245, 23, 146, 172, 114, 124, 241,
    229, 10, 188, 151, 130, 138, 197, 153, 79, 60, 194,
]);

/// `Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo`.
pub const MEMO_LEGACY_PROGRAM: Address = Address::from_bytes([
    5, 74, 83, 80, 248, 93, 200, 130, 214, 20, 165, 86, 114, 120, 138, 41, 109, 223, 30, 171, 171,
    208, 166, 6, 120, 136, 73, 50, 244, 238, 246, 160,
]);

/// `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr`.
pub const MEMO_LEGACY_PROGRAM_ADDRESS_V3: Address = Address::from_bytes([
    5, 74, 83, 90, 153, 41, 33, 6, 77, 36, 232, 113, 96, 218, 56, 124, 124, 53, 181, 221, 188, 146,
    187, 129, 228, 31, 168, 64, 65, 5, 68, 141,
]);
