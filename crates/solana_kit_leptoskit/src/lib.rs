#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "host", all(feature = "guest", target_arch = "wasm32")))]
mod wire;

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
mod rest_guest;
#[cfg(any(feature = "host", all(feature = "guest", target_arch = "wasm32")))]
mod rest_wire;
#[cfg(all(feature = "guest", target_arch = "wasm32"))]
pub use rest_guest::GuestRest;

#[cfg(any(feature = "host", all(feature = "guest", target_arch = "wasm32")))]
mod socket_wire;
#[cfg(any(feature = "host", all(feature = "guest", target_arch = "wasm32")))]
pub use socket_wire::SocketFailure;

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
mod socket_guest;
#[cfg(all(feature = "guest", target_arch = "wasm32"))]
pub use socket_guest::{GuestWebSocket, SocketCall, SocketState};

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
mod socket_session;
#[cfg(all(feature = "guest", target_arch = "wasm32"))]
pub use socket_session::GuestSubscriptionSocket;

#[cfg(feature = "host")]
mod socket_actor;
#[cfg(feature = "host")]
mod socket_driver;
#[cfg(feature = "host")]
pub use socket_driver::{SocketCompletion, SocketDriver, SocketLimits};

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
mod guest;
#[cfg(all(feature = "guest", target_arch = "wasm32"))]
pub use guest::{GuestExchange, GuestHttp};

#[cfg(all(feature = "wallet-platform", target_arch = "wasm32"))]
mod wallet_platform;
#[cfg(all(feature = "wallet-platform", target_arch = "wasm32"))]
pub use wallet_platform::{GuestWalletPlatform, WalletPlatformError};

#[cfg(any(feature = "wallet-session", feature = "wallet-host"))]
mod wallet_wire;

/// Native session service with one joined worker and one wallet authority.
#[cfg(feature = "wallet-host")]
pub mod wallet_service;

#[cfg(all(feature = "wallet-session", target_arch = "wasm32"))]
mod wallet_guest;
#[cfg(all(feature = "wallet-session", target_arch = "wasm32"))]
pub use wallet_guest::{GuestLocalWallet, GuestMobileWallet, WalletSessionError};

/// Optional accessible native wallet view and bounded local action channel.
#[cfg(feature = "wallet-ui")]
pub mod wallet_ui;

#[cfg(feature = "host")]
mod driver;
#[cfg(feature = "host")]
mod host;
#[cfg(feature = "host")]
pub use driver::{HostError, HostLimits, HttpCompletion, HttpDriver, Routed};
#[cfg(feature = "host")]
pub use host::HttpHost;
