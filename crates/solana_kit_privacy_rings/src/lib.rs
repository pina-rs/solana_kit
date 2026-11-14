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
#[cfg(feature = "crypto")]
mod admission;
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
#[cfg(feature = "poseidon")]
mod witness;

mod transact;
#[cfg(feature = "crypto")]
mod viewing;

pub use address::*;
#[cfg(all(feature = "crypto", feature = "poseidon"))]
pub use admission::verify_note;
#[cfg(feature = "crypto")]
pub use admission::{AssetBalance, VerifiedNote, private_balances};
pub use deposit::{AssetDeposit, Deposit, DepositAsset, SplDeposit};
pub use encode::{
    AnonymousRingProof, Bsb22, CacheAccess, CacheWrite, Circuit, ConfidentialPlaintext, EddsaProof,
    Input, Message, OutputSlot as ShieldedOutput, OwnerTag, Proof, TransactBody, TreeContext,
    read_confidential, write_confidential,
};
pub use error::{Error, ProverFailure};
pub use history::{Page, collect_nullifier_queue, collect_pages, spent_nullifiers};
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
    ProverDestination, ProverEndpoint, TEE_VERSION, gpu_nonce, hex_decode, hex_encode,
    parse_prover_proof, proving_key_sha256, report_data, request_aad,
};
#[cfg(feature = "tee")]
pub use prover::{
    NitroMeasurement, NitroPolicy, OpenedRequest, SealedRequest, VerifiedNitro,
    open_prover_request, open_prover_response, seal_prover_request, verify_attestation,
    verify_nitro, verify_nitro_with,
};
pub use rpc::*;
#[cfg(feature = "crypto")]
pub use spend::{retain_non_inclusion, subtract_nullified};
pub use transact::{
    CacheWriter, ConfidentialTransfer, Settlement, SplDepositAccounts, SplWithdrawalAccounts,
    Transact, Withdraw,
};
#[cfg(feature = "crypto")]
pub use viewing::{OpenedNote, ProoflessNote, TrackedNote, ViewingKey};
#[cfg(feature = "poseidon")]
pub use witness::{
    IndexedLookup, IndexedTree, NotePreimage, OutputNote, SignedOwner, SpendNote, TransferCircuit,
    TransferWitness, TreeSlotWitness, confidential_transfer_witness, indexed_proof_request,
};

#[cfg(all(test, feature = "crypto", feature = "poseidon"))]
mod admission_behavior_tests;
#[cfg(test)]
mod admission_tests;
#[cfg(test)]
mod tests;
