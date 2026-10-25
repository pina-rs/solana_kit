//! Actual REST calls share application routing and the host's existing job owner.
use crate::application::record;
use solana_kit_client::rest::Transport;
use solana_kit_client::{TransportFailureKind, rest};
use solana_kit_leptoskit::GuestRest;

pub async fn run() {
    let denied = GuestRest::new(vec![((), "denied".into())], 1024).expect("denied service");
    let error = denied
        .exchange(request(4))
        .await
        .expect_err("actual host grant");
    assert_eq!(error.kind, TransportFailureKind::Denied);
    record("rest-denied");
    let adapter = GuestRest::new(vec![((), "rest".into())], 1024).expect("installed REST");
    let reply = adapter
        .exchange(request(4))
        .await
        .expect("actual REST response");
    assert_eq!(reply.status, 200);
    assert_eq!(reply.body, b"done");
    record("rest-wire");
    let error = adapter
        .exchange(request(3))
        .await
        .expect_err("bounded REST body");
    assert_eq!(error.kind, TransportFailureKind::BodyLimit);
    record("rest-bounded");
}

fn request(limit: usize) -> rest::Request<()> {
    rest::Request {
        service: (),
        method: rest::Method::Post,
        path_segments: vec!["a/b".into()],
        query: vec![("key".into(), "a b".into()), ("key".into(), "&".into())],
        body: rest::Body::Json(b"null".to_vec()),
        max_reply_bytes: limit,
    }
}
