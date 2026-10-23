//! Standalone source contracts remain separate from strict node RPC validation.
use solana_kit_client::{
    Error,
    accounts::{
        assert_account_decoded, assert_maybe_account_decoded, parse_base_account,
        parse_base64_rpc_account, parse_json_rpc_account,
    },
    methods::accounts::AccountData,
};
use solana_kit_core::{
    Address,
    accounts::{Account, MaybeAccount},
};

const HEADER: &str =
    r#"{"executable":false,"lamports":1,"owner":"11111111111111111111111111111111","space":3}"#;
const HEX_VALUE: u64 = 16;

fn numeric(value: &str) -> String {
    HEADER
        .replace("\"lamports\":1", &format!("\"lamports\":{value}"))
        .replace("\"space\":3", &format!("\"space\":{value}"))
}

pub fn metadata_only_parser_returns_actual_external_header_fields() {
    let header = parse_base_account(HEADER).unwrap();
    assert!(!header.executable);
    assert_eq!(header.lamports, 1);
    assert_eq!(header.space, 3);
    assert_eq!(header.owner, Address::from_bytes([0; 32]));
    assert_eq!(header.rent_epoch, solana_kit_client::Presence::Missing);
    assert!(parse_base_account(&HEADER.replace("\"space\":3", "\"space\":null")).is_err());
}

pub fn standalone_numeric_strings_follow_source_syntax_with_checked_u64_bounds() {
    for (value, expected) in [
        ("1", 1),
        ("+1", 1),
        ("01", 1),
        (" 1", 1),
        ("1 ", 1),
        ("\t1\n", 1),
        ("0x10", HEX_VALUE),
        ("0Xff", 255),
        ("+0x10", HEX_VALUE),
        ("\u{feff}1\u{feff}", 1),
        ("18446744073709551615", u64::MAX),
    ] {
        let json = serde_json::to_string(value).unwrap();
        let header = parse_base_account(&numeric(&json)).unwrap();
        assert_eq!(
            (header.lamports, header.space),
            (expected, expected),
            "{value}"
        );
        let raw = numeric(&json).replace('}', ",\"data\":[\"AQID\",\"base64\"]}");
        let account = parse_base64_rpc_account(Address::from_bytes([0; 32]), &raw)
            .unwrap()
            .require()
            .unwrap();
        assert_eq!((account.lamports, account.space), (expected, expected));
        assert_eq!(account.data, [1, 2, 3]);
    }

    for value in [
        "-0",
        "-1",
        "1.0",
        "1e2",
        "",
        "0x",
        "1_0",
        "++1",
        "+ 1",
        "\u{0085}1\u{0085}",
        "18446744073709551616",
        "0x10000000000000000",
    ] {
        assert!(
            parse_base_account(&numeric(&serde_json::to_string(value).unwrap())).is_err(),
            "{value}"
        );
    }

    for value in ["-1", "1.5", "true", "null", "18446744073709551616"] {
        assert!(parse_base_account(&numeric(value)).is_err(), "{value}");
    }
}

pub fn optional_parsed_metadata_keeps_exact_payload_and_available_facts() {
    for metadata in [
        "",
        ",\"program\":\"future\"",
        ",\"space\":3",
        ",\"program\":\"future\",\"space\":3",
        ",\"program\":null",
    ] {
        let payload = r#"{ "info": {"x":7}, "type":"custom" }"#;
        let raw = HEADER.replace(
            '}',
            &format!(",\"data\":{{\"parsed\":{payload}{metadata}}}}}"),
        );
        let account = parse_json_rpc_account(Address::from_bytes([0; 32]), &raw)
            .unwrap()
            .require()
            .unwrap();
        let parsed = assert_account_decoded(&account).unwrap();
        assert_eq!(parsed.parsed.get(), payload);
        assert_eq!(
            parsed.program,
            if metadata.contains("future") {
                "future"
            } else {
                ""
            }
        );
        assert_eq!(parsed.space, metadata.contains("space").then_some(3));
        let value: serde_json::Value = serde_json::from_str(parsed.parsed.get()).unwrap();
        assert_eq!(value["info"]["x"], 7);
        assert_eq!(value["type"], "custom");
    }
}

fn assert_encoded_identity(account: Account<AccountData>) {
    let address = account.address;
    let single = assert_account_decoded(&account).unwrap_err();

    assert!(matches!(single, Error::AccountNotDecoded(actual) if actual == address));
    let maybe = assert_maybe_account_decoded(&MaybeAccount::Present(account)).unwrap_err();

    assert!(matches!(maybe, Error::AccountNotDecoded(actual) if actual == address));
}

pub fn each_single_decoded_assertion_preserves_its_exact_address() {
    for byte in [1, 2] {
        let address = Address::from_bytes([byte; 32]);
        let raw = HEADER.replace('}', ",\"data\":[\"AQID\",\"base64\"]}");
        let mut account = parse_json_rpc_account(address, &raw)
            .unwrap()
            .require()
            .unwrap();

        let bytes = parse_json_rpc_account(address, &raw)
            .unwrap()
            .require()
            .unwrap();
        assert_encoded_identity(bytes);
        account.data = AccountData::Zstd {
            compressed: vec![],
            slice: None,
        };
        assert_encoded_identity(account);

        assert!(
            assert_maybe_account_decoded(&MaybeAccount::Missing(address))
                .unwrap()
                .is_none()
        );
    }
}
