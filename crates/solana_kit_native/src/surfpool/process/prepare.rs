//! All path/config/client preparation precedes OS process creation.

use super::{InvalidProcessOptions as Invalid, SpawnError, SpawnStage};
use crate::{HttpConfig, NativeHttp};
use solana_kit_client::{Client, Limits};
use solana_kit_core::{Address, Hash};
use solana_kit_surfpool::{
    Surfnet,
    config::{Ports, SurfnetConfig},
};
use std::{
    ffi::OsString,
    fmt, fs,
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

// Stock 1.7 CLI/environment endpoint overrides must not supersede typed options.
const CONFIGURATION_ENVIRONMENT: [&str; 9] = [
    "SURFPOOL_NETWORK_HOST",
    "SURFPOOL_STUDIO_HOST",
    "SURFPOOL_PUBLIC_HOST",
    "SURFPOOL_PUBLIC_RPC_URL",
    "SURFPOOL_PUBLIC_WS_URL",
    "SURFPOOL_PUBLIC_STUDIO_URL",
    "SURFPOOL_DATASOURCE_RPC_URL",
    "SURFPOOL_METRICS_ENABLED",
    "SURFPOOL_METRICS_ADDR",
];

/// Explicit native process inputs; no payer, root, port or environment is discovered.
///
/// Runtime must already be caller-created, private and trusted. Genesis freshness
/// and actual OS containment are caller obligations, not properties this data type
/// can prove. Command prefix arguments belong to a trusted foreground launcher.
pub struct ProcessOptions {
    /// Offline portable settings, including fixed-port/payer consistency checks.
    pub config: SurfnetConfig,
    /// Three distinct nonzero ports selected explicitly by the caller.
    pub ports: Ports,
    /// Existing payer address; this owner never generates or imports a key.
    pub payer: Address,
    /// Existing absolute CLI key file matching `payer`; only metadata is checked here.
    /// The child CLI consumes its contents; this owner does not import a signer.
    pub keypair_path: PathBuf,
    /// Caller-fresh genesis challenge for accidental-collision detection.
    pub genesis: Hash,
    /// Existing caller-private absolute runtime, never removed by this owner.
    pub runtime_directory: PathBuf,
    /// Explicit child environment after env_clear; SDK home/temp paths win.
    /// Stock host/Studio/public URL/datasource/metrics override names are removed
    /// using the platform's environment-key matching; other names are preserved.
    pub environment: Vec<(OsString, OsString)>,
    /// Native HTTP parser and per-exchange bounds.
    pub http: HttpConfig,
    /// Explicit RPC byte/depth admission bounds.
    pub limits: Limits,
}

impl fmt::Debug for ProcessOptions {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProcessOptions")
            .field("ports", &self.ports)
            .finish_non_exhaustive()
    }
}

pub(super) struct Prepared {
    pub surfnet: Surfnet<NativeHttp>,
    pub endpoint: String,
    pub runtime: PathBuf,
    pub ports: Ports,
    pub genesis: Hash,
}

pub(super) fn prepare(
    command: &mut Command,
    options: ProcessOptions,
) -> Result<Prepared, SpawnError> {
    let endpoint = validate(command, &options)?;
    let arguments = options
        .config
        .start_args(options.ports, options.payer)
        .map_err(SpawnError::Config)?;
    configure(command, &options, arguments);
    let transport = NativeHttp::new(&endpoint, options.http).map_err(SpawnError::Http)?;

    Ok(Prepared {
        surfnet: Surfnet::new(Client::new(transport, options.limits)),
        endpoint,
        runtime: options.runtime_directory,
        ports: options.ports,
        genesis: options.genesis,
    })
}

fn validate(command: &Command, options: &ProcessOptions) -> Result<String, SpawnError> {
    check_path(
        Path::new(command.get_program()),
        SpawnStage::Executable,
        false,
    )?;
    check_path(&options.runtime_directory, SpawnStage::Runtime, true)?;
    check_path(&options.keypair_path, SpawnStage::Keypair, false)?;

    if !options.config.offline || options.config.remote_rpc_url.is_some() {
        return Err(SpawnError::Invalid(Invalid::Offline));
    }

    let host: IpAddr = options
        .config
        .host
        .parse()
        .map_err(|_| SpawnError::Invalid(Invalid::Host))?;

    if !host.is_loopback() {
        return Err(SpawnError::Invalid(Invalid::Host));
    }

    if options.limits.bytes == 0 || options.limits.depth == 0 {
        return Err(SpawnError::Invalid(Invalid::Limits));
    }

    Ok(format!(
        "http://{}",
        SocketAddr::new(host, options.ports.rpc.get())
    ))
}

fn configure(command: &mut Command, options: &ProcessOptions, arguments: Vec<String>) {
    command
        .args(arguments)
        .args([
            "--no-deploy",
            "--db",
            ":memory:",
            "--log-level",
            "none",
            "--disable-instruction-profiling",
        ])
        .arg("--genesis-hash")
        .arg(options.genesis.to_string())
        .arg("--surfnet-id")
        .arg(options.genesis.to_string())
        .arg("--airdrop-keypair-path")
        .arg(&options.keypair_path)
        .arg("--log-path")
        .arg(options.runtime_directory.join("logs"))
        .env_clear()
        .envs(options.environment.iter().map(|(key, value)| (key, value)));

    for name in CONFIGURATION_ENVIRONMENT {
        command.env_remove(name);
    }

    command
        .env("HOME", &options.runtime_directory)
        .env("SNAP_REAL_HOME", &options.runtime_directory)
        .env("USERPROFILE", &options.runtime_directory)
        .env("XDG_CONFIG_HOME", options.runtime_directory.join("config"))
        .env("TMPDIR", options.runtime_directory.join("tmp"))
        .env("TMP", options.runtime_directory.join("tmp"))
        .env("TEMP", options.runtime_directory.join("tmp"))
        .env("NO_DNA", "1")
        .current_dir(&options.runtime_directory)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
}

fn check_path(path: &Path, stage: SpawnStage, directory: bool) -> Result<(), SpawnError> {
    let invalid = match stage {
        SpawnStage::Executable => Invalid::Executable,
        SpawnStage::Runtime => Invalid::Runtime,
        _ => Invalid::Keypair,
    };

    if !path.is_absolute() {
        return Err(SpawnError::Invalid(invalid));
    }

    let metadata = fs::metadata(path).map_err(|cause| SpawnError::Io { stage, cause })?;

    if (directory && !metadata.is_dir()) || (!directory && !metadata.is_file()) {
        return Err(SpawnError::Invalid(invalid));
    }

    Ok(())
}
