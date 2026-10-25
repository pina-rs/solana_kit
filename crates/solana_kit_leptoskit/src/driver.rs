//! Task state never owns an interpreter or retains a borrow across native I/O.
use crate::{rest_wire, wire};
use leptoskit_bridge::{BridgeIdentity, Bytes, CallId, PluginId, Request};
use leptoskit_native::NativeBackend;
use leptoskit_wasm::{HostIdentity, WasmHost};
use solana_kit_client::{Transport, TransportError, TransportFailureKind};
use solana_kit_native::{NativeHttp, NativeRest};
use std::{collections::BTreeMap, sync::Arc};
use tokio::task::{AbortHandle, Id, JoinSet};

#[derive(Debug)]
/// Setup, ownership, task or actual interpreter failure.
pub enum HostError {
    /// Limits must be positive and fit the versioned wire envelope.
    Limits,
    /// The interpreter, Bridge or completion belongs to a different owner.
    Owner,
    /// Explicit shutdown has closed admission and completion delivery.
    Closed,
    /// The owning wrapper has unloaded its interpreter.
    Unloaded,
    /// Reserved instance exhaustion failure from earlier owning adapters.
    Exhausted,
    /// A native task panicked or violated its owned accounting.
    Task,
    /// The actual Bridge rejected registration, grants or completion.
    Bridge(leptoskit_bridge::Error),
    /// The actual interpreter rejected guest execution.
    Guest(leptoskit_wasm::WasmError),
}

impl std::fmt::Display for HostError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "LeptosKit HTTP host failure: {self:?}")
    }
}

impl std::error::Error for HostError {}

impl From<leptoskit_bridge::Error> for HostError {
    fn from(value: leptoskit_bridge::Error) -> Self {
        Self::Bridge(value)
    }
}

impl From<leptoskit_wasm::WasmError> for HostError {
    fn from(value: leptoskit_wasm::WasmError) -> Self {
        Self::Guest(value)
    }
}

/// Host admission ceilings independent of guest-provided receive budgets.
#[derive(Debug, Clone, Copy)]
pub struct HostLimits {
    /// Running and aborting native jobs together consume this capacity.
    pub jobs: usize,
    /// Maximum RPC JSON body or complete REST frame before starting native work.
    pub request_bytes: usize,
    /// Maximum received body, including non-success responses.
    pub reply_bytes: usize,
}

/// Unrelated plugin work returned intact to the application router.
#[derive(Debug, Default)]
pub struct Routed {
    /// Requests belonging to another application service.
    pub requests: Vec<Request>,
    /// Cancellation notifications belonging to another application service.
    pub cancelled: Vec<CallId>,
    /// Immediate replies entered the guest; drain again before waiting.
    pub guest_woken: bool,
}

struct Job {
    call: CallId,
    abort: AbortHandle,
}

#[derive(Clone)]
enum HttpService {
    Rpc(Arc<NativeHttp>),
    Rest(Arc<NativeRest<()>>),
}

/// Owned and deliberately neither Clone nor Copy. Joining releases all capacity.
///
/// Dropping this value abandons delivery without retaining a native job or permit.
/// A completion can only be delivered by its original driver and actual owners.
///
/// ```compile_fail,E0382
/// use solana_kit_leptoskit::{HttpCompletion, HttpDriver};
/// use leptoskit_native::HeadlessBackend;
/// use leptoskit_wasm::WasmHost;
/// fn duplicate(driver: &mut HttpDriver, host: &mut WasmHost<HeadlessBackend>, value: HttpCompletion) {
///     driver.deliver(host, value).unwrap();
///     driver.deliver(host, value).unwrap();
/// }
/// ```
pub struct HttpCompletion {
    owner: Arc<()>,
    interpreter: HostIdentity,
    bridge: BridgeIdentity,
    result: Option<(CallId, Vec<u8>)>,
}

/// Owns fixed HTTP services and native tasks while the caller owns its interpreter.
///
/// Construction, installation, draining and delivery borrow the actual host only
/// synchronously. Completion waits retain no interpreter reference or pointer.
/// Both interpreter and Bridge identities bind every operation, including after
/// a public Bridge transplant. Plugin Events remain application-owned.
/// Explicit shutdown aborts and joins tasks. Drop aborts without awaiting cleanup.
pub struct HttpDriver {
    owner: Arc<()>,
    interpreter: HostIdentity,
    bridge: BridgeIdentity,
    limits: HostLimits,
    services: BTreeMap<PluginId, HttpService>,
    jobs: BTreeMap<Id, Job>,
    tasks: JoinSet<(CallId, Vec<u8>)>,
    closed: bool,
}

