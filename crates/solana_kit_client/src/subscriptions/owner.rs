//! One mutable owner serializes registration, acknowledgements and bounded queues.
use super::SubscriptionMethod;
use crate::{Error, Limits, Presence};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

/// Resource budgets for one connection generation.
#[derive(Debug, Clone, Copy)]
pub struct Budget {
    /// Shared JSON byte and depth limits for every incoming and outgoing frame.
    pub frame: Limits,
    /// Maximum registrations, including cancelled registrations awaiting replies.
    pub subscriptions: usize,
    /// Maximum queued frames, including early notifications and outbound frames.
    pub messages: usize,
    /// Maximum total retained wire bytes across all queues.
    pub queued_bytes: usize,
}
impl Default for Budget {
    fn default() -> Self {
        Self {
            frame: Limits::default(),
            subscriptions: 64,
            messages: 256,
            queued_bytes: 1024 * 1024,
        }
    }
}
/// An explicit terminal or caller error. No overflow silently drops messages.
#[derive(Debug)]
pub enum Failure {
    /// Existing shared framing or method validation failure.
    Protocol(Error),
    /// Retained queue count or bytes exceeded the configured budget.
    Overflow,
    /// Registration budget was exhausted.
    Capacity,
    /// An acknowledgement deadline expired.
    Deadline,
    /// Server IDs collided or acknowledgement identity was unknown.
    Correlation,
    /// Notification method contradicted the registered family.
    Method,
    /// The server refused to unsubscribe.
    UnsubscribeFalse,
    /// Connection generation ended, was replaced, or the handle belongs elsewhere.
    Closed,
}
impl From<Error> for Failure {
    fn from(error: Error) -> Self {
        Self::Protocol(error)
    }
}
impl std::fmt::Display for Failure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "subscription failure: {self:?}")
    }
}
impl std::error::Error for Failure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Protocol(error) => Some(error),
            _ => None,
        }
    }
}
/// Acknowledgement state, observable even while a subscription is quiet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Subscribe acknowledgement is pending.
    Subscribing,
    /// The server acknowledged this subscription.
    Active,
    /// Unsubscribe acknowledgement is pending.
    Unsubscribing,
    /// Delivery ended. Drain to observe any stored terminal failure.
    Closed,
}
/// Observable subscription state when draining notifications.
#[derive(Debug)]
pub enum Drain<T> {
    /// No notification is available yet.
    Pending,
    /// Next notification in server order.
    Notification(T),
    /// Caller cancellation or a true unsubscribe acknowledgement completed.
    Closed,
}
/// Typed registration bound to exactly one owner and generation.
#[derive(Debug)]
pub struct Subscription<R> {
    key: u32,
    generation: u64,
    owner: Arc<()>,
    request: R,
}
/// Replacement always has a possible event gap. No historical replay is promised.
#[derive(Debug, PartialEq, Eq)]
pub struct Gap {
    /// Ended generation.
    pub previous: u64,
    /// New caller-selected generation.
    pub replacement: u64,
}
#[derive(Debug)]
enum Phase {
    Subscribing {
        request: u32,
        deadline: u64,
    },
    Active(u64),
    Unsubscribing {
        request: u32,
        server: u64,
        deadline: u64,
    },
    Ended,
}
impl Phase {
    fn expired(&self, now: u64) -> bool {
        match self {
            Self::Subscribing { deadline, .. } | Self::Unsubscribing { deadline, .. } => {
                now >= *deadline
            }
            _ => false,
        }
    }

