//! Independent real peers and bounded exact-host supervision.
mod embedded;

use anyhow::{Result, ensure};
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    process::{Child, Command, ExitStatus, Stdio},
    thread,
    time::{Duration, Instant},
};
use tungstenite::{Message, WebSocket};

const PROCESS_SECONDS: u64 = 8;
const PEER_SECONDS: u64 = 3;
const POLL_MILLIS: u64 = 5;
const QUIET_MILLIS: u64 = 240;
const CANCEL_MILLIS: u64 = 300;
const OUTPUT_BYTES: u64 = 8192;
const REQUEST_BYTES: usize = 4096;
const OVERSIZED_BYTES: usize = 65;

trait Io: Read + Write + Send + Sync {}
impl<T: Read + Write + Send + Sync> Io for T {}
type Peer = WebSocket<Box<dyn Io>>;

fn main() -> Result<()> {
    let arguments: Vec<_> = std::env::args().collect();

    if arguments[1] == "--embedded" {
        return embedded::run(Path::new(&arguments[2]));
    }

    let host = Path::new(&arguments[1]);
    let guest = Path::new(&arguments[2]);
    let version = Command::new(host).arg("--version").output()?;
    ensure!(String::from_utf8_lossy(&version.stdout).starts_with("wasmtime 50.0.0 "));
    for mode in [
        "plain",
        "exchange",
        "duplex",
        "fragment",
        "corrupt",
        "wrong-ca",
        "wrong-name",
        "bad-upgrade",
        "oversized",
        "binary",
        "invalid-utf8",
        "masked",
        "abrupt",
        "cancel",
        "retain",
        "unload",
        "denied",
    ] {
        run(host, guest, mode)?;
    }

    Ok(())
}

fn endpoint(mode: &str, port: u16) -> String {
    let name = if mode == "wrong-name" {
        "127.0.0.1"
    } else {
        "localhost"
    };
    let scheme = if mode == "plain" { "ws" } else { "wss" };
    format!("{scheme}://{name}:{port}/rpc")
}

fn run(host: &Path, guest: &Path, mode: &str) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    listener.set_nonblocking(true)?;
    let peer_mode = mode.to_owned();
    let peer = thread::spawn(move || fixture(listener, &peer_mode));
    let child = Command::new(host)
        .args([
            "run",
            "-S",
            "allow-ip-name-lookup=y",
            "-S",
            "udp=n",
            "-S",
            if mode == "denied" {
                "inherit-network=n"
            } else {
                "inherit-network=y"
            },
        ])
        .arg(guest)
        .arg(mode)
        .arg(endpoint(mode, port))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let (status, stdout, stderr) = supervise(child, mode)?;
    let observed = peer
        .join()
        .map_err(|_| anyhow::anyhow!("peer panicked"))??;

    verify(mode, status, &stdout, &stderr)?;
    println!("PASS {mode}: {observed}; exit={status}");
    Ok(())
}

fn verify(mode: &str, status: ExitStatus, stdout: &str, stderr: &str) -> Result<()> {
    if mode == "corrupt" {
        ensure!(
            !status.success() && stderr.contains("wrong text payload"),
            "insensitive corrupt peer"
        );

        return Ok(());
    }

    if mode != "unload" {
        ensure!(
            status.success() && stdout.contains(&format!("PASS {mode}")),
            "{mode}: {stderr} {stdout}"
        );
    }

    Ok(())
}

fn supervise(mut child: Child, mode: &str) -> Result<(ExitStatus, String, String)> {
    let stdout = child.stdout.take().expect("stdout");
    let (sender, ready) = std::sync::mpsc::sync_channel(1);
    let reader = thread::spawn(move || read_output(stdout, sender));

    if mode == "unload" {
        let pending = ready.recv_timeout(Duration::from_secs(PROCESS_SECONDS));
        child.kill()?;
        ensure!(
            pending?.trim() == "PENDING unload",
            "no actual pending marker"
        );
    }

    let status = wait_child(&mut child)?;

    let stdout = reader
        .join()
        .map_err(|_| anyhow::anyhow!("stdout reader panicked"))??;
    let mut stderr = String::new();
    child
        .stderr
        .take()
        .expect("stderr")
        .take(OUTPUT_BYTES)
        .read_to_string(&mut stderr)?;
    Ok((status, stdout, stderr))
}

fn read_output(
    stdout: std::process::ChildStdout,
    sender: std::sync::mpsc::SyncSender<String>,
) -> Result<String> {
    let mut reader = BufReader::new(stdout.take(OUTPUT_BYTES));
    let mut output = String::new();
    reader.read_line(&mut output)?;
    sender
        .send(output.clone())
        .map_err(|_| anyhow::anyhow!("supervisor disappeared"))?;
    reader.read_to_string(&mut output)?;
    Ok(output)
}

fn wait_child(child: &mut Child) -> Result<ExitStatus> {
    let limit = Instant::now() + Duration::from_secs(PROCESS_SECONDS);

    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }

        if Instant::now() >= limit {
            child.kill()?;
            child.wait()?;
            anyhow::bail!("guest timeout");
        }

        thread::sleep(Duration::from_millis(POLL_MILLIS));
    }
}

