//! Maintained public-client behavior owners, also called directly by portable consumers.
#[path = "contracts/accounts.rs"]
mod accounts;
#[path = "contracts/instructions.rs"]
mod instructions;
#[cfg(feature = "pda")]
#[path = "contracts/pdas.rs"]
mod pdas;
#[path = "contracts/schema.rs"]
mod schema;
