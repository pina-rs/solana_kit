//! One worker drives the existing binary session, with cancellation before commit.
use super::storage::{Directory, StorageError};
use crate::wallet_wire as wire;
use leptoskit_bridge::{CallId, Error, Request};
use solana_kit_core::crypto::ExportableKeypair;
use solana_kit_native::{
    keys::{NativeEntropy, NativeEntropyError},
    mwa::{MwaConfig, MwaError, NativeAssociation, NativeMobileWallet},
};
use solana_kit_wallet::mwa::{Method, Response};
use std::{
    collections::VecDeque,
    io,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    task::Waker,
    thread::JoinHandle,
};
use tokio::sync::{mpsc, watch};

const FAILURE_LIMIT: usize = 16;

/// Original native causes remain inspectable on the native owner, never in UI text.
pub enum WalletFailure {
    /// The finite service mailbox or wake state failed.
    Bridge(Error),
    /// Binary transport, wallet protocol or secure provider failure.
    Mobile(MwaError),
    /// Local key generation failed at the secure provider.
    Entropy(NativeEntropyError),
    /// Private persistence failed; its source chain retains the concrete cause.
    Storage(Box<dyn std::error::Error + Send + Sync>),
    /// Owned runtime or thread setup failed.
    Worker(io::Error),
    /// The bounded request format was rejected.
    Json(serde_json::Error),
}

impl std::fmt::Debug for WalletFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("WalletFailure(REDACTED)")
    }
}

impl std::fmt::Display for WalletFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("native wallet request failed")
    }
}

impl std::error::Error for WalletFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Bridge(error) => Some(error),
            Self::Mobile(error) => Some(error),
            Self::Entropy(error) => Some(error),
            Self::Storage(error) => Some(error.as_ref()),
            Self::Worker(error) => Some(error),
            Self::Json(error) => Some(error),
        }
    }
}

/// A bounded journal of original native errors for an application-owned observer.
#[derive(Clone)]
pub struct WalletFailures(pub(super) Arc<Shared>);

impl WalletFailures {
    /// Removes the oldest original error without formatting its contents.
    pub fn take(&self) -> Result<Option<WalletFailure>, Error> {
        self.0
            .failures
            .lock()
            .map(|mut errors| errors.pop_front())
            .map_err(|_| Error::Native)
    }
}

pub(super) struct Shared {
    epoch: AtomicU64,
    stopping: AtomicBool,
    stopped: AtomicBool,
    changed: watch::Sender<u64>,
    waker: Mutex<Option<Waker>>,
    failures: Mutex<VecDeque<WalletFailure>>,
    completion: Mutex<Option<Completion>>,
    failed: AtomicBool,
}

impl Default for Shared {
    fn default() -> Self {
        Self {
            epoch: AtomicU64::new(1),
            stopping: AtomicBool::new(false),
            stopped: AtomicBool::new(false),
            changed: watch::channel(1).0,
            waker: Mutex::new(None),
            failures: Mutex::new(VecDeque::new()),
            completion: Mutex::new(None),
            failed: AtomicBool::new(false),
        }
    }
}

impl Shared {
    pub(super) fn epoch(&self) -> u64 {
        self.epoch.load(Ordering::Acquire)
    }
    pub(super) fn stopped(&self) -> bool {
        self.stopped.load(Ordering::Acquire)
    }

    pub(super) fn watch(&self, waker: &Waker) -> Result<(), Error> {
        if self.failed.load(Ordering::Acquire) {
            return Err(Error::Native);
        }
        *self.waker.lock().map_err(|_| Error::Native)? = Some(waker.clone());
        Ok(())
    }

    pub(super) fn cancel(&self) {
        let epoch = self.epoch.fetch_add(1, Ordering::AcqRel) + 1;
        self.changed.send_replace(epoch);
    }

    pub(super) fn stop(&self) {
        self.stopping.store(true, Ordering::Release);
        self.cancel();
    }

    fn notify(&self) {
        let waker = match self.waker.lock() {
            Ok(mut waker) => waker.take(),
            Err(_) => {
                self.failed.store(true, Ordering::Release);
                None
            }
        };

        if let Some(waker) = waker {
            waker.wake();
        }
    }

    fn record(&self, error: WalletFailure) {
        if let Ok(mut errors) = self.failures.lock() {
            if errors.len() == FAILURE_LIMIT {
                errors.pop_front();
            }

            errors.push_back(error);
        } else {
            self.failed.store(true, Ordering::Release);
        }
    }

    pub(super) fn completion(&self) -> Result<Option<Completion>, Error> {
        self.completion
            .lock()
            .map(|mut completion| completion.take())
            .map_err(|_| Error::Native)
    }

