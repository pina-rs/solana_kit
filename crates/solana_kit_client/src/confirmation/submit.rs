//! One-shot guarded submission through the same shared confirmation operation.
use super::shared::{Subject, check_stop, failed, finish, flush, observe};
use super::{
    ConfirmationWatch, FactoryConfig, FactoryError, FactoryFailure, Failure, SessionAccess,
};
use crate::{
    Client, Transport,
    methods::submission::{SendConfig, SendTransaction, SignedWire, WireEncoding},
    subscriptions::{Clock, TextSocket},
};
use core::future::{Future, poll_fn};
use solana_kit_core::{Signature, transaction::Transaction};

/// Registers, sends frozen validated bytes exactly once, then confirms and cleans up.
///
/// The original submission error is retained even if acceptance is uncertain.
/// Cancellation or deadline during send is also uncertain. Neither path retries
/// submission. This operation never closes the application's shared Session.
pub async fn submit_and_confirm<T, S, A, C, X>(
    client: &Client<T>,
    access: &A,
    clock: &C,
    transaction: &Transaction,
    send_config: SendConfig,
    config: FactoryConfig,
    cancellation: X,
) -> Result<Signature, FactoryError<S::Error>>
where
    T: Transport,
    S: TextSocket,
    A: SessionAccess<S>,
    C: Clock,
    X: Future<Output = ()>,
{
    let (wire, signature, mut watch) = prepare_watch::<S, A>(access, transaction, config)?;
    let lifetime = transaction.message().lifetime();
    let mut cancellation = core::pin::pin!(cancellation);
    let outcome = async {
        flush(access, clock, config.deadline, cancellation.as_mut()).await?;
        let actual = send_once(
            client,
            clock,
            &wire,
            send_config,
            config,
            cancellation.as_mut(),
        )
        .await?;

        expect_signature(signature, actual)?;

        observe(
            client,
            access,
            clock,
            &mut watch,
            Subject {
                signature,
                lifetime,
                config,
            },
            cancellation.as_mut(),
        )
        .await
    }
    .await;
    finish(access, clock, &mut watch, config, outcome).await?;
    Ok(signature)
}

// All transaction validation and registration happen synchronously before the first I/O poll.
fn prepare_watch<S, A>(
    access: &A,
    transaction: &Transaction,
    config: FactoryConfig,
) -> Result<(Vec<u8>, Signature, ConfirmationWatch), FactoryError<S::Error>>
where
    S: TextSocket,
    A: SessionAccess<S>,
{
    let (wire, signature, reducer) = super::send::prepare(transaction, config.policy)
        .map_err(|error| failed(FactoryFailure::Confirmation(error)))?;
    let watch = access
        .with_session(|session| {
            ConfirmationWatch::register_reducer(session, signature, reducer, config)
        })
        .map_err(|error| failed(FactoryFailure::Session(error)))?
        .map_err(failed)?;

    Ok((wire, signature, watch))
}

fn expect_signature<E>(expected: Signature, actual: Signature) -> Result<(), FactoryFailure<E>> {
    if actual != expected {
        return Err(FactoryFailure::Confirmation(
            Failure::ReportedSignatureMismatch {
                expected,
                actual: Box::new(actual),
            },
        ));
    }

    Ok(())
}

// Cancellation and deadline win before polling the sole, never-retried submission future.
async fn send_once<T, C, X, E>(
    client: &Client<T>,
    clock: &C,
    wire: &[u8],
    mut config: SendConfig,
    factory: FactoryConfig,
    mut cancellation: core::pin::Pin<&mut X>,
) -> Result<Signature, FactoryFailure<E>>
where
    T: Transport,
    C: Clock,
    X: Future<Output = ()>,
{
    config
        .preflight_commitment
        .get_or_insert(factory.policy.commitment);
    let mut timeout = core::pin::pin!(clock.sleep_until(factory.deadline));
    let wire = SignedWire::new(wire)
        .map_err(|error| FactoryFailure::Confirmation(Failure::Encoding(error)))?;
    let mut send = core::pin::pin!(client.call(SendTransaction {
        wire,
        encoding: WireEncoding::Base64,
        config,
    }));

    poll_fn(|cx| {
        check_stop(cancellation.as_mut(), timeout.as_mut(), cx)?;

        send.as_mut()
            .poll(cx)
            .map_err(|error| FactoryFailure::Confirmation(Failure::submission_rpc(error)))
    })
    .await
}
