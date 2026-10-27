#![cfg(feature = "hermes")]
//! Exact model values are independently captured from the pinned Dart implementation.
use solana_kit_client::rest::{Budgets, JsonError};
use solana_kit_core::fixed_points::BigInt;
use solana_kit_pyth::hermes::*;

const RESPONSE: &[u8] = include_bytes!("fixtures/hermes-update.json");
const PRICE: &str = "-1234567890123456789012345678901234567890";
const CONFIDENCE: &str = "340282366920938463463374607431768211456";
const SLOT: u64 = 9_007_199_254_740_993;
const PREVIOUS_TIME: i64 = -123;
const PROOF_TIME: i64 = 1_700_000_000;
const EMA_PRICE: i64 = 42;
const EMA_CONFIDENCE: i64 = -7;
const EXPONENT: i32 = -8;

/// Compares every parsed record and integer with the actual Dart reference receipt.
#[test]
fn components_beyond_128_bits_and_full_timestamps_remain_exact() {
    let update = decode_price_update(RESPONSE, Budgets::default()).unwrap();
    let feed = &update.parsed.as_ref().unwrap()[0];
    assert_eq!(update.binary.encoding, Encoding::Hex);
    assert_eq!(update.binary.data, ["504e4155"]);
    assert_eq!(feed.id, "feed-one");
    assert_eq!(feed.price.price.to_string(), PRICE);
    assert_eq!(feed.price.confidence.to_string(), CONFIDENCE);
    assert_eq!(feed.price.exponent, EXPONENT);
    assert_eq!(feed.price.publish_time, i64::MAX);
    assert_eq!(feed.ema_price.price, BigInt::from(EMA_PRICE));
    assert_eq!(feed.ema_price.confidence, BigInt::from(EMA_CONFIDENCE));
    assert_eq!(feed.ema_price.exponent, EXPONENT);
    assert_eq!(feed.ema_price.publish_time, i64::MAX);
    assert_eq!(
        feed.metadata,
        Some(PriceMetadata {
            slot: Some(SLOT),
            prev_publish_time: Some(PREVIOUS_TIME),
            proof_available_time: Some(PROOF_TIME),
        })
    );
}

/// Serialization keeps big components textual and recreates the complete record.
#[test]
fn serialization_uses_decimal_strings_for_numeric_and_string_inputs() {
    let update = decode_price_update(RESPONSE, Budgets::default()).unwrap();
    let encoded = Budgets::default().serialize(&update).unwrap();
    let json: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(json["parsed"][0]["price"]["price"], PRICE);
    assert_eq!(json["parsed"][0]["price"]["conf"], CONFIDENCE);
    assert_eq!(
        json["parsed"][0]["ema_price"]["price"],
        EMA_PRICE.to_string()
    );
    assert_eq!(
        json["parsed"][0]["ema_price"]["conf"],
        EMA_CONFIDENCE.to_string()
    );
    assert_eq!(
        decode_price_update(&encoded, Budgets::default()).unwrap(),
        update
    );
}

/// No parsed result, a null result and an explicitly empty list keep their intended meaning.
#[test]
fn optional_records_and_null_binary_collections_preserve_presence() {
    let absent = br#"{"binary":{"encoding":"hex"}}"#;
    let null = br#"{"binary":{"encoding":"hex","data":null},"parsed":null}"#;
    let empty = br#"{"binary":{"encoding":"hex","data":[]},"parsed":[]}"#;
    let absent = decode_price_update(absent, Budgets::default()).unwrap();
    let null = decode_price_update(null, Budgets::default()).unwrap();
    let empty = decode_price_update(empty, Budgets::default()).unwrap();
    assert_eq!(absent, null);
    assert_ne!(absent, empty);
    assert!(absent.binary.data.is_empty());
    assert_eq!(empty.parsed, Some(Vec::new()));
}

/// Metadata equality includes attributes so two different provider records stay distinct.
#[test]
fn listing_fields_and_attributes_are_complete_and_participate_in_equality() {
    let bytes = br#"[{"id":"feed-one","attributes":{"symbol":"A","country":"GB"},
        "asset_type":"crypto","base":"BTC","description":"Bitcoin feed",
        "display_symbol":"BTC/USD","generic_symbol":"BTCUSD","quote":"USD","symbol":"BTC/USD"}]"#;
    let listing = decode_price_feeds(bytes, Budgets::default()).unwrap();
    let first = &listing[0];
    assert_eq!(first.id, "feed-one");
    assert_eq!(first.attributes["symbol"], "A");
    assert_eq!(first.attributes["country"], "GB");
    assert_eq!(first.asset_type.as_deref(), Some("crypto"));
    assert_eq!(first.base.as_deref(), Some("BTC"));
    assert_eq!(first.description.as_deref(), Some("Bitcoin feed"));
    assert_eq!(first.display_symbol.as_deref(), Some("BTC/USD"));
    assert_eq!(first.generic_symbol.as_deref(), Some("BTCUSD"));
    assert_eq!(first.quote.as_deref(), Some("USD"));
    assert_eq!(first.symbol.as_deref(), Some("BTC/USD"));
    let mut second = first.clone();
    second.attributes.insert("symbol".into(), "B".into());
    assert_ne!(*first, second);
}

