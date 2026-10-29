//! Each source-policy invariant has a targeted negative fixture.
use super::checks::{Outcome, require};
use super::{Address, Error, PublicationPolicy, support::*, validate_prepared_transaction};
use alloc::vec;
use solana_kit_core::address_constants::COMPUTE_BUDGET_PROGRAM;

#[cfg_attr(test, test)]
pub fn only_canonical_outer_programs_are_allowed_and_metadata_is_required() -> Outcome {
    let mut fixture = RawTransaction::release();
    fixture.accounts[2] = Address::from_bytes([9; 32]);
    assert_rejected(
        &fixture.signed_mint()?,
        release_policy(),
        LIMITS,
        Error::ProgramNotAllowed(fixture.accounts[2]),
    )?;
    fixture.accounts[2] = COMPUTE_BUDGET_PROGRAM;
    require(
        matches!(
            validate_prepared_transaction(&fixture.signed_mint()?, release_policy(), LIMITS),
            Err(Error::MissingMetadataInstruction)
        ),
        "missing metadata instruction accepted",
    )
}

#[cfg_attr(test, test)]
pub fn mint_identity_and_collection_must_match_independent_caller_expectations() -> Outcome {
    let fixture = RawTransaction::release();
    let wire = fixture.signed_mint()?;
    let wrong_mint = PublicationPolicy::ReleaseMint {
        expected: expected(),
        mint: Address::from_bytes([4; 32]),
        collection: COLLECTION,
    };
    require(
        matches!(
            validate_prepared_transaction(&wire, wrong_mint, LIMITS),
            Err(Error::SignerSetMismatch)
        ),
        "wrong mint signer set accepted",
    )?;
    let wrong_collection = PublicationPolicy::ReleaseMint {
        expected: expected(),
        mint: mint().address(),
        collection: Address::from_bytes([4; 32]),
    };
    require(
        matches!(
            validate_prepared_transaction(&wire, wrong_collection, LIMITS),
            Err(Error::CollectionMismatch)
        ),
        "wrong collection identity accepted",
    )
}

#[cfg_attr(test, test)]
pub fn missing_truncated_unknown_and_trailing_create_data_reject_before_signing() -> Outcome {
    let cases = [
        (vec![18], Error::CollectionMismatch),
        (
            vec![42],
            Error::Codec(solana_kit_core::codecs::Error::Truncated),
        ),
        (
            vec![42, 99],
            Error::Codec(solana_kit_core::codecs::Error::InvalidDiscriminator),
        ),
    ];

    for (data, error) in cases {
        let mut fixture = RawTransaction::release();
        fixture.data = data;
        assert_rejected(&fixture.signed_mint()?, release_policy(), LIMITS, error)?;
    }

    let mut fixture = RawTransaction::release();
    fixture.data.push(0);
    require(
        matches!(
            validate_prepared_transaction(&fixture.signed_mint()?, release_policy(), LIMITS),
            Err(Error::Codec(solana_kit_core::codecs::Error::TrailingBytes))
        ),
        "trailing Create data accepted",
    )
}

#[cfg_attr(test, test)]
pub fn a_missing_optional_collection_cannot_match_the_expected_collection() -> Outcome {
    const COLLECTION_OPTION: usize = 20;
    const COLLECTION_END: usize = 54;
    let mut fixture = RawTransaction::release();
    fixture.data.splice(COLLECTION_OPTION..COLLECTION_END, [0]);
    assert_rejected(
        &fixture.signed_mint()?,
        release_policy(),
        LIMITS,
        Error::CollectionMismatch,
    )
}

#[cfg_attr(test, test)]
pub fn collection_verification_requires_each_derived_and_explicit_account() -> Outcome {
    let fixture = RawTransaction::verification()?;
    let policy = verification_policy(payer().address());
    let sealed = validate_prepared_transaction(&fixture.wire(&[])?, policy, LIMITS)
        .map_err(|_| "valid collection verification proposal rejected")?;
    let signature = payer().sign(sealed.signing_request().message);
    require(
        sealed.attach_signature(signature).is_ok(),
        "valid collection caller signature rejected",
    )?;

    for index in 2..fixture.accounts.len() {
        let mut missing = RawTransaction::verification()?;
        let address = missing.accounts.remove(index);
        assert_rejected(
            &missing.wire(&[])?,
            policy,
            LIMITS,
            Error::MissingAccount(address),
        )?;
    }

    Ok(())
}

#[cfg_attr(test, test)]
pub fn a_changed_existing_signature_is_rejected_even_when_collection_policy_still_matches()
-> Outcome {
    let fixture = RawTransaction::verification()?;
    let signature = payer().sign(b"a different message");
    let wire = fixture.wire(&[(0, signature)])?;
    assert_rejected(
        &wire,
        verification_policy(payer().address()),
        LIMITS,
        Error::InvalidSignature(payer().address()),
    )
}
