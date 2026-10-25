//! One mutable owner serializes registration, acknowledgements and bounded queues.
use super::{NotificationKind, SubscriptionMethod};
use crate::{Error, Limits, Presence};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::borrow::Cow;
use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::Waker;

mod binary;
mod payload;
mod sharing;
use payload::Payload;
use sharing::{PendingFailure, Physical, SharingKey};

/// Independent limits for one local consumer. Aggregate connection limits also apply.
#[derive(Debug, Clone, Copy)]
pub struct QueueBudget {
    /// Maximum retained notifications for this handle.
    pub messages: usize,
    /// Maximum retained payload bytes for this handle.
    pub bytes: usize,
}

/// Resource budgets for one connection generation.
#[derive(Debug, Clone, Copy)]
pub struct Budget {
    /// Shared JSON byte and depth limits for every incoming and outgoing frame.
    pub frame: Limits,
    /// Maximum local registrations and physical registrations, each independently.
    /// Includes cancelled registrations and uncertain-unsubscribe tombstones.
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
    /// An identical provider registration has pending or uncertain physical settlement.
    Closing,
    /// Notification kind/method contradicted a family, or binary namespaces were ambiguous.
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
    lease: Arc<LocalLease>,
}

#[derive(Debug)]
struct LocalLease {
    waker: Mutex<Option<Waker>>,
    dropped: AtomicBool,
}

