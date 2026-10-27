//! Public Hermes models execute identically without browser globals or host I/O.
use crate::{CaseResult, case};
use alloc::string::ToString;
use solana_kit_client::rest::{Budgets, JsonError};
use solana_kit_core::fixed_points::BigInt;
use solana_kit_pyth::hermes::{
    AssetType, Encoding, Price, PriceFeed, decode_price_feeds, decode_price_update,
};

const RESPONSE: &[u8] =
    include_bytes!("../../../crates/solana_kit_pyth/tests/fixtures/hermes-update.json");
const PRICE: &str = "-1234567890123456789012345678901234567890";
const CONFIDENCE: &str = "340282366920938463463374607431768211456";
const SLOT: u64 = 9_007_199_254_740_993;
const PREVIOUS_TIME: i64 = -123;
const PROOF_TIME: i64 = 1_700_000_000;
const EMA_PRICE: i64 = 42;
const EMA_CONFIDENCE: i64 = -7;
const EXPONENT: i32 = -8;

/// Appends four groups while preserving every earlier report and negative control.
pub(crate) fn cases() -> [CaseResult; 4] {
    [
        case("pyth-hermes-exact-json-components", exact_components()),
        case("pyth-hermes-listing-and-wire-names", listing()),
        case("pyth-hermes-duplicate-aware-json-budgets", budgets()),
        case(
            "pyth-hermes-presence-and-explicit-approximation",
            presence(),
        ),
    ]
}

/// Compares components beyond 128 bits and exact timestamps with the Dart receipt.
fn exact_components() -> Result<(), &'static str> {
    let limits = Budgets::default();
    let update = decode_price_update(RESPONSE, limits).map_err(|_| "Hermes update")?;
    let feed = update
        .parsed
        .as_ref()
        .and_then(|feeds| feeds.first())
        .ok_or("Missing parsed feed")?;

    if update.binary.encoding != Encoding::Hex || update.binary.data != ["504e4155"] {
        return Err("Hermes binary envelope differs");
    }

    compare_prices(feed)?;
    compare_metadata(feed)?;
    let bytes = limits
        .serialize(&update)
        .map_err(|_| "Hermes serialization")?;

    if decode_price_update(&bytes, limits).map_err(|_| "Hermes roundtrip")? != update {
        return Err("Hermes response roundtrip differs");
    }

    Ok(())
}

/// Both aggregate and EMA retain their complete independently observed records.
fn compare_prices(feed: &PriceFeed) -> Result<(), &'static str> {
    if (
        feed.id.as_str(),
        feed.price.price.to_string(),
        feed.price.confidence.to_string(),
        feed.price.exponent,
        feed.price.publish_time,
    ) != (
        "feed-one",
        PRICE.to_string(),
        CONFIDENCE.to_string(),
        EXPONENT,
        i64::MAX,
    ) {
        return Err("Hermes exact aggregate differs");
    }

    if (
        feed.ema_price.price.to_string(),
        feed.ema_price.confidence.to_string(),
        feed.ema_price.exponent,
        feed.ema_price.publish_time,
    ) != (
        EMA_PRICE.to_string(),
        EMA_CONFIDENCE.to_string(),
        EXPONENT,
        i64::MAX,
    ) {
        return Err("Hermes integer-token EMA differs");
    }

    Ok(())
}

/// Optional publication metadata retains values beyond the JavaScript integer range.
fn compare_metadata(feed: &PriceFeed) -> Result<(), &'static str> {
    let metadata = feed.metadata.ok_or("Missing Hermes metadata")?;

    if (
        metadata.slot,
        metadata.prev_publish_time,
        metadata.proof_available_time,
    ) != (Some(SLOT), Some(PREVIOUS_TIME), Some(PROOF_TIME))
    {
        return Err("Hermes metadata integer precision differs");
    }

    Ok(())
}

/// Checks every reference category and includes attributes in record equality.
fn listing() -> Result<(), &'static str> {
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
        let serialized = serde_json::to_value(asset).map_err(|_| "Asset wire")?;

        if (asset.wire_name(), serialized.as_str()) != (name, Some(name)) {
            return Err("Hermes asset category differs");
        }
    }

    for (encoding, name) in [(Encoding::Hex, "hex"), (Encoding::Base64, "base64")] {
        if (encoding.wire_name(), Encoding::from_name(name)) != (name, Ok(encoding)) {
            return Err("Hermes binary encoding differs");
        }
    }

    let bytes = br#"[{"id":"one","attributes":{"symbol":"A"},"symbol":"A/USD"}]"#;
    let feeds = decode_price_feeds(bytes, Budgets::default()).map_err(|_| "Hermes listing")?;
    let first = feeds.first().ok_or("Missing Hermes listing entry")?;
    let mut second = first.clone();
    second.attributes.insert("symbol".into(), "B".into());

    if first == &second || first.symbol.as_deref() != Some("A/USD") {
        return Err("Hermes metadata attributes were lost");
    }

    Ok(())
}

/// Bounds and duplicate-map rejection execute before serde model construction.
fn budgets() -> Result<(), &'static str> {
    let limits = Budgets::default();

    if !matches!(
        decode_price_update(
            RESPONSE,
            Budgets {
                bytes: RESPONSE.len() - 1,
                ..limits
            }
        ),
        Err(JsonError::Bytes)
    ) {
        return Err("Hermes byte limit bypassed");
    }

    if !matches!(
        decode_price_update(RESPONSE, Budgets { depth: 1, ..limits }),
        Err(JsonError::Depth)
    ) {
        return Err("Hermes depth limit bypassed");
    }

    if !matches!(
        decode_price_update(RESPONSE, Budgets { items: 1, ..limits }),
        Err(JsonError::Items)
    ) {
        return Err("Hermes item limit bypassed");
    }

    let duplicate = br#"[{"id":"one","attributes":{"symbol":"A","symbol":"B"}}]"#;

    if decode_price_feeds(duplicate, limits).is_ok()
        || decode_price_update(br#"{"binary":{"encoding":"unknown"}}"#, limits).is_ok()
    {
        return Err("Hermes malformed response accepted");
    }

    Ok(())
}

/// Keeps absence distinct from empty results and makes floating-point loss explicit.
fn presence() -> Result<(), &'static str> {
    let limits = Budgets::default();
    let absent = decode_price_update(br#"{"binary":{"encoding":"hex"}}"#, limits)
        .map_err(|_| "Hermes absent parsed feeds")?;
    let null = decode_price_update(
        br#"{"binary":{"encoding":"hex","data":null},"parsed":null}"#,
        limits,
    )
    .map_err(|_| "Hermes null parsed feeds")?;
    let empty = decode_price_update(br#"{"binary":{"encoding":"hex"},"parsed":[]}"#, limits)
        .map_err(|_| "Hermes empty parsed feeds")?;

    if absent != null || absent == empty || !absent.binary.data.is_empty() {
        return Err("Hermes collection presence differs");
    }

    let price = Price {
        price: BigInt::from(0),
        confidence: BigInt::from(0),
        exponent: i32::MAX,
        publish_time: 0,
    };

    if !price.approximate_f64().is_nan() || price.price != BigInt::from(0) {
        return Err("Hermes explicit approximation differs");
    }

    Ok(())
}
