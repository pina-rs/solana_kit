//! One owned `solana-test-validator` process, its private ledger and its cleanup.
//!
//! The node is never borrowed, discovered from an ambient cluster or reused across
//! tests. Every path, port, key and config file below this owned root is created
//! by this process for this test and removed only after the direct child is reaped.

use crate::binary;
use crate::fixtures::LocalAccount;
use crate::ports::Ports;
use crate::process::{self, OwnedPaths};
use solana_kit_core::{Address, Hash};
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

/// Locating, versioning, spawning or observing the owned node failed.
#[derive(Debug)]
pub enum NodeError {
    /// No explicitly selected or discoverable validator exists.
    MissingBinary(String),
    /// The discovered executable is not the pinned Agave test validator.
    WrongBinary(String),
    /// An owned file, port or process operation failed.
    Io(io::Error),
    /// The direct child exited before readiness.
    EarlyExit {
        /// Exact wait status text.
        status: String,
        /// Bounded stderr tail from the owned log.
        log: String,
    },
    /// Readiness or cleanup did not complete inside its explicit bound.
    Readiness(String),
    /// The RPC identity differs from the keypair in the owned ledger.
    Identity {
        /// Ledger identity.
        expected: Address,
        /// RPC-reported identity.
        actual: Address,
    },
    /// The RPC genesis differs from the genesis recorded after readiness.
    Genesis {
        /// Recorded genesis.
        expected: Hash,
        /// Current RPC genesis.
        actual: Hash,
    },
    /// The owned ledger keypair could not be read.
    LedgerIdentity(String),
}

impl fmt::Display for NodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingBinary(searched) => write!(
                formatter,
                "solana-test-validator was not found. Set SOLANA_TEST_VALIDATOR or put it on PATH. Checked: {searched}"
            ),
            Self::WrongBinary(version) => write!(
                formatter,
                "expected an Agave 4.2.x solana-test-validator, got: {version}"
            ),
            Self::Io(error) => write!(formatter, "owned node I/O failed: {error}"),
            Self::EarlyExit { status, log } => write!(
                formatter,
                "owned validator exited before readiness with {status}; stderr tail:\n{log}"
            ),
            Self::Readiness(message) => {
                write!(formatter, "owned validator readiness failed: {message}")
            }
            Self::Identity { expected, actual } => write!(
                formatter,
                "RPC identity {actual} does not match the owned ledger identity {expected}"
            ),
            Self::Genesis { expected, actual } => write!(
                formatter,
                "RPC genesis {actual} does not match the recorded owned genesis {expected}"
            ),
            Self::LedgerIdentity(message) => {
                write!(formatter, "owned ledger identity is unreadable: {message}")
            }
        }
    }
}

impl std::error::Error for NodeError {}

impl From<io::Error> for NodeError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Ports and paths reserved for exactly one child process.
pub(crate) struct Node {
    child: Option<Child>,
    paths: OwnedPaths,
    rpc_port: u16,
}

impl Node {
    /// Resolves the explicit validator, proves its version and spawns one child.
    ///
    /// The caller supplies fresh entropy for the unique owned directory and the
    /// request nonce. No ambient Solana config, keypair, ledger or validator is
    /// consulted: `--config`, `--mint`, `--ledger`, HOME and TMPDIR are all owned.
    pub(crate) fn spawn(
        mint: Address,
        directory_nonce: &[u8; 8],
        port_nonce: &[u8; 2],
        bpf_programs: &[(Address, PathBuf)],
        accounts: &[LocalAccount],
    ) -> Result<Self, NodeError> {
        let (binary, _version) = binary::locate_validator()?;
        let mut ports = Ports::reserve(port_nonce)?;
        let paths = process::prepare(directory_nonce)?;
        let rpc_port = ports.rpc;
        let child = start_child(&binary, &paths, &mut ports, mint, bpf_programs, accounts)
            .map_err(|error| failed_spawn(&paths.root, error))?;

        Ok(Self {
            child: Some(child),
            rpc_port,
            paths,
        })
    }

    pub(crate) const fn rpc_port(&self) -> u16 {
        self.rpc_port
    }

