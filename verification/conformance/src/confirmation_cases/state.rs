//! Literal thresholds and independent event contexts use only public reducers.

use crate::require;
use solana_kit_client::{
    Presence,
    confirmation::{Confirmation, Event, Failure, Policy},
    methods::{
        Commitment, Context, Contextual,
        history::ConfirmationStatus,
        node::EpochInfo,
        submission::{
            ExecutionStatus, KnownTransactionFailure, SignatureStatus, TransactionFailure,
        },
    },
    subscriptions::SignatureInfo,
};
use solana_kit_core::{
    Address, Hash, Signature,
    transaction::{DurableNonce, Lifetime},
};

type Check = Result<(), &'static str>;
const MIN_SLOT: u64 = 20;
const LAST_VALID: u64 = 41;
const EXPIRED_HEIGHT: u64 = 42;
const NEW_NULL_SLOT: u64 = 500;
const STALE_SLOT: u64 = 19;

pub(super) fn signature() -> Signature {
    Signature::from_bytes([42; 64])
}

fn create(lifetime: Lifetime, commitment: Commitment) -> Result<Confirmation, &'static str> {
    Confirmation::new(
        signature(),
        lifetime,
        Policy {
            commitment,
            min_context_slot: MIN_SLOT,
        },
    )
    .map_err(|_| "valid confirmation rejected")
}

fn recent(commitment: Commitment) -> Result<Confirmation, &'static str> {
    create(
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([17; 32]),
            last_valid_block_height: LAST_VALID,
        },
        commitment,
    )
}

pub(super) fn notification(slot: u64, commitment: Commitment) -> Event {
    Event::Signature {
        signature: signature(),
        slot,
        commitment,
        value: SignatureInfo::Processed { err: None },
    }
}

fn lookup(slot: u64, value: Option<SignatureStatus>) -> Event {
    Event::Status {
        signature: signature(),
        snapshot: Contextual {
            context: Context {
                slot,
                api_version: Presence::Missing,
            },
            value,
        },
    }
}

fn epoch(slot: u64, height: u64) -> Event {
    Event::Epoch(EpochInfo {
        absolute_slot: slot,
        block_height: height,
        epoch: 0,
        slot_index: 0,
        slots_in_epoch: 100,
        transaction_count: None,
    })
}

pub(super) fn run() -> Check {
    let levels = [
        Commitment::Processed,
        Commitment::Confirmed,
        Commitment::Finalized,
    ];

    for (required, target) in levels.iter().enumerate() {
        for (observed, actual) in levels.iter().enumerate() {
            threshold(*target, *actual, observed >= required)?;
        }
    }

    nonce_ordering()?;
    status_contexts()
}

fn threshold(target: Commitment, actual: Commitment, sufficient: bool) -> Check {
    let mut strategy = recent(target)?;
    require(
        strategy.observe(epoch(MIN_SLOT, LAST_VALID)).is_none(),
        "height 41 must remain valid",
    )?;
    let outcome = strategy.observe(notification(MIN_SLOT, actual));
    require(
        outcome.is_some() == sufficient,
        "commitment threshold changed",
    )?;

    if sufficient {
        require(
            matches!(outcome, Some(Ok(()))),
            "sufficient commitment failed",
        )?;
        require(
            strategy.observe(epoch(MIN_SLOT, EXPIRED_HEIGHT)).is_none(),
            "late expiry replaced success",
        )?;
    } else {
        expired(&mut strategy)?;
    }

    let mut expired_first = recent(target)?;
    expired(&mut expired_first)?;
    require(
        expired_first
            .observe(notification(MIN_SLOT, actual))
            .is_none(),
        "late signature replaced expiry",
    )
}

fn expired(strategy: &mut Confirmation) -> Check {
    require(
        matches!(
            strategy.observe(epoch(MIN_SLOT, EXPIRED_HEIGHT)),
            Some(Err(Failure::BlockHeightExceeded {
                current: 42,
                last_valid: 41
            }))
        ),
        "height 42 must expire",
    )
}

fn nonce_ordering() -> Check {
    let address = Address::from_bytes([12; 32]);
    let token = Hash::from_bytes([13; 32]);
    let lifetime = Lifetime::DurableNonce(DurableNonce::new(token, address, address));
    let mut strategy = create(lifetime, Commitment::Finalized)?;
    require(
        strategy.observe(lookup(NEW_NULL_SLOT, None)).is_none(),
        "null lookup settled",
    )?;

    require(
        strategy
            .observe(Event::Nonce {
                address,
                slot: 100,
                token: Some(token),
            })
            .is_none(),
        "valid nonce settled",
    )?;
    require(
        strategy
            .observe(Event::Nonce {
                address,
                slot: 99,
                token: None,
            })
            .is_none(),
        "stale nonce replaced snapshot",
    )?;
    require(
        matches!(
            strategy.observe(notification(MIN_SLOT, Commitment::Finalized)),
            Some(Ok(()))
        ),
        "newer unrelated context suppressed signature",
    )?;
    require(
        strategy
            .observe(Event::Nonce {
                address,
                slot: 101,
                token: None,
            })
            .is_none(),
        "late nonce replaced success",
    )?;
    independent_nonce(lifetime, address)
}

fn independent_nonce(lifetime: Lifetime, address: Address) -> Check {
    let mut independent = create(lifetime, Commitment::Confirmed)?;
    require(
        independent.observe(lookup(NEW_NULL_SLOT, None)).is_none(),
        "null lookup settled nonce",
    )?;
    let outcome = independent.observe(Event::Nonce {
        address,
        slot: MIN_SLOT,
        token: None,
    });
    require(
        matches!(outcome, Some(Err(Failure::NonceAccountMissing(found))) if found == address),
        "null status suppressed independent nonce",
    )
}

fn status(failed: bool) -> SignatureStatus {
    SignatureStatus {
        slot: 1,
        confirmations: Some(2),
        err: failed.then_some(TransactionFailure::Known(
            KnownTransactionFailure::AccountNotFound,
        )),
        status: if failed {
            ExecutionStatus::Err(TransactionFailure::Known(
                KnownTransactionFailure::AccountNotFound,
            ))
        } else {
            ExecutionStatus::Ok(())
        },
        confirmation_status: Some(ConfirmationStatus::Confirmed),
    }
}

fn status_contexts() -> Check {
    let mut strategy = recent(Commitment::Confirmed)?;
    require(
        strategy.observe(lookup(NEW_NULL_SLOT, None)).is_none(),
        "null lookup settled status",
    )?;
    require(
        strategy
            .observe(lookup(STALE_SLOT, Some(status(true))))
            .is_none(),
        "stale execution error settled",
    )?;
    require(
        strategy
            .observe(epoch(STALE_SLOT, EXPIRED_HEIGHT))
            .is_none(),
        "stale expiry settled",
    )?;
    require(
        matches!(
            strategy.observe(lookup(MIN_SLOT, Some(status(false)))),
            Some(Ok(()))
        ),
        "transaction slot replaced evaluation context",
    )?;
    let mut failed = recent(Commitment::Finalized)?;
    require(
        matches!(
            failed.observe(lookup(MIN_SLOT, Some(status(true)))),
            Some(Err(Failure::Transaction(_)))
        ),
        "execution failure lost below commitment",
    )?;
    require(
        failed
            .observe(notification(MIN_SLOT, Commitment::Finalized))
            .is_none(),
        "success replaced execution failure",
    )
}
