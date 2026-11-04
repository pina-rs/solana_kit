//! Admission and schema failures for typed parsed accounts: byte, nesting and
//! duplicate-key budgets run before typed allocation; required members, exact
//! integers and addresses are never relaxed.
#![cfg(feature = "parsed-accounts")]

mod parsed_accounts_support;

use parsed_accounts_support::{LIMITS, decode, decode_with};
use solana_kit_client::{Error, Limits, parsed::ParsedAccountData};

const CLOCK: &str = r#"{
  "type": "clock",
  "info": {
    "epoch": 0,
    "epochStartTimestamp": 0,
    "leaderScheduleEpoch": 1,
    "slot": 2,
    "unixTimestamp": 3
  }
}"#;

const TOKEN_ACCOUNT: &str = r#"{
  "type": "account",
  "info": {
    "isNative": false,
    "mint": "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr",
    "owner": "6UsGbaMgchgj4wiwKKuE1v5URHdcDfEiMSM25QpesKir",
    "state": "initialized",
    "tokenAmount": {
      "amount": "0",
      "decimals": 0,
      "uiAmountString": "0"
    }
  }
}"#;

const TOKEN_ACCOUNT_WITHOUT_AMOUNT: &str = r#"{
  "type": "account",
  "info": {
    "isNative": false,
    "mint": "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr",
    "owner": "6UsGbaMgchgj4wiwKKuE1v5URHdcDfEiMSM25QpesKir",
    "state": "initialized"
  }
}"#;

const TOKEN_ACCOUNT_NULL_NATIVE: &str = r#"{
  "type": "account",
  "info": {
    "isNative": null,
    "mint": "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr",
    "owner": "6UsGbaMgchgj4wiwKKuE1v5URHdcDfEiMSM25QpesKir",
    "state": "initialized",
    "tokenAmount": {"amount": "0", "decimals": 0, "uiAmountString": "0"}
  }
}"#;

const TOKEN_ACCOUNT_UNKNOWN_STATE: &str = r#"{
  "type": "account",
  "info": {
    "isNative": false,
    "mint": "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr",
    "owner": "6UsGbaMgchgj4wiwKKuE1v5URHdcDfEiMSM25QpesKir",
    "state": "future",
    "tokenAmount": {"amount": "0", "decimals": 0, "uiAmountString": "0"}
  }
}"#;

const TOKEN_ACCOUNT_OBJECT_EXTENSIONS: &str = r#"{
  "type": "account",
  "info": {
    "extensions": {"extension": "future"},
    "isNative": false,
    "mint": "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr",
    "owner": "6UsGbaMgchgj4wiwKKuE1v5URHdcDfEiMSM25QpesKir",
    "state": "initialized",
    "tokenAmount": {"amount": "0", "decimals": 0, "uiAmountString": "0"}
  }
}"#;

const TOKEN_ACCOUNT_DUPLICATE_EXTENSION: &str = r#"{
  "type": "account",
  "info": {
    "isNative": false,
    "mint": "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr",
    "owner": "6UsGbaMgchgj4wiwKKuE1v5URHdcDfEiMSM25QpesKir",
    "state": "initialized",
    "tokenAmount": {"amount": "0", "decimals": 0, "uiAmountString": "0"},
    "extensions": [{"extension": "future", "extension": "future"}]
  }
}"#;

const MINT_WITHOUT_DECIMALS: &str = r#"{
  "type": "mint",
  "info": {"isInitialized": true, "supply": "0"}
}"#;

const MINT_NULL_SUPPLY: &str = r#"{
  "type": "mint",
  "info": {"decimals": 0, "isInitialized": true, "supply": null}
}"#;

const MINT_STRING_SUPPLY: &str = r#"{
  "type": "mint",
  "info": {"decimals": 0, "isInitialized": true, "supply": 0}
}"#;

const MINT_NEGATIVE_SUPPLY: &str = r#"{
  "type": "mint",
  "info": {"decimals": 0, "isInitialized": true, "supply": "-1"}
}"#;

