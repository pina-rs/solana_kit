//! Precision, optionality and known-field errors are checked independently from routing.

use crate::require;
use alloc::string::ToString;
use solana_kit_client::rest::Budgets;
use solana_kit_core::Address;
use solana_kit_jupiter::{decode_prices, decode_tokens};

const PRICE_DECIMALS: u8 = 9;
const TOKEN_DECIMALS: u8 = 6;

pub(super) fn precise() -> Result<(), &'static str> {
    let body = br#"{"11111111111111111111111111111111":{"usdPrice":0.000000000000000000000000000001,"blockId":18446744073709551615,"decimals":"9","priceChange24h":-1.234567890123456789}}"#;
    let prices =
        decode_prices(body, Budgets::default()).map_err(|_| "exact price decoding failed")?;
    let price = prices
        .get(&Address::from_bytes([0; 32]))
        .ok_or("price mint was lost")?;
    require(
        price.usd_price.as_ref().map(ToString::to_string).as_deref()
            == Some("0.000000000000000000000000000001"),
        "price precision was lost",
    )?;
    require(
        price.block_id.as_deref() == Some("18446744073709551615")
            && price.decimals == Some(PRICE_DECIMALS),
        "price scalar variants differ",
    )?;
    require(
        price
            .price_change24h
            .as_ref()
            .map(ToString::to_string)
            .as_deref()
            == Some("-1.234567890123456789"),
        "signed price change was rounded or renamed",
    )?;
    token_precision()
}

fn token_precision() -> Result<(), &'static str> {
    let body = br#"[{"id":"mint","name":"Token","symbol":"T","decimals":"6","tags":["verified","lfg"],"organicScore":99.125,"organicScoreLabel":"high","isVerified":true,"holderCount":"18446744073709551615","usdPrice":"1.234567890123456789","mcap":9007199254740993,"liquidity":"0"},{}]"#;
    let tokens =
        decode_tokens(body, Budgets::default()).map_err(|_| "exact token decoding failed")?;
    require(tokens.len() == 2, "token order/count differs")?;
    let token = &tokens[0];

    require(
        token.holder_count == Some(u64::MAX) && token.decimals == Some(TOKEN_DECIMALS),
        "integer precision was lost",
    )?;
    require(
        token.usd_price.as_ref().map(ToString::to_string).as_deref()
            == Some("1.234567890123456789"),
        "token price precision was lost",
    )?;
    require(
        token.mcap.as_ref().map(ToString::to_string).as_deref() == Some("9007199254740993"),
        "token market cap was rounded",
    )?;
    token_identity(token)?;
    require(
        tokens[1].id.is_none() && tokens[1].usd_price.is_none(),
        "missing optional field changed",
    )
}

fn token_identity(token: &solana_kit_jupiter::TokenItem) -> Result<(), &'static str> {
    require(
        token.id.as_deref() == Some("mint")
            && token.name.as_deref() == Some("Token")
            && token.symbol.as_deref() == Some("T")
            && token.icon.is_none(),
        "token display metadata changed",
    )?;
    require(
        token.tags.as_deref().map(|tags| {
            tags.iter()
                .map(|tag| tag.as_str())
                .collect::<alloc::vec::Vec<_>>()
        }) == Some(alloc::vec!["verified", "lfg"]),
        "token tag order changed",
    )?;
    require(
        token
            .organic_score
            .as_ref()
            .map(ToString::to_string)
            .as_deref()
            == Some("99.125")
            && token.organic_score_label.as_deref() == Some("high")
            && token.is_verified == Some(true),
        "token score or verification wire names changed",
    )?;
    require(
        token.liquidity.as_ref().map(ToString::to_string).as_deref() == Some("0"),
        "zero liquidity was lost",
    )
}

pub(super) fn invalid() -> Result<(), &'static str> {
    let bad_tokens: &[&[u8]] = &[
        b"{}",
        br#"[{"usdPrice":true}]"#,
        br#"[{"usdPrice":"NaN"}]"#,
        br#"[{"usdPrice":{"$serde_json::private::Number":"1"}}]"#,
        br#"[{"holderCount":"18446744073709551616"}]"#,
        br#"[{"holderCount":1.5}]"#,
        br#"[{"decimals":256}]"#,
        br#"[{"tags":[1]}]"#,
        br#"[{"isVerified":"true"}]"#,
        br#"[{"name":"first","name":"second"}]"#,
    ];

    for body in bad_tokens {
        require(
            decode_tokens(body, Budgets::default()).is_err(),
            "malformed token record accepted",
        )?;
    }

    let bad_prices: &[&[u8]] = &[
        b"[]",
        br#"{"not-a-mint":{}}"#,
        br#"{"11111111111111111111111111111111":null}"#,
        br#"{"11111111111111111111111111111111":{"usdPrice":"1"}}"#,
        br#"{"11111111111111111111111111111111":{"blockId":1.5}}"#,
    ];

    for body in bad_prices {
        require(
            decode_prices(body, Budgets::default()).is_err(),
            "malformed price record accepted",
        )?;
    }

    Ok(())
}
