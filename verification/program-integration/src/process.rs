//! Owned paths, private files, signals and bounded reaping.

use crate::node::NodeError;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::{Child, Stdio},
    time::{Duration, Instant},
};

pub(crate) const SHUTDOWN: Duration = Duration::from_secs(15);
pub(crate) const REAP_POLL: Duration = Duration::from_millis(50);
const LOG_TAIL: usize = 8 * 1024;

pub(crate) struct OwnedPaths {
    pub(crate) root: PathBuf,
    pub(crate) ledger: PathBuf,
    pub(crate) log: PathBuf,
    pub(crate) cli_config: PathBuf,
    pub(crate) tmp: PathBuf,
}

impl OwnedPaths {
    pub(crate) fn identity_file(&self) -> PathBuf {
        self.ledger.join("validator-keypair.json")
    }
}

pub(crate) fn prepare(nonce: &[u8; 8]) -> Result<OwnedPaths, NodeError> {
    let suffix: String = nonce.iter().map(|byte| format!("{byte:02x}")).collect();
    let root = env::temp_dir().join(format!(
        "solana-kit-program-integration-{}-{suffix}",
        std::process::id()
    ));

    let mut builder = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(&root)?;

    let tmp = root.join("tmp");
    if let Err(error) = fs::create_dir(&tmp) {
        return match fs::remove_dir(&root) {
            Ok(()) => Err(error.into()),
            Err(cleanup) => Err(NodeError::Readiness(format!(
                "create temporary directory: {error}; cleanup retained {}: {cleanup}",
                root.display()
            ))),
        };
    }

    Ok(OwnedPaths {
        ledger: root.join("ledger"),
        log: root.join("validator.stderr.log"),
        cli_config: root.join("cli.yml"),
        root,
        tmp,
    })
}

pub(crate) fn write_private(path: &Path, bytes: &[u8]) -> Result<(), NodeError> {
    use io::Write as _;

    let mut file = private_file(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;

    Ok(())
}

pub(crate) fn private_file(path: &Path) -> Result<fs::File, NodeError> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }

    Ok(options.open(path)?)
}

pub(crate) fn log_tail(path: &Path) -> String {
    let Ok(bytes) = fs::read(path) else {
        return "<unavailable>".to_owned();
    };
    let start = bytes.len().saturating_sub(LOG_TAIL);

    String::from_utf8_lossy(&bytes[start..]).trim().to_owned()
}

#[cfg(unix)]
pub(crate) fn terminate(child: &mut Child) -> io::Result<()> {
    let result = {
        // Safety: `kill` only receives this direct child's live PID and a valid signal.
        unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGTERM) }
    };

    if result == 0 {
        return Ok(());
    }

    let error = io::Error::last_os_error();

    if error.raw_os_error() == Some(libc::ESRCH) {
        return Ok(());
    }

    Err(error)
}

#[cfg(not(unix))]
pub(crate) fn terminate(child: &mut Child) -> io::Result<()> {
    child.kill()
}

pub(crate) async fn wait_until_async(child: &mut Child, deadline: Duration) -> bool {
    let started = Instant::now();

    while started.elapsed() < deadline {
        if matches!(child.try_wait(), Ok(Some(_))) {
            return true;
        }

        tokio::time::sleep(REAP_POLL).await;
    }

    false
}

pub(crate) fn wait_until_blocking(child: &mut Child, deadline: Duration) -> bool {
    let started = Instant::now();

    while started.elapsed() < deadline {
        if matches!(child.try_wait(), Ok(Some(_))) {
            return true;
        }

        std::thread::sleep(REAP_POLL);
    }

    false
}

pub(crate) fn log_stdio(path: &Path) -> Result<Stdio, NodeError> {
    Ok(Stdio::from(private_file(path)?))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::process::Command;

    #[tokio::test]
    async fn bounded_observation_retains_a_live_child_until_explicit_kill_and_reap() {
        let mut child = Command::new("/bin/sleep").arg("30").spawn().unwrap();
        let async_reaped = wait_until_async(&mut child, Duration::from_millis(1)).await;
        let blocking_reaped = wait_until_blocking(&mut child, Duration::from_millis(1));
        let still_alive = child.try_wait().unwrap().is_none();
        child.kill().unwrap();
        let reaped = wait_until_async(&mut child, Duration::from_secs(5)).await;

        assert!(!async_reaped);
        assert!(!blocking_reaped);
        assert!(still_alive);
        assert!(reaped);
        assert!(child.try_wait().unwrap().is_some());
    }
}
