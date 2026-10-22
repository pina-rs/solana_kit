//! Actual Chrome WebSocket journeys against the owned Rust fixture.
#![forbid(unsafe_code)]
#[cfg(all(test, target_arch = "wasm32"))]
mod tests;
