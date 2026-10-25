use super::*;
use crate::{
    methods::Method,
    subscriptions::{Budget, Drain, Provider, ProviderMethod, Slot},
};
use serde::{Serialize, Serializer};

#[derive(Clone, Copy)]
struct Binary<const N: u8>;

impl<const N: u8> Serialize for Binary<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        [N].serialize(serializer)
    }
}

impl<const N: u8> Method for Binary<N> {
    type Response = u64;
    const NAME: &'static str = "providerSubscribe";

    fn decode(&self, _raw: &str) -> Result<u64, Error> {
        Err(Error::Envelope)
    }
}

impl<const N: u8> ProviderMethod for Binary<N> {
    const UNSUBSCRIBE: &'static str = "providerUnsubscribe";
    const NOTIFICATION: &'static str = "providerNotification";
    const NOTIFICATION_KIND: NotificationKind =
        NotificationKind::Binary(if N == 1 { "first" } else { "second" });

    fn decode_binary(&self, raw: &[u8]) -> Result<u64, Error> {
        let bytes: [u8; 8] = raw.try_into().map_err(|_| Error::Envelope)?;
        Ok(u64::from_le_bytes(bytes))
    }
}

#[derive(Serialize)]
struct ConflictingFamily([u8; 0]);

impl Method for ConflictingFamily {
    type Response = u64;
    const NAME: &'static str = "otherSubscribe";

    fn decode(&self, _raw: &str) -> Result<u64, Error> {
        Err(Error::Envelope)
    }
}

impl ProviderMethod for ConflictingFamily {
    const UNSUBSCRIBE: &'static str = "providerUnsubscribe";
    const NOTIFICATION: &'static str = "providerNotification";
    const NOTIFICATION_KIND: NotificationKind = NotificationKind::Binary("first");
}

fn sent(connection: &mut Connection) -> u32 {
    let body = connection.outbound().expect("one outgoing request");
    let body: serde_json::Value = serde_json::from_slice(&body).expect("RPC request");
    u32::try_from(body["id"].as_u64().expect("request ID")).expect("bounded ID")
}

fn ack(connection: &mut Connection, id: u32, result: &str) {
    connection
        .ingest(
            1,
            format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#).as_bytes(),
        )
        .expect("ack");
}

#[test]
fn complete_broadcast_is_reserved_before_any_copy() {
    let mut connection = Connection::new(
        1,
        Budget {
            queued_bytes: 128,
            ..Budget::default()
        },
    );
    let first = connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("first");
    sent(&mut connection);
    let second = connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("second");
    sent(&mut connection);
    assert_eq!((connection.bytes, connection.messages), (0, 0));
    connection.budget.queued_bytes = 15;
    assert!(matches!(
        connection.ingest_binary(1, &7_u64.to_le_bytes()),
        Err(Failure::Overflow)
    ));

    assert_eq!((connection.bytes, connection.messages), (0, 0));
    assert!(connection.entries.is_empty());
    assert!(matches!(connection.next(&first), Err(Failure::Closed)));
    assert!(matches!(connection.next(&second), Err(Failure::Closed)));
}

#[test]
fn complete_broadcast_obeys_message_capacity() {
    let mut connection = Connection::new(1, Budget::default());
    connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("first");
    sent(&mut connection);
    connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("second");
    sent(&mut connection);
    connection.budget.messages = 1;
    assert!(matches!(
        connection.ingest_binary(1, &[0; 8]),
        Err(Failure::Overflow)
    ));
    assert_eq!((connection.bytes, connection.messages), (0, 0));
}

#[test]
fn incompatible_binary_namespaces_are_rejected_without_poisoning_existing_owner() {
    let mut connection = Connection::new(1, Budget::default());
    let first = connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("first");
    let id = sent(&mut connection);
    assert!(matches!(
        connection.subscribe(Provider(Binary::<2>), 10),
        Err(Failure::Method)
    ));
    assert_eq!((connection.bytes, connection.messages), (0, 0));
    ack(&mut connection, id, "7");
    connection
        .ingest_binary(1, &9_u64.to_le_bytes())
        .expect("first namespace stays usable");
    assert!(matches!(
        connection.next(&first),
        Ok(Drain::Notification(9))
    ));
}

