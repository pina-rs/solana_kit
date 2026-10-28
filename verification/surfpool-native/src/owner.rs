//! Unreaped direct-child ownership under the root's mandatory outer PGID controller.

use crate::{Error, Result, config::Config, logs::Logs, require};
use serde::{Serialize, Serializer};
use solana_kit_client::{
    Client, Limits,
    methods::{Method, node::GenesisHash},
};
use solana_kit_core::crypto::Keypair;
use solana_kit_native::{HttpConfig, NativeHttp};
use solana_kit_surfpool::Surfnet;
use std::{
    fs::{self, OpenOptions},
    process::{Child, Stdio},
    time::{Duration, Instant},
};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

const REQUEST: Duration = Duration::from_secs(2);
const SHUTDOWN: Duration = Duration::from_secs(3);
const KILL_WAIT: Duration = Duration::from_secs(2);
const POLL: Duration = Duration::from_millis(50);
const RPC_BYTES: usize = 64 * 1024;

/// Retains the live child and private directory until awaited close succeeds.
pub(crate) struct Owner {
    pub config: Config,
    pub surfnet: Surfnet<NativeHttp>,
    pub payer: Keypair,
    pub verified: bool,
    child: Option<Child>,
    logs: Logs,
}

impl Owner {
    /// Inherits the parent's PGID; only root's outer controller controls whole groups.
    pub(crate) fn spawn(config: Config) -> Result<Self> {
        let payer = config.prepare()?;
        let transport = transport(&config.endpoint())?;

        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        options.mode(0o600);
        let log = options.open(config.directory.join("child.log"))?;
        let mut command = config.boundary(config.executable.as_os_str());
        command
            .args(config.arguments(&payer)?)
            .env("NO_DNA", "1")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let child = command.spawn()?;
        let limits = Limits {
            bytes: RPC_BYTES,
            ..Limits::default()
        };
        let mut owner = Self {
            config,
            surfnet: Surfnet::new(Client::new(transport, limits)),
            payer,
            verified: false,
            child: Some(child),
            logs: Logs::new(log),
        };

        owner.read_logs()?;

        Ok(owner)
    }

    /// Rejects exited children and log failures before accepting an RPC observation.
    pub(crate) fn alive(&self) -> Result<()> {
        self.logs.check()?;

        if self.exited()? {
            return Err(Error::EarlyExit);
        }

        Ok(())
    }

    /// Retains child/runtime on any unconfirmed shutdown; never removes a live directory.
    pub(crate) async fn close(&mut self, retain_runtime: bool) -> Result<()> {
        if self.child.is_none() {
            return Ok(());
        }

        let exited = self.exited().map_err(|_| Error::CleanupUnconfirmed)?;
        let requested = if exited {
            Ok(())
        } else {
            self.request_exit().await
        };

        let graceful = self
            .drain(SHUTDOWN)
            .await
            .map_err(|_| Error::CleanupUnconfirmed)?;

        if !graceful {
            self.child
                .as_mut()
                .ok_or(Error::CleanupUnconfirmed)?
                .kill()
                .map_err(|_| Error::CleanupUnconfirmed)?;
        }

        if !graceful
            && !self
                .drain(KILL_WAIT)
                .await
                .map_err(|_| Error::CleanupUnconfirmed)?
        {
            return Err(Error::CleanupUnconfirmed);
        }

        // WNOWAIT has observed exit while this is still the sole unreaped handle.
        let status = self
            .child
            .as_mut()
            .ok_or(Error::CleanupUnconfirmed)?
            .wait()
            .map_err(|_| Error::CleanupUnconfirmed)?;
        self.child = None;
        self.logs.finish().await?;
        require(status.success(), "owned child exited unsuccessfully")?;

        if !retain_runtime && requested.is_ok() {
            fs::remove_dir_all(&self.config.directory)?;
        }

        requested?;

        Ok(())
    }

    /// Never sends exit to an unverified or now-different endpoint.
    async fn request_exit(&self) -> Result<()> {
        if !self.verified {
            return Ok(());
        }

        self.alive()?;
        let genesis = self.surfnet.rpc().call(GenesisHash).await?;
        require(
            genesis == self.config.genesis,
            "shutdown genesis challenge changed",
        )?;
        self.alive()?;
        self.surfnet.rpc().call(Exit).await?;

        Ok(())
    }

    /// Owns both pipes before any readiness request can accept the child.
    fn read_logs(&mut self) -> Result<()> {
        let child = self.child.as_mut().ok_or(Error::CleanupUnconfirmed)?;
        self.logs
            .read(child.stdout.take().ok_or(Error::CleanupUnconfirmed)?);
        self.logs
            .read(child.stderr.take().ok_or(Error::CleanupUnconfirmed)?);

        Ok(())
    }

    /// WNOWAIT retains the kernel child relationship and pins its PID through cleanup.
    fn exited(&self) -> Result<bool> {
        let child = self.child.as_ref().ok_or(Error::CleanupUnconfirmed)?;
        exit_observation(child.id())
    }

    /// Bounds observation polling and leaves the owner intact on failure.
    async fn drain(&self, duration: Duration) -> Result<bool> {
        let deadline = Instant::now() + duration;

        loop {
            if self.exited()? {
                return Ok(true);
            }

            if Instant::now() >= deadline {
                return Ok(false);
            }

            tokio::time::sleep(POLL).await;
        }
    }
}

impl Drop for Owner {
    /// Terminal private-harness failure relies on mandatory root containment, never a task.
    fn drop(&mut self) {
        if self.child.is_some() {
            eprintln!("owned child/runtime retained: outer controller cleanup required");
        }
    }
}

