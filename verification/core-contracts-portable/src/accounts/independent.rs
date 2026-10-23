//! Fresh public boundary observations against the immutable source oracle.
use solana_kit_client::{
    Error, Presence,
    accounts::*,
    methods::{
        Method,
        accounts::{AccountConfig, AccountData, AccountEncoding, AccountInfoWithConfig, DataSlice},
    },
};
use solana_kit_core::{Address, accounts::MaybeAccount};

const OWNER: &str = "11111111111111111111111111111111";
const HEADER: &str =
    r#"{"executable":true,"lamports":17,"owner":"11111111111111111111111111111111","space":91}"#;
const PAYLOAD: &str =
    r#"{ "info": {"wide":18446744073709551616,"nested":[null,7]}, "type": "novel" }"#;

fn address() -> Address {
    Address::from_bytes([9; 32])
}

fn numeric(value: &str) -> String {
    HEADER
        .replace("\"lamports\":17", &format!("\"lamports\":{value}"))
        .replace("\"space\":91", &format!("\"space\":{value}"))
}

fn with_data(data: &str) -> String {
    HEADER.replace('}', &format!(",\"data\":{data}}}"))
}

fn node(raw: &str, encoding: AccountEncoding, slice: Option<DataSlice>) -> Result<(), Error> {
    let method = AccountInfoWithConfig(
        address(),
        Some(AccountConfig {
            encoding: Some(encoding),
            data_slice: slice,
            ..AccountConfig::default()
        }),
    );
    let value = format!("{},\"rentEpoch\":0}}", raw.strip_suffix('}').unwrap());
    let result = format!(r#"{{"context":{{"slot":1}},"value":{value}}}"#);
    method.decode(&result).map(|_| ())
}

pub fn header_requires_each_fact_and_preserves_rent_presence() {
    let parsed = parse_base_account(HEADER).unwrap();
    assert!(parsed.executable);
    assert_eq!((parsed.lamports, parsed.space), (17, 91));
    assert_eq!(parsed.owner.to_string(), OWNER);
    assert_eq!(parsed.rent_epoch, Presence::Missing);

    for (key, value) in [
        ("executable", "true"),
        ("lamports", "17"),
        ("owner", "\"11111111111111111111111111111111\""),
        ("space", "91"),
    ] {
        let mut object: serde_json::Value = serde_json::from_str(HEADER).unwrap();
        object.as_object_mut().unwrap().remove(key);
        assert!(
            parse_base_account(&object.to_string()).is_err(),
            "missing {key}"
        );
        let null = HEADER.replace(&format!("\"{key}\":{value}"), &format!("\"{key}\":null"));
        assert!(parse_base_account(&null).is_err(), "null {key}");
    }

    for epoch in [0, u64::MAX] {
        let raw = HEADER.replace('}', &format!(",\"rentEpoch\":{epoch}}}"));
        assert_eq!(
            parse_base_account(&raw).unwrap().rent_epoch,
            Presence::Present(epoch)
        );
    }

    assert!(parse_base_account(&HEADER.replace('}', ",\"rentEpoch\":null}")).is_err());
}

pub fn fresh_dart_grammar_and_complete_unicode_oracle_drive_all_standalone_numbers() {
    for row in include_str!("../oracle.tsv").lines() {
        let (input, result) = row.split_once('\t').unwrap();

        if input == "whitespace" {
            whitespace(result);

            continue;
        }

        number_case(input, result);
    }

    for code in [0, 0x85, 0x180e, 0x200b, 0x2060, 0xfffe, 0x10ffff] {
        let pad = char::from_u32(code).unwrap();
        let value = serde_json::to_string(&format!("{pad}1{pad}")).unwrap();
        assert!(
            parse_base_account(&numeric(&value)).is_err(),
            "U+{code:04X}"
        );
    }
}

fn whitespace(result: &str) {
    for code in result.split(',').map(|code| code.parse::<u32>().unwrap()) {
        let pad = char::from_u32(code).unwrap();
        let value = serde_json::to_string(&format!("{pad}+0x2a{pad}")).unwrap();
        assert_eq!(parse_base_account(&numeric(&value)).unwrap().lamports, 42);
    }
}

fn number_case(input: &str, result: &str) {
    let parsed = parse_base_account(&numeric(input));
    let expected = result.parse::<u64>();

    if input.contains("-0") || expected.is_err() {
        assert!(parsed.is_err(), "{input}: {result}");

        return;
    }

    let expected = expected.unwrap();
    let header = parsed.unwrap();
    assert_eq!(
        (header.lamports, header.space),
        (expected, expected),
        "{input}"
    );

    for data in [r#"["AQI=","base64"]"#, r#""AQI=""#] {
        let raw = numeric(input).replace('}', &format!(",\"data\":{data}}}"));
        let account = parse_base64_rpc_account(address(), &raw)
            .unwrap()
            .require()
            .unwrap();
        assert_eq!(account.data, [1, 2]);
    }
}

pub fn raw_json_numeric_types_and_malformed_documents_remain_errors() {
    for value in [
        "-0",
        "-1",
        "1.0",
        "1e0",
        "1E2",
        "true",
        "null",
        "18446744073709551616",
        "[1]",
        "{}",
        "\"+\"",
    ] {
        let raw = numeric(value);
        assert!(parse_base_account(&raw).is_err(), "{value}");
        let bytes = raw.replace('}', ",\"data\":[\"AQI=\",\"base64\"]}");
        assert!(
            parse_base64_rpc_account(address(), &bytes).is_err(),
            "{value}"
        );
    }

    for malformed in ["{", "nullx", "[]", "true", "", "{\"lamports\":}"] {
        assert!(parse_base_account(malformed).is_err(), "{malformed}");
        assert!(
            parse_json_rpc_account(address(), malformed).is_err(),
            "{malformed}"
        );
    }
}

pub fn parsed_optional_cross_product_keeps_exact_raw_and_independent_allocations() {
    for program in ["", ",\"program\":null", ",\"program\":\"novel\""] {
        for space in ["", ",\"space\":null", ",\"space\":4"] {
            parsed_case(program, space);
        }
    }

    let account = parse_base64_rpc_account(address(), &with_data(r#"["AQI=","base64"]"#))
        .unwrap()
        .require()
        .unwrap();
    assert_eq!((account.space, account.data.len()), (91, 2));
}

fn parsed_case(program: &str, space: &str) {
    let data = format!(r#"{{"parsed":{PAYLOAD}{program}{space}}}"#);
    let account = parse_json_rpc_account(address(), &with_data(&data))
        .unwrap()
        .require()
        .unwrap();
    let parsed = assert_account_decoded(&account).unwrap();
    let expected_program = if program.contains("novel") {
        "novel"
    } else {
        ""
    };

    assert_eq!(account.space, 91);
    assert_eq!(parsed.space, space.contains(":4").then_some(4));

    assert_eq!(parsed.program, expected_program);
    assert_eq!(parsed.parsed.get(), PAYLOAD);
    assert!(node(&with_data(&data), AccountEncoding::JsonParsed, None).is_err());
}

pub fn strict_node_never_inherits_metadata_string_coercion_or_optional_facts() {
    let full = with_data(r#"{"parsed":{"info":{},"type":"custom"},"program":"p","space":91}"#);
    assert!(node(&full, AccountEncoding::JsonParsed, None).is_ok());

    for (key, old) in [("lamports", "17"), ("space", "91")] {
        let raw = full.replacen(
            &format!("\"{key}\":{old}"),
            &format!("\"{key}\":\"{old}\""),
            1,
        );
        assert!(parse_json_rpc_account(address(), &raw).is_ok());
        assert!(node(&raw, AccountEncoding::JsonParsed, None).is_err());
    }

    for metadata in [
        "",
        ",\"program\":\"p\"",
        ",\"space\":91",
        ",\"program\":null,\"space\":91",
        ",\"program\":\"p\",\"space\":null",
    ] {
        let raw = with_data(&format!(r#"{{"parsed":{{"info":{{}}}}{metadata}}}"#));
        assert!(parse_json_rpc_account(address(), &raw).is_ok());
        assert!(node(&raw, AccountEncoding::JsonParsed, None).is_err());
    }
}

pub fn slice_lengths_and_compressed_states_stay_explicit() {
    let raw = with_data(r#"["AQI=","base64"]"#);
    let slice = Some(DataSlice {
        offset: 89,
        length: 4,
    });
    assert!(node(&raw, AccountEncoding::Base64, slice).is_ok());
    assert!(node(&raw, AccountEncoding::Base64, None).is_err());
    assert!(node(&raw.replace("AQI=", "AQ=="), AccountEncoding::Base64, slice).is_err());

    let compressed = with_data(r#"["AQI=","base64+zstd"]"#);
    assert!(node(&compressed, AccountEncoding::Base64Zstd, None).is_ok());
    assert!(parse_json_rpc_account(address(), &compressed).is_err());

    for seed in [3, 19] {
        let requested = Address::from_bytes([seed; 32]);
        let account = parse_json_rpc_account(requested, &raw)
            .unwrap()
            .require()
            .unwrap();
        assertions(account);
    }

    collections();
}

fn collections() {
    let first = Address::from_bytes([3; 32]);
    let second = Address::from_bytes([19; 32]);
    let expected = vec![second, first, second];
    let snapshots = [
        state(second, false),
        state(first, true),
        state(second, false),
    ];
    assert_eq!(assert_accounts_decoded(&snapshots), Err(expected.clone()));

    let maybe = [
        MaybeAccount::Missing(first),
        MaybeAccount::Present(state(second, false)),
        MaybeAccount::Present(state(first, true)),
        MaybeAccount::Present(state(second, false)),
    ];
    assert_eq!(assert_maybe_accounts_decoded(&maybe), Err(expected.clone()));

    let missing = [
        MaybeAccount::Missing(second),
        MaybeAccount::Present(state(first, false)),
        MaybeAccount::Missing(first),
        MaybeAccount::Missing(second),
    ];
    assert_eq!(assert_accounts_exist(&missing), Err(expected));
}

fn state(address: Address, compressed: bool) -> solana_kit_core::accounts::Account<AccountData> {
    let raw = with_data(r#"["AQI=","base64"]"#);
    let mut account = parse_json_rpc_account(address, &raw)
        .unwrap()
        .require()
        .unwrap();

    if compressed {
        account.data = AccountData::Zstd {
            compressed: vec![1, 2],
            slice: None,
        };
    }

    account
}

fn assertions(mut account: solana_kit_core::accounts::Account<AccountData>) {
    let requested = account.address;
    let single = assert_account_decoded(&account).unwrap_err();

    assert!(matches!(single, Error::AccountNotDecoded(actual) if actual == requested));

    account.data = AccountData::Zstd {
        compressed: vec![1, 2],
        slice: None,
    };
    let single = assert_account_decoded(&account).unwrap_err();

    assert!(matches!(single, Error::AccountNotDecoded(actual) if actual == requested));

    let maybe = MaybeAccount::Present(account);
    let wrapped = assert_maybe_account_decoded(&maybe).unwrap_err();

    assert!(matches!(wrapped, Error::AccountNotDecoded(actual) if actual == requested));
}
