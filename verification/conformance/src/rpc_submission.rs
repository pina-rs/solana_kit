//! Fees, positional statuses and opaque submission use the public client.
extern crate std;

use crate::{
    require,
    rpc_fixture::{call, framed},
    rpc_history::{KEY, MAX, SIG, signature},
};
use solana_kit_client::{
    Error, Presence,
    methods::{Commitment, ReadConfig, submission::*},
};
use solana_kit_core::Address;
use std::{format, vec};

pub(crate) fn operations() -> Result<(), &'static str> {
    fee_and_airdrop()?;
    statuses()?;
    submissions()?;
    failures()
}

/// Fees and airdrop signatures preserve their distinct exact request options.
fn fee_and_airdrop() -> Result<(), &'static str> {
    let fee = call(
        FeeForMessage(
            &[0, 1, 2],
            Some(ReadConfig {
                commitment: Some(Commitment::Processed),
                min_context_slot: Some(u64::MAX),
            }),
        ),
        "getFeeForMessage",
        &format!(r#"["AAEC",{{"commitment":"processed","minContextSlot":{MAX}}}]"#),
        &format!(r#"{{"context":{{"slot":{MAX}}},"value":{MAX}}}"#),
    )
    .map_err(|_| "fee failed")?;
    require(
        fee.context.slot == u64::MAX && fee.value == Some(u64::MAX),
        "fee narrowed",
    )?;
    let airdrop = call(
        RequestAirdrop(
            Address::from_bytes([0; 32]),
            u64::MAX,
            Some(AirdropConfig {
                commitment: Some(Commitment::Confirmed),
            }),
        ),
        "requestAirdrop",
        &format!(r#"["{KEY}",{MAX},{{"commitment":"confirmed"}}]"#),
        &format!(r#""{SIG}""#),
    )
    .map_err(|_| "airdrop failed")?;
    require(airdrop == signature(), "airdrop signature changed")
}

/// Positional statuses retain null slots and typed instruction-error details.
fn statuses() -> Result<(), &'static str> {
    let signatures = [signature(), signature()];
    let failure = r#"{"InstructionError":[255,{"Custom":4294967295}]}"#;
    let result = format!(
        concat!(
            r#"{{"context":{{"slot":{0}}},"value":[null,{{"slot":{0},"#,
            r#""confirmations":null,"err":{1},"status":{{"Err":{1}}},"confirmationStatus":"finalized"}}]}}"#
        ),
        MAX, failure
    );
    let statuses = call(
        SignatureStatuses(
            &signatures,
            Some(StatusesConfig {
                search_transaction_history: Some(false),
            }),
        ),
        "getSignatureStatuses",
        &format!(r#"[["{SIG}","{SIG}"],{{"searchTransactionHistory":false}}]"#),
        &result,
    )
    .map_err(|_| "statuses failed")?;

    require(
        statuses.value.len() == 2 && statuses.value[0].is_none(),
        "status null positions changed",
    )?;
    let value = statuses.value[1].as_ref().ok_or("status missing")?;
    require(
        value.slot == u64::MAX
            && matches!(
                value.err,
                Some(TransactionFailure::Known(
                    KnownTransactionFailure::InstructionError(
                        255,
                        InstructionFailure::Custom(u32::MAX)
                    )
                ))
            ),
        "typed execution failure changed",
    )
}

/// Submission treats wire bytes as opaque while validating response signatures.
fn submissions() -> Result<(), &'static str> {
    let result = format!(r#""{SIG}""#);
    let sent = call(
        SendTransaction {
            wire: &[0, 1, 2],
            encoding: WireEncoding::Base64,
            config: SendConfig {
                skip_preflight: Some(false),
                preflight_commitment: Some(Commitment::Finalized),
                max_retries: Some(u64::MAX),
                min_context_slot: Some(u64::MAX),
            },
        },
        "sendTransaction",
        &format!(
            concat!(
                r#"["AAEC",{{"encoding":"base64","skipPreflight":false,"#,
                r#""preflightCommitment":"finalized","maxRetries":{0},"minContextSlot":{0}}}]"#
            ),
            MAX
        ),
        &result,
    )
    .map_err(|_| "submission failed")?;
    require(sent == signature(), "submission signature changed")?;
    let wire = vec![0; 4096];
    let encoded = "1".repeat(4096);
    let sent = call(
        SendTransaction {
            wire: &wire,
            encoding: WireEncoding::Base58,
            config: SendConfig::default(),
        },
        "sendTransaction",
        &format!(r#"["{encoded}",{{"encoding":"base58"}}]"#),
        &result,
    )
    .map_err(|_| "opaque 4096-byte submission rejected")?;
    require(sent == signature(), "opaque submission signature changed")
}

fn failures() -> Result<(), &'static str> {
    let signatures = [signature()];
    let params = format!(r#"[["{SIG}"]]"#);
    let exact = r#"{ "FutureVariant" : [1e400, -1e-400, 184467440737095516160] }"#;
    let result = format!(
        concat!(
            r#"{{"context":{{"slot":0}},"value":[{{"slot":0,"confirmations":null,"#,
            r#""err":{0},"status":{{"Err":{0}}},"confirmationStatus":null}}]}}"#
        ),
        exact
    );
    let value = call(
        SignatureStatuses(&signatures, None),
        "getSignatureStatuses",
        &params,
        &result,
    )
    .map_err(|_| "future failure failed")?;
    let Some(TransactionFailure::Unknown(raw)) =
        &value.value[0].as_ref().ok_or("future status missing")?.err
    else {
        return Err("future error lost");
    };

    require(raw.get() == exact, "opaque numeric spelling changed")?;

    for invalid in [
        r#"{"InstructionError":[256,"InvalidArgument"]}"#,
        r#"{"AccountInUse":7}"#,
    ] {
        let invalid = result.replace(exact, invalid);
        require(
            call(
                SignatureStatuses(&signatures, None),
                "getSignatureStatuses",
                &params,
                &invalid,
            )
            .is_err(),
            "malformed known failure accepted",
        )?;
    }
    remote_preflight()
}

/// RPC preflight errors preserve raw numeric data rather than losing their cause.
fn remote_preflight() -> Result<(), &'static str> {
    let response =
        r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32002,"message":"preflight","data":1e400}}"#;
    let error = framed(
        SendTransaction {
            wire: &[],
            encoding: WireEncoding::Base64,
            config: SendConfig::default(),
        },
        "sendTransaction",
        r#"["",{"encoding":"base64"}]"#,
        response,
    );
    let Err(Error::Remote(value)) = error else {
        return Err("remote preflight error lost");
    };
    require(
        matches!(value.data, Presence::Present(raw) if raw.get() == "1e400"),
        "remote preflight data changed",
    )
}
