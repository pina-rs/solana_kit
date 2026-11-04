//! An independently installed wallet approves every signature; the generated
//! app owns the granted RPC, submission and subscription work.
use super::{
    AMOUNT, FUNDING, Transfer, accessibility, effect, fund, public_bytes, subscription,
};
use crate::{Error, Result, android, require, wallet};
use ed25519_dalek::{Signature as Ed25519Signature, VerifyingKey};
use leptoskit_testing::native::{Locator, NativeTester, Platform};
use std::path::Path;

const ADDRESS_BYTES: usize = 32;
const RECEIPT: &str = "files/journey-mwa.txt";
const MESSAGE: &str = "Solana Kit Android review";
const ADDRESS_LABEL: &str = "External wallet address";
const CHOOSE: &str = "Choose a wallet";

pub(crate) struct External<'a> {
    pub app: &'a str,
    pub adb: &'a Path,
    pub device: &'a str,
}

pub(crate) struct State {
    payer: String,
    balance: u64,
}

/// The first process authorizes the fixed wallet key, funds its visible public
/// account once, verifies a message, rejects and approves one exact transfer,
/// runs the subscription race, cancels a pending request, reauthorizes, and
/// disconnects. Every wallet interaction is a real installed application.
pub(crate) fn initial(
    tester: &mut NativeTester,
    node: &mut super::Validator,
    platform: Platform,
    external: &External<'_>,
) -> Result<State> {
    selection(tester, platform)?;
    connect(tester, platform, external)?;
    let payer = address(tester)?;
    public_bytes(&payer, ADDRESS_BYTES)?;
    fund(node, &payer)?;
    read_balance(tester, platform, FUNDING)?;
    tester.checkpoint("Funded external wallet balance read through the SDK")?;

    message(tester, platform, external, 1)?;
    let recipient = bs58::encode([98; ADDRESS_BYTES]).into_string();
    let transfer = Transfer {
        payer: payer.clone(),
        recipient,
        before: 0,
    };
    reject(tester, node, platform, external, &transfer)?;
    approve(tester, node, platform, external, &transfer)?;
    let balance = node
        .rpc
        .balance(&payer)
        .map_err(|error| Error::Response(error.to_string()))?;
    tester.checkpoint("External wallet transfer confirmed by the owned ledger")?;

    subscription::run(tester, node, platform, "subscription-mwa-initial.json")?;
    pending_cancel(tester, node, platform, &transfer, balance)?;
    reconnect(tester, platform, external, &payer)?;
    disconnect(tester, platform)?;
    retain(tester, external, "mwa-initial")?;
    tester.checkpoint("External wallet journey, cancellation and disconnect completed")?;
    Ok(State { payer, balance })
}

/// A genuine process relaunch must require a fresh authorization and retain the
/// same public account and ledger balance. No transfer is repeated.
pub(crate) fn relaunch(
    tester: &mut NativeTester,
    platform: Platform,
    external: &External<'_>,
    state: &State,
) -> Result<()> {
    selection(tester, platform)?;
    connect(tester, platform, external)?;
    let payer = address(tester)?;
    require(payer == state.payer, "relaunch authorized a different account")?;
    read_balance(tester, platform, state.balance)?;
    tester.checkpoint("Relaunched app restored the same external public account")?;
    message(tester, platform, external, 1)?;
    disconnect(tester, platform)?;
    retain(tester, external, "mwa-relaunch")?;
    tester.checkpoint("Relaunched external wallet authorization and signature verified")
}

fn selection(tester: &NativeTester, platform: Platform) -> Result<()> {
    tester.expect_visible(&Locator::text(CHOOSE))?;
    tester.expect_visible(&accessibility::action(platform, "Connect Android wallet"))?;
    Ok(())
}

fn connect(
    tester: &mut NativeTester,
    platform: Platform,
    external: &External<'_>,
) -> Result<()> {
    tester.tap(&accessibility::action(platform, "Connect Android wallet"))?;
    wallet::authorize(tester)?;
    wallet::app(tester, external.app)?;
    tester.expect_visible(&Locator::text("Android wallet authorized"))?;
    Ok(())
}

fn address(tester: &mut NativeTester) -> Result<String> {
    tester.checkpoint("External public account visible in the generated app")?;
    accessibility::value(tester, ADDRESS_LABEL)
}