impl HttpDriver {
    /// Binds an actual host and its current Bridge without retaining either owner.
    pub fn new<B: NativeBackend>(
        host: &mut WasmHost<B>,
        limits: HostLimits,
    ) -> Result<Self, HostError> {
        if limits.jobs == 0
            || limits.request_bytes == 0
            || limits.reply_bytes == 0
            || limits.reply_bytes > u32::MAX as usize - 3
        {
            return Err(HostError::Limits);
        }

        Ok(Self {
            owner: Arc::new(()),
            interpreter: host.identity(),
            bridge: host.plugins().identity(),
            limits,
            services: BTreeMap::new(),
            jobs: BTreeMap::new(),
            tasks: JoinSet::new(),
            closed: false,
        })
    }

    fn check<B: NativeBackend>(&self, host: &mut WasmHost<B>) -> Result<(), HostError> {
        if self.closed {
            return Err(HostError::Closed);
        }

        if self.interpreter != host.identity() || self.bridge != host.plugins().identity() {
            return Err(HostError::Owner);
        }

        Ok(())
    }

    /// Registers fixed host endpoint, authentication, TLS and deadline policy.
    pub fn install<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        name: &str,
        service: NativeHttp,
        granted: bool,
    ) -> Result<PluginId, HostError> {
        self.check(host)?;
        let plugin = host.plugins().register(name, [wire::POST])?;
        host.plugins().grant(plugin, wire::POST, granted)?;
        self.services
            .insert(plugin, HttpService::Rpc(Arc::new(service)));
        Ok(plugin)
    }

    /// Counts running and aborting jobs until their join is observed.
    pub fn retained_jobs(&self) -> usize {
        self.jobs.len()
    }

    /// Installs a named REST capability with fixed native roots and policy.
    ///
    /// Method1 is additive. Existing POST method0 and its wire remain unchanged.
    /// Guest path/query/body bytes cannot grant or select another destination.
    pub fn install_rest<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        name: &str,
        service: NativeRest<()>,
        granted: bool,
    ) -> Result<PluginId, HostError> {
        self.check(host)?;
        let plugin = host.plugins().register(name, [rest_wire::REST])?;
        host.plugins().grant(plugin, rest_wire::REST, granted)?;
        self.services
            .insert(plugin, HttpService::Rest(Arc::new(service)));
        Ok(plugin)
    }

    #[cfg(test)]
    pub(super) fn has_finished_job(&self) -> bool {
        self.jobs.values().any(|job| job.abort.is_finished())
    }

    fn abort_call(&self, call: CallId) -> bool {
        let Some(job) = self.jobs.values().find(|job| job.call == call) else {
            return false;
        };
        job.abort.abort();
        true
    }

    /// Drains cancellation first, admits bounded HTTP and returns other work.
    pub fn drain<B: NativeBackend>(&mut self, host: &mut WasmHost<B>) -> Result<Routed, HostError> {
        self.check(host)?;
        let mut incoming = Routed::default();

        while let Some(call) = host.plugins().next_cancelled() {
            incoming.cancelled.push(call);
        }

        while let Some(request) = host.plugins().next_request() {
            incoming.requests.push(request);
        }

        self.route(host, incoming)
    }

    /// Admits already-drained HTTP work and preserves other services' ownership.
    /// Use this after another service wakes a guest that submits an HTTP call.
    pub fn route<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        incoming: Routed,
    ) -> Result<Routed, HostError> {
        self.check(host)?;
        let mut routed = Routed {
            guest_woken: incoming.guest_woken,
            ..Routed::default()
        };
        let mut wake_guest = false;

        for call in incoming.cancelled {
            if !self.abort_call(call) {
                routed.cancelled.push(call);
            }
        }

        for request in incoming.requests {
            let Some(service) = self.services.get(&request.plugin).cloned() else {
                routed.requests.push(request);
                continue;
            };

            if !matches!(host.plugins().result(request.id), Ok(None)) {
                continue;
            }

            wake_guest |= match service {
                HttpService::Rpc(service) => self.admit(host, request, service)?,
                HttpService::Rest(service) => self.admit_rest(host, request, service)?,
            };
        }

        if wake_guest {
            host.wake_plugins()?;
        }

        routed.guest_woken |= wake_guest;
        Ok(routed)
    }

    // Admission validates the guest envelope before reserving a native job or task.
    fn admit<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        request: Request,
        service: Arc<NativeHttp>,
    ) -> Result<bool, HostError> {
        if request.method != wire::POST || self.jobs.len() >= self.limits.jobs {
            host.plugins()
                .complete(request.id, Err(leptoskit_bridge::Error::Busy))?;

            return Ok(true);
        }

        let decoded = if request.payload.len() > self.limits.request_bytes.saturating_add(4) {
            Err(TransportError::new(TransportFailureKind::BodyLimit))
        } else {
            wire::parse_request(&request.payload)
        };

        let decoded = decoded.and_then(|request| {
            if request.body.is_empty()
                || request.body.len() > self.limits.request_bytes
                || request.max_reply_bytes == 0
                || request.max_reply_bytes > self.limits.reply_bytes
            {
                return Err(TransportError::new(TransportFailureKind::BodyLimit));
            }

            Ok(request)
        });

        let decoded = match decoded {
            Ok(request) => request,
            Err(error) => {
                host.plugins()
                    .complete(request.id, Ok(wire::reply(Err(error)).into()))?;
                return Ok(true);
            }
        };

        let call = request.id;
        let abort = self
            .tasks
            .spawn(async move { (call, wire::reply(service.exchange(decoded).await)) });
        self.jobs.insert(abort.id(), Job { call, abort });
        Ok(false)
    }

    // Malformed REST calls complete synchronously so they never consume native capacity.
    fn admit_rest<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        request: Request,
        service: Arc<NativeRest<()>>,
    ) -> Result<bool, HostError> {
        if request.method != rest_wire::REST || self.jobs.len() >= self.limits.jobs {
            host.plugins()
                .complete(request.id, Err(leptoskit_bridge::Error::Busy))?;

            return Ok(true);
        }

        let decoded = if request.payload.len() > self.limits.request_bytes {
            Err(TransportError::new(TransportFailureKind::BodyLimit))
        } else {
            rest_wire::decode(&request.payload)
        }
        .and_then(|decoded| {
            if decoded.max_reply_bytes == 0 || decoded.max_reply_bytes > self.limits.reply_bytes {
                return Err(TransportError::new(TransportFailureKind::BodyLimit));
            }

            Ok(decoded)
        });

        let decoded = match decoded {
            Ok(decoded) => decoded,
            Err(error) => {
                host.plugins()
                    .complete(request.id, Ok(wire::reply(Err(error)).into()))?;
                return Ok(true);
            }
        };

        let call = request.id;
        let abort = self.tasks.spawn(async move {
            let reply =
                solana_kit_client::rest::Transport::exchange(service.as_ref(), decoded).await;
            (call, wire::reply(reply))
        });
        self.jobs.insert(abort.id(), Job { call, abort });
        Ok(false)
    }

    /// Joins one native completion without borrowing the interpreter.
    ///
    /// Idle waits remain Pending, so an ordinary select loop never spins. Joining
    /// releases capacity before returning the opaque completion. Cancellation of
    /// this wait does not discard a completion or remove unjoined accounting.
    pub async fn next_completion(&mut self) -> Result<HttpCompletion, HostError> {
        if self.closed {
            return Err(HostError::Closed);
        }

        if self.tasks.is_empty() {
            std::future::pending::<()>().await;
        }

        let result = self
            .tasks
            .join_next_with_id()
            .await
            .ok_or(HostError::Task)?;

        let result = match result {
            Ok((id, result)) => Some(self.join_success(id, result)?),
            Err(error) => self.join_failure(error)?,
        };

        Ok(HttpCompletion {
            owner: Arc::clone(&self.owner),
            interpreter: self.interpreter.clone(),
            bridge: self.bridge.clone(),
            result,
        })
    }

    fn join_success(
        &mut self,
        id: Id,
        result: (CallId, Vec<u8>),
    ) -> Result<(CallId, Vec<u8>), HostError> {
        let job = self.jobs.remove(&id).ok_or(HostError::Task)?;

        if job.call != result.0 {
            return Err(HostError::Task);
        }

        Ok(result)
    }

    fn join_failure(
        &mut self,
        error: tokio::task::JoinError,
    ) -> Result<Option<(CallId, Vec<u8>)>, HostError> {
        self.jobs.remove(&error.id()).ok_or(HostError::Task)?;

        if !error.is_cancelled() {
            return Err(HostError::Task);
        }

        Ok(None)
    }

    /// Consumes one completion and wakes its original live guest when still valid.
    ///
    /// Cancelled calls and dropped guest work discard late replies. Foreign owner
    /// rejection occurs before touching the supplied Bridge or entering its guest.
    pub fn deliver<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        completion: HttpCompletion,
    ) -> Result<(), HostError> {
        self.check(host)?;

        if !Arc::ptr_eq(&self.owner, &completion.owner)
            || self.interpreter != completion.interpreter
            || self.bridge != completion.bridge
        {
            return Err(HostError::Owner);
        }

        let Some((call, bytes)) = completion.result else {
            return Ok(());
        };

        match host.plugins().complete(call, Ok(Bytes::from(bytes))) {
            Ok(()) => host.wake_plugins()?,
            Err(leptoskit_bridge::Error::InvalidCall) => {}
            Err(error) => return Err(error.into()),
        }

        Ok(())
    }

    /// Closes admission, aborts and joins all native I/O while leaving the host alive.
    pub async fn shutdown(&mut self) -> Result<(), HostError> {
        self.closed = true;
        self.tasks.abort_all();
        let mut panicked = false;

        while let Some(result) = self.tasks.join_next_with_id().await {
            let id = match result {
                Ok((id, _)) => id,
                Err(error) => {
                    panicked |= !error.is_cancelled();
                    error.id()
                }
            };

            self.jobs.remove(&id).ok_or(HostError::Task)?;
        }

        if panicked {
            return Err(HostError::Task);
        }

        Ok(())
    }
}