/// Uses the real native HTTP adapter and a short complete exchange deadline.
pub(crate) fn transport(endpoint: &str) -> Result<NativeHttp> {
    Ok(NativeHttp::new(
        endpoint,
        HttpConfig {
            connect_timeout: REQUEST,
            exchange_timeout: REQUEST,
            ..HttpConfig::default()
        },
    )?)
}

/// Avoids macOS's setuid ps, which cannot execute inside the sandbox.
#[cfg(target_os = "macos")]
fn exit_observation(child: u32) -> Result<bool> {
    let pid = libc::pid_t::try_from(child).map_err(|_| Error::Observation)?;

    if kernel_exit(child)? {
        return Ok(true);
    }

    // SAFETY: these calls read process-group identifiers without signaling or reaping.
    let group = unsafe { libc::getpgid(pid) };
    let group_error = std::io::Error::last_os_error();
    // SAFETY: getpgrp reads the calling process's group identifier.
    if group == unsafe { libc::getpgrp() } {
        return Ok(false);
    }

    // Darwin may stop exposing a PID's group after exit. Reobserve that exact
    // kernel child once to distinguish this race from an actual group change.
    if group == -1 && group_error.raw_os_error() == Some(libc::ESRCH) && kernel_exit(child)? {
        return Ok(true);
    }

    Err(Error::Check("owned child process group changed"))
}

#[cfg(target_os = "macos")]
fn kernel_exit(child: u32) -> Result<bool> {
    // SAFETY: zero is valid for every siginfo field; waitid receives writable storage.
    let mut information: libc::siginfo_t = unsafe { std::mem::zeroed() };
    // SAFETY: WNOHANG cannot block and WNOWAIT never consumes the owned child's status.
    let result = unsafe {
        libc::waitid(
            libc::P_PID,
            child,
            &mut information,
            libc::WEXITED | libc::WNOWAIT | libc::WNOHANG,
        )
    };
    require(result == 0, "kernel child observation failed")?;

    if information.si_pid == 0 {
        return Ok(false);
    }

    require(
        u32::try_from(information.si_pid).ok() == Some(child)
            && information.si_signo == libc::SIGCHLD
            && matches!(
                information.si_code,
                libc::CLD_EXITED | libc::CLD_KILLED | libc::CLD_DUMPED
            ),
        "unexpected kernel child status",
    )?;
    Ok(true)
}

#[cfg(not(target_os = "macos"))]
fn exit_observation(_child: u32) -> Result<bool> {
    Err(Error::Observation)
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;

    #[tokio::test(flavor = "current_thread")]
    async fn close_rejects_unsuccessful_exit_and_retains_diagnostics_after_reaping() {
        rejected_exit("/usr/bin/false").await;
        rejected_exit("/bin/sleep").await;
    }

    async fn rejected_exit(executable: &str) {
        use solana_kit_core::Hash;
        use solana_kit_surfpool::config::Ports;
        use std::{num::NonZeroU16, path::PathBuf, process::Command, time::SystemTime};

        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "solana-kit-failed-exit-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&directory).unwrap();
        let log = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(directory.join("child.log"))
            .unwrap();
        let child = Command::new(executable)
            .arg("30")
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let child_id = child.id();

        let config = Config {
            executable: PathBuf::from(executable),
            profile: PathBuf::from("/dev/null"),
            directory: directory.clone(),
            genesis: Hash::from_bytes([0; 32]),
            nonce: nonce.to_string(),
            ports: Ports::new(
                NonZeroU16::new(1).unwrap(),
                NonZeroU16::new(2).unwrap(),
                NonZeroU16::new(3).unwrap(),
            )
            .unwrap(),
            body_probe: false,
        };
        let surfnet = Surfnet::new(Client::new(
            transport(&config.endpoint()).unwrap(),
            Limits::default(),
        ));
        let mut owner = Owner {
            config,
            surfnet,
            payer: Keypair::from_seed(&[7; 32]),
            verified: false,
            child: Some(child),
            logs: Logs::new(log),
        };
        let result = owner.close(false).await;
        let retained = directory.join("child.log").is_file();
        let released = owner.child.is_none();
        drop(owner);

        if directory.exists() {
            fs::remove_dir_all(directory).unwrap();
        }

        assert!(matches!(
            result,
            Err(Error::Check("owned child exited unsuccessfully"))
        ));
        assert!(retained && released);
        assert!(kernel_exit(child_id).is_err());
    }

    #[test]
    fn observation_retains_exit_status_until_explicit_wait() {
        let mut child = std::process::Command::new("/bin/sleep")
            .arg("30")
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let running = exit_observation(child.id());
        child.kill().unwrap();
        let deadline = Instant::now() + Duration::from_secs(2);

        while !exit_observation(child.id()).unwrap() {
            assert!(Instant::now() < deadline);
            std::thread::sleep(POLL);
        }

        let retained = exit_observation(child.id());
        let status = child.wait().unwrap();
        assert!(matches!(running, Ok(false)));
        assert!(matches!(retained, Ok(true)));
        assert!(!status.success());
        assert!(exit_observation(child.id()).is_err());
    }

    #[test]
    fn observation_rejects_a_process_that_is_not_our_child() {
        assert!(exit_observation(std::process::id()).is_err());
    }
}

/// Private typed extension for Surfpool's documented admin shutdown RPC.
struct Exit;

impl Serialize for Exit {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        <[u8; 0]>::default().serialize(serializer)
    }
}

impl Method for Exit {
    type Response = ();
    const NAME: &'static str = "exit";

    fn decode(&self, value: &str) -> std::result::Result<(), solana_kit_client::Error> {
        serde_json::from_str(value).map_err(solana_kit_client::Error::Schema)
    }
}
