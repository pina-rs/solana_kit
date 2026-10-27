//! Pinned receiver identities and instruction discriminators for off-chain clients.
use solana_kit_core::Address;

/// Pyth Solana Receiver, `rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ`.
pub const PYTH_SOLANA_RECEIVER_PROGRAM: Address = Address::from_bytes([
    12, 183, 250, 187, 82, 247, 166, 72, 187, 91, 49, 125, 154, 1, 139, 144, 87, 203, 2, 71, 116,
    250, 254, 1, 230, 196, 223, 152, 204, 56, 88, 129,
]);
/// Default receiver Wormhole bridge, `HDwcJBJXjL9FpJ7UBsYBtaDjsBUhuLCUYoz3zr8SWWaQ`.
pub const PYTH_WORMHOLE_PROGRAM: Address = Address::from_bytes([
    241, 11, 180, 229, 13, 86, 253, 161, 61, 254, 31, 50, 155, 141, 57, 61, 210, 74, 1, 69, 145,
    225, 131, 22, 151, 148, 13, 124, 52, 163, 141, 221,
]);
/// Push oracle, `pythWSnswVUd12oZpeFP8e9CVaEqJg25g1Vtc2biRsT`.
pub const PYTH_PUSH_ORACLE_PROGRAM: Address = Address::from_bytes([
    12, 74, 160, 18, 142, 149, 211, 225, 98, 42, 165, 1, 197, 133, 169, 235, 7, 179, 115, 84, 193,
    8, 234, 11, 121, 27, 69, 109, 199, 238, 163, 54,
]);
/// Canonical System program used by receiver account initialization.
pub const SYSTEM_PROGRAM: Address = Address::from_bytes([0; 32]);
/// First eight SHA256 bytes of `global:post_update_atomic`.
pub const POST_UPDATE_ATOMIC_DISCRIMINATOR: [u8; 8] =
    [0x31, 0xac, 0x54, 0xc0, 0xaf, 0xb4, 0x34, 0xea];
/// First eight SHA256 bytes of `global:post_update`.
pub const POST_UPDATE_DISCRIMINATOR: [u8; 8] = [0x85, 0x5f, 0xcf, 0xaf, 0x0b, 0x4f, 0x76, 0x2c];
/// Default receiver treasury, matching the pinned client.
pub const DEFAULT_TREASURY_ID: u8 = 0;
/// Receiver configuration seed.
pub const CONFIG_PDA_SEED: &[u8] = b"config";
/// Receiver treasury seed; followed by one treasury byte.
pub const TREASURY_PDA_SEED: &[u8] = b"treasury";
/// Wormhole guardian-set seed; followed by the big-endian index.
pub const GUARDIAN_SET_PDA_SEED: &[u8] = b"GuardianSet";

/// Selects a treasury using exactly one caller-supplied random byte.
/// The provider owns entropy and must fill the byte uniformly. No ambient RNG,
/// platform import or global state is used; provider failures propagate unchanged.
pub fn random_treasury_id<E>(fill: impl FnOnce(&mut [u8; 1]) -> Result<(), E>) -> Result<u8, E> {
    let mut byte = [0];
    fill(&mut byte)?;
    Ok(byte[0])
}