/// Bounded acceptance distinguishes explicit denial from a missing peer.
fn accept(listener: &TcpListener) -> Result<Option<TcpStream>> {
    let limit = Instant::now() + Duration::from_secs(PEER_SECONDS);

    loop {
        match listener.accept() {
            Ok((tcp, _)) => return Ok(Some(tcp)),

            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error.into()),
        }

        if Instant::now() >= limit {
            return Ok(None);
        }

        thread::sleep(Duration::from_millis(POLL_MILLIS));
    }
}

fn fixture(listener: TcpListener, mode: &str) -> Result<String> {
    let Some(tcp) = accept(&listener)? else {
        ensure!(mode == "denied", "no connection {mode}");

        return Ok("no granted TCP connection".into());
    };
    tcp.set_nonblocking(false)?;
    tcp.set_read_timeout(Some(Duration::from_secs(PEER_SECONDS)))?;
    tcp.set_write_timeout(Some(Duration::from_secs(PEER_SECONDS)))?;
    let mut stream = peer_stream(tcp, mode)?;

    if matches!(mode, "wrong-ca" | "wrong-name") {
        return tls_rejected(&mut stream);
    }

    if mode == "bad-upgrade" {
        return upgrade_rejected(&mut stream);
    }

    let mut socket = tungstenite::accept(stream)?;
    peer_messages(&mut socket, mode)?;
    Ok("real peer assertions complete".into())
}

fn peer_stream(tcp: TcpStream, mode: &str) -> Result<Box<dyn Io>> {
    if mode == "plain" {
        return Ok(Box::new(tcp));
    }

    let connection = rustls::ServerConnection::new(solana_kit_wasi_sockets_proof::server()?)?;
    Ok(Box::new(rustls::StreamOwned::new(connection, tcp)))
}

fn tls_rejected(stream: &mut Box<dyn Io>) -> Result<String> {
    let mut byte = [0];
    ensure!(stream.read(&mut byte).is_err(), "invalid TLS accepted");
    Ok("actual TLS rejection alert".into())
}

fn upgrade_rejected(stream: &mut Box<dyn Io>) -> Result<String> {
    read_request(stream.as_mut())?;
    stream.write_all(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n")?;
    stream.flush()?;
    Ok("upgrade denied".into())
}

fn peer_messages(socket: &mut Peer, mode: &str) -> Result<()> {
    match mode {
        "plain" | "exchange" | "duplex" | "fragment" | "corrupt" => exchange(socket, mode),
        "cancel" | "retain" | "unload" => quiet(socket, mode),
        _ => malformed(socket, mode),
    }
}

/// Fragmented and corrupt peers preserve the independent expected request.
fn exchange(socket: &mut Peer, mode: &str) -> Result<()> {
    ensure!(socket.read()? == Message::Text("subscribe:7".into()));

    if mode == "fragment" {
        socket
            .get_mut()
            .write_all(b"\x01\x0dnotification:\x80\x0241")?;
        socket.get_mut().flush()?;
    } else {
        let text = if mode == "corrupt" {
            "notification:42"
        } else {
            "notification:41"
        };

        socket.send(Message::Text(text.into()))?;
    }

    if mode == "corrupt" {
        return Ok(());
    }

    ensure!(matches!(socket.read()?, Message::Close(_)));

    match socket.flush() {
        Ok(()) | Err(tungstenite::Error::ConnectionClosed) => Ok(()),
        Err(error) => Err(error.into()),
    }
}

fn quiet(socket: &mut Peer, mode: &str) -> Result<()> {
    let started = Instant::now();
    let received = socket.read();
    ensure!(received.is_err(), "unexpected quiet data");
    let elapsed = started.elapsed();
    let correct = if mode == "retain" {
        elapsed >= Duration::from_millis(QUIET_MILLIS)
    } else {
        elapsed < Duration::from_millis(CANCEL_MILLIS)
    };

    ensure!(correct, "wrong EOF timing: {elapsed:?}");
    Ok(())
}

fn malformed(socket: &mut Peer, mode: &str) -> Result<()> {
    let frame: Vec<u8> = match mode {
        "oversized" => [
            vec![0x81, OVERSIZED_BYTES as u8],
            vec![b'x'; OVERSIZED_BYTES],
        ]
        .concat(),
        "binary" => vec![0x82, 1, 7],
        "invalid-utf8" => vec![0x81, 1, 0xff],
        "masked" => vec![0x81, 0x81, 1, 2, 3, 4, 9],
        "abrupt" => return Ok(()),
        _ => anyhow::bail!("unknown peer mode"),
    };

    socket.get_mut().write_all(&frame)?;
    socket.get_mut().flush()?;
    Ok(())
}

fn read_request(stream: &mut dyn Io) -> Result<()> {
    let mut bytes = Vec::new();

    while !bytes.ends_with(b"\r\n\r\n") {
        ensure!(bytes.len() < REQUEST_BYTES);
        let mut byte = [0];
        stream.read_exact(&mut byte)?;
        bytes.push(byte[0]);
    }

    Ok(())
}
