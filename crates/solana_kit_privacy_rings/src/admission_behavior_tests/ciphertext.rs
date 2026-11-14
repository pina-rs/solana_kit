//! Real CTR decryption and tamper admission.

use super::*;

#[test]
fn real_ciphertext_parse_and_tamper_reject_admission() {
    let viewing = ViewingKey::from_bytes([7; 32]);
    let recipient = viewing.public_key().expect("viewing key");
    let expected_owner = [7; 32];
    let plain = confidential_plain(99, 1, [2; 32]);
    let published_commitment = confidential_commitment(expected_owner, 1, 1, 99, [2; 32]);
    let plaintext = write_confidential(&plain);

    let salt = [3; 16];
    let ephemeral_secret = [9; 32];
    let ephemeral_u = MontgomeryPoint::mul_base_clamped(ephemeral_secret).to_bytes();
    let ciphertext = encrypt(&plaintext, recipient, salt, ephemeral_secret, ephemeral_u);
    let mut eph_tag = [4u8; 33];
    eph_tag[1..].copy_from_slice(&ephemeral_u);

    let payload = encrypted_payload(recipient, &ciphertext);

    let opened_plaintext = viewing
        .open_output(&eph_tag, &salt, 1, &payload)
        .expect("real parse");
    let verified = verify_note(
        opened_plaintext,
        1,
        1,
        expected_owner,
        published_commitment,
        |_| None,
    )
    .expect("admits");
    assert_eq!(verified.commitment(), published_commitment);

    // Tamper ciphertext -> decrypted plaintext changes -> admission rejects.
    let mut tampered_ct = ciphertext;
    tampered_ct[0] ^= 1;

    let payload2 = encrypted_payload(recipient, &tampered_ct);

    let tampered_plaintext = viewing
        .open_output(&eph_tag, &salt, 1, &payload2)
        .expect("parse still succeeds for unauthenticated CTR");
    let error = verify_note(
        tampered_plaintext,
        1,
        1,
        expected_owner,
        published_commitment,
        |_| None,
    )
    .expect_err("tampered ciphertext");
    assert_eq!(limit_message(error), "commitment mismatch");
}
fn encrypt(
    plaintext: &[u8],
    recipient: [u8; 33],
    salt: [u8; 16],
    ephemeral_secret: [u8; 32],
    ephemeral_u: [u8; 32],
) -> Vec<u8> {
    use aes::Aes256;
    use ctr::Ctr32BE;
    use ctr::cipher::{KeyIvInit, StreamCipher};
    use hkdf::Hkdf;
    use sha2::Sha256;
    type Aes256Ctr = Ctr32BE<Aes256>;
    let peer = {
        let mut peer_bytes = [0u8; 32];
        peer_bytes.copy_from_slice(&recipient[1..]);
        peer_bytes
    };
    let shared = MontgomeryPoint(peer)
        .mul_clamped(ephemeral_secret)
        .to_bytes();
    let mut ikm = [0u8; 32 + 33 + 33];
    ikm[..32].copy_from_slice(&shared);
    let mut eph_tag = [4u8; 33];
    eph_tag[1..].copy_from_slice(&ephemeral_u);
    ikm[32..65].copy_from_slice(&eph_tag);
    ikm[65..].copy_from_slice(&recipient);

    let mut okm = [0u8; 44];
    Hkdf::<Sha256>::new(None, &ikm)
        .expand_multi_info(
            &[b"TSPP/hpke/", b"TSPP/tx", &salt, &1u32.to_be_bytes()],
            &mut okm,
        )
        .unwrap();

    let mut key = [0u8; 32];
    let mut nonce = [0u8; 12];
    key.copy_from_slice(&okm[..32]);
    nonce.copy_from_slice(&okm[32..]);

    let mut counter = [0u8; 16];
    counter[..12].copy_from_slice(&nonce);
    counter[15] = 2;
    let mut cipher = Aes256Ctr::new((&key).into(), (&counter).into());
    let mut buf = plaintext.to_vec();
    cipher.apply_keystream(&mut buf);
    buf
}

fn encrypted_payload(recipient: [u8; 33], ciphertext: &[u8]) -> Vec<u8> {
    let mut body = recipient.to_vec();
    body.extend(ciphertext);
    let mut payload = framed(3, &body);
    payload[0] = 1;
    payload
}
