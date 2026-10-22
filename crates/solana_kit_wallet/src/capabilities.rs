use crate::{WalletAccount, WalletFeature, WalletFeatures};
use alloc::{boxed::Box, string::String, vec::Vec};
use core::{fmt, future::Future};

/// Standard authorization capability identifier.
pub const CONNECT: &str = "standard:connect";
/// Standard disconnect capability identifier.
pub const DISCONNECT: &str = "standard:disconnect";
/// Standard change events capability identifier.
pub const EVENTS: &str = "standard:events";

/// Stable wallet failure categories, matching the pinned contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WalletErrorCode {
    /// Wallet or account unavailable.
    Unavailable,
    /// Capability unsupported.
    UnsupportedFeature,
    /// User refused authorization or signing.
    UserRejected,
    /// Invalid active-wallet request.
    InvalidRequest,
    /// Malformed wallet response.
    InvalidResponse,
    /// Wallet disconnected during the request.
    Disconnected,
    /// Platform transport failed.
    Transport,
    /// Deadline exceeded.
    Timeout,
}

/// A wallet failure retaining a caller-chosen concrete cause.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletError<C = String> {
    /// Stable category.
    pub code: WalletErrorCode,
    /// Developer explanation.
    pub message: String,
    /// Original failure, when available.
    pub cause: Option<C>,
}

impl<C> fmt::Display for WalletError<C> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "WalletError({:?}, {})", self.code, self.message)
    }
}

impl<C: fmt::Debug> core::error::Error for WalletError<C> {}

/// Authorization options. Silent requests must avoid prompting the user.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ConnectInput {
    /// Whether the backend must avoid a user prompt.
    pub silent: bool,
}

/// Accounts actually authorized by the backend.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConnectOutput {
    /// Authorized accounts in backend order.
    pub accounts: Vec<WalletAccount>,
}

/// Authorization owned by a concrete backend, never simulated by the SDK.
pub trait Connect: WalletFeature {
    /// Concrete backend failure.
    type Error;
    /// Concrete operation, optionally Send depending on the implementation.
    type Future<'a>: Future<Output = Result<ConnectOutput, Self::Error>> + 'a
    where
        Self: 'a;
    /// Requests authorization. Dropping the future cancels caller interest.
    /// Backends must document any platform work that cannot be cancelled.
    fn connect(&self, input: ConnectInput) -> Self::Future<'_>;
}

/// Disconnects a backend without choosing an executor.
pub trait Disconnect: WalletFeature {
    /// Concrete backend failure.
    type Error;
    /// Concrete operation future.
    type Future<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;
    /// Releases the active application connection.
    fn disconnect(&self) -> Self::Future<'_>;
}

/// Changed properties. None means unchanged, while Some(empty) clears a value.
#[derive(Default)]
pub struct WalletChange {
    /// Newly advertised chains.
    pub chains: Option<Vec<String>>,
    /// Newly advertised capabilities.
    pub features: Option<WalletFeatures>,
    /// Newly authorized accounts.
    pub accounts: Option<Vec<WalletAccount>>,
}

/// Wallet property event capability. Implementations must invoke callbacks
/// outside mutable shared borrows to allow reentry and unsubscription.
pub trait Events: WalletFeature {
    /// Subscription owner. Dropping it removes the listener.
    type Subscription;
    /// Subscribes until the returned owner is dropped or explicitly removed.
    fn on_change(&self, listener: Box<dyn Fn(&WalletChange)>) -> Self::Subscription;
}
