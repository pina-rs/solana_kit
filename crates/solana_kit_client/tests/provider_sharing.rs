//! Public physical-registration and independent-consumer lifecycle contracts.
use serde::{Serialize, Serializer, ser::SerializeMap};
use solana_kit_client::{Error, methods::Method, subscriptions::*};

const SECOND_DEADLINE: u64 = 20;
const CLEANUP_DEADLINE: u64 = 30;
const REPLACEMENT_DEADLINE: u64 = 40;
const LATER_ACK_DEADLINE: u64 = 9;
const FIRST_VALUE: u64 = 99;
const EARLY_VALUE: u64 = 8;

const SMALL_QUEUE_BYTES: usize = 8;
const LARGE_QUEUE_BYTES: usize = 16;
const BYTE_LIMIT: usize = 128;
const AGGREGATE_BYTES: usize = 256;
const OVERFLOW_ITEMS: usize = 32;
const REMOTE_CODE: i64 = -9;

struct Json {
    value: u64,
    reverse: bool,
}

impl Serialize for Json {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;

        if self.reverse {
            map.serialize_entry("value", &self.value)?;
            map.serialize_entry("fixed", &true)?;
        } else {
            map.serialize_entry("fixed", &true)?;
            map.serialize_entry("value", &self.value)?;
        }

        map.end()
    }
}

impl Method for Json {
    type Response = u64;
    const NAME: &'static str = "providerSubscribe";
    fn decode(&self, raw: &str) -> Result<u64, Error> {
        serde_json::from_str(raw).map_err(Error::Schema)
    }
}

impl ProviderMethod for Json {
    const SHARE: bool = true;
    const UNSUBSCRIBE: &'static str = "providerUnsubscribe";
    const NOTIFICATION: &'static str = "providerNotification";
}

#[derive(Serialize)]
#[serde(transparent)]
struct Other(Json);

impl Method for Other {
    type Response = u64;
    const NAME: &'static str = Json::NAME;
    fn decode(&self, raw: &str) -> Result<u64, Error> {
        self.0.decode(raw)
    }
}

impl ProviderMethod for Other {
    const SHARE: bool = true;
    const UNSUBSCRIBE: &'static str = Json::UNSUBSCRIBE;
    const NOTIFICATION: &'static str = Json::NOTIFICATION;
}

struct Duplicate;
impl Serialize for Duplicate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("value", &1)?;
        map.serialize_entry("value", &2)?;
        map.end()
    }
}
impl Method for Duplicate {
    type Response = u64;
    const NAME: &'static str = Json::NAME;
    fn decode(&self, _: &str) -> Result<u64, Error> {
        Err(Error::Envelope)
    }
}
impl ProviderMethod for Duplicate {
    const SHARE: bool = true;
    const UNSUBSCRIBE: &'static str = Json::UNSUBSCRIBE;
    const NOTIFICATION: &'static str = Json::NOTIFICATION;
}

fn request(value: u64) -> Provider<Json> {
    Provider(Json {
        value,
        reverse: false,
    })
}

fn reversed_request(value: u64) -> Provider<Json> {
    Provider(Json {
        value,
        reverse: true,
    })
}

fn sent(owner: &mut Connection) -> u32 {
    let body = owner.outbound().expect("one physical frame");
    let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
    u32::try_from(value["id"].as_u64().unwrap()).unwrap()
}

fn ack(owner: &mut Connection, id: u32, result: &str) {
    owner
        .ingest(
            1,
            format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#).as_bytes(),
        )
        .unwrap();
}

fn notify(owner: &mut Connection, value: u64) -> Result<(), Failure> {
    let frame = format!(
        concat!(
            r#"{{"jsonrpc":"2.0","method":"providerNotification","params":{{"#,
            r#""subscription":7,"result":{value}}}}}"#,
        ),
        value = value,
    );
    owner.ingest(1, frame.as_bytes())
}

fn delivered(owner: &mut Connection, handle: &Subscription<Provider<Json>>, value: u64) {
    assert!(matches!(owner.next(handle), Ok(Drain::Notification(actual)) if actual == value));
}

fn refuse_unsubscribe(owner: &mut Connection, id: u32) {
    let frame = format!(
        concat!(
            r#"{{"jsonrpc":"2.0","id":{id},"error":{{"code":-1,"#,
            r#""message":"refused"}}}}"#,
        ),
        id = id,
    );
    owner.ingest(1, frame.as_bytes()).unwrap();
}

