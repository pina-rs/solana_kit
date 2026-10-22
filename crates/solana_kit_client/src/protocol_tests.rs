use crate::Error;
use crate::json::{self, Limits, Presence};
use crate::methods::{AccountInfo, Balance, Commitment, LatestBlockhash, ReadConfig};
use crate::{Reply, Request};

const REQUEST_ID: u64 = 7;
const ADDRESS: &str = "11111111111111111111111111111111";
const CONTEXT: &str = r#"{"slot":18446744073709551615,"apiVersion":"4.2"}"#;

fn reply(body: impl Into<Vec<u8>>) -> Reply {
    Reply {
        status: 200,
        body: body.into(),
    }
}

fn envelope(result: &str) -> Vec<u8> {
    format!(r#"{{"jsonrpc":"2.0","id":7,"result":{result}}}"#).into_bytes()
}

fn balance(body: impl Into<Vec<u8>>) -> Result<u64, Error> {
    let value: crate::methods::Contextual<u64> =
        json::response(REQUEST_ID, reply(body), Limits::default())?;
    assert_eq!(value.context.slot, u64::MAX);
    Ok(value.value)
}

#[test]
fn exact_request_vectors_preserve_omission_and_large_integer() {
    let absent = json::request(
        REQUEST_ID,
        &Balance(ADDRESS.parse().unwrap(), None),
        Limits::default(),
    )
    .unwrap();
    assert_eq!(absent.body, br#"{"jsonrpc":"2.0","id":7,"method":"getBalance","params":["11111111111111111111111111111111"]}"#);
    let config = ReadConfig {
        commitment: Some(Commitment::Confirmed),
        min_context_slot: Some(u64::MAX),
    };
    let present = json::request(
        REQUEST_ID,
        &Balance(ADDRESS.parse().unwrap(), Some(config)),
        Limits::default(),
    )
    .unwrap();
    assert_eq!(present.body, br#"{"jsonrpc":"2.0","id":7,"method":"getBalance","params":["11111111111111111111111111111111",{"commitment":"confirmed","minContextSlot":18446744073709551615}]}"#);
    let blockhash = json::request(REQUEST_ID, &LatestBlockhash(None), Limits::default()).unwrap();
    assert_eq!(
        blockhash.body,
        br#"{"jsonrpc":"2.0","id":7,"method":"getLatestBlockhash","params":[]}"#
    );
    let account = json::request(
        REQUEST_ID,
        &AccountInfo(ADDRESS.parse().unwrap(), ReadConfig::default()),
        Limits::default(),
    )
    .unwrap();

    assert_eq!(account.body, br#"{"jsonrpc":"2.0","id":7,"method":"getAccountInfo","params":["11111111111111111111111111111111",{"encoding":"base64"}]}"#);
}

#[test]
fn u64_is_lossless_and_does_not_coerce_float_string_or_overflow() {
    for integer in ["9007199254740993", "18446744073709551615"] {
        assert_eq!(
            balance(envelope(&format!(
                r#"{{"context":{CONTEXT},"value":{integer}}}"#
            )))
            .unwrap(),
            integer.parse::<u64>().unwrap()
        );
    }

    for invalid in ["18446744073709551616", "-1", "1.0", "1e0", "\"1\"", "null"] {
        assert!(
            matches!(
                balance(envelope(&format!(
                    r#"{{"context":{CONTEXT},"value":{invalid}}}"#
                ))),
                Err(Error::Schema(_))
            ),
            "{invalid}"
        );
    }
}

#[test]
fn framing_rejects_wrong_version_id_ambiguous_duplicate_and_trailing_data() {
    let invalid = [
        r#"{"jsonrpc":"1.0","id":7,"result":0}"#,
        r#"{"jsonrpc":"2.0","id":8,"result":0}"#,
        r#"{"jsonrpc":"2.0","id":"7","result":0}"#,
        r#"{"jsonrpc":"2.0","id":7.0,"result":0}"#,
        r#"{"jsonrpc":"2.0","id":null,"result":0}"#,
        r#"{"jsonrpc":"2.0","id":7}"#,
        r#"{"jsonrpc":"2.0","id":7,"result":null,"error":null}"#,
        r#"{"jsonrpc":"2.0","id":7,"result":0,"error":{"code":-1,"message":"bad"}}"#,
        r#"{"jsonrpc":"2.0","id":7,"id":7,"result":0}"#,
        r#"{"jsonrpc":"2.0","id":7,"result":0,"result":1}"#,
        r#"{"jsonrpc":"2.0","id":7,"result":0} []"#,
        r#"[{"jsonrpc":"2.0","id":7,"result":0}]"#,
    ];

    for body in invalid {
        assert!(
            json::response::<u64>(REQUEST_ID, reply(body.as_bytes()), Limits::default()).is_err(),
            "{body}"
        );
    }
}

#[test]
fn required_context_value_and_nullable_account_have_distinct_missing_policy() {
    let null = envelope(&format!(r#"{{"context":{CONTEXT},"value":null}}"#));
    let value: crate::wire::NullableContext<crate::wire::Account> =
        json::response(REQUEST_ID, reply(null), Limits::default()).unwrap();
    assert!(value.value.is_none());

    for malformed in [
        format!(r#"{{"context":{CONTEXT}}}"#),
        r#"{"value":null}"#.into(),
        r#"{"context":{},"value":null}"#.into(),
    ] {
        assert!(
            json::response::<crate::wire::NullableContext<crate::wire::Account>>(
                REQUEST_ID,
                reply(envelope(&malformed)),
                Limits::default()
            )
            .is_err()
        );
    }
}

#[test]
fn typed_blockhash_and_account_reject_bad_domain_values() {
    let block = format!(
        r#"{{"context":{CONTEXT},"value":{{"blockhash":"{ADDRESS}","lastValidBlockHeight":18446744073709551615}}}}"#
    );
    let decoded: crate::methods::Contextual<crate::methods::BlockhashValue> =
        json::response(REQUEST_ID, reply(envelope(&block)), Limits::default()).unwrap();
    assert_eq!(decoded.value.last_valid_block_height, u64::MAX);
    assert!(
        json::response::<crate::methods::Contextual<crate::methods::BlockhashValue>>(
            REQUEST_ID,
            reply(envelope(&block.replace(ADDRESS, "invalid"))),
            Limits::default()
        )
        .is_err()
    );
    let account = format!(
        r#"{{"context":{CONTEXT},"value":{{"executable":false,"lamports":18446744073709551615,"owner":"{ADDRESS}","space":0,"rentEpoch":18446744073709551615,"data":["","base64"]}}}}"#
    );
    let value: crate::wire::NullableContext<crate::wire::Account> =
        json::response(REQUEST_ID, reply(envelope(&account)), Limits::default()).unwrap();
    assert_eq!(value.value.unwrap().lamports, u64::MAX);

    for malformed in [
        account.replace(ADDRESS, "bad"),
        account.replace("base64", "jsonParsed"),
        account.replace("false", "0"),
    ] {
        assert!(
            json::response::<crate::wire::NullableContext<crate::wire::Account>>(
                REQUEST_ID,
                reply(envelope(&malformed)),
                Limits::default()
            )
            .is_err()
        );
    }
}

#[test]
fn remote_data_preserves_missing_null_and_unknown_typed_payloads() {
    for (data, present) in [
        ("", false),
        (",\"data\":null", true),
        (",\"data\":{\"futureCode\":18446744073709551615}", true),
    ] {
        let body = format!(
            r#"{{"jsonrpc":"2.0","id":7,"error":{{"code":-32002,"message":"preflight failed"{data}}}}}"#
        );
        let Err(Error::Remote(error)) =
            json::response::<u64>(REQUEST_ID, reply(body.into_bytes()), Limits::default())
        else {
            panic!("remote error expected");
        };
        assert_eq!(error.code, -32002);
        assert_eq!(matches!(error.data, Presence::Present(_)), present);
    }
}

#[test]
fn body_and_depth_limits_precede_json_and_http_status_precedes_success() {
    assert!(matches!(
        json::response::<u64>(REQUEST_ID, reply(vec![b' '; 8193]), Limits::default()),
        Err(Error::BodyLimit)
    ));
    assert!(matches!(
        json::response::<u64>(REQUEST_ID, reply(vec![b'['; 33]), Limits::default()),
        Err(Error::DepthLimit)
    ));
    assert!(matches!(
        json::response::<u64>(
            REQUEST_ID,
            Reply {
                status: 503,
                body: envelope("0")
            },
            Limits::default()
        ),
        Err(Error::Status(503))
    ));
    let quoted = br#"{"jsonrpc":"2.0","id":7,"result":"[\\\"{{{{"}"#;
    let value: String = json::response(
        REQUEST_ID,
        reply(quoted.as_slice()),
        Limits {
            bytes: 100,
            depth: 1,
        },
    )
    .unwrap();
    assert!(value.starts_with('['));
    let _: Request = json::request(REQUEST_ID, &LatestBlockhash(None), Limits::default()).unwrap();
}

#[test]
fn optional_api_version_preserves_absence_and_rejects_explicit_null() {
    use crate::methods::Contextual;
    let missing = envelope(r#"{"context":{"slot":1},"value":0}"#);
    let value: Contextual<u64> =
        json::response(REQUEST_ID, reply(missing), Limits::default()).unwrap();
    assert_eq!(value.context.api_version, Presence::Missing);
    let explicit_null = envelope(r#"{"context":{"slot":1,"apiVersion":null},"value":0}"#);
    assert!(matches!(
        json::response::<Contextual<u64>>(REQUEST_ID, reply(explicit_null), Limits::default()),
        Err(Error::Schema(_))
    ));
}
