//! The generated host owns this service through AppService and ClosingApp.
mod install_error;
mod mailbox;
mod storage;
pub use install_error::WalletInstallError;
#[cfg(all(test, unix))]
mod composition_tests;
#[cfg(all(test, unix))]
mod routing_tests;
#[cfg(all(test, unix))]
mod tests;
mod worker;

use crate::wallet_wire as wire;
use leptoskit_bridge::{BridgeIdentity, CallId, Error, Event, MethodId, PluginId};
use leptoskit_bridge_ffi::app::{AppService, ServiceClose, ServiceInstaller, ServiceWork};
use leptoskit_native::HeadlessBackend;
use leptoskit_wasm::{HostIdentity, WasmHost};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
    task::{Context, Poll},
    thread::JoinHandle,
};
use tokio::sync::mpsc;
pub use worker::{WalletFailure, WalletFailures};

const SERVICE: &str = "solana-wallet-session";
const MAX_BYTES: usize = 65_536;
const MAX_CALLS: u64 = 256;
const MAX_ROUTE_TURNS: usize = wire::POLL_BUDGET;

/// Owns a unique native session, its native worker, and original error journal.
pub struct WalletService {
    interpreter: HostIdentity,
    bridge: BridgeIdentity,
    plugin: PluginId,
    mailbox: mailbox::Mailbox,
    shared: Arc<worker::Shared>,
    thread: Option<JoinHandle<()>>,
    pending: BTreeMap<CallId, (u64, u32)>,
    lease: Option<CallId>,
    calls: u64,
    queued: ServiceWork,
    drained: BTreeSet<CallId>,
}

/// Returns an inert installer using only host-selected application directories.
/// The platform registrant separately installs Android's Intent capability.
pub fn from_app_dirs(data: &Path, _cache: &Path) -> Result<ServiceInstaller, Error> {
    let directory = storage::Directory::new(data).map_err(|_| Error::Native)?;
    Ok(Box::new(move |host| {
        let service =
            WalletService::install_directory(host, directory).map_err(|_| Error::Native)?;
        Ok(Box::new(service))
    }))
}

impl WalletService {
    /// Installs against the actual interpreter and a trusted app sandbox root.
    /// Retain `failures()` before moving this owner into a custom AppService.
    pub fn install(
        host: &mut WasmHost<HeadlessBackend>,
        data: &Path,
    ) -> Result<Self, WalletInstallError> {
        let directory = storage::Directory::new(data).map_err(WalletInstallError::Directory)?;
        Self::install_directory(host, directory)
    }

    fn install_directory(
        host: &mut WasmHost<HeadlessBackend>,
        directory: storage::Directory,
    ) -> Result<Self, WalletInstallError> {
        let plugin = host
            .plugins()
            .register(SERVICE, (0..wire::METHODS).map(MethodId))?;

        for method in 0..wire::METHODS {
            host.plugins().grant(plugin, MethodId(method), true)?;
        }

        let shared = Arc::new(worker::Shared::default());
        let (sender, commands) = mpsc::channel(1);
        let thread = worker::start(directory, commands, Arc::clone(&shared))
            .map_err(WalletInstallError::Worker)?;
        Ok(Self {
            interpreter: host.identity(),
            bridge: host.plugins().identity(),
            plugin,
            mailbox: mailbox::Mailbox::new(sender),
            shared,
            thread: Some(thread),
            pending: BTreeMap::new(),
            lease: None,
            calls: 0,
            queued: ServiceWork::default(),
            drained: BTreeSet::new(),
        })
    }

    /// Retains opaque original native causes without logging peer or key bytes.
    pub fn failures(&self) -> WalletFailures {
        WalletFailures(Arc::clone(&self.shared))
    }

    /// Check generation before emitting authority; a completed lease remains
    /// pending until its guest owner closes or cancels it.
    fn deliver(&mut self, host: &mut WasmHost<HeadlessBackend>) -> Result<(), Error> {
        while let Some(completion) = self.shared.completion()? {
            let Some((epoch, method)) = self.pending.remove(&completion.call) else {
                continue;
            };

            if epoch != self.shared.epoch() || epoch != completion.epoch {
                continue;
            }

            if wire::starting(method) && completion.result.is_ok() {
                self.open_lease(host, completion, method)?;
                continue;
            }

            if method == wire::CLOSE || completion.result.is_err() {
                self.end_lease(host)?;
            }

            finish(host, completion.call, completion.result)?;
        }

        Ok(())
    }

