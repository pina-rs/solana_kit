//! Viewing-key decryption.
//!
//! The scheme is the v0.8.0 `spr-keypair` path: tagged X25519 (`0x04 || u`),
//! HKDF-SHA256 over `TSPP/hpke/ || TSPP/tx || salt || slot`, then AES-256-CTR
//! whose first keystream block is counter 2. Poseidon commitment checks stay
//! with the caller; this module does not claim a note is unspent.

use std::collections::BTreeMap;

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
        if body.len() < 33 {
            return Err(Error::Plaintext);
        }
        let mut embedded = [0; 33];
        embedded.copy_from_slice(&body[..33]);
        if embedded != self.public_key()? {
            return Err(Error::NotForViewer);
        }
        let plaintext = decrypt_ciphertext(self, tx_viewing_pk, salt, slot_index, &body[33..])?;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProoflessNote {
    /// Recipient owner hash.
    pub owner: [u8; 32],
    /// Deposit blinding.
    pub blinding: [u8; 32],
    /// Mint, or the zero address for SOL.
    pub asset: [u8; 32],
    /// Amount in base units.
    pub amount: u64,
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

/// Sum of unspent notes that share an asset key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssetBalance {
    /// Confidential asset id, or `None` for a proofless mint.
    pub asset_id: Option<u64>,
    /// Proofless mint, or `None` for a confidential asset id.
    pub mint: Option<[u8; 32]>,
    /// Total base units.
    pub amount: u64,
}

/// Sums notes whose `(tree_id, leaf_index)` is not in `spent`.
///
/// The same leaf is counted once. Two different notes for one leaf are
/// rejected. Confidential and proofless keys stay separate: a confidential
/// asset id is not assumed to be a mint. This function does not check a
/// proofless owner or a confidential commitment, and it does not derive
/// nullifiers.
///
/// # Errors
/// Rejects a leaf that appears twice with different contents, or a total that
/// overflows `u64`.
pub fn private_balances(
    notes: &[TrackedNote],
    spent: &std::collections::BTreeSet<(u16, u64)>,
) -> Result<Vec<AssetBalance>, Error> {
    let mut seen: BTreeMap<(u16, u64), &OpenedNote> = BTreeMap::new();
    let mut totals: BTreeMap<(u8, [u8; 8], [u8; 32]), u64> = BTreeMap::new();
    for note in notes {
        if spent.contains(&(note.tree_id, note.leaf_index)) {
            continue;
        }
        match seen.entry((note.tree_id, note.leaf_index)) {
            std::collections::btree_map::Entry::Occupied(existing) => {
                if existing.get() != &&note.note {
                    return Err(Error::Limit("conflicting leaf"));
                }
                continue;
            }
            std::collections::btree_map::Entry::Vacant(slot) => {
                slot.insert(&note.note);
            }
        }
        let (kind, id, mint, amount) = match &note.note {
            OpenedNote::Confidential(plain) => {
                (0, plain.asset_id.to_le_bytes(), [0; 32], plain.amount)
            }
            OpenedNote::Proofless(plain) => (1, [0; 8], plain.asset, plain.amount),
        };
        let entry = totals.entry((kind, id, mint)).or_insert(0);
        *entry = entry
            .checked_add(amount)
            .ok_or(Error::Limit("balance overflow"))?;
    }
    Ok(totals
        .into_iter()
        .map(|((kind, id, mint), amount)| AssetBalance {
            asset_id: (kind == 0).then(|| u64::from_le_bytes(id)),
            mint: (kind == 1).then_some(mint),
            amount,
        })
        .collect())
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
    let mut iv = [0; 16];
    iv[..12].copy_from_slice(nonce);
    iv[15] = 2;
    let mut cipher = Aes256Ctr::new(key.into(), (&iv).into());
    cipher.apply_keystream(buffer);
}

fn tagged(u: &[u8; 32]) -> Result<[u8; 33], Error> {
    if !canonical(u) {
        return Err(Error::Key);
    }
    let mut bytes = [X25519_KEY_TAG; 33];
    bytes[1..].copy_from_slice(u);
    Ok(bytes)
}

fn coordinate(tagged_key: &[u8; 33]) -> Result<[u8; 32], Error> {
    if tagged_key[0] != X25519_KEY_TAG {
        return Err(Error::Key);
    }
    let mut u = [0; 32];
    u.copy_from_slice(&tagged_key[1..]);
    if !canonical(&u) {
        return Err(Error::Key);
    }
    Ok(u)
}

fn canonical(u: &[u8; 32]) -> bool {
    if u[31] & 0x80 != 0 {
        return false;
    }
    let mut prime = [0xff; 32];
    prime[0] = 237;
    prime[31] = 127;
    for index in (0..32).rev() {
        if u[index] < prime[index] {
            return true;
        }
        if u[index] > prime[index] {
            return false;
        }
    }
    false
}

