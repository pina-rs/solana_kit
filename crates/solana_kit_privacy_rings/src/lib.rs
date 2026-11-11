//! Solana Privacy Rings on Kit types.
//!
//! Deposit, confidential transfer, and withdraw build shielded-pool instructions
//! with [`solana_kit_core::Instruction`]. Indexer methods implement
//! [`solana_kit_client::methods::Method`] and run through `Client::call`. The
//! owner signs every spend; this crate has no signer and no HTTP stack.
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
#[cfg(feature = "crypto")]
mod pda;
mod rpc;
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
pub use error::Error;
#[cfg(feature = "crypto")]
pub use pda::{nullifier_account, sol_interface, spl_interface, state_tree, user_record};
pub use rpc::*;
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
