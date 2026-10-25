//! Public fetch/decode composition with explicit in-memory protocol fixtures.
extern crate std;

use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_client::{
    Client, Error, Limits, Presence, Reply, Request, Transport, TransportError,
    TransportFailureKind,
    methods::{Commitment, Contextual, ReadConfig},
};
use solana_kit_core::{
    Address, Hash,
    accounts::{AccountError, MaybeEncodedAccount},
    codecs::{Error as CodecError, Limits as DecodeLimits},
    sysvars::{CLOCK_ADDRESS, Clock, OWNER_ADDRESS, SLOT_HASHES_ADDRESS, SlotHashes, SysvarCodec},
};
use std::{
    error::Error as _,
    fmt, format,
    future::{Future, Ready, ready},
    pin::pin,
    string::{String, ToString},
    task::{Context, Poll, Waker},
    vec,
};

const CONTEXT_SLOT: u64 = u64::MAX;
const REPLY_LIMIT: usize = 8192;
const CLOCK_BYTES: [u8; 40] = [
    119, 233, 246, 16, 0, 0, 0, 0, 246, 255, 255, 255, 255, 255, 255, 255, 4, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 224, 177, 255, 255, 255, 255, 255, 255,
];

#[derive(Debug)]
struct OriginalCause;

impl fmt::Display for OriginalCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("original sysvar fixture cause")
    }
}
impl std::error::Error for OriginalCause {}

struct Fixture {
    address: Address,
    response: String,
    failure: bool,
    reply_limit: usize,
}

impl Transport for Fixture {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let body: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(
            body,
            serde_json::json!({"jsonrpc":"2.0", "id":1,
            "method":"getAccountInfo", "params":[self.address.to_string(), {
                "commitment":"confirmed", "minContextSlot":CONTEXT_SLOT, "encoding":"base64"}]})
        );
        assert_eq!(request.max_reply_bytes, self.reply_limit);

        if self.failure {
            return ready(Err(TransportError::with_source(
                TransportFailureKind::Io,
                OriginalCause,
            )));
        }

        ready(Ok(Reply {
            status: 200,
            body: self.response.as_bytes().to_vec(),
        }))
    }
}

fn fetch(
    address: Address,
    response: String,
    failure: bool,
    reply_limit: usize,
) -> Result<Contextual<MaybeEncodedAccount>, Error> {
    let client = Client::new(
        Fixture {
            address,
            response,
            failure,
            reply_limit,
        },
        Limits {
            bytes: reply_limit,
            depth: 32,
        },
    );
    let future = client.fetch_encoded_account(
        address,
        ReadConfig {
            commitment: Some(Commitment::Confirmed),
            min_context_slot: Some(CONTEXT_SLOT),
        },
    );
    let mut future = pin!(future);

    match future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("immediate fixture unexpectedly pending"),
    }
}

fn response(value: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":1,"result":{{"context":{{"slot":{CONTEXT_SLOT},"apiVersion":"fixture"}},"value":{value}}}}}"#
    )
}

fn snapshot(data: &[u8], owner: Address) -> String {
    response(&format!(
        r#"{{"owner":"{owner}","lamports":18446744073709551615,"executable":false,"space":{},"rentEpoch":18446744073709551615,"data":["{}","base64"]}}"#,
        data.len(),
        STANDARD.encode(data)
    ))
}

/// Client account fetch and strict core decoding retain every snapshot/context field.
pub fn clock_composition() {
    let Contextual { context, value } = fetch(
        CLOCK_ADDRESS,
        snapshot(&CLOCK_BYTES, OWNER_ADDRESS),
        false,
        REPLY_LIMIT,
    )
    .unwrap();
    let decoded = Contextual {
        context,
        value: value
            .require()
            .unwrap()
            .decode(
                &SysvarCodec::<Clock>::NEW,
                OWNER_ADDRESS,
                DecodeLimits::new(40, 0),
            )
            .unwrap(),
    };
    assert_eq!(decoded.context.slot, CONTEXT_SLOT);
    assert_eq!(
        decoded.context.api_version,
        Presence::Present("fixture".into())
    );
    assert_eq!(decoded.value.address, CLOCK_ADDRESS);
    assert_eq!(decoded.value.owner, OWNER_ADDRESS);
    assert_eq!(decoded.value.lamports, u64::MAX);
    assert_eq!(decoded.value.rent_epoch, Some(u64::MAX));
    assert_eq!(decoded.value.space, 40);
    assert!(!decoded.value.executable);
    assert_eq!(
        decoded.value.data,
        Clock {
            slot: 284617079,
            epoch_start_timestamp: -10,
            epoch: 4,
            leader_schedule_epoch: 0,
            unix_timestamp: -20000
        }
    );
}

