//! Executor-neutral signing capabilities for off-chain application clients.
//!
//! Futures may retain local state. Native callers can require `Send` on each
//! associated future without imposing that constraint on browser or guest users.
//! This crate has no entropy, networking, persistence, or on-chain program API.
//!
//! Erasing a future into a local trait object discards its conditional `Send`.
//! Backend traits retain concrete associated futures. Heterogeneous composition
//! offers explicit [`local`] adapters and [`native`] adapters whose erased futures
//! retain `Send` under checked associated-future bounds.
//!
//! ```compile_fail
//! use core::{future::Future, pin::Pin};
//! fn requires_send<T: Send>(_: T) {}
//! let local: Pin<Box<dyn Future<Output = ()>>> = Box::pin(async {});
//! requires_send(local);
//! ```

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

mod capability;
mod composition;
pub mod local;
pub mod native;
mod pipeline;
mod snapshot;

use core::{fmt, future::Future};

use solana_kit_core::{Address, Signature, transaction::Transaction};

pub use capability::SignerCapability;
pub use composition::Role;
pub use pipeline::TransactionSigners;
pub use snapshot::SignatureOutput;

/// Address owned by a signer backend. Pipeline identity uses [`SignerCapability`].
pub trait Signer {
    /// Address whose signature slot this capability owns.
    fn address(&self) -> Address;
}

/// Signs exact bare bytes. This does not add an off-chain message envelope.
pub trait MessageSigner: Signer {
    /// Signing operation, which may retain non-`Send` state across suspension.
    type Sign<'a>: Future<Output = Result<SignatureOutput, Error>> + 'a
    where
        Self: 'a;

    /// Signs the exact supplied bytes without modifying them.
    fn sign_message<'a>(&'a self, bytes: &'a [u8]) -> Self::Sign<'a>;
}

/// Adds only this signer's signature to an immutable transaction snapshot.
pub trait TransactionPartialSigner: Signer {
    /// Signing operation without an executor or universal `Send` requirement.
    type Sign<'a>: Future<Output = Result<SignatureOutput, Error>> + 'a
    where
        Self: 'a;

    /// Returns the address, bytes, and signature actually authorized.
    fn sign_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Sign<'a>;
}

/// May replace a transaction before subsequent signers authorize its bytes.
pub trait TransactionModifyingSigner: Signer {
    /// Modification and signing operation.
    type Modify<'a>: Future<Output = Result<Transaction, Error>> + 'a
    where
        Self: 'a;

    /// Returns an immutable replacement, with no reused signature on changed bytes.
    fn modify_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Modify<'a>;
}

/// Explicit caller-granted submission capability.
pub trait TransactionSendingSigner: Signer {
    /// Submission operation. Its result is not confirmation of execution.
    type Send<'a>: Future<Output = Result<Signature, Error>> + 'a
    where
        Self: 'a;

    /// Authorizes and submits the supplied snapshot once under caller policy.
    fn send_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Send<'a>;
}

/// A capability failure or an invalid result at the signer boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// The user or backend rejected authorization.
    Rejected,
    /// The bounded signer or transaction batch limit was exceeded.
    TooManySigners,
    /// The caller or backend cancelled this operation.
    Cancelled,
    /// Different logical capabilities claim one address, or roles are incompatible.
    ConflictingSigner,
    /// A backend returned a signature owned by another address.
    ForeignSigner,
    /// A backend signed different bytes, or reused a signature after modification.
    ChangedSnapshot,
    /// No sending capability was supplied.
    MissingSender,
    /// More than one sending capability was supplied.
    MultipleSenders,
    /// Core rejected a malformed, missing, or invalid transaction signature.
    Transaction(solana_kit_core::transaction::Error),
}

impl From<solana_kit_core::transaction::Error> for Error {
    fn from(error: solana_kit_core::transaction::Error) -> Self {
        Self::Transaction(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "signer error: {self:?}")
    }
}

impl core::error::Error for Error {}

#[cfg(feature = "crypto")]
mod keypair;
