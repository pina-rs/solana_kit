//! Viewing-key decryption.
//!
//! The scheme is the v0.8.0 `spr-keypair` path: tagged X25519 (`0x04 || u`),
//! HKDF-SHA256 over `TSPP/hpke/ || TSPP/tx || salt || slot`, then AES-256-CTR
//! whose first keystream block is counter 2. Poseidon commitment checks stay
//! with the caller. [`crate::subtract_nullified`] drops a note whose derived
//! nullifier appears in history. This module does not derive that nullifier.

use aes::Aes256;
use ctr::Ctr32BE;
use ctr::cipher::{KeyIvInit, StreamCipher};
use curve25519_dalek::montgomery::MontgomeryPoint;
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroize;

use crate::Error;
use crate::address::X25519_KEY_TAG;
use crate::encode::{ConfidentialPlaintext, read_confidential};

const TRANSFER_INFO: &[u8] = b"TSPP/tx";
const HPKE_PREFIX: &[u8] = b"TSPP/hpke/";
const CONFIDENTIAL_SCHEME: u8 = 3;
const RING_CONFIDENTIAL_SCHEME: u8 = 4;
const PROOFLESS_SCHEME: u8 = 0;
const VIEWING_KEY_LEN: usize = 33;
const PAYLOAD_HEADER_LEN: usize = 5;
const PROOFLESS_NOTE_PREFIX_LEN: usize = 104;
const SIGN_BYTE: usize = 31;
const CURVE_LOW: u8 = 237;
const CLAMP_BYTE: u8 = 127;
const OWNER_LEN: usize = 32;
const BLINDING_END: usize = 64;
const ASSET_END: usize = 96;

type Aes256Ctr = Ctr32BE<Aes256>;

/// RFC 7748 viewing secret. The secret is wiped when the value is dropped.
pub struct ViewingKey {
    secret: [u8; 32],
}

impl Drop for ViewingKey {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}

impl std::fmt::Debug for ViewingKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ViewingKey")
    }
}

impl ViewingKey {
    /// Wraps an unclamped 32-byte secret, the encoding used by the Helius SDK.
    #[must_use]
    pub const fn from_bytes(secret: [u8; 32]) -> Self {
        Self { secret }
    }

    /// Tagged public key `0x04 || u`.
    ///
    /// # Errors
    /// Rejects a public key that is not a canonical u-coordinate.
    pub fn public_key(&self) -> Result<[u8; 33], Error> {
        let point = MontgomeryPoint::mul_base_clamped(self.secret);
        tagged(&point.to_bytes())
    }

    /// Opens one indexer payload with this viewing key.
    ///
    /// `slot_index` is this output's position in the event's full `outputSlots`
    /// list. `GetEncryptedUtxosByTags` returns matching outputs, which can be a
    /// subset, so that list's order is not the slot index. Look the signature
    /// up with `GetShieldedTransactionsBySignature` and use the matching
    /// output's position in that event.
    ///
    /// Confidential ciphertext is AES-CTR and has no authentication tag. This
    /// function does not check the Poseidon commitment. Compare a decrypted
    /// note with a commitment you trust before summing it.
    ///
    /// A proofless deposit is cleartext. This function does not check
    /// [`ProoflessNote::owner`]. Leave out notes owned by someone else before
    /// summing.
    ///
    /// `tx_viewing_pk` and `salt` are the transaction fields, already decoded
    /// from base64. `payload` is the decoded `outputSlot.payload`.
    ///
    /// # Errors
    /// Rejects a foreign viewing key, a short or over-long payload, or a
    /// plaintext the documented layout does not accept.
    pub fn open_output(
        &self,
        tx_viewing_pk: &[u8; 33],
        salt: &[u8; 16],
        slot_index: u32,
        payload: &[u8],
    ) -> Result<OpenedNote, Error> {
        let body = encoding_body(payload)?;
        let (&scheme, rest) = body.split_first().ok_or(Error::Plaintext)?;

        match scheme {
            PROOFLESS_SCHEME => parse_proofless(rest),
            CONFIDENTIAL_SCHEME | RING_CONFIDENTIAL_SCHEME => {
                self.open_confidential(tx_viewing_pk, salt, slot_index, rest)
            }
            _ => Err(Error::Plaintext),
        }
    }

