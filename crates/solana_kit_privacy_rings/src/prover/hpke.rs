//! HPKE seal for an attested prover request.
//!
//! The suite is DHKEM(X25519, HKDF-SHA256) and AES-256-GCM, base mode, with
//! info `zolana/prover-tee/v1`. The response key is the HPKE export labeled
//! `zolana/prover-tee/v1/response`. Additional data is [`super::request_aad`].

use aes_gcm::aead::{Aead, Payload};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use curve25519_dalek::montgomery::MontgomeryPoint;
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroize;

use crate::Error;

const KEM_SUITE: &[u8] = &[0x4b, 0x45, 0x4d, 0x00, 0x20];
const HPKE_SUITE: &[u8] = &[0x48, 0x50, 0x4b, 0x45, 0x00, 0x20, 0x00, 0x01, 0x00, 0x02];
const INFO: &[u8] = b"zolana/prover-tee/v1";
const RESPONSE_EXPORT: &str = "zolana/prover-tee/v1/response";
const HASH_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;
const STATUS_LEN: usize = 2;

/// Ciphertext posted to the prover, and the key that opens its answer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SealedRequest {
    /// Encapsulated ephemeral public key.
    pub enc: [u8; HASH_LEN],
    /// AEAD ciphertext, including the tag.
    pub body: Vec<u8>,
    /// Exporter secret used as the response AES-256-GCM key.
    pub response_key: [u8; HASH_LEN],
}

/// Plaintext recovered from a request sealed to `ikm`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenedRequest {
    /// Request body.
    pub plaintext: Vec<u8>,
    /// Key that opens the prover's encrypted answer.
    pub response_key: [u8; HASH_LEN],
}

/// Encrypts `plaintext` to `hpke_public_key`.
///
/// `ephemeral_secret` is the unclamped X25519 seed. Tests pass a fixed seed.
/// A caller that wants a fresh encapsulation draws 32 random bytes.
///
/// # Errors
/// Rejects a public key or a seal the AEAD will not accept.
pub fn seal_prover_request(
    hpke_public_key: &[u8; HASH_LEN],
    method: &str,
    request_uri: &str,
    plaintext: &[u8],
    ephemeral_secret: &[u8; HASH_LEN],
) -> Result<SealedRequest, Error> {
    let enc = MontgomeryPoint::mul_base_clamped(*ephemeral_secret).to_bytes();
    let shared = MontgomeryPoint(*hpke_public_key)
        .mul_clamped(*ephemeral_secret)
        .to_bytes();
    let context = key_schedule(&shared, &enc, hpke_public_key)?;
    let body = seal_aead(&context, method, request_uri, plaintext)?;
    Ok(SealedRequest {
        enc,
        body,
        response_key: context.exporter,
    })
}

/// Opens a ciphertext produced for the recipient `ikm`.
///
/// # Errors
/// Rejects a ciphertext that is shorter than its tag or that does not authenticate.
pub fn open_prover_request(
    ikm: &[u8],
    enc: &[u8; HASH_LEN],
    ciphertext: &[u8],
    method: &str,
    request_uri: &str,
) -> Result<OpenedRequest, Error> {
    let secret = derive_private(ikm);
    let public_key = MontgomeryPoint::mul_base_clamped(secret).to_bytes();
    let shared = MontgomeryPoint(*enc).mul_clamped(secret).to_bytes();
    let context = key_schedule(&shared, enc, &public_key)?;
    let plaintext = open_aead(&context, method, request_uri, ciphertext)?;
    Ok(OpenedRequest {
        plaintext,
        response_key: context.exporter,
    })
}

/// Decrypts a TEE response. The nonce is the first 12 bytes. The plaintext
/// starts with a big-endian HTTP status.
///
/// # Errors
/// Rejects a truncated body or a tag that does not match `key`.
pub fn open_prover_response(
    key: &[u8; HASH_LEN],
    encrypted: &[u8],
) -> Result<(u16, Vec<u8>), Error> {
    if encrypted.len() < NONCE_LEN + STATUS_LEN + TAG_LEN {
        return Err(Error::Attestation("response"));
    }

    let (nonce, ciphertext) = encrypted.split_at(NONCE_LEN);
    let cipher = Aes256Gcm::new_from_slice(key).expect("response key");
    let clear = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| Error::Attestation("response"))?;

    let status = u16::from_be_bytes([clear[0], clear[1]]);
    Ok((status, clear[STATUS_LEN..].to_vec()))
}