    pub(crate) fn endpoint(&self) -> String {
        format!("http://127.0.0.1:{}", self.rpc_port)
    }

    /// Rejects an exited direct child before any RPC observation is accepted.
    pub(crate) fn alive(&mut self) -> Result<(), NodeError> {
        match self.child_mut()?.try_wait()? {
            Some(status) => Err(NodeError::EarlyExit {
                status: status.to_string(),
                log: self.log_tail(),
            }),
            None => Ok(()),
        }
    }

    /// Reads the validator keypair written into this owned ledger.
    pub(crate) fn ledger_identity(&self) -> Result<Address, NodeError> {
        let path = self.paths.identity_file();
        let bytes = fs::read(&path)
            .map_err(|error| NodeError::LedgerIdentity(format!("{}: {error}", path.display())))?;
        let keypair: Vec<u8> = serde_json::from_slice(&bytes)
            .map_err(|error| NodeError::LedgerIdentity(format!("invalid JSON: {error}")))?;

        if keypair.len() != 64 {
            return Err(NodeError::LedgerIdentity(format!(
                "expected 64 keypair bytes, found {}",
                keypair.len()
            )));
        }

        let mut public = [0_u8; 32];
        public.copy_from_slice(&keypair[32..]);

        Ok(Address::from_bytes(public))
    }

    fn log_tail(&self) -> String {
        process::log_tail(&self.paths.log)
    }

    /// Terminates, reaps and deletes only this owned root.
    ///
    /// A returned failure means data was retained because reaping was not proven.
    pub(crate) async fn shutdown(&mut self) -> Result<(), NodeError> {
        let Some(mut child) = self.child.take() else {
            return Ok(());
        };
        let mut failures = Vec::new();
        let reaped = stop_child(&mut child, &mut failures).await;

        if reaped
            && failures.is_empty()
            && let Err(error) = fs::remove_dir_all(&self.paths.root)
        {
            failures.push(format!("remove {}: {error}", self.paths.root.display()));
        }

        if failures.is_empty() {
            return Ok(());
        }

        Err(NodeError::Readiness(format!(
            "cleanup retained {}: {}",
            self.paths.root.display(),
            failures.join("; ")
        )))
    }

    fn child_mut(&mut self) -> Result<&mut Child, NodeError> {
        self.child
            .as_mut()
            .ok_or_else(|| NodeError::Readiness("owned child handle was already consumed".into()))
    }
}

fn failed_spawn(root: &Path, error: NodeError) -> NodeError {
    match fs::remove_dir_all(root) {
        Ok(()) => error,
        Err(cleanup) => NodeError::Readiness(format!(
            "{error}; cleanup retained {}: {cleanup}",
            root.display()
        )),
    }
}

// A clean child is already reaped; otherwise SIGTERM then SIGKILL each get
// their own bounded observation so a hung child cannot block cleanup.
async fn stop_child(child: &mut Child, failures: &mut Vec<String>) -> bool {
    match child.try_wait() {
        Ok(Some(_)) => return true,
        Ok(None) => {}
        Err(error) => {
            failures.push(format!("observe: {error}"));
        }
    }

    if let Err(error) = process::terminate(child) {
        failures.push(format!("terminate: {error}"));
    }

    if process::wait_until_async(child, process::SHUTDOWN).await {
        return true;
    }

    if let Err(error) = child.kill() {
        failures.push(format!("kill: {error}"));
    }

    if process::wait_until_async(child, process::SHUTDOWN).await {
        return true;
    }

    failures.push("child was not reaped after SIGKILL".to_owned());
    false
}