fn read_balance(tester: &mut NativeTester, platform: Platform, expected: u64) -> Result<()> {
    tester.tap(&accessibility::action(
        platform,
        "Read external wallet balance",
    ))?;
    tester.expect_visible(&Locator::text(format!("Balance: {expected} lamports")))
}

fn message(
    tester: &mut NativeTester,
    platform: Platform,
    external: &External<'_>,
    generation: u64,
) -> Result<()> {
    tester.tap(&accessibility::action(platform, "Request wallet signature"))?;
    tester.expect_visible(&accessibility::action(platform, "Cancel request"))?;
    wallet::present(tester)?;
    wallet::sign(tester, "SIGN MESSAGE(S)")?;
    wallet::app(tester, external.app)?;
    tester.expect_visible(&Locator::text("Wallet signature verified"))?;
    tester.checkpoint("External wallet message signature verified")?;
    verify_message(&receipt(external)?, &address(tester)?, generation)
}

fn reject(
    tester: &mut NativeTester,
    node: &mut super::Validator,
    platform: Platform,
    external: &External<'_>,
    transfer: &Transfer,
) -> Result<()> {
    tester.tap(&accessibility::action(
        platform,
        "Review external wallet transfer",
    ))?;
    wallet::present(tester)?;
    wallet::decline(tester)?;
    wallet::app(tester, external.app)?;
    tester.expect_visible(&Locator::text("Transfer rejected; nothing submitted"))?;
    tester.checkpoint("External wallet rejected the transfer without submission")?;
    require(
        node.rpc.balance(&transfer.recipient).map_err(|error| {
            Error::Response(error.to_string())
        })? == transfer.before,
        "rejection changed the recipient",
    )?;
    let signatures = node
        .rpc
        .call(
            "getSignaturesForAddress",
            serde_json::json!([transfer.recipient, {"commitment":"confirmed","limit":2}]),
        )
        .map_err(|error| Error::Response(error.to_string()))?;
    require(
        signatures
            .as_array()
            .is_some_and(|values| values.is_empty()),
        "rejected transfer reached the ledger",
    )?;
    verify_rejected(&receipt(external)?, transfer)
}

fn approve(
    tester: &mut NativeTester,
    node: &mut super::Validator,
    platform: Platform,
    external: &External<'_>,
    transfer: &Transfer,
) -> Result<()> {
    tester.tap(&accessibility::action(
        platform,
        "Review external wallet transfer",
    ))?;
    wallet::present(tester)?;
    wallet::sign(tester, "SIGN TRANSACTION(S)")?;
    wallet::app(tester, external.app)?;
    let executed = effect::wait_for_transfer(node, transfer)?;
    tester.expect_visible(&Locator::text(format!(
        "Transfer confirmed: {}. Recipient balance: {} lamports",
        executed.signature,
        transfer.after()?
    )))?;
    tester.checkpoint("External wallet transaction independently executed")?;
    verify_approved(&receipt(external)?, transfer, &executed.signature)?;
    require(
        node.rpc
            .balance(&transfer.payer)
            .map_err(|error| Error::Response(error.to_string()))?
            == executed.payer_balance,
        "independent payer balance after the external wallet transfer",
    )
}

fn pending_cancel(
    tester: &mut NativeTester,
    node: &super::Validator,
    platform: Platform,
    transfer: &Transfer,
    payer_balance: u64,
) -> Result<()> {
    tester.tap(&accessibility::action(platform, "Request wallet signature"))?;
    tester.expect_visible(&accessibility::action(platform, "Cancel request"))?;
    tester.checkpoint("Pending external wallet request before cancellation")?;
    tester.tap(&accessibility::action(platform, "Cancel request"))?;
    tester.expect_visible(&Locator::text(CHOOSE))?;
    tester.checkpoint("Pending wallet request cancelled and session unloaded")?;
    require(
        node.rpc
            .balance(&transfer.recipient)
            .map_err(|error| Error::Response(error.to_string()))?
            == transfer.after()?,
        "cancelled request changed the recipient",
    )?;
    require(
        node.rpc
            .balance(&transfer.payer)
            .map_err(|error| Error::Response(error.to_string()))?
            == payer_balance,
        "cancelled request changed the payer",
    )
}

