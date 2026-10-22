//! Bounded loopback host experiments with assertion-bearing negative controls.

use anyhow::{Context, Result, bail, ensure};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use wasmparser::{Encoding, Parser, Payload};

const WAIT: Duration = Duration::from_secs(4);
const MAX_REQUEST: usize = 8192;
const POLL_INTERVAL: Duration = Duration::from_millis(5);
const CLOSE_TIMEOUT: Duration = Duration::from_millis(250);
const STALL_DURATION: Duration = Duration::from_millis(200);
const TLS_HANDSHAKE: u8 = 22;
const CREATED: u16 = 201;
const UNAVAILABLE: u16 = 503;

const BODY: &[u8] = b"{\"id\":7}";

/// Bound request parsing independently of the component under test.
fn read_request(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 512];
    stream.set_read_timeout(Some(WAIT))?;

    loop {
        let count = stream.read(&mut buffer)?;
        ensure!(count != 0, "request ended before its body");
        bytes.extend_from_slice(&buffer[..count]);
        ensure!(bytes.len() <= MAX_REQUEST, "request exceeds fixture limit");

        if let Some(length) = request_length(&bytes)? {
            ensure!(bytes.len() <= length, "unexpected trailing request bytes");

            if bytes.len() == length {
                return Ok(bytes);
            }
        }
    }
}

/// Require the exact method, path, headers, and content length.
fn request_length(bytes: &[u8]) -> Result<Option<usize>> {
    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut request = httparse::Request::new(&mut headers);
    let httparse::Status::Complete(header_length) = request.parse(bytes)? else {
        return Ok(None);
    };
    ensure!(request.method == Some("POST"), "incorrect method");
    ensure!(
        request.path.is_some_and(|path| path.starts_with('/')),
        "incorrect path"
    );

    for (name, expected) in [
        ("content-type", b"application/json".as_slice()),
        ("x-contract", b"wasi-p2".as_slice()),
    ] {
        ensure!(
            request
                .headers
                .iter()
                .any(|header| header.name.eq_ignore_ascii_case(name) && header.value == expected),
            "incorrect {name}"
        );
    }

    let length = request
        .headers
        .iter()
        .find(|header| header.name.eq_ignore_ascii_case("content-length"))
        .context("content-length missing")?;
    let length: usize = std::str::from_utf8(length.value)?.parse()?;
    ensure!(length == BODY.len(), "incorrect request body length");

    Ok(Some(header_length + length))
}

/// Accept one connection with a deadline so failed guests never silently skip fixtures.
fn accept(listener: &TcpListener) -> Result<TcpStream> {
    let start = Instant::now();
    listener.set_nonblocking(true)?;

    while start.elapsed() < WAIT {
        match listener.accept() {
            Ok((stream, _)) => {
                stream.set_nonblocking(false)?;
                return Ok(stream);
            }

            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(POLL_INTERVAL);
            }
            Err(error) => return Err(error.into()),
        }
    }

    bail!("component never reached loopback fixture")
}

/// Observe cancellation while the component remains alive after dropping its future.
fn observe_close(stream: &mut TcpStream, retained: bool) -> Result<String> {
    let start = Instant::now();
    let mut byte = [0_u8; 1];
    stream.set_read_timeout(Some(CLOSE_TIMEOUT))?;
    let result = stream.read(&mut byte);

    if retained {
        let error = result.expect_err("retained request must keep connection pending");
        ensure!(
            matches!(
                error.kind(),
                std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
            ),
            "unexpected retained-request failure: {error}"
        );

        return Ok("fixture observed no EOF during 250ms with response retained".into());
    }

    ensure!(
        matches!(result, Ok(0)),
        "no early TCP EOF after guest drop: {result:?}"
    );

    Ok(format!(
        "fixture observed TCP EOF after {}ms",
        start.elapsed().as_millis()
    ))
}

