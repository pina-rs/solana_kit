//! HPKE seal for an attested prover request.
//!
//! The suite is DHKEM(X25519, HKDF-SHA256) and AES-256-GCM, base mode, with
//! info `zolana/prover-tee/v1`. The response key is the HPKE export labeled
//! `zolana/prover-tee/v1/response`. Additional data is [`super::request_aad`].
//! The ephemeral seed is drawn from the caller's cryptographic RNG and wiped
//! after use, and an all-zero X25519 shared secret aborts both directions, so
//! no schedule is ever derived from a non-contributory exchange.

use aes_gcm::aead::{Aead, Payload};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use curve25519_dalek::montgomery::MontgomeryPoint;
use hkdf::Hkdf;
use rand_core::{CryptoRng, RngCore};
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
#[derive(Clone, PartialEq, Eq)]
pub struct SealedRequest {
    /// Encapsulated ephemeral public key.
    pub enc: [u8; HASH_LEN],
    /// AEAD ciphertext, including the tag.
    pub body: Vec<u8>,
    /// Exporter secret used as the response AES-256-GCM key.
    pub response_key: [u8; HASH_LEN],
}

impl std::fmt::Debug for SealedRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Security: the response key is the AES-256-GCM key for the prover's
        // answer and never appears in Debug. `enc` is public wire data.
        formatter
            .debug_struct("SealedRequest")
            .field("enc", &self.enc)
            .field("body_len", &self.body.len())
            .finish_non_exhaustive()
    }
}

/// Plaintext recovered from a request sealed to `ikm`.
#[derive(Clone, PartialEq, Eq)]
pub struct OpenedRequest {
    /// Request body.
    pub plaintext: Vec<u8>,
    /// Key that opens the prover's encrypted answer.
    pub response_key: [u8; HASH_LEN],
}

impl std::fmt::Debug for OpenedRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Security: the plaintext is the decrypted witness and the response
        // key is live key material; Debug keeps only the shape.
        formatter
            .debug_struct("OpenedRequest")
            .field("plaintext_len", &self.plaintext.len())
            .finish_non_exhaustive()
    }
}

/// Encrypts `plaintext` to `hpke_public_key` with a fresh ephemeral key.
///
/// The X25519 ephemeral seed is drawn from `rng`, a caller-supplied
/// cryptographic generator. One 32-byte draw seeds each request, and that
/// fresh draw is what keeps the AES-256-GCM key and nonce unique across
/// requests. This crate never falls back to a fixed seed or to an OS
/// generator of its own.
///
/// # Errors
/// Rejects a non-contributory X25519 shared secret, such as an all-zero or
/// low-order recipient public key, and a seal the AEAD will not accept.
pub fn seal_prover_request<R>(
    hpke_public_key: &[u8; HASH_LEN],
    method: &str,
    request_uri: &str,
    plaintext: &[u8],
    rng: &mut R,
) -> Result<SealedRequest, Error>
where
    R: RngCore + CryptoRng,
{
    let mut ephemeral = [0u8; HASH_LEN];
    rng.fill_bytes(&mut ephemeral);
    let sealed =
        seal_with_ephemeral_seed(hpke_public_key, method, request_uri, plaintext, &ephemeral);
    ephemeral.zeroize();
    sealed
}

