//! Independent event boundaries through the public confirmation API.
use core::cmp::Ordering;
use solana_kit_client::{
    Presence,
    confirmation::{Confirmation, Event, Failure, Policy, commitment_comparator, timeout_millis},
    methods::{Commitment, Context, Contextual, node::EpochInfo, submission::SignatureStatus},
    subscriptions::SignatureInfo,
};
use solana_kit_core::{
    Address, Hash, Signature,
    transaction::{DurableNonce, Lifetime},
};

fn signature() -> Signature {
    Signature::from_bytes([7; 64])
}
fn blockhash(height: u64) -> Confirmation {
    Confirmation::new(
        signature(),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([3; 32]),
            last_valid_block_height: height,
        },
        Policy {
            commitment: Commitment::Confirmed,
            min_context_slot: 10,
        },
    )
    .unwrap()
}
fn epoch(height: u64) -> Event {
    Event::Epoch(EpochInfo {
        absolute_slot: u64::MAX,
        block_height: height,
        epoch: 0,
        slot_index: 0,
        slots_in_epoch: 100,
        transaction_count: None,
    })
}
fn status(slot: u64, value: &str) -> Event {
    Event::Status {
        signature: signature(),
        snapshot: Contextual {
            context: Context {
                slot,
                api_version: Presence::Missing,
            },
            value: serde_json::from_str::<Option<SignatureStatus>>(value).unwrap(),
        },
    }
}
const CONFIRMED: &str = r#"{"slot":1,"confirmations":2,"err":null,"status":{"Ok":null},"confirmationStatus":"confirmed"}"#;
const PROCESSED: &str = r#"{"slot":1,"confirmations":0,"err":null,"status":{"Ok":null},"confirmationStatus":"processed"}"#;
const FAILED: &str = r#"{"slot":1,"confirmations":0,"err":"AccountNotFound","status":{"Err":"AccountNotFound"},"confirmationStatus":"processed"}"#;

#[test]
fn inclusive_expiry_and_maximum_integer_are_independent_boundaries() {
    let mut exact = blockhash(41);
    assert!(exact.observe(epoch(40)).is_none());
    assert!(exact.observe(epoch(41)).is_none());
    assert!(matches!(
        exact.observe(epoch(42)),
        Some(Err(Failure::BlockHeightExceeded {
            current: 42,
            last_valid: 41
        }))
    ));
    assert!(exact.observe(status(10, CONFIRMED)).is_none());

    let mut maximum = blockhash(u64::MAX);
    assert!(maximum.observe(epoch(u64::MAX)).is_none());
    assert!(matches!(
        maximum.observe(status(u64::MAX, CONFIRMED)),
        Some(Ok(()))
    ));
}

