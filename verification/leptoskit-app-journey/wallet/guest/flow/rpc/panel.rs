//! One finite RPC action channel belongs to each selected local wallet.
use super::{Action, AppError, Error, LocalWalletCommands, LocalWalletEvents, failed, show};
use futures::{StreamExt, channel::mpsc, future::Either};
use leptoskit::native::{Element, EventOutcome, NativeEventSlot, NativeKind};
use solana_kit_leptoskit::wallet_ui::local_wallet_panel;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub(super) enum RpcAction {
    Balance,
    Transfer,
}

pub(super) enum Intent {
    Wallet(Action),
    Rpc(RpcAction),
}

// All rendered callbacks share one Sender, including its one reserved slot.
// Replacing a wallet drops its receiver, so stale RPC buttons cannot reach a new key.
pub(super) type Commands = Arc<Mutex<mpsc::Sender<RpcAction>>>;
pub(super) type Actions = mpsc::Receiver<RpcAction>;

pub(super) fn channel() -> (Commands, Actions) {
    let (sender, receiver) = mpsc::channel(7);
    (Arc::new(Mutex::new(sender)), receiver)
}

pub(super) async fn next(
    wallet: &mut LocalWalletEvents,
    rpc: &mut Actions,
) -> Result<Intent, AppError> {
    let wallet = Box::pin(wallet.next_action());
    let rpc = Box::pin(rpc.next());

    match futures::future::select(wallet, rpc).await {
        Either::Left((Ok(action), _)) => {
            super::super::trace::action(action);
            Ok(Intent::Wallet(action))
        }
        Either::Right((Some(action), _)) => Ok(Intent::Rpc(action)),
        _ => Err(Error::Queue.into()),
    }
}

pub(super) fn view(
    address: &str,
    rpc: bool,
    wallet_commands: &LocalWalletCommands,
    rpc_commands: &Commands,
) -> Element {
    let panel = local_wallet_panel(Some(address), false, false, wallet_commands);

    if !rpc {
        return panel.child(Element::text("RPC service is unavailable in this host"));
    }

    panel
        .child(button(
            "Read balance",
            "balance",
            RpcAction::Balance,
            rpc_commands,
        ))
        .child(button(
            "Review test SOL transfer",
            "transfer",
            RpcAction::Transfer,
            rpc_commands,
        ))
}

fn button(label: &str, id: &str, action: RpcAction, commands: &Commands) -> Element {
    let commands = Arc::clone(commands);
    let mut button = Element::new(NativeKind::Button);
    button.props.text = Some(label.into());
    button
        .id(format!("wallet_rpc.{id}"))
        .class("w-full min-h-[44px] px-3 py-2")
        .aria_label(label)
        .on(NativeEventSlot::Press, move |_| {
            let admitted = commands
                .lock()
                .map_err(|_| Error::Queue)
                .and_then(|mut sender| sender.try_send(action).map_err(|_| Error::Queue));

            match admitted {
                Ok(()) => EventOutcome::Handled,
                Err(error) => {
                    failed(error);
                    show(
                        Element::text("Action was not accepted"),
                        "Action queue unavailable",
                    );
                    EventOutcome::Unhandled
                }
            }
        })
}
