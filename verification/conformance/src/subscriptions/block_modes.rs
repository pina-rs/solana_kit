//! Each request-selected block transaction view is observed through the connection.

use super::*;
use solana_kit_client::methods::{history::HistoricalCommitment, transaction_results as tr};
use solana_kit_core::Address;

fn request(details: tr::TransactionDetails) -> Block {
    Block(
        BlockFilter::Mentions(Address::from_bytes([0; 32])),
        Some(BlockConfig(tr::BlockConfig {
            encoding: Some(tr::TransactionEncoding::Json),
            commitment: Some(HistoricalCommitment::Confirmed),
            max_supported_transaction_version: Some(1),
            rewards: Some(false),
            transaction_details: Some(details),
        })),
    )
}

fn mode(details: tr::TransactionDetails, name: &str, extra: &str) -> Check {
    let expected = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"blockSubscribe","params":[{{"mentionsAccountOrProgram":"11111111111111111111111111111111"}},{{"commitment":"confirmed","encoding":"json","maxSupportedTransactionVersion":1,"showRewards":false,"transactionDetails":"{name}"}}]}}"#
    );
    let raw = contextual(&format!(
        r#"{{"slot":1,"err":null,"block":{{"blockhash":"{ADDRESS}","previousBlockhash":"{ADDRESS}","parentSlot":0,"blockTime":null,"blockHeight":null,{extra}}}}}"#
    ));
    let result = delivered(request(details), &expected, "blockNotification", &raw)?;
    let result = result.value.block.ok_or("block view missing")?;

    let valid = match result.transactions {
        tr::BlockTransactions::Full(values) => name == "full" && values.is_empty(),
        tr::BlockTransactions::Accounts(values) => name == "accounts" && values.is_empty(),
        tr::BlockTransactions::Signatures(values) => {
            name == "signatures" && values == [solana_kit_core::Signature::from_bytes([0; 64])]
        }
        tr::BlockTransactions::None => false,
    };

    crate::require(valid, "request-selected block view differs")
}

pub(super) fn modes() -> Check {
    mode(tr::TransactionDetails::Full, "full", r#""transactions":[]"#)?;
    mode(
        tr::TransactionDetails::Accounts,
        "accounts",
        r#""transactions":[]"#,
    )?;
    mode(
        tr::TransactionDetails::Signatures,
        "signatures",
        &format!(r#""signatures":["{SIGNATURE}"]"#),
    )
}
