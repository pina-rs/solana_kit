//! Public observations from a standalone consumer.
use futures::executor::block_on;
use solana_kit_client::{
    Client, Error, Limits, Reply, Request, Transport, TransportError, TransportFailureKind,
    accounts::*,
    methods::{
        Commitment, ReadConfig,
        accounts::{AccountData, ParsedAccount},
    },
};
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, BASE_ACCOUNT_SIZE, MaybeAccount},
    codecs::{self, Decoder, Limits as Budget, Number, Reader},
};
use std::{
    cell::Cell,
    error::Error as _,
    future::Future,
    hash::{Hash, Hasher},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

const ADDRESS_TEXT: &str = "11111111111111111111111111111111";
const RAW: &str = r#"{"executable":true,"lamports":18446744073709551615,"owner":"11111111111111111111111111111111","rentEpoch":18446744073709551615,"space":3,"data":["AQID","base64"]}"#;
const PAYLOAD: &str =
    r#"{"program":"future","parsed":{ "type":"new","info":null,"wide":1e400 },"space":3}"#;
fn a() -> Address {
    Address::from_bytes([0; 32])
}
fn options() -> ReadConfig {
    ReadConfig {
        commitment: Some(Commitment::Finalized),
        min_context_slot: Some(u64::MAX),
    }
}
fn envelope(value: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":1,"result":{{"context":{{"slot":18446744073709551615,"apiVersion":"opaque"}},"value":{value}}}}}"#
    )
}
struct Local {
    expected: String,
    reply: String,
    polls: Rc<Cell<usize>>,
    failure: bool,
}
struct Exchange {
    reply: Option<Result<Reply, TransportError>>,
    polls: Rc<Cell<usize>>,
}
impl Future for Exchange {
    type Output = Result<Reply, TransportError>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let count = self.polls.get();
        self.polls.set(count + 1);

        if count == 0 {
            cx.waker().wake_by_ref();

            return Poll::Pending;
        }

