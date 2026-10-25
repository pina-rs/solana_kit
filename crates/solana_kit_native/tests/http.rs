#[path = "support/rest.rs"]
mod rest;
mod support;
use solana_kit_client::{
    Client, Limits, Transport, TransportFailureKind as Kind, methods::Balance,
};
use solana_kit_core::Address;
use solana_kit_native::{HeaderValue, HttpConfig, NativeHttp};
use std::{error::Error, sync::Arc, time::Duration};
use support::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::oneshot,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_client_executes_through_generic_send_spawn() {
    let body = br#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":18446744073709551615},"value":18446744073709551615}}"#;
    let (address, server) = server(response(200, body)).await;
    let mut config = HttpConfig::default();
    config.headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer synthetic-local-token"),
    );
    let adapter = NativeHttp::new(&format!("http://{address}/rpc?fixture=1"), config).unwrap();
    let result = spawn_balance(adapter).await;

    assert_eq!(result.value, u64::MAX);
    assert_eq!(result.context.slot, u64::MAX);
    let request = server.await.unwrap();
    let request = String::from_utf8(request).unwrap();

    assert!(request.starts_with("POST /rpc?fixture=1 HTTP/1.1\r\n"));
    assert!(request.contains("content-type: application/json\r\n"));
    assert!(request.contains("authorization: Bearer synthetic-local-token\r\n"));
    assert!(request.ends_with(r#"{"jsonrpc":"2.0","id":1,"method":"getBalance","params":["11111111111111111111111111111111"]}"#));
}

async fn spawn_balance<T>(transport: T) -> solana_kit_client::methods::Contextual<u64>
where
    T: Transport + Send + Sync + 'static,
    for<'a> T::Exchange<'a>: Send,
{
    let client = Arc::new(Client::new(transport, Limits::default()));
    tokio::spawn(async move {
        client
            .call(Balance(Address::from_bytes([0; 32]), None))
            .await
            .unwrap()
    })
    .await
    .unwrap()
}

#[tokio::test]
async fn body_budgets_apply_to_fixed_chunked_and_close_delimited_bodies() {
    for bytes in [
        response(200, b"1234"),
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2\r\n12\r\n2\r\n34\r\n0\r\n\r\n"
            .to_vec(),
        b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n1234".to_vec(),
    ] {
        let reply = raw(bytes.clone(), 4, HttpConfig::default()).await.unwrap();
        assert_eq!(reply.body, b"1234");
        let error = raw(bytes, 3, HttpConfig::default()).await.unwrap_err();
        assert_eq!(error.kind, Kind::BodyLimit);
    }
}

#[tokio::test]
async fn status_and_bounded_error_body_survive() {
    let reply = raw(response(503, b"try later"), 9, HttpConfig::default())
        .await
        .unwrap();
    assert_eq!(reply.status, 503);
    assert_eq!(reply.body, b"try later");
}

#[tokio::test]
async fn malformed_truncated_and_lying_framing_rejects() {
    for bytes in [
        b"garbage\r\n\r\n".as_slice(),
        b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nabc",
        b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\nContent-Length: 4\r\n\r\nabc",
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nZ\r\nabc\r\n",
    ] {
        let error = raw(bytes.to_vec(), 1024, HttpConfig::default())
            .await
            .unwrap_err();
        assert!(matches!(error.kind, Kind::InvalidResponse | Kind::Io));
        assert!(error.source().is_some());
    }

    // Content-Length defines the response boundary; surplus bytes are not a body.
    let reply = raw(
        b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\nabcdef".to_vec(),
        3,
        HttpConfig::default(),
    )
    .await
    .unwrap();
    assert_eq!(reply.body, b"abc");
}

#[tokio::test]
async fn parser_enforces_header_byte_and_field_budgets() {
    let config = HttpConfig {
        max_header_bytes: 128,
        ..HttpConfig::default()
    };
    let bytes = format!(
        "HTTP/1.1 200 OK\r\nX-Huge: {}\r\nContent-Length: 0\r\n\r\n",
        "a".repeat(256)
    );
    assert_eq!(
        raw(bytes.into_bytes(), 0, config).await.unwrap_err().kind,
        Kind::HeaderLimit
    );
    let config = HttpConfig {
        max_headers: 1,
        ..HttpConfig::default()
    };
    assert_eq!(
        raw(response(200, b""), 0, config).await.unwrap_err().kind,
        Kind::HeaderLimit
    );
}

#[tokio::test]
async fn redirects_are_returned_without_forwarding_authentication() {
    let destination = listener().await;
    let address = destination.local_addr().unwrap();
    let bytes = format!(
        "HTTP/1.1 307 Redirect\r\nLocation: http://{address}/stolen\r\nContent-Length: 0\r\n\r\n"
    );
    let mut config = HttpConfig::default();
    config.headers.insert(
        "authorization",
        HeaderValue::from_static("synthetic-local-secret"),
    );
    assert_eq!(
        raw(bytes.into_bytes(), 0, config).await.unwrap().status,
        307
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(100), destination.accept())
            .await
            .is_err()
    );
}

