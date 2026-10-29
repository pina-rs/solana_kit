//! The same detached-signature assertions run natively and on every portable host.

use super::assertions::{JSON, LIMITS, REQUEST_ID, SIGNATURE_BYTES};
use super::checks::{Outcome, require};
use solana_kit_core::{
    Address, Hash, Signature,
    codecs::{Base64, Encoder},
    crypto::Keypair,
};
use solana_kit_dapp_publisher::{AttestationMessage, Error, RequestId};

fn message(slot: u64) -> Result<AttestationMessage, &'static str> {
    AttestationMessage::new(
        slot,
        Hash::from_bytes([0; 32]),
        RequestId::parse(REQUEST_ID).map_err(|_| "valid signature request ID rejected")?,
        LIMITS,
    )
    .map_err(|_| "valid signature attestation message rejected")
}

#[cfg_attr(test, test)]
pub fn detached_signature_is_verified_and_prefixed_to_the_original_json() -> Outcome {
    let key = Keypair::from_seed(&[7; 32]);
    let message = message(u64::MAX)?;
    let signature = key.sign(message.bytes());
    let attestation = message
        .attach_signature(key.address(), signature)
        .map_err(|_| "valid detached attestation signature rejected")?;
    let decoded = Base64
        .encode(attestation.payload(), LIMITS.max_bytes)
        .map_err(|_| "attestation payload base64 rejected")?;

    require(
        decoded.get(..SIGNATURE_BYTES) == Some(signature.as_bytes().as_slice()),
        "attestation signature prefix mismatch",
    )?;
    require(
        decoded.get(SIGNATURE_BYTES..) == Some(JSON),
        "signed attestation JSON mismatch",
    )?;
    require(
        attestation.signer() == key.address(),
        "attestation signer changed",
    )?;
    require(
        attestation.message().bytes() == JSON,
        "attached message changed",
    )
}

#[cfg_attr(test, test)]
pub fn wrong_identity_changed_message_and_reserved_zero_signature_are_rejected() -> Outcome {
    let key = Keypair::from_seed(&[7; 32]);
    let wrong = Keypair::from_seed(&[8; 32]).address();
    let signature = key.sign(message(1)?.bytes());
    let wrong_identity = message(1)?.attach_signature(wrong, signature);
    require(
        wrong_identity.err() == Some(Error::InvalidSignature(wrong)),
        "wrong attestation identity was not rejected",
    )?;
    require(
        matches!(
            message(2)?.attach_signature(key.address(), signature),
            Err(Error::InvalidSignature(_))
        ),
        "changed attestation message was not rejected",
    )?;
    require(
        matches!(
            message(1)?
                .attach_signature(Address::from_bytes([0; 32]), Signature::from_bytes([0; 64])),
            Err(Error::InvalidSignature(_))
        ),
        "reserved zero attestation signature was not rejected",
    )
}
