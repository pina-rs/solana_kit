//! Wallet v1 fixtures exercise real REST requests and typed provider result contracts.

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, value::RawValue};
use solana_kit_helius::{Config, SensitiveString, service::*, wallet::*};

#[derive(Deserialize)]
struct Fixture {
    name: String,
    verb: String,
    path: Vec<String>,
    query: Vec<(String, String)>,
    params: Box<RawValue>,
    result: Box<RawValue>,
}

fn contract<OperationType: Operation + DeserializeOwned>(fixture: &Fixture)
where
    OperationType::Response: Serialize,
{
    let operation: OperationType =
        serde_json::from_str(fixture.params.get()).expect("literal wallet parameters");
    assert_eq!(
        serde_json::to_value(&operation).expect("typed parameters"),
        serde_json::from_str::<Value>(fixture.params.get()).expect("literal parameters"),
    );

    let decoded = operation
        .decode(fixture.result.get().as_bytes())
        .expect("typed wallet result");
    assert_eq!(
        serde_json::to_value(decoded).expect("typed result"),
        serde_json::from_str::<Value>(fixture.result.get()).expect("literal result"),
    );

    let request = super::service::exchange(operation, fixture.result.get().as_bytes());
    assert_eq!(request.service, Service::Wallet);
    assert_eq!(request.path_segments, fixture.path);
    assert_eq!(request.query, fixture.query);

    let verb = match fixture.verb.as_str() {
        "GET" => Verb::Get,
        "POST" => Verb::Post,
        other => panic!("unexpected fixture verb {other}"),
    };

    assert_eq!(request.method, verb);

    if verb == Verb::Post {
        assert_eq!(request.body.content_type(), Some("application/json"));
        assert_eq!(
            serde_json::from_slice::<Value>(request.body.as_deref().expect("POST body"))
                .expect("JSON body"),
            serde_json::from_str::<Value>(fixture.params.get()).expect("literal body"),
        );
    } else {
        assert!(request.body.as_deref().is_none());
    }
}

