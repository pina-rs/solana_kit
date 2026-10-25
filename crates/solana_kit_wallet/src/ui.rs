//! View snapshots and actions. Authorization stays in the selected wallet owner.
use crate::{WalletAccount, WalletErrorCode};
use alloc::{string::String, vec::Vec};

/// Opaque, non-reused identity selected by an application controller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChoiceKey(pub u64);

/// Current discoverable wallet metadata, with an explicit availability decision.
#[derive(Clone, Debug)]
pub struct WalletChoice {
    /// Exact identity resolved again by the controller before effects.
    pub key: ChoiceKey,
    /// Owned display name from the current wallet advertisement.
    pub name: String,
    /// Whether this wallet supports the selected chain and required capabilities.
    pub enabled: bool,
}

/// Visual state projected from the actual authorization owner.
#[derive(Clone)]
pub enum PanelState {
    /// No wallet is connected.
    Disconnected,
    /// Local authority is removed while the wallet's disconnect remains pending.
    Disconnecting,
    /// The named wallet is awaiting authorization.
    Connecting(String),
    /// The owner currently authorizes this account.
    Connected {
        /// Selected wallet's current display name.
        wallet: String,
        /// Exact selected account snapshot.
        account: WalletAccount,
        /// A signing or disconnect operation is outstanding.
        operating: bool,
    },
    /// The owner has been disposed and cannot accept another operation.
    Closed,
}

/// Complete wallet view data. It never owns private keys, futures or a runtime.
#[derive(Clone)]
pub struct WalletPanel {
    /// Current visual connection state.
    pub state: PanelState,
    /// Current discoverable choices, in registry order.
    pub wallets: Vec<WalletChoice>,
    /// Authorized account choices, with controller-owned identities.
    pub accounts: Vec<(ChoiceKey, WalletAccount)>,
    /// Whether the picker is open. This is presentation, not authorization.
    pub picker_open: bool,
    /// Last redacted failure category. The controller retains its original cause.
    pub error: Option<WalletErrorCode>,
}

/// User intent delivered to the owning local controller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PanelAction {
    /// Rebuild from current backend metadata; this carries no authority.
    Refresh,
    /// Open the discoverable-wallet picker.
    OpenPicker,
    /// Close the picker without changing authorization.
    ClosePicker,
    /// Connect an exact currently registered wallet.
    Connect(ChoiceKey),
    /// Select an exact currently authorized account.
    SelectAccount(ChoiceKey),
    /// Disconnect the current wallet.
    Disconnect,
    /// Cancel local caller interest; this cannot undo a remote side effect.
    Cancel,
    /// Clear the visible failure without changing authorization.
    ClearError,
}
