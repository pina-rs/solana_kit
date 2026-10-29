//! Occupied caller slots are immutable even though core can replace a signature.
use super::checks::{Outcome, require};
use super::{Error, Version, support::*, validate_prepared_transaction};

#[cfg_attr(test, test)]
pub fn conflicting_signature_bytes_are_rejected_for_an_occupied_caller_slot() -> Outcome {
    for version in [Version::Legacy, Version::V0, Version::V1] {
        let mut fixture = RawTransaction::release();
        fixture.version = version;
        let message = fixture.message()?;

        let payer_signature = payer().sign(&message);
        let mint_signature = mint().sign(&message);
        let wire = fixture.wire(&[(0, payer_signature), (1, mint_signature)])?;
        let sealed = validate_prepared_transaction(&wire, release_policy(), LIMITS)
            .map_err(|_| "valid occupied caller transaction rejected")?;
        require(
            sealed.transaction().signatures().first().copied() == Some(Some(payer_signature)),
            "occupied caller signature changed before attachment",
        )?;

        // The occupancy rule rejects every differing byte sequence before verification.
        let result = sealed.attach_signature(mint_signature);
        require(
            result.err() == Some(Error::OccupiedSignatureMismatch(payer().address())),
            "conflicting occupied caller signature was not rejected",
        )?;
    }

    Ok(())
}

#[cfg_attr(test, test)]
pub fn identical_occupied_signatures_preserve_the_entire_wire_transaction() -> Outcome {
    for version in [Version::Legacy, Version::V0, Version::V1] {
        let mut fixture = RawTransaction::release();
        fixture.version = version;
        let message = fixture.message()?;

        let payer_signature = payer().sign(&message);
        let wire = fixture.wire(&[(0, payer_signature), (1, mint().sign(&message))])?;
        let sealed = validate_prepared_transaction(&wire, release_policy(), LIMITS)
            .map_err(|_| "valid occupied transaction rejected")?;
        let signed = sealed
            .attach_signature(payer_signature)
            .map_err(|_| "identical occupied caller signature rejected")?;
        signed
            .verify()
            .map_err(|_| "identical occupied signatures failed verification")?;

        require(
            signed
                .encode(true)
                .map_err(|_| "occupied transaction encoding failed")?
                == wire,
            "identical occupied signature changed wire bytes",
        )?;
    }

    Ok(())
}

#[cfg_attr(test, test)]
pub fn matching_occupied_signature_can_fill_a_duplicate_empty_identity_slot() -> Outcome {
    let mut fixture = RawTransaction::release();
    fixture.accounts.insert(1, payer().address());
    fixture.signers = 3;
    fixture.program = 3;
    let message = fixture.message()?;

    let payer_signature = payer().sign(&message);
    let mint_signature = mint().sign(&message);
    let wire = fixture.wire(&[(0, payer_signature), (2, mint_signature)])?;
    let sealed = validate_prepared_transaction(&wire, release_policy(), LIMITS)
        .map_err(|_| "valid duplicate signer proposal rejected")?;
    require(
        sealed.transaction().signatures().get(1).copied() == Some(None),
        "duplicate signer slot was not empty before attachment",
    )?;

    let signed = sealed
        .attach_signature(payer_signature)
        .map_err(|_| "matching signature did not fill duplicate signer")?;
    signed
        .verify()
        .map_err(|_| "filled duplicate signatures failed verification")?;
    let expected = fixture.wire(&[
        (0, payer_signature),
        (1, payer_signature),
        (2, mint_signature),
    ])?;
    require(
        signed
            .encode(true)
            .map_err(|_| "duplicate signer transaction encoding failed")?
            == expected,
        "duplicate signer attachment changed wire bytes",
    )
}
