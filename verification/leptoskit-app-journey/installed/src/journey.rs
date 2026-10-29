//! Real OS controls, fresh native wallet entropy and independently checked ledger effects.
mod effect;
mod persistence;
mod subscription;

pub(crate) use persistence::Wallets;

use crate::{Error, Result, accessibility, require, validator::Validator};
use leptoskit_testing::native::{Locator, NativeTester, Platform};
use serde_json::{Value, json};
use std::{
    thread,
    time::{Duration, Instant},
};

const FUNDING: u64 = 2_000_000_000;
const AMOUNT: u64 = 1_000_000;
const ADDRESS_BYTES: usize = 32;
const SIGNATURE_BYTES: usize = 64;
const MINIMUM_COLORS: usize = 32;
const LOCAL_ACCOUNT: &str = "Local wallet address";
const OBSERVE: Duration = Duration::from_secs(30);
const INTERVAL: Duration = Duration::from_millis(100);

struct Transfer {
    payer: String,
    recipient: String,
    before: u64,
}

impl Transfer {
    fn after(&self) -> Result<u64> {
        self.before
            .checked_add(AMOUNT)
            .ok_or_else(|| Error::Response("recipient balance overflow".into()))
    }
}

pub(crate) fn run(
    tester: &mut NativeTester,
    node: &mut Validator,
    platform: Platform,
) -> Result<Wallets> {
    let payer = fresh_wallet(tester, platform)?;
    tester.tap(&accessibility::action(platform, "Create private copy"))?;
    tester.expect_visible(&Locator::text("Private copy created inside the app"))?;
    fund(node, &payer)?;
    tester.tap(&accessibility::action(platform, "Read balance"))?;
    tester.expect_visible(&Locator::text(format!("Balance: {FUNDING} lamports")))?;
    let recipient = bs58::encode([98; ADDRESS_BYTES]).into_string();
    let before = node.rpc.balance(&recipient)?;
    let transfer = Transfer {
        payer,
        recipient,
        before,
    };

    reject_transfer(tester, node, &transfer, platform)?;
    let balance = approve_transfer(tester, node, &transfer, platform)?;
    tester.tap(&accessibility::action(platform, "Read balance"))?;
    tester.expect_visible(&Locator::text(format!("Balance: {balance} lamports")))?;
    tester.checkpoint("Initial payer balance checked before process termination")?;
    subscription::run(tester, node, platform, "subscription-initial.json")?;
    require(
        node.rpc.balance(&transfer.payer)? == balance,
        "subscription probe changed payer funds",
    )?;

    node.alive()?;
    tester.tap(&accessibility::action(platform, "Disconnect wallet"))?;
    tester.expect_visible(&accessibility::action(platform, "Create local wallet"))?;
    tester.checkpoint("Explicit disconnect completed")?;
    persistence::save_other_wallet(tester, platform, transfer.payer, balance)
}

fn fresh_wallet(tester: &mut NativeTester, platform: Platform) -> Result<String> {
    tester.tap(&accessibility::action(platform, "Create local wallet"))?;
    tester.expect_visible(&Locator::text(
        "Local wallet connected; not saved automatically",
    ))?;
    tester.checkpoint("Fresh native wallet in the generated app")?;
    let account = accessibility::value(tester, LOCAL_ACCOUNT)?;
    public_bytes(&account, ADDRESS_BYTES)?;
    tester.expect_text(&Locator::accessibility_id(LOCAL_ACCOUNT), &account)?;
    Ok(account)
}

fn fund(node: &mut Validator, account: &str) -> Result<()> {
    node.alive()?;
    require(
        node.rpc.balance(account)? == 0,
        "UI key must be fresh on this ledger",
    )?;
    let funded = node.rpc.call(
        "requestAirdrop",
        json!([
            account,FUNDING,{"commitment":"confirmed"}
        ]),
    )?;
    let funded = funded
        .as_str()
        .ok_or_else(|| Error::Response("airdrop signature missing".into()))?;
    public_bytes(funded, SIGNATURE_BYTES)?;
    confirmed(node, funded)?;
    require(
        node.rpc.balance(account)? == FUNDING,
        "fresh payer funding effect",
    )
}