#[test]
fn cancellation_before_ack_releases_early_payload_and_waits_for_true_remote_cleanup() {
    let mut connection = Connection::new(1, Budget::default());
    let handle = connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("subscribe");
    let id = sent(&mut connection);
    connection.ingest_binary(1, &[0; 8]).expect("early payload");
    assert_eq!((connection.bytes, connection.messages), (8, 1));
    assert!(matches!(connection.next(&handle), Ok(Drain::Pending)));
    connection.cancel(&handle).expect("quiet cancellation");
    assert_eq!((connection.bytes, connection.messages), (0, 0));

    connection
        .ingest_binary(1, &[0; 8])
        .expect("late cancelled data is discarded without a copy");
    assert_eq!((connection.bytes, connection.messages), (0, 0));
    ack(&mut connection, id, "7");
    connection.tick(0, 20).expect("deferred cleanup");
    let unsubscribe = sent(&mut connection);
    ack(&mut connection, unsubscribe, "true");
    connection
        .forget(handle)
        .expect("terminal registration released");
    assert!(connection.entries.is_empty());
    assert_eq!((connection.bytes, connection.messages), (0, 0));
}

#[test]
fn remote_error_and_deadline_keep_retention_accounted_until_terminal_handle_is_forgotten() {
    for remote_error in [true, false] {
        let mut connection = Connection::new(1, Budget::default());
        let handle = connection
            .subscribe(Provider(Binary::<1>), 10)
            .expect("subscribe");
        let id = sent(&mut connection);
        connection.ingest_binary(1, &[0; 8]).expect("early payload");

        if remote_error {
            connection
                .ingest(
                    1,
                    format!(
                        r#"{{"jsonrpc":"2.0","id":{id},"error":{{"code":-1,"message":"denied"}}}}"#
                    )
                    .as_bytes(),
                )
                .expect("remote error retained");
            assert!(matches!(
                connection.next(&handle),
                Err(Failure::Protocol(Error::Remote(_)))
            ));
        } else {
            connection.tick(10, 20).expect("deadline applied");
            assert!(matches!(connection.next(&handle), Err(Failure::Deadline)));
        }

        assert_eq!((connection.bytes, connection.messages), (8, 1));
        connection.forget(handle).expect("terminal bytes released");
        assert_eq!((connection.bytes, connection.messages), (0, 0));
    }
}

#[test]
fn standard_json_families_do_not_receive_binary() {
    let mut connection = Connection::new(1, Budget::default());
    let handle = connection
        .subscribe(Slot, 10)
        .expect("standard JSON family");
    sent(&mut connection);
    assert!(matches!(
        connection.ingest_binary(1, &[0; 8]),
        Err(Failure::Method)
    ));
    assert!(matches!(connection.next(&handle), Err(Failure::Closed)));
    assert_eq!((connection.bytes, connection.messages), (0, 0));
}

#[test]
fn mixed_json_and_binary_families_keep_separate_queues() {
    let mut connection = Connection::new(1, Budget::default());
    let json = connection.subscribe(Slot, 10).expect("JSON");
    let json_id = sent(&mut connection);
    let binary = connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("binary");
    let binary_id = sent(&mut connection);
    ack(&mut connection, json_id, "7");
    ack(&mut connection, binary_id, "8");
    connection
        .ingest_binary(1, &5_u64.to_le_bytes())
        .expect("binary payload");

    assert!(matches!(connection.next(&json), Ok(Drain::Pending)));
    assert!(matches!(
        connection.next(&binary),
        Ok(Drain::Notification(5))
    ));
    assert_eq!((connection.bytes, connection.messages), (0, 0));
}

#[test]
fn positive_broadcast_and_each_delivery_release_exact_retained_bytes() {
    let mut connection = Connection::new(1, Budget::default());
    let first = connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("first");
    let first_id = sent(&mut connection);
    let second = connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("second");
    let second_id = sent(&mut connection);
    connection
        .ingest_binary(1, &u64::MAX.to_le_bytes())
        .expect("early fan-out");
    assert_eq!((connection.bytes, connection.messages), (16, 2));
    ack(&mut connection, first_id, "7");

    assert!(matches!(
        connection.next(&first),
        Ok(Drain::Notification(u64::MAX))
    ));
    assert_eq!((connection.bytes, connection.messages), (8, 1));
    ack(&mut connection, second_id, "8");
    assert!(matches!(
        connection.next(&second),
        Ok(Drain::Notification(u64::MAX))
    ));
    assert_eq!((connection.bytes, connection.messages), (0, 0));
    connection.terminate();
    assert!(connection.entries.is_empty());
}

