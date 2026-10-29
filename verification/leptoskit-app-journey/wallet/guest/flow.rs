//! One application executor and event owner select the concrete wallet backend.
mod approval;
mod local;
mod mobile;
mod rpc;

use super::{AppError, failed, show, trace};
use leptoskit_bridge::{Error, Event, guest::Events};
use solana_kit_leptoskit::wallet_ui::{
    LocalWalletAction as Action, LocalWalletCommands, LocalWalletEvents, local_wallet_panel,
};
use solana_kit_leptoskit::{
    GuestLocalWallet, GuestMobileWallet, GuestWalletPlatform, WalletSessionError,
};
use std::{collections::VecDeque, future::Future, task::Poll};

const EVENT_LIMIT: usize = 64;
const ACTION_BUDGET: usize = 32;

/// Keep the event receiver and non-reused review sequence across wallet
/// replacement. A selection owns its key until completion or cancellation.
pub(super) async fn run(
    mut events: Events,
    mut actions: LocalWalletEvents,
    commands: LocalWalletCommands,
) {
    let android = match availability().await {
        Ok(available) => available,
        Err(error) => {
            failed(error);
            false
        }
    };

    let mut unrelated = VecDeque::new();
    let mut generation = 0_u64;
    show(
        local_wallet_panel(None, false, android, &commands),
        "Choose a wallet",
    );

    while let Ok(action) = actions.next_action().await {
        trace::action(action);
        show(
            local_wallet_panel(None, true, android, &commands),
            "Opening selected wallet",
        );
        let result = select(
            action,
            android,
            &mut events,
            &mut unrelated,
            &mut actions,
            &commands,
            &mut generation,
        )
        .await;
        // This fixture has no other event consumers; finish each selection's queue.
        unrelated.clear();

        let status = result_status(result);
        show(local_wallet_panel(None, false, android, &commands), &status);
    }
}

fn result_status(result: Result<(), AppError>) -> String {
    trace::result(&result);

    match result {
        Ok(()) => "Choose a wallet".to_owned(),
        Err(error)
            if matches!(
                error.downcast_ref::<WalletSessionError>(),
                Some(WalletSessionError::Bridge(Error::Cancelled))
            ) =>
        {
            "Wallet request cancelled".to_owned()
        }
        Err(error) => {
            let status = error.to_string();
            failed(error);
            status
        }
    }
}

async fn availability() -> Result<bool, WalletSessionError> {
    let platform = match GuestWalletPlatform::new() {
        Ok(platform) => platform,
        Err(solana_kit_leptoskit::WalletPlatformError::Bridge(Error::Unavailable)) => {
            return Ok(false);
        }
        Err(error) => return Err(WalletSessionError::Platform(error)),
    };

    platform
        .available()
        .await
        .map_err(WalletSessionError::Platform)
}

/// Keep a single review sequence across wallet replacement, so an old button
/// can never approve a newly selected wallet's request with a reused identity.
async fn select(
    action: Action,
    android: bool,
    events: &mut Events,
    unrelated: &mut VecDeque<Event>,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
    generation: &mut u64,
) -> Result<(), AppError> {
    let route = |event: Event| {
        if unrelated.len() == EVENT_LIMIT {
            return Err(Error::Busy);
        }

        unrelated.push_back(event);
        Ok(())
    };

    match action {
        Action::Create | Action::Load | Action::RestoreBackup => {
            let operation = open_local(action, events, route);
            let Some(wallet) = cancellable(operation, actions).await else {
                return Ok(());
            };

            rpc::run(wallet?, events, unrelated, actions, commands, generation).await
        }

        Action::Android if android => {
            let operation = GuestMobileWallet::connect_android(events, route);
            let Some(wallet) = cancellable(operation, actions).await else {
                return Ok(());
            };

            mobile::run(wallet?, actions, commands, generation)
                .await
                .map_err(Into::into)
        }

        _ => Ok(()),
    }
}

async fn open_local(
    action: Action,
    events: &mut Events,
    route: impl FnMut(Event) -> Result<(), Error>,
) -> Result<GuestLocalWallet, WalletSessionError> {
    match action {
        Action::Create => GuestLocalWallet::create(events, route).await,
        Action::Load => GuestLocalWallet::load(events, route).await,
        Action::RestoreBackup => GuestLocalWallet::restore_backup(events, route).await,
        _ => Err(WalletSessionError::Malformed),
    }
}

pub(super) async fn cancellable<T>(
    operation: impl Future<Output = T>,
    actions: &mut LocalWalletEvents,
) -> Option<T> {
    let mut operation = Box::pin(operation);
    let mut action = Box::pin(actions.next_action());
    std::future::poll_fn(|context| {
        for _ in 0..ACTION_BUDGET {
            match action.as_mut().poll(context) {
                Poll::Ready(Ok(action @ (Action::Cancel | Action::Disconnect))) => {
                    trace::action(action);
                    return Poll::Ready(None);
                }
                Poll::Ready(Err(_)) => {
                    return Poll::Ready(None);
                }
                Poll::Ready(Ok(action)) => trace::action(action),
                Poll::Pending => return operation.as_mut().poll(context).map(Some),
            }
        }

        context.waker().wake_by_ref();
        operation.as_mut().poll(context).map(Some)
    })
    .await
}
