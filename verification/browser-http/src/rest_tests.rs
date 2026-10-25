use futures::{FutureExt, poll};
use solana_kit_browser::{BrowserHttp, BrowserRest, HttpConfig};
use solana_kit_client::{
    TransportFailureKind as Kind,
    rest::{Body, Method, Request, Transport},
};
use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

const STATE_ATTEMPTS: usize = 40;
const STATE_POLL_MILLIS: i32 = 25;

fn adapter(path: &str) -> BrowserRest<u8> {
    let endpoint = format!("{}{path}", env!("SOLANA_KIT_BROWSER_FIXTURE"));
    let http = BrowserHttp::new(&endpoint, HttpConfig::default()).unwrap();
    BrowserRest::new(vec![(0, http)], 1000).unwrap()
}

fn request() -> Request<u8> {
    Request {
        service: 0,
        method: Method::Get,
        path_segments: vec!["a/b".into()],
        query: vec![("k".into(), "a b".into()), ("k".into(), "two".into())],
        body: Body::Absent,
        max_reply_bytes: 100,
    }
}

#[wasm_bindgen_test]
async fn public_rest_exact_methods_and_local_owner() {
    let routes = adapter("/rest/");
    let local = Rc::new(7);
    let call = async {
        let reply = routes.exchange(request()).await.unwrap();
        assert_eq!(reply.status, 200);
        assert_eq!(reply.body, b"rest");
        assert_eq!(*local, 7);
    };
    call.await;

    for (path, method, body) in [
        ("post", Method::Post, Body::Json(b"null".to_vec())),
        ("form", Method::Post, Body::Form(b"x=a+b".to_vec())),
    ] {
        let mut next = request();
        next.method = method;
        next.path_segments = vec![path.into()];
        next.query.clear();
        next.body = body;
        assert_eq!(routes.exchange(next).await.unwrap().body, b"rest");
    }
}

#[wasm_bindgen_test]
async fn public_rest_denial_bounds_and_owned_drop() {
    let routes = adapter("/rest/");
    let mut unknown = request();
    unknown.service = 1;
    assert_eq!(
        routes.exchange(unknown).await.unwrap_err().kind,
        Kind::Denied
    );
    let mut get_body = request();
    get_body.body = Body::Json(b"null".to_vec());
    assert_eq!(
        routes.exchange(get_body).await.unwrap_err().kind,
        Kind::Denied
    );
    let mut bounded = request();
    bounded.max_reply_bytes = 3;
    assert_eq!(
        routes.exchange(bounded).await.unwrap_err().kind,
        Kind::BodyLimit
    );

    let quiet = adapter("/quiet-headers-rest");
    let mut next = request();
    next.path_segments.clear();
    next.query.clear();
    let mut exchange = quiet.exchange(next).boxed_local();
    assert!(poll!(exchange.as_mut()).is_pending());
    wait_for_state("true,false").await;
    drop(exchange);
    wait_for_state("true,true").await;
}

async fn wait_for_state(expected: &str) {
    for _ in 0..STATE_ATTEMPTS {
        if super::tests::state("/quiet-headers-rest").await == expected {
            return;
        }

        super::tests::pause(STATE_POLL_MILLIS).await;
    }

    panic!("REST fixture never reached {expected}");
}
