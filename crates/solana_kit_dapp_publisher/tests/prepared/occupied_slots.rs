//! Occupied caller slots are immutable even though core can replace a signature.
use super::{Error, Version, support::*, validate_prepared_transaction};

#[cfg_attr(test, test)]
pub fn conflicting_signature_bytes_are_rejected_for_an_occupied_caller_slot() {
    for version in [Version::Legacy, Version::V0, Version::V1] {
        let mut fixture = RawTransaction::release();
        fixture.version = version;
        let message = fixture.message();

        let payer_signature = payer().sign(&message);
        let mint_signature = mint().sign(&message);
        let wire = fixture.wire(&[(0, payer_signature), (1, mint_signature)]);
        let sealed = validate_prepared_transaction(&wire, release_policy(), LIMITS).unwrap();
        assert_eq!(sealed.transaction().signatures()[0], Some(payer_signature));

        // The occupancy rule rejects every differing byte sequence before verification.
        let result = sealed.attach_signature(mint_signature);
        assert_eq!(
            result.unwrap_err(),
            Error::OccupiedSignatureMismatch(payer().address())
        );
    }
}

#[cfg_attr(test, test)]
pub fn identical_occupied_signatures_preserve_the_entire_wire_transaction() {
    for version in [Version::Legacy, Version::V0, Version::V1] {
        let mut fixture = RawTransaction::release();
        fixture.version = version;
        let message = fixture.message();

        let payer_signature = payer().sign(&message);
        let wire = fixture.wire(&[(0, payer_signature), (1, mint().sign(&message))]);
        let sealed = validate_prepared_transaction(&wire, release_policy(), LIMITS).unwrap();
        let signed = sealed.attach_signature(payer_signature).unwrap();
        signed.verify().unwrap();

        assert_eq!(signed.encode(true).unwrap(), wire);
    }
}

#[cfg_attr(test, test)]
pub fn matching_occupied_signature_can_fill_a_duplicate_empty_identity_slot() {
    let mut fixture = RawTransaction::release();
    fixture.accounts.insert(1, payer().address());
    fixture.signers = 3;
    fixture.program = 3;
    let message = fixture.message();

    let payer_signature = payer().sign(&message);
    let mint_signature = mint().sign(&message);
    let wire = fixture.wire(&[(0, payer_signature), (2, mint_signature)]);
    let sealed = validate_prepared_transaction(&wire, release_policy(), LIMITS).unwrap();
    assert_eq!(sealed.transaction().signatures()[1], None);

    let signed = sealed.attach_signature(payer_signature).unwrap();
    signed.verify().unwrap();
    let expected = fixture.wire(&[
        (0, payer_signature),
        (1, payer_signature),
        (2, mint_signature),
    ]);
    assert_eq!(signed.encode(true).unwrap(), expected);
}
