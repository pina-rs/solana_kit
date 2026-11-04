//! Android's external wallet owns its own signing approval and persistence.
//!
//! This fixture deliberately keeps the wallet application authoritative: every
//! transaction and review signature is produced there, while the generated app
//! owns the granted RPC reads, the single SDK submission and the subscription
//! socket. No wallet key, seed or private file reaches this process.
use super::{
    Action, AppError, LocalWalletCommands, LocalWalletEvents, cancellable, failed, rpc, show, trace,
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
use solana_kit_leptoskit::{
    GuestMobileWallet, WalletSessionError, wallet_ui::mobile_wallet_panel,
};
use solana_kit_system::{TransferSol, TransferSolAccounts};
use solana_kit_wallet::mwa::{Method, ProtocolError};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

/// One-millions-lamport transfer review on the fixture's local validator.
const AMOUNT: u64 = 1_000_000;

/// A single Ed25519 signature appended by the wallet.
const SIGNATURE_BYTES: usize = 64;

/// The largest legacy packet the fixture compiles.
const PACKET_BYTES: usize = 1232;

/// A bounded account collection for the compiled instruction.
const COLLECTION_ITEMS: usize = 256;

/// A public test recipient. It has no signing authority and is never derived
/// from any wallet or application key.
const RECIPIENT: Address = Address::from_bytes([98; 32]);

/// The application pause that lets a person bring the wallet task forward
/// before the request is sent. The request would otherwise arrive while the
/// wallet's fragment manager is stopped and its navigation would be dropped.
const SWITCH_MILLIS: u64 = 15_000;

/// Application-owned RPC actions available while the external wallet is open.
#[derive(Clone, Copy)]
enum RemoteAction {
    Balance,
    Transfer,
    Subscription,
}

/// One intent from either native control family.
enum Intent {
    Wallet(Action),
    Remote(RemoteAction),
}

/// How one interaction loop ended. Dropping an in-flight request during a
/// pending cancellation already ended the native lease, so only an explicit
/// release from the idle panel must still close the wallet owner.
enum Step {
    /// Continue the loop with this status line.
    Next(String),
    /// The pending request was cancelled; the session is already unloaded.
    Cancelled,
    /// The person released the wallet; close the native owner explicitly.
    Released,
}

/// The wallet's observed answer to a reviewed transaction.
enum TransferOutcome {
    Rejected,
    Confirmed {
        signature: Signature,
        recipient_balance: u64,
    },
}

type RemoteCommands = Arc<Mutex<mpsc::Sender<RemoteAction>>>;
type RemoteEvents = mpsc::Receiver<RemoteAction>;

/// Authorization establishes the exact account used to verify later signatures.
/// Cancellation drops the pending operation and the external wallet lease.
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
    let mut status = "Android wallet authorized".to_owned();

    let close = interact(
        &mut wallet,
        &encoded,
        address,
        client.as_ref(),
        events,
        unrelated,
        actions,
        commands,
        generation,
        &mut status,
    )
    .await?;

    // A cancelled pending request already unloaded its lease with the dropped
    // call; closing again would fail an already-ended session.
    if close {
        wallet.close().await?;
    }

    Ok(())
}

/// One bounded application loop. Every iteration renders the current public
/// state before waiting for either a wallet button or an RPC action.
#[allow(clippy::too_many_arguments)]
async fn interact(
    wallet: &mut GuestMobileWallet,
    encoded: &str,
    address: Address,
    client: Option<&rpc::Rpc>,
    events: &mut Events,
    unrelated: &mut VecDeque<Event>,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
    generation: &mut u64,
    status: &mut String,
) -> Result<bool, AppError> {
    let (remote_commands, mut remote_actions) = channel();

    loop {
        show(
            view(&address.to_string(), client.is_some(), commands, &remote_commands),
            status,
        );

        let intent = next(actions, &mut remote_actions).await?;
        match dispatch(
            intent,
            wallet,
            encoded,
            address,
            client,
            events,
            unrelated,
            actions,
            commands,
            generation,
        )
        .await?
        {
            Step::Next(next) => *status = next,
            Step::Cancelled => return Ok(false),
            Step::Released => return Ok(true),
        }
    }
}

