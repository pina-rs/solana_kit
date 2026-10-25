//! Fixed-grant socket ownership composed with the HTTP router's returned work.
use crate::{
    HostError, Routed,
    socket_actor::{self as actor, Command, Exit, Notice},
    socket_wire::{self as wire, SocketFailure},
};
use leptoskit_bridge::{BridgeIdentity, CallId, Event, PluginId, Request};
use leptoskit_native::NativeBackend;
use leptoskit_wasm::{HostIdentity, WasmHost};
use solana_kit_native::WsConfig;
use std::{
    collections::BTreeMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::{
    sync::mpsc,
    task::{AbortHandle, Id, JoinSet},
};

/// Native ceilings. Queued text is bounded by messages times frame_bytes.
#[derive(Debug, Clone, Copy)]
pub struct SocketLimits {
    /// Running and aborting sockets together consume capacity.
    pub sockets: usize,
    /// Global native notice and per-socket command count ceiling.
    pub messages: usize,
    /// Incoming messages and outgoing UTF-8 text ceiling.
    pub frame_bytes: usize,
}

struct Grant {
    endpoint: String,
    config: WsConfig,
}
struct Socket {
    lease: CallId,
    plugin: PluginId,
    abort: AbortHandle,
    commands: mpsc::Sender<Command>,
    live: Arc<AtomicBool>,
}

enum Delivery {
    Event(u8, Vec<u8>),
    Reply(CallId, Result<(), SocketFailure>),
    End(End),
}

struct End {
    close: Option<CallId>,
    result: Result<(), SocketFailure>,
    pending: Vec<CallId>,
}

enum Ready {
    Notice(Option<Notice>),
    Joined(Option<Result<(Id, Exit), tokio::task::JoinError>>),
}

/// Owned single-use native result, bound to driver, interpreter and Bridge.
pub struct SocketCompletion {
    owner: Arc<()>,
    interpreter: HostIdentity,
    bridge: BridgeIdentity,
    lease: CallId,
    plugin: PluginId,
    live: Arc<AtomicBool>,
    delivery: Delivery,
}

/// Owns bounded socket actors; all actual-host borrows are synchronous.
///
/// Pass `HttpDriver::drain`'s Routed value to `route`. The application retains
/// unrelated work and its sole guest Events receiver. Cancellation closes even
/// an idle socket and capacity remains occupied through actual task join.
/// Await shutdown before unloading the interpreter. Drop aborts tasks only.
pub struct SocketDriver {
    owner: Arc<()>,
    interpreter: HostIdentity,
    bridge: BridgeIdentity,
    limits: SocketLimits,
    grants: BTreeMap<PluginId, Grant>,
    sockets: BTreeMap<Id, Socket>,
    calls: BTreeMap<CallId, CallId>,
    tasks: JoinSet<Exit>,
    sender: mpsc::Sender<Notice>,
    receiver: mpsc::Receiver<Notice>,
    closed: bool,
}

impl SocketDriver {
    /// Binds both actual owners and validates finite allocation ceilings.
    pub fn new<B: NativeBackend>(
        host: &mut WasmHost<B>,
        limits: SocketLimits,
    ) -> Result<Self, HostError> {
        if limits.sockets == 0
            || limits.messages == 0
            || limits.frame_bytes == 0
            || limits.frame_bytes > u32::MAX as usize - wire::HEADER_BYTES
            || limits.messages.checked_mul(limits.frame_bytes).is_none()
            || limits.sockets.checked_mul(limits.messages).is_none()
        {
            return Err(HostError::Limits);
        }

        let (sender, receiver) = mpsc::channel(limits.messages);
        Ok(Self {
            owner: Arc::new(()),
            interpreter: host.identity(),
            bridge: host.plugins().identity(),
            limits,
            grants: BTreeMap::new(),
            sockets: BTreeMap::new(),
            calls: BTreeMap::new(),
            tasks: JoinSet::new(),
            sender,
            receiver,
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

    /// Registers immutable destination, authentication, TLS and native policy.
    pub fn install<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        name: &str,
        endpoint: String,
        config: WsConfig,
        granted: bool,
    ) -> Result<PluginId, HostError> {
        self.check(host)?;

        if config.max_frame_bytes > self.limits.frame_bytes
            || config.max_message_bytes > self.limits.frame_bytes
        {
            return Err(HostError::Limits);
        }

        let plugin = host
            .plugins()
            .register(name, [wire::OPEN, wire::SEND, wire::CLOSE])?;

        for method in [wire::OPEN, wire::SEND, wire::CLOSE] {
            host.plugins().grant(plugin, method, granted)?;
        }

        self.grants.insert(plugin, Grant { endpoint, config });
        Ok(plugin)
    }

    /// Counts connecting, live and aborting sockets until their joins are observed.
    pub fn retained_sockets(&self) -> usize {
        self.sockets.len()
    }

    /// Consumes caller-routed cancellation first, preserving unrelated work intact.
    pub fn route<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        routed: Routed,
    ) -> Result<Routed, HostError> {
        self.check(host)?;
        let mut output = Routed {
            guest_woken: routed.guest_woken,
            ..Routed::default()
        };

        for call in routed.cancelled {
            if !self.cancel(call) {
                output.cancelled.push(call);
            }
        }

        for request in routed.requests {
            if !self.grants.contains_key(&request.plugin) {
                output.requests.push(request);
                continue;
            }

            output.guest_woken |= self.admit(host, request)?;
        }

        if output.guest_woken {
            host.wake_plugins()?;
        }

        Ok(output)
    }

    fn cancel(&self, call: CallId) -> bool {
        let lease = self.calls.get(&call).copied().unwrap_or(call);
        let Some(socket) = self.sockets.values().find(|socket| socket.lease == lease) else {
            return false;
        };
        socket.live.store(false, Ordering::Release);
        socket.abort.abort();
        true
    }

    fn admit<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        request: Request,
    ) -> Result<bool, HostError> {
        if request.method == wire::OPEN {
            if !request.payload.is_empty() || self.sockets.len() >= self.limits.sockets {
                host.plugins()
                    .complete(request.id, Err(leptoskit_bridge::Error::Busy))?;

                return Ok(true);
            }

            let grant = self.grants.get(&request.plugin).ok_or(HostError::Task)?;
            let (commands, receiver) = mpsc::channel(self.limits.messages);
            let abort = self.tasks.spawn(actor::run(
                request.id,
                grant.endpoint.clone(),
                grant.config.clone(),
                receiver,
                self.sender.clone(),
            ));
            self.sockets.insert(
                abort.id(),
                Socket {
                    lease: request.id,
                    plugin: request.plugin,
                    abort,
                    commands,
                    live: Arc::new(AtomicBool::new(true)),
                },
            );

            return Ok(false);
        }

        let result = self.command(&request);

        if let Err(error) = result {
            host.plugins()
                .complete(request.id, Ok(wire::reply(Err(error)).into()))?;

            return Ok(true);
        }

        Ok(false)
    }

    /// Correlation and budgets precede allocation and bounded native queue admission.
    fn command(&mut self, request: &Request) -> Result<(), SocketFailure> {
        if request.payload.len() > self.limits.frame_bytes.saturating_add(wire::HEADER_BYTES) {
            return Err(SocketFailure::Limit);
        }

        let (lease, kind, body) = wire::parse(&request.payload)?;
        let socket = self
            .sockets
            .values()
            .find(|socket| {
                socket.lease == lease
                    && socket.plugin == request.plugin
                    && socket.live.load(Ordering::Acquire)
            })
            .ok_or(SocketFailure::Closed)?;

        if kind != 0 || self.calls.len() >= self.limits.messages {
            return Err(SocketFailure::Limit);
        }

        let command = match request.method {
            wire::SEND => Command::Send(
                request.id,
                std::str::from_utf8(body)
                    .map_err(|_| SocketFailure::Protocol)?
                    .to_owned(),
            ),

            wire::CLOSE if body.is_empty() => Command::Close(request.id),
            _ => return Err(SocketFailure::Protocol),
        };

        socket
            .commands
            .try_send(command)
            .map_err(|_| SocketFailure::Limit)?;
        self.calls.insert(request.id, lease);
        Ok(())
    }

    /// Waits on actual native I/O without retaining any host borrow or pointer.
    /// Dropping this wait preserves all queued results and join accounting.
    pub async fn next_completion(&mut self) -> Result<SocketCompletion, HostError> {
        if self.closed {
            return Err(HostError::Closed);
        }

        loop {
            let ready = tokio::select! {
                biased;
                notice = self.receiver.recv() => Ready::Notice(notice),
                joined = join_next(&mut self.tasks) => Ready::Joined(joined),
            };

            let completion = match ready {
                Ready::Notice(notice) => self.notice(notice.ok_or(HostError::Task)?),
                Ready::Joined(joined) => Some(self.join(joined.ok_or(HostError::Task)?)?),
            };

            if let Some(completion) = completion {
                return Ok(completion);
            }
        }
    }

    fn join(
        &mut self,
        joined: Result<(Id, Exit), tokio::task::JoinError>,
    ) -> Result<SocketCompletion, HostError> {
        let (id, exit) = match joined {
            Ok((id, exit)) => (id, Some(exit)),

            Err(error) if error.is_cancelled() => (error.id(), None),
            Err(_) => return Err(HostError::Task),
        };

        let socket = self.sockets.remove(&id).ok_or(HostError::Task)?;
        let pending = self
            .calls
            .iter()
            .filter_map(|(call, lease)| (*lease == socket.lease).then_some(*call))
            .collect();
        self.calls.retain(|_, lease| *lease != socket.lease);
        let exit = exit.unwrap_or(Exit {
            lease: socket.lease,
            close: None,
            result: Err(SocketFailure::Closed),
        });

        if exit.lease != socket.lease {
            return Err(HostError::Task);
        }

        Ok(self.completion(
            &socket,
            Delivery::End(End {
                close: exit.close,
                result: exit.result,
                pending,
            }),
        ))
    }

    fn notice(&mut self, notice: Notice) -> Option<SocketCompletion> {
        let (lease, delivery) = match notice {
            Notice::Opened(lease) => (lease, Delivery::Event(wire::OPENED, Vec::new())),
            Notice::Text(lease, text) => (lease, Delivery::Event(wire::TEXT, text.into_bytes())),
            Notice::Binary(lease, bytes) => (lease, Delivery::Event(wire::BINARY, bytes)),
            Notice::Reply(lease, call, result) => (lease, Delivery::Reply(call, result)),
        };

        let socket = self.sockets.values().find(|socket| socket.lease == lease)?;
        Some(self.completion(socket, delivery))
    }

    fn completion(&self, socket: &Socket, delivery: Delivery) -> SocketCompletion {
        SocketCompletion {
            owner: Arc::clone(&self.owner),
            interpreter: self.interpreter.clone(),
            bridge: self.bridge.clone(),
            lease: socket.lease,
            plugin: socket.plugin,
            live: Arc::clone(&socket.live),
            delivery,
        }
    }

    /// Delivers once on the original live interpreter and Bridge, then wakes it.
    /// Stale queued frames after cancellation are discarded before guest entry.
    pub fn deliver<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        completion: SocketCompletion,
    ) -> Result<(), HostError> {
        self.check(host)?;

        if !Arc::ptr_eq(&self.owner, &completion.owner)
            || self.interpreter != completion.interpreter
            || self.bridge != completion.bridge
        {
            return Err(HostError::Owner);
        }

        let pending = matches!(host.plugins().result(completion.lease), Ok(None));
        let live = completion.live.load(Ordering::Acquire) && pending;

        if !live {
            self.cancel(completion.lease);
        }

        match completion.delivery {
            Delivery::Event(kind, body) if live => {
                self.emit(host, completion.plugin, completion.lease, kind, &body)?;
            }

            Delivery::Reply(call, result) if live => {
                complete(host, call, result)?;
                self.calls.remove(&call);
            }

            Delivery::End(ref end) => {
                self.end(host, &completion, end, live)?;
            }
            _ => return Ok(()),
        }

        host.wake_plugins()?;
        Ok(())
    }

    fn end<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        completion: &SocketCompletion,
        end: &End,
        live: bool,
    ) -> Result<(), HostError> {
        completion.live.store(false, Ordering::Release);

        for call in &end.pending {
            if Some(*call) != end.close {
                complete(host, *call, Err(SocketFailure::Closed))?;
            }
        }

        if let Some(call) = end.close {
            complete(host, call, end.result)?;
        }

        let emitted = if live {
            self.emit(
                host,
                completion.plugin,
                completion.lease,
                wire::ENDED,
                &wire::reply(end.result),
            )
        } else {
            Ok(())
        };

        complete(host, completion.lease, end.result)?;
        emitted
    }

    fn emit<B: NativeBackend>(
        &mut self,
        host: &mut WasmHost<B>,
        plugin: PluginId,
        lease: CallId,
        kind: u8,
        body: &[u8],
    ) -> Result<(), HostError> {
        let result = host.plugins().emit(Event {
            plugin,
            kind: wire::EVENT,
            payload: wire::envelope(lease, kind, body).into(),
        });

        if let Err(error) = result {
            if let Some(socket) = self.sockets.values().find(|socket| socket.lease == lease) {
                socket.live.store(false, Ordering::Release);
                socket.abort.abort();
            }

            complete(host, lease, Err(SocketFailure::Limit))?;
            host.wake_plugins()?;

            return Err(error.into());
        }

        Ok(())
    }

    /// Closes admission and aborts and joins every connecting or live socket.
    pub async fn shutdown(&mut self) -> Result<(), HostError> {
        self.closed = true;

        for socket in self.sockets.values() {
            socket.live.store(false, Ordering::Release);
        }

        self.tasks.abort_all();
        let mut failed = false;

        while let Some(result) = self.tasks.join_next().await {
            failed |= result.is_err_and(|error| !error.is_cancelled());
        }

        self.sockets.clear();
        self.calls.clear();

        while self.receiver.try_recv().is_ok() {}

        if failed {
            return Err(HostError::Task);
        }

        Ok(())
    }
}

async fn join_next(
    tasks: &mut JoinSet<Exit>,
) -> Option<Result<(Id, Exit), tokio::task::JoinError>> {
    if tasks.is_empty() {
        std::future::pending::<()>().await;
    }

    tasks.join_next_with_id().await
}

fn complete<B: NativeBackend>(
    host: &mut WasmHost<B>,
    call: CallId,
    result: Result<(), SocketFailure>,
) -> Result<(), HostError> {
    match host
        .plugins()
        .complete(call, Ok(wire::reply(result).into()))
    {
        Ok(()) | Err(leptoskit_bridge::Error::InvalidCall) => Ok(()),
        Err(error) => Err(error.into()),
    }
}
