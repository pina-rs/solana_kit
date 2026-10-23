//! Literal public account helper checks, independent of SDK encoders.

use futures::executor::block_on;
use solana_kit_client::{
    Client, Error, Limits, Reply, Request, Transport, TransportError,
    accounts::*,
    methods::{Commitment, ReadConfig, accounts::AccountData},
};
use solana_kit_core::{
    Address,
    accounts::MaybeAccount,
    codecs::{Limits as CodecLimits, Number},
};
use std::{
    collections::hash_map::DefaultHasher,
    error::Error as _,
    future::{Ready, ready},
    hash::{Hash, Hasher},
};

const ADDRESS: &str = "11111111111111111111111111111111";
const RAW: &str = r#"{"executable":true,"lamports":18446744073709551615,"owner":"11111111111111111111111111111111","rentEpoch":18446744073709551615,"space":3,"data":["AQID","base64"]}"#;

struct Literal {
    method: &'static str,
    params: String,
    value: String,
}

impl Transport for Literal {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let expected = format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"{}","params":{}}}"#,
            self.method, self.params
        );
        assert_eq!(request.body, expected.as_bytes());

        ready(Ok(Reply { status: 200, body: format!(r#"{{"jsonrpc":"2.0","id":1,"result":{{"context":{{"slot":18446744073709551615,"apiVersion":"future"}},"value":{}}}}}"#, self.value).into_bytes() }))
    }
}

fn address() -> Address {
    Address::from_bytes([0; 32])
}

fn client(method: &'static str, params: String, value: String) -> Client<Literal> {
    Client::new(
        Literal {
            method,
            params,
            value,
        },
        Limits::default(),
    )
}

#[test]
fn complete_options_and_full_width_metadata_survive_fetch() {
    let config = ReadConfig {
        commitment: Some(Commitment::Confirmed),
        min_context_slot: Some(u64::MAX),
    };
    let params = format!(
        r#"["{ADDRESS}",{{"encoding":"base64","commitment":"confirmed","minContextSlot":18446744073709551615}}]"#
    );
    let rpc = client("getAccountInfo", params, RAW.to_owned());
    let result = block_on(rpc.fetch_encoded_account(address(), config)).unwrap();
    let account = result.value.require().unwrap();

    assert_eq!(result.context.slot, u64::MAX);
    assert_eq!(account.address, address());
    assert_eq!(account.owner, address());
    assert_eq!(account.lamports, u64::MAX);
    assert_eq!(account.rent_epoch, Some(u64::MAX));
    assert!(account.executable);
    assert_eq!(account.data, [1, 2, 3]);
}

#[test]
fn multiple_fetch_preserves_duplicates_and_missing_neighbors() {
    let other = Address::from_bytes([1; 32]);
    let params = format!(r#"[["{ADDRESS}","{other}","{ADDRESS}"],{{"encoding":"base64"}}]"#);
    let rpc = client("getMultipleAccounts", params, format!("[{RAW},null,{RAW}]"));
    let result = block_on(
        rpc.fetch_encoded_accounts(vec![address(), other, address()], ReadConfig::default()),
    )
    .unwrap();

    assert_eq!(result.value[0], result.value[2]);
    assert_eq!(result.value[1], MaybeAccount::Missing(other));
    assert_eq!(assert_accounts_exist(&result.value), Err(vec![other]));
}

#[test]
fn missing_and_inconsistent_responses_are_distinct() {
    let params = format!(r#"["{ADDRESS}",{{"encoding":"base64"}}]"#);
    let rpc = client("getAccountInfo", params, "null".to_owned());
    let value = block_on(rpc.fetch_encoded_account(address(), ReadConfig::default())).unwrap();
    assert_eq!(value.value, MaybeAccount::Missing(address()));

    let params = format!(r#"[["{ADDRESS}"],{{"encoding":"base64"}}]"#);
    let rpc = client("getMultipleAccounts", params, "[]".to_owned());
    assert!(matches!(
        block_on(rpc.fetch_encoded_accounts(vec![address()], ReadConfig::default())),
        Err(Error::AccountData)
    ));
}

#[test]
fn standalone_parsers_preserve_allocation_and_reject_malformed_encoding() {
    let raw = RAW.replace("\"space\":3", "\"space\":18446744073709551615");
    let account = parse_base64_rpc_account(address(), &raw)
        .unwrap()
        .require()
        .unwrap();
    assert_eq!(account.space, u64::MAX);
    assert_eq!(account.data, [1, 2, 3]);

    for malformed in [
        RAW.replace("AQID", "?"),
        RAW.replace("base64", "base64+zstd"),
        RAW.replace("AQID", "AB=="),
    ] {
        assert!(parse_base64_rpc_account(address(), &malformed).is_err());
    }
}

#[test]
fn legacy_base58_plain_base64_and_absent_rent_preserve_facts() {
    let raw = RAW
        .replace(r#"["AQID","base64"]"#, r#""Ldp""#)
        .replace(r#""rentEpoch":18446744073709551615,"#, "");
    let account = parse_base58_rpc_account(address(), &raw)
        .unwrap()
        .require()
        .unwrap();
    assert_eq!(account.data, [1, 2, 3]);
    assert_eq!(account.rent_epoch, None);

    let raw = RAW.replace(r#"["AQID","base64"]"#, r#""AQID""#);
    assert_eq!(
        parse_base64_rpc_account(address(), &raw)
            .unwrap()
            .require()
            .unwrap()
            .data,
        [1, 2, 3]
    );
    assert_eq!(
        parse_base58_rpc_account(address(), "null").unwrap(),
        MaybeAccount::Missing(address())
    );
}

#[test]
fn opaque_parsed_json_and_binary_fallback_keep_their_shapes() {
    let payload =
        r#"{"program":"future","parsed":{ "type":"unknown","info":null,"other":1e400 },"space":3}"#;
    let raw = RAW.replace(r#"["AQID","base64"]"#, payload);
    let params = format!(r#"["{ADDRESS}",{{"encoding":"jsonParsed"}}]"#);
    let rpc = client("getAccountInfo", params, raw.clone());
    let result = block_on(rpc.fetch_json_parsed_account(address(), ReadConfig::default())).unwrap();
    let AccountData::Parsed(parsed) = result.value.require().unwrap().data else {
        panic!()
    };
    assert_eq!(parsed.program, "future");
    assert_eq!(
        parsed.parsed.get(),
        r#"{ "type":"unknown","info":null,"other":1e400 }"#
    );
    assert!(matches!(
        parse_json_rpc_account(address(), RAW)
            .unwrap()
            .require()
            .unwrap()
            .data,
        AccountData::Bytes { .. }
    ));
    assert!(parse_json_rpc_account(address(), &raw).is_ok());
}

#[test]
fn borrowed_decode_retains_snapshot_cause_and_typed_missing_state() {
    let encoded = parse_base64_rpc_account(address(), RAW).unwrap();
    let failure =
        decode_maybe_account(&encoded, &Number::<u16>::LE, CodecLimits::new(1024, 64)).unwrap_err();
    assert!(failure.source().is_some());
    assert_eq!(encoded.clone().require().unwrap().lamports, u64::MAX);
    let decoded = decode_maybe_account(&encoded, &Number::<u32>::LE, CodecLimits::new(1024, 64));
    assert!(decoded.is_err());

    let short = parse_base64_rpc_account(address(), &RAW.replace("AQID", "AQI=")).unwrap();
    let decoded = decode_maybe_account(&short, &Number::<u16>::LE, CodecLimits::new(1024, 64))
        .unwrap()
        .require()
        .unwrap();
    assert_eq!(decoded.data, 513_u16);
    assert_eq!(decoded.space, 3);

    let missing = MaybeAccount::Missing(address());
    assert_eq!(
        decode_maybe_account(&missing, &Number::<u32>::LE, CodecLimits::new(1024, 64)).unwrap(),
        MaybeAccount::Missing(address())
    );
}

#[test]
fn equality_hash_debug_and_existence_have_native_contracts() {
    let value = parse_base64_rpc_account(address(), RAW).unwrap();
    let mut left = DefaultHasher::new();
    let mut right = DefaultHasher::new();
    value.hash(&mut left);
    value.clone().hash(&mut right);

    assert!(value.exists());
    assert_eq!(left.finish(), right.finish());
    assert!(format!("{value:?}").contains("lamports"));
    assert!(assert_accounts_exist::<u8>(&[]).is_ok());
}

#[test]
fn parsed_multiple_and_decoded_assertions_preserve_positions() {
    let parsed = RAW.replace(
        r#"["AQID","base64"]"#,
        r#"{"program":"future","parsed":{"type":"custom","info":{"a":1}},"space":3}"#,
    );
    let params = format!(r#"[["{ADDRESS}","{ADDRESS}","{ADDRESS}"],{{"encoding":"jsonParsed"}}]"#);
    let rpc = client(
        "getMultipleAccounts",
        params,
        format!("[{parsed},null,{RAW}]"),
    );
    let result =
        block_on(rpc.fetch_json_parsed_accounts(vec![address(); 3], ReadConfig::default()))
            .unwrap();

    assert!(
        assert_maybe_account_decoded(&result.value[0])
            .unwrap()
            .is_some()
    );
    assert!(
        assert_maybe_account_decoded(&result.value[1])
            .unwrap()
            .is_none()
    );
    assert!(assert_maybe_account_decoded(&result.value[2]).is_err());
    assert_eq!(
        assert_maybe_accounts_decoded(&result.value),
        Err(vec![address()])
    );
    assert!(assert_accounts_decoded(&[]).is_ok());
}