/// Seals with a caller-chosen seed. Private so the published Go vectors can
/// pin one; production callers go through [`seal_prover_request`].
fn seal_with_ephemeral_seed(
    hpke_public_key: &[u8; HASH_LEN],
    method: &str,
    request_uri: &str,
    plaintext: &[u8],
    ephemeral_seed: &[u8; HASH_LEN],
) -> Result<SealedRequest, Error> {
    let enc = MontgomeryPoint::mul_base_clamped(*ephemeral_seed).to_bytes();
    let mut shared = MontgomeryPoint(*hpke_public_key)
        .mul_clamped(*ephemeral_seed)
        .to_bytes();
    let schedule = key_schedule(&shared, &enc, hpke_public_key);
    shared.zeroize();
    let context = schedule?;
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
/// Rejects a non-contributory X25519 shared secret, such as an all-zero or
/// low-order `enc` on the wire, a ciphertext that is shorter than its tag,
/// and a tag that does not authenticate.
pub fn open_prover_request(
    ikm: &[u8],
    enc: &[u8; HASH_LEN],
    ciphertext: &[u8],
    method: &str,
    request_uri: &str,
) -> Result<OpenedRequest, Error> {
    let mut secret = derive_private(ikm);
    let public_key = MontgomeryPoint::mul_base_clamped(secret).to_bytes();
    let mut shared = MontgomeryPoint(*enc).mul_clamped(secret).to_bytes();
    secret.zeroize();
    let schedule = key_schedule(&shared, enc, &public_key);
    shared.zeroize();
    let context = schedule?;
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
    // Security: RFC 9180 aborts Encap and Decap when the DH output does not
    // contribute. An all-zero X25519 result makes the whole schedule
    // computable from public bytes, so it is rejected before any key is
    // derived.
    if *shared == [0; HASH_LEN] {
        return Err(Error::NonContributory);
    }

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
    let mut secret = labeled_extract(HPKE_SUITE, &secret_dh, "secret", &[]);
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
    secret.zeroize();
    export_secret.zeroize();
    let mut key_bytes = [0u8; HASH_LEN];
    key_bytes.copy_from_slice(&key);
    key.zeroize();
    let mut nonce_bytes = [0u8; NONCE_LEN];
    nonce_bytes.copy_from_slice(&nonce);
    nonce.zeroize();
    let mut exporter_bytes = [0u8; HASH_LEN];
    exporter_bytes.copy_from_slice(&exporter);
    exporter.zeroize();

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
    use rand_core::Error as RngError;

    fn vector() -> serde_json::Value {
        serde_json::from_str(include_str!("../../tests/fixtures/tee-vectors.json")).expect("vector")
    }

    fn bytes32(value: &serde_json::Value) -> [u8; 32] {
        hex_decode(value.as_str().unwrap())
            .unwrap()
            .try_into()
            .unwrap()
    }

    /// A deterministic, advancing cryptographic-RNG double. Every draw moves
    /// the stream forward, so two seals from one instance never share a seed.
    struct StepRng {
        state: u64,
        fills: usize,
    }

    impl StepRng {
        fn new(state: u64) -> Self {
            Self { state, fills: 0 }
        }
    }

    impl RngCore for StepRng {
        fn next_u32(&mut self) -> u32 {
            (self.next_u64() >> 32) as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.state = self.state.wrapping_add(0xA076_1D64_78BD_642F);
            let mut mixed = self.state;
            mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            mixed ^ (mixed >> 31)
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            self.fills += 1;

            for (index, slot) in dest.iter_mut().enumerate() {
                *slot = (self.next_u64() >> (8 * (index % 8))) as u8;
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), RngError> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl CryptoRng for StepRng {}

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
        let mut rng = StepRng::new(7);
        let sealed =
            seal_prover_request(&key, "POST", "/prove/x", b"note", &mut rng).expect("seal");
        assert_eq!(rng.fills, 1);
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

    #[test]
    fn a_fixed_seed_seals_deterministically_for_the_published_vectors() {
        let vector = vector();
        let key = bytes32(&vector["hpke_public_key"]);
        let seed = [7u8; 32];
        let first =
            seal_with_ephemeral_seed(&key, "POST", "/prove/x", b"note", &seed).expect("seal");
        let second =
            seal_with_ephemeral_seed(&key, "POST", "/prove/x", b"note", &seed).expect("seal");
        assert_eq!(first, second);
        let mut other_seed = seed;
        other_seed[31] ^= 1;
        let other =
            seal_with_ephemeral_seed(&key, "POST", "/prove/x", b"note", &other_seed).expect("seal");
        assert_ne!(first.enc, other.enc);
        assert_ne!(first.response_key, other.response_key);
    }

    #[test]
    fn each_seal_draws_a_fresh_ephemeral_from_the_rng() {
        let vector = vector();
        let key = bytes32(&vector["hpke_public_key"]);
        let ikm = hex_decode(vector["ikm"].as_str().unwrap()).unwrap();
        let mut rng = StepRng::new(0x0BAD_C0DE);
        let first =
            seal_prover_request(&key, "POST", "/prove/x", b"note-one", &mut rng).expect("first");
        let second =
            seal_prover_request(&key, "POST", "/prove/x", b"note-two", &mut rng).expect("second");
        assert_eq!(rng.fills, 2);
        assert_ne!(first.enc, second.enc);
        assert_ne!(first.response_key, second.response_key);
        assert_ne!(first.body, second.body);
        let one =
            open_prover_request(&ikm, &first.enc, &first.body, "POST", "/prove/x").expect("one");
        assert_eq!(one.plaintext, b"note-one");
        let two =
            open_prover_request(&ikm, &second.enc, &second.body, "POST", "/prove/x").expect("two");
        assert_eq!(two.plaintext, b"note-two");
    }

    #[test]
    fn an_all_zero_recipient_key_is_refused_before_any_key_is_derived() {
        let mut rng = StepRng::new(1);
        let error = seal_prover_request(&[0u8; 32], "POST", "/prove/x", b"note", &mut rng)
            .expect_err("zero recipient key");
        assert!(matches!(error, Error::NonContributory));
        assert!(key_schedule(&[0u8; 32], &[0x68; 32], &[0u8; 32]).is_err());
    }

    #[test]
    fn a_low_order_enc_is_refused_when_opening() {
        let vector = vector();
        let ikm = hex_decode(vector["ikm"].as_str().unwrap()).unwrap();
        let key = bytes32(&vector["hpke_public_key"]);
        let mut rng = StepRng::new(3);
        let genuine =
            seal_prover_request(&key, "POST", "/prove/x", b"note", &mut rng).expect("seal");

        // The identity point on the wire drives the shared secret to zero, so
        // the body never even reaches the AEAD.
        let error = open_prover_request(&ikm, &[0u8; 32], &genuine.body, "POST", "/prove/x")
            .expect_err("identity enc");
        assert!(matches!(error, Error::NonContributory));
        let error = open_prover_request(&ikm, &[1u8; 32], &genuine.body, "POST", "/prove/x")
            .expect_err("order-eight enc");
        assert!(matches!(error, Error::NonContributory));
    }

    #[test]
    fn debug_output_redacts_the_response_key_and_plaintext() {
        let vector = vector();
        let key = bytes32(&vector["hpke_public_key"]);
        let mut rng = StepRng::new(9);
        let sealed = seal_prover_request(&key, "POST", "/prove/x", b"TOP-SECRET-WITNESS", &mut rng)
            .expect("seal");
        let ikm = hex_decode(vector["ikm"].as_str().unwrap()).unwrap();
        let opened =
            open_prover_request(&ikm, &sealed.enc, &sealed.body, "POST", "/prove/x").expect("open");

        let rendered_sealed = format!("{sealed:?}");
        assert!(rendered_sealed.contains("SealedRequest"));
        assert!(rendered_sealed.contains("body_len"));
        assert!(!rendered_sealed.contains(format!("{:?}", sealed.response_key).as_str()));
        assert!(!rendered_sealed.contains(hex_encode_key(&sealed.response_key).as_str()));

        let rendered_opened = format!("{opened:?}");
        assert!(rendered_opened.contains("OpenedRequest"));
        assert!(!rendered_opened.contains("TOP-SECRET-WITNESS"));
        assert!(!rendered_opened.contains(format!("{:?}", opened.plaintext).as_str()));
        assert!(!rendered_opened.contains(format!("{:?}", opened.response_key).as_str()));
        assert!(!rendered_opened.contains(hex_encode_key(&opened.response_key).as_str()));
        assert!(!rendered_sealed.contains(format!("{:?}", opened.plaintext).as_str()));
    }

    fn hex_encode_key(bytes: &[u8]) -> String {
        super::super::hex_encode(bytes)
    }
}
