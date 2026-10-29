//! Fees, positional statuses and identity-bound submission use the public client.
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
use solana_kit_core::{Address, Signature};
use std::{format, vec, vec::Vec};

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

/// Submission retains exact signed bytes and rejects foreign acknowledgements.
fn submissions() -> Result<(), &'static str> {
    use base64::{Engine, engine::general_purpose::STANDARD};
    let wire = signed_wire(false);
    let expected = Signature::from_bytes([7; 64]);
    let result = format!(r#""{expected}""#);
    let encoded = STANDARD.encode(&wire);

    for invalid in [vec![], vec![0, 1, 2], vec![0; 4096]] {
        require(
            SignedWire::new(&invalid).is_err(),
            "malformed signed wire accepted",
        )?;
    }
    let sent = call(
        SendTransaction {
            wire: SignedWire::new(&wire).map_err(|_| "invalid submission wire")?,
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
                r#"["{encoded}",{{"encoding":"base64","skipPreflight":false,"#,
                r#""preflightCommitment":"finalized","maxRetries":{0},"minContextSlot":{0}}}]"#
            ),
            MAX,
            encoded = encoded
        ),
        &result,
    )
    .map_err(|_| "submission failed")?;
    require(sent == expected, "submission signature changed")?;
    let wire = signed_wire(true);
    require(wire.len() == 4096, "V1 packet fixture changed")?;
    let encoded = bs58::encode(&wire).into_string();
    let sent = call(
        SendTransaction {
            wire: SignedWire::new(&wire).map_err(|_| "invalid V1 wire")?,
            encoding: WireEncoding::Base58,
            config: SendConfig::default(),
        },
        "sendTransaction",
        &format!(r#"["{encoded}",{{"encoding":"base58"}}]"#),
        &result,
    )
    .map_err(|_| "signed 4096-byte submission rejected")?;
    require(sent == expected, "V1 submission signature changed")?;
    let request = SendTransaction {
        wire: SignedWire::new(&wire).map_err(|_| "invalid V1 wire")?,
        encoding: WireEncoding::Base64,
        config: SendConfig::default(),
    };
    let foreign = Signature::from_bytes([8; 64]);
    let rejected = solana_kit_client::methods::Method::decode(&request, &format!(r#""{foreign}""#));
    require(
        matches!(rejected, Err(Error::SubmissionSignatureMismatch { expected: e, actual: a })
        if e == expected && *a == foreign),
        "foreign acknowledgement accepted",
    )
}

/// Literal legacy and maximum-sized V1 fixtures do not use the SDK encoder.
fn signed_wire(v1: bool) -> Vec<u8> {
    let mut bytes = vec![];

    if v1 {
        bytes.extend([0x81, 1, 0, 1]);
        bytes.extend([0; 4]);
        bytes.extend([2; 32]);
        bytes.extend([1, 2]);
        bytes.extend([1; 32]);
        bytes.extend([3; 32]);
        bytes.extend([1, 0]);
        bytes.extend(3922u16.to_le_bytes());
        bytes.extend([0; 3922]);
        bytes.extend([7; 64]);

        return bytes;
    }

    bytes.push(1);
    bytes.extend([7; 64]);
    bytes.extend([1, 0, 0, 1]);
    bytes.extend([1; 32]);
    bytes.extend([2; 32]);
    bytes.push(0);

    bytes
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
    use base64::{Engine, engine::general_purpose::STANDARD};
    let wire = signed_wire(false);
    let encoded = STANDARD.encode(&wire);
    let response =
        r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32002,"message":"preflight","data":1e400}}"#;
    let error = framed(
        SendTransaction {
            wire: SignedWire::new(&wire).map_err(|_| "invalid preflight wire")?,
            encoding: WireEncoding::Base64,
            config: SendConfig::default(),
        },
        "sendTransaction",
        &format!(r#"["{encoded}",{{"encoding":"base64"}}]"#),
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
