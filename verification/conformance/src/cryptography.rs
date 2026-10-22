//! Fixed standards vectors, never generated from the implementation under test.

use hex_literal::hex;
use solana_kit_core::{
    Address, Signature,
    crypto::{Keypair, hash, verify},
};

use crate::{FixtureMode, require};

const SEED_ONE: [u8; 32] = hex!(
    "9d61b19deffd5a60ba844af492ec2cc4
     4449c5697b326919703bac031cae7f60"
);
const KEY_ONE: [u8; 32] = hex!("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");
const SIGNATURE_ONE: [u8; 64] = hex!(
    "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e06522490155
     5fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"
);

/// Compares contiguous and split input against RFC 6234's SHA-256 abc example.
pub(crate) fn sha256(mode: FixtureMode) -> Result<(), &'static str> {
    let mut expected = hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");

    if mode == FixtureMode::CorruptHash {
        expected[0] ^= 1;
    }

    require(
        hash(&[b"abc"]).to_bytes() == expected,
        "SHA-256 digest mismatch",
    )?;
    require(
        hash(&[b"a", b"bc"]).to_bytes() == expected,
        "split SHA-256 mismatch",
    )
}

/// RFC 8032 section 7.1 test 1 covers the empty message and deterministic seed.
pub(crate) fn rfc8032_empty() -> Result<(), &'static str> {
    let signer = Keypair::from_seed(&SEED_ONE);
    let expected = Signature::from_bytes(SIGNATURE_ONE);

    require(
        signer.address().to_bytes() == KEY_ONE,
        "RFC 8032 public key mismatch",
    )?;
    require(
        signer.sign(b"") == expected,
        "RFC 8032 empty-message signature mismatch",
    )?;
    verify(&Address::from_bytes(KEY_ONE), b"", &expected)
        .map_err(|_| "RFC 8032 empty-message verification failed")
}

/// RFC 8032 section 7.1 test 2 covers a nonempty message with a second key.
pub(crate) fn rfc8032_one_byte() -> Result<(), &'static str> {
    let seed = hex!("4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb");
    let public = hex!("3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c");
    let signature = Signature::from_bytes(hex!(
        "92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da
         085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00"
    ));
    let signer = Keypair::from_seed(&seed);

    require(
        signer.address().to_bytes() == public,
        "RFC 8032 test 2 public key mismatch",
    )?;
    require(
        signer.sign(&[0x72]) == signature,
        "RFC 8032 test 2 signature mismatch",
    )?;
    verify(&Address::from_bytes(public), &[0x72], &signature)
        .map_err(|_| "RFC 8032 test 2 verification failed")
}

/// Mutates a fixed signature rather than signing a changed input with the same code.
pub(crate) fn altered_signature() -> Result<(), &'static str> {
    let mut altered = SIGNATURE_ONE;
    altered[0] ^= 1;

    require(
        verify(
            &Address::from_bytes(KEY_ONE),
            b"",
            &Signature::from_bytes(altered),
        )
        .is_err(),
        "altered signature was accepted",
    )
}

/// The original signature must not authorize even a one-byte different message.
pub(crate) fn altered_message() -> Result<(), &'static str> {
    require(
        verify(
            &Address::from_bytes(KEY_ONE),
            &[0],
            &Signature::from_bytes(SIGNATURE_ONE),
        )
        .is_err(),
        "altered message was accepted",
    )
}

/// A small-order identity point cannot serve as a verification key.
pub(crate) fn weak_key() -> Result<(), &'static str> {
    let mut identity = [0; 32];
    identity[0] = 1;
    let mut forged_signature = [0; 64];
    forged_signature[0] = 1;

    require(
        verify(
            &Address::from_bytes(identity),
            b"",
            &Signature::from_bytes(forged_signature),
        )
        .is_err(),
        "weak Ed25519 key was accepted",
    )
}
