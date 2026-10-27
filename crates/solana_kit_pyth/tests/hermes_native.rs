//! The public Hermes client uses real sockets, explicit host policy and Tokio time.
#![cfg(all(feature = "hermes", not(target_family = "wasm")))]

#[path = "support/hermes_native.rs"]
mod support;
use solana_kit_client::{
    TransportFailureKind,
    rest::{Budgets, JsonError},
    subscriptions::{Clock, ClockError},
};
use solana_kit_native::{HeaderValue, HttpConfig, NativeHttp, NativeRest};
use solana_kit_pyth::hermes::{
    AssetType, Client, Encoding, Error, Policy, PriceUpdate, Service, UpdateOptions,
};
use std::{future::Future, net::SocketAddr, pin::Pin, time::Duration};
use tokio::{io::AsyncWriteExt, sync::oneshot, task::JoinHandle, time::Instant};

const OK: u16 = 200;
const UNAUTHORIZED: u16 = 401;
const UNAVAILABLE: u16 = 503;
const TIMEOUT_MILLIS: u64 = 5_000;
const MAX_TARGET_BYTES: usize = 4096;

const UPDATE: &[u8] = concat!(
    r#"{"binary":{"encoding":"hex","data":["504e4155"]},"parsed":[{"id":"feed","price":{"#,
    r#""price":"9007199254740993","conf":"18446744073709551615","expo":-8,"publish_time":7},"#,
    r#""ema_price":{"price":"-1","conf":"2","expo":-8,"publish_time":6}}]}"#,
)
.as_bytes();
const LISTING: &[u8] = br#"[{"id":"feed","attributes":{"symbol":"TEST/USD"}}]"#;
const LATEST: &str = concat!(
    "/provider/v2/updates/price/latest?tenant=public&ids%5B%5D=0xAB&ids%5B%5D=BTC%2FUSD%3F",
    "&ids%5B%5D=0xAB&encoding=base64&parsed=true&ignore_invalid_price_ids=true"
);

struct Monotonic(Instant);

impl Clock for Monotonic {
    type Sleep<'a> = Pin<Box<dyn Future<Output = Result<(), ClockError>> + Send + 'a>>;

    fn now(&self) -> u64 {
        self.0.elapsed().as_millis().try_into().unwrap()
    }

    fn sleep_until(&self, deadline: u64) -> Self::Sleep<'_> {
        Box::pin(async move {
            tokio::time::sleep_until(self.0 + Duration::from_millis(deadline)).await;
            Ok(())
        })
    }
}

fn client(address: SocketAddr, budgets: Budgets) -> Client<NativeRest<Service>, Monotonic> {
    let mut config = HttpConfig::default();
    config.headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer synthetic-local-token"),
    );
    config
        .headers
        .insert("x-hermes-fixture", HeaderValue::from_static("public"));
    let http =
        NativeHttp::new(&format!("http://{address}/provider/?tenant=public"), config).unwrap();
    let transport = NativeRest::new(vec![(Service, http)], MAX_TARGET_BYTES).unwrap();
    Client::new(
        transport,
        Monotonic(Instant::now()),
        Policy::new(TIMEOUT_MILLIS, 1, 0).unwrap(),
        budgets,
    )
}

fn check_request(bytes: &[u8], target: &str) {
    let text = std::str::from_utf8(bytes).unwrap();
    assert!(text.starts_with(&format!("GET {target} HTTP/1.1\r\n")));
    assert!(text.contains("authorization: Bearer synthetic-local-token\r\n"));
    assert!(text.contains("x-hermes-fixture: public\r\n"));
    assert!(text.ends_with("\r\n\r\n"));
}