impl<R> Drop for Subscription<R> {
    fn drop(&mut self) {
        self.lease.dropped.store(true, Ordering::Release);
        let waker = self
            .lease
            .waker
            .lock()
            .expect("infallible waker replacement")
            .take();

        if let Some(waker) = waker {
            waker.wake();
        }
    }
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
    physical: u32,
    deadline: u64,
    ended: bool,
    cancelled: bool,
    lease: Weak<LocalLease>,
    budget: QueueBudget,
    bytes: usize,
    queue: VecDeque<Payload>,
    failure: Option<PendingFailure>,
}
impl Entry {
    fn abandoned(&self) -> bool {
        self.lease
            .upgrade()
            .is_none_or(|lease| lease.dropped.load(Ordering::Acquire))
    }
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
    physical: BTreeMap<u32, Physical>,
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
            physical: BTreeMap::new(),
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
        self.subscribe_with_budget(
            request,
            deadline,
            QueueBudget {
                messages: self.budget.messages,
                bytes: self.budget.queued_bytes,
            },
        )
    }
    /// Registers a consumer with its own limits. Any overflow is terminal, with no partial fan-out.
    /// Identical provider work awaiting unsubscribe is rejected with `Failure::Closing`.
    /// For opted-in providers, false, failed or missing unsubscribe acknowledgement quarantines the key
    /// and reserves its known server ID until the connection is replaced.
    /// An expired shared subscribe acknowledgement also quarantines the key.
    /// Its server ID is unknown; a late acknowledgement is a terminal correlation
    /// error. Replace the connection before retrying this uncertain provider work.
    pub fn subscribe_with_budget<R: SubscriptionMethod>(
        &mut self,
        request: R,
        deadline: u64,
        budget: QueueBudget,
    ) -> Result<Subscription<R>, Failure> {
        self.open()?;

        if self.entries.len() >= self.budget.subscriptions {
            return Err(Failure::Capacity);
        }

        self.binary_namespace::<R>()?;

        let key = self.reserve()?;
        let body = crate::json::request(u64::from(key), &request, self.budget.frame)?.body;
        let sharing = SharingKey::new::<R>(&body)?;
        let physical = self.registration::<R>(key, deadline, sharing, body)?;
        let lease = Arc::new(LocalLease {
            waker: Mutex::new(None),
            dropped: AtomicBool::new(false),
        });
        self.entries.insert(
            key,
            Entry {
                physical,
                deadline,
                ended: false,
                cancelled: false,
                lease: Arc::downgrade(&lease),
                budget,
                bytes: 0,
                queue: VecDeque::new(),
                failure: None,
            },
        );
        Ok(Subscription {
            key,
            generation: self.generation,
            owner: Arc::clone(&self.identity),
            request,
            lease,
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

        let physical = self.physical.get(&entry.physical).ok_or(Failure::Closed)?;

        if entry.ended
            || (entry.cancelled && !matches!(physical.phase, Phase::Unsubscribing { .. }))
        {
            return Ok(Status::Closed);
        }

        Ok(match physical.phase {
            Phase::Subscribing { .. } => Status::Subscribing,
            Phase::Active(_) => Status::Active,
            Phase::Unsubscribing { .. } => Status::Unsubscribing,
            Phase::Ended => Status::Closed,
        })
    }
    /// Whether this local consumer has settled. Detached consumers do not close shared work.
    pub fn settled<R>(&self, subscription: &Subscription<R>) -> Result<bool, Failure> {
        self.handle(subscription)?;
        self.entries
            .get(&subscription.key)
            .map(|entry| entry.ended)
            .ok_or(Failure::Closed)
    }
    /// Earliest physical or independent local acknowledgement deadline.
    #[must_use]
    pub fn next_deadline(&self) -> Option<u64> {
        let physical = self
            .physical
            .values()
            .filter_map(|registration| match registration.phase {
                Phase::Subscribing { deadline, .. } | Phase::Unsubscribing { deadline, .. } => {
                    Some(deadline)
                }
                _ => None,
            });

        let local = self.entries.values().filter_map(|entry| {
            let registration = self.physical.get(&entry.physical)?;
            (!entry.ended && matches!(registration.phase, Phase::Subscribing { .. }))
                .then_some(entry.deadline)
        });

        physical.chain(local).min()
    }
    pub(super) fn register_waker(&self, waker: &Waker) {
        for entry in self.entries.values() {
            if let Some(lease) = entry.lease.upgrade() {
                let replacement = waker.clone();
                let previous = lease
                    .waker
                    .lock()
                    .expect("infallible waker replacement")
                    .replace(replacement);
                drop(previous);
            }
        }
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
    /// Broadcasts a complete binary frame within one explicitly named provider namespace.
    ///
    /// Binary headers need not contain a subscription ID. Only live, uncancelled
    /// registrations that explicitly accept the same namespace receive a copy.
    /// Pre-ack frames remain queued until their subscribe acknowledgement. The
    /// complete broadcast is budgeted before any payload is allocated or copied.
    /// A current-generation failure terminates this connection; stale frames do not.
    pub fn ingest_binary(&mut self, generation: u64, body: &[u8]) -> Result<(), Failure> {
        self.open()?;

        if generation != self.generation {
            return Err(Failure::Closed);
        }

        let result = self.binary(body);

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
            return Err(error.into_failure());
        }

        if entry.cancelled || entry.ended {
            return Ok(Drain::Closed);
        }

        if matches!(
            self.physical
                .get(&entry.physical)
                .ok_or(Failure::Closed)?
                .phase,
            Phase::Subscribing { .. }
        ) {
            return Ok(Drain::Pending);
        }

        let Some(raw) = entry.queue.pop_front() else {
            return Ok(Drain::Pending);
        };
        entry.bytes -= raw.len();
        self.release(raw.len());

        let value = match raw.decode(&subscription.request) {
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
        entry.bytes = 0;
        let queue = std::mem::take(&mut entry.queue);

        for raw in queue {
            self.release(raw.len());
        }

        self.detach_if_shared(subscription.key)?;

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
        let physical = self
            .entries
            .get(&subscription.key)
            .ok_or(Failure::Closed)?
            .physical;
        self.start_unsubscribe(physical, deadline)
    }
    /// Applies caller monotonic time and starts cleanup for quiet cancellations.
    /// Missing acknowledgements become observable per-subscription deadline errors.
    pub fn tick(&mut self, now: u64, cleanup_deadline: u64) -> Result<(), Failure> {
        self.open()?;
        let keys: Vec<_> = self.entries.keys().copied().collect();

        for key in keys {
            let entry = self.entries.get_mut(&key).ok_or(Failure::Closed)?;
            let registration = self.physical.get(&entry.physical).ok_or(Failure::Closed)?;

            if !entry.ended
                && matches!(registration.phase, Phase::Subscribing { .. })
                && now >= entry.deadline
            {
                entry.ended = true;
                entry.failure = Some(PendingFailure::Local(Failure::Deadline));
            }

            if entry.abandoned() && !entry.cancelled {
                entry.cancelled = true;
                entry.bytes = 0;
                let queue = std::mem::take(&mut entry.queue);

                for raw in queue {
                    self.release(raw.len());
                }
            }

            self.detach_if_shared(key)?;
        }

        self.tick_physical(now, cleanup_deadline)?;

        self.discard_early_if_settled();
        Ok(())
    }
    /// Removes a terminal registration and frees its bounded slot.
    pub fn forget<R>(&mut self, subscription: Subscription<R>) -> Result<(), Failure> {
        self.handle(&subscription)?;
        let entry = self.entries.get(&subscription.key).ok_or(Failure::Closed)?;

        if !entry.ended {
            return Err(Failure::Correlation);
        }

        let entry = self
            .entries
            .remove(&subscription.key)
            .ok_or(Failure::Closed)?;

        for raw in entry.queue {
            self.release(raw.len());
        }

        self.remove_physical_if_unused(entry.physical);

        Ok(())
    }
    /// Closes this generation immediately and releases retained bytes.
    pub fn terminate(&mut self) {
        self.closed = true;
        self.entries.clear();
        self.physical.clear();
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
        entry.ended = true;
        entry.bytes = 0;
        let physical = entry.physical;
        let queue = std::mem::take(&mut entry.queue);

        for raw in queue {
            self.release(raw.len());
        }

        if !self.has_receivers(physical) {
            self.physical
                .get_mut(&physical)
                .ok_or(Failure::Closed)?
                .phase = Phase::Ended;
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
        if self.has_receivers(key) {
            return Ok(());
        }

        let entry = self.physical.get(&key).ok_or(Failure::Closed)?;
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
        self.physical.get_mut(&key).ok_or(Failure::Closed)?.phase = Phase::Unsubscribing {
            request,
            server,
            deadline,
        };
        let entry = self.physical.get_mut(&key).ok_or(Failure::Closed)?;
        entry.quarantined = entry.sharing.is_some();
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
            .physical
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
                self.remote_error(key, error)?;
            }
            Err(error) => return Err(error.into()),
            Ok(raw) => self.accept(key, &raw)?,
        }

        self.discard_early_if_settled();
        Ok(())
    }
    fn accept(&mut self, key: u32, raw: &RawValue) -> Result<(), Failure> {
        let entry = self.physical.get(&key).ok_or(Failure::Closed)?;

        if matches!(entry.phase, Phase::Unsubscribing { .. }) {
            return self.accept_unsubscribe(key, raw);
        }

        let server: u64 = serde_json::from_str(raw.get()).map_err(Error::Schema)?;

        if self
            .physical
            .values()
            .any(|entry| entry.server() == Some(server))
        {
            return Err(Failure::Correlation);
        }

        self.physical.get_mut(&key).ok_or(Failure::Closed)?.phase = Phase::Active(server);
        self.route_early(key, server)
    }

    fn accept_unsubscribe(&mut self, key: u32, raw: &RawValue) -> Result<(), Failure> {
        let accepted: bool = serde_json::from_str(raw.get()).map_err(Error::Schema)?;
        let entry = self.physical.get_mut(&key).ok_or(Failure::Closed)?;
        entry.reserved_server = if !accepted && entry.sharing.is_some() {
            entry.phase.server()
        } else {
            None
        };

        entry.phase = Phase::Ended;
        entry.quarantined = !accepted && entry.sharing.is_some();

        for entry in self
            .entries
            .values_mut()
            .filter(|entry| entry.physical == key)
        {
            if !entry.ended {
                entry.ended = true;

                if !accepted {
                    entry.failure = Some(PendingFailure::Local(Failure::UnsubscribeFalse));
                }
            }
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

            self.release(early.method.len() + early.result.get().len());
            self.route(key, &early.method, &early.result)?;
        }

        Ok(())
    }
    // Unknown server IDs are retained only while subscribe replies can identify them.
    fn notification(&mut self, body: &[u8]) -> Result<(), Failure> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Frame<'a> {
            #[serde(borrow)]
            jsonrpc: Cow<'a, str>,
            #[serde(borrow)]
            method: Cow<'a, str>,
            #[serde(borrow)]
            params: Params<'a>,
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Params<'a> {
            subscription: u64,
            #[serde(borrow)]
            result: &'a RawValue,
        }
        let frame: Frame = serde_json::from_slice(body).map_err(Error::Json)?;

        if frame.jsonrpc != "2.0" {
            return Err(Failure::Protocol(Error::Envelope));
        }

        let key = self.physical.iter().find_map(|(key, entry)| {
            (entry.server() == Some(frame.params.subscription)).then_some(*key)
        });

        if let Some(key) = key {
            return self.route(key, &frame.method, frame.params.result);
        }

        if self
            .physical
            .values()
            .any(|e| matches!(e.phase, Phase::Subscribing { .. }))
        {
            self.retain(frame.params.result.get().len() + frame.method.len())?;
            self.early.push_back(Early {
                server: frame.params.subscription,
                method: frame.method.into_owned(),
                result: frame.params.result.to_owned(),
            });
        }
        Ok(())
    }
    fn route(&mut self, key: u32, method: &str, raw: &RawValue) -> Result<(), Failure> {
        let entry = self.physical.get(&key).ok_or(Failure::Closed)?;

        if entry.notification != method || entry.kind != NotificationKind::Json {
            return Err(Failure::Method);
        }

        let bytes = raw.get().len();
        let keys = self.receivers(key);
        self.preflight(&keys, bytes)?;

        for key in keys {
            let entry = self.entries.get_mut(&key).ok_or(Failure::Closed)?;
            entry.bytes += bytes;
            entry.queue.push_back(Payload::Json(raw.to_owned()));
        }

        Ok(())
    }
    fn discard_early_if_settled(&mut self) {
        if self
            .physical
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
