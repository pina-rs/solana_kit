//! Fixed diagnostics keep original process and RPC causes behind explicit access.

use solana_kit_client::Error as RpcError;
use solana_kit_surfpool::errors::SurfpoolRpcError;
use std::{fmt, io, process::ExitStatus};

/// Invalid explicit process settings, before OS process creation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvalidProcessOptions {
    /// Command must select an absolute regular executable/launcher.
    Executable,
    /// Runtime must be an existing absolute directory supplied by the caller.
    Runtime,
    /// Keypair path must name an explicit absolute regular file.
    Keypair,
    /// This initial process owner supports offline configuration without a fork.
    Offline,
    /// Host must be an IP-literal loopback address.
    Host,
    /// Native RPC byte/depth limits must be positive.
    Limits,
}

/// Operation that failed before returning the child owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpawnStage {
    /// Inspecting the explicit executable/launcher.
    Executable,
    /// Inspecting the caller's runtime directory.
    Runtime,
    /// Inspecting the explicit keypair path.
    Keypair,
    /// Synchronous OS child creation.
    Spawn,
}

/// Spawn failure retaining original causes without implicit path/environment data.
pub enum SpawnError {
    /// Invalid explicit process setting.
    Invalid(InvalidProcessOptions),
    /// Portable fixed-port/payer configuration failure.
    Config(solana_kit_surfpool::config::ConfigError),
    /// Native HTTP configuration failure.
    Http(crate::ConfigError),
    /// Original metadata or spawn I/O failure.
    Io {
        /// Operation that failed.
        stage: SpawnStage,
        /// Original cause for deliberate inspection.
        cause: io::Error,
    },
}

/// Conservative validation of the stock 1.7 startup projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartupError {
    /// Missing/malformed data, duplicate tasks or unknown phases/kinds/states.
    Malformed,
    /// Startup or a required task reported failure/error.
    Failed,
    /// Claimed phase contradicts sealing or task completion.
    Inconsistent,
}

/// Readiness failed; the owner remains available for explicit close.
pub enum StartError {
    /// Timeout must be positive and representable.
    InvalidTimeout,
    /// Close already began; this owner cannot become ready again.
    Closing,
    /// The direct child exited; the status was reaped and cached.
    Exited(ExitStatus),
    /// Original nonretryable normal RPC failure.
    Rpc(RpcError),
    /// Original nonretryable Surfpool status request failure.
    Surfpool(SurfpoolRpcError),
    /// Endpoint does not match the caller-fresh genesis challenge.
    Identity,
    /// Startup projection failed conservative validation.
    Startup(StartupError),
    /// One total deadline covered probes and sleeps.
    Deadline,
    /// Original child observation failure, with the handle retained.
    Observe(io::Error),
}

/// Shutdown request failed, even if subsequent child cleanup is confirmed.
pub enum ShutdownError {
    /// Endpoint changed from the caller-fresh genesis challenge; no exit was sent.
    Identity,
    /// Original genesis/exit RPC failure.
    Rpc(RpcError),
    /// Child exited before the exit request; status is cached.
    Exited(ExitStatus),
    /// Original child observation failure.
    Observe(io::Error),
    /// Bounded identity/exit request deadline elapsed.
    Deadline,
}

/// Cleanup remains unconfirmed; the direct child handle/runtime are retained.
pub enum CloseError {
    /// All close budgets must be positive and representable.
    InvalidPolicy,
    /// Original child observation failure.
    Observe(io::Error),
    /// Original handle-based force-termination failure.
    Kill(io::Error),
    /// Force termination was requested but bounded reaping was not confirmed.
    Deadline,
}

macro_rules! redacted_error {
    ($($name:ty),* $(,)?) => {$(
        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self, formatter)
            }
        }
        impl std::error::Error for $name {}
    )*};
}

redacted_error! {
    SpawnError, StartError, ShutdownError, CloseError,
}

impl fmt::Display for SpawnError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(kind) => write!(formatter, "invalid Surfpool process option: {kind:?}"),
            Self::Config(error) => fmt::Display::fmt(error, formatter),
            Self::Http(error) => fmt::Display::fmt(error, formatter),
            Self::Io { stage, .. } => write!(formatter, "Surfpool process {stage:?} failed"),
        }
    }
}

impl fmt::Display for StartError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTimeout => formatter.write_str("invalid RPC readiness timeout"),
            Self::Closing => formatter.write_str("Surfpool shutdown has already begun"),
            Self::Exited(status) => write!(formatter, "Surfpool child exited: {status}"),
            Self::Rpc(_) => formatter.write_str("Surfpool genesis RPC failed"),
            Self::Surfpool(_) => formatter.write_str("Surfpool startup RPC failed"),
            Self::Identity => formatter.write_str("Surfpool genesis challenge mismatch"),
            Self::Startup(kind) => write!(formatter, "Surfpool startup rejected: {kind:?}"),
            Self::Deadline => formatter.write_str("Surfpool RPC readiness deadline elapsed"),
            Self::Observe(_) => formatter.write_str("Surfpool child observation failed"),
        }
    }
}

impl fmt::Display for ShutdownError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Identity => "shutdown genesis challenge mismatch",
            Self::Rpc(_) => "Surfpool shutdown RPC failed",
            Self::Exited(_) => "Surfpool child exited before shutdown request",
            Self::Observe(_) => "Surfpool shutdown observation failed",
            Self::Deadline => "Surfpool shutdown request deadline elapsed",
        })
    }
}

impl fmt::Display for CloseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidPolicy => "invalid Surfpool close policy",
            Self::Observe(_) => "Surfpool observation failed; cleanup unconfirmed",
            Self::Kill(_) => "Surfpool force termination failed; cleanup unconfirmed",
            Self::Deadline => "Surfpool reap deadline elapsed; cleanup unconfirmed",
        })
    }
}
