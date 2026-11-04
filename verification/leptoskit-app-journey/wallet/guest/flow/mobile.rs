//! Android's external wallet owns its own signing approval and persistence.
use super::{
    Action, AppError, LocalWalletCommands, LocalWalletEvents, cancellable, failed, rpc, show,
};
use base64::{Engine, engine::general_purpose::STANDARD};
use futures::{StreamExt, channel::mpsc, future::Either};
use leptoskit::native::{Element, EventOutcome, NativeEventSlot, NativeKind};
use leptoskit_bridge::{Event, guest::Events};
use solana_kit_client::methods::LatestBlockhash;
use solana_kit_core::{
    Address, Signature, codecs,
    crypto::verify,
    transaction::{Lifetime, Message, Transaction, Version},
};
use solana_kit_leptoskit::{GuestMobileWallet, WalletSessionError, wallet_ui::mobile_wallet_panel};
use solana_kit_system::{TransferSol, TransferSolAccounts};
use solana_kit_wallet::mwa::Method;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

const AMOUNT: u64 = 1_000_000;
const SIGNATURE_BYTES: usize = 64;
const PACKET_BYTES: usize = 1232;
const COLLECTION_ITEMS: usize = 256;
// This is a public test recipient on the fixture's explicit local validator.
const RECIPIENT: Address = Address::from_bytes([98; 32]);

#[derive(Clone, Copy)]
enum RemoteAction {
    Balance,
    Transfer,
    Subscription,
}

enum Intent {
    Wallet(Action),
    Remote(RemoteAction),
}

type RemoteCommands = Arc<Mutex<mpsc::Sender<RemoteAction>>>;
type RemoteEvents = mpsc::Receiver<RemoteAction>;

/// Authorization establishes the exact account used to verify later signatures;
/// cancellation drops the pending operation and the external wallet lease.
pub(super) async fn run(
    mut wallet: GuestMobileWallet,
    events: &mut Events,
    unrelated: &mut VecDeque<Event>,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
    generation: &mut u64,
) -> Result<(), AppError> {
    let Some((encoded, address)) = authorize(&mut wallet, actions).await? else {
        return Ok(());
    };
    let client = rpc::available()?;
    let (remote_commands, mut remote_actions) = channel();
    let mut status = "Android wallet authorized".to_owned();

    loop {
        show(
            view(
                &address.to_string(),
                client.is_some(),
                commands,
                &remote_commands,
            ),
            &status,
        );

        match next(actions, &mut remote_actions).await? {
            Intent::Wallet(Action::Cancel) => {
                wallet.close().await?;

                return Ok(());
            }
            Intent::Wallet(Action::Disconnect) => return wallet.close().await.map_err(Into::into),
            Intent::Wallet(Action::Review) => {
                *generation = generation
                    .checked_add(1)
                    .ok_or(WalletSessionError::Malformed)?;

                let Some(next) =
                    message(&mut wallet, &encoded, &address, *generation, actions).await?
                else {
                    wallet.close().await?;

                    return Ok(());
                };
                status = next;
            }
            Intent::Remote(RemoteAction::Balance) => {
                let Some(client) = client.as_ref() else {
                    continue;
                };
                status = match rpc::bounded(rpc::balance(client, address), actions).await {
                    Ok(balance) => format!("Balance: {balance} lamports"),
                    Err(error) => {
                        let message = error.to_string();
                        failed(error);

                        message
                    }
                };
            }
            Intent::Remote(RemoteAction::Transfer) => {
                let Some(client) = client.as_ref() else {
                    continue;
                };
                *generation = generation
                    .checked_add(1)
                    .ok_or(WalletSessionError::Malformed)?;

                let Some(next) = transfer(&mut wallet, client, address, actions, commands).await?
                else {
                    wallet.close().await?;

                    return Ok(());
                };
                status = next;
            }
            Intent::Remote(RemoteAction::Subscription) => {
                let Some(client) = client.as_ref() else {
                    continue;
                };
                *generation = generation
                    .checked_add(1)
                    .ok_or(WalletSessionError::Malformed)?;

                match rpc::watch(
                    &address.to_string(),
                    *generation,
                    events,
                    unrelated,
                    actions,
                    commands,
                    client,
                )
                .await
                {
                    Ok(()) => status = "Subscription socket closed".into(),
                    Err(error) => {
                        status = error.to_string();
                        failed(error);
                    }
                }
            }
            Intent::Wallet(_) => {}
        }
    }
}

