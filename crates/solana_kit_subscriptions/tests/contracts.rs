//! Independent public Subscriptions contract matrices, reusable on every portable host.
#[path = "contracts/accounts.rs"]
pub mod accounts;
#[path = "contracts/events.rs"]
pub mod events;
#[path = "contracts/instructions.rs"]
pub mod instructions;

#[path = "contracts/pdas.rs"]
pub mod pdas;
#[path = "contracts/schedule.rs"]
pub mod schedule;
#[path = "support/mod.rs"]
pub mod support;
