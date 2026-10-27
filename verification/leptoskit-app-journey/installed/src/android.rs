//! Copy public bootstrap endpoints into a fresh, already-installed debug app.
//! CI owns the installation and uninstalls it after every controller outcome.
use crate::{Error, Result, finish, require};
use std::{
    io::Write,
    path::Path,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const COMMAND_TIMEOUT: Duration = Duration::from_secs(30);
const POLL: Duration = Duration::from_millis(25);
const CONFIG_BYTES: usize = 4096;

pub(crate) fn validate_package(identifier: &str) -> Result<()> {
    require(
        !identifier.is_empty()
            && identifier.contains('.')
            && identifier
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_')),
        "Android package must be an ASCII application identifier",
    )
}

pub(crate) fn configure(adb: &Path, device: &str, identifier: &str, bytes: &[u8]) -> Result<()> {
    require(
        bytes.len() <= CONFIG_BYTES,
        "Android bootstrap exceeds its byte limit",
    )?;
    run(adb, device, identifier, &["mkdir", "files"], None)?;
    run(
        adb,
        device,
        identifier,
        &["tee", "files/journey.json"],
        Some(bytes),
    )
}

fn run(
    adb: &Path,
    device: &str,
    identifier: &str,
    arguments: &[&str],
    input: Option<&[u8]>,
) -> Result<()> {
    let mut child = Command::new(adb)
        .args(["-s", device])
        .args(["shell", "-T", "run-as"])
        .arg(identifier)
        .args(arguments)
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()?;
    let outcome = exchange(&mut child, input);
    finish([outcome, reap(&mut child)])
}

fn exchange(child: &mut Child, input: Option<&[u8]>) -> Result<()> {
    if let Some(bytes) = input {
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| Error::Response("Android setup stdin was not piped".into()))?;
        stdin.write_all(bytes)?;
    }

    let deadline = Instant::now() + COMMAND_TIMEOUT;

    loop {
        if let Some(status) = child.try_wait()? {
            return require(
                status.success(),
                format!("Android setup command failed: {status}"),
            );
        }

        require(Instant::now() < deadline, "Android setup command deadline")?;
        thread::sleep(POLL);
    }
}

fn reap(child: &mut Child) -> Result<()> {
    let observed = child.try_wait();

    let killed = match &observed {
        Ok(Some(_)) => Ok(()),
        _ => child.kill().map_err(Error::from),
    };

    let joined = child.wait().map(|_| ()).map_err(Error::from);
    finish([observed.map(|_| ()).map_err(Error::from), killed, joined])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adb_package_arguments_cannot_be_shell_programs() {
        assert!(validate_package("com.leptoskit.solanawalletapp").is_ok());

        for identifier in [
            "",
            "test; echo injected",
            "test/../../other",
            "test app",
            "$(echo test)",
        ] {
            assert!(validate_package(identifier).is_err());
        }
    }

    #[cfg(unix)]
    #[test]
    fn cleanup_reaps_a_running_owned_child() {
        let mut child = Command::new("/bin/sleep").arg("30").spawn().unwrap();
        reap(&mut child).unwrap();
        assert!(child.try_wait().unwrap().is_some());
    }

    #[cfg(unix)]
    #[test]
    fn a_failed_setup_still_leaves_a_reaped_child() {
        let mut child = Command::new("/bin/sh")
            .args(["-c", "exit 7"])
            .spawn()
            .unwrap();
        let result = exchange(&mut child, None);
        reap(&mut child).unwrap();
        assert!(result.is_err());
        assert_eq!(child.try_wait().unwrap().unwrap().code(), Some(7));
    }
}
