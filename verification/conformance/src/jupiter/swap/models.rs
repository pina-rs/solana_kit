//! Exact integer alternatives and raw bodies survive provider schema extensions.

use crate::require;
use alloc::{format, string::ToString};
use solana_kit_client::rest::Budgets;
use solana_kit_jupiter::{
    ExecutionResponse, Integer, OrderMode, OrderResponse, decode_execution, decode_order,
};

const SLIPPAGE_BPS: i64 = 42;
const FAILED_CODE: i64 = -1001;

pub(super) fn responses() -> Result<(), &'static str> {
    let body = br#" {"inAmount":"1844674407370955161600001","outAmount":900719925474099312345,"transaction":"opaque-sensitive","requestId":"id-sensitive","router":"metis","mode":"manual","lastValidBlockHeight":184467440737095516161,"expireAt":"9007199254740993","swapType":"ExactIn","slippageBps":"42","future":{"value":0.00000000000000000001}} "#;
    let order = decode_order(body, Budgets::default()).map_err(|_| "order decoding failed")?;
    require(
        order.in_amount.as_ref().map(Integer::as_str) == Some("1844674407370955161600001")
            && order.out_amount.as_ref().map(Integer::as_str) == Some("900719925474099312345"),
        "order amounts were rounded",
    )?;
    require(
        order.last_valid_block_height.as_ref().map(Integer::as_str)
            == Some("184467440737095516161")
            && order.expire_at == Some(9007199254740993),
        "order integer alternatives lost precision",
    )?;
    require(
        order.encoded_transaction.as_deref() == Some("opaque-sensitive")
            && order.request_id.as_deref() == Some("id-sensitive"),
        "order identity fields changed",
    )?;
    require(
        order.router.as_deref() == Some("metis")
            && order.mode == Some(OrderMode::Manual)
            && order.swap_type.as_deref() == Some("ExactIn")
            && order.slippage_bps == Some(SLIPPAGE_BPS),
        "order metadata fields changed",
    )?;
    require(
        order.raw_response == body && !format!("{order:?}").contains("sensitive"),
        "order raw bytes lost or debug exposed them",
    )?;
    order_optionality()?;

    execution()
}

fn order_optionality() -> Result<(), &'static str> {
    for body in [
        b"{}".as_slice(),
        br#"{"inAmount":null,"outAmount":null,"mode":"future","transaction":null}"#.as_slice(),
    ] {
        let order =
            decode_order(body, Budgets::default()).map_err(|_| "nullable order rejected")?;
        require(
            order.in_amount.is_none()
                && order.out_amount.is_none()
                && order.mode.is_none()
                && order.encoded_transaction.is_none(),
            "optional order states changed",
        )?;
    }

    let ultra = decode_order(br#"{"mode":"ultra"}"#, Budgets::default())
        .map_err(|_| "ultra mode rejected")?;
    require(ultra.mode == Some(OrderMode::Ultra), "ultra mode changed")?;
    require(
        OrderResponse::default().raw_response.is_empty()
            && ExecutionResponse::default().raw_response.is_empty(),
        "direct construction raw default changed",
    )?;
    let normalized: Integer = "-000123456789012345678901234567890"
        .parse()
        .map_err(|_| "signed integer rejected")?;
    require(
        normalized.to_string() == "-123456789012345678901234567890",
        "integer normalization lost precision",
    )?;
    require(
        "-00"
            .parse::<Integer>()
            .map(|value| value.to_string())
            .ok()
            .as_deref()
            == Some("0"),
        "negative zero did not normalize",
    )
}

fn execution() -> Result<(), &'static str> {
    let body = br#"{"swapTransaction":"legacy-sensitive","signature":"signature-sensitive","error":"landing-sensitive","status":"Failed","code":-1001,"outputAmountResult":"18446744073709551616000","events":[{}]}"#;
    let response =
        decode_execution(body, Budgets::default()).map_err(|_| "execution decoding failed")?;
    require(
        response.encoded_swap_transaction.as_deref() == Some("legacy-sensitive")
            && response.signature.as_deref() == Some("signature-sensitive")
            && response.error.as_deref() == Some("landing-sensitive"),
        "execution string fields changed",
    )?;
    require(
        response.status.as_deref() == Some("Failed") && response.code == Some(FAILED_CODE),
        "failed execution with signature was lost",
    )?;
    require(
        response.raw_response == body && !format!("{response:?}").contains("sensitive"),
        "execution raw bytes lost or debug exposed them",
    )?;

    for body in [
        br#"{"error":{"detail":"future"}}"#.as_slice(),
        br#"{"error":42}"#.as_slice(),
        br#"{"error":null}"#.as_slice(),
        b"{}".as_slice(),
    ] {
        let response = decode_execution(body, Budgets::default())
            .map_err(|_| "reference error shape rejected")?;
        require(
            response.error.is_none() && response.raw_response == body,
            "nonstring execution error was not retained in raw JSON",
        )?;
    }

    Ok(())
}