    fn open_lease(
        &mut self,
        host: &mut WasmHost<HeadlessBackend>,
        completion: worker::Completion,
        method: u32,
    ) -> Result<(), Error> {
        let bytes = completion.result?;
        let mut payload = Vec::with_capacity(wire::CALL_BYTES + bytes.len());
        payload.extend_from_slice(&completion.call.0.to_le_bytes());
        payload.extend_from_slice(&bytes);
        host.plugins().emit(Event {
            plugin: self.plugin,
            kind: wire::OPENED,
            payload: payload.into(),
        })?;
        self.lease = Some(completion.call);
        self.pending
            .insert(completion.call, (completion.epoch, method));
        host.wake_plugins().map_err(|_| Error::Native)
    }

    fn end_lease(&mut self, host: &mut WasmHost<HeadlessBackend>) -> Result<(), Error> {
        if let Some(call) = self.lease.take() {
            self.pending.remove(&call);
            finish(host, call, Err(Error::Cancelled))?;
        }

        Ok(())
    }

    fn cancellations(
        &mut self,
        host: &mut WasmHost<HeadlessBackend>,
        incoming: Vec<CallId>,
        work: &mut ServiceWork,
    ) -> Result<(), Error> {
        for call in incoming {
            if self.pending.remove(&call).is_none() {
                if !self.drained.remove(&call) {
                    work.cancelled.push(call);
                }
                continue;
            }

            self.invalidate(host)?;
        }

        Ok(())
    }

    fn invalidate(&mut self, host: &mut WasmHost<HeadlessBackend>) -> Result<(), Error> {
        self.mailbox.clear();
        self.shared.cancel();
        self.lease = None;
        let cancelled = std::mem::take(&mut self.pending);

        for (pending, _) in cancelled {
            finish(host, pending, Err(Error::Cancelled))?;
        }

        host.wake_plugins().map_err(|_| Error::Native)
    }

    fn requests(
        &mut self,
        host: &mut WasmHost<HeadlessBackend>,
        incoming: Vec<leptoskit_bridge::Request>,
        work: &mut ServiceWork,
    ) -> Result<(), Error> {
        for request in incoming {
            if request.plugin != self.plugin {
                work.requests.push(request);
                continue;
            }

            if !matches!(host.plugins().result(request.id), Ok(None)) {
                continue;
            }

            self.drained.remove(&request.id);
            self.admit(host, request)?;
        }

        Ok(())
    }

    /// The lifetime lease is not an operation slot. Exactly one operation may
    /// coexist with it, and the finite call budget still permits awaited close.
    fn admit(
        &mut self,
        host: &mut WasmHost<HeadlessBackend>,
        request: leptoskit_bridge::Request,
    ) -> Result<(), Error> {
        let starting = wire::starting(request.method.0);
        let occupied = self.pending.keys().any(|call| Some(*call) != self.lease);
        let exhausted = self.calls >= MAX_CALLS && request.method.0 != wire::CLOSE;

        if occupied
            || (starting && self.lease.is_some())
            || exhausted
            || request.payload.len() > MAX_BYTES
        {
            return finish(host, request.id, Err(Error::Busy));
        }

        if !starting && self.lease.is_none() {
            return finish(host, request.id, Err(Error::Unavailable));
        }

        let call = request.id;
        let method = request.method.0;
        let epoch = self.shared.epoch();

        if let Err(error) = self.mailbox.send(worker::Command { request, epoch }) {
            return finish(host, call, Err(error));
        }

        self.calls = self.calls.saturating_add(1);
        self.pending.insert(call, (epoch, method));
        Ok(())
    }
}

impl WalletService {
    fn check(&self, host: &mut WasmHost<HeadlessBackend>) -> Result<(), Error> {
        if self.interpreter != host.identity() || self.bridge != host.plugins().identity() {
            return Err(Error::InvalidCall);
        }

        Ok(())
    }