fn encoding_body(payload: &[u8]) -> Result<&[u8], Error> {
    if payload.len() < 5 {
        return Err(Error::Plaintext);
    }
    let len = u32::from_le_bytes(payload[1..5].try_into().map_err(|_| Error::Plaintext)?) as usize;
    let end = 5usize.checked_add(len).ok_or(Error::Plaintext)?;
    let body = payload.get(5..end).ok_or(Error::Plaintext)?;
    if payload.len() != end {
        return Err(Error::Plaintext);
    }
    match payload[0] {
        0 | 1 => Ok(body),
        _ => Err(Error::Plaintext),
    }
}

fn parse_proofless(bytes: &[u8]) -> Result<OpenedNote, Error> {
    if bytes.len() < 104 {
        return Err(Error::Plaintext);
    }
    let mut owner = [0; 32];
    let mut blinding = [0; 32];
    let mut asset = [0; 32];
    owner.copy_from_slice(&bytes[0..32]);
    blinding.copy_from_slice(&bytes[32..64]);
    asset.copy_from_slice(&bytes[64..96]);
    let amount = u64::from_le_bytes(bytes[96..104].try_into().map_err(|_| Error::Plaintext)?);
    Ok(OpenedNote::Proofless(ProoflessNote {
        owner,
        blinding,
        asset,
        amount,
    }))
}

#[cfg(test)]
mod tests {
    use super::{ViewingKey, apply_ctr, canonical, coordinate, derive_key, diffie_hellman, tagged};
    use crate::encode::{ConfidentialPlaintext, read_confidential, write_confidential};
    use crate::{OpenedNote, TrackedNote, decode_base64, private_balances};
    use curve25519_dalek::montgomery::MontgomeryPoint;
    use std::collections::BTreeSet;

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
        match opened {
            OpenedNote::Confidential(plain) => assert_eq!(plain.amount, 99),
            OpenedNote::Proofless(_) => panic!("confidential note"),
        }
        assert!(viewing.open_output(&ephemeral, &salt, 2, &payload).is_err());
        let other = ViewingKey::from_bytes([8; 32]);
        assert!(matches!(
            other.open_output(&ephemeral, &salt, 1, &payload),
            Err(crate::Error::NotForViewer)
        ));
        let parsed = read_confidential(&plaintext).expect("parsed");
        assert_eq!(parsed.amount, note.amount);
    }

    #[test]
    fn proofless_prefix_and_spent_notes_sum() {
        let mut body = vec![0];
        body.extend([4; 32]);
        body.extend([5; 32]);
        body.extend([0; 32]);
        body.extend(40u64.to_le_bytes());
        let mut payload = vec![0];
        payload.extend(u32::try_from(body.len()).expect("len").to_le_bytes());
        payload.extend(body);
        let viewing = ViewingKey::from_bytes([1; 32]);
        let opened = viewing
            .open_output(&[4; 33], &[0; 16], 0, &payload)
            .expect("proofless");
        let OpenedNote::Proofless(note) = opened else {
            panic!("proofless");
        };
        assert_eq!(note.amount, 40);
        assert_eq!(note.asset, [0; 32]);
        let notes = [
            TrackedNote {
                tree_id: 0,
                leaf_index: 1,
                note: OpenedNote::Proofless(note),
            },
            TrackedNote {
                tree_id: 0,
                leaf_index: 2,
                note: OpenedNote::Proofless(super::ProoflessNote {
                    owner: [4; 32],
                    blinding: [5; 32],
                    asset: [0; 32],
                    amount: 2,
                }),
            },
        ];
        let mut spent = BTreeSet::new();
        spent.insert((0, 2));
        let totals = private_balances(&notes, &spent).expect("sum");
        assert_eq!(totals.len(), 1);
        assert_eq!(totals[0].amount, 40);
        assert_eq!(totals[0].mint, Some([0; 32]));
        assert_eq!(totals[0].asset_id, None);
        let repeated = [notes[0].clone(), notes[0].clone()];
        let once = private_balances(&repeated, &BTreeSet::new()).expect("dedup");
        assert_eq!(once[0].amount, 40);
        let mut conflict = notes[0].clone();
        if let OpenedNote::Proofless(plain) = &mut conflict.note {
            plain.amount = 1;
        }
        assert!(matches!(
            private_balances(&[notes[0].clone(), conflict], &BTreeSet::new()),
            Err(crate::Error::Limit("conflicting leaf"))
        ));
    }

    #[test]
    fn oversized_payload_length_is_rejected() {
        let viewing = ViewingKey::from_bytes([1; 32]);
        let error = viewing
            .open_output(&[4; 33], &[0; 16], 0, &[1, 255, 255, 255, 255])
            .expect_err("length");
        assert!(matches!(error, crate::Error::Plaintext));
    }

    #[test]
    fn published_payload_is_encrypted_confidential() {
        let bytes = decode_base64(
            "AVQAAAADBIs8UXWNMRXj0LYmli1ENBZyF5MaxY28XHJEj0+5z8Y2iCHFvZnXUY8xSxfga5e4QmD/NsPxsja5BbPBsI7tvH69q5E7NwC1p9JIVwbv/RfwioI=",
        )
        .expect("payload");
        assert_eq!(bytes.len(), 89);
        assert_eq!(&bytes[..7], &[1, 0x54, 0, 0, 0, 3, 4]);
    }
}
