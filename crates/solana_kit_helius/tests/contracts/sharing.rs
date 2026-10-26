//! Helius sharing uses the public provider bridge and the maintained Connection.

use solana_kit_client::subscriptions::{Budget, Connection, Drain, Failure, Provider, Root};
use solana_kit_helius::subscriptions::JsonSubscription;

fn outbound(connection: &mut Connection) -> u32 {
    let frame = connection.outbound().expect("one physical command");
    let value: serde_json::Value = serde_json::from_slice(&frame).expect("JSON command");
    u32::try_from(value["id"].as_u64().expect("request ID")).expect("bounded ID")
}

fn ack(connection: &mut Connection, id: u32, result: &str) {
    connection
        .ingest(
            1,
            format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#).as_bytes(),
        )
        .expect("physical ack");
}

/// Both registration orders share one wire subscribe and preserve the final true ack.
pub fn physical_registration_and_last_cancellation() {
    const SECOND_DEADLINE: u64 = 20;
    const CLOSING_DEADLINE: u64 = 40;

    for acknowledged in [false, true] {
        let mut connection = Connection::new(1, Budget::default());
        let first = connection
            .subscribe(Provider(JsonSubscription(Root)), 10)
            .expect("first local");
        let id = outbound(&mut connection);

        if acknowledged {
            ack(&mut connection, id, "7");
        }

        let second = connection
            .subscribe(Provider(JsonSubscription(Root)), SECOND_DEADLINE)
            .expect("second local");
        assert!(connection.outbound().is_none());
        connection.ingest(1, br#"{"jsonrpc":"2.0","method":"rootNotification","params":{"subscription":7,"result":18446744073709551615}}"#).expect("early or active data");

        if !acknowledged {
            assert!(matches!(connection.next(&first), Ok(Drain::Pending)));
            ack(&mut connection, id, "7");
        }

        assert!(matches!(
            connection.next(&first),
            Ok(Drain::Notification(u64::MAX))
        ));
        assert!(matches!(
            connection.next(&second),
            Ok(Drain::Notification(u64::MAX))
        ));
        connection.unsubscribe(&first, 30).expect("detach local");
        connection
            .forget(first)
            .expect("independent local slot released");
        assert!(connection.outbound().is_none());
        connection.unsubscribe(&second, 30).expect("last local");
        let id = outbound(&mut connection);
        assert!(matches!(
            connection.subscribe(Provider(JsonSubscription(Root)), CLOSING_DEADLINE),
            Err(Failure::Closing)
        ));

        ack(&mut connection, id, "true");
        connection
            .forget(second)
            .expect("physical lifecycle settled");
    }
}

/// Missing subscribe acknowledgement quarantines a shared key until replacement.
/// A late current-generation acknowledgement terminates rather than naming new work.
pub fn subscribe_ack_uncertainty_requires_replacement() {
    for late in [false, true] {
        uncertain_subscription(late);
    }
}

fn uncertain_subscription(late: bool) {
    const LATE_DEADLINE: u64 = 5;
    const UNSUBSCRIBE_DEADLINE: u64 = 20;
    const QUARANTINED_DEADLINE: u64 = 30;

    let mut connection = Connection::new(
        1,
        Budget {
            subscriptions: 2,
            ..Budget::default()
        },
    );
    let first = connection
        .subscribe(Provider(JsonSubscription(Root)), 2)
        .expect("early local deadline");
    let second = connection
        .subscribe(Provider(JsonSubscription(Root)), LATE_DEADLINE)
        .expect("later local deadline");
    let id = outbound(&mut connection);
    connection.tick(2, 20).expect("independent first expiry");
    assert!(matches!(connection.next(&first), Err(Failure::Deadline)));
    assert!(matches!(connection.next(&second), Ok(Drain::Pending)));

    connection.forget(first).expect("first local released");
    connection
        .tick(LATE_DEADLINE, UNSUBSCRIBE_DEADLINE)
        .expect("physical subscribe uncertainty");
    assert!(matches!(connection.next(&second), Err(Failure::Deadline)));
    connection.forget(second).expect("last local released");
    assert!(matches!(
        connection.subscribe(Provider(JsonSubscription(Root)), QUARANTINED_DEADLINE),
        Err(Failure::Closing)
    ));

    if late {
        let ack = format!(r#"{{"jsonrpc":"2.0","id":{id},"result":7}}"#);
        assert!(matches!(
            connection.ingest(1, ack.as_bytes()),
            Err(Failure::Correlation)
        ));
    }

    replacement_ignores_stale(&mut connection);
}

fn replacement_ignores_stale(connection: &mut Connection) {
    const REPLACEMENT_DEADLINE: u64 = 40;

    connection
        .replace(2)
        .expect("explicit new connection generation");
    let current = connection
        .subscribe(Provider(JsonSubscription(Root)), REPLACEMENT_DEADLINE)
        .expect("new physical owner");
    assert!(matches!(
        connection.ingest(1, br#"{"jsonrpc":"2.0","id":1,"result":7}"#),
        Err(Failure::Closed)
    ));
    assert_eq!(
        connection
            .status(&current)
            .expect("stale ack cannot end replacement"),
        solana_kit_client::subscriptions::Status::Subscribing
    );
}
