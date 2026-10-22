//! Run actual Chrome with joined Rust WS and rejecting WSS fixtures.
#[cfg(not(target_arch = "wasm32"))]
mod cases;
#[cfg(not(target_arch = "wasm32"))]
mod runner;
#[cfg(not(target_arch = "wasm32"))]
mod server;
#[cfg(not(target_arch = "wasm32"))]
mod tls;

#[cfg(target_arch = "wasm32")]
fn main() {}
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    runner::run()
}