#[tokio::test]
async fn deadline_covers_headers_and_body_and_closes_owned_socket() {
    for headers in [false, true] {
        let listener = listener().await;
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            read_request(&mut socket).await;

            if headers {
                socket
                    .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\n")
                    .await
                    .unwrap();
            }

            assert_eq!(eof(&mut socket).await.unwrap(), 0);
        });
        let config = HttpConfig {
            exchange_timeout: Duration::from_millis(150),
            ..HttpConfig::default()
        };
        let adapter = NativeHttp::new(&format!("http://{address}/"), config).unwrap();
        assert_eq!(
            adapter.exchange(request(100)).await.unwrap_err().kind,
            Kind::Deadline
        );
        server.await.unwrap();
    }
}

#[tokio::test]
async fn retained_quiet_exchange_stays_open_then_abort_closes_it() {
    let listener = listener().await;
    let address = listener.local_addr().unwrap();
    let (ready, received) = oneshot::channel();
    let (check, checked) = oneshot::channel();

    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        read_request(&mut socket).await;
        ready.send(()).unwrap();
        let pending = tokio::time::timeout(Duration::from_millis(100), socket.read(&mut [0])).await;
        assert!(pending.is_err(), "retained exchange unexpectedly closed");
        check.send(()).unwrap();
        assert_eq!(eof(&mut socket).await.unwrap(), 0);
    });
    let adapter = NativeHttp::new(&format!("http://{address}/"), HttpConfig::default()).unwrap();
    let call = tokio::spawn(async move { adapter.exchange(request(100)).await });
    received.await.unwrap();
    checked.await.unwrap();
    call.abort();
    assert!(call.await.unwrap_err().is_cancelled());
    server.await.unwrap();
}

#[test]
fn endpoints_headers_and_debug_values_do_not_leak_credentials() {
    for endpoint in [
        "ftp://localhost/",
        "http://user:secret@localhost/",
        "http://localhost/#secret",
        "http://localhost:99999/",
        "/relative",
        "http://local host/",
    ] {
        assert!(NativeHttp::new(endpoint, HttpConfig::default()).is_err());
    }

    for name in [
        "host",
        "content-length",
        "content-type",
        "transfer-encoding",
        "connection",
        "upgrade",
        "trailer",
        "expect",
        "proxy-authorization",
    ] {
        let mut config = HttpConfig::default();
        config.headers.insert(
            solana_kit_native::HeaderName::from_bytes(name.as_bytes()).unwrap(),
            HeaderValue::from_static("synthetic-private"),
        );
        assert!(NativeHttp::new("http://localhost/", config).is_err());
    }

    let mut config = HttpConfig::default();
    config.headers.insert(
        "authorization",
        HeaderValue::from_static("synthetic-private"),
    );
    assert!(!format!("{config:?}").contains("synthetic-private"));
    let adapter = NativeHttp::new("http://localhost/?synthetic-private", config).unwrap();
    assert!(!format!("{adapter:?}").contains("synthetic-private"));
}

#[path = "support/tls.rs"]
mod tls;

#[tokio::test]
async fn known_oversize_rejects_before_waiting_for_declared_body() {
    let listener = listener().await;
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        read_request(&mut socket).await;
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 65536\r\n\r\n")
            .await
            .unwrap();
        assert_eq!(eof(&mut socket).await.unwrap(), 0);
    });
    let adapter = NativeHttp::new(&format!("http://{address}/"), HttpConfig::default()).unwrap();
    assert_eq!(
        adapter.exchange(request(1)).await.unwrap_err().kind,
        Kind::BodyLimit
    );
    server.await.unwrap();
}

#[tokio::test]
async fn exact_header_byte_boundary_and_trailer_budget_are_enforced() {
    let bytes = response(200, b"");
    let exact = bytes.len();
    assert!(
        raw(
            bytes.clone(),
            0,
            HttpConfig {
                max_header_bytes: exact,
                ..HttpConfig::default()
            }
        )
        .await
        .is_ok()
    );
    assert_eq!(
        raw(
            bytes,
            0,
            HttpConfig {
                max_header_bytes: exact - 1,
                ..HttpConfig::default()
            }
        )
        .await
        .unwrap_err()
        .kind,
        Kind::HeaderLimit
    );
    let bytes = format!(
        "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n0\r\nX-Large: {}\r\n\r\n",
        "a".repeat(512)
    );
    let error = raw(
        bytes.into_bytes(),
        0,
        HttpConfig {
            max_header_bytes: 128,
            ..HttpConfig::default()
        },
    )
    .await
    .unwrap_err();
    assert!(matches!(
        error.kind,
        Kind::InvalidResponse | Kind::HeaderLimit | Kind::Io
    ));
}

#[tokio::test]
async fn dropping_directly_polled_exchange_closes_without_a_driver_task() {
    let listener = listener().await;
    let address = listener.local_addr().unwrap();
    let (ready, received) = oneshot::channel();
    let (check, checked) = oneshot::channel();

    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        read_request(&mut socket).await;
        ready.send(()).unwrap();
        assert!(
            tokio::time::timeout(Duration::from_millis(100), socket.read(&mut [0]))
                .await
                .is_err()
        );
        check.send(()).unwrap();
        assert_eq!(eof(&mut socket).await.unwrap(), 0);
    });
    let adapter = NativeHttp::new(&format!("http://{address}/"), HttpConfig::default()).unwrap();
    let mut exchange = Box::pin(adapter.exchange(request(1)));
    let observe = async {
        received.await.unwrap();
        checked.await.unwrap();
    };
    tokio::select! {
        result = &mut exchange => panic!("quiet exchange completed: {result:?}"),
        () = observe => {}
    }
    drop(exchange);
    server.await.unwrap();
}