const MINT_DECIMAL_SUPPLY: &str = r#"{
  "type": "mint",
  "info": {"decimals": 0, "isInitialized": true, "supply": "1.0"}
}"#;

const MINT_OVERFLOW_SUPPLY: &str = r#"{
  "type": "mint",
  "info": {
    "decimals": 0,
    "isInitialized": true,
    "supply": "18446744073709551616"
  }
}"#;

const MINT_WIDE_DECIMALS: &str = r#"{
  "type": "mint",
  "info": {"decimals": 256, "isInitialized": true, "supply": "0"}
}"#;

const MINT_FRACTIONAL_DECIMALS: &str = r#"{
  "type": "mint",
  "info": {"decimals": 6.0, "isInitialized": true, "supply": "0"}
}"#;

const MINT_OBJECT_EXTENSIONS: &str = r#"{
  "type": "mint",
  "info": {
    "decimals": 0,
    "extensions": "future",
    "isInitialized": true,
    "supply": "0"
  }
}"#;

const MINT_DUPLICATE_TYPE: &str = r#"{
  "type": "mint",
  "type": "mint",
  "info": {"decimals": 0, "isInitialized": true, "supply": "0"}
}"#;

const MINT_DUPLICATE_DECIMALS: &str = r#"{
  "type": "mint",
  "info": {"decimals": 0, "decimals": 1, "isInitialized": true, "supply": "0"}
}"#;

const STAKE_WITHOUT_META: &str = r#"{"type": "delegated", "info": {"stake": null}}"#;

const VOTE_WITHOUT_NODE: &str = r#"{
  "type": "vote",
  "info": {
    "authorizedVoters": [],
    "authorizedWithdrawer": "HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr",
    "commission": 0,
    "epochCredits": [],
    "lastTimestamp": {"slot": 0, "timestamp": 0},
    "priorVoters": [],
    "votes": []
  }
}"#;

const VOTE_NEGATIVE_ROOT: &str = r#"{
  "type": "vote",
  "info": {
    "authorizedVoters": [],
    "authorizedWithdrawer": "HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr",
    "commission": 0,
    "epochCredits": [],
    "lastTimestamp": {"slot": 0, "timestamp": 0},
    "nodePubkey": "HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr",
    "priorVoters": [],
    "rootSlot": -1,
    "votes": []
  }
}"#;

const NONCE_WITHOUT_BLOCKHASH: &str = r#"{
  "type": "initialized",
  "info": {
    "authority": "3xxDCjN8s6MgNHwdRExRLa6gHmmRTWPnUdzkbKfEgNAe",
    "feeCalculator": {"lamportsPerSignature": "0"}
  }
}"#;

const LOOKUP_TABLE_WITHOUT_ADDRESSES: &str = r#"{
  "type": "lookupTable",
  "info": {
    "deactivationSlot": "0",
    "lastExtendedSlot": "0",
    "lastExtendedSlotStartIndex": 0
  }
}"#;

const LOADER_BASE58_DATA: &str = r#"{
  "type": "programData",
  "info": {"data": ["", "base58"], "slot": 0}
}"#;

const CONFIG_WITHOUT_DATA: &str = r#"{
  "type": "validatorInfo",
  "info": {"keys": []}
}"#;

const CLOCK_WITHOUT_TIMESTAMP: &str = r#"{
  "type": "clock",
  "info": {
    "epoch": 0,
    "epochStartTimestamp": 0,
    "leaderScheduleEpoch": 0,
    "slot": 0
  }
}"#;

const CLOCK_STRING_SLOT: &str = r#"{
  "type": "clock",
  "info": {
    "epoch": 0,
    "epochStartTimestamp": 0,
    "leaderScheduleEpoch": 0,
    "slot": "1",
    "unixTimestamp": 0
  }
}"#;