    fn server(&self) -> Option<u64> {
        match self {
            Self::Active(server) | Self::Unsubscribing { server, .. } => Some(*server),
            _ => None,
        }
    }
}
struct Entry {
    phase: Phase,
    cancelled: bool,
    unsubscribe: &'static str,
    notification: &'static str,
    queue: VecDeque<Box<RawValue>>,
    failure: Option<Failure>,
}
struct Early {
    server: u64,
    method: String,
    result: Box<RawValue>,
}
/// Deterministic protocol owner. I/O, monotonic ticks and lifecycle are explicit.
///
/// The caller supplies monotonically increasing generations for this socket
/// namespace. Handles also carry a private owner identity. Never feed a frame
/// from another socket into this owner.
/// Errors from `ingest` for the current generation terminate the connection and
/// release all queues. The caller closes that socket. Stale-generation frames
/// are rejected without terminating the replacement connection or its socket.
/// Dropping this value releases local state only.
/// `cancel` is quiet and independent of outbound queue capacity. A cancelled
/// subscribe retains bounded correlation until its acknowledgement or deadline.
/// No executor, retries, reconnect task, global registry or `Send` bound exists.
pub struct Connection {
    generation: u64,
    identity: Arc<()>,
    budget: Budget,
    next: u32,
    entries: BTreeMap<u32, Entry>,
    outbound: VecDeque<Vec<u8>>,
    early: VecDeque<Early>,
    bytes: usize,
    messages: usize,
    closed: bool,
}
impl Connection {
    /// Starts an empty caller-owned generation with explicit budgets.
    pub fn new(generation: u64, budget: Budget) -> Self {
        Self {
            generation,
            identity: Arc::new(()),
            budget,
            next: 1,
            entries: BTreeMap::new(),
            outbound: VecDeque::new(),
            early: VecDeque::new(),
            bytes: 0,
            messages: 0,
            closed: false,
        }
    }
    /// Registers before exposing its subscribe frame. Deadline is a caller tick.
    pub fn subscribe<R: SubscriptionMethod>(
        &mut self,
        request: R,
        deadline: u64,
    ) -> Result<Subscription<R>, Failure> {
        self.open()?;

        if self.entries.len() >= self.budget.subscriptions {
            return Err(Failure::Capacity);
        }

        let key = self.reserve()?;
        let body = crate::json::request(u64::from(key), &request, self.budget.frame)?.body;
        self.enqueue(body)?;
        self.entries.insert(
            key,
            Entry {
                phase: Phase::Subscribing {
                    request: key,
                    deadline,
                },
                cancelled: false,
                unsubscribe: R::UNSUBSCRIBE,
                notification: R::NOTIFICATION,
                queue: VecDeque::new(),
                failure: None,
            },
        );
        Ok(Subscription {
            key,
            generation: self.generation,
            owner: Arc::clone(&self.identity),
            request,
        })
    }
    /// Removes the next complete frame. The caller sends it once without retries.
    pub fn outbound(&mut self) -> Option<Vec<u8>> {
        let body = self.outbound.pop_front()?;
        self.release(body.len());
        Some(body)
    }
    /// Reports acknowledgement state without requiring a server notification.
    pub fn status<R>(&self, subscription: &Subscription<R>) -> Result<Status, Failure> {
        self.handle(subscription)?;
        let entry = self.entries.get(&subscription.key).ok_or(Failure::Closed)?;

        if entry.cancelled && !matches!(entry.phase, Phase::Unsubscribing { .. }) {
            return Ok(Status::Closed);
        }

        Ok(match entry.phase {
            Phase::Subscribing { .. } => Status::Subscribing,
            Phase::Active(_) => Status::Active,
            Phase::Unsubscribing { .. } => Status::Unsubscribing,
            Phase::Ended => Status::Closed,
        })
    }
    /// Ingests one complete bounded JSON text frame from this generation's socket.
    /// A failure on the current generation is terminal for this connection.
    /// A different generation returns `Failure::Closed` without terminating the
    /// replacement connection. The caller must not close its replacement socket
    /// because a stale socket supplied a frame.
    pub fn ingest(&mut self, generation: u64, body: &[u8]) -> Result<(), Failure> {
        self.open()?;

        if generation != self.generation {
            return Err(Failure::Closed);
        }

        let result = self.receive(body);

        if result.is_err() {
            self.terminate();
        }

        result
    }
    /// Drains one typed notification. Decode failure terminates this connection.
    pub fn next<R: SubscriptionMethod>(
        &mut self,
        subscription: &Subscription<R>,
    ) -> Result<Drain<R::Response>, Failure> {
        self.handle(subscription)?;
        let entry = self
            .entries
            .get_mut(&subscription.key)
            .ok_or(Failure::Closed)?;

        if let Some(error) = entry.failure.take() {
            return Err(error);
        }

        if entry.cancelled || matches!(entry.phase, Phase::Ended) {
            return Ok(Drain::Closed);
        }

        let Some(raw) = entry.queue.pop_front() else {
            return Ok(Drain::Pending);
        };
        self.release(raw.get().len());

        let value = match subscription.request.decode(raw.get()) {
            Ok(value) => value,
            Err(error) => {
                self.terminate();

                return Err(error.into());
            }
        };

        if R::completes(&value) {
            self.finish(subscription.key)?;
        }

        Ok(Drain::Notification(value))
    }
    /// Quietly cancels local delivery. Tick drives bounded remote cleanup.
    pub fn cancel<R>(&mut self, subscription: &Subscription<R>) -> Result<(), Failure> {
        self.handle(subscription)?;
        let entry = self
            .entries
            .get_mut(&subscription.key)
            .ok_or(Failure::Closed)?;
        entry.cancelled = true;
        let queue = std::mem::take(&mut entry.queue);

        for raw in queue {
            self.release(raw.get().len());
        }

        Ok(())
    }
    /// Cancels delivery and sends unsubscribe when already acknowledged.
    /// The true acknowledgement must arrive before the supplied deadline.
    /// A pending subscribe is cancelled locally; tick starts cleanup after its ack.
    pub fn unsubscribe<R>(
        &mut self,
        subscription: &Subscription<R>,
        deadline: u64,
    ) -> Result<(), Failure> {
        self.cancel(subscription)?;
        self.start_unsubscribe(subscription.key, deadline)
    }
    /// Applies caller monotonic time and starts cleanup for quiet cancellations.
    /// Missing acknowledgements become observable per-subscription deadline errors.
    pub fn tick(&mut self, now: u64, cleanup_deadline: u64) -> Result<(), Failure> {
        self.open()?;
        let keys: Vec<_> = self.entries.keys().copied().collect();

        for key in keys {
            let entry = self.entries.get_mut(&key).ok_or(Failure::Closed)?;
            let expired = entry.phase.expired(now);

            if expired {
                entry.phase = Phase::Ended;
                entry.failure = Some(Failure::Deadline);
            }

            if entry.cancelled && matches!(entry.phase, Phase::Active(_)) {
                self.start_unsubscribe(key, cleanup_deadline)?;
            }
        }

        self.discard_early_if_settled();
        Ok(())
    }
    /// Removes a terminal registration and frees its bounded slot.
    pub fn forget<R>(&mut self, subscription: Subscription<R>) -> Result<(), Failure> {
        self.handle(&subscription)?;
        let entry = self.entries.get(&subscription.key).ok_or(Failure::Closed)?;

        if !matches!(entry.phase, Phase::Ended) {
            return Err(Failure::Correlation);
        }

        let entry = self
            .entries
            .remove(&subscription.key)
            .ok_or(Failure::Closed)?;

        for raw in entry.queue {
            self.release(raw.get().len());
        }

        Ok(())
    }
    /// Closes this generation immediately and releases retained bytes.
    pub fn terminate(&mut self) {
        self.closed = true;
        self.entries.clear();
        self.outbound.clear();
        self.early.clear();
        self.bytes = 0;
        self.messages = 0;
    }
    /// Ends old handles and returns a new empty generation plus an explicit gap.
    pub fn replace(&mut self, replacement: u64) -> Result<Gap, Failure> {
        if replacement <= self.generation {
            return Err(Failure::Correlation);
        }

        let gap = Gap {
            previous: self.generation,
            replacement,
        };
        self.terminate();
        *self = Self::new(replacement, self.budget);
        Ok(gap)
    }
    fn open(&self) -> Result<(), Failure> {
        if self.closed {
            Err(Failure::Closed)
        } else {
            Ok(())
        }
    }

