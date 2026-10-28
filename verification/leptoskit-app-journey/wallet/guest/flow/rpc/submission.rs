//! One submission followed only by safe typed status and lifetime reads.
use super::{AppError, Error, Rpc, balance, sleep};
use solana_kit_client::{
    confirmation::{Confirmation, Policy, reconcile},
    methods::submission::{SendConfig, SendTransaction, SignedWire, WireEncoding},
};
use solana_kit_core::{Address, transaction::Transaction};

const POLL_MILLIS: u64 = 250;

/// Validate frozen signed wire before the sole submission. Reconciliation only
/// reads status and authenticated lifetime state; it never retries the transfer.
/// A confirmed status still needs an independently fetched recipient effect.
pub(super) async fn send(
    client: &Rpc,
    transaction: &Transaction,
    recipient: Address,
    before: u64,
    amount: u64,
) -> Result<u64, AppError> {
    let wire = transaction.encode(true)?;
    let expected = transaction
        .signatures()
        .first()
        .copied()
        .flatten()
        .ok_or(Error::MissingSignature)?;
    let policy = Policy::default();
    let lifetime = transaction.message().lifetime();
    let mut confirmation = Confirmation::new(expected, lifetime, policy)?;
    let config = SendConfig {
        preflight_commitment: Some(policy.commitment),
        ..Default::default()
    };
    let actual = client
        .call(SendTransaction {
            wire: SignedWire::new(&wire)?,
            encoding: WireEncoding::Base64,
            config,
        })
        .await?;

    if actual != expected {
        return Err(Error::ReportedSignature { expected, actual }.into());
    }

    loop {
        let events = reconcile(client, expected, lifetime, policy)
            .await
            .map_err(Error::Reconciliation)?;

        for event in events {
            if let Some(result) = confirmation.observe(event) {
                result?;

                return effect(client, recipient, before, amount).await;
            }
        }

        sleep(POLL_MILLIS).await?;
    }
}

async fn effect(
    client: &Rpc,
    recipient: Address,
    before: u64,
    amount: u64,
) -> Result<u64, AppError> {
    let expected = before.checked_add(amount).ok_or(Error::AmountOverflow)?;
    let actual = balance(client, recipient).await?;

    if actual != expected {
        return Err(Error::Effect { expected, actual }.into());
    }

    Ok(actual)
}
