//! Browser Wallet Standard discovery and application-owned local sessions.
//!
//! This module requires a Window and never installs a global singleton. Drop or
//! close discovery to remove its listener and registrations. Wallet promises may
//! continue after caller cancellation; their stale results cannot authorize a
//! replaced session. This is off-chain application code, never on-chain code.

mod controller;
mod discovery;
mod interop;
mod metadata;
mod operation;
mod panel;
mod session;
mod signer;

pub use controller::{
    Connection, ControllerSubscription, WalletController, WalletFuture, WalletState,
};
pub use discovery::WalletDiscovery;
pub use interop::WalletFailure;
pub use panel::BrowserWalletPanel;
pub use session::{BrowserFeature, BrowserWallet};
pub use signer::WalletSigner;
