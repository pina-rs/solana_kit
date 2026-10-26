//! Independent provider schemas cover enhanced results and their REST query spelling.

use solana_kit_helius::{Field, enhanced::*};

const COMPLETE: &str = include_str!("../fixtures/enhanced.json");

/// Provider optionality, exact amounts and recursive instructions survive round trips.
pub fn models() {
    let minimal: EnhancedTransaction = serde_json::from_str(r#"{"signature":"s"}"#)
        .expect("provider permits a signature without optional enhanced fields");
    assert_eq!(minimal.fee_payer, Field::Missing);
    assert_eq!(
        serde_json::to_string(&minimal).expect("minimal result"),
        r#"{"signature":"s"}"#
    );

    let transaction: EnhancedTransaction = serde_json::from_str(COMPLETE).expect("provider schema");
    assert_eq!(transaction.slot, Field::Value(u64::MAX));
    assert_eq!(transaction.description, Field::Null);
    let accounts = transaction.account_data.value().expect("account data");
    assert_eq!(accounts[0].native_balance_change, Field::Value(-5000));
    let balances = accounts[0]
        .token_balance_changes
        .value()
        .expect("token changes");
    assert_eq!(
        balances[0].raw_token_amount.token_amount,
        "-18446744073709551615"
    );
    assert_eq!(balances[0].raw_token_amount.decimals, 6);
    assert_eq!(
        serde_json::to_value(&transaction).expect("typed result"),
        serde_json::from_str::<serde_json::Value>(COMPLETE).expect("independent literal"),
    );

    malformed_fields();
    amount_representations();
}

fn malformed_fields() {
    for (valid, malformed) in [
        (
            "\"feePayer\": \"11111111111111111111111111111111\"",
            "\"feePayer\": 2",
        ),
        (
            "\"nativeBalanceChange\": -5000",
            "\"nativeBalanceChange\": {\"raw\": -5000}",
        ),
        (
            "\"tokenAmount\": \"-18446744073709551615\"",
            "\"tokenAmount\": -1",
        ),
        ("\"programId\": \"r\"", "\"programId\": false"),
    ] {
        let invalid = COMPLETE.replace(valid, malformed);
        assert_ne!(
            invalid, COMPLETE,
            "mutation must change actual fixture bytes"
        );
        assert!(serde_json::from_str::<EnhancedTransaction>(&invalid).is_err());
    }
}

fn amount_representations() {
    for json in [
        "9007199254740993.125",
        "\"9007199254740993.125\"",
        "-1",
        "0",
    ] {
        let amount: TokenAmount = serde_json::from_str(json).expect("provider amount union");
        assert_eq!(serde_json::to_string(&amount).expect("exact amount"), json);
    }

    for invalid in [
        "null",
        "true",
        "[]",
        "{}",
        r#"{"$serde_json::private::Number":"1"}"#,
    ] {
        assert!(serde_json::from_str::<TokenAmount>(invalid).is_err());
    }
}

/// The SDK's request builder uses hyphenated history query keys in this exact order.
pub fn history_query() {
    let json = r#"{"address":"a","beforeSignature":"before","afterSignature":"after","commitment":"confirmed","source":"JUPITER","type":"TRANSFER","limit":1,"gtSlot":2,"gteSlot":3,"ltSlot":4,"lteSlot":5,"gtTime":-4,"gteTime":-3,"ltTime":-2,"lteTime":-1,"sortOrder":"asc"}"#;
    let operation: GetTransactionsByAddressRequest = serde_json::from_str(json).expect("query");
    let request = super::service::exchange(operation, b"[]");
    let expected = [
        ("before-signature", "before"),
        ("after-signature", "after"),
        ("commitment", "confirmed"),
        ("source", "JUPITER"),
        ("type", "TRANSFER"),
        ("limit", "1"),
        ("gt-slot", "2"),
        ("gte-slot", "3"),
        ("lt-slot", "4"),
        ("lte-slot", "5"),
        ("gt-time", "-4"),
        ("gte-time", "-3"),
        ("lt-time", "-2"),
        ("lte-time", "-1"),
        ("sort-order", "asc"),
    ]
    .map(|(key, value)| (key.to_owned(), value.to_owned()));
    assert_eq!(request.query, expected);
    assert!(request.body.as_deref().is_none());

    for invalid in [
        r#"{"transactions":["s"],"commitment":"processed"}"#,
        r#"{"transactions":["s"],"commitment":"unknown"}"#,
    ] {
        assert!(serde_json::from_str::<GetTransactionsRequest>(invalid).is_err());
    }
}