struct Schedule {
    key: [u8; HASH_LEN],
    nonce: [u8; NONCE_LEN],
    exporter: [u8; HASH_LEN],
}

fn key_schedule(
    shared: &[u8; HASH_LEN],
    enc: &[u8; HASH_LEN],
    recipient: &[u8; HASH_LEN],
) -> Result<Schedule, Error> {
    let mut kem_context = [0u8; HASH_LEN * 2];
    kem_context[..HASH_LEN].copy_from_slice(enc);
    kem_context[HASH_LEN..].copy_from_slice(recipient);
    let eae = labeled_extract(KEM_SUITE, &[], "eae_prk", shared);
    let mut secret_dh = labeled_expand(&eae, KEM_SUITE, "shared_secret", &kem_context, HASH_LEN);
    let psk_hash = labeled_extract(HPKE_SUITE, &[], "psk_id_hash", &[]);
    let info_hash = labeled_extract(HPKE_SUITE, &[], "info_hash", INFO);
    let mut schedule = Vec::with_capacity(1 + HASH_LEN * 2);
    schedule.push(0);
    schedule.extend(psk_hash);
    schedule.extend(info_hash);
    let secret = labeled_extract(HPKE_SUITE, &secret_dh, "secret", &[]);
    secret_dh.zeroize();
    let key = labeled_expand(&secret, HPKE_SUITE, "key", &schedule, HASH_LEN);
    let nonce = labeled_expand(&secret, HPKE_SUITE, "base_nonce", &schedule, NONCE_LEN);
    let export_secret = labeled_expand(&secret, HPKE_SUITE, "exp", &schedule, HASH_LEN);
    let exporter = labeled_expand(
        &export_secret,
        HPKE_SUITE,
        "sec",
        RESPONSE_EXPORT.as_bytes(),
        HASH_LEN,
    );
    let mut key_bytes = [0u8; HASH_LEN];
    key_bytes.copy_from_slice(&key);
    let mut nonce_bytes = [0u8; NONCE_LEN];
    nonce_bytes.copy_from_slice(&nonce);
    let mut exporter_bytes = [0u8; HASH_LEN];
    exporter_bytes.copy_from_slice(&exporter);
    Ok(Schedule {
        key: key_bytes,
        nonce: nonce_bytes,
        exporter: exporter_bytes,
    })
}

fn seal_aead(
    context: &Schedule,
    method: &str,
    request_uri: &str,
    plaintext: &[u8],
) -> Result<Vec<u8>, Error> {
    let aad = super::request_aad(method, request_uri);
    let cipher = Aes256Gcm::new_from_slice(&context.key).expect("seal key");
    cipher
        .encrypt(
            Nonce::from_slice(&context.nonce),
            Payload {
                msg: plaintext,
                aad: aad.as_bytes(),
            },
        )
        .map_err(|_| Error::Attestation("seal"))
}

fn open_aead(
    context: &Schedule,
    method: &str,
    request_uri: &str,
    ciphertext: &[u8],
) -> Result<Vec<u8>, Error> {
    if ciphertext.len() < TAG_LEN {
        return Err(Error::Attestation("ciphertext"));
    }

    let aad = super::request_aad(method, request_uri);
    let cipher = Aes256Gcm::new_from_slice(&context.key).expect("open key");
    cipher
        .decrypt(
            Nonce::from_slice(&context.nonce),
            Payload {
                msg: ciphertext,
                aad: aad.as_bytes(),
            },
        )
        .map_err(|_| Error::Attestation("ciphertext"))
}

fn derive_private(ikm: &[u8]) -> [u8; HASH_LEN] {
    let prk = labeled_extract(KEM_SUITE, &[], "dkp_prk", ikm);
    let secret = labeled_expand(&prk, KEM_SUITE, "sk", &[], HASH_LEN);
    let mut key = [0u8; HASH_LEN];
    key.copy_from_slice(&secret[..HASH_LEN]);
    key
}

fn labeled_extract(suite: &[u8], salt: &[u8], label: &str, ikm: &[u8]) -> [u8; HASH_LEN] {
    let mut labeled = Vec::with_capacity(7 + suite.len() + label.len() + ikm.len());
    labeled.extend_from_slice(b"HPKE-v1");
    labeled.extend_from_slice(suite);
    labeled.extend_from_slice(label.as_bytes());
    labeled.extend_from_slice(ikm);
    let used: &[u8] = if salt.is_empty() {
        &[0u8; HASH_LEN]
    } else {
        salt
    };
    let (prk, _) = Hkdf::<Sha256>::extract(Some(used), &labeled);
    let mut out = [0u8; HASH_LEN];
    out.copy_from_slice(&prk);
    out
}

