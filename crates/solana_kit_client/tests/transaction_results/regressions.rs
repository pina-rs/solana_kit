use super::*;

#[test]
fn historical_missing_metadata_accepts_omitted_version_when_maximum_is_requested() {
    for (encoding, encoded) in response_encodings() {
        for maximum in [None, Some(0), Some(1)] {
            check_historical_transaction(encoding, &encoded, maximum);
            check_historical_block(encoding, &encoded, maximum, TransactionDetails::Full);
            check_historical_block(encoding, &encoded, maximum, TransactionDetails::Accounts);
        }
    }
}

#[test]
fn token_display_number_rejects_literal_synthetic_number_objects() {
    let meta = META.replace(
        "\"uiAmount\":18.446744073709551615",
        r#""uiAmount":{"$serde_json::private::Number":"1"}"#,
    );
    assert_ne!(meta, META);
    let result = full(&transaction(RAW_MESSAGE), &meta, "");
    assert!(call(Transaction(signature(), None), json!([SIGNATURE]), &result).is_err());
}

fn response_encodings() -> Vec<(TransactionEncoding, String)> {
    vec![
        (TransactionEncoding::Json, transaction(RAW_MESSAGE)),
        (
            TransactionEncoding::JsonParsed,
            transaction(&format!(
                r#"{{"accountKeys":[],"recentBlockhash":"{ADDRESS}","instructions":[]}}"#
            )),
        ),
        (TransactionEncoding::Base58, r#"["12","base58"]"#.into()),
        (TransactionEncoding::Base64, r#"["AAE=","base64"]"#.into()),
    ]
}

fn check_historical_transaction(encoding: TransactionEncoding, encoded: &str, maximum: Option<u8>) {
    let config = TransactionReadConfig {
        encoding: Some(encoding),
        max_supported_transaction_version: maximum,
        ..Default::default()
    };
    let result = full(encoded, "null", "");
    let value = call(
        Transaction(signature(), Some(config)),
        json!([SIGNATURE, config]),
        &result,
    )
    .unwrap()
    .unwrap();
    assert!(value.details.meta.is_none());
    assert!(matches!(
        value.details.version,
        solana_kit_client::Presence::Missing
    ));

    for malformed in [
        result.replace("\"meta\":null", "\"meta\":{}"),
        result.replace(",\"meta\":null", ""),
    ] {
        assert!(
            call(
                Transaction(signature(), Some(config)),
                json!([SIGNATURE, config]),
                &malformed
            )
            .is_err()
        );
    }
}

fn check_complete_block_version(
    encoding: TransactionEncoding,
    encoded: &str,
    maximum: u8,
    meta: &str,
) {
    for mode in [TransactionDetails::Full, TransactionDetails::Accounts] {
        let accounts = format!(r#"{{"signatures":["{SIGNATURE}"],"accountKeys":[]}}"#);
        let selected = if mode == TransactionDetails::Accounts {
            &accounts
        } else {
            encoded
        };

        let config = BlockConfig {
            encoding: Some(encoding),
            max_supported_transaction_version: Some(maximum),
            transaction_details: Some(mode),
            rewards: Some(false),
            ..Default::default()
        };
        let result = block(&format!(
            r#", "transactions":[{{"transaction":{selected},"meta":{meta}}}]"#
        ));
        assert!(call(Block(1, Some(config)), json!([1, config]), &result).is_err());
    }
}

fn check_historical_block(
    encoding: TransactionEncoding,
    encoded: &str,
    maximum: Option<u8>,
    mode: TransactionDetails,
) {
    let accounts = format!(r#"{{"signatures":["{SIGNATURE}"],"accountKeys":[]}}"#);
    let selected = if mode == TransactionDetails::Accounts {
        &accounts
    } else {
        encoded
    };

    let config = BlockConfig {
        encoding: Some(encoding),
        max_supported_transaction_version: maximum,
        transaction_details: Some(mode),
        rewards: Some(false),
        ..Default::default()
    };
    let result = block(&format!(
        r#", "transactions":[{{"transaction":{selected},"meta":null}}]"#
    ));
    let value = call(Block(1, Some(config)), json!([1, config]), &result)
        .unwrap()
        .unwrap();

    let transactions = match value.transactions {
        BlockTransactions::Full(values) | BlockTransactions::Accounts(values) => values,
        _ => panic!("unexpected transaction details"),
    };

    assert_eq!(transactions.len(), 1);
    assert!(transactions[0].meta.is_none());
    assert!(matches!(
        transactions[0].version,
        solana_kit_client::Presence::Missing
    ));
}

#[test]
fn complete_metadata_still_requires_the_request_selected_version() {
    for (encoding, encoded) in response_encodings() {
        check_complete_versions(encoding, &encoded);
    }
}

fn check_complete_versions(encoding: TransactionEncoding, encoded: &str) {
    let meta = r#"{"err":null,"status":{"Ok":null},"fee":1,"preBalances":[],"postBalances":[]}"#;

    for maximum in [0, 1] {
        let config = TransactionReadConfig {
            encoding: Some(encoding),
            max_supported_transaction_version: Some(maximum),
            ..Default::default()
        };
        let params = json!([SIGNATURE, config]);
        check_complete_block_version(encoding, encoded, maximum, meta);

        assert!(
            call(
                Transaction(signature(), Some(config)),
                params.clone(),
                &full(encoded, meta, "")
            )
            .is_err()
        );

        for version in ["\"legacy\"", "0", "1"] {
            let result = call(
                Transaction(signature(), Some(config)),
                params.clone(),
                &full(encoded, meta, &format!(",\"version\":{version}")),
            );
            assert_eq!(result.is_ok(), maximum == 1 || version != "1");
        }
    }
}

#[test]
fn token_display_numbers_require_real_scalars_and_preserve_extreme_exponents() {
    for field in ["preTokenBalances", "postTokenBalances"] {
        let template = META
            .replace("\"preTokenBalances\":[],", "")
            .replace("postTokenBalances", field);

        check_numeric_scalars(&template, field);
        check_invalid_numbers(&template);
        check_nullable_numbers(&template);
    }
}

fn check_numeric_scalars(template: &str, field: &str) {
    for (token, expected) in [
        ("0", "0"),
        ("-1", "-1"),
        ("1.25", "1.25"),
        ("1e400", "1e+400"),
        ("-1e-400", "-1e-400"),
    ] {
        check_numeric_scalar(template, field, token, expected);
    }
}

fn check_numeric_scalar(template: &str, field: &str, token: &str, expected: &str) {
    let meta = template.replace("18.446744073709551615,", &format!("{token},"));
    let value = call(
        Transaction(signature(), None),
        json!([SIGNATURE]),
        &full(&transaction(RAW_MESSAGE), &meta, ""),
    )
    .unwrap()
    .unwrap();
    let metadata = value.details.meta.unwrap();
    let balances = if field == "preTokenBalances" {
        metadata.pre_token_balances
    } else {
        metadata.post_token_balances
    };

    let solana_kit_client::Presence::Present(Some(balances)) = balances else {
        panic!("missing balances")
    };

    assert_eq!(
        balances[0]
            .ui_token_amount
            .ui_amount
            .as_ref()
            .unwrap()
            .to_string(),
        expected
    );
}

fn check_invalid_numbers(template: &str) {
    for invalid in [
        r#"{"$serde_json::private::Number":"1"}"#,
        r#"{"$serde_json::private::Number":"bad"}"#,
        r#"{"\u0024serde_json::private::Number":"1"}"#,
        r#"{"$serde_json::private::Number":1}"#,
        "[]",
        r#""1""#,
        "true",
        "false",
    ] {
        let meta = template.replace("18.446744073709551615,", &format!("{invalid},"));
        assert!(
            call(
                Transaction(signature(), None),
                json!([SIGNATURE]),
                &full(&transaction(RAW_MESSAGE), &meta, "")
            )
            .is_err()
        );
    }
}

fn check_nullable_numbers(template: &str) {
    let null = template.replace("18.446744073709551615,", "null,");
    assert!(
        call(
            Transaction(signature(), None),
            json!([SIGNATURE]),
            &full(&transaction(RAW_MESSAGE), &null, "")
        )
        .is_ok()
    );
    let missing = template.replace("\"uiAmount\":18.446744073709551615,", "");
    assert!(
        call(
            Transaction(signature(), None),
            json!([SIGNATURE]),
            &full(&transaction(RAW_MESSAGE), &missing, "")
        )
        .is_err()
    );
}

#[test]
fn program_defined_synthetic_number_keys_remain_exact_opaque_json() {
    let payload = r#"{ "$serde_json::private::Number":"1", "wide":1e400 }"#;
    let message = format!(
        r#"{{"accountKeys":[],"recentBlockhash":"{ADDRESS}","instructions":[{{"programId":"{ADDRESS}","program":"fixture","parsed":{payload}}}]}}"#
    );
    let config = TransactionReadConfig {
        encoding: Some(TransactionEncoding::JsonParsed),
        ..Default::default()
    };
    let value = call(
        Transaction(signature(), Some(config)),
        json!([SIGNATURE, config]),
        &full(&transaction(&message), "null", ""),
    )
    .unwrap()
    .unwrap();

    let TransactionView::JsonParsed(tx) = value.details.transaction else {
        panic!("wrong view")
    };

    let ParsedOrPartialInstruction::Parsed(instruction) = &tx.message.instructions[0] else {
        panic!("wrong instruction")
    };

    assert_eq!(instruction.parsed.get(), payload);
}
