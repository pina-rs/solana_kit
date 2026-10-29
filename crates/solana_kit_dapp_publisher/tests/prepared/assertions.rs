//! The caller signs a sealed exact message; policy failures precede that operation.
#[path = "../support/checks.rs"]
mod checks;
#[path = "occupied_slots.rs"]
pub(super) mod occupied_slots;
#[path = "policy_cases.rs"]
pub(super) mod policy_cases;
#[path = "../support/mod.rs"]
mod support;

use checks::{Outcome, require};
use solana_kit_core::{
    Address, Hash, Signature,
    codecs::Limits,
    transaction::{Transaction, Version},
};
use solana_kit_dapp_publisher::{Error, PublicationPolicy, validate_prepared_transaction};
use support::*;

#[cfg_attr(test, test)]
pub fn legacy_v0_and_v1_preserve_independent_message_bytes_and_existing_mint_signature() -> Outcome
{
    for version in [Version::Legacy, Version::V0, Version::V1] {
        let mut fixture = RawTransaction::release();
        fixture.version = version;
        let wire = fixture.signed_mint()?;
        let sealed = validate_prepared_transaction(&wire, release_policy(), LIMITS)
            .map_err(|_| "valid release envelope rejected")?;
        let message = fixture.message()?;
        require(
            sealed.transaction().message().bytes() == message.as_slice(),
            "sealed message bytes changed",
        )?;
        require(
            sealed.policy() == release_policy(),
            "sealed publication policy changed",
        )?;

        let request = sealed.signing_request();
        require(
            request.signer == payer().address(),
            "prepared signing request changed signer",
        )?;
        let signature = payer().sign(request.message);
        let signed = sealed
            .attach_signature(signature)
            .map_err(|_| "valid release caller signature rejected")?;
        signed
            .verify()
            .map_err(|_| "attached release signatures did not verify")?;

        require(
            signed.message().bytes() == message.as_slice(),
            "attached message bytes changed",
        )?;
        require(
            signed.signatures().get(1).copied() == Some(Some(mint().sign(&message))),
            "existing mint signature changed",
        )?;
        let expected = fixture.wire(&[(0, signature), (1, mint().sign(&message))])?;
        require(
            signed
                .encode(true)
                .map_err(|_| "signed release encoding failed")?
                == expected,
            "signed release wire bytes changed",
        )?;
    }

    Ok(())
}

#[cfg_attr(test, test)]
pub fn common_expectations_reject_changed_blockhash_payer_and_nonrequired_local_signer() -> Outcome
{
    let wire = RawTransaction::release().signed_mint()?;
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
        assert_rejected(&wire, policy, LIMITS, error)?;
    }

    Ok(())
}

#[cfg_attr(test, test)]
pub fn invalid_existing_and_new_signatures_cannot_cross_the_seal() -> Outcome {
    let fixture = RawTransaction::release();
    let invalid = Signature::from_bytes([5; 64]);
    let wire = fixture.wire(&[(1, invalid)])?;
    assert_rejected(
        &wire,
        release_policy(),
        LIMITS,
        Error::InvalidSignature(mint().address()),
    )?;

    let sealed = validate_prepared_transaction(&fixture.signed_mint()?, release_policy(), LIMITS)
        .map_err(|_| "valid mint signature rejected before negative caller check")?;
    let result = sealed.attach_signature(mint().sign(&fixture.message()?));
    require(
        result.err() == Some(Error::InvalidSignature(payer().address())),
        "wrong caller signature crossed the seal",
    )
}

