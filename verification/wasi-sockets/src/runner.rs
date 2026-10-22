//! Independent loopback peer and bounded supervisor for actual WASI components.

mod imports;
mod peer;

use std::io::{BufRead, BufReader, Read};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, ensure};

const PROCESS_LIMIT: Duration = Duration::from_secs(5);
const RETRY_TICK: Duration = Duration::from_millis(5);
const OUTPUT_LIMIT: u64 = 4096;

/// Executes every success/error journey against the supplied pinned host binary.
fn main() -> Result<()> {
    let host = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .context("missing Wasmtime executable")?,
    );
    let version = Command::new(&host).arg("--version").output()?;
    let version = String::from_utf8(version.stdout)?;
    ensure!(
        version.starts_with("wasmtime 50.0.0 "),
        "wrong host version"
    );
    println!("{}", version.trim());
    let component =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("target/wasm32-wasip2/debug/guest.wasm");
    println!("IMPORTS\n{}", imports::inspect(&component)?);

    for mode in [
        "exchange",
        "corrupt",
        "wrong-ca",
        "wrong-name",
        "plaintext",
        "bad-upgrade",
        "oversized",
        "cancel",
        "retain",
        "unload",
        "denied",
        "unavailable",
    ] {
        run(&host, mode)?;
    }

    Ok(())
}

/// Keeps peer and guest lifetimes bounded; a process launch alone cannot pass.
fn run(host: &Path, mode: &str) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    listener.set_nonblocking(true)?;
    let peer_mode = mode.to_owned();
    let peer = thread::spawn(move || peer::fixture(listener, &peer_mode));
    let mut child = command(host, mode, port)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let (reader, ready) = capture(child.stdout.take().context("missing stdout")?);

    if mode == "unload" {
        let pending = ready.recv_timeout(PROCESS_LIMIT);
        child.kill()?;
        ensure!(
            pending?.trim() == "PENDING unload",
            "unload before real pending receive"
        );
    }

    let status = supervise(&mut child)?;
    let output = reader
        .join()
        .map_err(|_| anyhow::anyhow!("stdout reader panicked"))??;
    let mut stderr = String::new();
    child
        .stderr
        .take()
        .context("missing stderr")?
        .take(OUTPUT_LIMIT)
        .read_to_string(&mut stderr)?;
    let observation = peer
        .join()
        .map_err(|_| anyhow::anyhow!("peer panicked"))??;
    verify(mode, status, &output, &stderr)?;
    println!(
        "PASS {mode}: exit={status}; {observation}; guest={}",
        output.trim().replace('\n', "; ")
    );

    Ok(())
}

/// A corrupt peer passes only when the guest actually rejects its wrong value.
fn verify(mode: &str, status: ExitStatus, output: &str, stderr: &str) -> Result<()> {
    if mode == "corrupt" {
        ensure!(
            !status.success() && stderr.contains("wrong text payload"),
            "corruption not detected"
        );

        return Ok(());
    }

    ensure!(
        status.success() || mode == "unload",
        "guest {mode} failed: {stderr}"
    );
    ensure!(
        mode == "unload" || output.contains(&format!("PASS {mode}")),
        "missing guest assertion: {output}"
    );

    Ok(())
}

/// Reads bounded child output and exposes its first assertion for unload ordering.
fn capture(
    stdout: std::process::ChildStdout,
) -> (
    thread::JoinHandle<Result<String>>,
    std::sync::mpsc::Receiver<String>,
) {
    let (sender, ready) = std::sync::mpsc::sync_channel(1);
    let reader = thread::spawn(move || {
        let mut reader = BufReader::new(stdout.take(OUTPUT_LIMIT));
        let mut output = String::new();
        reader.read_line(&mut output)?;
        sender
            .send(output.clone())
            .map_err(|_| anyhow::anyhow!("supervisor disappeared"))?;
        reader.read_to_string(&mut output)?;

        Ok(output)
    });

    (reader, ready)
}

/// Grants only named interfaces; inherited-network remains an explicit CLI limit.
fn command(host: &Path, mode: &str, port: u16) -> Command {
    let component =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("target/wasm32-wasip2/debug/guest.wasm");
    let mut command = Command::new(host);
    command.args(["run", "-S", "allow-ip-name-lookup=n", "-S", "udp=n"]);

    command.args([
        "-S",
        if mode == "denied" {
            "inherit-network=n"
        } else {
            "inherit-network=y"
        },
    ]);

    command.args([
        "-S",
        if mode == "unavailable" {
            "tcp=n"
        } else {
            "tcp=y"
        },
    ]);
    command.arg(component).arg(mode).arg(port.to_string());

    command
}

/// Kills timed-out processes and reaps them before reporting failure.
fn supervise(child: &mut Child) -> Result<ExitStatus> {
    let deadline = Instant::now() + PROCESS_LIMIT;

    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }

        if Instant::now() >= deadline {
            child.kill()?;
            child.wait()?;
            anyhow::bail!("guest deadline exceeded");
        }

        thread::sleep(RETRY_TICK);
    }
}
