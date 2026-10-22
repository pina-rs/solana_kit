#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod endpoint;
pub use endpoint::Endpoint;

#[cfg(all(target_os = "wasi", target_env = "p2"))]
mod reactor;
#[cfg(all(target_os = "wasi", target_env = "p2"))]
pub use reactor::{DriveError, Reactor};
#[cfg(all(target_os = "wasi", target_env = "p2"))]
mod http;
#[cfg(all(target_os = "wasi", target_env = "p2"))]
pub use http::{ConfigurationError, Exchange, WasiHttp};
