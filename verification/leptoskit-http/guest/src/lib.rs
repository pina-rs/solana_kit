//! Real interpreted off-chain client fixture, with an application-owned executor.
#[cfg(target_arch = "wasm32")]
mod application;
#[cfg(target_arch = "wasm32")]
mod journey;
