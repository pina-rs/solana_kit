//! Actual Preview2 REST requests retain the same Reactor and HTTP owner.
use solana_kit_client::rest::Transport;
use solana_kit_client::{TransportFailureKind, rest};
use solana_kit_wasi::{Reactor, WasiHttp, WasiRest};

const ROUTE: u8 = 7;
const UNKNOWN_ROUTE: u8 = 8;
const REPLY_BYTES: usize = 64;

pub fn run(reactor: &Reactor, http: WasiHttp, mode: &str) {
    let adapter = WasiRest::new(vec![(ROUTE, http)], 1024).expect("approved root");
    let result = reactor
        .block_on(adapter.exchange(request(mode)))
        .expect("driver");

    match mode {
        "rest-malformed" => {
            let error = result.expect_err("actual truncated REST body");
            assert!(matches!(
                error.kind,
                TransportFailureKind::InvalidResponse | TransportFailureKind::Io
            ));
            assert!(std::error::Error::source(&error).is_some());
        }
        "rest-denied" => assert!(matches!(result,
            Err(error) if error.kind == TransportFailureKind::Denied)),
        "rest-limit" => assert!(matches!(result,
            Err(error) if error.kind == TransportFailureKind::BodyLimit)),
        "rest" => {
            let reply = result.expect("observed real REST response");
            assert_eq!(reply.status, 200);
            assert_eq!(reply.body, b"done");
        }
        _ => panic!("unknown REST case"),
    }

    assert_eq!(reactor.registrations(), 0);
    println!("PASS embedded {mode} actual REST wire and resources");
}

fn request(mode: &str) -> rest::Request<u8> {
    let service = if mode == "rest-denied" {
        UNKNOWN_ROUTE
    } else {
        ROUTE
    };
    let max_reply_bytes = if mode == "rest-limit" { 3 } else { REPLY_BYTES };

    rest::Request {
        service,
        method: rest::Method::Post,
        path_segments: vec!["a/b".into()],
        query: vec![("key".into(), "a b".into()), ("key".into(), "&".into())],
        body: rest::Body::Json(b"null".to_vec()),
        max_reply_bytes,
    }
}
