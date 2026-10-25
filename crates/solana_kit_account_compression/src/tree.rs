//! Checked account allocation arithmetic from the pinned concurrent-tree layout.
use solana_kit_core::codecs::Error;

const DEPTH_3: u32 = 3;
const DEPTH_5: u32 = 5;
const DEPTH_6: u32 = 6;
const DEPTH_7: u32 = 7;
const DEPTH_8: u32 = 8;
const DEPTH_9: u32 = 9;
const DEPTH_10: u32 = 10;
const DEPTH_11: u32 = 11;

const DEPTH_12: u32 = 12;
const DEPTH_13: u32 = 13;
const DEPTH_14: u32 = 14;
const DEPTH_15: u32 = 15;
const DEPTH_16: u32 = 16;
const DEPTH_17: u32 = 17;
const DEPTH_18: u32 = 18;
const DEPTH_19: u32 = 19;

const DEPTH_20: u32 = 20;
const DEPTH_24: u32 = 24;
const DEPTH_26: u32 = 26;
const DEPTH_30: u32 = 30;
const BUFFER_8: u32 = 8;
const BUFFER_16: u32 = 16;
const BUFFER_32: u32 = 32;
const BUFFER_64: u32 = 64;

const BUFFER_256: u32 = 256;
const BUFFER_512: u32 = 512;
const BUFFER_1024: u32 = 1024;
const BUFFER_2048: u32 = 2048;

const NODE_BYTES: usize = 32;
const INDEX_AND_PADDING_BYTES: usize = 8;
const TREE_COUNTER_BYTES: usize = 24;

/// Includes the account discriminant, V1 tag and the complete V1 header payload.
pub const HEADER_SIZE_V1: usize = 56;

/// Exactly the configurations admitted by the pinned program dispatch table.
pub const VALID_DEPTH_BUFFER_PAIRS: &[(u32, u32)] = &[
    (DEPTH_3, BUFFER_8),
    (DEPTH_5, BUFFER_8),
    (DEPTH_6, BUFFER_16),
    (DEPTH_7, BUFFER_16),
    (DEPTH_8, BUFFER_16),
    (DEPTH_9, BUFFER_16),
    (DEPTH_10, BUFFER_32),
    (DEPTH_11, BUFFER_32),
    (DEPTH_12, BUFFER_32),
    (DEPTH_13, BUFFER_32),
    (DEPTH_14, BUFFER_64),
    (DEPTH_14, BUFFER_256),
    (DEPTH_14, BUFFER_1024),
    (DEPTH_14, BUFFER_2048),
    (DEPTH_15, BUFFER_64),
    (DEPTH_16, BUFFER_64),
    (DEPTH_17, BUFFER_64),
    (DEPTH_18, BUFFER_64),
    (DEPTH_19, BUFFER_64),
    (DEPTH_20, BUFFER_64),
    (DEPTH_20, BUFFER_256),
    (DEPTH_20, BUFFER_1024),
    (DEPTH_20, BUFFER_2048),
    (DEPTH_24, BUFFER_64),
    (DEPTH_24, BUFFER_256),
    (DEPTH_24, BUFFER_512),
    (DEPTH_24, BUFFER_1024),
    (DEPTH_24, BUFFER_2048),
    (DEPTH_26, BUFFER_512),
    (DEPTH_26, BUFFER_1024),
    (DEPTH_26, BUFFER_2048),
    (DEPTH_30, BUFFER_512),
    (DEPTH_30, BUFFER_1024),
    (DEPTH_30, BUFFER_2048),
];

/// Checks a complete depth/buffer pair without accepting either dimension alone.
pub fn is_valid_depth_buffer_pair(depth: u32, buffer: u32) -> bool {
    VALID_DEPTH_BUFFER_PAIRS.contains(&(depth, buffer))
}

/// Returns header + sequence/index/buffer + changelogs + rightmost path + canopy.
/// All sizes use checked target-width arithmetic, including on 32-bit Wasm.
///
/// # Errors
/// Rejects unsupported pairs, canopy beyond the tree, or target-width overflow.
pub fn account_size(depth: u32, buffer: u32, canopy: u32) -> Result<usize, Error> {
    if !is_valid_depth_buffer_pair(depth, buffer) || canopy > depth {
        return Err(Error::InvalidValue);
    }

    let depth = usize::try_from(depth).map_err(|_| Error::Overflow)?;
    let buffer = usize::try_from(buffer).map_err(|_| Error::Overflow)?;
    let path = depth
        .checked_add(1)
        .and_then(|value| value.checked_mul(NODE_BYTES))
        .and_then(|value| value.checked_add(INDEX_AND_PADDING_BYTES))
        .ok_or(Error::Overflow)?;
    let tree = buffer
        .checked_add(1)
        .and_then(|value| value.checked_mul(path))
        .and_then(|value| value.checked_add(TREE_COUNTER_BYTES + HEADER_SIZE_V1))
        .ok_or(Error::Overflow)?;
    tree.checked_add(canopy_size(canopy)?)
        .ok_or(Error::Overflow)
}

/// Number of canopy bytes, excluding the root already stored in the tree.
///
/// # Errors
/// Rejects shifts, multiplication or conversion exceeding the target's usize.
pub fn canopy_size(depth: u32) -> Result<usize, Error> {
    let shift = depth.checked_add(1).ok_or(Error::Overflow)?;
    1usize
        .checked_shl(shift)
        .and_then(|nodes| nodes.checked_sub(2))
        .and_then(|nodes| nodes.checked_mul(NODE_BYTES))
        .ok_or(Error::Overflow)
}

/// Recovers an exact canopy depth, rejecting fractional/incomplete binary trees.
///
/// # Errors
/// Rejects non-node-aligned lengths, non-power-of-two trees and overflow.
pub fn canopy_depth(bytes: usize) -> Result<u32, Error> {
    if !bytes.is_multiple_of(NODE_BYTES) {
        return Err(Error::InvalidValue);
    }

    let nodes = (bytes / NODE_BYTES).checked_add(2).ok_or(Error::Overflow)?;

    if !nodes.is_power_of_two() {
        return Err(Error::InvalidValue);
    }

    nodes.ilog2().checked_sub(1).ok_or(Error::InvalidValue)
}
