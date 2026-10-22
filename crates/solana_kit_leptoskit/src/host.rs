//! One instance owner retains task accounting through actual abort and join.
use crate::wire;
use leptoskit_bridge::{Bytes, CallId, MethodId, PluginId, Request};
use leptoskit_native::NativeBackend;
use leptoskit_wasm::{WasmError, WasmHost};
use solana_kit_client::{Transport, TransportError, TransportFailureKind};
use solana_kit_native::NativeHttp;
use std::{
    collections::BTreeMap,
    error::Error,
    fmt,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};
use tokio::task::{AbortHandle, Id, JoinSet};

static NEXT_INSTANCE: AtomicU64 = AtomicU64::new(1);

fn next_instance() -> Result<u64, HostError> {
    let mut current = NEXT_INSTANCE.load(Ordering::Relaxed);

    loop {
        let next = current.checked_add(1).ok_or(HostError::Exhausted)?;

        match NEXT_INSTANCE.compare_exchange_weak(
            current,
            next,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(instance) => return Ok(instance),
            Err(actual) => current = actual,
        }
    }
}

/// Host-owned admission ceilings, independent of guest-provided receive budgets.
#[derive(Debug, Clone, Copy)]
pub struct HostLimits {
    /// Running and aborting native jobs together consume this capacity.
    pub jobs: usize,
    /// Maximum JSON body accepted before starting native work.
    pub request_bytes: usize,
    /// Maximum received response body, including non-success responses.
    pub reply_bytes: usize,
}

/// Host setup, lifecycle or actual guest entry-point failure.
#[derive(Debug)]
pub enum HostError {
    /// Limits must be positive and fit the versioned wire envelope.
    Limits,
    /// Instance identities never wrap or reuse.
    Exhausted,
    /// The guest has already been unloaded.
    Unloaded,
    /// The actual upstream interpreter rejected guest execution.
    Guest(WasmError),
    /// The actual bridge rejected registration, grants or completion.
    Bridge(leptoskit_bridge::Error),
    /// An owned native task panicked rather than completing normally.
    Task,
}

impl fmt::Display for HostError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "LeptosKit HTTP host failure: {self:?}")
    }
}
impl Error for HostError {}
impl From<WasmError> for HostError {
    fn from(error: WasmError) -> Self {
        Self::Guest(error)
    }
}
impl From<leptoskit_bridge::Error> for HostError {
    fn from(error: leptoskit_bridge::Error) -> Self {
        Self::Bridge(error)
    }
}

/// Work for other installed plugins, returned intact to the application router.
#[derive(Debug, Default)]
pub struct Routed {
    /// Non-HTTP requests; the adapter never completes or discards these.
    pub requests: Vec<Request>,
    /// Non-HTTP cancellation notifications owned by another application service.
    pub cancelled: Vec<CallId>,
    /// Immediate admission errors entered the guest; drain again before waiting.
    pub guest_woken: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Token {
    instance: u64,
    call: CallId,
}
struct Job {
    token: Token,
    abort: AbortHandle,
}

/// Actual interpreter plus all native HTTP tasks for one non-reused instance.
///
/// Applications route unrelated requests returned by `drain`. Plugin Events stay
/// entirely application-owned. No callback carries a guest memory pointer.
pub struct HttpHost<B: NativeBackend> {
    host: Option<WasmHost<B>>,
    instance: u64,
    limits: HostLimits,
    services: BTreeMap<PluginId, Arc<NativeHttp>>,
    jobs: BTreeMap<Id, Job>,
    tasks: JoinSet<(Token, Vec<u8>)>,
}

impl<B: NativeBackend> HttpHost<B> {
    /// Takes exclusive ownership of an unstarted actual host with explicit limits.
    pub fn new(host: WasmHost<B>, limits: HostLimits) -> Result<Self, HostError> {
        if limits.jobs == 0
            || limits.request_bytes == 0
            || limits.reply_bytes == 0
            || limits.reply_bytes > u32::MAX as usize - 3
        {
            return Err(HostError::Limits);
        }
        let instance = next_instance()?;
        Ok(Self {
            host: Some(host),
            instance,
            limits,
            services: BTreeMap::new(),
            jobs: BTreeMap::new(),
            tasks: JoinSet::new(),
        })
    }

    /// Binds a name to a fixed host-configured endpoint/auth/TLS/deadline policy.
    pub fn install(
        &mut self,
        name: &str,
        service: NativeHttp,
        granted: bool,
    ) -> Result<PluginId, HostError> {
        let id = self.host_mut()?.plugins().register(name, [wire::POST])?;
        self.host_mut()?.plugins().grant(id, wire::POST, granted)?;
        self.services.insert(id, Arc::new(service));
        Ok(id)
    }

    /// Accesses the owned UI host for application events; never replace its bridge.
    pub fn host_mut(&mut self) -> Result<&mut WasmHost<B>, HostError> {
        self.host.as_mut().ok_or(HostError::Unloaded)
    }

