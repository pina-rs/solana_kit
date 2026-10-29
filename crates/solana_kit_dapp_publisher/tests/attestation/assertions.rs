//! Independent JSON literals exercise the exact bytes presented to caller-owned signing.
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
pub fn ordered_json_preserves_full_width_slots_and_leading_request_zeroes() {
    let message = AttestationMessage::new(
        u64::MAX,
        Hash::from_bytes([0; 32]),
        RequestId::parse(REQUEST_ID).unwrap(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(message.bytes(), JSON);
    assert_eq!(message.request_id().as_str(), REQUEST_ID);
}

#[cfg_attr(test, test)]
pub fn request_identifiers_reject_length_unicode_and_injection_before_allocation() {
    for value in [
        "",
        "0000000000000000000000000000042",
        "000000000000000000000000000000042",
        "0000000000000000000000000000004a",
        "000000000000000000000000000000\"}",
        "１２３４５６７８９０１２３４５６",
    ] {
        assert_eq!(RequestId::parse(value), Err(Error::InvalidRequestId));
    }
}

#[cfg_attr(test, test)]
pub fn the_complete_attestation_output_is_admitted_before_json_allocation() {
    let bytes = JSON.len();
    let signed = bytes + SIGNATURE_BYTES;
    let encoded = signed.div_ceil(3) * 4;
    let items = bytes + signed + encoded;
    let build = |limits| {
        AttestationMessage::new(
            u64::MAX,
            Hash::from_bytes([0; 32]),
            RequestId::parse(REQUEST_ID).unwrap(),
            limits,
        )
    };
    assert!(build(Limits::new(encoded, items)).is_ok());
    assert!(matches!(
        build(Limits::new(encoded - 1, items)),
        Err(Error::LimitExceeded)
    ));
    assert!(matches!(
        build(Limits::new(encoded, items - 1)),
        Err(Error::LimitExceeded)
    ));
}
