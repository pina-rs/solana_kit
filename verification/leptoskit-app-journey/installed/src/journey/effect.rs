//! The validator's transaction and balances are the oracle, independent of the SDK.
use super::{AMOUNT, INTERVAL, OBSERVE, SIGNATURE_BYTES, Transfer, public_bytes};
use crate::{Error, Result, require, validator::Validator};
use serde_json::{Value, json};
use std::{thread, time::Instant};

/// Discover only successful transfers from this freshly funded UI key.
/// Safe observations may poll; the UI's approve action is never repeated.
pub(super) fn wait_for_transfer(node: &mut Validator, transfer: &Transfer) -> Result<String> {
    let deadline = Instant::now() + OBSERVE;

    loop {
        node.alive()?;
        let signature = submitted(node, &transfer.recipient)?;

        if let Some(signature) = signature {
            let transaction = node.rpc.call("getTransaction", json!([
                signature,{"commitment":"confirmed","encoding":"json","maxSupportedTransactionVersion":0}
            ]))?;

            if !transaction.is_null() {
                verify(&transaction, transfer, &signature)?;
                require(
                    node.rpc.balance(&transfer.recipient)? == transfer.after()?,
                    "independent recipient effect",
                )?;

                return Ok(signature);
            }
        }

        require(
            Instant::now() < deadline,
            "executed transfer observation deadline",
        )?;
        thread::sleep(INTERVAL);
    }
}

fn submitted(node: &Validator, recipient: &str) -> Result<Option<String>> {
    let replies = node.rpc.call(
        "getSignaturesForAddress",
        json!([
            recipient,{"commitment":"confirmed","limit":2}
        ]),
    )?;
    let replies = replies
        .as_array()
        .ok_or_else(|| Error::Response("signature list missing".into()))?;
    require(
        replies.len() <= 1,
        "recipient received more than one transaction",
    )?;
    let Some(reply) = replies.first() else {
        return Ok(None);
    };
    require(reply["err"] == Value::Null, "transfer execution failed")?;
    let signature = reply["signature"]
        .as_str()
        .ok_or_else(|| Error::Response("transfer signature missing".into()))?;
    public_bytes(signature, SIGNATURE_BYTES)?;
    Ok(Some(signature.to_owned()))
}

fn verify(transaction: &Value, transfer: &Transfer, signature: &str) -> Result<()> {
    let message = &transaction["transaction"]["message"];
    require(
        transaction["meta"]["err"] == Value::Null,
        "transaction meta error",
    )?;
    require(
        transaction["transaction"]["signatures"] == json!([signature]),
        "transaction signer count or signature",
    )?;
    require(
        message["accountKeys"]
            == json!([
                transfer.payer,
                transfer.recipient,
                "11111111111111111111111111111111"
            ]),
        "executed transfer accounts",
    )?;
    instruction(message)?;
    balances(&transaction["meta"], transfer)
}

fn instruction(message: &Value) -> Result<()> {
    let instructions = message["instructions"]
        .as_array()
        .ok_or_else(|| Error::Response("instructions missing".into()))?;
    require(instructions.len() == 1, "executed instruction count")?;
    let instruction = &instructions[0];
    require(
        instruction["programIdIndex"] == 2 && instruction["accounts"] == json!([0, 1]),
        "System transfer account roles",
    )?;
    let data = instruction["data"]
        .as_str()
        .ok_or_else(|| Error::Response("instruction data missing".into()))?;
    let data = bs58::decode(data)
        .into_vec()
        .map_err(|error| Error::Response(error.to_string()))?;
    let mut expected = 2_u32.to_le_bytes().to_vec();
    expected.extend_from_slice(&AMOUNT.to_le_bytes());
    require(data == expected, "executed System transfer bytes")
}

fn balances(meta: &Value, transfer: &Transfer) -> Result<()> {
    require(
        meta["preBalances"][1] == transfer.before && meta["postBalances"][1] == transfer.after()?,
        "ledger recipient balance delta",
    )?;
    let fee = meta["fee"]
        .as_u64()
        .ok_or_else(|| Error::Response("transaction fee missing".into()))?;
    let before = meta["preBalances"][0]
        .as_u64()
        .ok_or_else(|| Error::Response("payer prebalance missing".into()))?;
    let spent = AMOUNT
        .checked_add(fee)
        .ok_or_else(|| Error::Response("transaction cost overflow".into()))?;
    let after = before
        .checked_sub(spent)
        .ok_or_else(|| Error::Response("invalid payer balance delta".into()))?;
    require(
        fee > 0 && meta["postBalances"][0] == after,
        "ledger payer debit and network fee",
    )
}
