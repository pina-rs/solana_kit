//! Owned fake operations expose ordering, byte caps, cancellation and completion-time clocks.

use super::checks::{Outcome, require};
use solana_kit_client::{
    Reply, TransportError, TransportFailureKind, rest,
    subscriptions::{Clock, ClockError},
};
use solana_kit_dapp_publisher::{
    portal::{Client, PortalService, UploadRequest, Uploader},
    workflow::{self, Effect, Snapshot, SourceReader, Storage, Workflow},
};
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

pub const DEADLINE: u64 = 10_000;
const SNAPSHOT_BYTES: usize = 32_768;
const JSON_STRING_BYTES: usize = 16_384;
pub const JSON: rest::Budgets = rest::Budgets {
    bytes: SNAPSHOT_BYTES,
    depth: 24,
    items: 2048,
    string_bytes: JSON_STRING_BYTES,
};

pub fn limits() -> workflow::Limits {
    workflow::Limits {
        deadline: DEADLINE,
        max_polls: 3,
        poll_interval_ms: 10,
        source_bytes: 1024,
        media_bytes: 1024,
        media_items: 8,
        snapshot: JSON,
        metadata: JSON,
    }
}

pub fn options() -> workflow::Options {
    workflow::Options {
        snapshot_key: "local-checkpoint".into(),
        updated_on: "2026-10-29T12:00:00.000Z".into(),
        limits: limits(),
    }
}

pub fn complete<F: Future>(operation: F) -> Result<F::Output, &'static str> {
    let mut operation = std::pin::pin!(operation);

    match operation
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(result) => Ok(result),
        Poll::Pending => Err("synthetic ready operation stayed pending"),
    }
}

pub fn poll_pending<F: Future>(operation: Pin<&mut F>) -> Outcome {
    require(
        matches!(
            operation.poll(&mut Context::from_waker(Waker::noop())),
            Poll::Pending
        ),
        "synthetic suspended operation completed",
    )
}

pub struct Operation<T> {
    output: Option<Result<T, TransportError>>,
    cancelled: Rc<Cell<usize>>,
    clock: Rc<Cell<u64>>,
    completion: Option<u64>,
    completed: bool,
}

impl<T: Unpin> Future for Operation<T> {
    type Output = Result<T, TransportError>;
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let Some(output) = self.output.take() else {
            return Poll::Pending;
        };

        if let Some(tick) = self.completion {
            self.clock.set(tick);
        }

        self.completed = true;
        Poll::Ready(output)
    }
}

impl<T> Drop for Operation<T> {
    fn drop(&mut self) {
        if !self.completed {
            self.cancelled.set(self.cancelled.get() + 1);
        }
    }
}

pub enum Step {
    Json(String),
    Fail,
    Pending,
}
pub fn json(payload: &str) -> Step {
    Step::Json(format!("{{\"result\":{{\"data\":{payload}}}}}"))
}

pub struct PortalState {
    pub requests: RefCell<Vec<rest::Request<PortalService>>>,
    pub steps: RefCell<VecDeque<Step>>,
    pub cancelled: Rc<Cell<usize>>,
    pub require_intent: Cell<bool>,
    clock: Rc<Cell<u64>>,
    storage: Rc<StoreState>,
}

pub struct Portal(pub Rc<PortalState>);
impl rest::Transport<PortalService> for Portal {
    type Exchange<'a> = Operation<Reply>;
    fn exchange(&self, request: rest::Request<PortalService>) -> Self::Exchange<'_> {
        let effect = match request.path_segments.last().map(String::as_str) {
            Some("publication.createIngestionSession") => Some(Effect::CreateIngestion),
            Some("publication.createUploadTarget") => Some(Effect::CreateUploadTarget),
            Some("publication.cleanupRelease") => Some(Effect::Cleanup),
            _ => None,
        };

        let intent_matches = effect
            .filter(|_| self.0.require_intent.get())
            .is_none_or(|effect| {
                self.0
                    .storage
                    .snapshot()
                    .is_ok_and(|snapshot| snapshot.pending() == Some(effect))
            });
        let valid = self
            .0
            .storage
            .check(intent_matches, "portal effect preceded durable intent");
        self.0.requests.borrow_mut().push(request);
        let step = valid
            .then(|| self.0.steps.borrow_mut().pop_front())
            .flatten();

        let output = match step {
            Some(Step::Json(body)) => Some(Ok(Reply {
                status: 200,
                body: body.into_bytes(),
            })),
            Some(Step::Fail) => Some(Err(TransportError::new(TransportFailureKind::Io))),
            Some(Step::Pending) => None,
            None => {
                self.0
                    .storage
                    .check(false, "missing synthetic portal reply");
                Some(Err(TransportError::new(
                    TransportFailureKind::InvalidResponse,
                )))
            }
        };