#[test]
fn unsubscribe_remote_error_releases_cancelled_queue_and_terminal_registration() {
    let mut connection = Connection::new(1, Budget::default());
    let handle = connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("subscribe");
    let id = sent(&mut connection);
    ack(&mut connection, id, "7");
    connection
        .ingest_binary(1, &[0; 8])
        .expect("active payload");
    connection.unsubscribe(&handle, 20).expect("unsubscribe");
    let id = sent(&mut connection);
    assert_eq!((connection.bytes, connection.messages), (0, 0));

    connection
        .ingest(
            1,
            format!(r#"{{"jsonrpc":"2.0","id":{id},"error":{{"code":-9,"message":"denied"}}}}"#)
                .as_bytes(),
        )
        .expect("remote failure retained");
    assert!(matches!(
        connection.next(&handle),
        Err(Failure::Protocol(Error::Remote(_)))
    ));
    connection.forget(handle).expect("terminal slot released");
    assert_eq!((connection.bytes, connection.messages), (0, 0));
    assert!(connection.entries.is_empty());
}

#[test]
fn binary_frame_cap_and_decode_failure_are_terminal() {
    let mut connection = Connection::new(1, Budget::default());
    let handle = connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("subscribe");
    let id = sent(&mut connection);
    ack(&mut connection, id, "7");
    connection.budget.frame.bytes = 7;
    assert!(matches!(
        connection.ingest_binary(1, &[0; 8]),
        Err(Failure::Protocol(Error::BodyLimit))
    ));
    assert_eq!((connection.bytes, connection.messages), (0, 0));
    assert!(matches!(connection.next(&handle), Err(Failure::Closed)));

    let mut connection = Connection::new(1, Budget::default());
    let handle = connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("subscribe");
    let id = sent(&mut connection);
    ack(&mut connection, id, "7");
    connection
        .ingest_binary(1, &[0; 7])
        .expect("bounded malformed payload retained");
    assert!(matches!(
        connection.next(&handle),
        Err(Failure::Protocol(Error::Envelope))
    ));
    assert_eq!((connection.bytes, connection.messages), (0, 0));
}

#[test]
fn standard_duplicate_server_id_protection_and_foreign_handle_safety_remain() {
    let mut first = Connection::new(1, Budget::default());
    let one = first.subscribe(Provider(Binary::<1>), 10).expect("first");
    let first_id = sent(&mut first);
    first.subscribe(Provider(Binary::<1>), 10).expect("second");
    let second_id = sent(&mut first);
    ack(&mut first, first_id, "7");
    let mut foreign = Connection::new(1, Budget::default());
    assert!(matches!(foreign.cancel(&one), Err(Failure::Closed)));

    assert!(!foreign.closed);
    let duplicate = format!(r#"{{"jsonrpc":"2.0","id":{second_id},"result":7}}"#);
    assert!(matches!(
        first.ingest(1, duplicate.as_bytes()),
        Err(Failure::Correlation)
    ));
    assert!(first.closed);
    assert_eq!((first.bytes, first.messages), (0, 0));
}

#[test]
fn a_shared_namespace_label_does_not_admit_conflicting_wire_method_families() {
    let mut connection = Connection::new(1, Budget::default());
    let existing = connection
        .subscribe(Provider(Binary::<1>), 10)
        .expect("first family");
    let id = sent(&mut connection);
    assert!(matches!(
        connection.subscribe(Provider(ConflictingFamily([])), 10),
        Err(Failure::Method)
    ));
    ack(&mut connection, id, "7");
    connection
        .ingest_binary(1, &5_u64.to_le_bytes())
        .expect("existing family is usable");
    assert!(matches!(
        connection.next(&existing),
        Ok(Drain::Notification(5))
    ));
}
