//! Executable Preview 2 boundary probes; failures terminate the component.

use wasip2::clocks::monotonic_clock;
use wasip2::http::outgoing_handler;
use wasip2::http::types::{
    ErrorCode, Fields, HeaderError, IncomingResponse, Method, OutgoingBody, OutgoingRequest,
    RequestOptions, Scheme,
};
use wasip2::io::streams::StreamError;

const BODY: &[u8] = b"{\"id\":7}";
const LIMIT: usize = 16;
const SLEEP_NANOS: u64 = 20_000_000;
const CONNECT_NANOS: u64 = 500_000_000;
const RESPONSE_NANOS: u64 = 80_000_000;
const CANCEL_NANOS: u64 = 40_000_000;
const AFTER_CANCEL_NANOS: u64 = 300_000_000;
const RETAINED_RESPONSE_NANOS: u64 = 2_000_000_000;

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

/// Make forbidden upgrade headers an explicit failing capability experiment.
fn request_headers(mode: &str) -> Fields {
    let headers = Fields::from_list(&[
        ("content-type".into(), b"application/json".to_vec()),
        ("x-contract".into(), b"wasi-p2".to_vec()),
        ("content-length".into(), BODY.len().to_string().into_bytes()),
    ])
    .expect("valid fixture headers");

    if mode == "upgrade" {
        let result = headers.set("upgrade", &[b"websocket".to_vec()]);
        println!("upgrade-header={result:?}");
        assert!(matches!(result, Err(HeaderError::Forbidden)));
        let result = headers.set("connection", &[b"Upgrade".to_vec()]);
        println!("connection-header={result:?}");
        assert!(matches!(result, Err(HeaderError::Forbidden)));
    }

    headers
}

/// Keep cancellation probes alive beyond the fixture's observation window.
fn request_options(mode: &str) -> RequestOptions {
    let options = RequestOptions::new();
    let response_timeout = if matches!(mode, "cancel" | "retain") {
        RETAINED_RESPONSE_NANOS
    } else {
        RESPONSE_NANOS
    };

    options
        .set_connect_timeout(Some(CONNECT_NANOS))
        .expect("connect timeout supported");
    options
        .set_first_byte_timeout(Some(response_timeout))
        .expect("header timeout supported");
    options
        .set_between_bytes_timeout(Some(RESPONSE_NANOS))
        .expect("body timeout supported");

    options
}

/// Construct one exact POST while keeping transport errors observable.
fn request(authority: &str, mode: &str) -> wasip2::http::types::FutureIncomingResponse {
    let headers = request_headers(mode);
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
    let future =
        outgoing_handler::handle(request, Some(request_options(mode))).expect("request accepted");

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

/// Verify HTTP semantics separately from transport errors and resource lifetime.
fn check_response(mode: &str, response: IncomingResponse) {
    if mode == "upgrade" {
        assert_eq!(response.status(), 101);
        println!("PASS upgrade forbidden-headers; unsolicited-101-is-HTTP-response-only");

        return;
    }

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

    if mode == "retain" {
        sleep(AFTER_CANCEL_NANOS);
        assert!(future.get().is_none());
        drop(future);
        println!("PASS retain request-pending-for-300ms");

        return;
    }

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

    let expected_error = match mode.as_str() {
        "malformed" => Some(ErrorCode::HttpProtocolError),
        "timeout" => Some(ErrorCode::ConnectionReadTimeout),
        "tls" => Some(ErrorCode::TlsProtocolError),
        _ => None,
    };

    if let Some(expected) = expected_error {
        let error = result.expect_err("negative fixture must return transport error");
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&expected)
        );
        println!("PASS {mode} error={error:?}");

        return;
    }

    check_response(mode, result.expect("fixture HTTP response"));
}