        Poll::Ready(self.reply.take().unwrap())
    }
}
impl Transport for Local {
    type Exchange<'b> = Exchange;
    fn exchange(&self, request: Request) -> Exchange {
        assert_eq!(String::from_utf8(request.body).unwrap(), self.expected);
        let reply = if self.failure {
            Err(TransportError::new(TransportFailureKind::Denied))
        } else {
            Ok(Reply {
                status: 200,
                body: self.reply.as_bytes().to_vec(),
            })
        };

        Exchange {
            reply: Some(reply),
            polls: self.polls.clone(),
        }
    }
}
fn rpc(
    method: &str,
    params: &str,
    reply: String,
    failure: bool,
) -> (Client<Local>, Rc<Cell<usize>>) {
    let polls = Rc::new(Cell::new(0));
    let expected = format!(r#"{{"jsonrpc":"2.0","id":1,"method":"{method}","params":{params}}}"#);
    (
        Client::new(
            Local {
                expected,
                reply,
                polls: polls.clone(),
                failure,
            },
            Limits::default(),
        ),
        polls,
    )
}
fn params(multiple: bool, encoding: &str) -> String {
    let keys = if multiple {
        format!(r#"["{ADDRESS_TEXT}","{ADDRESS_TEXT}","{ADDRESS_TEXT}"]"#)
    } else {
        format!(r#""{ADDRESS_TEXT}""#)
    };

    if multiple || encoding == "jsonParsed" {
        return format!(
            r#"[{keys},{{"commitment":"finalized","minContextSlot":18446744073709551615,"encoding":"{encoding}"}}]"#
        );
    }

    format!(
        r#"[{keys},{{"encoding":"{encoding}","commitment":"finalized","minContextSlot":18446744073709551615}}]"#
    )
}
#[test]
fn all_four_fetches_have_literal_options_context_and_local_pending() {
    for (parsed, multiple) in [(false, false), (false, true), (true, false), (true, true)] {
        fetch_case(parsed, multiple);
    }
}

fn fetch_case(parsed: bool, multiple: bool) {
    let encoding = if parsed { "jsonParsed" } else { "base64" };
    let method = if multiple {
        "getMultipleAccounts"
    } else {
        "getAccountInfo"
    };
    let value = if multiple {
        format!("[{RAW},null,{RAW}]")
    } else {
        RAW.to_owned()
    };

    let (client, polls) = rpc(method, &params(multiple, encoding), envelope(&value), false);

    match (parsed, multiple) {
        (false, false) => {
            let result = block_on(client.fetch_encoded_account(a(), options())).unwrap();
            assert_eq!(result.context.slot, u64::MAX);
            assert_eq!(result.value.require().unwrap().lamports, u64::MAX);
        }
        (false, true) => {
            let result = block_on(client.fetch_encoded_accounts(vec![a(); 3], options())).unwrap();
            assert_eq!(result.context.slot, u64::MAX);
            assert_eq!(result.value[0], result.value[2]);
            assert_eq!(result.value[1], MaybeAccount::Missing(a()));
        }
        (true, false) => {
            let result = block_on(client.fetch_json_parsed_account(a(), options())).unwrap();
            assert_eq!(result.context.slot, u64::MAX);
            assert!(matches!(
                result.value.require().unwrap().data,
                AccountData::Bytes { .. }
            ));
        }
        (true, true) => {
            let result =
                block_on(client.fetch_json_parsed_accounts(vec![a(); 3], options())).unwrap();
            assert_eq!(result.context.slot, u64::MAX);
            assert!(!result.value[1].exists());
            assert_eq!(result.value.len(), 3);
        }
    }

    assert_eq!(polls.get(), 2);
}
#[test]
fn cardinality_rejects_both_directions_for_both_multiple_helpers() {
    for value in [format!("[{RAW}]"), format!("[{RAW},null,{RAW},null]")] {
        for parsed in [false, true] {
            cardinality_case(parsed, &value);
        }
    }
}

fn cardinality_case(parsed: bool, value: &str) {
    let encoding = if parsed { "jsonParsed" } else { "base64" };
    let (client, _) = rpc(
        "getMultipleAccounts",
        &params(true, encoding),
        envelope(value),
        false,
    );
    let failure = if parsed {
        block_on(client.fetch_json_parsed_accounts(vec![a(); 3], options())).unwrap_err()
    } else {
        block_on(client.fetch_encoded_accounts(vec![a(); 3], options())).unwrap_err()
    };

    assert!(matches!(failure, Error::AccountData));
}
#[test]
fn protocol_and_transport_errors_remain_errors() {
    let cases = [
        (
            r#"{"jsonrpc":"2.0","id":2,"result":null}"#.to_owned(),
            false,
        ),
        (
            r#"{"jsonrpc":"2.0","id":1,"result":null,"error":{"code":-1,"message":"bad"}}"#
                .to_owned(),
            false,
        ),
        (
            r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32001,"message":"denied","data":null}}"#
                .to_owned(),
            false,
        ),
        (String::new(), true),
    ];

    for (reply, denied) in cases {
        let (client, _) = rpc("getAccountInfo", &params(false, "base64"), reply, denied);
        let error = block_on(client.fetch_encoded_account(a(), options())).unwrap_err();
        assert!(matches!(
            error,
            Error::Correlation | Error::Envelope | Error::Remote(_) | Error::Transport(_)
        ));
    }
}
#[test]
fn standalone_and_node_boundaries_differ_only_in_documented_fields() {
    for raw in [
        RAW.replace("\"space\":3", "\"space\":99"),
        RAW.replace(r#""rentEpoch":18446744073709551615,"#, ""),
    ] {
        let standalone = parse_base64_rpc_account(a(), &raw)
            .unwrap()
            .require()
            .unwrap();
        assert_eq!(standalone.data, [1, 2, 3]);
        let (client, _) = rpc(
            "getAccountInfo",
            &params(false, "base64"),
            envelope(&raw),
            false,
        );
        assert!(block_on(client.fetch_encoded_account(a(), options())).is_err());
    }
    for raw in [
        RAW.replace("18446744073709551615", "18446744073709551616"),
        RAW.replace("AQID", "AB=="),
        RAW.replace("base64", "base58"),
        RAW.replace("\"space\":3", "\"space\":-1"),
        RAW.replace("\"rentEpoch\":18446744073709551615", "\"rentEpoch\":null"),
        RAW.replace("\"executable\":true", "\"executable\":null"),
    ] {
        assert!(parse_base64_rpc_account(a(), &raw).is_err());
    }
    assert_eq!(
        parse_base64_rpc_account(a(), &RAW.replace(r#"["AQID","base64"]"#, r#""AQID""#))
            .unwrap()
            .require()
            .unwrap()
            .data,
        [1, 2, 3]
    );
    assert_eq!(
        parse_base58_rpc_account(
            a(),
            &RAW.replace(r#"["AQID","base64"]"#, r#"["Ldp","base58"]"#)
        )
        .unwrap()
        .require()
        .unwrap()
        .data,
        [1, 2, 3]
    );
}
#[test]
fn parsed_unknown_json_is_exact_and_metadata_keeps_its_identity() {
    let raw = RAW.replace(r#"["AQID","base64"]"#, PAYLOAD);
    let parsed = parse_json_rpc_account(a(), &raw)
        .unwrap()
        .require()
        .unwrap();
    let pointer = assert_account_decoded(&parsed).unwrap();
    let AccountData::Parsed(ref data) = parsed.data else {
        panic!()
    };
    assert!(std::ptr::eq(pointer, data));
    assert_eq!(
        data.parsed.get(),
        r#"{ "type":"new","info":null,"wide":1e400 }"#
    );
    assert_eq!(data.program, "future");

    assert_eq!(data.space, Some(3));
    assert!(parse_json_rpc_account(a(), &raw.replace("\"space\":3}", "\"space\":8}")).is_ok());
    let (client, _) = rpc(
        "getAccountInfo",
        &params(false, "jsonParsed"),
        envelope(&raw.replace("\"space\":3}", "\"space\":8}")),
        false,
    );
    assert!(block_on(client.fetch_json_parsed_account(a(), options())).is_err());
}
struct Custom {
    fail: bool,
}
impl Decoder<Vec<u8>> for Custom {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<u8>, codecs::Error> {
        reader.claim(3)?;

        if self.fail {
            return Err(codecs::Error::InvalidDiscriminator);
        }

        Ok(reader.take(3)?.iter().rev().copied().collect())
    }
}
#[test]
fn custom_decoder_preserves_originals_exact_causes_and_limits() {
    let original = parse_base64_rpc_account(a(), RAW)
        .unwrap()
        .require()
        .unwrap();
    let decoded = decode_account(&original, &Custom { fail: false }, Budget::new(3, 3)).unwrap();
    assert_eq!(decoded.data, [3, 2, 1]);
    assert_eq!(decoded.address, original.address);
    assert_eq!(decoded.owner, original.owner);
    assert_eq!(decoded.space, original.space);
    assert_eq!(decoded.rent_epoch, original.rent_epoch);
    assert_eq!(decoded.executable, original.executable);

    assert_eq!(decoded.lamports, original.lamports);
    for (decoder, budget, cause) in [
        (
            Custom { fail: true },
            Budget::new(3, 3),
            codecs::Error::InvalidDiscriminator,
        ),
        (
            Custom { fail: false },
            Budget::new(2, 3),
            codecs::Error::LimitExceeded,
        ),
        (
            Custom { fail: false },
            Budget::new(3, 2),
            codecs::Error::LimitExceeded,
        ),
    ] {
        let error = decode_account(&original, &decoder, budget).unwrap_err();
        assert_eq!(
            error,
            AccountError::Decode {
                address: a(),
                source: cause
            }
        );
        assert_eq!(
            error.source().unwrap().downcast_ref::<codecs::Error>(),
            Some(&cause)
        );
        assert_eq!(original.data, [1, 2, 3]);
    }
    assert_eq!(
        decode_maybe_account(
            &MaybeAccount::Missing(a()),
            &Custom { fail: true },
            Budget::new(0, 0)
        )
        .unwrap(),
        MaybeAccount::Missing(a())
    );
    assert!(decode_account(&original, &Number::<u16>::LE, Budget::new(3, 3)).is_err());
}
#[test]
fn native_value_construction_hash_equality_debug_and_missing_assertions() {
    let value = Account {
        address: a(),
        owner: a(),
        lamports: 9,
        space: 17,
        executable: false,
        rent_epoch: None,
        data: 42_u32,
    };
    let present = MaybeAccount::Present(value.clone());
    let missing = MaybeAccount::Missing(a());
    let mut left = std::collections::hash_map::DefaultHasher::new();
    let mut right = std::collections::hash_map::DefaultHasher::new();
    present.hash(&mut left);
    present.clone().hash(&mut right);
    assert_eq!(left.finish(), right.finish());

    assert_eq!(present.clone().require().unwrap(), value);
    assert!(present.exists());
    assert!(!missing.exists());
    assert_eq!(missing.address(), a());
    assert_eq!(missing.clone().require(), Err(AccountError::Missing(a())));
    assert_eq!(
        assert_accounts_exist(&[missing.clone(), present, missing]),
        Err(vec![a(), a()])
    );
    assert!(format!("{value:?}").contains("42"));
    assert_eq!(BASE_ACCOUNT_SIZE, 128);
    let parsed = ParsedAccount {
        program: "custom".to_owned(),
        parsed: serde_json::value::RawValue::from_string("{\"type\":null}".to_owned()).unwrap(),
        space: Some(17),
    };
    assert!(format!("{parsed:?}").contains("custom"));
}
#[test]
fn assertions_retain_order_duplicates_and_compressed_state() {
    let bytes = parse_json_rpc_account(a(), RAW).unwrap();
    let parsed =
        parse_json_rpc_account(a(), &RAW.replace(r#"["AQID","base64"]"#, PAYLOAD)).unwrap();
    let compressed = Account {
        address: a(),
        owner: a(),
        lamports: 1,
        space: 99,
        rent_epoch: None,
        executable: false,
        data: AccountData::Zstd {
            compressed: vec![1, 2],
            slice: None,
        },
    };
    assert!(assert_account_decoded(&compressed).is_err());
    assert_eq!(
        assert_maybe_accounts_decoded(&[
            MaybeAccount::Missing(a()),
            bytes,
            parsed,
            MaybeAccount::Present(compressed)
        ]),
        Err(vec![a(), a()])
    );
    assert!(
        assert_maybe_account_decoded(&MaybeAccount::Missing(a()))
            .unwrap()
            .is_none()
    );
    assert!(assert_accounts_decoded(&[]).is_ok());
}