/// Variable-length sysvars share the existing codec's u64 count and entry budget.
pub fn history_composition() {
    let mut bytes = vec![
        1, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
    ];
    bytes.extend_from_slice(&[7; 32]);
    let snapshot = fetch(
        SLOT_HASHES_ADDRESS,
        snapshot(&bytes, OWNER_ADDRESS),
        false,
        REPLY_LIMIT,
    )
    .unwrap();
    assert_eq!(snapshot.context.slot, CONTEXT_SLOT);
    let account = snapshot.value.require().unwrap();
    let decoded = account
        .clone()
        .decode(
            &SysvarCodec::<SlotHashes>::NEW,
            OWNER_ADDRESS,
            DecodeLimits::new(bytes.len(), 1),
        )
        .unwrap();
    assert_eq!(decoded.address, SLOT_HASHES_ADDRESS);
    assert_eq!(decoded.data.len(), 1);
    assert_eq!(decoded.data[0].slot, u64::MAX);
    assert_eq!(decoded.data[0].hash, Hash::from_bytes([7; 32]));
    assert_eq!(
        account.decode(
            &SysvarCodec::<SlotHashes>::NEW,
            OWNER_ADDRESS,
            DecodeLimits::new(bytes.len(), 0)
        ),
        Err(AccountError::Decode {
            address: SLOT_HASHES_ADDRESS,
            source: CodecError::LimitExceeded
        })
    );
}

/// Missing accounts and wrong owners remain exact errors rather than default sysvars.
pub fn missing_and_owner() {
    let missing = fetch(CLOCK_ADDRESS, response("null"), false, REPLY_LIMIT).unwrap();
    assert_eq!(missing.context.slot, CONTEXT_SLOT);
    assert_eq!(
        missing.value.require(),
        Err(AccountError::Missing(CLOCK_ADDRESS))
    );
    let wrong_owner = Address::from_bytes([0; 32]);
    let snapshot = fetch(
        CLOCK_ADDRESS,
        snapshot(&CLOCK_BYTES, wrong_owner),
        false,
        REPLY_LIMIT,
    )
    .unwrap();
    assert_eq!(snapshot.context.slot, CONTEXT_SLOT);
    assert_eq!(
        snapshot.value.require().unwrap().decode(
            &SysvarCodec::<Clock>::NEW,
            OWNER_ADDRESS,
            DecodeLimits::new(40, 0)
        ),
        Err(AccountError::OwnerMismatch {
            address: CLOCK_ADDRESS,
            expected: OWNER_ADDRESS,
            actual: wrong_owner
        })
    );
}

/// Strict decoding rejects truncation, trailing bytes and byte-budget exhaustion.
pub fn decoding_failures() {
    let mut trailing = CLOCK_BYTES.to_vec();
    trailing.push(0);

    for (bytes, limit, error) in [
        (CLOCK_BYTES[..39].to_vec(), 40, CodecError::Truncated),
        (trailing, 41, CodecError::TrailingBytes),
        (CLOCK_BYTES.to_vec(), 39, CodecError::LimitExceeded),
    ] {
        let snapshot = fetch(
            CLOCK_ADDRESS,
            snapshot(&bytes, OWNER_ADDRESS),
            false,
            REPLY_LIMIT,
        )
        .unwrap();
        assert_eq!(snapshot.context.slot, CONTEXT_SLOT);
        assert_eq!(
            snapshot.value.require().unwrap().decode(
                &SysvarCodec::<Clock>::NEW,
                OWNER_ADDRESS,
                DecodeLimits::new(limit, 0)
            ),
            Err(AccountError::Decode {
                address: CLOCK_ADDRESS,
                source: error
            })
        );
    }
}

/// Original transport causes, remote payloads, correlation and reply limits survive.
pub fn rpc_failures() {
    let error = fetch(CLOCK_ADDRESS, String::new(), true, REPLY_LIMIT).unwrap_err();
    let Error::Transport(error) = error else {
        panic!("transport category lost")
    };
    assert_eq!(error.kind, TransportFailureKind::Io);
    assert!(
        error
            .source()
            .unwrap()
            .downcast_ref::<OriginalCause>()
            .is_some()
    );
    let remote = r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32000,"message":"sysvar unavailable","data":{"slot":18446744073709551615}}}"#;
    let Error::Remote(remote) =
        fetch(CLOCK_ADDRESS, remote.into(), false, REPLY_LIMIT).unwrap_err()
    else {
        panic!("remote failure lost")
    };
    assert_eq!(remote.code, -32000);
    assert_eq!(remote.message, "sysvar unavailable");
    let Presence::Present(data) = remote.data else {
        panic!("remote data lost")
    };
    assert_eq!(data.get(), r#"{"slot":18446744073709551615}"#);
    let wrong_id = response("null").replace("\"id\":1", "\"id\":2");
    assert!(matches!(
        fetch(CLOCK_ADDRESS, wrong_id, false, REPLY_LIMIT),
        Err(Error::Correlation)
    ));
    assert!(matches!(
        fetch(
            CLOCK_ADDRESS,
            " ".repeat(REPLY_LIMIT + 1),
            false,
            REPLY_LIMIT
        ),
        Err(Error::BodyLimit)
    ));
}