async fn authorize(
    wallet: &mut GuestMobileWallet,
    actions: &mut LocalWalletEvents,
) -> Result<Option<(String, Address)>, WalletSessionError> {
    let params =
        serde_json::json!({"chain":"solana:devnet", "identity":{"name":"Solana Kit fixture"}});
    let params = params
        .as_object()
        .ok_or(WalletSessionError::Malformed)?
        .clone();
    let Some(result) = cancellable(wallet.call(Method::Authorize, params), actions).await else {
        return Ok(None);
    };
    let encoded = account(&result?)?;
    let bytes = STANDARD
        .decode(&encoded)
        .map_err(|_| WalletSessionError::Malformed)?;
    let address = Address::try_from(bytes.as_slice()).map_err(|_| WalletSessionError::Malformed)?;
    Ok(Some((encoded, address)))
}

async fn message(
    wallet: &mut GuestMobileWallet,
    encoded: &str,
    address: &Address,
    generation: u64,
    actions: &mut LocalWalletEvents,
) -> Result<Option<String>, WalletSessionError> {
    let text = format!("Solana Kit Android review {generation}");
    let params =
        serde_json::json!({"addresses":[encoded],"payloads":[STANDARD.encode(text.as_bytes())]});
    let params = params
        .as_object()
        .ok_or(WalletSessionError::Malformed)?
        .clone();
    let Some(result) = cancellable(wallet.call(Method::SignMessages, params), actions).await else {
        return Ok(None);
    };
    let result = result?;
    let payloads = result
        .get("signed_payloads")
        .and_then(|value| value.as_array())
        .ok_or(WalletSessionError::Malformed)?;

    if payloads.len() != 1 {
        return Err(WalletSessionError::Malformed);
    }

    let encoded = payloads[0].as_str().ok_or(WalletSessionError::Malformed)?;
    let signed = STANDARD
        .decode(encoded)
        .map_err(|_| WalletSessionError::Malformed)?;

    if signed.len() != text.len() + SIGNATURE_BYTES || !signed.starts_with(text.as_bytes()) {
        return Err(WalletSessionError::Malformed);
    }

    let signature =
        Signature::try_from(&signed[text.len()..]).map_err(|_| WalletSessionError::Malformed)?;
    verify(address, text.as_bytes(), &signature).map_err(|_| WalletSessionError::Malformed)?;
    Ok(Some("Wallet signature verified".into()))
}

/// The external wallet produces only a signature. The SDK, not the wallet,
/// submits the exact approved bytes once to the granted local validator.
async fn transfer(
    wallet: &mut GuestMobileWallet,
    client: &rpc::Rpc,
    address: Address,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Result<Option<String>, AppError> {
    let before = rpc::bounded(rpc::balance(client, RECIPIENT), actions).await?;
    before
        .checked_add(AMOUNT)
        .ok_or(WalletSessionError::Malformed)?;
    let latest = rpc::bounded(client.call(LatestBlockhash(rpc::read_config())), actions).await?;
    let instruction = TransferSol { amount: AMOUNT }.instruction(
        TransferSolAccounts {
            source: address,
            destination: RECIPIENT,
        },
        codecs::Limits::new(PACKET_BYTES, COLLECTION_ITEMS),
    )?;
    let lifetime = Lifetime::Blockhash {
        blockhash: latest.value.blockhash,
        last_valid_block_height: latest.value.last_valid_block_height,
    };
    let transaction = Transaction::new(Message::compile(
        Version::Legacy,
        address,
        lifetime,
        &[instruction],
        &[],
    )?);
    let wire = transaction.encode(false)?;
    let params = serde_json::json!({"payloads":[STANDARD.encode(&wire)]});
    let params = params
        .as_object()
        .ok_or(WalletSessionError::Malformed)?
        .clone();
    show(
        mobile_wallet_panel(&address.to_string(), true, commands),
        "Waiting for external wallet approval",
    );

    let Some(result) = cancellable(wallet.call(Method::SignTransactions, params), actions).await
    else {
        return Ok(None);
    };
    let result = match result {
        Ok(result) => result,
        Err(WalletSessionError::Rejected(_)) => {
            return Ok(Some("Transfer rejected; nothing submitted".into()));
        }
        Err(error) => return Err(error.into()),
    };
    let signed = signed_payload(&result)?;
    let signature = approved_signature(&signed, &transaction, address)?;
    let approved = transaction
        .with_signature(address, transaction.message().bytes(), signature)
        .map_err(|_| WalletSessionError::Malformed)?;
    approved
        .verify()
        .map_err(|_| WalletSessionError::Malformed)?;
    show(
        mobile_wallet_panel(&address.to_string(), true, commands),
        &format!("Confirming transfer {signature}"),
    );
    let after = rpc::bounded(
        rpc::submission::send(client, &approved, RECIPIENT, before, AMOUNT),
        actions,
    )
    .await?;
    Ok(Some(format!(
        "Transfer confirmed: {signature}. Recipient balance: {after} lamports"
    )))
}

fn signed_payload(
    result: &serde_json::Map<String, serde_json::Value>,
) -> Result<Vec<u8>, WalletSessionError> {
    let payloads = result
        .get("signed_payloads")
        .and_then(|value| value.as_array())
        .ok_or(WalletSessionError::Malformed)?;

    if payloads.len() != 1 {
        return Err(WalletSessionError::Malformed);
    }

    let encoded = payloads[0].as_str().ok_or(WalletSessionError::Malformed)?;
    STANDARD
        .decode(encoded)
        .map_err(|_| WalletSessionError::Malformed)
}

/// The wallet must sign the exact frozen message with the authorized account.
fn approved_signature(
    signed: &[u8],
    transaction: &Transaction,
    address: Address,
) -> Result<Signature, WalletSessionError> {
    let decoded = Transaction::decode(signed).map_err(|_| WalletSessionError::Malformed)?;

    if decoded.message().bytes() != transaction.message().bytes() {
        return Err(WalletSessionError::Malformed);
    }

    let slot = decoded
        .message()
        .signers()
        .iter()
        .position(|signer| *signer == address)
        .ok_or(WalletSessionError::Malformed)?;
    let signature = decoded
        .signatures()
        .get(slot)
        .copied()
        .flatten()
        .ok_or(WalletSessionError::Malformed)?;
    verify(&address, decoded.message().bytes(), &signature)
        .map_err(|_| WalletSessionError::Malformed)?;
    Ok(signature)
}

fn account(
    result: &serde_json::Map<String, serde_json::Value>,
) -> Result<String, WalletSessionError> {
    result
        .get("accounts")
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("address"))
        .and_then(|value| value.as_str())
        .map(str::to_owned)
        .ok_or(WalletSessionError::Malformed)
}