    pub(super) fn complete(&self, completed: Completion) -> Result<(), Error> {
        let mut slot = self.completion.lock().map_err(|_| Error::Native)?;

        if slot
            .as_ref()
            .is_some_and(|old| old.epoch == completed.epoch)
        {
            return Err(Error::Busy);
        }

        *slot = Some(completed);
        drop(slot);
        self.notify();
        Ok(())
    }
}

pub(super) struct Command {
    pub request: Request,
    pub epoch: u64,
}
pub(super) struct Completion {
    pub call: CallId,
    pub epoch: u64,
    pub result: Result<Vec<u8>, Error>,
}

pub(super) fn start(
    directory: Directory,
    commands: mpsc::Receiver<Command>,
    shared: Arc<Shared>,
) -> io::Result<JoinHandle<()>> {
    std::thread::Builder::new()
        .name("solana-wallet".into())
        .spawn(move || {
            let _finished = Finished(Arc::clone(&shared));
            match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => {
                    if let Err(error) = runtime.block_on(run(directory, commands, &shared)) {
                        shared.record(WalletFailure::Bridge(error));
                        shared.failed.store(true, Ordering::Release);
                    }
                }
                Err(error) => shared.record(WalletFailure::Worker(error)),
            }
        })
}

struct Finished(Arc<Shared>);

impl Drop for Finished {
    fn drop(&mut self) {
        self.0.stopped.store(true, Ordering::Release);
        self.0.notify();
    }
}

#[derive(Default)]
struct State {
    association: Option<NativeAssociation>,
    mobile: Option<NativeMobileWallet>,
    local: Option<ExportableKeypair>,
}

async fn run(
    directory: Directory,
    mut commands: mpsc::Receiver<Command>,
    shared: &Shared,
) -> Result<(), Error> {
    let mut changed = shared.changed.subscribe();
    let mut state = State::default();

    while !shared.stopping.load(Ordering::Acquire) {
        let command = match receive(&mut commands, &mut changed).await {
            Incoming::Call(command) => command,
            Incoming::Reset => {
                state = State::default();
                continue;
            }
            Incoming::Stop => break,
        };

        if command.epoch != shared.epoch() {
            continue;
        }

        state = process(state, &directory, command, shared, &mut changed).await?;
    }

    Ok(())
}

enum Incoming {
    Call(Command),
    Reset,
    Stop,
}

async fn receive(
    commands: &mut mpsc::Receiver<Command>,
    changed: &mut watch::Receiver<u64>,
) -> Incoming {
    tokio::select! {
        biased;
        _ = changed.changed() => Incoming::Reset,
        command = commands.recv() => command.map(Incoming::Call).unwrap_or(Incoming::Stop),
    }
}

/// The selected future owns all private state. Cancellation drops it before
/// another command can obtain a fresh session or key, even if I/O was partial.
async fn process(
    state: State,
    directory: &Directory,
    command: Command,
    shared: &Shared,
    changed: &mut watch::Receiver<u64>,
) -> Result<State, Error> {
    let call = command.request.id;
    let epoch = command.epoch;
    let operation = execute(state, directory, command.request, shared);
    let outcome = tokio::select! {
        biased;
        _ = changed.changed() => None,
        outcome = operation => Some(outcome),
    };
    let Some((next, result)) = outcome else {
        return Ok(State::default());
    };

    if epoch != shared.epoch() {
        return Ok(State::default());
    }

    shared.complete(Completion {
        call,
        epoch,
        result,
    })?;
    Ok(next)
}

async fn execute(
    mut state: State,
    directory: &Directory,
    request: Request,
    shared: &Shared,
) -> (State, Result<Vec<u8>, Error>) {
    let result = apply(&mut state, directory, request.method.0, &request.payload).await;

    let result = match result {
        Ok(bytes) => Ok(bytes),
        Err(Failure::Bridge(error)) => Err(error),
        Err(Failure::Native(error)) => {
            shared.record(error);
            Err(Error::Native)
        }
    };

    if result.is_err() {
        state = State::default();
    }

    (state, result)
}

enum Failure {
    Bridge(Error),
    Native(WalletFailure),
}