    fn finish(&mut self, key: u32) -> Result<(), Failure> {
        let entry = self.entries.get_mut(&key).ok_or(Failure::Closed)?;
        entry.phase = Phase::Ended;
        let queue = std::mem::take(&mut entry.queue);

        for raw in queue {
            self.release(raw.get().len());
        }

        Ok(())
    }
    fn handle<R>(&self, handle: &Subscription<R>) -> Result<(), Failure> {
        self.open()?;

        if handle.generation != self.generation || !Arc::ptr_eq(&handle.owner, &self.identity) {
            return Err(Failure::Closed);
        }

        Ok(())
    }
    fn reserve(&mut self) -> Result<u32, Failure> {
        let id = self.next;
        self.next = id
            .checked_add(1)
            .ok_or(Failure::Protocol(Error::IdExhausted))?;
        Ok(id)
    }
    fn retain(&mut self, bytes: usize) -> Result<(), Failure> {
        if self.messages >= self.budget.messages
            || bytes > self.budget.queued_bytes.saturating_sub(self.bytes)
        {
            return Err(Failure::Overflow);
        }
        self.messages += 1;
        self.bytes += bytes;
        Ok(())
    }
    fn release(&mut self, bytes: usize) {
        self.messages -= 1;
        self.bytes -= bytes;
    }
    fn enqueue(&mut self, body: Vec<u8>) -> Result<(), Failure> {
        if let Err(error) = self.retain(body.len()) {
            self.terminate();

            return Err(error);
        }

        self.outbound.push_back(body);
        Ok(())
    }
    fn start_unsubscribe(&mut self, key: u32, deadline: u64) -> Result<(), Failure> {
        let entry = self.entries.get(&key).ok_or(Failure::Closed)?;
        let Phase::Active(server) = entry.phase else {
            return Ok(());
        };
        let method = entry.unsubscribe;
        let request = self.reserve()?;
        #[derive(Serialize)]
        struct Frame {
            jsonrpc: &'static str,
            id: u32,
            method: &'static str,
            params: [u64; 1],
        }
        let body = serde_json::to_vec(&Frame {
            jsonrpc: "2.0",
            id: request,
            method,
            params: [server],
        })
        .map_err(Error::Json)?;

        if let Err(error) = crate::json::check_limits(&body, self.budget.frame) {
            self.terminate();

            return Err(error.into());
        }

        self.enqueue(body)?;
        self.entries.get_mut(&key).ok_or(Failure::Closed)?.phase = Phase::Unsubscribing {
            request,
            server,
            deadline,
        };
        Ok(())
    }
    fn receive(&mut self, body: &[u8]) -> Result<(), Failure> {
        crate::json::check_limits(body, self.budget.frame)?;
        crate::validate::unique(body).map_err(Error::Json)?;
        #[derive(Deserialize)]
        struct Kind {
            #[serde(default)]
            id: Presence<u32>,
        }
        let kind: Kind = serde_json::from_slice(body).map_err(Error::Json)?;

        match kind.id {
            Presence::Present(id) => self.ack(id, body),
            Presence::Missing => self.notification(body),
        }
    }
    fn ack(&mut self, id: u32, body: &[u8]) -> Result<(), Failure> {
        let key = self
            .entries
            .iter()
            .find_map(|(key, entry)| match entry.phase {
                Phase::Subscribing { request, .. } | Phase::Unsubscribing { request, .. }
                    if request == id =>
                {
                    Some(*key)
                }
                _ => None,
            })
            .ok_or(Failure::Correlation)?;
        let raw = crate::json::response::<Box<RawValue>>(
            u64::from(id),
            crate::Reply {
                status: 200,
                body: body.to_vec(),
            },
            self.budget.frame,
        );

        match raw {
            Err(Error::Remote(error)) => {
                let entry = self.entries.get_mut(&key).ok_or(Failure::Closed)?;
                entry.phase = Phase::Ended;
                entry.failure = Some(Failure::Protocol(Error::Remote(error)));
            }
            Err(error) => return Err(error.into()),
            Ok(raw) => self.accept(key, &raw)?,
        }

        self.discard_early_if_settled();
        Ok(())
    }
    fn accept(&mut self, key: u32, raw: &RawValue) -> Result<(), Failure> {
        let entry = self.entries.get(&key).ok_or(Failure::Closed)?;

        if matches!(entry.phase, Phase::Unsubscribing { .. }) {
            return self.accept_unsubscribe(key, raw);
        }

        let server: u64 = serde_json::from_str(raw.get()).map_err(Error::Schema)?;

        if self
            .entries
            .values()
            .any(|entry| entry.phase.server() == Some(server))
        {
            return Err(Failure::Correlation);
        }

        self.entries.get_mut(&key).ok_or(Failure::Closed)?.phase = Phase::Active(server);
        self.route_early(key, server)
    }

