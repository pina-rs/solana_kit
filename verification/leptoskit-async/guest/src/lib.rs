//! Private off-chain client fixture; only the actual LeptosKit host supplies imports.

#[cfg(target_arch = "wasm32")]
mod application;

#[cfg(target_arch = "wasm32")]
mod capabilities;

#[cfg(target_arch = "wasm32")]
mod journey;