fn labeled_expand(prk: &[u8], suite: &[u8], label: &str, info: &[u8], length: usize) -> Vec<u8> {
    let width = u16::try_from(length).expect("hpke length");
    let mut labeled = Vec::with_capacity(2 + 7 + suite.len() + label.len() + info.len());
    labeled.extend_from_slice(&width.to_be_bytes());
    labeled.extend_from_slice(b"HPKE-v1");
    labeled.extend_from_slice(suite);
    labeled.extend_from_slice(label.as_bytes());
    labeled.extend_from_slice(info);
    let hkdf = Hkdf::<Sha256>::from_prk(prk).expect("prk");
    let mut out = vec![0u8; length];
    hkdf.expand(&labeled, &mut out).expect("expand");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prover::hex_decode;

    fn vector() -> serde_json::Value {
        serde_json::from_str(include_str!("../../tests/fixtures/tee-vectors.json")).expect("vector")
    }

    fn bytes32(value: &serde_json::Value) -> [u8; 32] {
        hex_decode(value.as_str().unwrap())
            .unwrap()
            .try_into()
            .unwrap()
    }

    #[test]
    fn published_ciphertext_opens_to_the_go_plaintext() {
        let vector = vector();
        let ikm = hex_decode(vector["ikm"].as_str().unwrap()).unwrap();
        let enc: [u8; 32] = hex_decode(vector["enc"].as_str().unwrap())
            .unwrap()
            .try_into()
            .unwrap();
        let ciphertext = hex_decode(vector["ciphertext"].as_str().unwrap()).unwrap();
        let opened = open_prover_request(
            &ikm,
            &enc,
            &ciphertext,
            vector["method"].as_str().unwrap(),
            vector["request_uri"].as_str().unwrap(),
        )
        .expect("open");
        assert_eq!(
            opened.plaintext,
            vector["plaintext"].as_str().unwrap().as_bytes()
        );
        assert_eq!(
            hex_encode_key(&opened.response_key),
            vector["response_key"].as_str().unwrap()
        );
        let public = MontgomeryPoint::mul_base_clamped(derive_private(&ikm)).to_bytes();
        assert_eq!(
            hex_encode_key(&public),
            vector["hpke_public_key"].as_str().unwrap()
        );
        let (status, body) = open_prover_response(
            &opened.response_key,
            &hex_decode(vector["encrypted_response"].as_str().unwrap()).unwrap(),
        )
        .expect("response");
        assert_eq!(status, vector["response_status"].as_u64().unwrap() as u16);
        assert_eq!(body, vector["response_body"].as_str().unwrap().as_bytes());
    }

    #[test]
    fn a_seal_round_trips_and_rejects_the_wrong_route() {
        let vector = vector();
        let key = bytes32(&vector["hpke_public_key"]);
        let secret = [7u8; 32];
        let sealed = seal_prover_request(&key, "POST", "/prove/x", b"note", &secret).expect("seal");
        let ikm = hex_decode(vector["ikm"].as_str().unwrap()).unwrap();
        let opened =
            open_prover_request(&ikm, &sealed.enc, &sealed.body, "POST", "/prove/x").expect("open");
        assert_eq!(opened.plaintext, b"note");
        assert!(open_prover_request(&ikm, &sealed.enc, &sealed.body, "POST", "/prove/y").is_err());
        assert!(open_prover_request(&ikm, &sealed.enc, &[0; 8], "POST", "/prove/x").is_err());
        assert!(open_prover_response(&opened.response_key, &[0; 10]).is_err());
        let mut tampered = hex_decode(vector["encrypted_response"].as_str().unwrap()).unwrap();
        tampered[20] ^= 1;
        assert!(open_prover_response(&bytes32(&vector["response_key"]), &tampered).is_err());
        let key = [3u8; 32];
        let nonce = [1u8; NONCE_LEN];
        let cipher = Aes256Gcm::new_from_slice(&key).expect("key");
        let sealed = cipher
            .encrypt(Nonce::from_slice(&nonce), b"xy".as_slice())
            .expect("seal");
        let mut short = nonce.to_vec();
        short.extend(sealed);
        assert_eq!(
            open_prover_response(&key, &short).expect("open").0,
            u16::from_be_bytes(*b"xy")
        );
    }

    fn hex_encode_key(bytes: &[u8]) -> String {
        super::super::hex_encode(bytes)
    }
}