/// Send intentionally narrow fixtures without another HTTP client implementation.
fn serve(listener: TcpListener, mode: &str) -> Result<String> {
    let mut stream = accept(&listener)?;

    if mode == "tls" {
        let mut hello = [0_u8; 5];
        stream.set_read_timeout(Some(WAIT))?;
        stream.read_exact(&mut hello)?;
        ensure!(
            hello[0] == TLS_HANDSHAKE && hello[1] == 3,
            "TLS ClientHello not observed"
        );
        stream.write_all(b"HTTP/1.1 400 Plaintext\r\nContent-Length: 0\r\n\r\n")?;

        return Ok("fixture observed TLS record; sent invalid TLS reply".into());
    }

    let request = read_request(&mut stream)?;
    let body_start = request_length(&request)?.context("complete headers required")? - BODY.len();
    ensure!(&request[body_start..] == BODY, "incorrect POST bytes");
    let path = format!("POST /{mode} HTTP/1.1\r\n");
    ensure!(
        request.starts_with(path.as_bytes()),
        "incorrect request path"
    );

    match mode {
        "cancel" | "retain" => return observe_close(&mut stream, mode == "retain"),
        "timeout" => {
            thread::sleep(STALL_DURATION);
            return Ok("fixture withheld response headers for 200ms".into());
        }
        "malformed" => stream.write_all(b"this is not HTTP\r\n\r\n")?,
        "upgrade" => stream.write_all(
            b"HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\n\r\n")?,
        _ => reply(&mut stream, mode)?,
    }

    Ok("fixture verified exact POST body and application headers".into())
}

