#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod browser;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use browser::{BrowserExchange, BrowserHttp, ConfigError, HttpConfig};
