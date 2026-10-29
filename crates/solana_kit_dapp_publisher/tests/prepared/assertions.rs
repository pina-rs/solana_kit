//! The caller signs a sealed exact message; policy failures precede that operation.
#[path = "occupied_slots.rs"]
pub(super) mod occupied_slots;
#[path = "policy_cases.rs"]
pub(super) mod policy_cases;
#[path = "../support/mod.rs"]
mod support;

use solana_kit_core::{
    Address, Hash, Signature,
    codecs::Limits,
    transaction::{Transaction, Version},
};
use solana_kit_dapp_publisher::{Error, PublicationPolicy, validate_prepared_transaction};
use support::*;

#[cfg_attr(test, test)]
pub fn legacy_v0_and_v1_preserve_independent_message_bytes_and_existing_mint_signature() {
    for version in [Version::Legacy, Version::V0, Version::V1] {
        let mut fixture = RawTransaction::release();
        fixture.version = version;
        let wire = fixture.signed_mint();
        let sealed = validate_prepared_transaction(&wire, release_policy(), LIMITS).unwrap();
        assert_eq!(sealed.transaction().message().bytes(), fixture.message());
        assert_eq!(sealed.policy(), release_policy());

        let request = sealed.signing_request();
        assert_eq!(request.signer, payer().address());
        let signature = payer().sign(request.message);
        let signed = sealed.attach_signature(signature).unwrap();
        signed.verify().unwrap();

        assert_eq!(signed.message().bytes(), fixture.message());
        assert_eq!(
            signed.signatures()[1],
            Some(mint().sign(&fixture.message()))
        );
        let expected = fixture.wire(&[(0, signature), (1, mint().sign(&fixture.message()))]);
        assert_eq!(signed.encode(true).unwrap(), expected);
    }
}

#[cfg_attr(test, test)]
pub fn common_expectations_reject_changed_blockhash_payer_and_nonrequired_local_signer() {
    let wire = RawTransaction::release().signed_mint();
    let checks = [
        (
            solana_kit_dapp_publisher::ExpectedTransaction {
                blockhash: Hash::from_bytes([4; 32]),
                ..expected()
            },
            Error::BlockhashMismatch,
        ),
        (
            solana_kit_dapp_publisher::ExpectedTransaction {
                payer: Address::from_bytes([4; 32]),
                ..expected()
            },
            Error::PayerMismatch,
        ),
        (
            solana_kit_dapp_publisher::ExpectedTransaction {
                signer: Address::from_bytes([4; 32]),
                ..expected()
            },
            Error::SignerNotRequired,
        ),
    ];

    for (expected, error) in checks {
        let policy = PublicationPolicy::ReleaseMint {
            expected,
            mint: mint().address(),
            collection: COLLECTION,
        };
        assert_rejected(&wire, policy, LIMITS, error);
    }
}

#[cfg_attr(test, test)]
pub fn invalid_existing_and_new_signatures_cannot_cross_the_seal() {
    let fixture = RawTransaction::release();
    let invalid = Signature::from_bytes([5; 64]);
    let wire = fixture.wire(&[(1, invalid)]);
    assert_rejected(
        &wire,
        release_policy(),
        LIMITS,
        Error::InvalidSignature(mint().address()),
    );

    let sealed =
        validate_prepared_transaction(&fixture.signed_mint(), release_policy(), LIMITS).unwrap();
    let result = sealed.attach_signature(mint().sign(&fixture.message()));
    assert_eq!(
        result.unwrap_err(),
        Error::InvalidSignature(payer().address())
    );
}

#[cfg_attr(test, test)]
pub fn each_transaction_allocation_is_admitted_before_parsing_or_nested_decoding() {
    let wire = RawTransaction::release().signed_mint();
    assert!(matches!(
        validate_prepared_transaction(
            &wire,
            release_policy(),
            Limits::new(wire.len() - 1, LIMITS.max_items)
        ),
        Err(Error::LimitExceeded)
    ));
    assert!(matches!(
        validate_prepared_transaction(
            &wire,
            release_policy(),
            Limits::new(LIMITS.max_bytes, wire.len() * 3 - 1)
        ),
        Err(Error::LimitExceeded)
    ));
    assert!(
        validate_prepared_transaction(
            &wire,
            release_policy(),
            Limits::new(LIMITS.max_bytes, wire.len() * 3)
        )
        .is_ok()
    );
    assert!(matches!(
        validate_prepared_transaction(
            &[0; 4097],
            release_policy(),
            Limits::new(8192, LIMITS.max_items)
        ),
        Err(Error::Transaction(_))
    ));
    assert!(matches!(
        validate_prepared_transaction(&[255], release_policy(), LIMITS),
        Err(Error::Transaction(_))
    ));
}

#[cfg_attr(test, test)]
pub fn metadata_allocations_cannot_reuse_the_transaction_reservation() {
    let mut fixture = RawTransaction::release();
    fixture.data.splice(2..6, [1, 0, 0, 0, b'N']);
    let wire = fixture.signed_mint();
    let items = wire.len() * 3;
    assert!(matches!(
        validate_prepared_transaction(
            &wire,
            release_policy(),
            Limits::new(LIMITS.max_bytes, items)
        ),
        Err(Error::Codec(solana_kit_core::codecs::Error::LimitExceeded))
    ));
    assert!(
        validate_prepared_transaction(
            &wire,
            release_policy(),
            Limits::new(LIMITS.max_bytes, items + 1)
        )
        .is_ok()
    );
}

#[cfg_attr(test, test)]
pub fn incomplete_collections_are_not_reported_as_fully_signed_transactions() {
    let mut fixture = RawTransaction::verification();
    fixture.accounts.insert(1, mint().address());
    fixture.signers = 2;
    fixture.program = 2;
    let sealed = validate_prepared_transaction(
        &fixture.wire(&[]),
        verification_policy(mint().address()),
        LIMITS,
    )
    .unwrap();
    let signature = payer().sign(sealed.signing_request().message);
    assert!(matches!(
        sealed.attach_signature(signature),
        Err(Error::Transaction(
            solana_kit_core::transaction::Error::MissingSignature
        ))
    ));
}

#[cfg_attr(test, test)]
pub fn lookup_descriptors_require_a_future_authenticated_snapshot_contract() {
    let mut fixture = RawTransaction::release();
    fixture.version = Version::V0;
    fixture.lookup = true;
    assert!(matches!(
        validate_prepared_transaction(&fixture.signed_mint(), release_policy(), LIMITS),
        Err(Error::UnresolvedLookups)
    ));
}

#[cfg_attr(test, test)]
pub fn attached_signatures_do_not_change_the_unsigned_slot_encoding_contract() {
    let fixture = RawTransaction::release();
    let unsigned = Transaction::decode(&fixture.wire(&[])).unwrap();
    assert_eq!(unsigned.signatures(), &[None, None]);
    assert!(matches!(
        validate_prepared_transaction(&fixture.wire(&[]), release_policy(), LIMITS),
        Err(Error::MissingMintSignature)
    ));
}
