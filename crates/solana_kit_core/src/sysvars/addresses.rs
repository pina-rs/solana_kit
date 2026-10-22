//! Exact address constants from the pinned Dart reference.

use crate::Address;

/// `Sysvar1111111111111111111111111111111111111`.
pub const OWNER_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 24, 117, 247, 41, 199, 61, 147, 64, 143, 33, 97, 32, 6, 126, 216, 140, 118,
    224, 140, 40, 127, 193, 148, 96, 0, 0, 0, 0,
]);

/// `SysvarC1ock11111111111111111111111111111111`.
pub const CLOCK_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 24, 199, 116, 201, 40, 86, 99, 152, 105, 29, 94, 182, 139, 94, 184, 163, 155,
    75, 109, 92, 115, 85, 91, 33, 0, 0, 0, 0,
]);

/// `SysvarEpochRewards1111111111111111111111111`.
pub const EPOCH_REWARDS_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 24, 220, 63, 238, 2, 165, 88, 191, 131, 206, 102, 225, 68, 66, 42, 28, 52,
    149, 11, 39, 193, 134, 155, 90, 156, 0, 0, 0,
]);

/// `SysvarEpochSchedu1e111111111111111111111111`.
pub const EPOCH_SCHEDULE_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 24, 220, 63, 238, 2, 211, 228, 127, 1, 0, 248, 176, 84, 247, 148, 46, 96, 89,
    30, 63, 80, 135, 25, 168, 5, 0, 0, 0,
]);

/// `SysvarFees111111111111111111111111111111111`.
pub const FEES_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 24, 226, 90, 141, 131, 80, 60, 37, 26, 122, 240, 113, 38, 253, 114, 0, 223,
    111, 196, 237, 82, 106, 156, 144, 0, 0, 0, 0,
]);

/// `Sysvar1nstructions1111111111111111111111111`.
pub const INSTRUCTIONS_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 24, 123, 209, 102, 53, 218, 212, 4, 85, 253, 194, 192, 193, 36, 198, 143, 33,
    86, 117, 165, 219, 186, 203, 95, 8, 0, 0, 0,
]);

/// `SysvarLastRestartS1ot1111111111111111111111`.
pub const LAST_RESTART_SLOT_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 25, 6, 221, 225, 205, 63, 148, 125, 202, 180, 200, 244, 244, 245, 27, 173, 15,
    152, 19, 184, 0, 210, 137, 71, 31, 192, 0, 0,
]);

/// `SysvarRecentB1ockHashes11111111111111111111`.
pub const RECENT_BLOCKHASHES_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 25, 44, 86, 142, 224, 138, 132, 95, 115, 210, 151, 136, 207, 3, 92, 49, 69,
    178, 26, 179, 68, 216, 6, 46, 169, 64, 0, 0,
]);

/// `SysvarRent111111111111111111111111111111111`.
pub const RENT_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 25, 44, 92, 81, 33, 140, 201, 76, 61, 74, 241, 127, 88, 218, 238, 8, 155, 161,
    253, 68, 227, 219, 217, 138, 0, 0, 0, 0,
]);

/// `SysvarRewards111111111111111111111111111111`.
pub const REWARDS_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 25, 44, 97, 55, 206, 224, 146, 217, 182, 146, 62, 225, 204, 214, 25, 3, 250,
    130, 184, 161, 97, 145, 87, 141, 128, 0, 0, 0,
]);

/// `SysvarS1otHashes111111111111111111111111111`.
pub const SLOT_HASHES_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 25, 47, 10, 175, 198, 242, 101, 227, 251, 119, 204, 122, 218, 130, 197, 41,
    208, 190, 59, 19, 110, 45, 0, 85, 32, 0, 0, 0,
]);

/// `SysvarS1otHistory11111111111111111111111111`.
pub const SLOT_HISTORY_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 25, 47, 10, 175, 200, 117, 226, 225, 132, 87, 124, 80, 105, 207, 200, 70, 73,
    227, 235, 146, 120, 47, 149, 141, 72, 0, 0, 0,
]);

/// `SysvarStakeHistory1111111111111111111111111`.
pub const STAKE_HISTORY_ADDRESS: Address = Address::from_bytes([
    6, 167, 213, 23, 25, 53, 132, 208, 254, 237, 155, 179, 67, 29, 19, 32, 107, 229, 68, 40, 27,
    87, 184, 86, 108, 197, 55, 95, 244, 0, 0, 0,
]);
