//! Historical full results retain their distinct version and number contracts.
extern crate std;

use crate::{
    require,
    rpc_fixture::call,
    rpc_history::{KEY, MAX, SIG, signature},
};
use solana_kit_client::{Presence, methods::transaction_results::*};
use std::{format, string::String};

pub(super) fn binary_entry(version: &str) -> String {
    format!(
        concat!(
            r#"{{"slot":{0},"blockTime":null,"transactionIndex":4294967295,"#,
            r#""transaction":["AAE=","base64"],"meta":null{1}}}"#
        ),
        MAX, version
    )
}

/// Historical omissions and recognized versions succeed; malformed versions still fail.
pub(crate) fn full_results() -> Result<(), &'static str> {
    let config = TransactionReadConfig {
        encoding: Some(TransactionEncoding::Base64),
        max_supported_transaction_version: Some(1),
        ..Default::default()
    };
    let params = format!(r#"["{SIG}",{{"encoding":"base64","maxSupportedTransactionVersion":1}}]"#);

    for version in [
        "",
        r#","version":0"#,
        r#","version":1"#,
        r#","version":"legacy""#,
    ] {
        let value = call(
            Transaction(signature(), Some(config)),
            "getTransaction",
            &params,
            &binary_entry(version),
        )
        .map_err(|_| "historical version omission failed")?
        .ok_or("transaction missing")?;
        require(
            value.slot == u64::MAX && value.transaction_index == Presence::Present(u32::MAX),
            "transaction metadata narrowed",
        )?;
        require(
            matches!(value.details.transaction, TransactionView::Binary(bytes) if bytes == [0, 1]),
            "transaction bytes changed",
        )?;
    }

    historical_block()?;

    for version in [r#","version":2"#, r#","version":null"#, r#","version":"1""#] {
        require(
            call(
                Transaction(signature(), Some(config)),
                "getTransaction",
                &params,
                &binary_entry(version),
            )
            .is_err(),
            "invalid transaction version accepted",
        )?;
    }

    parsed_numbers()
}

/// Historical block metadata remains exact even when transaction versions are absent.
fn historical_block() -> Result<(), &'static str> {
    const BLOCK_SLOT: u64 = 42;
    let block_config = BlockConfig {
        encoding: Some(TransactionEncoding::Base64),
        rewards: Some(false),
        max_supported_transaction_version: Some(1),
        ..Default::default()
    };
    let result = format!(
        concat!(
            r#"{{"blockhash":"{0}","previousBlockhash":"{0}","parentSlot":{1},"#,
            r#""blockTime":null,"blockHeight":null,"transactions":[{{"#,
            r#""transaction":["AAE=","base64"],"meta":null}}]}}"#
        ),
        KEY, MAX
    );
    let block = call(
        Block(BLOCK_SLOT, Some(block_config)),
        "getBlock",
        r#"[42,{"encoding":"base64","maxSupportedTransactionVersion":1,"rewards":false}]"#,
        &result,
    )
    .map_err(|_| "historical block failed")?
    .ok_or("block missing")?;
    require(block.parent_slot == u64::MAX, "block parent narrowed")
}

fn parsed_numbers() -> Result<(), &'static str> {
    let config = TransactionReadConfig {
        encoding: Some(TransactionEncoding::JsonParsed),
        ..Default::default()
    };
    let params = format!(r#"["{SIG}",{{"encoding":"jsonParsed"}}]"#);
    let exact = r#"{ "amount":1e400,"tiny":-1e-400,"text":"\u0061" }"#;
    let result = format!(
        concat!(
            r#"{{"slot":0,"blockTime":null,"transaction":{{"signatures":["{0}"],"#,
            r#""message":{{"accountKeys":[],"recentBlockhash":"{1}","instructions":[{{"#,
            r#""programId":"{1}","program":"future","parsed":{2}}}]}}}},"meta":null}}"#
        ),
        SIG, KEY, exact
    );
    let value = call(
        Transaction(signature(), Some(config)),
        "getTransaction",
        &params,
        &result,
    )
    .map_err(|_| "parsed instruction failed")?
    .ok_or("parsed transaction missing")?;

    let TransactionView::JsonParsed(value) = value.details.transaction else {
        return Err("parsed transaction variant changed");
    };
    let ParsedOrPartialInstruction::Parsed(instruction) = &value.message.instructions[0] else {
        return Err("parsed instruction variant changed");
    };
    require(
        instruction.parsed.get() == exact,
        "parsed exact JSON changed",
    )?;
    token_annotations(&result, &params, config)
}

/// Extreme UI annotations remain exact numbers, while synthetic number objects fail.
fn token_annotations(
    result: &str,
    params: &str,
    config: TransactionReadConfig,
) -> Result<(), &'static str> {
    let metadata = format!(
        concat!(
            r#"{{"err":null,"status":{{"Ok":null}},"fee":0,"preBalances":[],"postBalances":[],"#,
            r#""postTokenBalances":[{{"accountIndex":0,"mint":"{0}","uiTokenAmount":{{"amount":"1","#,
            r#""decimals":0,"uiAmount":1e400,"uiAmountString":"1"}}}}]}}"#
        ),
        KEY
    );
    let result = result.replace(r#""meta":null"#, &format!(r#""meta":{metadata}"#));
    call(
        Transaction(signature(), Some(config)),
        "getTransaction",
        params,
        &result,
    )
    .map_err(|_| "extreme token annotation failed")?;
    let synthetic = result.replace("1e400", r#"{"$serde_json::private::Number":"1e400"}"#);
    require(
        call(
            Transaction(signature(), Some(config)),
            "getTransaction",
            params,
            &synthetic,
        )
        .is_err(),
        "synthetic Number object accepted as scalar",
    )
}
