//! Bounded sparse tree operations never allocate all2^depth leaves.
use crate::{Node, bubblegum_hash};
use alloc::vec::Vec;
use solana_kit_core::codecs::{Error, Limits};

const MAX_TREE_DEPTH: usize = u32::BITS as usize;
const NODE_BYTES: usize = core::mem::size_of::<Node>();

/// Computes zero-leaf subtree roots iteratively with a bounded depth.
/// # Errors
/// Rejects depths beyond the program's u32 leaf-index domain.
pub fn compute_empty_node(level: u8) -> Result<Node, Error> {
    if usize::from(level) > MAX_TREE_DEPTH {
        return Err(Error::InvalidValue);
    }

    let mut node = [0; 32];

    for _ in 0..level {
        node = bubblegum_hash(&[&node, &node]);
    }

    Ok(node)
}

/// Verifies an exact path and rejects index aliases above the path depth.
#[must_use]
pub fn verify_proof(root: Node, leaf: Node, index: u32, proof: &[Node]) -> bool {
    if proof.len() > MAX_TREE_DEPTH
        || (proof.len() < MAX_TREE_DEPTH && u64::from(index) >= (1u64 << proof.len()))
    {
        return false;
    }

    let mut node = leaf;

    for (level, sibling) in proof.iter().enumerate() {
        node = if (index >> level) & 1 == 0 {
            bubblegum_hash(&[&node, sibling])
        } else {
            bubblegum_hash(&[sibling, &node])
        };
    }

    node == root
}

/// Sparse left-packed leaves with zero padding. Excess leaves are rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerkleTree {
    depth: u8,
    levels: Vec<Vec<Node>>,
    empty: Vec<Node>,
}

impl MerkleTree {
    /// Constructs retained levels within one shared element and byte budget.
    /// # Errors
    /// Rejects invalid depth/capacity, exhausted budgets and allocation failures.
    pub fn new(leaves: &[Node], depth: u8, limits: Limits) -> Result<Self, Error> {
        if usize::from(depth) > MAX_TREE_DEPTH
            || u64::try_from(leaves.len()).map_err(|_| Error::Overflow)? > (1u64 << depth)
        {
            return Err(Error::InvalidValue);
        }

        let mut budget = limits.max_items.min(limits.max_bytes / NODE_BYTES);
        let mut levels = Vec::new();
        let mut empty = Vec::new();
        levels
            .try_reserve_exact(usize::from(depth) + 1)
            .map_err(|_| Error::AllocationFailed)?;
        empty
            .try_reserve_exact(usize::from(depth) + 1)
            .map_err(|_| Error::AllocationFailed)?;
        claim(usize::from(depth) + 1, &mut budget, limits.max_bytes)?;
        let mut current = copied_nodes(leaves, &mut budget, limits.max_bytes)?;
        let mut zero = [0; 32];

        for _ in 0..depth {
            empty.push(zero);
            let mut parents = Vec::new();
            let length = current.len().div_ceil(2);
            claim(length, &mut budget, limits.max_bytes)?;
            parents
                .try_reserve_exact(length)
                .map_err(|_| Error::AllocationFailed)?;

            for pair in current.chunks(2) {
                parents.push(bubblegum_hash(&[&pair[0], pair.get(1).unwrap_or(&zero)]));
            }

            levels.push(current);
            current = parents;
            zero = bubblegum_hash(&[&zero, &zero]);
        }

        empty.push(zero);
        levels.push(current);
        Ok(Self {
            depth,
            levels,
            empty,
        })
    }

    /// Returns the committed root, including a completely empty tree.
    #[must_use]
    pub fn root(&self) -> Node {
        self.levels
            .last()
            .and_then(|level| level.first())
            .copied()
            .unwrap_or(self.empty[usize::from(self.depth)])
    }

    /// Produces a sibling path for an existing or zero-padded leaf.
    /// # Errors
    /// Rejects out-of-capacity indices, budgets and allocation failures.
    pub fn proof(&self, index: u32, limits: Limits) -> Result<Vec<Node>, Error> {
        if u64::from(index) >= (1u64 << self.depth) {
            return Err(Error::InvalidValue);
        }

        let mut budget = limits.max_items;
        claim(usize::from(self.depth), &mut budget, limits.max_bytes)?;
        let mut proof = Vec::new();
        proof
            .try_reserve_exact(usize::from(self.depth))
            .map_err(|_| Error::AllocationFailed)?;
        let mut position = u64::from(index);

        for level in 0..usize::from(self.depth) {
            let sibling = usize::try_from(position ^ 1).map_err(|_| Error::Overflow)?;
            proof.push(
                self.levels[level]
                    .get(sibling)
                    .copied()
                    .unwrap_or(self.empty[level]),
            );
            position >>= 1;
        }

        Ok(proof)
    }
}

fn claim(count: usize, budget: &mut usize, max_bytes: usize) -> Result<(), Error> {
    *budget = budget.checked_sub(count).ok_or(Error::LimitExceeded)?;

    if count.checked_mul(NODE_BYTES).ok_or(Error::Overflow)? > max_bytes {
        return Err(Error::LimitExceeded);
    }

    Ok(())
}

fn copied_nodes(nodes: &[Node], budget: &mut usize, max_bytes: usize) -> Result<Vec<Node>, Error> {
    claim(nodes.len(), budget, max_bytes)?;
    let mut copied = Vec::new();
    copied
        .try_reserve_exact(nodes.len())
        .map_err(|_| Error::AllocationFailed)?;
    copied.extend_from_slice(nodes);
    Ok(copied)
}
