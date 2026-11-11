//! Published program addresses and the Helius RPC paths that serve them.
//!
//! Addresses are the v0.8.0 constants in `helius-labs/zolana` `program-libs/interface`
//! and `program-libs/user-registry-interface`. The state tree string is the account
//! named by the Privacy Rings API reference and the addresses guide.

use solana_kit_core::Address;

use crate::Error;

/// Shielded-pool program. Mainnet and devnet use the same address.
///
/// Source: `SHIELDED_POOL_PROGRAM_ID` and
/// <https://www.helius.dev/docs/privacy/addresses>.
pub const SHIELDED_POOL_PROGRAM: Address = Address::from_bytes([
    12, 188, 181, 120, 174, 97, 37, 129, 50, 238, 175, 106, 193, 33, 192, 164, 43, 136, 162, 232,
    79, 177, 30, 126, 45, 91, 160, 175, 209, 39, 49, 99,
]);

/// User registry program. Source: `USER_REGISTRY_PROGRAM_ID`.
pub const USER_REGISTRY_PROGRAM: Address = Address::from_bytes([
    12, 184, 19, 39, 34, 169, 197, 80, 173, 230, 213, 235, 238, 72, 165, 105, 147, 8, 241, 138, 37,
    86, 197, 220, 174, 68, 119, 97, 14, 189, 93, 23,
]);

/// Native SOL custody PDA (`find_program_address([b"sol_interface", &[0]])`).
///
/// Source: `SOL_INTERFACE` and the addresses guide. The canonical bump is 248.
pub const SOL_INTERFACE: Address = Address::from_bytes([
    174, 9, 251, 185, 56, 68, 80, 105, 187, 159, 161, 65, 25, 120, 15, 51, 227, 171, 191, 213, 18,
    39, 39, 20, 57, 237, 147, 228, 185, 57, 125, 253,
]);

/// Canonical bump for [`SOL_INTERFACE`].
pub const SOL_INTERFACE_BUMP: u8 = 248;

/// Shielded-pool CPI authority PDA (`find_program_address([b"cpi_authority"])`).
///
/// Source: `SHIELDED_POOL_CPI_AUTHORITY`. The canonical bump is 250.
pub const CPI_AUTHORITY: Address = Address::from_bytes([
    171, 251, 122, 72, 2, 39, 82, 91, 136, 185, 82, 1, 239, 236, 213, 162, 38, 77, 248, 238, 242,
    103, 137, 104, 67, 64, 244, 78, 118, 136, 230, 59,
]);

/// Canonical bump for [`CPI_AUTHORITY`].
pub const CPI_AUTHORITY_BUMP: u8 = 250;

/// Deployed state Merkle tree named by the API reference examples.
///
/// `outputContext.tree` in the November 2026 mainnet and devnet fixtures, and
/// `treeAccount` for `getMerkleProofs`. Tree id 0 derives this account.
pub const STATE_TREE: Address = Address::from_bytes([
    90, 183, 83, 41, 174, 69, 177, 184, 231, 141, 26, 139, 137, 114, 248, 60, 242, 154, 182, 19,
    163, 243, 15, 71, 75, 98, 7, 100, 193, 141, 247, 113,
]);

/// `deposit` instruction tag. Source: `tag::DEPOSIT` in zolana v0.8.0.
pub const DEPOSIT_TAG: u8 = 15;

/// `transact` instruction tag. Source: `tag::TRANSACT`. Confidential transfers
/// and withdrawals are both this instruction; the settlement legs differ.
pub const TRANSACT_TAG: u8 = 16;

/// At most five settlement groups in one deposit. Source: `MAX_DEPOSIT_ASSETS`.
pub const MAX_DEPOSIT_ASSETS: usize = 5;

/// At most two input trees on one `transact`. Source: `MAX_INPUT_TREES`.
pub const MAX_INPUT_TREES: usize = 2;

/// At most 32 public settlement legs. Source: `MAX_INTERFACE_TRANSFERS`.
pub const MAX_INTERFACE_TRANSFERS: usize = 32;

/// At most 16 outputs. Source: `MAX_OUTPUTS`.
pub const MAX_OUTPUTS: usize = 16;

/// Cache write slots carried by a cached circuit selector. Source: `MAX_CACHE_WRITES`.
pub const MAX_CACHE_WRITES: usize = 16;

/// Seed for [`SOL_INTERFACE`].
pub const SOL_INTERFACE_SEED: &[u8] = b"sol_interface";

