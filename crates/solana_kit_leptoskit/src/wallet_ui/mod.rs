//! Native button semantics and bounded actions, independent of the wallet executor.
mod commands;
#[cfg(feature = "wallet-session")]
mod local;
#[cfg(feature = "wallet-session")]
pub use local::{
    LocalQueueError, LocalSigningReview, LocalWalletAction, LocalWalletCommands, LocalWalletEvents,
    local_signing_approval, local_wallet_channel, local_wallet_panel, mobile_wallet_panel,
};
#[cfg(all(test, feature = "wallet-session"))]
mod local_tests;
#[cfg(test)]
mod tests;
pub use commands::{PanelCommands, PanelEvents, PanelQueueError, panel_channel};

#[cfg(all(
    feature = "wallet-browser-ui",
    target_arch = "wasm32",
    target_os = "unknown"
))]
mod browser;
#[cfg(all(
    feature = "wallet-browser-ui",
    target_arch = "wasm32",
    target_os = "unknown"
))]
pub use browser::{BrowserWalletUpdate, BrowserWalletView, BrowserWalletViewError};

use leptoskit_native::{Element, EventOutcome, Live, NativeEventSlot, NativeKind};
use solana_kit_wallet::ui::{PanelAction, PanelState, WalletPanel};

/// Builds a responsive, accessible wallet picker and current-authorization panel.
/// The app awaits PanelEvents on its local executor, dispatches through the actual
/// wallet controller, and rebuilds this snapshot after controller notifications.
pub fn wallet_panel(model: &WalletPanel, commands: &PanelCommands) -> Element {
    let busy = matches!(
        model.state,
        PanelState::Connecting(_)
            | PanelState::Disconnecting
            | PanelState::Connected {
                operating: true,
                ..
            }
    );
    let mut view = Element::new(NativeKind::View)
        .class("w-full max-w-[560px] flex-col p-4 gap-y-3")
        .id("wallet_ui.panel")
        .aria_label("Solana wallet")
        .aria_busy(busy);
    let status = status(&model.state);
    view = view.child(
        Element::text(status)
            .id("wallet_ui.status")
            .aria_live(Live::Polite),
    );

    if let Some(error) = model.error {
        view = view
            .child(
                Element::text(format!("Wallet request: {error:?}"))
                    .id("wallet_ui.error")
                    .aria_live(Live::Assertive),
            )
            .child(button(
                "Dismiss error",
                "wallet_ui.dismiss_error",
                true,
                PanelAction::ClearError,
                commands,
            ));
    }

    if matches!(model.state, PanelState::Closed) {
        return view;
    }

    if busy {
        return view.child(button(
            "Cancel request",
            "wallet_ui.cancel",
            true,
            PanelAction::Cancel,
            commands,
        ));
    }

    controls(view, model, commands)
}

fn status(state: &PanelState) -> String {
    match state {
        PanelState::Disconnected => "Choose a wallet to connect".into(),
        PanelState::Disconnecting => "Disconnecting wallet".into(),
        PanelState::Connecting(name) => format!("Connecting to {name}"),
        PanelState::Connected {
            wallet, account, ..
        } => format!("{wallet}: {}", account.address()),
        PanelState::Closed => "Wallet controls closed".into(),
    }
}

fn controls(mut view: Element, model: &WalletPanel, commands: &PanelCommands) -> Element {
    view = match &model.state {
        PanelState::Connected { .. } => view.child(button(
            "Disconnect wallet",
            "wallet_ui.disconnect_button",
            true,
            PanelAction::Disconnect,
            commands,
        )),
        _ => view.child(button(
            "Connect wallet",
            "wallet_ui.connect_button",
            true,
            PanelAction::OpenPicker,
            commands,
        )),
    };

    for (key, account) in &model.accounts {
        view = view.child(button(
            account.address(),
            &format!("wallet_ui.account.{}", key.0),
            true,
            PanelAction::SelectAccount(*key),
            commands,
        ));
    }

    if model.picker_open {
        view = view.child(picker(model, commands));
    }

    view
}

fn picker(model: &WalletPanel, commands: &PanelCommands) -> Element {
    let mut view = Element::new(NativeKind::View)
        .class("w-full flex-col gap-y-2")
        .id("wallet_ui.picker")
        .aria_label("Choose a wallet")
        .child(Element::text("Choose a wallet"))
        .child(button(
            "Close wallet picker",
            "wallet_ui.close_picker",
            true,
            PanelAction::ClosePicker,
            commands,
        ));

    if model.wallets.is_empty() {
        return view.child(
            Element::text("No wallets found. Install a compatible wallet and try again.")
                .id("wallet_ui.empty_state"),
        );
    }

    for wallet in &model.wallets {
        view = view.child(button(
            &wallet.name,
            &format!("wallet_ui.wallet_tile.{}", wallet.key.0),
            wallet.enabled,
            PanelAction::Connect(wallet.key),
            commands,
        ));
    }

    view
}

fn button(
    label: &str,
    id: &str,
    enabled: bool,
    action: PanelAction,
    commands: &PanelCommands,
) -> Element {
    let mut view = Element::new(NativeKind::Button);
    view.props.text = Some(label.into());
    let commands = commands.clone();
    view.class("w-full min-h-[44px] px-3 py-2")
        .id(id)
        .aria_label(label)
        .enabled(enabled)
        .on(NativeEventSlot::Press, move |_| {
            match commands.send(action) {
                Ok(()) => EventOutcome::Handled,
                Err(_) => EventOutcome::Unhandled,
            }
        })
}
