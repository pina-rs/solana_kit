//! Public-owner protocol execution through deterministic in-memory text frames.
use solana_kit_client::{Error, Limits, methods::Method, subscriptions::*};

const SUBSCRIBE_DEADLINE: u64 = 20;
const CLEANUP_DEADLINE: u64 = 30;
const SERVER: u64 = 7;
const TICK: u64 = 5;
const EARLY_DEADLINE: u64 = 15;
const SHORT_CLEANUP: u64 = 8;

fn connection() -> Connection {
    Connection::new(1, Budget::default())
}
fn send(owner: &mut Connection) -> serde_json::Value {
    serde_json::from_slice(&owner.outbound().expect("outbound frame")).unwrap()
}
fn ingest(owner: &mut Connection, frame: &str) -> Result<(), Failure> {
    owner.ingest(1, frame.as_bytes())
}
fn ack(owner: &mut Connection, id: u32, result: &str) -> Result<(), Failure> {
    ingest(
        owner,
        &format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#),
    )
}
fn notification(
    owner: &mut Connection,
    method: &str,
    server: u64,
    result: &str,
) -> Result<(), Failure> {
    ingest(
        owner,
        &format!(
            r#"{{"jsonrpc":"2.0","method":"{method}","params":{{"subscription":{server},"result":{result}}}}}"#
        ),
    )
}

#[test]
fn memory_transport_preserves_early_order_and_exact_integers() {
    let mut owner = connection();
    let root = owner.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
    assert_eq!(
        send(&mut owner),
        serde_json::json!({"jsonrpc":"2.0","id":1,"method":"rootSubscribe","params":[]})
    );
    notification(
        &mut owner,
        "rootNotification",
        u64::MAX,
        "18446744073709551615",
    )
    .unwrap();
    notification(&mut owner, "rootNotification", u64::MAX, "9007199254740993").unwrap();
    assert!(matches!(owner.next(&root).unwrap(), Drain::Pending));
    ack(&mut owner, 1, "18446744073709551615").unwrap();
    assert!(matches!(
        owner.next(&root).unwrap(),
        Drain::Notification(u64::MAX)
    ));

    assert!(matches!(
        owner.next(&root).unwrap(),
        Drain::Notification(9007199254740993)
    ));
    owner.unsubscribe(&root, CLEANUP_DEADLINE).unwrap();
    assert_eq!(
        send(&mut owner),
        serde_json::json!({"jsonrpc":"2.0","id":2,"method":"rootUnsubscribe","params":[u64::MAX]})
    );
    ack(&mut owner, 2, "true").unwrap();
    assert!(matches!(owner.next(&root).unwrap(), Drain::Closed));
    owner.forget(root).unwrap();
}

#[test]
fn cancellation_before_ack_cleans_up_after_late_ack_without_delivery() {
    let mut owner = connection();
    let root = owner.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
    send(&mut owner);
    owner.cancel(&root).unwrap();
    assert!(matches!(owner.next(&root).unwrap(), Drain::Closed));
    notification(&mut owner, "rootNotification", SERVER, "8").unwrap();
    ack(&mut owner, 1, "7").unwrap();
    owner.tick(TICK, EARLY_DEADLINE).unwrap();

    assert_eq!(send(&mut owner)["method"], "rootUnsubscribe");
    notification(&mut owner, "rootNotification", SERVER, "9").unwrap();
    ack(&mut owner, 2, "true").unwrap();
    owner.forget(root).unwrap();
}

#[test]
fn quiet_cancel_does_not_need_a_free_command_queue_slot() {
    let budget = Budget {
        messages: 1,
        ..Budget::default()
    };
    let mut owner = Connection::new(1, budget);
    let root = owner.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
    owner.cancel(&root).unwrap();
    assert!(matches!(owner.next(&root).unwrap(), Drain::Closed));
    send(&mut owner);
    ack(&mut owner, 1, "0").unwrap();
    owner.tick(2, SHORT_CLEANUP).unwrap();

    assert_eq!(send(&mut owner)["params"], serde_json::json!([0]));
}

