use futures::{FutureExt, poll};
use solana_kit_browser::{BrowserHttp, HttpConfig};
use solana_kit_client::{
    Client, Error, Limits, Request, Transport, TransportFailureKind as Kind,
    methods::{Balance, ReadConfig},
};
use solana_kit_core::Address;
use std::{rc::Rc, time::Duration};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
wasm_bindgen_test_configure!(run_in_browser);

fn endpoint(path: &str) -> String {
    format!("{}{path}", env!("SOLANA_KIT_BROWSER_FIXTURE"))
}
fn adapter(path: &str, millis: u64) -> BrowserHttp {
    BrowserHttp::new(
        &endpoint(path),
        HttpConfig {
            exchange_timeout: Duration::from_millis(millis),
            ..Default::default()
        },
    )
    .unwrap()
}
async fn pause(millis: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, millis)
            .unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}
async fn raw(
    path: &str,
    max: usize,
) -> Result<solana_kit_client::Reply, solana_kit_client::TransportError> {
    adapter(path, 1000)
        .exchange(Request {
            body: b"{}".to_vec(),
            max_reply_bytes: max,
        })
        .await
}
async fn state(key: &str) -> String {
    String::from_utf8(raw(&format!("/state/{key}"), 100).await.unwrap().body).unwrap()
}

#[wasm_bindgen_test]
async fn typed_exact_request_and_local_rc() {
    let document = js_sys::Reflect::get(&js_sys::global(), &"document".into()).unwrap();
    js_sys::Reflect::set(
        &document,
        &"cookie".into(),
        &"fixture_ambient=must_not_send; SameSite=Lax".into(),
    )
    .unwrap();
    let cookies = js_sys::Reflect::get(&document, &"cookie".into())
        .unwrap()
        .as_string()
        .unwrap();
    assert!(cookies.contains("fixture_ambient=must_not_send"));

    let client = Client::new(adapter("/balance", 1000), Limits::default());
    let local = Rc::new(73);
    let call = async {
        let value = client
            .call(Balance(
                Address::from_bytes([0; 32]),
                Some(ReadConfig::default()),
            ))
            .await
            .unwrap();
        assert_eq!(value.value, u64::MAX);
        assert_eq!(value.context.slot, u64::MAX);
        assert_eq!(*local, 73);
    };
    let mut call = Box::pin(call);
    assert!(poll!(call.as_mut()).is_pending());
    call.await;
    js_sys::Reflect::set(
        &document,
        &"cookie".into(),
        &"fixture_ambient=; Max-Age=0".into(),
    )
    .unwrap();
}

#[wasm_bindgen_test]
async fn status_stream_bounds_and_network_failures() {
    assert_eq!(raw("/status", 100).await.unwrap().status, 429);
    let client = Client::new(adapter("/status", 1000), Limits::default());
    assert!(matches!(
        client
            .call(Balance(
                Address::from_bytes([0; 32]),
                Some(ReadConfig::default())
            ))
            .await,
        Err(Error::Status(429))
    ));

    for path in ["/exact", "/chunked", "/hint"] {
        assert_eq!(raw(path, 6).await.unwrap().body, b"abcdef");
        assert_eq!(raw(path, 5).await.unwrap_err().kind, Kind::BodyLimit);
        assert_eq!(raw(path, 0).await.unwrap_err().kind, Kind::BodyLimit);
    }

    assert_eq!(raw("/lie", 100).await.unwrap_err().kind, Kind::Io);
    assert_eq!(raw("/denied", 100).await.unwrap_err().kind, Kind::Io);
    let unavailable = BrowserHttp::new("http://127.0.0.1:1/rpc", HttpConfig::default()).unwrap();
    assert_eq!(
        unavailable
            .exchange(Request {
                body: vec![],
                max_reply_bytes: 100
            })
            .await
            .unwrap_err()
            .kind,
        Kind::Io
    );
}