const CLOCK_UNDERFLOW_TIMESTAMP: &str = r#"{
  "type": "clock",
  "info": {
    "epoch": 0,
    "epochStartTimestamp": 0,
    "leaderScheduleEpoch": 0,
    "slot": 1,
    "unixTimestamp": -9223372036854775809
  }
}"#;

const EPOCH_REWARDS_NULL_COMPLETION: &str = r#"{
  "type": "epochRewards",
  "info": {
    "distributedRewards": "0",
    "distributionCompleteBlockHeight": null,
    "totalRewards": "0"
  }
}"#;

fn invalid(program: &str, parsed: &str) -> Error {
    decode(program, parsed).expect_err("payload must be rejected")
}

fn is_schema(error: Error) -> bool {
    matches!(error, Error::Schema(_))
}

#[test]
fn the_control_literals_are_valid() {
    assert!(decode("spl-token", TOKEN_ACCOUNT).is_ok());
    assert!(decode("sysvar", CLOCK).is_ok());
}

#[test]
fn exact_byte_budget_is_checked_before_typed_projection() {
    let length = CLOCK.len();
    let exact = Limits {
        bytes: length,
        depth: 32,
    };

    assert!(decode_with("sysvar", CLOCK, exact).is_ok());
    let error = decode_with(
        "sysvar",
        CLOCK,
        Limits {
            bytes: length - 1,
            depth: 32,
        },
    )
    .unwrap_err();
    assert!(matches!(error, Error::BodyLimit), "{error:?}");
}

#[test]
fn nesting_budget_is_checked_before_typed_projection() {
    let nested = r#"{"type": "future", "info": {"a": {"b": {"c": 1}}}}"#;

    assert!(
        decode_with(
            "custom-program",
            nested,
            Limits {
                bytes: 1000,
                depth: 4,
            },
        )
        .is_ok()
    );

    let error = decode_with(
        "custom-program",
        nested,
        Limits {
            bytes: 1000,
            depth: 3,
        },
    )
    .unwrap_err();
    assert!(matches!(error, Error::DepthLimit), "{error:?}");
}

#[test]
fn duplicate_keys_are_rejected_before_any_projection() {
    for parsed in [
        MINT_DUPLICATE_TYPE,
        MINT_DUPLICATE_DECIMALS,
        TOKEN_ACCOUNT_DUPLICATE_EXTENSION,
    ] {
        assert!(is_schema(invalid("spl-token", parsed)), "{parsed}");
    }
}

#[test]
fn a_valid_but_misshaped_payload_is_a_schema_failure() {
    // RawValue guarantees JSON syntax before this boundary is reachable, so the
    // remaining admission concern is object shape and discriminator typing.
    for parsed in [
        r#"{"type": "mint", "info": []}"#,
        r#"{"info": {"decimals": 0}}"#,
        r#"{"type": "mint", "info": null}"#,
        r#"{"type": 7, "info": {}}"#,
        r#"{"type": "mint"}"#,
        "[]",
    ] {
        assert!(is_schema(invalid("spl-token", parsed)), "{parsed}");
    }
}

#[test]
fn every_required_member_is_enforced() {
    for (program, parsed) in [
        ("spl-token", TOKEN_ACCOUNT_WITHOUT_AMOUNT),
        ("spl-token", TOKEN_ACCOUNT_NULL_NATIVE),
        ("spl-token", MINT_WITHOUT_DECIMALS),
        ("spl-token", MINT_NULL_SUPPLY),
        ("stake", STAKE_WITHOUT_META),
        ("vote", VOTE_WITHOUT_NODE),
        ("nonce", NONCE_WITHOUT_BLOCKHASH),
        ("address-lookup-table", LOOKUP_TABLE_WITHOUT_ADDRESSES),
        ("bpf-upgradeable-loader", LOADER_BASE58_DATA),
        ("config", CONFIG_WITHOUT_DATA),
        ("sysvar", CLOCK_WITHOUT_TIMESTAMP),
        ("sysvar", EPOCH_REWARDS_NULL_COMPLETION),
    ] {
        assert!(is_schema(invalid(program, parsed)), "{program}: {parsed}");
    }
}