#[test]
fn rejected_subscribe_and_each_unsubscribe_failure_remain_observable() {
    let mut owner = connection();
    let root = owner.subscribe(Root, 10).unwrap();
    send(&mut owner);
    ingest(
        &mut owner,
        r#"{"jsonrpc":"2.0","id":1,"error":{"code":-1,"message":"denied"}}"#,
    )
    .unwrap();
    assert!(matches!(
        owner.next(&root),
        Err(Failure::Protocol(Error::Remote(_)))
    ));
    owner.forget(root).unwrap();

    for reply in [Some("false"), Some("true"), None] {
        let mut owner = connection();
        let root = owner.subscribe(Root, 10).unwrap();
        send(&mut owner);
        ack(&mut owner, 1, "5").unwrap();
        owner.unsubscribe(&root, SUBSCRIBE_DEADLINE).unwrap();
        send(&mut owner);

        match reply {
            Some("false") => {
                ack(&mut owner, 2, "false").unwrap();
                assert!(matches!(owner.next(&root), Err(Failure::UnsubscribeFalse)));
            }
            Some(_) => {
                ingest(
                    &mut owner,
                    r#"{"jsonrpc":"2.0","id":2,"error":{"code":-2,"message":"no"}}"#,
                )
                .unwrap();
                assert!(matches!(
                    owner.next(&root),
                    Err(Failure::Protocol(Error::Remote(_)))
                ));
            }
            None => {
                owner.tick(SUBSCRIBE_DEADLINE, CLEANUP_DEADLINE).unwrap();
                assert!(matches!(owner.next(&root), Err(Failure::Deadline)));
            }
        }

        owner.forget(root).unwrap();
    }
}

#[test]
fn malformed_ack_colliding_ids_and_wrong_notification_methods_end_connection() {
    for result in ["null", "-1", "1.0", "\"1\"", "18446744073709551616"] {
        let mut owner = connection();
        let root = owner.subscribe(Root, 10).unwrap();
        send(&mut owner);
        assert!(ack(&mut owner, 1, result).is_err());
        assert!(matches!(owner.next(&root), Err(Failure::Closed)));
        assert!(owner.outbound().is_none());
    }

    let mut owner = connection();

    let root = owner.subscribe(Root, 10).unwrap();
    send(&mut owner);
    ack(&mut owner, 1, "2").unwrap();
    let _slot = owner.subscribe(Slot, 10).unwrap();
    send(&mut owner);
    assert!(matches!(ack(&mut owner, 2, "2"), Err(Failure::Correlation)));
    assert!(matches!(owner.next(&root), Err(Failure::Closed)));

    let mut owner = connection();
    let _root = owner.subscribe(Root, 10).unwrap();
    send(&mut owner);
    ack(&mut owner, 1, "2").unwrap();
    assert!(matches!(
        notification(&mut owner, "slotNotification", 2, "4"),
        Err(Failure::Method)
    ));
}

