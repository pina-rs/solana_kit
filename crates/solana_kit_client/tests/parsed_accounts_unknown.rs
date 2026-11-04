//! Unknown parser names, unknown discriminators and opaque extension data keep
//! their exact source payloads; binary fallbacks stay binary.
#![cfg(feature = "parsed-accounts")]

mod parsed_accounts_support;

use parsed_accounts_support::{ADDRESS, decode};
use serde_json::json;
use solana_kit_client::{
    accounts::{assert_account_decoded, parse_json_rpc_account},
    methods::accounts::AccountData,
    parsed::{ParsedAccountData, TokenAccountState, TokenProgramAccount, UnknownParsed},
};

fn unknown_parts(projected: ParsedAccountData) -> UnknownParsed {
    match projected {
        ParsedAccountData::UnknownProgram(unknown) | ParsedAccountData::UnknownType(unknown) => {
            unknown
        }
        other => panic!("unknown payload expected, got {other:?}"),
    }
}

#[test]
fn unknown_types_keep_payloads_without_a_known_info_shape() {
    let literal = r#"{"type":"future","value":18446744073709551615}"#;

    for program in [
        "spl-token",
        "stake",
        "vote",
        "nonce",
        "address-lookup-table",
        "bpf-upgradeable-loader",
        "config",
        "sysvar",
    ] {
        let unknown = unknown_parts(decode(program, literal).unwrap());
        assert_eq!(unknown.type_name.as_deref(), Some("future"));
        assert_eq!(unknown.value.get(), literal);
    }

    assert!(matches!(
        decode("spl-token", r#"{"type":"mint"}"#),
        Err(solana_kit_client::Error::Schema(_))
    ));
}

#[test]
fn oversized_parser_names_are_bounded_before_retaining_unknown_data() {
    let raw = serde_json::value::RawValue::from_string("0".into()).unwrap();
    let limits = solana_kit_client::Limits { bytes: 8, depth: 2 };

    assert!(matches!(
        solana_kit_client::parsed::decode_parts("oversized", &raw, limits),
        Err(solana_kit_client::Error::BodyLimit)
    ));
}

#[test]
fn an_unknown_parser_name_retains_its_exact_payload() {
    let literal = r#"{"type":"metadata","info":{"name":"future","exact":18446744073709551615}}"#;
    let unknown = unknown_parts(decode("spl-token-metadata", literal).unwrap());

    assert_eq!(unknown.program, "spl-token-metadata");
    assert_eq!(unknown.type_name.as_deref(), Some("metadata"));
    assert_eq!(unknown.value.get(), literal);

    let value: serde_json::Value = serde_json::from_str(unknown.value.get()).unwrap();
    assert_eq!(value["info"]["exact"], json!(18446744073709551615_u64));
}

#[test]
fn unknown_parser_payloads_need_no_object_shape() {
    let array = "[1,2,3]";
    let unknown = unknown_parts(decode("custom-program", array).unwrap());
    assert_eq!(unknown.program, "custom-program");
    assert_eq!(unknown.type_name, None);
    assert_eq!(unknown.value.get(), array);

    let numeric_type = r#"{"type":7,"info":{}}"#;
    let unknown = unknown_parts(decode("custom-program", numeric_type).unwrap());
    assert_eq!(unknown.type_name, None, "a non-string type is not a name");
    assert_eq!(unknown.value.get(), numeric_type);
}

#[test]
fn every_reference_family_reports_an_undeclared_discriminator_as_unknown() {
    for (program, literal, type_name) in [
        (
            "spl-token",
            r#"{"type":"futureToken","info":{"value":1}}"#,
            "futureToken",
        ),
        (
            "stake",
            r#"{"type":"uninitialized","info":{}}"#,
            "uninitialized",
        ),
        (
            "stake",
            r#"{"type":"rewardsPool","info":{}}"#,
            "rewardsPool",
        ),
        ("vote", r#"{"type":"futureVote","info":{}}"#, "futureVote"),
        (
            "nonce",
            r#"{"type":"uninitialized","info":{}}"#,
            "uninitialized",
        ),
        (
            "address-lookup-table",
            r#"{"type":"uninitialized","info":{}}"#,
            "uninitialized",
        ),
        (
            "bpf-upgradeable-loader",
            r#"{"type":"buffer","info":{"authority":null,"data":["","base64"]}}"#,
            "buffer",
        ),
        (
            "bpf-upgradeable-loader",
            r#"{"type":"uninitialized","info":{}}"#,
            "uninitialized",
        ),
        (
            "config",
            r#"{"type":"futureConfig","info":{}}"#,
            "futureConfig",
        ),
        ("sysvar", r#"{"type":"rewards","info":{}}"#, "rewards"),
        (
            "sysvar",
            r#"{"type":"futureSysvar","info":{}}"#,
            "futureSysvar",
        ),
    ] {
        let projected = decode(program, literal).unwrap();
        assert!(
            matches!(projected, ParsedAccountData::UnknownType(_)),
            "{program}/{type_name}"
        );
        let unknown = unknown_parts(projected);
        assert_eq!(unknown.program, program);
        assert_eq!(unknown.type_name.as_deref(), Some(type_name));
        assert_eq!(unknown.value.get(), literal);
    }
}

#[test]
fn undeclared_token_extensions_remain_exact_in_the_typed_account() {
    let literal = r#"{"type":"account","info":{"closeAuthority":"5scSndUhfZJ8j8wZz5UNHhvuPBhvN1RboTdkKSvFHLtW","delegate":"Fkc4FN7PPhyGsAcHPW3dBBJ4BvtYkDr2rBFBgFpvy3nB","delegatedAmount":{"amount":"1","decimals":0,"uiAmountString":"1"},"extensions":[{"extension":"neverSeen","state":{"nested":[{"exact":18446744073709551615}]}}],"isNative":true,"mint":"Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr","owner":"6UsGbaMgchgj4wiwKKuE1v5URHdcDfEiMSM25QpesKir","rentExemptReserve":{"amount":"2","decimals":9,"uiAmountString":"0.000000002"},"state":"frozen","tokenAmount":{"amount":"3","decimals":6,"uiAmountString":"0.000003"}}}"#;
    let ParsedAccountData::Token(TokenProgramAccount::Account(info)) =
        decode("spl-token-2022", literal).unwrap()
    else {
        panic!("token account expected")
    };

    assert_eq!(info.state, TokenAccountState::Frozen);
    let extensions = info.extensions.unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(extensions.get()).unwrap(),
        json!([{"extension": "neverSeen", "state": {"nested": [{"exact": 18446744073709551615_u64}]}}])
    );
}

#[test]
fn parsed_payloads_with_unknown_members_keep_the_original_account_bytes() {
    let literal = r#"{"type":"mint","info":{"decimals":0,"futureMember":{"a":1},"isInitialized":true,"supply":"0"}}"#;
    let raw = parsed_accounts_support::account_json("spl-token", literal);
    let account = parse_json_rpc_account(ADDRESS, &raw)
        .unwrap()
        .require()
        .unwrap();
    let parsed = assert_account_decoded(&account).unwrap();

    assert!(matches!(
        parsed
            .decode_typed(parsed_accounts_support::LIMITS)
            .unwrap(),
        ParsedAccountData::Token(TokenProgramAccount::Mint(_))
    ));
    assert_eq!(parsed.parsed.get(), literal);
}

#[test]
fn decode_parts_projects_a_payload_without_an_account_wrapper() {
    let raw = serde_json::value::RawValue::from_string(
        r#"{"type":"program","info":{"programData":"3vnUTQbDuCgfVn7yQcigUwMQNGkLBZ7GfKWb3gYbAY23"}}"#
            .to_owned(),
    )
    .unwrap();
    let projected = solana_kit_client::parsed::decode_parts(
        "bpf-upgradeable-loader",
        &raw,
        parsed_accounts_support::LIMITS,
    )
    .unwrap();

    assert!(matches!(
        projected,
        ParsedAccountData::BpfUpgradeableLoader(
            solana_kit_client::parsed::BpfUpgradeableLoaderProgramAccount::Program(_)
        )
    ));
}

#[test]
fn node_binary_fallback_is_untouched_and_never_typed() {
    let raw = r#"{"executable":false,"lamports":1,"owner":"11111111111111111111111111111111","rentEpoch":0,"space":3,"data":["AQID","base64"]}"#;
    let account = parse_json_rpc_account(ADDRESS, raw)
        .unwrap()
        .require()
        .unwrap();

    let AccountData::Bytes { bytes, slice } = &account.data else {
        panic!("binary fallback expected")
    };
    assert_eq!(bytes, &[1, 2, 3]);
    assert_eq!(*slice, None);
    assert!(assert_account_decoded(&account).is_err());
}
