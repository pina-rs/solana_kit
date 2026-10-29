//! Semantic native controls share one finite sender for this subscription lifetime.
use super::activity::Activity;
use super::{Action, AppError, Error, LocalWalletCommands, LocalWalletEvents, Phase, View};
use futures::{StreamExt, channel::mpsc, future::Either};
use leptoskit::native::{Element, EventOutcome, NativeEventSlot, NativeKind};
use solana_kit_leptoskit::wallet_ui::local_wallet_panel;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub(super) enum Command {
    Hold,
    Cancel,
    Observe,
    Close,
}

pub(super) type Commands = Arc<Mutex<mpsc::Sender<Command>>>;

pub(super) fn channel() -> (Commands, mpsc::Receiver<Command>) {
    let (sender, receiver) = mpsc::channel(3);
    (Arc::new(Mutex::new(sender)), receiver)
}

pub(super) async fn next(
    wallet: &mut LocalWalletEvents,
    input: &mut mpsc::Receiver<Command>,
) -> Result<Command, AppError> {
    match futures::future::select(Box::pin(wallet.next_action()), Box::pin(input.next())).await {
        Either::Right((Some(command), _)) => Ok(command),
        Either::Left((Ok(Action::Cancel | Action::Disconnect), _)) => Err(Error::Cancelled.into()),
        _ => Err(Error::Queue.into()),
    }
}

/// The cancelled view renders the same stored balance, so stale delivery cannot
/// be hidden by replacing the visible value with an unrelated success snapshot.
pub(super) fn view(
    address: &str,
    view: &View,
    wallet: &LocalWalletCommands,
    commands: &Commands,
) -> Element {
    let balance = view
        .delivery
        .balance
        .map_or_else(|| "waiting".to_owned(), |value| format!("{value} lamports"));
    let label = if matches!(view.phase, Phase::Cancelled) {
        "Cancelled balance"
    } else {
        "Subscribed balance"
    };

    let mut panel = local_wallet_panel(Some(address), true, false, wallet)
        .child(
            Element::text(super::transport::ACCOUNT.to_string()).aria_label("Subscription account"),
        )
        .child(Element::text(format!("{label}: {balance}")).aria_label("Subscription balance"));
    panel = activity(panel, view.activity);

    let control = match view.phase {
        Phase::Live if view.delivery.balance.is_some() => {
            Some(("Hold next account update", Command::Hold))
        }
        Phase::Held(_) => Some(("Cancel account subscription", Command::Cancel)),

        Phase::Cancelled if matches!(view.activity, Activity::Idle) => {
            Some(("Observe later socket activity", Command::Observe))
        }

        _ => None,
    };

    if let Some((label, command)) = control {
        panel = panel.child(button(label, command, commands));
    }

    panel.child(button(
        "Close subscription socket",
        Command::Close,
        commands,
    ))
}

fn activity(panel: Element, activity: Activity) -> Element {
    let (context, balance, slot) = match activity {
        Activity::Idle => return panel,
        Activity::Waiting { context, balance } => (context, balance, "waiting".to_owned()),
        Activity::Observed {
            context,
            balance,
            slot,
        } => (context, balance, slot.to_string()),
    };

    panel
        .child(Element::text(context.to_string()).aria_label("Subscription observed context"))
        .child(Element::text(balance.to_string()).aria_label("Subscription observed RPC balance"))
        .child(Element::text(slot).aria_label("Subscription activity slot"))
}

fn button(label: &str, command: Command, commands: &Commands) -> Element {
    let commands = Arc::clone(commands);
    let mut element = Element::new(NativeKind::Button);
    element.props.text = Some(label.into());
    element
        .class("w-full min-h-[44px] px-3 py-2")
        .aria_label(label)
        .on(NativeEventSlot::Press, move |_| {
            let result = commands
                .lock()
                .map_err(|_| Error::Queue)
                .and_then(|mut sender| sender.try_send(command).map_err(|_| Error::Queue));

            match result {
                Ok(()) => EventOutcome::Handled,
                Err(error) => {
                    super::super::failed(error);
                    super::super::show(
                        Element::text("Subscription action unavailable"),
                        "Action queue unavailable",
                    );
                    EventOutcome::Unhandled
                }
            }
        })
}
