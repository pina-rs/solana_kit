//! Native entry points use the same complete assertion bodies as portable conformance.

#[path = "attestation/assertions.rs"]
mod assertions;
#[cfg(feature = "crypto")]
#[path = "attestation/signatures.rs"]
mod signatures;
