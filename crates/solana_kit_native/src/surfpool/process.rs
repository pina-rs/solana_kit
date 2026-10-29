//! Caller-contained direct-child ownership without a hidden runtime or cleanup task.

mod close;
mod errors;
mod prepare;
mod readiness;
mod status;

pub use close::{ClosePolicy, CloseReceipt, ShutdownRequest};
pub use errors::{
    CloseError, InvalidProcessOptions, ShutdownError, SpawnError, SpawnStage, StartError,
    StartupError,
};
pub use prepare::ProcessOptions;

use crate::NativeHttp;
use solana_kit_core::Hash;
use solana_kit_surfpool::{Surfnet, config::Ports};
use std::{
    fmt, io,
    path::{Path, PathBuf},
    process::{Child, Command, ExitStatus},
    time::Duration,
};

const POLL: Duration = Duration::from_millis(50);

enum ChildState {
    Live(Child),
    Reaped(ExitStatus),
}

/// One direct child, its configured RPC client, and cached terminal status.
///
/// Caller-established OS containment must deny external network and real-home
/// access for the actual executable and descendants. Stock Surfpool 1.7 performs
/// a cloud version request even with `--offline` and `NO_DNA`. This owner neither
/// establishes nor certifies containment. A launcher must preserve foreground
/// child ownership or supervise its contained descendants correctly.
///
/// Only the direct retained child is killed/reaped here. Runtime directory
/// creation and cleanup remain caller-owned. Drop cannot guarantee reaping;
/// await [`Self::close`] to confirm it. No detached task or executor is created.
/// The caller must not independently reap this owner's direct child.
pub struct OwnedSurfpool {
    child: ChildState,
    surfnet: Surfnet<NativeHttp>,
    endpoint: String,
    runtime: PathBuf,
    ports: Ports,
    genesis: Hash,
    verified: bool,
    closing: bool,
    forced: bool,
    request: ShutdownRequest,
    receipt: Option<CloseReceipt>,
}

impl OwnedSurfpool {
    /// Validates/prepares before spawning, then immediately retains the std child.
    ///
    /// `command` selects an absolute executable or caller-contained foreground
    /// launcher with trusted prefix arguments. SDK `start` arguments are appended
    /// directly, without a shell. Existing command environment/stdin/stdout/stderr
    /// are replaced by the explicit policy; all three streams become null.
    ///
    /// # Errors
    /// Returns explicit configuration, path, transport preparation or spawn errors.
    /// Every fallible owner setup operation precedes OS process creation. Once
    /// `Command::spawn` succeeds, owner construction has no fallible setup step.
    pub fn spawn(mut command: Command, options: ProcessOptions) -> Result<Self, SpawnError> {
        let prepared = prepare::prepare(&mut command, options)?;
        let child = command.spawn().map_err(|cause| SpawnError::Io {
            stage: SpawnStage::Spawn,
            cause,
        })?;

        Ok(Self {
            child: ChildState::Live(child),
            surfnet: prepared.surfnet,
            endpoint: prepared.endpoint,
            runtime: prepared.runtime,
            ports: prepared.ports,
            genesis: prepared.genesis,
            verified: false,
            closing: false,
            forced: false,
            request: ShutdownRequest::NotRequested,
            receipt: None,
        })
    }

    /// Borrows the configured RPC client; this is not a WS/Studio ownership proof.
    pub const fn surfnet(&self) -> &Surfnet<NativeHttp> {
        &self.surfnet
    }
    /// Configured loopback RPC endpoint, not a claim that every listener is owned.
    pub fn rpc_endpoint(&self) -> &str {
        &self.endpoint
    }
    /// Caller-selected RPC, WebSocket and Studio ports; no probing is performed.
    pub const fn ports(&self) -> Ports {
        self.ports
    }
    /// Caller-owned runtime directory, retained even when close fails or times out.
    pub fn runtime_directory(&self) -> &Path {
        &self.runtime
    }
    /// Last observed/reaped exit status; this accessor performs no observation.
    pub fn exit_status(&self) -> Option<ExitStatus> {
        match self.child {
            ChildState::Reaped(status) => Some(status),
            ChildState::Live(_) => None,
        }
    }

    fn observe(&mut self) -> io::Result<Option<ExitStatus>> {
        let status = match &mut self.child {
            ChildState::Live(child) => child.try_wait()?,
            ChildState::Reaped(status) => return Ok(Some(*status)),
        };

        if let Some(status) = status {
            self.child = ChildState::Reaped(status);
        }

        Ok(status)
    }
}

impl fmt::Debug for OwnedSurfpool {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OwnedSurfpool")
            .field("reaped", &self.exit_status().is_some())
            .field("rpc_verified", &self.verified)
            .finish_non_exhaustive()
    }
}

impl Drop for OwnedSurfpool {
    fn drop(&mut self) {
        if !matches!(self.observe(), Ok(None)) {
            return;
        }

        if let ChildState::Live(child) = &mut self.child {
            let _ = child.kill();
        }

        let _ = self.observe();
    }
}

fn deadline(duration: Duration) -> Option<tokio::time::Instant> {
    if duration.is_zero() {
        return None;
    }

    tokio::time::Instant::now().checked_add(duration)
}
