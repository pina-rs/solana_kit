//! Pinned current and historical Memo program identities.

use solana_kit_core::Address;

/// Current v4 Memo program.
pub const PROGRAM: Address = Address::from_bytes([
    5, 74, 83, 82, 40, 84, 205, 209, 3, 204, 46, 70, 49, 163, 245, 23, 146, 172, 114, 124, 241,
    229, 10, 188, 151, 130, 138, 197, 153, 79, 60, 194,
]);
/// Original v1 Memo program.
pub const LEGACY_PROGRAM: Address = Address::from_bytes([
    5, 74, 83, 80, 248, 93, 200, 130, 214, 20, 165, 86, 114, 120, 138, 41, 109, 223, 30, 171, 171,
    208, 166, 6, 120, 136, 73, 50, 244, 238, 246, 160,
]);
/// Legacy v3 Memo program.
pub const LEGACY_PROGRAM_V3: Address = Address::from_bytes([
    5, 74, 83, 90, 153, 41, 33, 6, 77, 36, 232, 113, 96, 218, 56, 124, 124, 53, 181, 221, 188, 146,
    187, 129, 228, 31, 168, 64, 65, 5, 68, 141,
]);
/// All supported identities in deployment order.
pub const SUPPORTED_PROGRAMS: [Address; 3] = [LEGACY_PROGRAM, LEGACY_PROGRAM_V3, PROGRAM];