fn start_child(
    binary: &Path,
    paths: &OwnedPaths,
    ports: &mut Ports,
    mint: Address,
    bpf_programs: &[(Address, PathBuf)],
    accounts: &[LocalAccount],
) -> Result<Child, NodeError> {
    write_cli_config(paths, ports.rpc)?;
    let mut command = Command::new(binary);
    command
        .current_dir(&paths.root)
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("HOME", &paths.root)
        .env("TMPDIR", &paths.tmp)
        .env("NO_DNA", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(process::log_stdio(&paths.log)?);
    add_node_arguments(&mut command, paths, ports, mint);
    add_bpf_programs(&mut command, bpf_programs);
    add_accounts(&mut command, &paths.root, accounts)?;

    // The reserved sockets are dropped only once every argument is fixed.
    // The child opens its listeners immediately after exec.
    ports.release();

    command.spawn().map_err(|error| {
        NodeError::Readiness(format!("could not spawn {}: {error}", binary.display()))
    })
}

fn write_cli_config(paths: &OwnedPaths, rpc_port: u16) -> Result<(), NodeError> {
    let contents = format!(
        "---\njson_rpc_url: http://127.0.0.1:{rpc_port}\nwebsocket_url: ''\naddress_labels: {{}}\ncommitment: confirmed\n"
    );

    process::write_private(&paths.cli_config, contents.as_bytes())
}

fn add_node_arguments(command: &mut Command, paths: &OwnedPaths, ports: &Ports, mint: Address) {
    command
        .arg("--ledger")
        .arg(&paths.ledger)
        .arg("--reset")
        .arg("--quiet")
        .arg("--bind-address")
        .arg("127.0.0.1")
        .arg("--rpc-port")
        .arg(ports.rpc.to_string())
        .arg("--faucet-port")
        .arg(ports.faucet.to_string())
        .arg("--dynamic-port-range")
        .arg(format!(
            "{}-{}",
            ports.dynamic_base,
            ports.dynamic_base + crate::ports::DYNAMIC_PORTS
        ))
        .arg("--config")
        .arg(&paths.cli_config)
        .arg("--mint")
        .arg(mint.to_string())
        .arg("--faucet-per-request-sol-cap")
        .arg("500")
        .arg("--faucet-sol")
        .arg("1000000")
        .arg("--limit-ledger-size")
        .arg("5000")
        .arg("--slots-per-epoch")
        .arg("256")
        .arg("--deactivate-feature")
        .arg("B8JJXCy5amZyWG9r7EnUYLwzXSXTxG7GZ1qZ1qggo83g");
}

fn add_bpf_programs(command: &mut Command, programs: &[(Address, PathBuf)]) {
    for (address, path) in programs {
        command
            .arg("--bpf-program")
            .arg(address.to_string())
            .arg(path);
    }
}

fn add_accounts(
    command: &mut Command,
    root: &Path,
    accounts: &[LocalAccount],
) -> Result<(), NodeError> {
    for (index, account) in accounts.iter().enumerate() {
        let path = root.join(format!("account-{index}.json"));
        let document = account_document(account);
        let bytes = serde_json::to_vec(&document).map_err(|error| {
            NodeError::Readiness(format!("account {index} dump failed: {error}"))
        })?;
        process::write_private(&path, &bytes)?;
        command
            .arg("--account")
            .arg(account.address.to_string())
            .arg(path);
    }

    Ok(())
}

fn account_document(account: &LocalAccount) -> serde_json::Value {
    serde_json::json!({
        "pubkey": account.address.to_string(),
        "account": {
            "lamports": account.lamports,
            "data": [
                base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    &account.data,
                ),
                "base64",
            ],
            "owner": account.owner.to_string(),
            "executable": false,
            "rentEpoch": u64::MAX,
            "space": account.data.len(),
        },
    })
}

impl Drop for Node {
    // Panic and cancelled-future cleanup must also stop at the deadline; an
    // unproven reap retains the ledger instead of entering an unbounded wait.
    fn drop(&mut self) {
        let Some(mut child) = self.child.take() else {
            return;
        };

        let mut reaped = child.try_wait().ok().flatten().is_some();

        if !reaped {
            let _ = process::terminate(&mut child);
            reaped = process::wait_until_blocking(&mut child, process::SHUTDOWN);

            if !reaped {
                let _ = child.kill();
                reaped = process::wait_until_blocking(&mut child, process::SHUTDOWN);
            }
        }

        if reaped {
            if let Err(error) = fs::remove_dir_all(&self.paths.root) {
                eprintln!(
                    "program-integration retained {}: {error}",
                    self.paths.root.display()
                );
            }
        } else {
            eprintln!(
                "program-integration retained {} after unproven reap",
                self.paths.root.display()
            );
        }
    }
}
