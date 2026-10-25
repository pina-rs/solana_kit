//! Pinned well known address literals for off-chain clients.

use crate::Address;

/// `So11111111111111111111111111111111111111112`.
pub const WRAPPED_SOL_MINT: Address = Address::from_bytes([
    6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218, 196, 57, 220, 26,
    235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
]);

/// `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`.
pub const USDC_MINT: Address = Address::from_bytes([
    198, 250, 122, 243, 190, 219, 173, 58, 61, 101, 243, 106, 171, 201, 116, 49, 177, 187, 228,
    194, 210, 246, 224, 228, 124, 166, 2, 3, 69, 47, 93, 97,
]);

/// `Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB`.
pub const USDT_MINT: Address = Address::from_bytes([
    206, 1, 14, 96, 175, 237, 178, 39, 23, 189, 99, 25, 47, 84, 20, 90, 63, 150, 90, 51, 187, 130,
    210, 199, 2, 158, 178, 206, 30, 32, 130, 100,
]);
