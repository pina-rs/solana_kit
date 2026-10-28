//! Well-known SNS addresses.

use solana_kit_core::Address;

/// SPL Name Service program.
pub const NAME_PROGRAM: Address = Address::from_bytes([
    11, 173, 81, 244, 19, 193, 243, 169, 148, 96, 217, 0, 216, 191, 46, 214, 146, 126, 202, 52,
    215, 183, 132, 43, 248, 16, 169, 115, 8, 45, 30, 220,
]);
/// Root domain account.
pub const SNS_ROOT_DOMAIN: Address = Address::from_bytes([
    61, 83, 194, 75, 56, 54, 14, 211, 129, 58, 35, 223, 178, 223, 216, 32, 171, 88, 33, 203, 121,
    41, 163, 141, 46, 170, 178, 82, 232, 56, 37, 149,
]);
/// Registrar program.
pub const SNS_REGISTRAR_PROGRAM: Address = Address::from_bytes([
    10, 207, 144, 136, 108, 148, 104, 166, 24, 48, 41, 246, 133, 100, 197, 101, 55, 17, 85, 23, 9,
    61, 75, 155, 71, 17, 74, 191, 101, 47, 185, 80,
]);
/// Records program.
pub const SNS_RECORDS_PROGRAM: Address = Address::from_bytes([
    243, 96, 67, 217, 255, 96, 154, 213, 150, 32, 207, 123, 211, 15, 67, 83, 236, 6, 129, 182, 188,
    147, 95, 211, 149, 236, 37, 100, 157, 16, 118, 220,
]);
/// Class for V2 records.
pub const CENTRAL_STATE_SNS_RECORDS: Address = Address::from_bytes([
    26, 253, 126, 22, 60, 117, 251, 235, 169, 206, 162, 129, 201, 20, 44, 12, 197, 46, 140, 248,
    228, 201, 213, 102, 236, 186, 150, 127, 18, 19, 242, 170,
]);
/// Class for reverse lookup accounts.
pub const REVERSE_LOOKUP_CLASS: Address = Address::from_bytes([
    30, 108, 88, 228, 177, 181, 74, 40, 91, 207, 20, 62, 230, 167, 136, 53, 110, 250, 134, 226, 89,
    243, 216, 171, 22, 106, 244, 14, 246, 43, 73, 142,
]);
/// Name tokenizer program.
pub const NAME_TOKENIZER_PROGRAM: Address = Address::from_bytes([
    11, 179, 25, 247, 166, 167, 29, 182, 28, 137, 181, 33, 75, 66, 0, 194, 145, 84, 78, 39, 168,
    99, 97, 254, 230, 223, 9, 233, 47, 115, 104, 119,
]);
/// Name offers program.
pub const NAME_OFFERS_PROGRAM: Address = Address::from_bytes([
    105, 54, 188, 200, 249, 7, 188, 129, 68, 243, 130, 6, 16, 28, 236, 40, 70, 124, 36, 199, 62,
    235, 53, 152, 156, 225, 210, 187, 94, 40, 88, 118,
]);
/// Twitter verification authority.
pub const TWITTER_VERIFICATION_AUTHORITY: Address = Address::from_bytes([
    221, 176, 150, 226, 178, 26, 233, 71, 18, 66, 112, 225, 78, 166, 133, 197, 196, 188, 11, 165,
    143, 161, 30, 170, 218, 56, 5, 179, 83, 83, 195, 5,
]);
/// Twitter root parent registry.
pub const TWITTER_ROOT_PARENT_REGISTRY: Address = Address::from_bytes([
    52, 172, 97, 107, 243, 249, 137, 157, 148, 102, 27, 83, 89, 194, 12, 167, 181, 202, 202, 110,
    185, 21, 17, 221, 191, 66, 133, 27, 222, 109, 170, 93,
]);