async fn apply(
    state: &mut State,
    directory: &Directory,
    method: u32,
    payload: &[u8],
) -> Result<Vec<u8>, Failure> {
    let empty = [
        wire::BEGIN_MOBILE,
        wire::CONNECT_MOBILE,
        wire::CLOSE,
        wire::CREATE_LOCAL,
    ];

    if empty.contains(&method) && !payload.is_empty() {
        return Err(Failure::Bridge(Error::InvalidArgument));
    }

    match method {
        wire::BEGIN_MOBILE | wire::CONNECT_MOBILE => associate(state, method).await,
        wire::AUTHORIZE..=wire::DEAUTHORIZE => mobile_call(state, method, payload).await,
        wire::CLOSE => {
            if let Some(wallet) = state.mobile.take() {
                wallet.close().await.map_err(mobile)?;
            }

            *state = State::default();
            Ok(Vec::new())
        }

        wire::CREATE_LOCAL | wire::LOAD_LOCAL => {
            select_local(state, directory, method, payload).await
        }
        wire::SIGN_LOCAL => {
            let key = state
                .local
                .as_ref()
                .ok_or(Failure::Bridge(Error::Unavailable))?;
            Ok(key.signer().sign(payload).to_string().into_bytes())
        }
        wire::SAVE_LOCAL => save_local(state, directory, payload),
        _ => Err(Failure::Bridge(Error::InvalidArgument)),
    }
}

async fn associate(state: &mut State, method: u32) -> Result<Vec<u8>, Failure> {
    match method {
        wire::BEGIN_MOBILE => {
            *state = State::default();
            let association = NativeAssociation::generate(NativeEntropy::default())
                .await
                .map_err(mobile)?;
            let uri = association.uri(None).map_err(mobile)?;
            state.association = Some(association);
            Ok(uri.into_bytes())
        }
        wire::CONNECT_MOBILE => {
            let association = state
                .association
                .take()
                .ok_or(Failure::Bridge(Error::InvalidCall))?;
            state.mobile = Some(
                association
                    .connect(MwaConfig::default())
                    .await
                    .map_err(mobile)?,
            );
            Ok(Vec::new())
        }
        _ => Err(Failure::Bridge(Error::InvalidArgument)),
    }
}

async fn select_local(
    state: &mut State,
    directory: &Directory,
    method: u32,
    payload: &[u8],
) -> Result<Vec<u8>, Failure> {
    if state.local.is_some() {
        return Err(Failure::Bridge(Error::Busy));
    }

    let key = match method {
        wire::CREATE_LOCAL => ExportableKeypair::generate(&mut NativeEntropy::default())
            .await
            .map_err(|error| Failure::Native(WalletFailure::Entropy(error)))?,
        wire::LOAD_LOCAL => directory.load(backup_flag(payload)?).map_err(storage)?,
        _ => return Err(Failure::Bridge(Error::InvalidArgument)),
    };

    let address = key.signer().address().to_string().into_bytes();
    state.local = Some(key);
    Ok(address)
}

fn save_local(state: &State, directory: &Directory, payload: &[u8]) -> Result<Vec<u8>, Failure> {
    let backup = backup_flag(payload)?;

    let key = state
        .local
        .as_ref()
        .ok_or(Failure::Bridge(Error::Unavailable))?;
    directory.save(key, backup).map_err(storage)?;
    Ok(Vec::new())
}

fn backup_flag(payload: &[u8]) -> Result<bool, Failure> {
    match payload {
        [0] => Ok(false),
        [1] => Ok(true),
        _ => Err(Failure::Bridge(Error::InvalidArgument)),
    }
}

async fn mobile_call(state: &mut State, method: u32, payload: &[u8]) -> Result<Vec<u8>, Failure> {
    let method = match method {
        wire::AUTHORIZE => Method::Authorize,
        wire::SIGN_TRANSACTIONS => Method::SignTransactions,
        wire::SIGN_MESSAGES => Method::SignMessages,
        wire::SEND_TRANSACTIONS => Method::SignAndSendTransactions,
        wire::DEAUTHORIZE => Method::Deauthorize,
        _ => return Err(Failure::Bridge(Error::InvalidArgument)),
    };

    let params = serde_json::from_slice(payload)
        .map_err(|error| Failure::Native(WalletFailure::Json(error)))?;
    let wallet = state
        .mobile
        .as_mut()
        .ok_or(Failure::Bridge(Error::Unavailable))?;
    let result = wallet.call(method, params).await.map_err(mobile)?;

    let bytes = match result {
        Response::Result(result) => serde_json::to_vec(&serde_json::json!({ "result": result })),
        Response::Error(error) => serde_json::to_vec(&serde_json::json!({ "error": error })),
    }
    .map_err(|error| Failure::Native(WalletFailure::Json(error)))?;

    if bytes.len() > super::MAX_BYTES {
        return Err(Failure::Bridge(Error::TooLarge));
    }

    Ok(bytes)
}

fn mobile(error: MwaError) -> Failure {
    Failure::Native(WalletFailure::Mobile(error))
}
fn storage(error: StorageError) -> Failure {
    Failure::Native(WalletFailure::Storage(Box::new(error)))
}