/// Keep success, application failure, and body-limit controls distinct.
fn reply(stream: &mut TcpStream, mode: &str) -> Result<()> {
    let status = if mode == "status" {
        UNAVAILABLE
    } else {
        CREATED
    };

    let body = match mode {
        "oversize" => "012345678901234567890123456789",
        "wrong-body" => "reply:8",
        _ => "reply:7",
    };

    write!(
        stream,
        "HTTP/1.1 {status} Fixture\r\nContent-Length: {}\r\n\
        X-Reply: exact\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )?;

    Ok(())
}

/// Bound every host process; timeout is an explicit failure, never a skip.
fn execute(host: &Path, component: &Path, flags: &[&str], args: &[&str]) -> Result<Output> {
    let mut command = Command::new(host);
    command.arg("run");

    for flag in flags {
        command.args(["-S", flag]);
    }

    let mut child = command
        .current_dir(component.parent().context("component directory")?)
        .arg(component.file_name().context("component filename")?)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let start = Instant::now();

    while child.try_wait()?.is_none() {
        if start.elapsed() > WAIT {
            child.kill()?;
            let output = child.wait_with_output()?;
            bail!(
                "host exceeded {WAIT:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        thread::sleep(POLL_INTERVAL);
    }

    Ok(child.wait_with_output()?)
}

/// Refuse a core module and record the actual component import names.
fn imports(component: &Path) -> Result<String> {
    let bytes = std::fs::read(component)?;
    let mut lines = Vec::new();
    let mut depth = 0_u32;
    let mut payloads = Parser::new(0).parse_all(&bytes);
    ensure!(
        matches!(
            payloads.next().transpose()?,
            Some(Payload::Version {
                encoding: Encoding::Component,
                ..
            })
        ),
        "expected a real component"
    );

    for payload in payloads {
        match payload? {
            Payload::ModuleSection { .. } | Payload::ComponentSection { .. } => depth += 1,
            Payload::End(_) if depth > 0 => depth -= 1,
            Payload::ComponentImportSection(section) if depth == 0 => lines.extend(
                section
                    .into_iter()
                    .map(|import| import.map(|entry| entry.name.0.to_owned()))
                    .collect::<std::result::Result<Vec<_>, _>>()?,
            ),
            _ => {}
        }
    }

    lines.sort();
    lines.dedup();
    ensure!(
        lines
            .iter()
            .any(|line| line.starts_with("wasi:http/outgoing-handler@")),
        "missing HTTP import"
    );
    Ok(lines.join("\n") + "\n")
}

/// Capture success and failure text without treating a successful launch as a passing proof.
fn record(label: &str, output: Output, success: bool, marker: &str) -> Result<String> {
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    let transcript = format!("CASE {label}\nexit={}\n{stdout}{stderr}\n", output.status);
    ensure!(
        output.status.success() == success,
        "unexpected exit: {transcript}"
    );
    ensure!(
        transcript.contains(marker),
        "missing required evidence {marker}: {transcript}"
    );

    Ok(transcript)
}

/// Bound each fixture and require both guest and server observations.
fn network_probes(host: &Path, component: &Path, transcript: &mut String) -> Result<()> {
    for mode in [
        "success",
        "status",
        "oversize",
        "malformed",
        "timeout",
        "cancel",
        "retain",
        "upgrade",
        "tls",
        "wrong-body",
    ] {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let authority = listener.local_addr()?.to_string();
        let fixture = thread::spawn(move || serve(listener, mode));
        let output = execute(host, component, &["http=y"], &[mode, &authority]);
        let observation = fixture
            .join()
            .map_err(|_| anyhow::anyhow!("fixture panicked"))??;
        let success = mode != "wrong-body";
        let marker = if success {
            format!("PASS {mode}")
        } else {
            "assertion".into()
        };

        *transcript += &record(mode, output?, success, &marker)?;
        *transcript += &format!("{observation}\n\n");
    }

    Ok(())
}

/// Missing host imports and denied entropy must terminate instead of returning success.
fn capability_probes(host: &Path, component: &Path, transcript: &mut String) -> Result<()> {
    *transcript += &record(
        "http-unavailable",
        execute(host, component, &["http=n"], &["success", "127.0.0.1:1"])?,
        false,
        "wasi:http",
    )?;
    *transcript += &record(
        "random-limit",
        execute(
            host,
            component,
            &["http=y", "max-random-size=16"],
            &["entropy-clock"],
        )?,
        false,
        "random",
    )?;

    Ok(())
}

/// Run every required case against the pinned external Wasmtime executable.
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let host = PathBuf::from(args.get(1).context("pass pinned Wasmtime path")?);
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("workspace parent")?;
    let component = root.join("target/wasm32-wasip2/debug/solana_kit_wasi_contract_guest.wasm");
    let version = Command::new(&host).arg("--version").output()?;
    ensure!(version.status.success(), "host version command failed");
    let version = String::from_utf8(version.stdout)?;
    ensure!(
        version == "wasmtime 50.0.0 (ebe5e8df1 2026-10-20)\n",
        "incorrect host pin: {version}"
    );

    std::fs::write(root.join("evidence/imports.txt"), imports(&component)?)?;
    let mut transcript = format!("{version}flags: -S http=y (no network inheritance)\n");
    transcript += &record(
        "entropy-clock",
        execute(&host, &component, &["http=y"], &["entropy-clock"])?,
        true,
        "PASS entropy-filled=32",
    )?;
    network_probes(&host, &component, &mut transcript)?;
    capability_probes(&host, &component, &mut transcript)?;
    transcript.truncate(transcript.trim_end().len());
    transcript.push('\n');
    std::fs::write(root.join("evidence/execution.txt"), &transcript)?;
    print!("{transcript}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Negative controls make fixture validation fail if request semantics drift.
    #[test]
    fn request_parser_rejects_wrong_method_headers_and_lengths() {
        let valid = b"POST /success HTTP/1.1\r\nContent-Type: application/json\r\n\
            X-Contract: wasi-p2\r\nContent-Length: 8\r\n\r\n{\"id\":7}";
        assert_eq!(
            request_length(valid).expect("valid request"),
            Some(valid.len())
        );
        assert!(request_length(b"GET / HTTP/1.1\r\n\r\n").is_err());
        assert!(request_length(b"POST / HTTP/1.1\r\nContent-Length: 8\r\n\r\n").is_err());
        let wrong = String::from_utf8_lossy(valid).replace("Length: 8", "Length: 9");
        assert!(request_length(wrong.as_bytes()).is_err());
        assert_eq!(request_length(b"POST /").expect("partial"), None);
    }
}
