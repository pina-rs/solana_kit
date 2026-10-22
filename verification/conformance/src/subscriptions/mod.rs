//! Public memory-frame subscription fixtures shared by every runtime.

use alloc::{format, string::String};
use solana_kit_client::subscriptions::*;

mod block_modes;
mod bounds;
mod families;
mod lifecycle;
mod variants;

pub(crate) fn cases() -> [crate::CaseResult; 4] {
    [
        crate::case("subscriptions-nine-families", families::families()),
        crate::case("subscriptions-typed-variants", variants::variants()),
        crate::case("subscriptions-lifecycle", lifecycle::lifecycle()),
        crate::case("subscriptions-malformed-bounds", bounds::bounds()),
    ]
}

type Check<T = ()> = Result<T, &'static str>;
const GENERATION: u64 = 9;
const SERVER: u64 = 7;
const PEER: u64 = 8;
const SUBSCRIBE_DEADLINE: u64 = 10;
const CLEANUP_DEADLINE: u64 = 20;
const UNSUBSCRIBE_DEADLINE: u64 = 30;
const NEXT_DEADLINE: u64 = 40;

const REPLACEMENT_VALUE: u64 = 6;
const OVERSIZED_RECORD: usize = 81;
const ADDRESS: &str = "11111111111111111111111111111111";
const SIGNATURE: &str = concat!(
    "11111111111111111111111111111111",
    "11111111111111111111111111111111"
);
const ACCOUNT: &str = r#"{"executable":false,"lamports":18446744073709551615,"owner":"11111111111111111111111111111111","rentEpoch":18446744073709551615,"space":1,"data":["AQ==","base64"]}"#;

fn wire(owner: &mut Connection, expected: &str) -> Check {
    crate::require(
        owner.outbound().as_deref() == Some(expected.as_bytes()),
        "subscription outbound wire differs",
    )
}

fn ack(owner: &mut Connection, id: u32, result: &str) -> Check {
    owner
        .ingest(
            GENERATION,
            format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#).as_bytes(),
        )
        .map_err(|_| "subscription acknowledgement failed")
}

fn notify(owner: &mut Connection, method: &str, server: u64, result: &str) -> Check {
    owner
        .ingest(
            GENERATION,
            format!(r#"{{"jsonrpc":"2.0","method":"{method}","params":{{"subscription":{server},"result":{result}}}}}"#).as_bytes(),
        )
        .map_err(|_| "subscription notification ingest failed")
}

fn value<R: SubscriptionMethod>(
    owner: &mut Connection,
    handle: &Subscription<R>,
) -> Check<R::Response> {
    match owner.next(handle) {
        Ok(Drain::Notification(value)) => Ok(value),
        _ => Err("subscription typed notification missing"),
    }
}

fn delivered<R: SubscriptionMethod>(
    request: R,
    expected: &str,
    method: &str,
    raw: &str,
) -> Check<R::Response> {
    let mut owner = Connection::new(GENERATION, Budget::default());
    let handle = owner
        .subscribe(request, SUBSCRIBE_DEADLINE)
        .map_err(|_| "subscribe failed")?;
    wire(&mut owner, expected)?;
    ack(&mut owner, 1, "18446744073709551615")?;
    notify(&mut owner, method, u64::MAX, raw)?;
    value(&mut owner, &handle)
}

fn contextual(value: &str) -> String {
    format!(r#"{{"context":{{"slot":18446744073709551615}},"value":{value}}}"#)
}