/// Null or missing attributes are empty; malformed attribute values remain errors.
#[test]
fn attributes_and_metadata_reject_wrong_types_without_silent_fallbacks() {
    for bytes in [
        br#"[{"id":"one"}]"#.as_slice(),
        br#"[{"id":"one","attributes":null}]"#,
    ] {
        assert!(
            decode_price_feeds(bytes, Budgets::default()).unwrap()[0]
                .attributes
                .is_empty()
        );
    }

    for bytes in [
        br#"[{"id":"one","attributes":false}]"#.as_slice(),
        br#"[{"id":"one","attributes":{"symbol":42}}]"#,
        br#"[{"id":null}]"#,
        br#"{}"#,
    ] {
        assert!(decode_price_feeds(bytes, Budgets::default()).is_err());
    }
}

/// Duplicate-aware admission covers nested maps as well as struct field names.
#[test]
fn lexical_budgets_and_duplicate_keys_precede_typed_decode() {
    let limits = Budgets::default();
    assert!(matches!(
        decode_price_update(
            RESPONSE,
            Budgets {
                bytes: RESPONSE.len() - 1,
                ..limits
            }
        ),
        Err(JsonError::Bytes)
    ));
    assert!(matches!(
        decode_price_update(RESPONSE, Budgets { depth: 1, ..limits }),
        Err(JsonError::Depth)
    ));
    assert!(matches!(
        decode_price_update(RESPONSE, Budgets { items: 1, ..limits }),
        Err(JsonError::Items)
    ));
    assert!(matches!(
        decode_price_update(
            RESPONSE,
            Budgets {
                string_bytes: 1,
                ..limits
            }
        ),
        Err(JsonError::String)
    ));
    let duplicate = br#"[{"id":"one","attributes":{"symbol":"A","symbol":"B"}}]"#;
    assert!(matches!(
        decode_price_feeds(duplicate, limits),
        Err(JsonError::Schema(_))
    ));
}

/// Component bounds precede large-integer conversion, while decimal syntax remains strict.
#[test]
fn component_limit_and_invalid_decimal_forms_fail_explicitly() {
    let mut json: serde_json::Value = serde_json::from_slice(RESPONSE).unwrap();

    for length in [MAX_INTEGER_BYTES, MAX_INTEGER_BYTES + 1] {
        json["parsed"][0]["price"]["price"] = "9".repeat(length).into();
        let bytes = serde_json::to_vec(&json).unwrap();
        assert_eq!(
            decode_price_update(&bytes, Budgets::default()).is_ok(),
            length == MAX_INTEGER_BYTES
        );
    }

    for invalid in ["", "1.5", "1e3", "0xff", " 12 ", "NaN", "-", "1_2", "12_"] {
        json["parsed"][0]["price"]["price"] = invalid.into();
        assert!(
            decode_price_update(&serde_json::to_vec(&json).unwrap(), Budgets::default()).is_err(),
            "accepted invalid decimal component: {invalid}"
        );
    }

    json["parsed"][0]["price"]["price"] = serde_json::json!(1.5);
    assert!(decode_price_update(&serde_json::to_vec(&json).unwrap(), Budgets::default()).is_err());
}

/// Names match all eleven categories and both encodings captured from Dart.
#[test]
fn asset_and_encoding_wire_names_are_exact_and_closed() {
    let assets = [
        (AssetType::Equity, "equity"),
        (AssetType::Fx, "fx"),
        (AssetType::Crypto, "crypto"),
        (AssetType::Metal, "metal"),
        (AssetType::Rates, "rates"),
        (AssetType::CryptoRedemptionRate, "crypto_redemption_rate"),
        (AssetType::Commodities, "commodities"),
        (AssetType::CryptoIndex, "crypto_index"),
        (AssetType::CryptoNav, "crypto_nav"),
        (AssetType::Eco, "eco"),
        (AssetType::Kalshi, "kalshi"),
    ];

    for (asset, name) in assets {
        assert_eq!(asset.wire_name(), name);
        assert_eq!(serde_json::to_value(asset).unwrap(), name);
        assert_eq!(
            serde_json::from_value::<AssetType>(name.into()).unwrap(),
            asset
        );
    }

    for (encoding, name) in [(Encoding::Hex, "hex"), (Encoding::Base64, "base64")] {
        assert_eq!(encoding.wire_name(), name);
        assert_eq!(Encoding::from_name(name).unwrap(), encoding);
        assert_eq!(serde_json::to_value(encoding).unwrap(), name);
    }

    for unknown in ["Hex", "BASE64", "", "binary"] {
        assert!(Encoding::from_name(unknown).is_err());
    }

    assert!(
        decode_price_update(br#"{"binary":{"encoding":"other"}}"#, Budgets::default()).is_err()
    );
}

/// Approximation never changes the stored integer or loops over a huge exponent.
#[test]
fn explicitly_lossy_float_conversion_retains_ieee_overflow_semantics() {
    const COMPONENT: i64 = 4200;
    const SCALE: i32 = -2;
    let mut price = Price {
        price: BigInt::from(COMPONENT),
        confidence: BigInt::from(0),
        exponent: SCALE,
        publish_time: 0,
    };
    assert_eq!(price.approximate_f64(), EMA_PRICE as f64);
    assert_eq!(price.price, BigInt::from(COMPONENT));
    price.exponent = i32::MAX;
    assert_eq!(price.approximate_f64(), f64::INFINITY);
    price.exponent = i32::MIN;
    assert_eq!(price.approximate_f64(), 0.0);
    price.price = BigInt::from(0);
    price.exponent = i32::MAX;
    assert!(price.approximate_f64().is_nan());
}
