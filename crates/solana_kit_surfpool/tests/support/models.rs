//! Model fixtures distinguish signed exact integers, object schemas and secret display.

use super::{call, require};
use alloc::{format, vec};
use solana_kit_surfpool::{cheatcodes::GetSurfnetInfo, hex, types::*};

pub fn run() -> Result<(), &'static str> {
    let response = call(
        GetSurfnetInfo,
        "surfnet_getSurfnetInfo",
        "[]",
        r#"{"context":{"slot":18446744073709551615},"value":{"future":18446744073709551615},"extension":true}"#,
    )?;
    require(
        response
            .context()
            .and_then(|context| context.get("slot"))
            .and_then(serde_json::Value::as_u64)
            == Some(u64::MAX),
        "response context lost precision",
    )?;
    require(
        response
            .envelope
            .as_ref()
            .and_then(|envelope| envelope.get("extension"))
            == Some(&serde_json::Value::Bool(true)),
        "context envelope extension was discarded",
    )?;
    require(
        response.value["future"].as_u64() == Some(u64::MAX),
        "raw response integer lost precision",
    )?;
    objects()?;
    secret_and_hex()
}

fn objects() -> Result<(), &'static str> {
    let event: SimnetEventValue = serde_json::from_str(r#"{"kind":"clock","clock":{"slot":18446744073709551615,"unixTimestamp":-9223372036854775808},"logs":["one"],"runbookErrors":[],"fee":9007199254740993}"#).map_err(|_| "valid event object rejected")?;
    require(
        event.clock.as_ref().and_then(|clock| clock.unix_timestamp) == Some(i128::from(i64::MIN)),
        "event signed timestamp was altered",
    )?;
    require(
        event.logs == Some(vec!["one".into()]) && event.fee == Some(9_007_199_254_740_993),
        "event optional list or exact fee changed",
    )?;
    require(
        serde_json::from_str::<EpochInfoValue>("[1,2,3,4,5,null]").is_err(),
        "epoch sequence accepted as object",
    )?;
    require(
        serde_json::from_str::<ClockValue>("[]").is_err(),
        "clock sequence accepted as object",
    )?;
    require(
        serde_json::from_str::<SimnetEventValue>(r#"{"kind":"clock","clock":[]}"#).is_err(),
        "nested clock sequence accepted",
    )?;
    require(
        serde_json::from_str::<ConfidentialBalance>(r#"{"pendingBalanceCreditCounter":1.5}"#)
            .is_err(),
        "fractional counter accepted",
    )?;
    require(
        serde_json::from_str::<ConfidentialKeys>(
            r#"{"elgamalPubkey":"public","elgamalSecretKey":false,"aesKey":"aes"}"#,
        )
        .is_err(),
        "wrong confidential key type accepted",
    )
}

fn secret_and_hex() -> Result<(), &'static str> {
    let keys = ConfidentialBalanceKeys::Both {
        aes_key: SecretString::new("aes-secret"),
        elgamal_secret_key: SecretString::new("elgamal-secret"),
    };
    require(
        !format!("{keys:?}").contains("aes-secret"),
        "debug output disclosed a secret",
    )?;
    require(
        serde_json::to_string(&keys).map_err(|_| "key serialization failed")?
            == r#"{"aesKey":"aes-secret","elgamalSecretKey":"elgamal-secret"}"#,
        "key serialization changed source data",
    )?;
    require(
        hex::encode(&[0, 16, 255]) == "0010ff",
        "lowercase hex encoding changed",
    )?;
    require(
        hex::decode("0010ff") == Ok(vec![0, 16, 255]),
        "hex roundtrip changed bytes",
    )?;

    for malformed in ["0", "0x00", "FF", "gg", "é"] {
        require(
            hex::decode(malformed).is_err(),
            "malformed or noncanonical hex accepted",
        )?;
    }

    Ok(())
}
