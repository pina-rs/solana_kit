//! Submission preserves exact frozen transaction bytes and node errors.
use super::{Confirmation, Failure, Observer, Policy, WaitError, wait};
use crate::{
    Client, Transport,
    methods::submission::{SendConfig, SendTransaction, WireEncoding},
};
use alloc::{boxed::Box, vec::Vec};
use core::{
    future::{Future, poll_fn},
    task::Poll,
};
use solana_kit_core::{Signature, transaction::Transaction};

/// Sends a fully signed transaction, verifies the reported signature, then waits.
///
/// `observe` constructs the caller-owned observer after submission. It must
/// reconcile current state after subscription registration. Configured node-side
/// retries remain node-side only. The SDK never re-signs or retries submission.
/// Unknown received lifetime metadata fails before any transport operation.
/// Submission cancellation can be uncertain: node acceptance may already have
/// occurred. Dropping the exchange follows the transport's cancellation contract.
pub async fn send_and_confirm<T, O, F, C>(
    client: &Client<T>,
    transaction: &Transaction,
    mut config: SendConfig,
    policy: Policy,
    observe: F,
    cancellation: C,
) -> Result<Signature, WaitError>
where
    T: Transport,
    O: Observer,
    F: FnOnce(Signature) -> O,
    C: Future<Output = ()>,
{
    let (wire, expected, confirmation) = prepare_confirmation(transaction, policy)?;
    config.preflight_commitment.get_or_insert(policy.commitment);
    let mut cancellation = core::pin::pin!(cancellation);

    let actual = {
        let mut send = core::pin::pin!(client.call(SendTransaction {
            wire: &wire,
            encoding: WireEncoding::Base64,
            config,
        }));
        poll_fn(|cx| {
            if cancellation.as_mut().poll(cx).is_ready() {
                return Poll::Ready(Err(Failure::Cancelled));
            }

            send.as_mut().poll(cx).map_err(Failure::Rpc)
        })
        .await
        .map_err(WaitError::Outcome)?
    };

    if actual != expected {
        return Err(WaitError::Outcome(Failure::ReportedSignatureMismatch {
            expected,
            actual: Box::new(actual),
        }));
    }

    wait(confirmation, observe(actual), cancellation).await?;
    Ok(actual)
}

/// Validates the frozen wire and lifetime before any submission future is polled.
fn prepare_confirmation(
    transaction: &Transaction,
    policy: Policy,
) -> Result<(Vec<u8>, Signature, Confirmation), WaitError> {
    let wire = transaction
        .encode(true)
        .map_err(|error| WaitError::Outcome(Failure::Encoding(error)))?;
    let expected = transaction
        .signatures()
        .first()
        .and_then(|signature| *signature)
        .ok_or(WaitError::Outcome(Failure::MissingSignature))?;
    let confirmation = Confirmation::new(expected, transaction.message().lifetime(), policy)
        .map_err(WaitError::Outcome)?;

    Ok((wire, expected, confirmation))
}