#[test]
fn addresses_must_decode_to_exactly_thirty_two_bytes() {
    for address in [
        "not-an-address",
        "1111111111111111111111111111111",
        "0OIl",
        "",
    ] {
        let mint = format!(
            r#"{{
  "type": "mint",
  "info": {{
    "decimals": 0,
    "isInitialized": true,
    "mintAuthority": "{address}",
    "supply": "0"
  }}
}}"#
        );
        assert!(is_schema(invalid("spl-token", &mint)), "{address:?}");

        let program = format!(
            r#"{{
  "type": "program",
  "info": {{"programData": "{address}"}}
}}"#
        );
        assert!(
            is_schema(invalid("bpf-upgradeable-loader", &program)),
            "{address:?}"
        );
    }
}

#[test]
fn numbers_never_pass_through_floats_or_lose_width() {
    for (program, parsed) in [
        ("spl-token", MINT_STRING_SUPPLY),
        ("spl-token", MINT_NEGATIVE_SUPPLY),
        ("spl-token", MINT_DECIMAL_SUPPLY),
        ("spl-token", MINT_OVERFLOW_SUPPLY),
        ("spl-token", MINT_WIDE_DECIMALS),
        ("spl-token", MINT_FRACTIONAL_DECIMALS),
        ("sysvar", CLOCK_STRING_SLOT),
        ("sysvar", CLOCK_UNDERFLOW_TIMESTAMP),
        ("vote", VOTE_NEGATIVE_ROOT),
    ] {
        assert!(is_schema(invalid(program, parsed)), "{program}: {parsed}");
    }
}

#[test]
fn extension_members_must_form_an_array() {
    assert!(is_schema(invalid(
        "spl-token",
        TOKEN_ACCOUNT_OBJECT_EXTENSIONS
    )));
    assert!(is_schema(invalid("spl-token", MINT_OBJECT_EXTENSIONS)));
}

#[test]
fn token_state_discriminator_is_closed() {
    assert!(is_schema(invalid("spl-token", TOKEN_ACCOUNT_UNKNOWN_STATE)));
}

#[test]
fn errors_never_echo_the_payload() {
    let marker = "MUST_NOT_APPEAR_IN_ERROR_9f3c";
    let parsed = format!(
        r#"{{
  "type": "mint",
  "info": {{
    "decimals": 0,
    "isInitialized": true,
    "marker": "{marker}"
  }}
}}"#
    );
    let error = invalid("spl-token", &parsed);
    let rendered = format!("{error} {error:?}");
    assert!(!rendered.contains(marker), "{rendered}");
}

#[test]
fn an_absent_program_stays_an_explicit_unknown() {
    let literal = r#"{"type": "thing", "info": {}}"#;
    let unknown = match decode("", literal).unwrap() {
        ParsedAccountData::UnknownProgram(unknown) => unknown,
        other => panic!("unknown expected, got {other:?}"),
    };
    assert_eq!(unknown.program, "");
    assert_eq!(unknown.type_name.as_deref(), Some("thing"));
    assert_eq!(unknown.value.get(), literal);
}

#[test]
fn admission_runs_before_an_unknown_program_can_retain_raw() {
    let numbers = (0..1000).map(|value| value.to_string()).collect::<Vec<_>>();
    let too_large = format!(r#"{{"type": "thing", "info": [{}]}}"#, numbers.join(","));
    let error = decode_with(
        "custom-program",
        &too_large,
        Limits {
            bytes: 100,
            depth: 32,
        },
    )
    .unwrap_err();
    assert!(matches!(error, Error::BodyLimit), "{error:?}");

    let nested = r#"{"type": "thing", "info": {"a": {"b": {"c": 1}}}}"#;
    let error = decode_with(
        "custom-program",
        nested,
        Limits {
            bytes: LIMITS.bytes,
            depth: 2,
        },
    )
    .unwrap_err();
    assert!(matches!(error, Error::DepthLimit), "{error:?}");
}
