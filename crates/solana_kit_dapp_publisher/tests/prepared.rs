#![cfg(feature = "crypto")]
//! Native entry points use the same complete assertion bodies as portable conformance.
extern crate alloc;

#[path = "prepared/assertions.rs"]
mod assertions;
#[path = "support/checks.rs"]
mod checks;