    /// Starts the guest; call `drain` after this or any application guest entry.
    pub fn start(&mut self, width: f32, height: f32) -> Result<(), HostError> {
        self.host_mut()?.start(width, height)?;
        Ok(())
    }

    /// Counts running and aborting tasks until their join is actually observed.
    pub fn retained_jobs(&self) -> usize {
        self.jobs.len()
    }

    /// Drains cancellation first, then admits bounded HTTP and returns other work.
    pub fn drain(&mut self) -> Result<Routed, HostError> {
        let mut routed = Routed::default();
        let mut ready = false;

        while let Some(call) = self.host_mut()?.plugins().next_cancelled() {
            if !self.abort_call(call) {
                routed.cancelled.push(call);
            }
        }

        while let Some(request) = self.host_mut()?.plugins().next_request() {
            if !self.services.contains_key(&request.plugin) {
                routed.requests.push(request);
                continue;
            }

            ready |= self.admit(request)?;
        }

        if ready {
            self.host_mut()?.wake_plugins()?;
            routed.guest_woken = true;
        }

        Ok(routed)
    }

    fn abort_call(&self, call: CallId) -> bool {
        let Some(job) = self.jobs.values().find(|job| job.token.call == call) else {
            return false;
        };
        job.abort.abort();

        true
    }

    fn admit(&mut self, request: Request) -> Result<bool, HostError> {
        let token = Token {
            instance: self.instance,
            call: request.id,
        };
        let decoded = if request.payload.len() > self.limits.request_bytes.saturating_add(4) {
            Err(TransportError::new(TransportFailureKind::BodyLimit))
        } else {
            wire::parse_request(&request.payload)
        };

        let decoded = decoded.and_then(|decoded| self.check_request(decoded));
        let service = Arc::clone(self.services.get(&request.plugin).ok_or(HostError::Task)?);

        if request.method != MethodId(0) || self.jobs.len() >= self.limits.jobs {
            self.host_mut()?
                .plugins()
                .complete(request.id, Err(leptoskit_bridge::Error::Busy))?;

            return Ok(true);
        }

        let decoded = match decoded {
            Ok(decoded) => decoded,
            Err(error) => {
                self.host_mut()?
                    .plugins()
                    .complete(request.id, Ok(wire::reply(Err(error)).into()))?;
                return Ok(true);
            }
        };

        let abort = self
            .tasks
            .spawn(async move { (token, wire::reply(service.exchange(decoded).await)) });
        self.jobs.insert(abort.id(), Job { token, abort });
        Ok(false)
    }

    fn check_request(
        &self,
        request: solana_kit_client::Request,
    ) -> Result<solana_kit_client::Request, TransportError> {
        if request.body.is_empty()
            || request.body.len() > self.limits.request_bytes
            || request.max_reply_bytes == 0
            || request.max_reply_bytes > self.limits.reply_bytes
        {
            return Err(TransportError::new(TransportFailureKind::BodyLimit));
        }
        Ok(request)
    }

    /// Awaits one real native completion or abort, then wakes registered guest work.
    ///
    /// With no jobs this returns false immediately. The application must select
    /// other UI/event sources separately; this method never consumes guest Events.
    pub async fn next_completion(&mut self) -> Result<bool, HostError> {
        let Some(result) = self.tasks.join_next_with_id().await else {
            return Ok(false);
        };

        let (id, (token, bytes)) = match result {
            Ok(completion) => completion,
            Err(error) => return self.join_failure(error),
        };

        let job = self.jobs.remove(&id).ok_or(HostError::Task)?;

        if token != job.token || token.instance != self.instance {
            return Err(HostError::Task);
        }

        let result = self
            .host_mut()?
            .plugins()
            .complete(token.call, Ok(Bytes::from(bytes)));

        match result {
            Ok(()) => {}
            Err(leptoskit_bridge::Error::InvalidCall) => return Ok(true),
            Err(error) => return Err(error.into()),
        }

        self.host_mut()?.wake_plugins()?;

        Ok(true)
    }

    fn join_failure(&mut self, error: tokio::task::JoinError) -> Result<bool, HostError> {
        self.jobs.remove(&error.id());

        if error.is_cancelled() {
            return Ok(true);
        }

        Err(HostError::Task)
    }

    /// Aborts and joins all actual I/O before removing accounting and unloading.
    pub async fn shutdown(&mut self) -> Result<(), HostError> {
        self.tasks.abort_all();
        let mut panicked = false;

        while let Some(result) = self.tasks.join_next_with_id().await {
            match result {
                Ok((id, _)) => {
                    self.jobs.remove(&id);
                }
                Err(error) => {
                    self.jobs.remove(&error.id());
                    panicked |= !error.is_cancelled();
                }
            }
        }

        self.host.take();

        if panicked {
            return Err(HostError::Task);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