fn check_update(update: &PriceUpdate) {
    assert_eq!(update.binary.encoding, Encoding::Hex);
    assert_eq!(update.binary.data, ["504e4155"]);
    let feeds = update.parsed.as_ref().unwrap();
    assert_eq!(feeds.len(), 1);
    assert_eq!(feeds[0].price.price.to_string(), "9007199254740993");
    assert_eq!(
        feeds[0].price.confidence.to_string(),
        "18446744073709551615"
    );
    assert_eq!(feeds[0].price.exponent, -8);
    assert_eq!(feeds[0].price.publish_time, 7);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn latest_runs_in_send_spawn_with_exact_query_headers_and_integer_results() {
    let (address, peer) = support::server(support::response(OK, UPDATE)).await;
    let client = client(address, Budgets::default());
    let update = tokio::spawn(async move {
        let options = UpdateOptions {
            encoding: Encoding::Base64,
            parsed: true,
            ignore_invalid_price_ids: true,
        };
        client
            .get_latest_price_updates(&["0xAB", "BTC/USD?", "0xAB"], options)
            .await
    })
    .await
    .unwrap()
    .unwrap();
    check_update(&update);
    check_request(&peer.await.unwrap(), LATEST);
}

#[tokio::test]
async fn listing_and_signed_timestamp_use_the_same_explicit_provider_root() {
    let (address, peer) = support::server(support::response(OK, LISTING)).await;
    let feeds = client(address, Budgets::default())
        .get_price_feeds(Some("BTC / USD"), Some(AssetType::Crypto))
        .await
        .unwrap();
    assert_eq!(feeds[0].attributes["symbol"], "TEST/USD");
    check_request(
        &peer.await.unwrap(),
        "/provider/v2/price_feeds?tenant=public&query=BTC%20%2F%20USD&asset_type=crypto",
    );

    let (address, peer) = support::server(support::response(OK, UPDATE)).await;
    let update = client(address, Budgets::default())
        .get_price_updates_at_timestamp(-1, &["id"], UpdateOptions::default())
        .await
        .unwrap();
    check_update(&update);
    check_request(
        &peer.await.unwrap(),
        "/provider/v2/updates/price/-1?tenant=public&ids%5B%5D=id&encoding=hex&parsed=false",
    );
}

async fn replies(bodies: Vec<Vec<u8>>) -> (SocketAddr, JoinHandle<Vec<Vec<u8>>>) {
    let listener = support::listener().await;
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        let mut requests = Vec::new();

        for body in bodies {
            let (mut socket, _) = listener.accept().await.unwrap();
            requests.push(support::read_request(&mut socket).await);
            socket.write_all(&body).await.unwrap();
        }

        requests
    });
    (address, task)
}

#[tokio::test]
async fn transient_response_retries_the_same_actual_request() {
    let (address, peer) = replies(vec![
        support::response(UNAVAILABLE, b"try again"),
        support::response(OK, UPDATE),
    ])
    .await;
    let update = client(address, Budgets::default())
        .get_latest_price_updates(&["id"], UpdateOptions::default())
        .await
        .unwrap();
    check_update(&update);
    let observed = peer.await.unwrap();
    assert_eq!(observed.len(), 2);
    assert_eq!(observed[0], observed[1]);
}

#[tokio::test]
async fn permanent_http_failure_retains_body_without_retry_or_diagnostic_leak() {
    let body = b"synthetic-provider-private-body";
    let (address, peer) = support::server(support::response(UNAUTHORIZED, body)).await;
    let error = client(address, Budgets::default())
        .get_price_feeds(None, None)
        .await
        .unwrap_err();
    assert!(!format!("{error:?} {error}").contains("synthetic"));
    let Error::Http {
        status,
        body: actual,
    } = error
    else {
        panic!("permanent HTTP failure must not retry");
    };

    assert_eq!(status, UNAUTHORIZED);
    assert_eq!(actual, body);
    peer.await.unwrap();
}

#[tokio::test]
async fn malformed_success_and_body_limit_remain_terminal_over_real_sockets() {
    let (address, peer) = support::server(support::response(OK, b"{}")).await;
    let error = client(address, Budgets::default())
        .get_latest_price_updates(&[], UpdateOptions::default())
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Json(JsonError::Schema(_))));
    peer.await.unwrap();

    let (address, peer) = support::server(support::response(OK, UPDATE)).await;
    let budgets = Budgets {
        bytes: 64,
        ..Budgets::default()
    };
    let error = client(address, budgets)
        .get_price_feeds(None, None)
        .await
        .unwrap_err();
    assert!(
        matches!(error, Error::Transport(error) if error.kind == TransportFailureKind::BodyLimit)
    );
    peer.await.unwrap();
}

#[tokio::test]
async fn dropping_the_hermes_operation_closes_its_observed_pending_socket() {
    let listener = support::listener().await;
    let client = client(listener.local_addr().unwrap(), Budgets::default());
    let (accepted, ready) = oneshot::channel();
    let peer = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        support::read_request(&mut socket).await;
        accepted.send(()).unwrap();
        assert_eq!(support::eof(&mut socket).await.unwrap(), 0);
    });
    let mut call = Box::pin(client.get_price_feeds(None, None));
    tokio::select! {
        result = ready => result.unwrap(),
        result = &mut call => panic!("held peer has not replied: {result:?}"),
    }
    drop(call);
    peer.await.unwrap();
}