#[test]
fn identical_canonical_params_share_before_or_after_the_physical_ack() {
    for acknowledge_first in [false, true] {
        let mut owner = Connection::new(1, Budget::default());
        let first = owner.subscribe(request(1), 10).unwrap();
        let id = sent(&mut owner);

        if acknowledge_first {
            ack(&mut owner, id, "7");
        }

        let second = owner
            .subscribe(reversed_request(1), SECOND_DEADLINE)
            .unwrap();
        assert!(owner.outbound().is_none(), "one physical subscribe");
        notify(&mut owner, FIRST_VALUE).unwrap();

        if !acknowledge_first {
            assert!(matches!(owner.next(&first), Ok(Drain::Pending)));
            assert!(matches!(owner.next(&second), Ok(Drain::Pending)));
            ack(&mut owner, id, "7");
        }

        delivered(&mut owner, &first, FIRST_VALUE);
        delivered(&mut owner, &second, FIRST_VALUE);
        owner.unsubscribe(&first, CLEANUP_DEADLINE).unwrap();
        assert!(owner.settled(&first).unwrap());
        owner.forget(first).unwrap();
        assert!(
            owner.outbound().is_none(),
            "one local cancellation keeps shared work"
        );
        notify(&mut owner, 100).unwrap();
        delivered(&mut owner, &second, 100);

        owner.unsubscribe(&second, CLEANUP_DEADLINE).unwrap();
        let unsubscribe = sent(&mut owner);
        assert!(!owner.settled(&second).unwrap());
        assert!(matches!(
            owner.subscribe(request(1), REPLACEMENT_DEADLINE),
            Err(Failure::Closing)
        ));
        ack(&mut owner, unsubscribe, "true");
        assert!(owner.settled(&second).unwrap());
        owner.forget(second).unwrap();
        owner.subscribe(request(1), REPLACEMENT_DEADLINE).unwrap();

        sent(&mut owner);
    }
}

