//! Application ownership of bounded work and generation-scoped host callbacks.

use anyhow::{Context, Result, ensure};
use leptoskit_bridge::{Bridge, Bytes, CallId, Error, Event, Limits, MethodId, PluginId, Request};
use leptoskit_native::HeadlessBackend;
use leptoskit_wasm::WasmHost;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::rc::Rc;

const VIEWPORT_WIDTH: f32 = 320.0;
const VIEWPORT_HEIGHT: f32 = 480.0;
const FUEL: u64 = 1_000_000_000;
const JOB_LIMIT: usize = 2;
const PAYLOAD_LIMIT: usize = 16;
const EVENT_LIMIT: usize = 2;
const BYTE_LIMIT: usize = 64;
const DEADLINE: u64 = 10;

/// Native callback identity includes the guest instance, not just a bridge ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Token {
    pub generation: u64,
    pub id: CallId,
}

/// A deterministic fixture resource whose destruction is observed externally.
struct Job {
    request: Request,
    live: Rc<Cell<usize>>,
}

impl Drop for Job {
    fn drop(&mut self) {
        self.live.set(self.live.get() - 1);
    }
}

/// The installed application owns both the real host and all outstanding jobs.
pub(super) struct Application {
    pub host: WasmHost<HeadlessBackend>,
    pub generation: u64,
    pub live: Rc<Cell<usize>>,
    pub cancellations: usize,
    jobs: BTreeMap<CallId, Job>,
    now: u64,
}

impl Application {
    /// Installs capabilities before guest entry, as required by the real API.
    pub fn start(wasm: &[u8], generation: u64, install: bool) -> Result<Self> {
        let mut host = WasmHost::headless(wasm).context("instantiate actual pinned WasmHost")?;
        *host.plugins() = Bridge::new(Limits {
            calls: JOB_LIMIT,
            events: EVENT_LIMIT,
            payload_bytes: PAYLOAD_LIMIT,
            buffered_bytes: BYTE_LIMIT,
        });

        if install {
            let plugin = host
                .plugins()
                .register("solana-kit-probe", (0..4).map(MethodId))?;
            host.plugins().register("application", [])?;

            for method in 0..3 {
                host.plugins().grant(plugin, MethodId(method), true)?;
            }
        }

        host.set_fuel_budget(FUEL);
        host.start(VIEWPORT_WIDTH, VIEWPORT_HEIGHT)
            .context("start actual guest")?;
        let mut app = Self {
            host,
            generation,
            live: Rc::new(Cell::new(0)),
            cancellations: 0,
            jobs: BTreeMap::new(),
            now: 0,
        };
        app.drain()?;
        Ok(app)
    }

    /// Drains cancellations before requests and owns resources until actual Drop.
    pub fn drain(&mut self) -> Result<()> {
        while let Some(id) = self.host.plugins().next_cancelled() {
            ensure!(
                self.jobs.remove(&id).is_some(),
                "cancellation has no owned job"
            );
            self.cancellations += 1;
        }

        while let Some(request) = self.host.plugins().next_request() {
            ensure!(self.jobs.len() < JOB_LIMIT, "native work capacity exceeded");
            ensure!(
                request.payload.len() <= PAYLOAD_LIMIT,
                "native payload limit"
            );
            self.live.set(self.live.get() + 1);
            let id = request.id;
            let old = self.jobs.insert(
                id,
                Job {
                    request,
                    live: Rc::clone(&self.live),
                },
            );
            ensure!(old.is_none(), "duplicate live native job");
        }

        Ok(())
    }

    /// Retrieves the sole owned request and verifies its actual binary arguments.
    pub fn request(&self, method: u32, payload: &[u8]) -> Result<Token> {
        ensure!(
            self.jobs.len() == 1,
            "expected exactly one owned native job"
        );
        let job = self.jobs.values().next().context("missing native job")?;
        ensure!(job.request.plugin == PluginId(0), "wrong plugin request");
        ensure!(
            job.request.method == MethodId(method),
            "wrong method request"
        );
        ensure!(
            job.request.payload == payload,
            "wrong binary request payload"
        );
        Ok(Token {
            generation: self.generation,
            id: job.request.id,
        })
    }

    /// Stale callbacks never reach a replacement bridge, even when IDs repeat.
    pub fn complete(
        &mut self,
        token: Token,
        reply: Result<Bytes, Error>,
        wake: bool,
    ) -> Result<bool> {
        if token.generation != self.generation {
            return Ok(false);
        }

        let job = self
            .jobs
            .remove(&token.id)
            .context("completion has no owned job")?;
        drop(job);
        self.host.plugins().complete(token.id, reply)?;

        if wake {
            self.wake()?;
        }

        Ok(true)
    }

    /// Explicit host event-loop pump; no periodic or busy polling is involved.
    pub fn wake(&mut self) -> Result<()> {
        self.host.wake_plugins().context("wake actual guest")?;
        self.drain()
    }

    /// Native events enter the real bounded bridge; the guest app routes them.
    pub fn event(&mut self, plugin: u32, kind: u32) -> Result<()> {
        self.host.plugins().emit(Event {
            plugin: PluginId(plugin),
            kind,
            payload: Bytes::new(),
        })?;
        Ok(())
    }

    /// Injects monotonically increasing time; wakes only when the deadline fires.
    pub fn advance(&mut self, tick: u64) -> Result<()> {
        ensure!(tick >= self.now, "monotonic time moved backward");
        self.now = tick;
        let token = self.request(1, &DEADLINE.to_le_bytes())?;

        if tick >= DEADLINE {
            self.complete(token, Ok(Bytes::copy_from_slice(&tick.to_le_bytes())), true)?;
        }

        Ok(())
    }
}

impl Drop for Application {
    /// Dropping the host cannot cancel app jobs for it; the owner does so here.
    fn drop(&mut self) {
        self.jobs.clear();
    }
}