type WireContract = (&'static str, fn(&Fixture));

const CONTRACTS: &[WireContract] = &[
    ("identity", contract::<GetIdentityRequest>),
    ("batch-identity", contract::<GetBatchIdentityRequest>),
    ("balances", contract::<GetBalancesRequest>),
    ("history", contract::<GetHistoryRequest>),
    ("transfers", contract::<GetTransfersRequest>),
    ("balance-at", contract::<GetBalanceAtRequest>),
    ("funded-by", contract::<GetFundedByRequest>),
];

/// Each current wallet endpoint retains its approved service, verb, path, query and result shape.
pub fn vectors() {
    let fixtures: Vec<Fixture> =
        serde_json::from_str(include_str!("../fixtures/wallet-methods.json"))
            .expect("literal wallet catalog");
    assert_eq!(fixtures.len(), 7);
    assert_eq!(fixtures.len(), CONTRACTS.len());

    for ((name, execute), fixture) in CONTRACTS.iter().zip(&fixtures) {
        assert_eq!(*name, fixture.name);
        execute(fixture);
    }

    let config = Config::new(SensitiveString::new("fixture")).expect("caller-owned key");
    assert_eq!(config.wallet_origin(), "https://api.helius.xyz");
    malformed_fields(&fixtures);
}

fn malformed_fields(fixtures: &[Fixture]) {
    let identity: GetIdentityRequest =
        serde_json::from_str(fixtures[0].params.get()).expect("identity request");
    assert!(identity.decode(br#"{"socials":{}}"#).is_err());
    let wrong_wallet = fixtures[0].result.get().replace("a/b?c", "other");
    assert!(identity.decode(wrong_wallet.as_bytes()).is_err());

    let balances: GetBalancesRequest =
        serde_json::from_str(fixtures[2].params.get()).expect("balance request");
    assert!(
        balances
            .decode(br#"{"nativeBalance":1,"tokens":[]}"#)
            .is_err()
    );
    let bad_token = fixtures[2].result.get().replace("spl-token", "other");
    assert!(balances.decode(bad_token.as_bytes()).is_err());
    let missing_price = fixtures[2]
        .result
        .get()
        .replace("\"pricePerToken\": null,", "");
    assert_ne!(missing_price, fixtures[2].result.get());
    assert!(balances.decode(missing_price.as_bytes()).is_err());

    let history: GetHistoryRequest =
        serde_json::from_str(fixtures[3].params.get()).expect("history request");
    assert!(history.decode(b"[]").is_err());
    let missing_timestamp = fixtures[3].result.get().replace("\"timestamp\": null,", "");
    assert_ne!(missing_timestamp, fixtures[3].result.get());
    assert!(history.decode(missing_timestamp.as_bytes()).is_err());

    let transfers: GetTransfersRequest =
        serde_json::from_str(fixtures[4].params.get()).expect("transfer request");
    assert!(transfers.decode(b"[]").is_err());
    let invalid_direction = fixtures[4].result.get().replace("\"out\"", "\"sideways\"");
    assert!(transfers.decode(invalid_direction.as_bytes()).is_err());

    let funded: GetFundedByRequest =
        serde_json::from_str(fixtures[6].params.get()).expect("funding request");
    assert!(funded.decode(br#"{"transactions":[]}"#).is_err());
}

/// One selector is represented in Rust and on the wire; unused selectors are never submitted.
pub fn historical_points() {
    for (selector, echo, key, value) in [
        (
            BalanceAtPoint::Time(-1),
            r#"{"time":-1,"slot":null,"datetime":null}"#,
            "time",
            "-1",
        ),
        (
            BalanceAtPoint::Datetime("2025-01-10 19:20:00".into()),
            r#"{"time":1736536800,"slot":null,"datetime":"2025-01-10 19:20:00"}"#,
            "datetime",
            "2025-01-10 19:20:00",
        ),
        (
            BalanceAtPoint::Slot(u64::MAX),
            r#"{"time":null,"slot":18446744073709551615,"datetime":null}"#,
            "slot",
            "18446744073709551615",
        ),
    ] {
        historical_point(selector, echo, key, value);
    }

    for selectors in [
        "",
        r#", "time": null"#,
        r#", "time":1,"slot":2"#,
        r#", "datetime":"2025-01-10","time":1"#,
        r#", "datetime":"2025-01-10","slot":1"#,
        r#", "slot":-1"#,
        r#", "time":true"#,
        r#", "slot":18446744073709551616"#,
    ] {
        let json = format!(r#"{{"wallet":"a","mint":"m"{selectors}}}"#);
        assert!(
            serde_json::from_str::<GetBalanceAtRequest>(&json).is_err(),
            "accepted {json}"
        );
    }
}

fn historical_point(point: BalanceAtPoint, echo: &str, key: &str, value: &str) {
    let operation = GetBalanceAtRequest {
        wallet: "a/b?c".into(),
        mint: "m&n".into(),
        point,
    };
    let response = format!(
        r#"{{"wallet":"a/b?c","mint":"m&n","isNative":false,"balance":"0","balanceRaw":"0","decimals":6,"requested":{echo},"asOf":null}}"#
    );
    let encoded = serde_json::to_string(&operation).expect("single selector parameters");
    assert_eq!(
        serde_json::from_str::<GetBalanceAtRequest>(&encoded).expect("decode selector"),
        operation
    );

    for invalid in [
        response.replace("a/b?c", "other"),
        response.replace("m&n", "other"),
        response.replace(r#","asOf":null"#, ""),
        response.replace("\"requested\":", "\"otherRequested\":"),
        response.replace(echo, r#"{"time":2,"slot":3,"datetime":"other"}"#),
    ] {
        assert_ne!(
            invalid, response,
            "every negative control must change its input"
        );
        assert!(
            operation.decode(invalid.as_bytes()).is_err(),
            "accepted invalid echo"
        );
    }

    let request = super::service::exchange(operation, response.as_bytes());
    assert_eq!(request.service, Service::Wallet);
    assert_eq!(
        request.path_segments,
        ["v1", "wallet", "a/b?c", "balance-at"]
    );
    assert_eq!(
        request.query,
        [("mint".into(), "m&n".into()), (key.into(), value.into())]
    );
    assert!(request.body.as_deref().is_none());
    let display = format!("{request:?}");
    assert!(!display.contains("m&n"));
    assert!(!display.contains("a/b?c"));
}