    /// Routes one supplied batch, preserving unrelated work in its input order.
    /// The composite caller owns raw Bridge draining and quantum overflow. Use
    /// `AppService::poll` consistently when this service is the standalone owner.
    ///
    /// Returns `InvalidCall` for another host or a held standalone quantum.
    /// Finish standalone polling before changing the application's draining owner.
    pub fn route(
        &mut self,
        host: &mut WasmHost<HeadlessBackend>,
        context: &mut Context<'_>,
        incoming: ServiceWork,
    ) -> Result<ServiceWork, Error> {
        self.check(host)?;
        self.shared.watch(context.waker())?;

        if self.shared.stopped() {
            return Err(Error::Native);
        }

        if !self.queued.requests.is_empty() || !self.queued.cancelled.is_empty() {
            return Err(Error::InvalidCall);
        }

        let mut work = ServiceWork::default();
        self.drained.extend(
            incoming
                .requests
                .iter()
                .filter(|request| request.plugin == self.plugin)
                .map(|request| request.id),
        );
        self.cancellations(host, incoming.cancelled, &mut work)?;
        self.deliver(host)?;
        self.requests(host, incoming.requests, &mut work)?;
        self.mailbox.flush(context)?;
        Ok(work)
    }

    fn append_queued(&mut self, incoming: ServiceWork) -> ServiceWork {
        let mut queued = std::mem::take(&mut self.queued);
        queued.requests.extend(incoming.requests);
        queued.cancelled.extend(incoming.cancelled);
        queued
    }
}

impl AppService for WalletService {
    fn poll(
        &mut self,
        host: &mut WasmHost<HeadlessBackend>,
        context: &mut Context<'_>,
    ) -> Result<ServiceWork, Error> {
        self.check(host)?;
        self.shared.watch(context.waker())?;

        if self.shared.stopped() {
            return Err(Error::Native);
        }

        let mut incoming = self.append_queued(drain(host));
        let mut work = ServiceWork::default();

        for _ in 0..MAX_ROUTE_TURNS {
            let mut routed = self.route(host, context, incoming)?;
            work.requests.append(&mut routed.requests);
            work.cancelled.append(&mut routed.cancelled);
            incoming = drain(host);

            if incoming.requests.is_empty() && incoming.cancelled.is_empty() {
                return Ok(work);
            }
        }

        // Keep the next quantum owned until the standalone service is polled again.
        self.queued = incoming;
        context.waker().wake_by_ref();
        Ok(work)
    }

    fn close(mut self: Box<Self>) -> ServiceClose {
        self.mailbox.clear();
        self.pending.clear();
        self.shared.stop();
        let shared = Arc::clone(&self.shared);
        let mut thread = self.thread.take();
        Box::pin(std::future::poll_fn(move |context| {
            poll_join(&shared, &mut thread, context)
        }))
    }
}

fn drain(host: &mut WasmHost<HeadlessBackend>) -> ServiceWork {
    ServiceWork {
        cancelled: std::iter::from_fn(|| host.plugins().next_cancelled()).collect(),
        requests: std::iter::from_fn(|| host.plugins().next_request()).collect(),
    }
}

/// Even a poisoned wake state must wait for the actual worker join before an
/// error becomes Ready, because ClosingApp then releases the interpreter.
fn poll_join(
    shared: &worker::Shared,
    thread: &mut Option<JoinHandle<()>>,
    context: &mut Context<'_>,
) -> Poll<Result<(), Error>> {
    let watch_failed = shared.watch(context.waker()).is_err();

    if !shared.stopped() {
        if watch_failed {
            context.waker().wake_by_ref();
        }

        return Poll::Pending;
    }

    match thread.take().map(JoinHandle::join) {
        Some(Ok(())) | None if !watch_failed => Poll::Ready(Ok(())),
        _ => Poll::Ready(Err(Error::Native)),
    }
}

fn finish(
    host: &mut WasmHost<HeadlessBackend>,
    call: CallId,
    result: Result<Vec<u8>, Error>,
) -> Result<(), Error> {
    match host.plugins().complete(call, result.map(Into::into)) {
        Ok(()) => host.wake_plugins().map_err(|_| Error::Native),
        Err(Error::InvalidCall) => Ok(()),
        Err(error) => Err(error),
    }
}

impl Drop for WalletService {
    fn drop(&mut self) {
        self.shared.stop();
    }
}