    fn open_confidential(
        &self,
        tx_viewing_pk: &[u8; 33],
        salt: &[u8; 16],
        slot_index: u32,
        body: &[u8],
    ) -> Result<OpenedNote, Error> {
        if body.len() < VIEWING_KEY_LEN {
            return Err(Error::Plaintext);
        }

        let mut embedded = [0; VIEWING_KEY_LEN];
        embedded.copy_from_slice(&body[..VIEWING_KEY_LEN]);

        if embedded != self.public_key()? {
            return Err(Error::NotForViewer);
        }

        let plaintext = decrypt_ciphertext(
            self,
            tx_viewing_pk,
            salt,
            slot_index,
            &body[VIEWING_KEY_LEN..],
        )?;
        Ok(OpenedNote::Confidential(read_confidential(&plaintext)?))
    }
}

/// A note recovered from an indexer payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpenedNote {
    /// AES-CTR confidential or ring-confidential plaintext.
    Confidential(ConfidentialPlaintext),
    /// Cleartext deposit output.
    Proofless(ProoflessNote),
}

/// The fixed prefix of a proofless deposit output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProoflessNote {
    /// Recipient owner hash.
    pub owner: [u8; 32],
    /// Deposit blinding.
    pub blinding: [u8; 32],
    /// Mint, or the zero address for SOL.
    pub asset: [u8; 32],
    /// Amount in base units.
    pub amount: u64,
    /// Trailing application/ring metadata retained from the payload. A
    /// data-free proofless note has an empty vector.
    pub records: Vec<u8>,
}

/// A decrypted note plus the leaf the indexer assigned it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackedNote {
    /// State tree id.
    pub tree_id: u16,
    /// Leaf index.
    pub leaf_index: u64,
    /// Decrypted or proofless note.
    pub note: OpenedNote,
}

fn decrypt_ciphertext(
    viewing: &ViewingKey,
    tx_viewing_pk: &[u8; 33],
    salt: &[u8; 16],
    slot_index: u32,
    ciphertext: &[u8],
) -> Result<Vec<u8>, Error> {
    let ephemeral = coordinate(tx_viewing_pk)?;
    let recipient = viewing.public_key()?;
    let shared = diffie_hellman(&viewing.secret, &ephemeral)?;
    let (mut key, nonce) = derive_key(&shared, &ephemeral, &recipient, salt, slot_index)?;
    let mut buffer = ciphertext.to_vec();
    apply_ctr(&key, &nonce, &mut buffer);
    key.zeroize();
    Ok(buffer)
}

fn diffie_hellman(secret: &[u8; 32], peer: &[u8; 32]) -> Result<[u8; 32], Error> {
    let shared = MontgomeryPoint(*peer).mul_clamped(*secret).to_bytes();

    if shared == [0; 32] {
        return Err(Error::NonContributory);
    }

    Ok(shared)
}

fn derive_key(
    shared: &[u8; 32],
    ephemeral: &[u8; 32],
    recipient: &[u8; 33],
    salt: &[u8; 16],
    slot_index: u32,
) -> Result<([u8; 32], [u8; 12]), Error> {
    let mut ikm = [0; 32 + 33 + 33];
    ikm[..32].copy_from_slice(shared);
    ikm[32..65].copy_from_slice(tagged(ephemeral)?.as_slice());
    ikm[65..].copy_from_slice(recipient);
    let mut okm = [0; 44];
    Hkdf::<Sha256>::new(None, &ikm)
        .expand_multi_info(
            &[HPKE_PREFIX, TRANSFER_INFO, salt, &slot_index.to_be_bytes()],
            &mut okm,
        )
        .map_err(|_| Error::Key)?;
    ikm.zeroize();
    let mut key = [0; 32];
    let mut nonce = [0; 12];
    key.copy_from_slice(&okm[..32]);
    nonce.copy_from_slice(&okm[32..]);
    okm.zeroize();
    Ok((key, nonce))
}

fn apply_ctr(key: &[u8; 32], nonce: &[u8; 12], buffer: &mut [u8]) {
    let mut counter = [0; 16];
    counter[..12].copy_from_slice(nonce);
    counter[15] = 2;
    let mut cipher = Aes256Ctr::new(key.into(), (&counter).into());
    cipher.apply_keystream(buffer);
}

fn tagged(scalar: &[u8; 32]) -> Result<[u8; 33], Error> {
    if !canonical(scalar) {
        return Err(Error::Key);
    }

    let mut bytes = [X25519_KEY_TAG; VIEWING_KEY_LEN];
    bytes[1..].copy_from_slice(scalar);
    Ok(bytes)
}