#[test]
fn subscribe_deadline_replacement_and_old_generation_are_explicit() {
    let mut owner = connection();
    let root = owner.subscribe(Root, 10).unwrap();
    send(&mut owner);
    owner.tick(10, SUBSCRIBE_DEADLINE).unwrap();
    assert!(matches!(owner.next(&root), Err(Failure::Deadline)));
    assert_eq!(
        owner.replace(2).unwrap(),
        Gap {
            previous: 1,
            replacement: 2
        }
    );
    assert!(matches!(owner.next(&root), Err(Failure::Closed)));
    let replacement = owner.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();

    assert!(matches!(owner.ingest(1, b"{}"), Err(Failure::Closed)));
    assert_eq!(send(&mut owner)["id"], 1);
    owner
        .ingest(2, br#"{"jsonrpc":"2.0","id":1,"result":4}"#)
        .unwrap();
    assert!(matches!(owner.next(&replacement).unwrap(), Drain::Pending));
}

#[test]
fn malicious_frame_and_slow_readers_hit_real_budgets() {
    let mut owner = Connection::new(
        1,
        Budget {
            messages: 1,
            ..Budget::default()
        },
    );
    let root = owner.subscribe(Root, 10).unwrap();
    send(&mut owner);
    ack(&mut owner, 1, "1").unwrap();
    notification(&mut owner, "rootNotification", 1, "2").unwrap();
    assert!(matches!(
        notification(&mut owner, "rootNotification", 1, "3"),
        Err(Failure::Overflow)
    ));
    assert!(matches!(owner.next(&root), Err(Failure::Closed)));

    for body in [
        r#"{"jsonrpc":"2.0","id":1,"result":1,"result":2}"#,
        r#"{"jsonrpc":"2.0","id":1,"result":{"x":[[[[[]]]]]}}"#,
    ] {
        let mut owner = Connection::new(
            1,
            Budget {
                frame: Limits {
                    bytes: 128,
                    depth: 4,
                },
                ..Budget::default()
            },
        );
        let _root = owner.subscribe(Root, 10).unwrap();
        send(&mut owner);
        assert!(ingest(&mut owner, body).is_err());
    }
    let mut owner = Connection::new(
        1,
        Budget {
            queued_bytes: 2,
            ..Budget::default()
        },
    );
    assert!(matches!(owner.subscribe(Root, 10), Err(Failure::Overflow)));
}

#[test]
fn each_family_has_exact_identity_and_config_omission() {
    assert_eq!(
        serde_json::to_string(&Logs(LogsFilter::All, None)).unwrap(),
        r#"["all"]"#
    );
    assert_eq!(Root::NAME, "rootSubscribe");
    assert_eq!(Slot::NOTIFICATION, "slotNotification");
    const {
        assert!(!Account::UNSTABLE);
    }
    const {
        assert!(!Program::UNSTABLE);
    }
    const {
        assert!(!Logs::UNSTABLE);
    }
    const {
        assert!(!SignatureNotifications::UNSTABLE);
    }
    const {
        assert!(Block::UNSTABLE);
    }

    const {
        assert!(Vote::UNSTABLE);
    }
    const {
        assert!(SlotsUpdates::UNSTABLE);
    }
    assert_eq!(Vote::UNSUBSCRIBE, "voteUnsubscribe");
    let config = BlockConfig(
        solana_kit_client::methods::transaction_results::BlockConfig {
            rewards: Some(false),
            ..Default::default()
        },
    );
    assert_eq!(
        serde_json::to_string(&Block(BlockFilter::All, Some(config))).unwrap(),
        r#"["all",{"showRewards":false}]"#
    );
}

#[test]
fn quiet_subscription_acknowledgements_are_observable_without_notifications() {
    let mut owner = connection();
    let root = owner.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
    assert_eq!(owner.status(&root).unwrap(), Status::Subscribing);
    send(&mut owner);
    ack(&mut owner, 1, "0").unwrap();
    assert_eq!(owner.status(&root).unwrap(), Status::Active);
    owner.unsubscribe(&root, CLEANUP_DEADLINE).unwrap();
    assert_eq!(owner.status(&root).unwrap(), Status::Unsubscribing);

    send(&mut owner);
    ack(&mut owner, 2, "true").unwrap();
    assert_eq!(owner.status(&root).unwrap(), Status::Closed);
}

#[test]
fn repeated_early_notifications_release_count_and_byte_budgets() {
    const CYCLES: usize = 12;
    const QUEUED_BYTES: usize = 100;
    let mut owner = Connection::new(
        1,
        Budget {
            subscriptions: 1,
            messages: 1,
            queued_bytes: QUEUED_BYTES,
            ..Budget::default()
        },
    );

    for _ in 0..CYCLES {
        let root = owner.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
        let request = send(&mut owner)["id"].as_u64().unwrap().try_into().unwrap();
        notification(&mut owner, "rootNotification", SERVER, "9").unwrap();
        ack(&mut owner, request, "7").unwrap();
        assert!(matches!(owner.next(&root).unwrap(), Drain::Notification(9)));
        owner.unsubscribe(&root, CLEANUP_DEADLINE).unwrap();
        let request = send(&mut owner)["id"].as_u64().unwrap().try_into().unwrap();

        ack(&mut owner, request, "true").unwrap();
        owner.forget(root).unwrap();
    }
}

#[test]
fn frame_byte_limit_and_unsubscribe_send_failure_are_terminal() {
    const FRAME_BYTES: usize = 64;
    let budget = Budget {
        frame: Limits {
            bytes: FRAME_BYTES,
            depth: 8,
        },
        ..Budget::default()
    };
    let mut owner = Connection::new(1, budget);
    let root = owner.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
    send(&mut owner);
    ack(&mut owner, 1, "18446744073709551615").unwrap();
    assert!(matches!(
        owner.unsubscribe(&root, CLEANUP_DEADLINE),
        Err(Failure::Protocol(Error::BodyLimit))
    ));
    assert!(matches!(owner.next(&root), Err(Failure::Closed)));

    let mut owner = connection();
    let _root = owner.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
    let oversized = vec![b' '; Budget::default().frame.bytes + 1];
    assert!(matches!(
        owner.ingest(1, &oversized),
        Err(Failure::Protocol(Error::BodyLimit))
    ));
    assert!(owner.outbound().is_none());
}

#[test]
fn replacement_releases_unsent_frames_and_unmatched_early_notifications() {
    let mut owner = connection();
    let old = owner.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
    notification(&mut owner, "rootNotification", SERVER, "9").unwrap();
    owner.replace(2).unwrap();
    assert!(owner.outbound().is_none());
    assert!(matches!(owner.next(&old), Err(Failure::Closed)));
    let new = owner.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
    send(&mut owner);

    owner
        .ingest(2, br#"{"jsonrpc":"2.0","id":1,"result":7}"#)
        .unwrap();
    assert!(matches!(owner.next(&new).unwrap(), Drain::Pending));
}

#[test]
fn owner_identity_prevents_cross_connection_handles_and_generation_reuse() {
    let mut first = connection();
    let original = first.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
    let mut second = connection();
    let other = second.subscribe(Root, SUBSCRIBE_DEADLINE).unwrap();
    assert!(matches!(second.cancel(&original), Err(Failure::Closed)));
    assert_eq!(second.status(&other).unwrap(), Status::Subscribing);
    first.replace(2).unwrap();
    assert!(matches!(first.replace(1), Err(Failure::Correlation)));

    assert!(matches!(first.replace(2), Err(Failure::Correlation)));
    assert!(matches!(first.next(&original), Err(Failure::Closed)));
}
