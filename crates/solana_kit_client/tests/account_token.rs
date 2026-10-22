//! Literal account/token fixtures through the real public client.
#[path = "support/account_token.rs"]
mod support;
use solana_kit_client::methods::{Balance, LatestBlockhash};
use solana_kit_client::methods::{Commitment, ReadConfig, accounts::*, tokens::*};
use solana_kit_core::{Address, Hash, accounts::MaybeAccount};
use support::*;
#[test]
fn every_standard_operation_executes_with_literal_omitted_params() {
    assert_eq!(
        call(
            Balance(a(), None),
            &format!(r#"["{ADDRESS}"]"#),
            &contextual("0")
        )
        .unwrap()
        .value,
        0
    );
    assert_eq!(
        call(
            LatestBlockhash(None),
            "[]",
            &contextual(&format!(
                r#"{{"blockhash":"{ADDRESS}","lastValidBlockHeight":18446744073709551615}}"#
            ))
        )
        .unwrap()
        .value
        .last_valid_block_height,
        u64::MAX
    );
    assert_eq!(
        call(
            MinimumBalanceForRentExemption(u64::MAX, None),
            "[18446744073709551615]",
            "18446744073709551615"
        )
        .unwrap(),
        u64::MAX
    );
    assert!(
        call(
            BlockhashValid(Hash::from_bytes([0; 32]), None),
            &format!(r#"["{ADDRESS}"]"#),
            &contextual("true")
        )
        .unwrap()
        .value
    );
    let binary = ACCOUNT.replace(r#"["AQID","base64"]"#, r#""Ldp""#);
    assert!(
        call(
            AccountInfoWithConfig(a(), None),
            &format!(r#"["{ADDRESS}"]"#),
            &contextual(&binary)
        )
        .is_ok()
    );
}

#[test]
fn token_operations_accept_omitted_configs() {
    for method in ["owner", "delegate"] {
        let params = format!(r#"["{ADDRESS}",{{"mint":"{ADDRESS}"}}]"#);
        let response = contextual("[]");

        if method == "owner" {
            assert!(
                call(
                    TokenAccountsByOwner(a(), TokenAccountFilter::Mint(a()), None),
                    &params,
                    &response
                )
                .unwrap()
                .value
                .is_empty()
            );
        } else {
            assert!(
                call(
                    TokenAccountsByDelegate(a(), TokenAccountFilter::Mint(a()), None),
                    &params,
                    &response
                )
                .unwrap()
                .value
                .is_empty()
            );
        }
    }
}

#[test]
fn token_amount_operations_preserve_full_width_raw_amount() {
    assert_eq!(
        call(
            TokenAccountBalance(a(), None),
            &format!(r#"["{ADDRESS}"]"#),
            &contextual(AMOUNT)
        )
        .unwrap()
        .value
        .amount
        .0,
        u64::MAX
    );

    assert_eq!(
        call(
            TokenSupply(a(), None),
            &format!(r#"["{ADDRESS}"]"#),
            &contextual(AMOUNT)
        )
        .unwrap()
        .value
        .decimals,
        255
    );
    let largest = format!(
        r#"[{{"address":"{ADDRESS}","amount":"0","decimals":0,"uiAmount":0.0,"uiAmountString":"0"}}]"#
    );
    assert_eq!(
        call(
            TokenLargestAccounts(a(), None),
            &format!(r#"["{ADDRESS}"]"#),
            &contextual(&largest)
        )
        .unwrap()
        .value[0]
            .balance
            .amount
            .0,
        0
    );
    let result = call(
        ProgramAccounts(a(), None),
        &format!(r#"["{ADDRESS}"]"#),
        "[]",
    )
    .unwrap();
    let ProgramAccountsResponse::Accounts(value) = result else {
        panic!()
    };
    assert!(value.is_empty());
    assert!(
        call(MultipleAccounts(vec![], None), "[[]]", &contextual("[]"))
            .unwrap()
            .value
            .is_empty()
    );
}

#[test]
fn complete_config_and_filters_have_exact_json_and_context_choice() {
    let read = ReadConfig {
        commitment: Some(Commitment::Finalized),
        min_context_slot: Some(u64::MAX),
    };
    let config = AccountConfig {
        read,
        encoding: Some(AccountEncoding::Base64),
        data_slice: Some(DataSlice {
            offset: 1,
            length: 2,
        }),
    };
    let options = r#"{"commitment":"finalized","minContextSlot":18446744073709551615,"encoding":"base64","dataSlice":{"offset":1,"length":2}}"#;
    let sliced = ACCOUNT.replace("AQID", "AgM=");
    assert!(
        call(
            AccountInfoWithConfig(a(), Some(config)),
            &format!(r#"["{ADDRESS}",{options}]"#),
            &contextual(&sliced)
        )
        .is_ok()
    );
    let value = call(
        MultipleAccounts(vec![a(), Address::from_bytes([1; 32])], Some(config)),
        &format!(r#"[["{ADDRESS}","4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi"],{options}]"#),
        &contextual(&format!("[{sliced},null]")),
    )
    .unwrap();
    assert_eq!(value.context.slot, u64::MAX);
    assert!(
        matches!(value.value[1], MaybeAccount::Missing(address) if address == Address::from_bytes([1;32]))
    );
}

#[test]
fn program_filter_config_selects_contextual_result() {
    let config = AccountConfig {
        read: ReadConfig {
            commitment: Some(Commitment::Finalized),
            min_context_slot: Some(u64::MAX),
        },
        encoding: Some(AccountEncoding::Base64),
        data_slice: Some(DataSlice {
            offset: 1,
            length: 2,
        }),
    };
    let sliced = ACCOUNT.replace("AQID", "AgM=");
    let filters = vec![
        Filter::DataSize(3),
        Filter::Memcmp(Memcmp::new(0, vec![1, 2, 3], FilterEncoding::Base64).unwrap()),
        Filter::TokenAccountState,
    ];
    let options = r#"{"commitment":"finalized","minContextSlot":18446744073709551615,"encoding":"base64","dataSlice":{"offset":1,"length":2},"filters":[{"dataSize":3},{"memcmp":{"offset":0,"encoding":"base64","bytes":"AQID"}},"tokenAccountState"],"withContext":true}"#;
    let method = ProgramAccounts(
        a(),
        Some(ProgramAccountsConfig {
            account: config,
            filters: Some(filters),
            with_context: Some(true),
        }),
    );
    let keyed = format!(r#"[{{"pubkey":"{ADDRESS}","account":{sliced}}}]"#);
    let result = call(
        method,
        &format!(r#"["{ADDRESS}",{options}]"#),
        &contextual(&keyed),
    )
    .unwrap();
    let ProgramAccountsResponse::Contextual(value) = result else {
        panic!()
    };
    assert_eq!(value.value.len(), 1);
}

#[test]
fn commitment_only_operations_encode_exact_config() {
    let read = ReadConfig {
        commitment: Some(Commitment::Finalized),
        min_context_slot: Some(u64::MAX),
    };
    let commitment = Some(CommitmentConfig {
        commitment: Some(Commitment::Confirmed),
    });
    let params = format!(r#"["{ADDRESS}",{{"commitment":"confirmed"}}]"#);
    assert!(call(TokenSupply(a(), commitment), &params, &contextual(AMOUNT)).is_ok());
    assert!(
        call(
            TokenAccountBalance(a(), commitment),
            &params,
            &contextual(AMOUNT)
        )
        .is_ok()
    );
    assert!(
        call(
            TokenLargestAccounts(a(), commitment),
            &params,
            &contextual("[]")
        )
        .is_ok()
    );
    assert!(
        call(
            MinimumBalanceForRentExemption(3, commitment),
            r#"[3,{"commitment":"confirmed"}]"#,
            "0"
        )
        .is_ok()
    );
    assert!(
        call(
            BlockhashValid(Hash::from_bytes([0; 32]), Some(read)),
            &format!(
                r#"["{ADDRESS}",{{"commitment":"finalized","minContextSlot":18446744073709551615}}]"#
            ),
            &contextual("false")
        )
        .is_ok()
    );
}

#[test]
fn encoding_and_slice_boundaries_are_explicit_and_strict() {
    for (encoding, wire) in [
        (AccountEncoding::Base58, r#"["Ldp","base58"]"#),
        (AccountEncoding::Base64, r#"["AQID","base64"]"#),
        (AccountEncoding::JsonParsed, r#"["AQID","base64"]"#),
    ] {
        let result = contextual(&ACCOUNT.replace(r#"["AQID","base64"]"#, wire));
        let config = AccountConfig {
            encoding: Some(encoding),
            ..Default::default()
        };
        let params = serde_json::to_string(&AccountInfoWithConfig(a(), Some(config))).unwrap();
        let value = present(
            call(AccountInfoWithConfig(a(), Some(config)), &params, &result)
                .unwrap()
                .value,
        );
        let AccountData::Bytes { bytes, slice } = value.data else {
            panic!()
        };
        assert_eq!(bytes, [1, 2, 3]);
        assert_eq!(slice, None);
    }
}

#[test]
fn parsed_payload_retains_exact_future_program_json() {
    let parsed = r#"{"program":"future","parsed":{ "future":1e400 },"space":3}"#;
    let config = AccountConfig {
        encoding: Some(AccountEncoding::JsonParsed),
        ..Default::default()
    };
    let params = serde_json::to_string(&AccountInfoWithConfig(a(), Some(config))).unwrap();
    let result = contextual(&ACCOUNT.replace(r#"["AQID","base64"]"#, parsed));
    let MaybeAccount::Present(value) =
        call(AccountInfoWithConfig(a(), Some(config)), &params, &result)
            .unwrap()
            .value
    else {
        panic!()
    };

    let AccountData::Parsed(value) = value.data else {
        panic!()
    };
    assert_eq!(value.parsed.get(), r#"{ "future":1e400 }"#);
}

#[test]
fn compressed_bytes_remain_explicit() {
    let config = AccountConfig {
        encoding: Some(AccountEncoding::Base64Zstd),
        ..Default::default()
    };
    let params = serde_json::to_string(&AccountInfoWithConfig(a(), Some(config))).unwrap();
    let value = call(
        AccountInfoWithConfig(a(), Some(config)),
        &params,
        &contextual(&ACCOUNT.replace("base64", "base64+zstd")),
    )
    .unwrap();
    let AccountData::Zstd { compressed, .. } = present(value.value).data else {
        panic!()
    };
    assert_eq!(compressed, [1, 2, 3]);
}

#[test]
fn sliced_bytes_match_available_range() {
    const PAST_END: u64 = 9;

    for (offset, length, bytes) in [
        (0, 0, ""),
        (2, PAST_END, "Aw=="),
        (3, PAST_END, ""),
        (u64::MAX, 0, ""),
    ] {
        let config = AccountConfig {
            data_slice: Some(DataSlice { offset, length }),
            ..self::config()
        };
        let params = serde_json::to_string(&AccountInfoWithConfig(a(), Some(config))).unwrap();
        assert!(
            call(
                AccountInfoWithConfig(a(), Some(config)),
                &params,
                &contextual(&ACCOUNT.replace("AQID", bytes))
            )
            .is_ok()
        );
    }
}

#[test]
fn malformed_account_shapes_never_become_present_or_empty_success() {
    for body in [
        ACCOUNT.replace("AQID", "AB=="),
        ACCOUNT.replace("AQID", "AQI="),
        ACCOUNT.replace("base64", "base58"),
        ACCOUNT.replace(ADDRESS, "bad"),
        ACCOUNT.replace(r#""space":3,"#, ""),
        ACCOUNT.replace(r#""data":["AQID","base64"]"#, r#""data":null"#),
        ACCOUNT.replace(r#""rentEpoch":18446744073709551615"#, r#""rentEpoch":null"#),
    ] {
        assert!(
            call(
                AccountInfoWithConfig(a(), Some(config())),
                &account_params(),
                &contextual(&body)
            )
            .is_err(),
            "{body}"
        );
    }
}

#[test]
fn missing_and_null_account_contexts_are_distinct() {
    for result in [
        r#"{"value":null}"#,
        r#"{"context":{"slot":0}}"#,
        r#"{"context":null,"value":null}"#,
        r#"{"context":{"slot":0},"value":{}}"#,
    ] {
        assert!(
            call(
                AccountInfoWithConfig(a(), Some(config())),
                &account_params(),
                result
            )
            .is_err()
        );
    }
    let result = call(
        AccountInfoWithConfig(a(), Some(config())),
        &account_params(),
        &contextual("null"),
    )
    .unwrap();
    let MaybeAccount::Missing(address) = result.value else {
        panic!()
    };
    assert_eq!(address, a());
}

#[test]
fn response_cardinality_and_context_choice_are_checked() {
    for value in ["[]", "[null,null]"] {
        assert!(
            call(
                MultipleAccounts(vec![a()], Some(config())),
                &format!(r#"[["{ADDRESS}"],{{"encoding":"base64"}}]"#),
                &contextual(value)
            )
            .is_err()
        );
    }

    assert!(
        call(
            ProgramAccounts(a(), None),
            &format!(r#"["{ADDRESS}"]"#),
            &contextual("[]")
        )
        .is_err()
    );
    let method = ProgramAccounts(
        a(),
        Some(ProgramAccountsConfig {
            with_context: Some(true),
            ..Default::default()
        }),
    );
    assert!(
        call(
            method,
            &format!(r#"["{ADDRESS}",{{"withContext":true}}]"#),
            "[]"
        )
        .is_err()
    );
}

#[test]
fn token_integer_precision_and_required_fields_never_use_display_float() {
    let params = format!(r#"["{ADDRESS}"]"#);

    for amount in ["18446744073709551616", "-1", "1.0", "1e2", "", "+1"] {
        let value = AMOUNT.replace("18446744073709551615", amount);
        assert!(call(TokenSupply(a(), None), &params, &contextual(&value)).is_err());
    }
    for value in [
        AMOUNT.replace(r#""amount":"18446744073709551615""#, r#""amount":1"#),
        AMOUNT.replace(r#""decimals":255"#, r#""decimals":256"#),
        AMOUNT.replace(r#""uiAmount":null,"#, ""),
        AMOUNT.replace(r#""uiAmount":null"#, r#""uiAmount":1e400"#),
        AMOUNT.replace(r#", "x":0"#, ""),
        AMOUNT.replace(
            r#""uiAmountString":"0.0000000000000000000000000000000000000001""#,
            r#""uiAmountString":null"#,
        ),
    ] {
        if value == AMOUNT {
            continue;
        }

        assert!(
            call(TokenSupply(a(), None), &params, &contextual(&value)).is_err(),
            "{value}"
        );
    }
}

#[test]
fn invalid_request_ranges_and_filters_fail_before_transport() {
    assert!(Memcmp::new(0, vec![0; 129], FilterEncoding::Base58).is_err());
    assert!(Memcmp::new(u64::MAX, vec![0], FilterEncoding::Bytes).is_err());
    for encoding in [
        FilterEncoding::Base58,
        FilterEncoding::Base64,
        FilterEncoding::Bytes,
    ] {
        assert!(Memcmp::new(0, vec![0; 128], encoding).is_ok());
    }
    for config in [
        AccountConfig {
            encoding: Some(AccountEncoding::JsonParsed),
            data_slice: Some(DataSlice {
                offset: 0,
                length: 1,
            }),
            ..Default::default()
        },
        AccountConfig {
            data_slice: Some(DataSlice {
                offset: u64::MAX,
                length: 1,
            }),
            ..Default::default()
        },
    ] {
        assert!(serde_json::to_string(&AccountInfoWithConfig(a(), Some(config))).is_err());
    }
    assert!(serde_json::to_string(&MultipleAccounts(vec![a(); 101], None)).is_err());
}

#[test]
fn multiple_accounts_omitted_encoding_uses_official_base64_default() {
    let result = contextual(&format!("[{ACCOUNT}]"));
    assert!(
        call(
            MultipleAccounts(vec![a()], None),
            &format!(r#"[["{ADDRESS}"]]"#),
            &result
        )
        .is_ok()
    );
    assert!(
        call(
            MultipleAccounts(vec![a()], Some(AccountConfig::default())),
            &format!(r#"[["{ADDRESS}"],{{}}]"#),
            &result
        )
        .is_ok()
    );
}

#[path = "support/account_token_filters.rs"]
mod account_token_filters;

#[path = "support/account_token_protocol.rs"]
mod account_token_protocol;