        Operation {
            output,
            cancelled: self.0.cancelled.clone(),
            clock: self.0.clock.clone(),
            completion: None,
            completed: false,
        }
    }
}

#[derive(Default)]
pub struct StoreState {
    violation: Cell<Option<&'static str>>,
    pub data: RefCell<Option<Vec<u8>>>,
    pub history: RefCell<Vec<Vec<u8>>>,
    pub saves: Cell<usize>,
    pub fail_at: Cell<Option<usize>>,
    pub advance_save: Cell<Option<u64>>,
    pub pending_save: Cell<bool>,
    pub cancelled: Rc<Cell<usize>>,
}

impl StoreState {
    pub fn valid(&self) -> Outcome {
        self.violation.get().map_or(Ok(()), Err)
    }

    fn check(&self, condition: bool, reason: &'static str) -> bool {
        if !condition {
            self.violation.set(self.violation.get().or(Some(reason)));
        }

        condition
    }

    pub fn snapshot(&self) -> Result<Snapshot, &'static str> {
        self.valid()?;
        Snapshot::decode(
            self.data
                .borrow()
                .as_ref()
                .ok_or("checkpoint not persisted")?,
            JSON,
        )
        .map_err(|_| "persisted workflow checkpoint rejected")
    }
}

pub struct Memory {
    pub state: Rc<StoreState>,
    clock: Rc<Cell<u64>>,
}
impl Storage for Memory {
    type Load<'a> = Operation<Option<Vec<u8>>>;
    type Save<'a> = Operation<()>;
    fn load<'a>(&'a self, key: &'a str, maximum: usize) -> Self::Load<'a> {
        let valid = self
            .state
            .check(key == "local-checkpoint", "workflow load key changed")
            && self
                .state
                .check(maximum == JSON.bytes, "workflow load cap changed");
        let output = if valid {
            Ok(self.state.data.borrow().clone())
        } else {
            Err(TransportError::new(TransportFailureKind::InvalidResponse))
        };

        Operation {
            output: Some(output),

            cancelled: self.state.cancelled.clone(),
            clock: self.clock.clone(),
            completion: None,
            completed: false,
        }
    }
    fn save<'a>(&'a mut self, key: &'a str, bytes: &'a [u8], maximum: usize) -> Self::Save<'a> {
        // Failed acknowledgements leave the preceding checkpoint inspectable on resume.
        let valid = self
            .state
            .check(key == "local-checkpoint", "workflow save key changed")
            && self
                .state
                .check(bytes.len() <= maximum, "workflow save cap exceeded");

        let count = self.state.saves.get() + 1;
        self.state.saves.set(count);
        let output = if !valid {
            Some(Err(TransportError::new(
                TransportFailureKind::InvalidResponse,
            )))
        } else if self.state.pending_save.get() {
            None
        } else if self.state.fail_at.get() == Some(count) {
            Some(Err(TransportError::new(TransportFailureKind::Io)))
        } else {
            self.state.history.borrow_mut().push(bytes.to_vec());
            *self.state.data.borrow_mut() = Some(bytes.to_vec());
            Some(Ok(()))
        };

        Operation {
            output,
            cancelled: self.state.cancelled.clone(),
            clock: self.clock.clone(),
            completion: self.state.advance_save.take(),
            completed: false,
        }
    }
}

pub struct UploadObservation {
    pub url: String,
    pub bytes: Vec<u8>,
    pub mime: String,
}
pub struct UploadState {
    pub observations: RefCell<Vec<UploadObservation>>,
    pub fail: Cell<bool>,
    pub pending: Cell<bool>,
    pub require_intent: Cell<bool>,
    pub advance: Cell<Option<u64>>,
    pub cancelled: Rc<Cell<usize>>,
    storage: Rc<StoreState>,
    clock: Rc<Cell<u64>>,
}
pub struct Uploads(pub Rc<UploadState>);
impl Uploader for Uploads {
    type Upload<'a> = Operation<()>;
    fn upload<'a>(&'a self, request: UploadRequest<'a>) -> Self::Upload<'a> {
        // Negative cases must retain guard failures separately from expected transport errors.
        let intent_matches = !self.0.require_intent.get()
            || self
                .0
                .storage
                .snapshot()
                .is_ok_and(|snapshot| snapshot.pending() == Some(Effect::Upload));
        let valid = self
            .0
            .storage
            .check(intent_matches, "binary PUT preceded durable intent")
            && self.0.storage.check(
                request.body.len() <= request.max_bytes,
                "binary PUT cap exceeded",
            );

        if valid {
            self.0.observations.borrow_mut().push(UploadObservation {
                url: request.target.upload_url.clone(),
                bytes: request.body.to_vec(),
                mime: request.content_type.into(),
            });
        }

        let output = if !valid {
            Some(Err(TransportError::new(
                TransportFailureKind::InvalidResponse,
            )))
        } else if self.0.pending.get() {
            None
        } else if self.0.fail.get() {
            Some(Err(TransportError::new(TransportFailureKind::Io)))
        } else {
            Some(Ok(()))
        };

        Operation {
            output,
            cancelled: self.0.cancelled.clone(),
            clock: self.0.clock.clone(),
            completion: self.0.advance.take(),
            completed: false,
        }
    }
}