fn channel() -> (RemoteCommands, RemoteEvents) {
    let (sender, receiver) = mpsc::channel(4);
    (Arc::new(Mutex::new(sender)), receiver)
}

async fn next(
    wallet: &mut LocalWalletEvents,
    remote: &mut RemoteEvents,
) -> Result<Intent, AppError> {
    let wallet = Box::pin(wallet.next_action());
    let remote = Box::pin(remote.next());

    match futures::future::select(wallet, remote).await {
        Either::Left((Ok(action), _)) => Ok(Intent::Wallet(action)),
        Either::Right((Some(action), _)) => Ok(Intent::Remote(action)),
        _ => Err(WalletSessionError::Malformed.into()),
    }
}

fn view(
    address: &str,
    rpc: bool,
    wallet_commands: &LocalWalletCommands,
    remote_commands: &RemoteCommands,
) -> Element {
    let panel = mobile_wallet_panel(address, false, wallet_commands).child(
        Element::text(address)
            .id("wallet_mobile.address")
            .aria_label("External wallet address"),
    );

    if !rpc {
        return panel.child(Element::text("RPC service is unavailable in this host"));
    }

    panel
        .child(remote_button(
            "Read external wallet balance",
            "balance",
            RemoteAction::Balance,
            remote_commands,
        ))
        .child(remote_button(
            "Review external wallet transfer",
            "transfer",
            RemoteAction::Transfer,
            remote_commands,
        ))
        .child(remote_button(
            "Watch account updates",
            "subscription",
            RemoteAction::Subscription,
            remote_commands,
        ))
}

fn remote_button(
    label: &str,
    id: &str,
    action: RemoteAction,
    commands: &RemoteCommands,
) -> Element {
    let commands = Arc::clone(commands);
    let mut button = Element::new(NativeKind::Button);
    button.props.text = Some(label.into());
    button
        .id(format!("wallet_mobile.{id}"))
        .class("w-full min-h-[44px] px-3 py-2")
        .aria_label(label)
        .on(NativeEventSlot::Press, move |_| {
            let admitted = commands
                .lock()
                .map_err(|_| WalletSessionError::Malformed)
                .and_then(|mut sender| {
                    sender
                        .try_send(action)
                        .map_err(|_| WalletSessionError::Malformed)
                });

            match admitted {
                Ok(()) => EventOutcome::Handled,
                Err(error) => {
                    failed(error);
                    EventOutcome::Unhandled
                }
            }
        })
}
