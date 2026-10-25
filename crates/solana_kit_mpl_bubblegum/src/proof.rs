//! Caller-owned proof nodes are never fetched implicitly or granted signer privileges.
use alloc::vec::Vec;
use solana_kit_core::{
    AccountMeta, AccountRole, Address,
    codecs::{Error, Limits},
};

/// Hash nodes supplied by a DAS provider or the caller's own tree.
pub type Node = [u8; 32];

const MAX_TREE_DEPTH: usize = u32::BITS as usize;
const NODE_BYTES: usize = core::mem::size_of::<Node>();

/// A checked binding between the tree, leaf index and bounded proof path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetProof {
    /// Merkle tree account.
    pub tree: Address,
    /// Root the proof commits to.
    pub root: Node,
    /// Leaf digest.
    pub leaf: Node,
    /// Zero-based leaf index, distinct from a DAS heap node index.
    pub index: u32,
    /// Siblings ordered from the leaf towards the root.
    pub siblings: Vec<Node>,
}

impl AssetProof {
    /// Removes a caller-selected canopy suffix after checking the complete depth.
    /// # Errors
    /// Rejects depth/index aliases, invalid canopy and caller budget exhaustion.
    pub fn remaining_accounts(
        &self,
        depth: u8,
        canopy_depth: u8,
        limits: Limits,
    ) -> Result<Vec<AccountMeta>, Error> {
        if usize::from(depth) > MAX_TREE_DEPTH
            || canopy_depth > depth
            || self.siblings.len() != usize::from(depth)
        {
            return Err(Error::InvalidValue);
        }

        if usize::from(depth) < MAX_TREE_DEPTH && u64::from(self.index) >= (1u64 << depth) {
            return Err(Error::InvalidValue);
        }

        let count = usize::from(depth - canopy_depth);

        if count > limits.max_items
            || count.checked_mul(NODE_BYTES).ok_or(Error::Overflow)? > limits.max_bytes
        {
            return Err(Error::LimitExceeded);
        }

        let mut accounts = Vec::new();
        accounts
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        accounts.extend(self.siblings[..count].iter().map(|node| AccountMeta {
            address: Address::from_bytes(*node),
            role: AccountRole::Readonly,
        }));
        Ok(accounts)
    }
}
