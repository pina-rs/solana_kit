//! A local-wallet controller uses the actual host-granted typed RPC client.
mod error;
mod panel;
mod submission;
mod transfer;

use super::{Action, AppError, LocalWalletCommands, LocalWalletEvents, failed, local, show};
use error::Error;
use futures::future::Either;
use leptoskit_bridge::{MethodId, guest::Client as Plugin};
use panel::{Intent, RpcAction};
use solana_kit_client::{
    Client, Limits, TransportFailureKind,
    methods::{Balance, Commitment, ReadConfig},
};
use solana_kit_core::Address;
use solana_kit_leptoskit::{GuestHttp, GuestLocalWallet, wallet_ui::local_wallet_panel};
use std::future::Future;

type Rpc = Client<GuestHttp>;

struct Connected {
    wallet: GuestLocalWallet,
    status: String,
}

/// The selected key and RPC action receiver share one lifetime. Closing the
/// selection disposes old callbacks before another wallet can be selected.
pub(super) async fn run(
    wallet: GuestLocalWallet,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
    generation: &mut u64,
) -> Result<(), AppError> {
    let client = available()?;
    let address = wallet.address().to_string();
    let (rpc_commands, mut rpc_actions) = panel::channel();
    let mut connected = Connected {
        wallet,
        status: "Local wallet connected; not saved automatically".to_owned(),
    };

    loop {
        show(
            panel::view(&address, client.is_some(), commands, &rpc_commands),
            &connected.status,
        );
        let intent = panel::next(actions, &mut rpc_actions).await?;
        let Some(next) = update(
            connected,
            intent,
            client.as_ref(),
            actions,
            commands,
            generation,
        )
        .await?
        else {
            return Ok(());
        };
        connected = next;
    }
}

/// Only operations which consume signing authority move the key. Ordinary RPC
/// failures retain it, while an explicit cancel closes this entire selection.
async fn update(
    mut connected: Connected,
    intent: Intent,
    client: Option<&Rpc>,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
    generation: &mut u64,
) -> Result<Option<Connected>, AppError> {
    match intent {
        Intent::Wallet(Action::Cancel | Action::Disconnect) => {
            connected.wallet.close().await?;

            return Ok(None);
        }
        Intent::Wallet(Action::Review) => {
            *generation = generation.checked_add(1).ok_or(Error::Generation)?;
            let (wallet, status) =
                local::sign(connected.wallet, *generation, actions, commands).await?;
            connected.wallet = wallet;
            connected.status = status.into();
        }
        Intent::Wallet(action @ (Action::Save | Action::Backup)) => {
            let Some(result) = local::save(&mut connected.wallet, action, actions, commands).await
            else {
                return Ok(None);
            };
            connected.status = result?.into();
        }
        Intent::Rpc(action) => {
            let client = client.ok_or(Error::Queue)?;
            let address = connected.wallet.address().to_string();
            show(
                local_wallet_panel(Some(&address), true, false, commands),
                "RPC request pending",
            );
            let (wallet, status) = rpc_action(
                action,
                client,
                connected.wallet,
                actions,
                commands,
                generation,
            )
            .await?;
            connected = Connected { wallet, status };
        }
        _ => {}
    }

    Ok(Some(connected))
}

fn available() -> Result<Option<Rpc>, AppError> {
    match GuestHttp::new("solana.rpc") {
        Ok(http) => Ok(Some(Client::new(http, Limits::default()))),
        Err(error) if error.kind == TransportFailureKind::Unavailable => Ok(None),
        Err(error) => Err(error.into()),
    }
}

async fn rpc_action(
    action: RpcAction,
    client: &Rpc,
    wallet: GuestLocalWallet,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
    generation: &mut u64,
) -> Result<(GuestLocalWallet, String), AppError> {
    match action {
        RpcAction::Balance => match bounded(balance(client, wallet.address()), actions).await {
            Ok(result) => Ok((wallet, format!("Balance: {result} lamports"))),
            Err(error) => retain(wallet, error),
        },
        RpcAction::Transfer => {
            *generation = generation.checked_add(1).ok_or(Error::Generation)?;
            transfer::run(client, wallet, actions, commands, *generation).await
        }
    }
}

fn cancelled(error: &AppError) -> bool {
    matches!(error.downcast_ref::<Error>(), Some(Error::Cancelled))
}

fn retain(
    wallet: GuestLocalWallet,
    error: AppError,
) -> Result<(GuestLocalWallet, String), AppError> {
    if cancelled(&error) {
        return Err(error);
    }

    let status = error.to_string();
    failed(error);
    Ok((wallet, status))
}

fn read_config() -> Option<ReadConfig> {
    Some(ReadConfig {
        commitment: Some(Commitment::Confirmed),
        min_context_slot: None,
    })
}

async fn balance(client: &Rpc, address: Address) -> Result<u64, AppError> {
    Ok(client.call(Balance(address, read_config())).await?.value)
}

/// The same native timer bounds quiet network operations; no browser or OS clock is assumed.
async fn sleep(millis: u64) -> Result<(), AppError> {
    Plugin::new("journey.timer")?
        .call(MethodId(1), &millis.to_le_bytes())?
        .await?;
    Ok(())
}

async fn bounded<T, E: Into<AppError>>(
    operation: impl Future<Output = Result<T, E>>,
    actions: &mut LocalWalletEvents,
) -> Result<T, AppError> {
    let pending = futures::future::select(Box::pin(sleep(30_000)), Box::pin(operation));

    match super::cancellable(pending, actions).await {
        Some(Either::Right((result, _))) => result.map_err(Into::into),
        Some(Either::Left((timer, _))) => {
            timer?;
            Err(Error::Deadline.into())
        }
        None => Err(Error::Cancelled.into()),
    }
}
