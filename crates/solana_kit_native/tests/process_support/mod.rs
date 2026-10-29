//! Caller-created directories and fixed explicit fixture ports; no production probe.

use solana_kit_client::Limits;
use solana_kit_core::{Address, Hash};
use solana_kit_native::{
    HttpConfig,
    surfpool::process::{ClosePolicy, OwnedSurfpool, ProcessOptions},
};
use solana_kit_surfpool::config::{Ports, SurfnetConfig};
use std::{
    ffi::OsString,
    num::NonZeroU16,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU16, AtomicUsize, Ordering},
    time::Duration,
};
use tokio::fs;

static NEXT_PORT: AtomicU16 = AtomicU16::new(26000);
static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);
const RPC_BYTES: usize = 64 * 1024;
const EVENT_POLL: Duration = Duration::from_millis(10);
const GRACEFUL_WAIT: Duration = Duration::from_millis(400);
pub const WAIT: Duration = Duration::from_secs(5);
pub const SHORT_WAIT: Duration = Duration::from_millis(300);

pub struct Fixture {
    pub directory: PathBuf,
    pub options: ProcessOptions,
}

impl Fixture {
    pub async fn new(mode: &str) -> Self {
        let sequence = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "solana-kit-process-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&directory).await.unwrap();

        for name in ["config", "tmp", "logs"] {
            fs::create_dir(directory.join(name)).await.unwrap();
        }

        fs::write(directory.join("payer.json"), b"[]")
            .await
            .unwrap();
        let options = process_options(&directory, mode);

        Self { directory, options }
    }

    pub fn with_stock_overrides(mut self, lowercase: bool) -> Self {
        for (name, value) in [
            ("SURFPOOL_NETWORK_HOST", "0.0.0.0"),
            ("SURFPOOL_STUDIO_HOST", "0.0.0.0:1234"),
            ("SURFPOOL_PUBLIC_HOST", "external.invalid"),
            ("SURFPOOL_PUBLIC_RPC_URL", "https://external.invalid"),
            ("SURFPOOL_PUBLIC_WS_URL", "wss://external.invalid"),
            ("SURFPOOL_PUBLIC_STUDIO_URL", "https://external.invalid"),
            ("SURFPOOL_DATASOURCE_RPC_URL", "https://external.invalid"),
            ("SURFPOOL_METRICS_ENABLED", "true"),
            ("SURFPOOL_METRICS_ADDR", "0.0.0.0:1234"),
            ("HOME", "private-home-sentinel"),
            ("SNAP_REAL_HOME", "private-home-sentinel"),
            ("USERPROFILE", "private-home-sentinel"),
            ("XDG_CONFIG_HOME", "private-config-sentinel"),
            ("TMPDIR", "private-temp-sentinel"),
            ("TMP", "private-temp-sentinel"),
            ("TEMP", "private-temp-sentinel"),
        ] {
            let mut key = name.to_owned();

            if lowercase {
                key.make_ascii_lowercase();
            }

            self.options.environment.push((key.into(), value.into()));
        }

        self
    }

    pub fn command() -> Command {
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--exact",
                "fixture_entry",
                "--nocapture",
                "--test-threads=1",
                "--",
            ])
            .env("SOLANA_KIT_COMMAND_ENV", "must-be-cleared");

        command
    }

    pub fn spawn(self) -> (OwnedSurfpool, PathBuf) {
        let owner = OwnedSurfpool::spawn(Self::command(), self.options).unwrap();

        (owner, self.directory)
    }
}

fn process_options(directory: &Path, mode: &str) -> ProcessOptions {
    ProcessOptions {
        config: SurfnetConfig::default(),
        ports: ports(),
        payer: Address::from_bytes([7; 32]),
        keypair_path: directory.join("payer.json"),
        genesis: genesis(),
        runtime_directory: directory.to_path_buf(),
        environment: environment(mode),
        http: HttpConfig::default(),
        limits: Limits {
            bytes: RPC_BYTES,
            depth: 32,
        },
    }
}

fn ports() -> Ports {
    let base = NEXT_PORT.fetch_add(3, Ordering::Relaxed);

    Ports::new(
        NonZeroU16::new(base).unwrap(),
        NonZeroU16::new(base + 1).unwrap(),
        NonZeroU16::new(base + 2).unwrap(),
    )
    .unwrap()
}

fn genesis() -> Hash {
    let mut genesis = [0; 32];
    tokio_rustls::rustls::crypto::ring::default_provider()
        .secure_random
        .fill(&mut genesis)
        .unwrap();

    Hash::from_bytes(genesis)
}

fn environment(mode: &str) -> Vec<(OsString, OsString)> {
    let environment = vec![
        ("SOLANA_KIT_PROCESS_CHILD".into(), "1".into()),
        ("SOLANA_KIT_PROCESS_MODE".into(), mode.into()),
        ("SOLANA_KIT_SELECTED_ENV".into(), "selected".into()),
        ("SURFPOOL_CALLER_VALUE".into(), "preserved".into()),
    ];
    #[cfg(windows)]
    let environment = {
        let mut environment = environment;

        if let Some(root) = std::env::var_os("SystemRoot") {
            environment.push(("SystemRoot".into(), root));
        }

        environment
    };

    environment
}

pub fn close_policy() -> ClosePolicy {
    ClosePolicy {
        request_timeout: SHORT_WAIT,
        graceful_timeout: GRACEFUL_WAIT,
        reap_timeout: Duration::from_secs(3),
    }
}

pub async fn calls(directory: &Path) -> String {
    fs::read_to_string(directory.join("fixture-calls"))
        .await
        .unwrap()
}

pub async fn wait_listening(directory: &Path) {
    tokio::time::timeout(WAIT, async {
        while recorded(directory).await.is_none() {
            tokio::time::sleep(EVENT_POLL).await;
        }
    })
    .await
    .unwrap();
}

pub async fn wait_calls(directory: &Path, method: &str, count: usize) {
    tokio::time::timeout(WAIT, async {
        loop {
            let recorded = recorded(directory).await.unwrap_or_default();

            if recorded.lines().filter(|line| *line == method).count() >= count {
                return;
            }

            tokio::time::sleep(EVENT_POLL).await;
        }
    })
    .await
    .unwrap();
}

async fn recorded(directory: &Path) -> Option<String> {
    match fs::read_to_string(directory.join("fixture-calls")).await {
        Ok(recorded) => Some(recorded),

        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => panic!("fixture events unavailable: {error}"),
    }
}
