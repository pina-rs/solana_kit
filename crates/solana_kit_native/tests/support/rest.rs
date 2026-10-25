use crate::support;

use solana_kit_client::rest::Transport;
use solana_kit_client::{TransportFailureKind as Kind, rest};
use solana_kit_native::{HttpConfig, NativeHttp, NativeRest};

const ROUTE: u8 = 7;
const UNKNOWN_ROUTE: u8 = 8;
const CREATED: u16 = 201;
const OK: u16 = 200;
const NO_CONNECTION_MILLIS: u64 = 80;

fn request(method: rest::Method, body: rest::Body, limit: usize) -> rest::Request<u8> {
    rest::Request {
        service: ROUTE,
        method,
        path_segments: vec!["a/b".into()],
        query: vec![("key".into(), "a b".into()), ("key".into(), "&".into())],
        body,
        max_reply_bytes: limit,
    }
}

fn adapter(address: std::net::SocketAddr) -> NativeRest<u8> {
    let http = NativeHttp::new(
        &format!("http://{address}/root?fixed=1"),
        HttpConfig::default(),
    )
    .unwrap();
    NativeRest::new(vec![(ROUTE, http)], 1024).unwrap()
}

#[tokio::test]
async fn public_rest_preserves_methods_encoded_targets_and_body_types() {
    for (method, body, media, payload) in [
        (
            rest::Method::Post,
            rest::Body::Json(b"null".to_vec()),
            "application/json",
            "null",
        ),
        (
            rest::Method::Put,
            rest::Body::Form(b"a=b%20c".to_vec()),
            "application/x-www-form-urlencoded",
            "a=b%20c",
        ),
    ] {
        let (address, peer) = support::server(support::response(CREATED, b"done")).await;
        let transport = adapter(address);
        let reply = transport.exchange(request(method, body, 4)).await.unwrap();
        assert_eq!(reply.status, CREATED);
        assert_eq!(reply.body, b"done");
        let observed = String::from_utf8(peer.await.unwrap()).unwrap();
        assert!(observed.starts_with(&format!(
            "{} /root/a%2Fb?fixed=1&key=a%20b&key=%26 HTTP/1.1\r\n",
            method.as_str()
        )));
        assert!(observed.contains(&format!("content-type: {media}\r\n")));
        assert!(observed.ends_with(payload));
    }
}

#[tokio::test]
async fn public_rest_denies_unknown_routes_and_bounds_actual_bodies() {
    let listener = support::listener().await;
    let transport = adapter(listener.local_addr().unwrap());
    let mut unknown = request(rest::Method::Get, rest::Body::Absent, 4);
    unknown.service = UNKNOWN_ROUTE;
    assert_eq!(
        transport.exchange(unknown).await.unwrap_err().kind,
        Kind::Denied
    );
    assert!(
        tokio::time::timeout(
            std::time::Duration::from_millis(NO_CONNECTION_MILLIS),
            listener.accept()
        )
        .await
        .is_err()
    );

    let (address, peer) = support::server(support::response(OK, b"oversized")).await;
    let transport = adapter(address);
    let error = transport
        .exchange(request(
            rest::Method::Post,
            rest::Body::Json(b"null".to_vec()),
            3,
        ))
        .await
        .unwrap_err();
    assert_eq!(error.kind, Kind::BodyLimit);
    peer.await.unwrap();
}

#[tokio::test]
async fn public_rest_preserves_actual_framing_errors() {
    let (address, peer) =
        support::server(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nabc".to_vec()).await;
    let error = adapter(address)
        .exchange(request(
            rest::Method::Post,
            rest::Body::Json(b"null".to_vec()),
            64,
        ))
        .await
        .unwrap_err();
    assert!(matches!(error.kind, Kind::InvalidResponse | Kind::Io));
    assert!(std::error::Error::source(&error).is_some());
    peer.await.unwrap();
}

#[tokio::test]
async fn dropping_public_rest_closes_the_observed_pending_socket() {
    let listener = support::listener().await;
    let transport = adapter(listener.local_addr().unwrap());
    let (accepted, ready) = tokio::sync::oneshot::channel();
    let peer = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        support::read_request(&mut socket).await;
        accepted.send(()).unwrap();
        assert_eq!(support::eof(&mut socket).await.unwrap(), 0);
    });
    let mut call = Box::pin(transport.exchange(request(
        rest::Method::Post,
        rest::Body::Json(b"null".to_vec()),
        4,
    )));
    tokio::select! {
        result = ready => result.unwrap(),
        result = &mut call => panic!("peer has not replied: {result:?}"),
    }
    drop(call);
    peer.await.unwrap();
}