#[wasm_bindgen_test]
async fn decompressed_body_limits() {
    for path in ["/gzip", "/gzip-hidden"] {
        assert_eq!(raw(path, 64).await.unwrap().body, vec![b'a'; 64]);
        assert_eq!(raw(path, 63).await.unwrap_err().kind, Kind::BodyLimit);
        assert_eq!(raw(path, 24).await.unwrap_err().kind, Kind::BodyLimit);
    }
}

#[wasm_bindgen_test]
async fn total_deadline_and_owned_drop() {
    let http = adapter("/quiet-unpolled", 2000);
    drop(http.exchange(Request {
        body: vec![],
        max_reply_bytes: 2000,
    }));
    assert_eq!(state("/quiet-unpolled").await, "false,false");

    for path in ["/quiet-headers-timeout", "/quiet-body-timeout"] {
        let http = adapter(path, 500);
        assert_eq!(
            http.exchange(Request {
                body: vec![],
                max_reply_bytes: 2000
            })
            .await
            .unwrap_err()
            .kind,
            Kind::Deadline
        );
        pause(50).await;
        assert_eq!(state(path).await, "true,true");
    }

    for path in ["/quiet-headers-drop", "/quiet-body-drop"] {
        let http = adapter(path, 2000);
        let mut exchange = http
            .exchange(Request {
                body: vec![],
                max_reply_bytes: 2000,
            })
            .boxed_local();
        assert!(poll!(exchange.as_mut()).is_pending());
        pause(100).await;
        assert_eq!(state(path).await, "true,false");
        drop(exchange);
        pause(100).await;

        assert_eq!(state(path).await, "true,true");
    }
}

#[wasm_bindgen_test]
async fn authentication_redirect_and_validation() {
    let config = HttpConfig {
        headers: vec![
            ("authorization".into(), "fixture-only".into()),
            ("x-client".into(), "synthetic".into()),
        ],
        ..Default::default()
    };
    let auth = BrowserHttp::new(&endpoint("/auth"), config.clone()).unwrap();
    assert_eq!(
        auth.exchange(Request {
            body: vec![],
            max_reply_bytes: 10
        })
        .await
        .unwrap()
        .body,
        b"ok"
    );
    let redirect = BrowserHttp::new(&endpoint("/redirect"), config).unwrap();
    assert_eq!(
        redirect
            .exchange(Request {
                body: vec![],
                max_reply_bytes: 10
            })
            .await
            .unwrap_err()
            .kind,
        Kind::Io
    );
    assert_eq!(state("sink").await, "false,false");
    assert!(!format!("{auth:?}").contains("fixture-only"));
}

#[wasm_bindgen_test]
fn configuration_validation() {
    for url in [
        "",
        "/rpc",
        "http:example.com",
        "http:///example.com",
        "ftp://example.com",
        "http://user:pass@example.com",
        "http://@example.com",
        "http://example.com/#",
        "http://example.com:65536",
        "http://example.com:+80",
        "http://example.com/\n",
        "http:\\example.com",
    ] {
        assert!(
            BrowserHttp::new(url, HttpConfig::default()).is_err(),
            "accepted invalid endpoint"
        );
    }
    for name in [
        "host",
        "cookie",
        "authorization\r\n",
        "content-length",
        "content-type",
        "origin",
        "referer",
        "sec-token",
        "proxy-authorization",
        "user-agent",
        "connection",
        "transfer-encoding",
    ] {
        let config = HttpConfig {
            headers: vec![(name.into(), "test".into())],
            ..Default::default()
        };
        assert!(
            BrowserHttp::new(&endpoint("/auth"), config).is_err(),
            "accepted invalid header"
        );
    }

    for duration in [Duration::ZERO, Duration::from_millis(i32::MAX as u64 + 1)] {
        assert!(
            BrowserHttp::new(
                &endpoint("/auth"),
                HttpConfig {
                    exchange_timeout: duration,
                    ..Default::default()
                }
            )
            .is_err()
        );
    }
}
