//! Minimal bounded fixture HTTP, never a production HTTP parser.
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
pub type Events = Arc<Mutex<BTreeMap<String, (bool, bool)>>>;
const CORS: &str = "Access-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: content-type, authorization, x-client\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Expose-Headers: content-encoding\r\n";
const OK: u16 = 200;
const EMPTY: u16 = 204;
const REDIRECT: u16 = 307;
const LIMITED: u16 = 429;
const SOCKET_TIMEOUT: Duration = Duration::from_secs(5);
const POLL_INTERVAL: Duration = Duration::from_millis(50);
const MAX_HEADERS: usize = 16384;

/// Quiet sockets and redirect sinks retain independent observations of browser behavior.
pub fn serve(mut stream: TcpStream, events: Events, sink_url: &str) -> std::io::Result<()> {
    stream.set_nonblocking(false)?;
    stream.set_read_timeout(Some(SOCKET_TIMEOUT))?;
    stream.set_write_timeout(Some(SOCKET_TIMEOUT))?;
    let Some(request) = incoming(&mut stream)? else {
        return Ok(());
    };

    if request.first.starts_with("OPTIONS ") {
        let cors = if request.path == "/denied" { "" } else { CORS };

        return respond(&mut stream, EMPTY, cors, b"");
    }

    if let Some(key) = request.path.strip_prefix("/state/") {
        let event = events.lock().unwrap().get(key).copied().unwrap_or_default();
        return respond(
            &mut stream,
            OK,
            CORS,
            format!("{},{}", event.0, event.1).as_bytes(),
        );
    }

    if request.path.starts_with("/quiet-") {
        return quiet(stream, events, request.path);
    }

    route(&mut stream, events, request, sink_url)
}
fn route(
    stream: &mut TcpStream,
    events: Events,
    request: Incoming,
    sink_url: &str,
) -> std::io::Result<()> {
    if request.path.starts_with("/rest/") {
        return rest(stream, request);
    }

    match request.path.as_str() {
        "/balance" => balance(stream, request),
        "/auth" => {
            assert!(
                request.lower.contains("authorization: fixture-only")
                    && request.lower.contains("x-client: synthetic")
            );
            respond(stream, OK, CORS, b"ok")
        }
        "/status" => respond(stream, LIMITED, CORS, b"limited"),
        "/gzip" | "/gzip-hidden" => gzip(stream, request.path == "/gzip-hidden"),
        "/denied" => respond(stream, OK, "", b"hidden"),
        "/redirect" => {
            assert!(request.lower.contains("authorization: fixture-only"));
            respond(
                stream,
                REDIRECT,
                &format!("{CORS}Location: {sink_url}\r\n"),
                b"",
            )
        }
        "/sink" => {
            events.lock().unwrap().insert("sink".into(), (true, false));
            respond(stream, OK, CORS, b"unexpected")
        }
        "/chunked" => write!(
            stream,
            "HTTP/1.1 200 OK\r\n{CORS}Transfer-Encoding: chunked\r\nConnection: close\r\n\r\n3\r\nabc\r\n3\r\ndef\r\n0\r\n\r\n"
        ),
        "/lie" => write!(
            stream,
            "HTTP/1.1 200 OK\r\n{CORS}Content-Length: 10\r\nConnection: close\r\n\r\nabc"
        ),
        "/hint" => respond(
            stream,
            OK,
            &format!("{CORS}Content-Encoding: identity\r\n"),
            b"abcdef",
        ),
        _ => respond(stream, OK, CORS, b"abcdef"),
    }
}
fn gzip(stream: &mut TcpStream, hidden: bool) -> std::io::Result<()> {
    // `gzip -n` of 64 ASCII 'a' bytes gives this reproducible public fixture.
    let body = [
        31, 139, 8, 0, 0, 0, 0, 0, 0, 3, 75, 76, 164, 12, 0, 0, 85, 101, 180, 137, 64, 0, 0, 0,
    ];
    let cors = if hidden {
        CORS.replace("Access-Control-Expose-Headers: content-encoding\r\n", "")
    } else {
        CORS.to_owned()
    };

    respond(
        stream,
        OK,
        &format!("{cors}Content-Encoding: gzip\r\n"),
        &body,
    )
}
fn balance(stream: &mut TcpStream, request: Incoming) -> std::io::Result<()> {
    assert!(request.first.starts_with("POST "));
    assert!(request.lower.contains("content-type: application/json"));
    assert!(
        !request.lower.contains("authorization:")
            && !request.lower.contains("cookie:")
            && !request.lower.contains("referer:")
    );
    assert_eq!(request.body.as_slice(), br#"{"jsonrpc":"2.0","id":1,"method":"getBalance","params":["11111111111111111111111111111111",{}]}"#);
    respond(stream, OK, CORS, br#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":18446744073709551615},"value":18446744073709551615}}"#)
}
struct Incoming {
    first: String,
    path: String,
    lower: String,
    body: Vec<u8>,
}
fn incoming(stream: &mut TcpStream) -> std::io::Result<Option<Incoming>> {
    let Some(data) = headers(stream)? else {
        return Ok(None);
    };
    let header = String::from_utf8(data).expect("ASCII headers");
    let first = header.lines().next().unwrap().to_owned();
    let path = first.split_whitespace().nth(1).unwrap().to_owned();
    let lower = header.to_ascii_lowercase();
    let length: usize = lower
        .lines()
        .find_map(|line| line.strip_prefix("content-length: "))
        .unwrap_or("0")
        .parse()
        .unwrap();
    assert!(length <= MAX_HEADERS, "fixture request body too large");

    let mut body = vec![0; length];
    stream.read_exact(&mut body)?;

    Ok(Some(Incoming {
        first,
        path,
        lower,
        body,
    }))
}
fn headers(stream: &mut TcpStream) -> std::io::Result<Option<Vec<u8>>> {
    let mut data = Vec::new();

    loop {
        let mut byte = [0];

        if stream.read(&mut byte)? == 0 {
            return Ok(None);
        }

        data.push(byte[0]);

        if data.ends_with(b"\r\n\r\n") {
            return Ok(Some(data));
        }

        assert!(
            data.len() <= MAX_HEADERS,
            "fixture request headers too large"
        );
    }
}
fn quiet(mut stream: TcpStream, events: Events, path: String) -> std::io::Result<()> {
    events.lock().unwrap().insert(path.clone(), (true, false));

    if path.starts_with("/quiet-body") {
        write!(
            stream,
            "HTTP/1.1 200 OK\r\n{CORS}Content-Length: 1000\r\nConnection: close\r\n\r\nx"
        )?;
        stream.flush()?;
    }

    stream.set_read_timeout(Some(POLL_INTERVAL))?;
    wait_for_close(&mut stream)?;
    events.lock().unwrap().insert(path, (true, true));

    Ok(())
}
/// An elapsed read timeout alone is not cancellation; only EOF or reset proves it.
fn wait_for_close(stream: &mut TcpStream) -> std::io::Result<()> {
    let deadline = Instant::now() + Duration::from_secs(4);

    loop {
        match stream.read(&mut [0; 64]) {
            Ok(0) => return Ok(()),
            Ok(_) => (),
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
                ) && Instant::now() < deadline => {}
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::ConnectionAborted
                ) =>
            {
                return Ok(());
            }
            Err(error) => return Err(error),
        }
    }
}
fn respond(stream: &mut TcpStream, status: u16, headers: &str, body: &[u8]) -> std::io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status} Fixture\r\n{headers}Content-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(body)
}

fn rest(stream: &mut TcpStream, request: Incoming) -> std::io::Result<()> {
    assert!(!request.lower.contains("cookie:") && !request.lower.contains("referer:"));

    match request.path.as_str() {
        "/rest/a%2Fb?k=a%20b&k=two" => {
            assert!(request.first.starts_with("GET "));
            assert!(request.body.is_empty());
            assert!(!request.lower.contains("content-type:"));
        }
        "/rest/post" => {
            assert!(request.first.starts_with("POST "));
            assert!(request.lower.contains("content-type: application/json"));
            assert_eq!(request.body, b"null");
        }
        "/rest/form" => {
            assert!(request.first.starts_with("POST "));
            assert!(
                request
                    .lower
                    .contains("content-type: application/x-www-form-urlencoded")
            );
            assert_eq!(request.body, b"x=a+b");
        }
        other => panic!("unexpected REST fixture target: {other}"),
    }

    respond(stream, OK, CORS, b"rest")
}