fn coordinate(tagged_key: &[u8; 33]) -> Result<[u8; 32], Error> {
    if tagged_key[0] != X25519_KEY_TAG {
        return Err(Error::Key);
    }

    let mut scalar = [0; 32];
    scalar.copy_from_slice(&tagged_key[1..]);

    if !canonical(&scalar) {
        return Err(Error::Key);
    }

    Ok(scalar)
}

/// Rejects a u-coordinate that is not the canonical representative.
///
/// The high bit is a sign bit in the curve encoding, and a value at or above
/// the field prime is not an X25519 scalar this crate can use.
fn canonical(scalar: &[u8; 32]) -> bool {
    if scalar[SIGN_BYTE] & 0x80 != 0 {
        return false;
    }

    let mut prime = [0xff; 32];
    prime[0] = CURVE_LOW;
    prime[SIGN_BYTE] = CLAMP_BYTE;

    for index in (0..scalar.len()).rev() {
        if scalar[index] < prime[index] {
            return true;
        }

        if scalar[index] > prime[index] {
            return false;
        }
    }

    false
}

fn encoding_body(payload: &[u8]) -> Result<&[u8], Error> {
    if payload.len() < PAYLOAD_HEADER_LEN {
        return Err(Error::Plaintext);
    }

    let len = u32::from_le_bytes(
        payload[1..PAYLOAD_HEADER_LEN]
            .try_into()
            .map_err(|_| Error::Plaintext)?,
    ) as usize;
    let end = PAYLOAD_HEADER_LEN
        .checked_add(len)
        .ok_or(Error::Plaintext)?;
    let body = payload
        .get(PAYLOAD_HEADER_LEN..end)
        .ok_or(Error::Plaintext)?;

    if payload.len() != end {
        return Err(Error::Plaintext);
    }

    match payload[0] {
        0 | 1 => Ok(body),
        _ => Err(Error::Plaintext),
    }
}

fn parse_proofless(bytes: &[u8]) -> Result<OpenedNote, Error> {
    if bytes.len() < PROOFLESS_NOTE_PREFIX_LEN {
        return Err(Error::Plaintext);
    }

    let mut owner = [0; OWNER_LEN];
    let mut blinding = [0; OWNER_LEN];
    let mut asset = [0; OWNER_LEN];
    owner.copy_from_slice(&bytes[0..OWNER_LEN]);
    blinding.copy_from_slice(&bytes[OWNER_LEN..BLINDING_END]);
    asset.copy_from_slice(&bytes[BLINDING_END..ASSET_END]);

    let amount = u64::from_le_bytes(
        bytes[ASSET_END..PROOFLESS_NOTE_PREFIX_LEN]
            .try_into()
            .map_err(|_| Error::Plaintext)?,
    );
    let tail = &bytes[PROOFLESS_NOTE_PREFIX_LEN..];
    // ProoflessOutput has six Option fields after the fixed prefix:
    // data_hash, utxo_data, ring_program_id, ring_data_hash, ring_data, memo.
    // Admission supports only canonical data-free records: all six None tags.
    let canonical_empty = tail == [0, 0, 0, 0, 0, 0];
    if !canonical_empty {
        // Preserve raw inspection for callers but reject malformed/unsupported
        // extension shapes rather than silently accepting them as data-free.
        // We only accept the fully canonical empty tail here.
        return Err(Error::Plaintext);
    }

    Ok(OpenedNote::Proofless(ProoflessNote {
        owner,
        blinding,
        asset,
        amount,
        records: Vec::new(),
    }))
}

#[cfg(test)]
mod tests {
    use super::{ViewingKey, apply_ctr, canonical, coordinate, derive_key, diffie_hellman, tagged};
    use crate::encode::{ConfidentialPlaintext, read_confidential, write_confidential};
    use crate::{Error, OpenedNote, ProoflessNote};
    use curve25519_dalek::montgomery::MontgomeryPoint;

    fn seal(
        ephemeral: &[u8; 32],
        recipient: &[u8; 33],
        salt: &[u8; 16],
        slot: u32,
        plaintext: &[u8],
    ) -> Vec<u8> {
        let ephemeral_u = MontgomeryPoint::mul_base_clamped(*ephemeral).to_bytes();
        let shared =
            diffie_hellman(ephemeral, &coordinate(recipient).expect("recipient")).expect("ecdh");
        let (key, nonce) = derive_key(&shared, &ephemeral_u, recipient, salt, slot).expect("hkdf");
        let mut buffer = plaintext.to_vec();
        apply_ctr(&key, &nonce, &mut buffer);
        buffer
    }

