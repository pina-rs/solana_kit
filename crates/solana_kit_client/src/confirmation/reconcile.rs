//! Read-only confirmation snapshots with a final freshness barrier for expiry.
use super::{Event, FactoryFailure, Failure, NonceError, Policy};
use crate::{
    Client, Transport,
    methods::{
        AccountInfo, Contextual, ReadConfig,
        node::GetEpochInfo,
        submission::{SignatureStatus, SignatureStatuses, StatusesConfig},
    },
};
use core::convert::Infallible;
use solana_kit_core::{
    Hash, Signature,
    accounts::{EncodedAccount, MaybeAccount},
    codecs::Limits,
    transaction::Lifetime,
};
const NONCE_ITEM_BUDGET: usize = 16;
type ReadFailure = FactoryFailure<Infallible>;

/// Reads status and authenticated lifetime state without submitting anything.
///
/// An invalid lifetime is decisive only after a subsequent status lookup reaches
/// its observed slot. A stale final lookup returns no expiry event. The caller
/// retries reads under its own deadline and cancellation. Status RPC supports
/// searchTransactionHistory, but has no minContextSlot request configuration.
/// Node-reported context is not cryptographic proof of an honest node.
pub async fn reconcile<T: Transport>(
    client: &Client<T>,
    signature: Signature,
    lifetime: Lifetime,
    policy: Policy,
) -> Result<Vec<Event>, ReadFailure> {
    let initial = status(client, signature).await?;
    let decisive = initial.context.slot >= policy.min_context_slot
        && initial
            .value
            .as_ref()
            .is_some_and(|value| status_settles(value, policy));
    let mut events = vec![Event::Status {
        signature,
        snapshot: initial,
    }];

    if decisive {
        return Ok(events);
    }

    let observation = lifetime_event(client, lifetime, policy).await?;

    if let Some(floor) = invalidation_slot(lifetime, &observation)
        && floor >= policy.min_context_slot
    {
        let final_status = status(client, signature).await?;
        let fresh = final_status.context.slot >= floor;
        events.push(Event::Status {
            signature,
            snapshot: final_status,
        });

        if !fresh {
            return Ok(events);
        }
    }

    events.push(observation);
    Ok(events)
}
async fn status<T: Transport>(
    client: &Client<T>,
    signature: Signature,
) -> Result<Contextual<Option<SignatureStatus>>, ReadFailure> {
    let signatures = [signature];
    let mut response = client
        .call(SignatureStatuses(
            &signatures,
            Some(StatusesConfig {
                search_transaction_history: Some(true),
            }),
        ))
        .await
        .map_err(rpc)?;
    let value = response
        .value
        .pop()
        .ok_or(FactoryFailure::Confirmation(Failure::ForeignObservation))?;

    Ok(Contextual {
        context: response.context,
        value,
    })
}
async fn lifetime_event<T: Transport>(
    client: &Client<T>,
    lifetime: Lifetime,
    policy: Policy,
) -> Result<Event, ReadFailure> {
    let read = ReadConfig {
        commitment: Some(policy.commitment),
        min_context_slot: Some(policy.min_context_slot),
    };

    match lifetime {
        Lifetime::Blockhash { .. } => Ok(Event::Epoch(
            client.call(GetEpochInfo(Some(read))).await.map_err(rpc)?,
        )),
        Lifetime::DurableNonce(nonce) => {
            let address = nonce.advance().nonce_account;
            let snapshot = client.call(AccountInfo(address, read)).await.map_err(rpc)?;
            let token = match snapshot.value {
                MaybeAccount::Missing(_) => None,
                MaybeAccount::Present(account) => Some(nonce_token(account)?),
            };

            Ok(Event::Nonce {
                address,
                slot: snapshot.context.slot,
                token,
            })
        }
        Lifetime::Unresolved(_) => Err(FactoryFailure::Confirmation(Failure::UnresolvedLifetime)),
    }
}
fn invalidation_slot(lifetime: Lifetime, event: &Event) -> Option<u64> {
    match (lifetime, event) {
        (
            Lifetime::Blockhash {
                last_valid_block_height,
                ..
            },
            Event::Epoch(info),
        ) if info.block_height > last_valid_block_height => Some(info.absolute_slot),
        (Lifetime::DurableNonce(nonce), Event::Nonce { slot, token, .. })
            if *token != Some(nonce.token()) =>
        {
            Some(*slot)
        }
        _ => None,
    }
}
fn rpc(error: crate::Error) -> ReadFailure {
    FactoryFailure::Confirmation(Failure::Rpc(error))
}
fn status_settles(status: &SignatureStatus, policy: Policy) -> bool {
    use crate::methods::{Commitment, history::ConfirmationStatus};

    if status.err.is_some() {
        return true;
    }

    let commitment = match status.confirmation_status {
        Some(ConfirmationStatus::Processed) => Commitment::Processed,
        Some(ConfirmationStatus::Confirmed) => Commitment::Confirmed,
        Some(ConfirmationStatus::Finalized) => Commitment::Finalized,
        None => return false,
    };

    super::commitment_comparator(commitment, policy.commitment) != core::cmp::Ordering::Less
}
fn nonce_token(account: EncodedAccount) -> Result<Hash, ReadFailure> {
    let address = account.address;

    if account.executable {
        return Err(FactoryFailure::Nonce(NonceError::Executable(address)));
    }

    let nonce = solana_kit_system::decode_nonce(
        account,
        Limits::new(solana_kit_system::NONCE_SIZE, NONCE_ITEM_BUDGET),
    )
    .map_err(|error| FactoryFailure::Nonce(NonceError::Account(error)))?;
    nonce
        .data
        .durable_data()
        .map(|data| data.blockhash)
        .ok_or(FactoryFailure::Nonce(NonceError::Unusable(address)))
}
