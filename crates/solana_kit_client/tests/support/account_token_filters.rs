//! Keyed token identities and exact filter request vectors.
use super::*;

#[test]
fn token_keyed_accounts_validate_all_configs_identity_owner_and_metadata() {
    let config = AccountConfig {
        read: ReadConfig {
            commitment: Some(Commitment::Confirmed),
            min_context_slot: Some(u64::MAX),
        },
        data_slice: Some(DataSlice {
            offset: 1,
            length: 2,
        }),
        encoding: Some(AccountEncoding::Base64),
    };
    let params = format!(
        r#"["{ADDRESS}",{{"programId":"{ADDRESS}"}},{{"commitment":"confirmed","minContextSlot":18446744073709551615,"encoding":"base64","dataSlice":{{"offset":1,"length":2}}}}]"#
    );
    let account = ACCOUNT.replace("AQID", "AgM=");
    let keyed = format!(r#"{{"pubkey":"{ADDRESS}","account":{account}}}"#);
    let result = contextual(&format!("[{keyed}]"));
    assert_eq!(
        call(
            TokenAccountsByOwner(a(), TokenAccountFilter::ProgramId(a()), Some(config)),
            &params,
            &result
        )
        .unwrap()
        .value[0]
            .address,
        a()
    );
    assert_eq!(
        call(
            TokenAccountsByDelegate(a(), TokenAccountFilter::ProgramId(a()), Some(config)),
            &params,
            &result
        )
        .unwrap()
        .value[0]
            .space,
        3
    );
}

#[test]
fn duplicate_or_invalid_token_identity_and_owner_are_rejected() {
    let config = AccountConfig {
        read: ReadConfig {
            commitment: Some(Commitment::Confirmed),
            min_context_slot: Some(u64::MAX),
        },
        data_slice: Some(DataSlice {
            offset: 1,
            length: 2,
        }),
        encoding: Some(AccountEncoding::Base64),
    };
    let params = format!(
        r#"["{ADDRESS}",{{"programId":"{ADDRESS}"}},{{"commitment":"confirmed","minContextSlot":18446744073709551615,"encoding":"base64","dataSlice":{{"offset":1,"length":2}}}}]"#
    );
    let account = ACCOUNT.replace("AQID", "AgM=");
    let keyed = format!(r#"{{"pubkey":"{ADDRESS}","account":{account}}}"#);
    for value in [
        format!("[{keyed},{keyed}]"),
        format!(
            "[{}]",
            keyed.replace(
                r#""pubkey":"11111111111111111111111111111111""#,
                r#""pubkey":"bad""#
            )
        ),
        format!(
            "[{}]",
            keyed.replace(
                r#""owner":"11111111111111111111111111111111""#,
                r#""owner":"4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi""#
            )
        ),
    ] {
        assert!(
            call(
                TokenAccountsByOwner(a(), TokenAccountFilter::ProgramId(a()), Some(config)),
                &params,
                &contextual(&value)
            )
            .is_err()
        );
    }
}

#[test]
fn filter_representations_and_empty_config_preserve_request_intent() {
    for (encoding, expected) in [
        (
            FilterEncoding::Base58,
            r#"{"memcmp":{"offset":7,"encoding":"base58","bytes":"Ldp"}}"#,
        ),
        (
            FilterEncoding::Base64,
            r#"{"memcmp":{"offset":7,"encoding":"base64","bytes":"AQID"}}"#,
        ),
        (
            FilterEncoding::Bytes,
            r#"{"memcmp":{"offset":7,"encoding":"bytes","bytes":[1,2,3]}}"#,
        ),
    ] {
        let method = ProgramAccounts(
            a(),
            Some(ProgramAccountsConfig {
                filters: Some(vec![Filter::Memcmp(
                    Memcmp::new(7, vec![1, 2, 3], encoding).unwrap(),
                )]),
                with_context: Some(false),
                ..Default::default()
            }),
        );
        let params = format!(r#"["{ADDRESS}",{{"filters":[{expected}],"withContext":false}}]"#);
        assert!(call(method, &params, "[]").is_ok());
    }
    assert!(
        call(
            ProgramAccounts(
                a(),
                Some(ProgramAccountsConfig {
                    filters: Some(vec![]),
                    ..Default::default()
                })
            ),
            &format!(r#"["{ADDRESS}",{{"filters":[]}}]"#),
            "[]"
        )
        .is_ok()
    );
}

#[test]
fn excess_program_filters_fail_before_transport_access() {
    let method = ProgramAccounts(
        a(),
        Some(ProgramAccountsConfig {
            filters: Some(vec![Filter::DataSize(0); 5]),
            ..Default::default()
        }),
    );
    assert!(call(method, "unreachable", "[]").is_err());
}
