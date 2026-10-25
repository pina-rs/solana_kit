#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod browser;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod cause;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use cause::BrowserCause;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use browser::{BrowserExchange, BrowserHttp, BrowserRest, ConfigError, HttpConfig};

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod websocket;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use websocket::{BrowserSubscriptionSocket, BrowserWebSocket, WsConfig, WsError};
