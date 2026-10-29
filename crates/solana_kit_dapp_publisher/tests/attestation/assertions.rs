//! Independent JSON literals exercise the exact bytes presented to caller-owned signing.
use super::checks::{Outcome, require};
use solana_kit_core::{Hash, codecs::Limits};
use solana_kit_dapp_publisher::{AttestationMessage, Error, RequestId};

pub(super) const REQUEST_ID: &str = "00000000000000000000000000000042";
pub(super) const SIGNATURE_BYTES: usize = 64;
pub(super) const LIMITS: Limits = Limits::new(1024, 4096);
pub(super) const JSON: &[u8] = concat!(
    "{\"slot_number\":18446744073709551615,",
    "\"blockhash\":\"11111111111111111111111111111111\",",
    "\"request_unique_id\":\"00000000000000000000000000000042\"}"
)
.as_bytes();

#[cfg_attr(test, test)]
pub fn ordered_json_preserves_full_width_slots_and_leading_request_zeroes() -> Outcome {
    ordered_json_matches(JSON)
}

/// The portable continuation control changes only this independent expected literal.
pub fn ordered_json_matches(expected: &[u8]) -> Outcome {
    let message = AttestationMessage::new(
        u64::MAX,
        Hash::from_bytes([0; 32]),
        RequestId::parse(REQUEST_ID).map_err(|_| "valid attestation request ID rejected")?,
        LIMITS,
    )
    .map_err(|_| "valid ordered attestation message rejected")?;
    require(
        message.bytes() == expected,
        "ordered attestation JSON mismatch",
    )?;
    require(
        message.request_id().as_str() == REQUEST_ID,
        "leading request ID zeroes changed",
    )
}

#[cfg_attr(test, test)]
pub fn request_identifiers_reject_length_unicode_and_injection_before_allocation() -> Outcome {
    for value in [
        "",
        "0000000000000000000000000000042",
        "000000000000000000000000000000042",
        "0000000000000000000000000000004a",
        "000000000000000000000000000000\"}",
        "１２３４５６７８９０１２３４５６",
    ] {
        require(
            RequestId::parse(value) == Err(Error::InvalidRequestId),
            "invalid request ID was not rejected",
        )?;
    }

    Ok(())
}

#[cfg_attr(test, test)]
pub fn the_complete_attestation_output_is_admitted_before_json_allocation() -> Outcome {
    let bytes = JSON.len();
    let signed = bytes + SIGNATURE_BYTES;
    let encoded = signed.div_ceil(3) * 4;
    let items = bytes + signed + encoded;
    let request_id =
        RequestId::parse(REQUEST_ID).map_err(|_| "valid budget request ID rejected")?;
    let build = |limits| {
        AttestationMessage::new(
            u64::MAX,
            Hash::from_bytes([0; 32]),
            request_id.clone(),
            limits,
        )
    };
    require(
        build(Limits::new(encoded, items)).is_ok(),
        "exact attestation budget rejected",
    )?;
    require(
        matches!(
            build(Limits::new(encoded - 1, items)),
            Err(Error::LimitExceeded)
        ),
        "attestation byte cap was not enforced",
    )?;
    require(
        matches!(
            build(Limits::new(encoded, items - 1)),
            Err(Error::LimitExceeded)
        ),
        "attestation aggregate item cap was not enforced",
    )
}