    fn accept_unsubscribe(&mut self, key: u32, raw: &RawValue) -> Result<(), Failure> {
        let accepted: bool = serde_json::from_str(raw.get()).map_err(Error::Schema)?;
        let entry = self.entries.get_mut(&key).ok_or(Failure::Closed)?;
        entry.phase = Phase::Ended;

        if !accepted {
            entry.failure = Some(Failure::UnsubscribeFalse);
        }

        Ok(())
    }

    fn route_early(&mut self, key: u32, server: u64) -> Result<(), Failure> {
        let buffered = std::mem::take(&mut self.early);

        for early in buffered {
            if early.server != server {
                self.early.push_back(early);

                continue;
            }

            self.bytes -= early.method.len();
            self.route(key, &early.method, early.result)?;
        }

        Ok(())
    }
    // Unknown server IDs are retained only while subscribe replies can identify them.
    fn notification(&mut self, body: &[u8]) -> Result<(), Failure> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Frame {
            jsonrpc: String,
            method: String,
            params: Params,
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Params {
            subscription: u64,
            result: Box<RawValue>,
        }
        let frame: Frame = serde_json::from_slice(body).map_err(Error::Json)?;

        if frame.jsonrpc != "2.0" {
            return Err(Failure::Protocol(Error::Envelope));
        }

        let key = self.entries.iter().find_map(|(key, entry)| {
            (entry.phase.server() == Some(frame.params.subscription)).then_some(*key)
        });

        if let Some(key) = key {
            self.retain(frame.params.result.get().len())?;

            return self.route(key, &frame.method, frame.params.result);
        }

        if self
            .entries
            .values()
            .any(|e| matches!(e.phase, Phase::Subscribing { .. }))
        {
            self.retain(frame.params.result.get().len() + frame.method.len())?;
            self.early.push_back(Early {
                server: frame.params.subscription,
                method: frame.method,
                result: frame.params.result,
            });
        }
        Ok(())
    }
    fn route(&mut self, key: u32, method: &str, raw: Box<RawValue>) -> Result<(), Failure> {
        let entry = self.entries.get_mut(&key).ok_or(Failure::Closed)?;

        if entry.notification != method {
            return Err(Failure::Method);
        }

        if entry.cancelled {
            self.release(raw.get().len());

            return Ok(());
        }

        entry.queue.push_back(raw);
        Ok(())
    }
    fn discard_early_if_settled(&mut self) {
        if self
            .entries
            .values()
            .any(|e| matches!(e.phase, Phase::Subscribing { .. }))
        {
            return;
        }
        let early = std::mem::take(&mut self.early);

        for frame in early {
            self.release(frame.result.get().len() + frame.method.len());
        }
    }
}