    #[test]
    fn field_prime_is_not_a_canonical_coordinate() {
        let mut prime = [0xff; 32];
        prime[0] = 237;
        prime[31] = 127;
        assert!(!canonical(&prime));
        prime[0] -= 1;
        assert!(canonical(&prime));
        assert!(tagged(&prime).is_ok());
    }

    #[test]
    fn ciphertext_matches_x25519_dalek() {
        // Produced by x25519-dalek 2.0.1 StaticSecret, the type zolana v0.8.0 uses,
        // with the same HKDF info and AES-256-CTR counter. Secrets are [7;32] and [9;32].
        let viewing = ViewingKey::from_bytes([7; 32]);
        let recipient = viewing.public_key().expect("viewing key");
        assert_eq!(
            &recipient[1..],
            hex("13be4feaeaf204c7fd3358fc9c00721881d174278128227ec674f37f7fe97b6d").as_slice()
        );
        let note = ConfidentialPlaintext {
            asset_id: 1,
            amount: 99,
            blinding: [2; 32],
            ring_program_id: None,
            records: Vec::new(),
        };
        let plaintext = write_confidential(&note);
        let ciphertext = seal(&[9; 32], &recipient, &[3; 16], 1, &plaintext);
        assert_eq!(
            ciphertext,
            hex(
                "54b5db33bd248172a33551ffca34472b3bbe001eeda2cb7a1e6191e630462e52294756409957dab33fc95c9eaafc3e404fb6"
            )
        );
    }

    fn hex(text: &str) -> Vec<u8> {
        (0..text.len())
            .step_by(2)
            .map(|index| u8::from_str_radix(&text[index..index + 2], 16).expect("hex"))
            .collect()
    }

    #[test]
    fn confidential_round_trip_uses_the_output_slot() {
        let viewing = ViewingKey::from_bytes([7; 32]);
        let recipient = viewing.public_key().expect("viewing key");
        let note = ConfidentialPlaintext {
            asset_id: 1,
            amount: 99,
            blinding: [2; 32],
            ring_program_id: None,
            records: Vec::new(),
        };
        let plaintext = write_confidential(&note);
        let salt = [3; 16];
        let ciphertext = seal(&[9; 32], &recipient, &salt, 1, &plaintext);
        let ephemeral =
            tagged(&MontgomeryPoint::mul_base_clamped([9; 32]).to_bytes()).expect("ephemeral");

        let mut body = recipient.to_vec();
        body.extend(ciphertext);
        let mut payload = vec![1];
        payload.extend(u32::try_from(1 + body.len()).expect("len").to_le_bytes());
        payload.push(3);
        payload.extend(body);
        let opened = viewing
            .open_output(&ephemeral, &salt, 1, &payload)
            .expect("open");

        assert!(matches!(opened, OpenedNote::Confidential(ref plain) if plain.amount == 99));

        assert!(viewing.open_output(&ephemeral, &salt, 2, &payload).is_err());
        let other = ViewingKey::from_bytes([8; 32]);
        assert!(matches!(
            other.open_output(&ephemeral, &salt, 1, &payload),
            Err(Error::NotForViewer)
        ));
        let parsed = read_confidential(&plaintext).expect("parsed");
        assert_eq!(parsed.amount, note.amount);
    }

    #[test]
    fn proofless_canonical_data_free_payload_is_accepted() {
        let mut body = vec![0u8];
        body.extend([4; 32]); // owner
        body.extend([5; 32]); // blinding
        body.extend([0; 32]); // asset (SOL mint)
        body.extend(40u64.to_le_bytes());
        body.extend([0, 0, 0, 0, 0, 0]); // canonical empty options
        let mut payload = vec![0];
        payload.extend(u32::try_from(body.len()).expect("len").to_le_bytes());
        payload.extend(body);

        let viewing = ViewingKey::from_bytes([1; 32]);
        let opened = viewing
            .open_output(&[4; 33], &[0; 16], 0, &payload)
            .expect("proofless");
        assert_eq!(
            opened,
            OpenedNote::Proofless(ProoflessNote {
                owner: [4; 32],
                blinding: [5; 32],
                asset: [0; 32],
                amount: 40,
                records: Vec::new(),
            })
        );
    }

