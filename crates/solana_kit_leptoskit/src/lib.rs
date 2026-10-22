#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "host", all(feature = "guest", target_arch = "wasm32")))]
mod wire;

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
mod guest;
#[cfg(all(feature = "guest", target_arch = "wasm32"))]
pub use guest::{GuestExchange, GuestHttp};

#[cfg(feature = "host")]
mod driver;
#[cfg(feature = "host")]
mod host;
#[cfg(feature = "host")]
pub use driver::{HostError, HostLimits, HttpCompletion, HttpDriver, Routed};
#[cfg(feature = "host")]
pub use host::HttpHost;
