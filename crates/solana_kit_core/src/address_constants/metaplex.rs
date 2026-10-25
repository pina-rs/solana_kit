//! Pinned metaplex address literals for off-chain clients.

use crate::Address;

/// `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s`.
pub const TOKEN_METADATA_PROGRAM: Address = Address::from_bytes([
    11, 112, 101, 177, 227, 209, 124, 69, 56, 157, 82, 127, 107, 4, 195, 205, 88, 184, 108, 115,
    26, 160, 253, 181, 73, 182, 209, 188, 3, 248, 41, 70,
]);

/// `BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY`.
pub const MPL_BUBBLEGUM_PROGRAM: Address = Address::from_bytes([
    152, 139, 128, 235, 121, 53, 40, 105, 178, 36, 116, 95, 89, 221, 191, 138, 38, 88, 202, 19,
    220, 104, 129, 33, 38, 53, 28, 174, 7, 193, 165, 165,
]);

/// `auth9SigNpDKz4sJJ1DfCTuZrZNSAgh9sFD3rboVmgg`.
pub const MPL_TOKEN_AUTH_RULES_PROGRAM: Address = Address::from_bytes([
    8, 175, 248, 148, 195, 250, 103, 192, 22, 58, 48, 242, 52, 163, 61, 52, 103, 240, 58, 161, 100,
    23, 49, 94, 180, 184, 20, 39, 189, 155, 166, 237,
]);

/// `CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d`.
pub const MPL_CORE_PROGRAM: Address = Address::from_bytes([
    175, 84, 171, 16, 189, 151, 165, 66, 160, 158, 247, 179, 152, 137, 221, 12, 211, 148, 164, 204,
    233, 223, 166, 205, 201, 126, 190, 45, 35, 91, 167, 72,
]);

/// `cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK`.
pub const SPL_ACCOUNT_COMPRESSION_PROGRAM: Address = Address::from_bytes([
    9, 42, 19, 238, 149, 196, 28, 186, 8, 166, 127, 90, 198, 126, 141, 247, 225, 218, 17, 98, 94,
    29, 100, 19, 127, 143, 79, 35, 131, 3, 127, 20,
]);

/// `noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV`.
pub const NOOP_PROGRAM: Address = Address::from_bytes([
    11, 188, 15, 192, 187, 71, 202, 47, 116, 196, 17, 46, 148, 171, 19, 207, 163, 198, 52, 229,
    220, 23, 234, 203, 3, 205, 26, 35, 205, 126, 120, 124,
]);
