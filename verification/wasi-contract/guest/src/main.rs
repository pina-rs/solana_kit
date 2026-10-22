//! Executable Preview 2 boundary probes; failures terminate the component.

use wasip2::clocks::monotonic_clock;
use wasip2::http::outgoing_handler;
use wasip2::http::types::{Fields, Method, OutgoingBody, OutgoingRequest, RequestOptions, Scheme};
use wasip2::io::streams::StreamError;

const BODY: &[u8] = b"{\"id\":7}";
const LIMIT: usize = 16;
const SLEEP_NANOS: u64 = 20_000_000;
const CONNECT_NANOS: u64 = 500_000_000;
const RESPONSE_NANOS: u64 = 80_000_000;
const CANCEL_NANOS: u64 = 40_000_000;
const AFTER_CANCEL_NANOS: u64 = 300_000_000;

/// Keep a timer resource alive until the host wakes it.
fn sleep(nanos: u64) {
    monotonic_clock::subscribe_duration(nanos).block();
}

/// Check capability-backed entropy without asserting its random value.
fn entropy_clock() {
    let mut caller = [0_u8; 32];
    let random = wasip2::random::random::get_random_bytes(caller.len() as u64);
    assert_eq!(random.len(), caller.len());
    caller.copy_from_slice(&random);
    assert_eq!(caller.as_slice(), random.as_slice());
    let before = monotonic_clock::now();
    let timer = monotonic_clock::subscribe_duration(SLEEP_NANOS);
    assert!(!timer.ready(), "timer fired before requested duration");

    timer.block();
    assert!(timer.ready());
    assert!(monotonic_clock::now() - before >= SLEEP_NANOS);
    println!("PASS entropy-filled=32 monotonic-sleep>=20ms wake-ready");
}

/// Construct one exact POST while keeping transport errors observable.
fn request(authority: &str, mode: &str) -> wasip2::http::types::FutureIncomingResponse {
    let headers = Fields::from_list(&[
        ("content-type".into(), b"application/json".to_vec()),
        ("x-contract".into(), b"wasi-p2".to_vec()),
    ])
    .expect("valid fixture headers");

    if mode == "upgrade" {
        let result = headers.set("upgrade", &[b"websocket".to_vec()]);
        println!("upgrade-header={result:?}");
        let result = headers.set("connection", &[b"Upgrade".to_vec()]);
        println!("connection-header={result:?}");
    }

    let request = OutgoingRequest::new(headers);
    request.set_method(&Method::Post).expect("POST supported");
    request
        .set_authority(Some(authority))
        .expect("valid authority");
    request
        .set_path_with_query(Some(&format!("/{mode}")))
        .expect("valid path");
    let scheme = if mode == "tls" {
        Scheme::Https
    } else {
        Scheme::Http
    };

    request.set_scheme(Some(&scheme)).expect("supported scheme");
    let body = request.body().expect("body available once");
    let stream = body.write().expect("stream available once");
    let options = RequestOptions::new();
    options
        .set_connect_timeout(Some(CONNECT_NANOS))
        .expect("connect timeout supported");
    options
        .set_first_byte_timeout(Some(RESPONSE_NANOS))
        .expect("header timeout supported");
    options
        .set_between_bytes_timeout(Some(RESPONSE_NANOS))
        .expect("body timeout supported");
    let future = outgoing_handler::handle(request, Some(options)).expect("request accepted");

    stream
        .blocking_write_and_flush(BODY)
        .expect("write fixture body");
    drop(stream);
    OutgoingBody::finish(body, None).expect("finish fixture body");

    future
}

/// Bound allocation and report oversized bodies as an error.
fn read_body(response: &wasip2::http::types::IncomingResponse) -> Result<Vec<u8>, String> {
    let body = response.consume().map_err(|()| "body already consumed")?;
    let stream = body.stream().map_err(|()| "stream already consumed")?;
    let mut bytes = Vec::new();

    loop {
        match stream.blocking_read((LIMIT + 1 - bytes.len()) as u64) {
            Ok(chunk) => bytes.extend(chunk),
            Err(StreamError::Closed) => break,
            Err(error) => return Err(format!("stream error: {error:?}")),
        }

        if bytes.len() > LIMIT {
            return Err("response-limit-exceeded".into());
        }
    }

    Ok(bytes)
}

/// Run a single assertion-bearing probe selected by the native harness.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).expect("probe mode required");

    if mode == "entropy-clock" {
        entropy_clock();

        return;
    }

    let authority = args.get(2).expect("loopback authority required");
    let future = request(authority, mode);

    if mode == "cancel" {
        sleep(CANCEL_NANOS);
        assert!(future.get().is_none(), "request must still be pending");
        drop(future);
        println!("cancel-dropped-pending-response");
        sleep(AFTER_CANCEL_NANOS);
        println!("PASS cancel guest-alive-after-drop");

        return;
    }

    future.subscribe().block();
    let result = future
        .get()
        .expect("response ready")
        .expect("response taken once");

    if matches!(mode.as_str(), "malformed" | "timeout" | "upgrade" | "tls") {
        let error = result.expect_err("negative fixture must return transport error");
        println!("PASS {mode} error={error:?}");

        return;
    }

    let response = result.expect("fixture HTTP response");
    let expected_status = if mode == "status" { 503 } else { 201 };
    assert_eq!(response.status(), expected_status);
    assert_eq!(response.headers().get("x-reply"), [b"exact".to_vec()]);
    let body = read_body(&response);

    if mode == "oversize" {
        assert_eq!(body, Err("response-limit-exceeded".into()));
    } else {
        assert_eq!(body.expect("bounded body"), b"reply:7");
    }

    println!("PASS {mode} status={expected_status} bounded-body");
}
