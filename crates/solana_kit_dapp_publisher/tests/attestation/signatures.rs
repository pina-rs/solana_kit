//! The same detached-signature assertions run natively and on every portable host.

use super::assertions::{JSON, LIMITS, REQUEST_ID, SIGNATURE_BYTES};
use solana_kit_core::{
    Address, Hash, Signature,
    codecs::{Base64, Encoder},
    crypto::Keypair,
};
use solana_kit_dapp_publisher::{AttestationMessage, Error, RequestId};

fn message(slot: u64) -> AttestationMessage {
    AttestationMessage::new(
        slot,
        Hash::from_bytes([0; 32]),
        RequestId::parse(REQUEST_ID).unwrap(),
        LIMITS,
    )
    .unwrap()
}

#[cfg_attr(test, test)]
pub fn detached_signature_is_verified_and_prefixed_to_the_original_json() {
    let key = Keypair::from_seed(&[7; 32]);
    let message = message(u64::MAX);
    let signature = key.sign(message.bytes());
    let attestation = message.attach_signature(key.address(), signature).unwrap();
    let decoded = Base64
        .encode(attestation.payload(), LIMITS.max_bytes)
        .unwrap();

    assert_eq!(&decoded[..SIGNATURE_BYTES], signature.as_bytes());
    assert_eq!(&decoded[SIGNATURE_BYTES..], JSON);
    assert_eq!(attestation.signer(), key.address());
    assert_eq!(attestation.message().bytes(), JSON);
}

#[cfg_attr(test, test)]
pub fn wrong_identity_changed_message_and_reserved_zero_signature_are_rejected() {
    let key = Keypair::from_seed(&[7; 32]);
    let wrong = Keypair::from_seed(&[8; 32]).address();
    let signature = key.sign(message(1).bytes());
    let wrong_identity = message(1).attach_signature(wrong, signature);
    assert_eq!(wrong_identity.unwrap_err(), Error::InvalidSignature(wrong));
    assert!(matches!(
        message(2).attach_signature(key.address(), signature),
        Err(Error::InvalidSignature(_))
    ));
    assert!(matches!(
        message(1).attach_signature(Address::from_bytes([0; 32]), Signature::from_bytes([0; 64])),
        Err(Error::InvalidSignature(_))
    ));
}
