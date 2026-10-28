//! Known bad types, numeric corruption and object aliases fail the same shared boundary.

use crate::require;
use solana_kit_client::rest::Budgets;
use solana_kit_jupiter::{Integer, decode_build, decode_execution, decode_order};

pub(super) fn responses() -> Result<(), &'static str> {
    let invalid_orders: &[&[u8]] = &[
        b"[]",
        b"[null,null]",
        b"null",
        b"42",
        br#"{"inAmount":1.5}"#,
        br#"{"outAmount":"1e3"}"#,
        br#"{"inAmount":{"$serde_json::private::Number":"1"}}"#,
        br#"{"transaction":12}"#,
        br#"{"requestId":false}"#,
        br#"{"router":[]}"#,
        br#"{"mode":1}"#,
        br#"{"lastValidBlockHeight":"1.2"}"#,
        br#"{"expireAt":"9223372036854775808"}"#,
        br#"{"slippageBps":1.25}"#,
        br#"{"swapType":{}}"#,
        br#"{"requestId":"first","requestId":"second"}"#,
    ];

    for body in invalid_orders {
        require(
            decode_order(body, Budgets::default()).is_err(),
            "malformed order accepted",
        )?;
    }

    for text in ["", "-", "+1", "1.0", "1e3", " 1", "0x1", "NaN"] {
        require(
            text.parse::<Integer>().is_err(),
            "invalid decimal integer accepted",
        )?;
    }

    invalid_execution()?;
    invalid_build()?;
    admission()
}

fn invalid_execution() -> Result<(), &'static str> {
    for body in [
        b"[]".as_slice(),
        br#"{"code":"0"}"#,
        br#"{"code":0.5}"#,
        br#"{"status":true}"#,
        br#"{"signature":[]}"#,
        br#"{"swapTransaction":{}}"#,
    ] {
        require(
            decode_execution(body, Budgets::default()).is_err(),
            "malformed execution accepted",
        )?;
    }

    Ok(())
}

fn invalid_build() -> Result<(), &'static str> {
    let bodies: &[&[u8]] = &[
        b"[]",
        br#"{"swapInstruction":[]}"#,
        br#"{"swapInstruction":{"programId":1}}"#,
        br#"{"swapInstruction":{"accounts":[[]]}}"#,
        br#"{"swapInstruction":{"accounts":[{"pubkey":false}]}}"#,
        br#"{"swapInstruction":{"accounts":[{"isSigner":"true"}]}}"#,
        br#"{"swapInstruction":{"accounts":[{"isWritable":1}]}}"#,
        br#"{"computeBudgetInstructions":[null]}"#,
        br#"{"setupInstructions":[1]}"#,
        br#"{"otherInstructions":{}}"#,
        br#"{"tipInstruction":{"data":[]}}"#,
        br#"{"cleanupInstruction":false}"#,
        br#"{"addressLookupTableAddresses":[null]}"#,
        br#"{"addressesByLookupTableAddress":[]}"#,
        br#"{"addressesByLookupTableAddress":{"table":null}}"#,
        br#"{"addressesByLookupTableAddress":{"table":[1]}}"#,
        br#"{"addressesByLookupTableAddress":{"table":[],"table":[]}}"#,
        br#"{"blockhashWithMetadata":[]}"#,
        br#"{"blockhashWithMetadata":{}}"#,
        br#"{"blockhashWithMetadata":{"blockhash":[256],"lastValidBlockHeight":1}}"#,
        br#"{"blockhashWithMetadata":{"blockhash":[-1],"lastValidBlockHeight":1}}"#,
        br#"{"blockhashWithMetadata":{"blockhash":[],"lastValidBlockHeight":null}}"#,
        br#"{"blockhashWithMetadata":{"blockhash":[],"lastValidBlockHeight":1.5}}"#,
    ];

    for body in bodies {
        require(
            decode_build(body, Budgets::default()).is_err(),
            "malformed build accepted",
        )?;
    }

    Ok(())
}

fn admission() -> Result<(), &'static str> {
    let body = br#"{"future":{"nested":["large"]}}"#;
    let budgets = [
        Budgets {
            bytes: 1,
            ..Budgets::default()
        },
        Budgets {
            depth: 1,
            ..Budgets::default()
        },
        Budgets {
            items: 1,
            ..Budgets::default()
        },
        Budgets {
            string_bytes: 1,
            ..Budgets::default()
        },
    ];

    for budget in budgets {
        require(
            decode_order(body, budget).is_err()
                && decode_build(body, budget).is_err()
                && decode_execution(body, budget).is_err(),
            "swap JSON admission budget ignored",
        )?;
    }

    Ok(())
}
