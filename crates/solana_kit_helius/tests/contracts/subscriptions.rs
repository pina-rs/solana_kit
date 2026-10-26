//! Actual Helius typed methods execute through the real bounded Connection owner.

use solana_kit_client::subscriptions::{
    Budget, Connection, Drain, Failure, Provider, Slot, Status, Subscription,
};
use solana_kit_helius::{preconf::Subscribe, subscriptions::JsonSubscription};

fn outbound(connection: &mut Connection) -> serde_json::Value {
    serde_json::from_slice(&connection.outbound().expect("queued protocol request"))
        .expect("JSON frame")
}

fn ack(connection: &mut Connection, generation: u64, id: u32, result: &str) {
    connection
        .ingest(
            generation,
            format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#).as_bytes(),
        )
        .expect("matched acknowledgement");
}

fn notification<T: std::fmt::Debug>(drain: Drain<T>) -> T {
    match drain {
        Drain::Notification(value) => value,
        other => panic!("expected notification, received {other:?}"),
    }
}

fn frame() -> Vec<u8> {
    let mut bytes = vec![0; 152];
    bytes[0] = 1;
    bytes[1] = 9;
    bytes[9] = 7;
    bytes[17] = 1;
    bytes[18] = 1;
    bytes[83] = 1;
    bytes[86] = 1;

    bytes
}

/// A typed Helius JSON method retains early routing and true unsubscribe acknowledgement.
pub fn json_provider_lifecycle() {
    let mut connection = Connection::new(4, Budget::default());
    let subscription = connection
        .subscribe(Provider(JsonSubscription(Slot)), 100)
        .expect("registration");
    let request = outbound(&mut connection);
    assert_eq!(request["method"], "slotSubscribe");
    assert_eq!(request["params"], serde_json::json!([]));
    connection.ingest(4, br#"{"jsonrpc":"2.0","method":"slotNotification","params":{"subscription":8,"result":{"slot":3,"parent":2,"root":1}}}"#).expect("early notification");
    assert!(matches!(connection.next(&subscription), Ok(Drain::Pending)));
    ack(&mut connection, 4, 1, "8");

    let value = notification(connection.next(&subscription).expect("typed notification"));
    assert_eq!((value.slot, value.parent, value.root), (3, 2, 1));

    connection
        .unsubscribe(&subscription, 100)
        .expect("unsubscribe admitted");
    let request = outbound(&mut connection);
    assert_eq!(request["method"], "slotUnsubscribe");
    assert_eq!(request["params"], serde_json::json!([8]));
    assert_eq!(
        connection.status(&subscription).expect("status"),
        Status::Unsubscribing
    );
    ack(&mut connection, 4, 2, "true");
    assert!(matches!(connection.next(&subscription), Ok(Drain::Closed)));
    connection
        .forget(subscription)
        .expect("terminal slot released");
}

/// Binary pre-ack data fans out only within its explicit namespace and waits for each ack.
pub fn binary_early_broadcast_and_cancellation() {
    let mut connection = Connection::new(2, Budget::default());
    let first = connection
        .subscribe(
            Provider(Subscribe {
                max_frame_bytes: 152,
            }),
            100,
        )
        .expect("first");
    let second = connection
        .subscribe(
            Provider(Subscribe {
                max_frame_bytes: 152,
            }),
            100,
        )
        .expect("second");
    assert_eq!(outbound(&mut connection)["method"], "preconfSubscribe");
    outbound(&mut connection);
    connection
        .ingest_binary(2, &frame())
        .expect("bounded early broadcast");
    assert!(matches!(connection.next(&first), Ok(Drain::Pending)));
    assert!(matches!(connection.next(&second), Ok(Drain::Pending)));

    ack(&mut connection, 2, 1, "10");

    let value = notification(connection.next(&first).expect("first typed frame"));
    assert_eq!((value.slot, value.transaction_index), (9, 7));

    assert!(matches!(connection.next(&second), Ok(Drain::Pending)));
    ack(&mut connection, 2, 2, "11");
    assert!(matches!(
        connection.next(&second),
        Ok(Drain::Notification(_))
    ));
    binary_cancellation(&mut connection, first, second);
}

fn binary_cancellation(
    connection: &mut Connection,
    first: Subscription<Provider<Subscribe>>,
    second: Subscription<Provider<Subscribe>>,
) {
    connection.cancel(&first).expect("local cancellation");
    connection
        .ingest_binary(2, &frame())
        .expect("only remaining listener receives");
    assert!(matches!(connection.next(&first), Ok(Drain::Closed)));
    assert!(matches!(
        connection.next(&second),
        Ok(Drain::Notification(_))
    ));
    connection.tick(0, 100).expect("quiet unsubscribe starts");

    assert_eq!(outbound(connection)["params"], serde_json::json!([10]));
    ack(connection, 2, 3, "true");
    connection.forget(first).expect("cancelled first released");
    connection
        .unsubscribe(&second, 100)
        .expect("second unsubscribe");
    outbound(connection);
    ack(connection, 2, 4, "false");
    assert!(matches!(
        connection.next(&second),
        Err(Failure::UnsubscribeFalse)
    ));
    connection.forget(second).expect("false ack is terminal");
}

/// Typed binary decode failures terminate, while stale binary cannot terminate a replacement.
pub fn binary_failure_and_replacement() {
    let mut connection = Connection::new(1, Budget::default());
    let old = connection
        .subscribe(
            Provider(Subscribe {
                max_frame_bytes: 152,
            }),
            10,
        )
        .expect("old");
    outbound(&mut connection);
    connection.replace(2).expect("new generation");
    let current = connection
        .subscribe(
            Provider(Subscribe {
                max_frame_bytes: 152,
            }),
            10,
        )
        .expect("current");
    outbound(&mut connection);
    assert!(matches!(
        connection.ingest_binary(1, &frame()),
        Err(Failure::Closed)
    ));
    assert_eq!(
        connection.status(&current).expect("replacement stays open"),
        Status::Subscribing
    );

    assert!(matches!(connection.next(&old), Err(Failure::Closed)));
    ack(&mut connection, 2, 1, "12");
    let mut invalid = frame();
    invalid[0] = 99;
    connection
        .ingest_binary(2, &invalid)
        .expect("bounded frame enqueued");
    assert!(matches!(
        connection.next(&current),
        Err(Failure::Protocol(_))
    ));
    assert!(matches!(connection.status(&current), Err(Failure::Closed)));
}