/// Seed for an SPL interface PDA. The historical bytes are part of the deployment.
pub const SPL_INTERFACE_SEED: &[u8] = b"spl_asset_vault";

/// Seed for a state tree PDA.
pub const TREE_SEED: &[u8] = b"tree";

/// Seed for a nullifier PDA.
pub const NULLIFIER_SEED: &[u8] = b"nullifier";

/// Seed for a user-registry record PDA.
pub const USER_RECORD_SEED: &[u8] = b"zolana/registry/v0";

/// Domain label for deposit blinding. Source: `DEPOSIT_BLINDING_DOMAIN`.
pub const DEPOSIT_BLINDING_DOMAIN: &[u8] = b"Deposit";

/// Poseidon asset field for native SOL, big-endian. Source: `SOL_ASSET_FIELD`.
pub const SOL_ASSET_FIELD: [u8; 32] = [
    0x20, 0x98, 0xf5, 0xfb, 0x9e, 0x23, 0x9e, 0xab, 0x3c, 0xea, 0xc3, 0xf2, 0x7b, 0x81, 0xe4, 0x81,
    0xdc, 0x31, 0x24, 0xd5, 0x5f, 0xfe, 0xd5, 0x23, 0xa8, 0x39, 0xee, 0x84, 0x46, 0xb6, 0x48, 0x64,
];

/// Leading tag on a 33-byte X25519 viewing key: `0x04 || u`.
pub const X25519_KEY_TAG: u8 = 4;

/// Which Helius cluster hosts the shared RPC, indexer, and prover URL.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cluster {
    /// `https://mainnet.helius-rpc.com`.
    Mainnet,
    /// `https://devnet.helius-rpc.com`.
    Devnet,
}

impl Cluster {
    /// RPC origin without a path, query, or API key.
    #[must_use]
    pub const fn rpc_origin(self) -> &'static str {
        match self {
            Self::Mainnet => "https://mainnet.helius-rpc.com",
            Self::Devnet => "https://devnet.helius-rpc.com",
        }
    }

    /// Indexer URL for one Privacy Rings method.
    ///
    /// The path's last segment is the JSON-RPC method name. The application
    /// appends its API key. This function never reads the environment.
    ///
    /// # Errors
    /// Rejects an empty name or any character outside ASCII letters and digits.
    pub fn indexer_url(self, method: &str) -> Result<String, Error> {
        if method.is_empty() || !method.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
            return Err(Error::Limit("privacy RPC method name"));
        }

        Ok(format!("{}/v1/spr/{method}", self.rpc_origin()))
    }

    /// Prover base URL. Proof calls append `/prove/{circuit}`.
    ///
    /// The string has no query and no API key. A Helius RPC root lends the
    /// prover its `/v1/spr` path, matching `resolveClientEndpoints`.
    #[must_use]
    pub fn prover_url(self) -> String {
        format!("{}/v1/spr", self.rpc_origin())
    }
}

/// Local Photon indexer path used by the Helius local stack.
///
/// # Errors
/// Rejects an empty name or any character outside ASCII letters and digits.
pub fn local_indexer_url(method: &str) -> Result<String, Error> {
    if method.is_empty() || !method.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        return Err(Error::Limit("privacy RPC method name"));
    }

    Ok(format!("http://127.0.0.1:8784/{method}"))
}

/// Local prover used by the Helius local stack.
///
/// The published prover binary listens here when started with
/// `--prover-address 127.0.0.1:3001`. Loopback does not need
/// `allow_unauthenticated_remote_prover`.
#[must_use]
pub fn local_prover_url() -> &'static str {
    "http://127.0.0.1:3001"
}

/// SHA-256 deposit blinding with the leading byte cleared.
///
/// Source: `deposit_blinding` in zolana v0.8.0. The pinned vector is shared
/// with the TypeScript client.
#[must_use]
pub fn deposit_blinding(tree: &[u8; 32], leaf_index: u64) -> [u8; 32] {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(DEPOSIT_BLINDING_DOMAIN);
    hasher.update(tree);
    hasher.update(leaf_index.to_be_bytes());
    let mut digest = hasher.finalize();
    digest[0] = 0;
    digest.into()
}

#[cfg(test)]
mod tests {
    use super::local_indexer_url;

    #[test]
    fn local_indexer_rejects_a_punctuated_method() {
        assert!(local_indexer_url("").is_err());
        assert!(local_indexer_url("get/user").is_err());
    }
}
