//! Request contracts compare exact decoded pairs with the independent Dart receipt.
use super::{
    LISTING, UPDATE, client,
    clock::{Manual, run},
    peer::{Peer, reply},
};
use solana_kit_client::rest::{self, Budgets, JsonError, Method};
use solana_kit_pyth::hermes::{AssetType, Client, Encoding, Error, Policy, UpdateOptions};

#[test]
fn listing_preserves_filter_names_unicode_and_custom_root_prefix() {
    let peer = Peer::new([reply(200, LISTING)]);
    let clock = Manual::default();
    let client = client(peer.clone(), clock.clone(), Policy::default());
    let feeds = run(
        client.get_price_feeds(Some("btc +/雪"), Some(AssetType::Crypto)),
        &clock,
    )
    .unwrap();
    assert_eq!(feeds[0].id, "feed-one");

    let actual = peer.requests();
    assert_eq!(actual[0].path_segments, ["v2", "price_feeds"]);
    assert_eq!(
        actual[0].query,
        [
            ("query".into(), "btc +/雪".into()),
            ("asset_type".into(), "crypto".into())
        ]
    );
    assert_request(&actual[0]);
    assert_eq!(
        rest::request_target(
            "/root",
            &actual[0].path_segments,
            &actual[0].query,
            Budgets::default().bytes
        )
        .unwrap(),
        "/root/v2/price_feeds?query=btc%20%2B%2F%E9%9B%AA&asset_type=crypto"
    );
    assert_eq!((clock.now(), clock.pending()), (0, 0));
}

#[test]
fn latest_preserves_duplicate_ids_and_default_option_order() {
    let peer = Peer::new([reply(200, UPDATE)]);
    let clock = Manual::default();
    let client = client(peer.clone(), clock.clone(), Policy::default());
    run(
        client.get_latest_price_updates(
            &["feed-one", "0xfeed two", "feed-one"],
            UpdateOptions::default(),
        ),
        &clock,
    )
    .unwrap();

    let actual = peer.requests();
    assert_eq!(
        actual[0].path_segments,
        ["v2", "updates", "price", "latest"]
    );
    assert_eq!(
        actual[0].query,
        [
            ("ids[]", "feed-one"),
            ("ids[]", "0xfeed two"),
            ("ids[]", "feed-one"),
            ("encoding", "hex"),
            ("parsed", "false")
        ]
        .map(|(key, value)| (key.into(), value.into()))
    );
    assert_request(&actual[0]);
}

#[test]
fn historical_preserves_signed_time_and_requested_update_options() {
    const TIME: i64 = -123;
    let peer = Peer::new([reply(200, UPDATE)]);
    let clock = Manual::default();
    let client = client(peer.clone(), clock.clone(), Policy::default());
    run(
        client.get_price_updates_at_timestamp(
            TIME,
            &["feed-one"],
            UpdateOptions {
                encoding: Encoding::Base64,
                parsed: true,
                ignore_invalid_price_ids: true,
            },
        ),
        &clock,
    )
    .unwrap();

    let actual = peer.requests();
    assert_eq!(actual[0].path_segments, ["v2", "updates", "price", "-123"]);
    assert_eq!(
        actual[0].query,
        [
            ("ids[]", "feed-one"),
            ("encoding", "base64"),
            ("parsed", "true"),
            ("ignore_invalid_price_ids", "true")
        ]
        .map(|(key, value)| (key.into(), value.into()))
    );
    assert_request(&actual[0]);
}

/// Every route is an absent-body GET with the same explicit receive ceiling.
fn assert_request(request: &rest::Request<solana_kit_pyth::hermes::Service>) {
    assert_eq!(request.method, Method::Get);
    assert!(request.body.as_deref().is_none());
    assert_eq!(request.max_reply_bytes, Budgets::default().bytes);
}

#[test]
fn input_budgets_precede_transport_effects() {
    for limits in [
        Budgets {
            bytes: 1,
            ..Budgets::default()
        },
        Budgets {
            items: 0,
            ..Budgets::default()
        },
        Budgets {
            string_bytes: 0,
            ..Budgets::default()
        },
    ] {
        let peer = Peer::new([]);
        let clock = Manual::default();
        let client = Client::new(peer.clone(), clock.clone(), Policy::default(), limits);
        assert!(
            run(
                client.get_latest_price_updates(&["feed-one"], UpdateOptions::default()),
                &clock
            )
            .is_err()
        );
        assert_eq!(peer.count(), 0);
        assert_eq!(clock.pending(), 0);
    }
}

#[test]
fn malformed_success_and_oversized_error_do_not_retry() {
    let peer = Peer::new([reply(200, br#"[{"id":"a","id":"b"}]"#)]);
    let clock = Manual::default();
    let client = client(peer.clone(), clock.clone(), Policy::default());
    assert!(matches!(
        run(client.get_price_feeds(None, None), &clock),
        Err(Error::Json(JsonError::Schema(_)))
    ));
    assert_eq!(peer.count(), 1);

    let limits = Budgets {
        bytes: 32,
        ..Budgets::default()
    };
    let peer = Peer::new([reply(503, &[b'x'; 33])]);
    let client = Client::new(peer.clone(), clock.clone(), Policy::default(), limits);
    assert!(matches!(
        run(client.get_price_feeds(None, None), &clock),
        Err(Error::Json(JsonError::Bytes))
    ));
    assert_eq!(peer.count(), 1);
}