#[test]
fn receipts_absence_stale_context_and_low_commitment_remain_pending() {
    let mut confirmation = blockhash(100);
    let received = serde_json::from_str::<SignatureInfo>(r#""receivedSignature""#).unwrap();
    assert!(
        confirmation
            .observe(Event::Signature {
                signature: signature(),
                slot: 10,
                commitment: Commitment::Finalized,
                value: received
            })
            .is_none()
    );
    assert!(confirmation.observe(status(10, "null")).is_none());
    assert!(confirmation.observe(status(9, CONFIRMED)).is_none());
    assert!(confirmation.observe(status(10, PROCESSED)).is_none());
    assert!(matches!(
        confirmation.observe(status(10, FAILED)),
        Some(Err(Failure::Transaction(_)))
    ));
    assert!(confirmation.observe(Event::Timeout).is_none());
}

#[test]
fn nonce_replacement_requires_matching_identity_and_fresh_context() {
    let address = Address::from_bytes([1; 32]);
    let expected = Hash::from_bytes([2; 32]);
    let lifetime = Lifetime::DurableNonce(DurableNonce::new(expected, address, address));
    let mut confirmation = Confirmation::new(
        signature(),
        lifetime,
        Policy {
            commitment: Commitment::Confirmed,
            min_context_slot: 10,
        },
    )
    .unwrap();
    assert!(
        confirmation
            .observe(Event::Nonce {
                address,
                slot: 9,
                token: Some(Hash::from_bytes([3; 32]))
            })
            .is_none()
    );
    assert!(
        confirmation
            .observe(Event::Nonce {
                address,
                slot: 10,
                token: Some(expected)
            })
            .is_none()
    );
    assert!(confirmation.observe(epoch(u64::MAX)).is_none());
    assert!(matches!(
        confirmation.observe(Event::Nonce {
            address,
            slot: 10,
            token: Some(Hash::from_bytes([3; 32]))
        }),
        Some(Err(Failure::NonceInvalidated { .. }))
    ));

    let mut missing = Confirmation::new(signature(), lifetime, Policy::default()).unwrap();
    assert!(
        matches!(missing.observe(Event::Nonce { address, slot: 10, token: None }),
        Some(Err(Failure::NonceAccountMissing(found))) if found == address)
    );
    let mut foreign = Confirmation::new(signature(), lifetime, Policy::default()).unwrap();
    assert!(matches!(
        foreign.observe(Event::Nonce {
            address: Address::from_bytes([9; 32]),
            slot: 10,
            token: None
        }),
        Some(Err(Failure::ForeignObservation))
    ));
}

#[test]
fn unresolved_lifetime_never_invents_expiry_and_timeout_is_distinct() {
    assert!(matches!(
        Confirmation::new(
            signature(),
            Lifetime::Unresolved(Hash::from_bytes([0; 32])),
            Policy::default()
        ),
        Err(Failure::UnresolvedLifetime)
    ));
    assert!(matches!(
        blockhash(100).observe(Event::Timeout),
        Some(Err(Failure::Timeout))
    ));
}

#[test]
fn older_nonce_replacements_cannot_override_a_fresher_snapshot() {
    let address = Address::from_bytes([1; 32]);
    let token = Hash::from_bytes([2; 32]);
    let lifetime = Lifetime::DurableNonce(DurableNonce::new(token, address, address));
    let mut confirmation = Confirmation::new(signature(), lifetime, Policy::default()).unwrap();
    assert!(
        confirmation
            .observe(Event::Nonce {
                address,
                slot: 50,
                token: Some(token)
            })
            .is_none()
    );
    assert!(
        confirmation
            .observe(Event::Nonce {
                address,
                slot: 49,
                token: Some(Hash::from_bytes([3; 32]))
            })
            .is_none()
    );
    assert!(matches!(
        confirmation.observe(Event::Nonce {
            address,
            slot: 50,
            token: Some(Hash::from_bytes([3; 32]))
        }),
        Some(Err(Failure::NonceInvalidated { .. }))
    ));
    assert!(confirmation.is_settled());
}

#[test]
fn newer_lookup_context_does_not_suppress_older_committed_notification() {
    let mut confirmation = blockhash(100);
    assert!(confirmation.observe(status(100, "null")).is_none());
    let committed = Event::Signature {
        signature: signature(),
        slot: 10,
        commitment: Commitment::Confirmed,
        value: SignatureInfo::Processed { err: None },
    };
    assert!(matches!(confirmation.observe(committed), Some(Ok(()))));
}

#[test]
fn canonical_commitment_matrix_and_timeout_policy() {
    let levels = [
        Commitment::Processed,
        Commitment::Confirmed,
        Commitment::Finalized,
    ];

    for (left, a) in levels.iter().enumerate() {
        for (right, b) in levels.iter().enumerate() {
            assert_eq!(commitment_comparator(*a, *b), left.cmp(&right));
        }
    }

    assert_eq!(
        commitment_comparator(Commitment::Finalized, Commitment::Processed),
        Ordering::Greater
    );
    assert_eq!(timeout_millis(Commitment::Processed), 30_000);
    assert_eq!(timeout_millis(Commitment::Confirmed), 60_000);
    assert_eq!(timeout_millis(Commitment::Finalized), 60_000);
}