fn reject_transfer(
    tester: &mut NativeTester,
    node: &Validator,
    transfer: &Transfer,
    platform: Platform,
) -> Result<()> {
    tester.tap(&accessibility::action(platform, "Review test SOL transfer"))?;
    review(tester, transfer)?;
    tester.checkpoint("Real transfer review before rejection")?;
    tester.tap(&accessibility::action(platform, "Reject signature"))?;
    tester.expect_visible(&Locator::text("Transfer rejected; nothing submitted"))?;

    require(
        node.rpc.balance(&transfer.recipient)? == transfer.before,
        "rejection changed the recipient",
    )?;
    require(
        node.rpc.balance(&transfer.payer)? == FUNDING,
        "rejection charged the payer",
    )
}

fn approve_transfer(
    tester: &mut NativeTester,
    node: &mut Validator,
    transfer: &Transfer,
    platform: Platform,
) -> Result<u64> {
    tester.tap(&accessibility::action(platform, "Review test SOL transfer"))?;
    review(tester, transfer)?;
    tester.tap(&accessibility::action(platform, "Approve signature"))?;
    let after = transfer.after()?;
    let effect::Receipt {
        signature,
        payer_balance,
    } = effect::wait_for_transfer(node, transfer)?;
    tester.expect_visible(&Locator::text(format!(
        "Transfer confirmed: {signature}. Recipient balance: {after} lamports"
    )))?;
    tester.checkpoint("Generated app confirmed the independently executed transfer")?;

    let colors = tester.screenshot()?.distinct_colors(MINIMUM_COLORS)?;
    require(
        colors > MINIMUM_COLORS,
        "native pixels are blank or synthetic",
    )?;
    Ok(payer_balance)
}

fn review(tester: &NativeTester, transfer: &Transfer) -> Result<()> {
    let Transfer {
        payer, recipient, ..
    } = transfer;
    tester.expect_visible(&Locator::text("Approve local wallet signature"))?;
    tester.expect_visible(&Locator::text(format!(
        "Transfer {AMOUNT} lamports from {payer} to {recipient}. Network fees also apply."
    )))
}

pub(super) fn public_bytes(text: &str, length: usize) -> Result<Vec<u8>> {
    let bytes = bs58::decode(text)
        .into_vec()
        .map_err(|error| Error::Response(error.to_string()))?;
    require(
        bytes.len() == length && bs58::encode(&bytes).into_string() == text,
        "public base58 value is not canonical",
    )?;
    Ok(bytes)
}

/// Poll only safe funding observations after the single airdrop request.
fn confirmed(node: &mut Validator, signature: &str) -> Result<()> {
    let deadline = Instant::now() + OBSERVE;

    loop {
        node.alive()?;
        let reply = node.rpc.call(
            "getSignatureStatuses",
            json!([
                [signature],{"searchTransactionHistory":true}
            ]),
        )?;

        if funding_confirmed(&reply)? {
            return Ok(());
        }

        require(Instant::now() < deadline, "funding confirmation deadline")?;
        thread::sleep(INTERVAL);
    }
}

fn funding_confirmed(reply: &Value) -> Result<bool> {
    let statuses = reply["value"]
        .as_array()
        .ok_or_else(|| Error::Response("status array missing".into()))?;
    require(statuses.len() == 1, "funding status cardinality")?;
    let status = &statuses[0];

    if status.is_null() {
        return Ok(false);
    }

    require(status["err"] == Value::Null, "funding execution failed")?;
    Ok(matches!(
        status["confirmationStatus"].as_str(),
        Some("confirmed" | "finalized")
    ))
}
