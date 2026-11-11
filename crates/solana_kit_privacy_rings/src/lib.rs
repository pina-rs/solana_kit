//! Solana Privacy Rings on Kit types.
//!
//! Deposit, confidential transfer, and withdraw build shielded-pool instructions
//! with [`solana_kit_core::Instruction`]. Indexer methods implement
//! [`solana_kit_client::methods::Method`] and run through `Client::call`.
//! [`PrivacyProverClient`] posts a witness through the caller's REST transport.
//! The owner signs every spend. This crate has no signer and no HTTP stack.
//!
//! The wire format follows Helius `zolana` tag `v0.8.0` and the Privacy Rings
//! API reference (OpenAPI 0.51.2). It does not wrap the `zolana-client` crate,
//! which is built on Solana's own SDK types.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod address;
mod deposit;
mod encode;
mod error;
mod history;
#[cfg(feature = "crypto")]
mod pda;
#[cfg(feature = "poseidon")]
mod poseidon;
mod prover;
mod rpc;
#[cfg(feature = "crypto")]
mod spend;

mod transact;
#[cfg(feature = "crypto")]
mod viewing;

pub use address::*;
pub use deposit::{AssetDeposit, Deposit, DepositAsset, SplDeposit};
pub use encode::{
    AnonymousRingProof, Bsb22, CacheAccess, CacheWrite, Circuit, ConfidentialPlaintext, EddsaProof,
    Input, Message, OutputSlot as ShieldedOutput, OwnerTag, Proof, TransactBody, TreeContext,
    read_confidential, write_confidential,
};
pub use error::{Error, ProverFailure};
pub use history::{Page, collect_pages, spent_nullifiers};
#[cfg(feature = "crypto")]
pub use pda::{nullifier_account, sol_interface, spl_interface, state_tree, user_record};
#[cfg(feature = "poseidon")]
pub use poseidon::{
    UtxoPreimage, derive_output_blinding_seed, derive_private_tx_blinding,
    derive_transact_output_blinding, nullifier, nullifier_pubkey, owner_hash, poseidon_hash,
    right_align, solana_owner_identity, utxo_commitment,
};
pub use prover::{
    Delivery, HEADER_CIPHERTEXT, HEADER_ENC, HEADER_VERSION, IndexedCircuit, PrivacyProverClient,
    ProverDestination, ProverEndpoint, TEE_VERSION, gpu_nonce, hex_encode, parse_prover_proof,
    proving_key_sha256, report_data, request_aad,
};
pub use rpc::*;
#[cfg(feature = "crypto")]
pub use spend::{retain_non_inclusion, subtract_nullified};
pub use transact::{
    CacheWriter, ConfidentialTransfer, Settlement, SplDepositAccounts, SplWithdrawalAccounts,
    Transact, Withdraw,
};
#[cfg(feature = "crypto")]
pub use viewing::{
    AssetBalance, OpenedNote, ProoflessNote, TrackedNote, ViewingKey, private_balances,
};

#[cfg(test)]
mod tests;