#[test]
fn distinct_params_and_concrete_decoders_never_alias_a_server_id() {
    for different_schema in [false, true] {
        let mut owner = Connection::new(1, Budget::default());
        let first = owner.subscribe(request(1), 10).unwrap();
        let id = sent(&mut owner);

        if different_schema {
            let _other = owner
                .subscribe(
                    Provider(Other(Json {
                        value: 1,
                        reverse: false,
                    })),
                    10,
                )
                .unwrap();
        } else {
            let _other = owner.subscribe(request(2), 10).unwrap();
        }

        let other = sent(&mut owner);
        ack(&mut owner, id, "7");
        assert!(matches!(
            owner.ingest(
                1,
                format!(r#"{{"jsonrpc":"2.0","id":{other},"result":7}}"#).as_bytes()
            ),
            Err(Failure::Correlation)
        ));
        assert!(matches!(owner.next(&first), Err(Failure::Closed)));
    }
}

#[test]
fn cancellation_before_ack_preserves_the_other_local_and_only_last_unsubscribes() {
    let mut owner = Connection::new(1, Budget::default());
    let first = owner.subscribe(request(1), 10).unwrap();
    let second = owner.subscribe(request(1), SECOND_DEADLINE).unwrap();
    let id = sent(&mut owner);
    notify(&mut owner, EARLY_VALUE).unwrap();
    owner.cancel(&first).unwrap();
    owner.forget(first).unwrap();
    ack(&mut owner, id, "7");

    owner.tick(1, CLEANUP_DEADLINE).unwrap();
    assert!(owner.outbound().is_none());
    delivered(&mut owner, &second, EARLY_VALUE);
    owner.unsubscribe(&second, CLEANUP_DEADLINE).unwrap();
    let id = sent(&mut owner);
    ack(&mut owner, id, "true");
    owner.forget(second).unwrap();
}

#[test]
fn false_error_and_missing_unsubscribe_acks_quarantine_the_exact_key() {
    for reply in [Some("false"), Some("error"), None] {
        let mut owner = Connection::new(1, Budget::default());
        let handle = owner.subscribe(request(1), 10).unwrap();
        let id = sent(&mut owner);
        ack(&mut owner, id, "7");
        owner.unsubscribe(&handle, SECOND_DEADLINE).unwrap();
        let id = sent(&mut owner);

        match reply {
            Some("error") => {
                refuse_unsubscribe(&mut owner, id);
                assert!(matches!(
                    owner.next(&handle),
                    Err(Failure::Protocol(Error::Remote(_)))
                ));
            }
            Some(value) => {
                ack(&mut owner, id, value);
                assert!(matches!(
                    owner.next(&handle),
                    Err(Failure::UnsubscribeFalse)
                ));
            }
            None => {
                owner.tick(SECOND_DEADLINE, CLEANUP_DEADLINE).unwrap();
                assert!(matches!(owner.next(&handle), Err(Failure::Deadline)));
            }
        }

        owner.forget(handle).unwrap();
        assert!(matches!(
            owner.subscribe(request(1), REPLACEMENT_DEADLINE),
            Err(Failure::Closing)
        ));
        let different = owner.subscribe(request(2), REPLACEMENT_DEADLINE).unwrap();
        let id = sent(&mut owner);
        let duplicate = format!(r#"{{"jsonrpc":"2.0","id":{id},"result":7}}"#);
        assert!(matches!(
            owner.ingest(1, duplicate.as_bytes()),
            Err(Failure::Correlation)
        ));
        assert!(matches!(owner.next(&different), Err(Failure::Closed)));
        owner.replace(2).unwrap();

        owner.subscribe(request(1), REPLACEMENT_DEADLINE).unwrap();
    }
}

#[test]
fn local_ack_deadlines_and_retained_queues_are_independent() {
    let mut owner = Connection::new(1, Budget::default());
    let first = owner.subscribe(request(1), 2).unwrap();
    let second = owner.subscribe(request(1), LATER_ACK_DEADLINE).unwrap();
    let id = sent(&mut owner);
    assert_eq!(owner.next_deadline(), Some(2));
    owner.tick(2, SECOND_DEADLINE).unwrap();
    assert!(matches!(owner.next(&first), Err(Failure::Deadline)));
    assert_eq!(owner.next_deadline(), Some(LATER_ACK_DEADLINE));

    ack(&mut owner, id, "7");
    notify(&mut owner, 3).unwrap();
    delivered(&mut owner, &second, 3);
    owner.forget(first).unwrap();

    let mut owner = Connection::new(1, Budget::default());
    let first = owner
        .subscribe_with_budget(
            request(1),
            10,
            QueueBudget {
                messages: 1,
                bytes: SMALL_QUEUE_BYTES,
            },
        )
        .unwrap();
    let second = owner
        .subscribe_with_budget(
            request(1),
            10,
            QueueBudget {
                messages: 2,
                bytes: LARGE_QUEUE_BYTES,
            },
        )
        .unwrap();
    let id = sent(&mut owner);
    ack(&mut owner, id, "7");
    notify(&mut owner, 1).unwrap();
    delivered(&mut owner, &first, 1);
    notify(&mut owner, 2).unwrap();

    delivered(&mut owner, &first, 2);
    delivered(&mut owner, &second, 1);
    delivered(&mut owner, &second, 2);
    notify(&mut owner, 3).unwrap();
    assert!(matches!(notify(&mut owner, 4), Err(Failure::Overflow)));
    assert!(matches!(owner.next(&second), Err(Failure::Closed)));
}

#[test]
fn abandoned_handles_release_only_after_actual_last_ack_and_tick() {
    let mut owner = Connection::new(
        1,
        Budget {
            subscriptions: 2,
            ..Budget::default()
        },
    );
    let first = owner.subscribe(request(1), 10).unwrap();
    let second = owner.subscribe(request(1), 10).unwrap();
    let id = sent(&mut owner);
    drop(first);
    owner.tick(1, SECOND_DEADLINE).unwrap();
    ack(&mut owner, id, "7");
    owner.tick(2, SECOND_DEADLINE).unwrap();

    assert!(owner.outbound().is_none());
    drop(second);
    owner.tick(3, SECOND_DEADLINE).unwrap();
    let id = sent(&mut owner);
    assert!(matches!(
        owner.subscribe(request(1), CLEANUP_DEADLINE),
        Err(Failure::Closing)
    ));
    ack(&mut owner, id, "true");
    owner.tick(4, CLEANUP_DEADLINE).unwrap();
    owner.subscribe(request(1), REPLACEMENT_DEADLINE).unwrap();

    sent(&mut owner);
}

#[test]
fn shared_remote_failure_is_observable_on_each_local_consumer() {
    let mut owner = Connection::new(1, Budget::default());
    let first = owner.subscribe(request(1), 10).unwrap();
    let second = owner.subscribe(request(1), 10).unwrap();
    let id = sent(&mut owner);
    let frame = format!(
        concat!(
            r#"{{"jsonrpc":"2.0","id":{id},"error":{{"code":-9,"#,
            r#""message":"denied","data":null}}}}"#,
        ),
        id = id,
    );
    owner.ingest(1, frame.as_bytes()).unwrap();

    for handle in [&first, &second] {
        let Err(Failure::Protocol(Error::Remote(error))) = owner.next(handle) else {
            panic!("independent retained error");
        };
        assert_eq!(error.code, REMOTE_CODE);
        assert!(matches!(
            error.data,
            solana_kit_client::Presence::Present(_)
        ));
    }

    owner.forget(first).unwrap();
    owner.forget(second).unwrap();
}

#[test]
fn aggregate_overflow_and_stale_or_foreign_identity_preserve_their_contracts() {
    for by_bytes in [false, true] {
        let mut owner = Connection::new(
            1,
            Budget {
                messages: 1,
                queued_bytes: AGGREGATE_BYTES,
                ..Budget::default()
            },
        );
        let first = owner.subscribe(request(1), 10).unwrap();
        let id = sent(&mut owner);
        let second = owner.subscribe(request(1), 10).unwrap();
        ack(&mut owner, id, "7");

        if by_bytes {
            let mut owner = Connection::new(
                1,
                Budget {
                    queued_bytes: BYTE_LIMIT,
                    ..Budget::default()
                },
            );
            let first = owner.subscribe(request(1), 10).unwrap();
            let id = sent(&mut owner);
            let second = owner.subscribe(request(1), 10).unwrap();
            ack(&mut owner, id, "7");
            let result = format!("[{}]", "0,".repeat(OVERFLOW_ITEMS) + "0");
            let frame = format!(
                r#"{{"jsonrpc":"2.0","method":"providerNotification","params":{{"subscription":7,"result":{result}}}}}"#
            );

            assert!(matches!(
                owner.ingest(1, frame.as_bytes()),
                Err(Failure::Overflow)
            ));
            assert!(matches!(owner.next(&first), Err(Failure::Closed)));
            assert!(matches!(owner.next(&second), Err(Failure::Closed)));
        } else {
            assert!(matches!(notify(&mut owner, 1), Err(Failure::Overflow)));
            assert!(matches!(owner.next(&first), Err(Failure::Closed)));
            assert!(matches!(owner.next(&second), Err(Failure::Closed)));
        }
    }

    rejects_stale_and_foreign_handles();
}

fn rejects_stale_and_foreign_handles() {
    let mut owner = Connection::new(1, Budget::default());
    let old = owner.subscribe(request(1), 10).unwrap();
    let mut foreign = Connection::new(1, Budget::default());
    assert!(matches!(foreign.cancel(&old), Err(Failure::Closed)));
    foreign.subscribe(request(1), 10).unwrap();
    owner.replace(2).unwrap();
    let current = owner.subscribe(request(1), 10).unwrap();
    assert!(matches!(notify(&mut owner, 1), Err(Failure::Closed)));

    assert!(matches!(owner.next(&old), Err(Failure::Closed)));
    assert_eq!(owner.status(&current).unwrap(), Status::Subscribing);
}

#[test]
fn built_in_subscribe_deadline_keeps_its_existing_registration_policy() {
    let mut owner = Connection::new(1, Budget::default());
    let first = owner.subscribe(Root, 2).unwrap();
    sent(&mut owner);
    owner.tick(2, 10).unwrap();
    assert!(matches!(owner.next(&first), Err(Failure::Deadline)));
    owner.forget(first).unwrap();
    owner.subscribe(Root, SECOND_DEADLINE).unwrap();
    sent(&mut owner);
}

#[test]
fn duplicate_parameter_keys_cannot_be_canonicalized_into_shared_work() {
    let mut owner = Connection::new(1, Budget::default());
    assert!(matches!(
        owner.subscribe(Provider(Duplicate), 10),
        Err(Failure::Protocol(Error::Json(_)))
    ));
    assert!(owner.outbound().is_none());
    owner.subscribe(Root, 10).unwrap();
    sent(&mut owner);
}