pub struct TestClock {
    pub tick: Rc<Cell<u64>>,
    pub sleeps: RefCell<Vec<u64>>,
    pub dropped: Cell<usize>,
}
pub struct Sleep<'a> {
    clock: &'a TestClock,
    deadline: u64,
}
impl Future for Sleep<'_> {
    type Output = Result<(), ClockError>;
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.deadline == DEADLINE && self.clock.tick.get() < DEADLINE {
            return Poll::Pending;
        }

        self.clock
            .tick
            .set(self.clock.tick.get().max(self.deadline));
        Poll::Ready(Ok(()))
    }
}
impl Drop for Sleep<'_> {
    fn drop(&mut self) {
        self.clock.dropped.set(self.clock.dropped.get() + 1);
    }
}
impl Clock for TestClock {
    type Sleep<'a> = Sleep<'a>;
    fn now(&self) -> u64 {
        self.tick.get()
    }
    fn sleep_until(&self, deadline: u64) -> Self::Sleep<'_> {
        self.sleeps.borrow_mut().push(deadline);
        Sleep {
            clock: self,
            deadline,
        }
    }
}

pub struct Rig {
    pub client: Client<Portal>,
    pub portal: Rc<PortalState>,
    pub storage: Memory,
    pub uploads: Uploads,
    pub clock: TestClock,
}
impl Rig {
    pub fn new(steps: Vec<Step>) -> Self {
        let tick = Rc::new(Cell::new(0));
        let storage = Rc::new(StoreState::default());
        let portal = Rc::new(PortalState {
            requests: RefCell::new(Vec::new()),
            steps: RefCell::new(steps.into()),
            cancelled: Rc::new(Cell::new(0)),
            require_intent: Cell::new(true),
            clock: tick.clone(),
            storage: storage.clone(),
        });
        let uploads = Uploads(Rc::new(UploadState {
            observations: RefCell::new(Vec::new()),
            fail: Cell::new(false),
            pending: Cell::new(false),
            require_intent: Cell::new(true),
            advance: Cell::new(None),
            cancelled: Rc::new(Cell::new(0)),
            storage: storage.clone(),
            clock: tick.clone(),
        }));
        Self {
            client: Client::new(Portal(portal.clone()), JSON),
            portal,
            storage: Memory {
                state: storage,
                clock: tick.clone(),
            },
            uploads,
            clock: TestClock {
                tick,
                sleeps: RefCell::new(Vec::new()),
                dropped: Cell::new(0),
            },
        }
    }
    pub fn valid(&self) -> Outcome {
        self.storage.state.valid()
    }
    pub fn workflow(
        &mut self,
    ) -> Result<Workflow<'_, Portal, Uploads, Memory, TestClock>, &'static str> {
        self.valid()?;
        Workflow::new(
            &self.client,
            &self.uploads,
            &mut self.storage,
            &self.clock,
            options(),
        )
        .map_err(|_| "synthetic workflow options rejected")
    }
    pub fn procedures(&self) -> Result<Vec<String>, &'static str> {
        self.valid()?;
        self.portal
            .requests
            .borrow()
            .iter()
            .map(|request| {
                request
                    .path_segments
                    .last()
                    .cloned()
                    .ok_or("portal procedure missing")
            })
            .collect()
    }
}

pub struct Reader {
    pub calls: RefCell<Vec<(String, usize)>>,
    pub bytes: Vec<u8>,
    pub pending: bool,
    pub cancelled: Rc<Cell<usize>>,
    pub tick: Rc<Cell<u64>>,
    pub completion: Option<u64>,
}
impl SourceReader for Reader {
    type Read<'a> = Operation<Vec<u8>>;
    fn read<'a>(&'a self, token: &'a str, maximum: usize) -> Self::Read<'a> {
        self.calls.borrow_mut().push((token.into(), maximum));
        Operation {
            output: (!self.pending).then(|| Ok(self.bytes.clone())),
            cancelled: self.cancelled.clone(),
            clock: self.tick.clone(),
            completion: self.completion,
            completed: false,
        }
    }
}