    #[test]
    fn proofless_with_extension_data_is_rejected() {
        let mut body = vec![0u8];
        body.extend([4; 32]);
        body.extend([5; 32]);
        body.extend([0; 32]);
        body.extend(40u64.to_le_bytes());
        // utxo_data present: data_hash None, utxo_data Some(empty vec encoded later?) - make simple non-empty option
        body.extend([0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]); // malformed/unsupported tail
        let mut payload = vec![0];
        payload.extend(u32::try_from(body.len()).expect("len").to_le_bytes());
        payload.extend(body);

        let viewing = ViewingKey::from_bytes([1; 32]);
        assert!(matches!(
            viewing.open_output(&[4; 33], &[0; 16], 0, &payload),
            Err(Error::Plaintext)
        ));
    }

    #[test]
    fn oversized_payload_length_is_rejected() {
        let viewing = ViewingKey::from_bytes([1; 32]);
        let error = viewing
            .open_output(&[4; 33], &[0; 16], 0, &[1, 255, 255, 255, 255])
            .expect_err("length");
        assert!(matches!(error, Error::Plaintext));
    }

    #[test]
    fn published_payload_is_encrypted_confidential() {
        let bytes = crate::decode_base64(
            "AVQAAAADBIs8UXWNMRXj0LYmli1ENBZyF5MaxY28XHJEj0+5z8Y2iCHFvZnXUY8xSxfga5e4QmD/NsPxsja5BbPBsI7tvH69q5E7NwC1p9JIVwbv/RfwioI=",
        )
        .expect("payload");
        assert_eq!(bytes.len(), 89);
        assert_eq!(&bytes[..7], &[1, 0x54, 0, 0, 0, 3, 4]);
    }

    fn framed(body: &[u8]) -> Vec<u8> {
        let mut payload = vec![1];
        payload.extend(u32::try_from(body.len()).expect("len").to_le_bytes());
        payload.extend(body);
        payload
    }

    #[test]
    fn rejected_payloads_stay_closed() {
        let viewing = ViewingKey::from_bytes([1; 32]);
        assert!(format!("{viewing:?}").contains("ViewingKey"));
        assert!(viewing.open_output(&[4; 33], &[0; 16], 0, &[]).is_err());
        assert!(
            viewing
                .open_output(&[4; 33], &[0; 16], 0, &[1, 0, 0, 0, 0, 9])
                .is_err()
        );
        assert!(
            viewing
                .open_output(&[4; 33], &[0; 16], 0, &[2, 0, 0, 0, 0])
                .is_err()
        );
        let untagged = [0; 33];
        let mut keyed = vec![3];
        keyed.extend(viewing.public_key().expect("key"));
        keyed.extend([1, 2, 3]);
        assert!(
            viewing
                .open_output(&untagged, &[0; 16], 0, &framed(&keyed))
                .is_err()
        );
        assert!(
            viewing
                .open_output(&[4; 33], &[0; 16], 0, &framed(&[9]))
                .is_err()
        );
        assert!(
            viewing
                .open_output(&[4; 33], &[0; 16], 0, &framed(&[3]))
                .is_err()
        );
        assert!(
            viewing
                .open_output(&[4; 33], &[0; 16], 0, &framed(&[0]))
                .is_err()
        );
        let mut bad_tag = viewing.public_key().expect("key");
        bad_tag[0] = 0;
        let mut bad_body = vec![3];
        bad_body.extend(bad_tag);
        assert!(
            viewing
                .open_output(&[4; 33], &[0; 16], 0, &framed(&bad_body))
                .is_err()
        );
        let mut high = [0; 33];
        high[0] = 4;
        high[32] = 0x80;
        let mut confidential = vec![3];
        confidential.extend(viewing.public_key().expect("key"));
        confidential.extend([1, 2, 3]);
        assert!(
            viewing
                .open_output(&high, &[0; 16], 0, &framed(&confidential))
                .is_err()
        );
        let mut above = [0xff; 32];
        above[31] = 127;
        above[0] = 238;
        assert!(tagged(&above).is_err());
        let mut sign = [0; 32];
        sign[31] = 0x80;
        assert!(tagged(&sign).is_err());
        let mut zero_peer = [0; 33];
        zero_peer[0] = 4;
        assert!(
            viewing
                .open_output(&zero_peer, &[0; 16], 0, &framed(&confidential))
                .is_err()
        );
        let peer = ViewingKey::from_bytes([9; 32]).public_key().expect("peer");
        assert!(
            viewing
                .open_output(&peer, &[0; 16], 0, &framed(&confidential))
                .is_err()
        );
    }
}
