#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod attestation;
mod error;

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

#[cfg(feature = "crypto")]
pub use attestation::PublicationAttestation;
#[cfg(feature = "crypto")]
pub use policy::{
    ExpectedTransaction, PublicationPolicy, SigningRequest, ValidatedPreparedTransaction,
    validate_prepared_transaction,
};