/// Applies one intent without nested client checks. `Step::Cancelled` ends the
/// selection on an already-unloaded lease; `Step::Released` closes it.
#[allow(clippy::too_many_arguments)]
async fn dispatch(
    intent: Intent,
    wallet: &mut GuestMobileWallet,
    encoded: &str,
    address: Address,
    client: Option<&rpc::Rpc>,
    events: &mut Events,
    unrelated: &mut VecDeque<Event>,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
    generation: &mut u64,
) -> Result<Step, AppError> {
    match (intent, client) {
        (Intent::Wallet(Action::Cancel | Action::Disconnect), _) => Ok(Step::Released),
        (Intent::Wallet(Action::Review), _) => {
            review(wallet, encoded, &address, generation, actions, commands)
                .await
                .map(step)
        }
        (Intent::Remote(RemoteAction::Balance), Some(client)) => {
            Ok(Step::Next(balance(client, address, actions).await))
        }
        (Intent::Remote(RemoteAction::Transfer), Some(client)) => {
            transfer(wallet, client, address, generation, actions, commands)
                .await
                .map(step)
        }
        (Intent::Remote(RemoteAction::Subscription), Some(client)) => {
            *generation = next_generation(*generation)?;

            Ok(Step::Next(
                watch(
                    &address.to_string(),
                    *generation,
                    events,
                    unrelated,
                    actions,
                    commands,
                    client,
                )
                .await,
            ))
        }
        (Intent::Remote(_), None) => Ok(Step::Next(
            "RPC service is unavailable in this host".into(),
        )),
        (Intent::Wallet(_), _) => Ok(Step::Next("Android wallet authorized".into())),
    }
}

/// Maps one reviewed operation onto the loop: `None` means the pending request
/// was cancelled and its lease already ended with the dropped call.
fn step(status: Option<String>) -> Step {
    match status {
        Some(status) => Step::Next(status),
        None => Step::Cancelled,
    }
}

/// Reads the authorized account through the same typed client and confirmed
/// commitment used by the local wallet journey.
async fn balance(
    client: &rpc::Rpc,
    address: Address,
    actions: &mut LocalWalletEvents,
) -> String {
    match rpc::bounded(rpc::balance(client, address), actions).await {
        Ok(lamports) => {
            trace::receipt(&format!("balance {address} {lamports}"));

            format!("Balance: {lamports} lamports")
        }
        Err(error) => {
            let message = error.to_string();
            failed(error);

            message
        }
    }
}

