//! The owned ledger changes while a cancelled display remains frozen.
use super::{SIGNATURE_BYTES, confirmed, public_bytes};
use crate::{Error, Result, accessibility, require, validator::Validator};
use leptoskit_testing::native::{Locator, NativeTester, Platform};
use serde_json::json;
use std::{fs::OpenOptions, io::Write};

const CREDIT: u64 = 1_000_000;
const BALANCE: &str = "Subscription balance";
const ACTIVITY_SLOT: &str = "Subscription activity slot";
const OBSERVED_CONTEXT: &str = "Subscription observed context";
const OBSERVED_BALANCE: &str = "Subscription observed RPC balance";

struct Credit {
    balance: u64,
    slot: u64,
}

struct Activity {
    context: u64,
    balance: u64,
    slot: u64,
}

pub(super) fn run(
    tester: &mut NativeTester,
    node: &mut Validator,
    platform: Platform,
    receipt: &str,
) -> Result<()> {
    let result = observe(tester, node, platform, receipt);
    let failure_capture = if result.is_err() {
        tester.checkpoint("Subscription failure before owned socket cleanup")
    } else {
        Ok(())
    };

    let cleanup = (|| {
        tester.tap(&accessibility::action(
            platform,
            "Close subscription socket",
        ))?;
        tester.expect_visible(&Locator::text("Subscription socket closed"))?;
        tester.checkpoint("Owned account and slot streams and their socket closed")
    })();
    crate::finish([result, failure_capture, cleanup])
}

fn observe(
    tester: &mut NativeTester,
    node: &mut Validator,
    platform: Platform,
    receipt: &str,
) -> Result<()> {
    let account = bs58::encode([97; 32]).into_string();
    let before = node.rpc.balance(&account)?;
    tester.tap(&accessibility::action(platform, "Watch account updates"))?;
    tester.expect_visible(&Locator::text("Account subscription active"))?;
    tester.expect_text(
        &Locator::accessibility_id(BALANCE),
        "Subscribed balance: waiting",
    )?;

    let accepted = credit(node, &account, before)?;
    expect_balance(tester, "Subscribed", accepted.balance)?;
    tester
        .checkpoint("Real subscription update matches the independently confirmed probe credit")?;
    require(
        accessibility::value(tester, "Subscription account")? == account,
        "subscription probe address",
    )?;

    let held = cancel(tester, node, platform, &account, accepted.balance)?;
    let late = credit(node, &account, held.balance)?;
    tester.tap(&accessibility::action(
        platform,
        "Observe later socket activity",
    ))?;
    let (activity, displayed) =
        frozen_after_activity(tester, node, &account, accepted.balance, &late)?;

    let evidence = json!({
        "account":account,
        "accepted_rpc_balance":accepted.balance,
        "held_rpc_balance":held.balance,
        "late_rpc_balance":late.balance,
        "cancelled_display":displayed,
        "late_transaction_slot":late.slot,
        "sdk_balance_context":activity.context,
        "sdk_balance":activity.balance,
        "same_socket_slot":activity.slot,
    });
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(tester.artifacts().join(receipt))?
        .write_all(&serde_json::to_vec_pretty(&evidence)?)?;
    Ok(())
}

fn frozen_after_activity(
    tester: &mut NativeTester,
    node: &Validator,
    account: &str,
    balance: u64,
    late: &Credit,
) -> Result<(Activity, String)> {
    tester.expect_visible(&Locator::text("Same-socket slot activity observed"))?;
    expect_balance(tester, "Cancelled", balance)?;
    tester.checkpoint("Later ledger credit and same-socket slot leave cancelled balance frozen")?;
    let activity = Activity {
        context: captured_number(tester, OBSERVED_CONTEXT)?,
        balance: captured_number(tester, OBSERVED_BALANCE)?,
        slot: captured_number(tester, ACTIVITY_SLOT)?,
    };
    let displayed = accessibility::value(tester, BALANCE)?;
    require(
        displayed == format!("Cancelled balance: {balance} lamports"),
        "late activity capture changed the cancelled display",
    )?;
    require(
        activity.context >= late.slot && activity.balance == late.balance,
        "SDK observation did not cover the independently confirmed late credit",
    )?;
    require(
        activity.slot > activity.context,
        "same-socket slot did not pass the SDK observation",
    )?;
    require(
        node.rpc.balance(account)? == late.balance,
        "late probe ledger balance changed",
    )?;
    Ok((activity, displayed))
}

fn cancel(
    tester: &mut NativeTester,
    node: &mut Validator,
    platform: Platform,
    account: &str,
    balance: u64,
) -> Result<Credit> {
    tester.tap(&accessibility::action(platform, "Hold next account update"))?;
    tester.expect_visible(&Locator::text("Waiting to hold a real subscription update"))?;
    let held = credit(node, account, balance)?;
    tester.expect_visible(&Locator::text("Real subscription update held"))?;
    expect_balance(tester, "Subscribed", balance)?;
    tester.checkpoint("Second genuine account notification held before cancellation")?;
    tester.tap(&accessibility::action(
        platform,
        "Cancel account subscription",
    ))?;
    tester.expect_visible(&Locator::text("Held update ignored after cancellation"))?;
    expect_balance(tester, "Cancelled", balance)?;
    tester.checkpoint("Unsubscribe acknowledged and held callback rejected")?;
    Ok(held)
}

fn expect_balance(tester: &NativeTester, prefix: &str, balance: u64) -> Result<()> {
    tester.expect_text(
        &Locator::accessibility_id(BALANCE),
        &format!("{prefix} balance: {balance} lamports"),
    )
}

fn captured_number(tester: &NativeTester, label: &str) -> Result<u64> {
    accessibility::value(tester, label)?
        .parse::<u64>()
        .map_err(|error| Error::Response(error.to_string()))
}

/// Each credit executes once; only confirmation and balance observations poll.
fn credit(node: &mut Validator, account: &str, before: u64) -> Result<Credit> {
    let expected = before
        .checked_add(CREDIT)
        .ok_or_else(|| Error::Response("subscription probe balance overflow".into()))?;
    node.alive()?;
    let signature = node.rpc.call(
        "requestAirdrop",
        json!([account,CREDIT,{"commitment":"confirmed"}]),
    )?;
    let signature = signature
        .as_str()
        .ok_or_else(|| Error::Response("subscription credit signature missing".into()))?;
    public_bytes(signature, SIGNATURE_BYTES)?;
    confirmed(node, signature)?;

    let balance = node.rpc.balance(account)?;
    require(balance == expected, "independent subscription probe credit")?;
    let statuses = node.rpc.call(
        "getSignatureStatuses",
        json!([[signature],{"searchTransactionHistory":true}]),
    )?;
    let slot = statuses["value"][0]["slot"]
        .as_u64()
        .ok_or_else(|| Error::Response("subscription credit slot missing".into()))?;
    Ok(Credit { balance, slot })
}
