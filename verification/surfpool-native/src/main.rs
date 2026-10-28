//! Actual-binary verification; execution requires root's sandbox and outer controller.

mod config;
mod journey;
mod logs;
mod owner;

use std::{fmt, io, process::ExitCode};

type Result<T> = std::result::Result<T, Error>;

/// Fixed diagnostics exclude child log text, paths and supplied argument values.
#[derive(Debug)]
enum Error {
    Configuration(&'static str),
    Io(io::Error),
    Rpc(solana_kit_client::Error),
    Surfnet(solana_kit_surfpool::errors::SurfpoolRpcError),
    Native(solana_kit_native::ConfigError),
    Check(&'static str),
    Observation,
    EarlyExit,
    StartupDeadline,
    LogLimit,
    CleanupUnconfirmed,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(message) | Self::Check(message) => formatter.write_str(message),
            Self::Io(error) => write!(formatter, "I/O failure: {:?}", error.kind()),
            Self::Rpc(error) => rpc_failure(error, formatter),
            Self::Surfnet(error) => {
                write!(formatter, "{error}: ")?;
                rpc_failure(&error.cause, formatter)
            }
            Self::Native(error) => write!(formatter, "{error}"),
            other => write!(formatter, "{other:?}"),
        }
    }
}

impl std::error::Error for Error {}

/// Exposes only typed failure categories and status codes, never remote bodies or URLs.
fn rpc_failure(
    error: &solana_kit_client::Error,
    formatter: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    match error {
        solana_kit_client::Error::Transport(error) => {
            write!(formatter, "RPC transport failure: {:?}", error.kind)
        }
        solana_kit_client::Error::Status(status) => write!(formatter, "RPC HTTP status {status}"),
        _ => formatter.write_str("RPC protocol/schema failure"),
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<solana_kit_client::Error> for Error {
    fn from(error: solana_kit_client::Error) -> Self {
        Self::Rpc(error)
    }
}

impl From<solana_kit_surfpool::errors::SurfpoolRpcError> for Error {
    fn from(error: solana_kit_surfpool::errors::SurfpoolRpcError) -> Self {
        Self::Surfnet(error)
    }
}

impl From<solana_kit_native::ConfigError> for Error {
    fn from(error: solana_kit_native::ConfigError) -> Self {
        Self::Native(error)
    }
}

/// Uses one explicit current-thread executor; no cleanup runtime is constructed.
#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    if !cfg!(target_os = "macos") {
        eprintln!("surfpool verification requires the selected macOS sandbox boundary");
        return ExitCode::from(2);
    }

    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("surfpool verification failed: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Attempts cleanup after both startup and journey failures, retaining both causes.
async fn run() -> Result<()> {
    let config = config::Config::parse()?;
    let mut owner = owner::Owner::spawn(config)?;
    let result = journey::run(&mut owner).await;
    let cleanup = owner.close(result.is_err()).await;

    if let Err(error) = &result {
        eprintln!("journey result: {error}");
    }

    cleanup?;
    result?;
    println!("PASS actual Surfpool genesis, ready, account, clock and reaped shutdown");

    Ok(())
}

/// Keeps actual behavioral checks explicit rather than accepting a healthy listener.
fn require(condition: bool, message: &'static str) -> Result<()> {
    if !condition {
        return Err(Error::Check(message));
    }

    Ok(())
}