/// Signs and verifies one public review message. `None` means the pending
/// request was cancelled while the wallet owned it.
async fn review(
    wallet: &mut GuestMobileWallet,
    encoded: &str,
    address: &Address,
    generation: &mut u64,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Result<Option<String>, AppError> {
    *generation = next_generation(*generation)?;
    let current = *generation;
    let Some((status, signature)) =
        message(wallet, encoded, address, current, actions, commands).await?
    else {
        return Ok(None);
    };
    trace::receipt(&format!("message {address} generation={current} {signature}"));

    Ok(Some(status))
}

/// Reviews one transfer with the external wallet. The app submits the approved
/// bytes exactly once; a wallet rejection never reaches the validator.
async fn transfer(
    wallet: &mut GuestMobileWallet,
    client: &rpc::Rpc,
    address: Address,
    generation: &mut u64,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Result<Option<String>, AppError> {
    *generation = next_generation(*generation)?;
    let Some(outcome) = reviewed_transfer(wallet, client, address, actions, commands).await? else {
        return Ok(None);
    };

    match outcome {
        TransferOutcome::Rejected => Ok(Some(rejected(address))),
        TransferOutcome::Confirmed {
            signature,
            recipient_balance,
        } => Ok(Some(approved(
            address,
            signature,
            recipient_balance,
        ))),
    }
}

fn rejected(address: Address) -> String {
    trace::receipt(&format!("rejected {address} {RECIPIENT} {AMOUNT}"));

    "Transfer rejected; nothing submitted".into()
}

fn approved(address: Address, signature: Signature, recipient_balance: u64) -> String {
    trace::receipt(&format!(
        "approved {address} {RECIPIENT} {AMOUNT} {signature} {recipient_balance}"
    ));

    format!(
        "Transfer confirmed: {signature}. Recipient balance: {recipient_balance} lamports"
    )
}

/// Opens the granted socket, exercises the existing subscription race, and
/// always attempts both unsubscribes and a real socket close.
async fn watch(
    address: &str,
    generation: u64,
    events: &mut Events,
    unrelated: &mut VecDeque<Event>,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
    client: &rpc::Rpc,
) -> String {
    match rpc::watch(
        address, generation, events, unrelated, actions, commands, client,
    )
    .await
    {
        Ok(()) => "Subscription socket closed".into(),
        Err(error) => {
            let message = error.to_string();
            failed(error);

            message
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

/// The wallet appends one signature to the exact UTF-8 message.
async fn message(
    wallet: &mut GuestMobileWallet,
    encoded: &str,
    address: &Address,
    generation: u64,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Result<Option<(String, Signature)>, WalletSessionError> {
    let text = format!("Solana Kit Android review {generation}");
    let params =
        serde_json::json!({"addresses":[encoded],"payloads":[STANDARD.encode(text.as_bytes())]});
    let params = params
        .as_object()
        .ok_or(WalletSessionError::Malformed)?
        .clone();
    show(
        mobile_wallet_panel(&address.to_string(), true, commands),
        "Waiting for external wallet message approval",
    );
    pause().await?;

    let Some(result) = cancellable(wallet.call(Method::SignMessages, params), actions).await else {
        return Ok(None);
    };
    let result = result?;
    let signature = signed_message(&result, &text, address)?;

    Ok(Some(("Wallet signature verified".into(), signature)))
}

fn signed_message(
    result: &serde_json::Map<String, serde_json::Value>,
    text: &str,
    address: &Address,
) -> Result<Signature, WalletSessionError> {
    let payload = single_payload(result)?;

    if payload.len() != text.len() + SIGNATURE_BYTES || !payload.starts_with(text.as_bytes()) {
        return Err(WalletSessionError::Malformed);
    }

    let signature =
        Signature::try_from(&payload[text.len()..]).map_err(|_| WalletSessionError::Malformed)?;
    verify(address, text.as_bytes(), &signature).map_err(|_| WalletSessionError::Malformed)?;

    Ok(signature)
}

/// Prepares one exact legacy message, lets the wallet sign it, verifies the
/// returned bytes, and submits once through the SDK's typed send method.
async fn reviewed_transfer(
    wallet: &mut GuestMobileWallet,
    client: &rpc::Rpc,
    address: Address,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Result<Option<TransferOutcome>, AppError> {
    let prepared = prepare(client, address, actions).await?;
    let params = transaction_params(&prepared.transaction)?;
    let Some(outcome) =
        sign_transaction(wallet, &address, &prepared.transaction, params, actions, commands).await?
    else {
        return Ok(None);
    };

    let signature = match outcome {
        SignatureOutcome::Declined => return Ok(Some(TransferOutcome::Rejected)),
        SignatureOutcome::Signed(signature) => signature,
    };

    let recipient_balance = submit(
        client,
        prepared.transaction,
        signature,
        prepared.recipient_before,
        address,
        actions,
        commands,
    )
    .await?;

    Ok(Some(TransferOutcome::Confirmed {
        signature,
        recipient_balance,
    }))
}

/// Verifies the wallet's exact message signature, installs it into the frozen
/// transaction, and performs the single SDK submission.
async fn submit(
    client: &rpc::Rpc,
    transaction: Transaction,
    signature: Signature,
    recipient_before: u64,
    address: Address,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Result<u64, AppError> {
    let signed = transaction
        .with_signature(
            address,
            transaction.message().bytes(),
            signature,
        )
        .map_err(|_| WalletSessionError::Malformed)?;
    signed
        .verify()
        .map_err(|_| WalletSessionError::Malformed)?;
    show(
        mobile_wallet_panel(&address.to_string(), true, commands),
        &format!("Confirming transfer {signature}"),
    );

    rpc::bounded(
        rpc::submission::send(client, &signed, RECIPIENT, recipient_before, AMOUNT),
        actions,
    )
    .await
}

/// The wallet either declines or returns a signed serialized transaction. A
/// decline is reported as a normal review outcome; transport and protocol
/// failures remain application errors.
async fn sign_transaction(
    wallet: &mut GuestMobileWallet,
    address: &Address,
    transaction: &Transaction,
    params: serde_json::Map<String, serde_json::Value>,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Result<Option<SignatureOutcome>, AppError> {
    show(
        mobile_wallet_panel(&address.to_string(), true, commands),
        "Waiting for external wallet approval",
    );
    pause().await?;

    let Some(result) = cancellable(wallet.call(Method::SignTransactions, params), actions).await
    else {
        return Ok(None);
    };

    let result = match result {
        Ok(result) => result,
        Err(WalletSessionError::Rejected(error)) if error.code == ProtocolError::NOT_SIGNED => {
            return Ok(Some(SignatureOutcome::Declined));
        }
        Err(error) => return Err(error.into()),
    };
    let signed = single_payload(&result)?;

    Ok(Some(SignatureOutcome::Signed(approved_signature(
        &signed,
        transaction,
        *address,
    )?)))
}

/// A wallet decline is a normal review outcome, not a malformed response.
enum SignatureOutcome {
    Declined,
    Signed(Signature),
}

/// Performs only safe reads before the wallet review. The recipient is empty
/// on the first run, and the payer is the exact authorized account.
struct Prepared {
    transaction: Transaction,
    recipient_before: u64,
}

async fn prepare(
    client: &rpc::Rpc,
    address: Address,
    actions: &mut LocalWalletEvents,
) -> Result<Prepared, AppError> {
    let recipient_before = rpc::bounded(rpc::balance(client, RECIPIENT), actions).await?;
    recipient_before
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
    let message = Message::compile(Version::Legacy, address, lifetime, &[instruction], &[])?;

    Ok(Prepared {
        transaction: Transaction::new(message),
        recipient_before,
    })
}

fn transaction_params(
    transaction: &Transaction,
) -> Result<serde_json::Map<String, serde_json::Value>, WalletSessionError> {
    let wire = transaction
        .encode(false)
        .map_err(|_| WalletSessionError::Malformed)?;
    let params = serde_json::json!({"payloads":[STANDARD.encode(&wire)]});

    params
        .as_object()
        .cloned()
        .ok_or(WalletSessionError::Malformed)
}

fn single_payload(
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

/// The application's own public account from the authorization response.
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

fn next_generation(generation: u64) -> Result<u64, WalletSessionError> {
    generation
        .checked_add(1)
        .ok_or(WalletSessionError::Malformed)
}

/// A visible pending state is shown before the wallet is brought forward. This
/// bounded pause lets the person switch to the external application before the
/// request arrives, so the wallet renders it while its fragment is resumed.
async fn pause() -> Result<(), WalletSessionError> {
    if let Err(error) = rpc::sleep(SWITCH_MILLIS).await {
        failed(error);

        return Err(WalletSessionError::Malformed);
    }

    Ok(())
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
        .on(NativeEventSlot::Press, move |_| admit(action, &commands))
}

/// A stale panel cannot reach a replacement wallet because its receiver drops
/// with the selection.
fn admit(action: RemoteAction, commands: &RemoteCommands) -> EventOutcome {
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
}
