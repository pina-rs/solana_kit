#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod attestation;
#[cfg(feature = "cli")]
pub mod cli;
mod error;
pub mod funding;

#[cfg(feature = "portal")]
mod deadline;

/// Bounded portal protocol with caller-owned destination, authentication and upload policy.
#[cfg(feature = "portal")]
pub mod portal;

/// Resumable ingestion and metadata hydration with explicit caller-owned I/O.
#[cfg(feature = "workflow")]
pub mod workflow;

#[cfg(feature = "crypto")]
mod policy;

pub use attestation::{AttestationMessage, RequestId};
pub use error::Error;
pub use funding::{
    DEFAULT_MAINNET_RPC_URL, FundingCheck, FundingError, MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS,
    check_signer_balance, format_sol_amount, resolve_funding_rpc_url,
};

#[cfg(feature = "crypto")]
pub use attestation::PublicationAttestation;
#[cfg(feature = "crypto")]
pub use policy::{
    ExpectedTransaction, PublicationPolicy, SigningRequest, ValidatedPreparedTransaction,
    validate_prepared_transaction,
};