#[cfg_attr(test, test)]
pub fn each_transaction_allocation_is_admitted_before_parsing_or_nested_decoding() -> Outcome {
    let wire = RawTransaction::release().signed_mint()?;
    require(
        matches!(
            validate_prepared_transaction(
                &wire,
                release_policy(),
                Limits::new(wire.len() - 1, LIMITS.max_items)
            ),
            Err(Error::LimitExceeded)
        ),
        "transaction byte cap was not enforced",
    )?;
    require(
        matches!(
            validate_prepared_transaction(
                &wire,
                release_policy(),
                Limits::new(LIMITS.max_bytes, wire.len() * 3 - 1)
            ),
            Err(Error::LimitExceeded)
        ),
        "transaction aggregate reservation was not enforced",
    )?;
    require(
        validate_prepared_transaction(
            &wire,
            release_policy(),
            Limits::new(LIMITS.max_bytes, wire.len() * 3),
        )
        .is_ok(),
        "exact transaction reservation rejected",
    )?;
    require(
        matches!(
            validate_prepared_transaction(
                &[0; 4097],
                release_policy(),
                Limits::new(8192, LIMITS.max_items)
            ),
            Err(Error::Transaction(_))
        ),
        "oversized invalid transaction did not reject",
    )?;
    require(
        matches!(
            validate_prepared_transaction(&[255], release_policy(), LIMITS),
            Err(Error::Transaction(_))
        ),
        "truncated transaction did not reject",
    )
}

#[cfg_attr(test, test)]
pub fn metadata_allocations_cannot_reuse_the_transaction_reservation() -> Outcome {
    let mut fixture = RawTransaction::release();
    fixture.data.splice(2..6, [1, 0, 0, 0, b'N']);
    let wire = fixture.signed_mint()?;
    let items = wire.len() * 3;
    require(
        matches!(
            validate_prepared_transaction(
                &wire,
                release_policy(),
                Limits::new(LIMITS.max_bytes, items)
            ),
            Err(Error::Codec(solana_kit_core::codecs::Error::LimitExceeded))
        ),
        "metadata allocation reused transaction reservation",
    )?;
    require(
        validate_prepared_transaction(
            &wire,
            release_policy(),
            Limits::new(LIMITS.max_bytes, items + 1),
        )
        .is_ok(),
        "exact metadata allocation allowance rejected",
    )
}

#[cfg_attr(test, test)]
pub fn incomplete_collections_are_not_reported_as_fully_signed_transactions() -> Outcome {
    let mut fixture = RawTransaction::verification()?;
    fixture.accounts.insert(1, mint().address());
    fixture.signers = 2;
    fixture.program = 2;
    let sealed = validate_prepared_transaction(
        &fixture.wire(&[])?,
        verification_policy(mint().address()),
        LIMITS,
    )
    .map_err(|_| "valid incomplete collection proposal rejected")?;
    let signature = payer().sign(sealed.signing_request().message);
    require(
        matches!(
            sealed.attach_signature(signature),
            Err(Error::Transaction(
                solana_kit_core::transaction::Error::MissingSignature
            ))
        ),
        "incomplete collection accepted as fully signed",
    )
}

#[cfg_attr(test, test)]
pub fn lookup_descriptors_require_a_future_authenticated_snapshot_contract() -> Outcome {
    let mut fixture = RawTransaction::release();
    fixture.version = Version::V0;
    fixture.lookup = true;
    require(
        matches!(
            validate_prepared_transaction(&fixture.signed_mint()?, release_policy(), LIMITS),
            Err(Error::UnresolvedLookups)
        ),
        "unresolved lookup descriptor accepted",
    )
}

#[cfg_attr(test, test)]
pub fn attached_signatures_do_not_change_the_unsigned_slot_encoding_contract() -> Outcome {
    let fixture = RawTransaction::release();
    let unsigned =
        Transaction::decode(&fixture.wire(&[])?).map_err(|_| "unsigned fixture decoding failed")?;
    require(
        unsigned.signatures() == &[None, None],
        "unsigned slots did not decode as empty",
    )?;
    require(
        matches!(
            validate_prepared_transaction(&fixture.wire(&[])?, release_policy(), LIMITS),
            Err(Error::MissingMintSignature)
        ),
        "missing mint signature was not rejected",
    )
}