fn reconnect(
    tester: &mut NativeTester,
    platform: Platform,
    external: &External<'_>,
    payer: &str,
) -> Result<()> {
    connect(tester, platform, external)?;
    require(
        address(tester)? == payer,
        "reconnected wallet authorized a different account",
    )?;
    tester.checkpoint("Fresh authorization after pending cancellation")?;
    Ok(())
}

fn disconnect(tester: &mut NativeTester, platform: Platform) -> Result<()> {
    tester.tap(&accessibility::action(platform, "Disconnect wallet"))?;
    tester.expect_visible(&Locator::text(CHOOSE))?;
    tester.checkpoint("External wallet disconnect acknowledged")
}

/// Retains only the guest's public receipt, its bounded diagnostics and the
/// fixed startup phases in the session evidence directory.
fn retain(tester: &NativeTester, external: &External<'_>, prefix: &str) -> Result<()> {
    for (name, path) in [
        ("receipt.txt", RECEIPT),
        ("diagnostics.txt", "files/journey-diagnostics.txt"),
        ("startup.txt", "files/journey-startup.txt"),
    ] {
        let bytes = android::read(external.adb, external.device, external.app, path)?;
        std::fs::write(tester.artifacts().join(format!("{prefix}-{name}")), bytes)?;
    }

    Ok(())
}

fn receipt(external: &External<'_>) -> Result<String> {
    let bytes = android::read(external.adb, external.device, external.app, RECEIPT)?;
    String::from_utf8(bytes)
        .map_err(|_| Error::Response("external wallet receipt is not UTF-8".into()))
}

fn verify_message(receipt: &str, payer: &str, generation: u64) -> Result<()> {
    let line = receipt
        .lines()
        .find(|line| line.starts_with("message "))
        .ok_or_else(|| Error::Response("message receipt missing".into()))?;
    let parts = line.split_whitespace().collect::<Vec<_>>();
    require(
        parts.len() == 4 && parts[1] == payer && parts[2] == format!("generation={generation}"),
        "message receipt does not name the authorized account",
    )?;
    let key = verifying_key(payer)?;
    let signature = signature(parts[3])?;
    key.verify_strict(format!("{MESSAGE} {generation}").as_bytes(), &signature)
        .map_err(|error| Error::Response(error.to_string()))
}

fn verify_rejected(receipt: &str, transfer: &Transfer) -> Result<()> {
    let line = receipt
        .lines()
        .find(|line| line.starts_with("rejected "))
        .ok_or_else(|| Error::Response("rejection receipt missing".into()))?;
    let amount = AMOUNT.to_string();
    let parts = line.split_whitespace().collect::<Vec<_>>();
    require(
        parts == ["rejected", transfer.payer.as_str(), transfer.recipient.as_str(), &amount],
        "rejection receipt does not match the reviewed transfer",
    )
}

fn verify_approved(receipt: &str, transfer: &Transfer, expected: &str) -> Result<()> {
    let line = receipt
        .lines()
        .find(|line| line.starts_with("approved "))
        .ok_or_else(|| Error::Response("approval receipt missing".into()))?;
    let parts = line.split_whitespace().collect::<Vec<_>>();
    require(
        parts.len() == 6
            && parts[1] == transfer.payer
            && parts[2] == transfer.recipient
            && parts[3] == AMOUNT.to_string()
            && parts[4] == expected
            && parts[5] == transfer.after()?.to_string(),
        "approval receipt does not match the executed transfer",
    )
}

fn verifying_key(address: &str) -> Result<VerifyingKey> {
    let bytes = public_bytes(address, ADDRESS_BYTES)?;
    let bytes: [u8; ADDRESS_BYTES] = bytes
        .try_into()
        .map_err(|_| Error::Response("wallet address length".into()))?;
    VerifyingKey::from_bytes(&bytes).map_err(|error| Error::Response(error.to_string()))
}

fn signature(text: &str) -> Result<Ed25519Signature> {
    let bytes = bs58::decode(text)
        .into_vec()
        .map_err(|error| Error::Response(error.to_string()))?;
    Ed25519Signature::from_slice(&bytes).map_err(|error| Error::Response(error.to_string()))
}
