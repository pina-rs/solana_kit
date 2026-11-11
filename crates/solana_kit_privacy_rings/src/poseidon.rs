//! Poseidon field hashing used by Privacy Rings proofs.
//!
//! The permutations are Circom BN254 Poseidon, the same construction
//! `spr-hasher` calls through `light-poseidon`. Domain tags, right-alignment,
//! packed byte hashing, the UTXO commitment, and the nullifier preimage follow
//! the upstream circuit gadgets and the published derivation vectors.

use ark_bn254::Fr;
use light_poseidon::{Poseidon, PoseidonBytesHasher};

use crate::Error;

const FIELD_BYTES: usize = 32;
const PACK_CHUNK: usize = 31;
const MAX_POSEIDON_INPUTS: usize = 12;
const SOLANA_OWNER_TAG: u8 = b'S';
const DOMAIN_OUTPUT_BLINDING_SEED: u32 = 0x5458_4f53;
const DOMAIN_OUTPUT_BLINDING: u32 = 0x5458_4f42;
const DOMAIN_PRIVATE_TX_BLINDING: u32 = 0x5458_5042;

/// One UTXO the commitment gadget hashes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UtxoPreimage {
    /// Circuit domain separator.
    pub domain: [u8; FIELD_BYTES],
    /// State tree id, right-aligned.
    pub tree_id: [u8; FIELD_BYTES],
    /// Owner field stored on the note.
    pub owner: [u8; FIELD_BYTES],
    /// Asset id, right-aligned.
    pub asset: [u8; FIELD_BYTES],
    /// Amount, right-aligned.
    pub amount: [u8; FIELD_BYTES],
    /// Note blinding.
    pub blinding: [u8; FIELD_BYTES],
    /// Extra data hash.
    pub data_hash: [u8; FIELD_BYTES],
    /// Ring data hash.
    pub ring_data_hash: [u8; FIELD_BYTES],
    /// Ring program id.
    pub ring_program_id: [u8; FIELD_BYTES],
}

/// Right-aligns `bytes` in a 32-byte big-endian field element.
///
/// # Errors
/// Rejects a value longer than one field.
pub fn right_align(bytes: &[u8]) -> Result<[u8; FIELD_BYTES], Error> {
    if bytes.len() > FIELD_BYTES {
        return Err(Error::Limit("field"));
    }

    let mut field = [0u8; FIELD_BYTES];
    let start = FIELD_BYTES - bytes.len();
    field[start..].copy_from_slice(bytes);
    Ok(field)
}

/// Circom Poseidon over big-endian field elements.
///
/// # Errors
/// Rejects the wrong width, a non-field input, or a hasher failure.
pub fn poseidon_hash(inputs: &[&[u8]]) -> Result<[u8; FIELD_BYTES], Error> {
    if inputs.is_empty() || inputs.len() > MAX_POSEIDON_INPUTS {
        return Err(Error::Limit("poseidon width"));
    }

    for input in inputs {
        if input.len() != FIELD_BYTES {
            return Err(Error::Limit("poseidon field"));
        }
    }

    hash_fields(inputs)
}

/// `Poseidon(TXOS, first_nullifier, blinding_seed)`.
///
/// # Errors
/// Rejects a field the hasher will not accept.
pub fn derive_output_blinding_seed(
    first_nullifier: &[u8; FIELD_BYTES],
    blinding_seed: &[u8; FIELD_BYTES],
) -> Result<[u8; FIELD_BYTES], Error> {
    let domain = domain_field(DOMAIN_OUTPUT_BLINDING_SEED);
    let seed = right_align(blinding_seed)?;
    poseidon_hash(&[&domain, first_nullifier, &seed])
}

/// `Poseidon(TXPB, first_nullifier, secret)`.
///
/// # Errors
/// Rejects a field the hasher will not accept.
pub fn derive_private_tx_blinding(
    first_nullifier: &[u8; FIELD_BYTES],
    secret: &[u8; FIELD_BYTES],
) -> Result<[u8; FIELD_BYTES], Error> {
    let domain = domain_field(DOMAIN_PRIVATE_TX_BLINDING);
    let aligned = right_align(secret)?;
    poseidon_hash(&[&domain, first_nullifier, &aligned])
}

/// `Poseidon(TXOB, first_nullifier, seed, output_index)`.
///
/// # Errors
/// Rejects a field the hasher will not accept.
pub fn derive_transact_output_blinding(
    first_nullifier: &[u8; FIELD_BYTES],
    seed: &[u8; FIELD_BYTES],
    output_index: u32,
) -> Result<[u8; FIELD_BYTES], Error> {
    let domain = domain_field(DOMAIN_OUTPUT_BLINDING);
    let aligned = right_align(seed)?;
    let index = right_align(&output_index.to_be_bytes())?;
    poseidon_hash(&[&domain, first_nullifier, &aligned, &index])
}

/// Owner identity `hash_bytes(S || pubkey)`.
///
/// # Errors
/// Rejects a hasher failure.
pub fn solana_owner_identity(pubkey: &[u8; FIELD_BYTES]) -> Result<[u8; FIELD_BYTES], Error> {
    let mut tagged = [0u8; FIELD_BYTES + 1];
    tagged[0] = SOLANA_OWNER_TAG;
    tagged[1..].copy_from_slice(pubkey);
    hash_bytes(&tagged)
}

/// `Poseidon(owner_identity, nullifier_pubkey)`.
///
/// # Errors
/// Rejects a hasher failure.
pub fn owner_hash(
    signing_pubkey: &[u8; FIELD_BYTES],
    nullifier_pubkey: &[u8; FIELD_BYTES],
) -> Result<[u8; FIELD_BYTES], Error> {
    let identity = solana_owner_identity(signing_pubkey)?;
    poseidon_hash(&[&identity, nullifier_pubkey])
}

/// Poseidon image of the nullifier secret. This is the nullifier public key.
///
/// # Errors
/// Rejects a hasher failure.
pub fn nullifier_pubkey(secret: &[u8; FIELD_BYTES]) -> Result<[u8; FIELD_BYTES], Error> {
    let aligned = right_align(secret)?;
    poseidon_hash(&[&aligned])
}

/// `Poseidon(utxo_hash, blinding, nullifier_secret)`.
///
/// # Errors
/// Rejects a hasher failure.
pub fn nullifier(
    utxo_hash: &[u8; FIELD_BYTES],
    blinding: &[u8; FIELD_BYTES],
    secret: &[u8; FIELD_BYTES],
) -> Result<[u8; FIELD_BYTES], Error> {
    let aligned = right_align(secret)?;
    poseidon_hash(&[utxo_hash, blinding, &aligned])
}

/// UTXO commitment from the circuit gadget.
///
/// The gadget hashes the owner with the blinding, hashes the ring fields, then
/// hashes `domain, tree_id, asset, amount, data_hash, ring_hash, owner_hash`.
///
/// # Errors
/// Rejects a hasher failure.
pub fn utxo_commitment(preimage: &UtxoPreimage) -> Result<[u8; FIELD_BYTES], Error> {
    let owner_hash = poseidon_hash(&[&preimage.owner, &preimage.blinding])?;
    let ring_hash = poseidon_hash(&[&preimage.ring_data_hash, &preimage.ring_program_id])?;
    poseidon_hash(&[
        &preimage.domain,
        &preimage.tree_id,
        &preimage.asset,
        &preimage.amount,
        &preimage.data_hash,
        &ring_hash,
        &owner_hash,
    ])
}

fn domain_field(tag: u32) -> [u8; FIELD_BYTES] {
    let mut field = [0u8; FIELD_BYTES];
    field[FIELD_BYTES - tag.to_be_bytes().len()..].copy_from_slice(&tag.to_be_bytes());
    field
}

fn hash_fields(inputs: &[&[u8]]) -> Result<[u8; FIELD_BYTES], Error> {
    let mut hasher =
        Poseidon::<Fr>::new_circom(inputs.len()).map_err(|_| Error::Limit("poseidon"))?;
    hasher
        .hash_bytes_be(inputs)
        .map_err(|_| Error::Limit("poseidon"))
}

pub(crate) fn hash_bytes(bytes: &[u8]) -> Result<[u8; FIELD_BYTES], Error> {
    let mut chunks = bytes.chunks(PACK_CHUNK);
    let Some(first) = chunks.next() else {
        return Ok([0; FIELD_BYTES]);
    };

    let mut folded = pack_chunk(first);

    for chunk in chunks {
        let packed = pack_chunk(chunk);
        folded = poseidon_hash(&[&folded, &packed])?;
    }

    Ok(folded)
}

fn pack_chunk(chunk: &[u8]) -> [u8; FIELD_BYTES] {
    let mut field = [0u8; FIELD_BYTES];
    let start = FIELD_BYTES - chunk.len();
    field[start..].copy_from_slice(chunk);
    field
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_SEVEN: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 7,
    ];
    const SMALL_FORTY_TWO: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 42,
    ];
    const OUTPUT_BLINDING_SEED: [u8; 32] = [
        0x06, 0xbc, 0xa3, 0x16, 0x06, 0x66, 0x30, 0x05, 0x65, 0x39, 0x77, 0x2e, 0x0d, 0x19, 0xf5,
        0xd9, 0x45, 0x33, 0x31, 0xf9, 0xdd, 0xa2, 0xf0, 0xd0, 0x8e, 0xe0, 0x63, 0x2b, 0x6b, 0x51,
        0x2d, 0xe3,
    ];
    const PRIVATE_TX_BLINDING: [u8; 32] = [
        0x19, 0x91, 0xf1, 0x66, 0x20, 0x8c, 0x44, 0x0b, 0xa5, 0xec, 0xdb, 0x0f, 0x8c, 0xcc, 0x79,
        0x2e, 0x8d, 0x27, 0x39, 0x03, 0x8a, 0xe9, 0xd9, 0x98, 0x62, 0x62, 0x1c, 0xf2, 0xc2, 0x92,
        0xf6, 0x10,
    ];
    const OUTPUT_BLINDING: [u8; 32] = [
        0x06, 0x26, 0x15, 0x40, 0xe8, 0x57, 0xfe, 0xbb, 0x5f, 0x8d, 0x59, 0xeb, 0x74, 0x2a, 0xd3,
        0xd4, 0xd8, 0x20, 0x0f, 0xf3, 0x8c, 0xcb, 0xf2, 0xea, 0x16, 0xcd, 0x1e, 0x0a, 0x90, 0x85,
        0xe8, 0x81,
    ];
    const SOLANA_PUBKEY: [u8; 32] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
        0xff, 0x10, 0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87, 0x98, 0xa9, 0xba, 0xbb, 0xdc, 0xdd,
        0xed, 0xef,
    ];
    const SOLANA_PK_HASH: [u8; 32] = [
        0x09, 0x2a, 0xcb, 0x76, 0x56, 0x8e, 0x6c, 0x3d, 0xd1, 0x77, 0xe9, 0xb6, 0x93, 0x1e, 0x87,
        0xc2, 0xa2, 0xb6, 0x56, 0xbf, 0xac, 0xb7, 0x29, 0x69, 0x42, 0xa5, 0x90, 0x81, 0x29, 0x46,
        0x2f, 0x4c,
    ];
    const OUTPUT_INDEX: u32 = 3;

    #[test]
    fn derivation_vectors_match_the_circuit() {
        assert_eq!(
            derive_output_blinding_seed(&SMALL_SEVEN, &SMALL_FORTY_TWO).unwrap(),
            OUTPUT_BLINDING_SEED
        );
        assert_eq!(
            derive_private_tx_blinding(&SMALL_SEVEN, &SMALL_FORTY_TWO).unwrap(),
            PRIVATE_TX_BLINDING
        );
        assert_eq!(
            derive_transact_output_blinding(&SMALL_SEVEN, &SMALL_FORTY_TWO, OUTPUT_INDEX).unwrap(),
            OUTPUT_BLINDING
        );
        assert_ne!(PRIVATE_TX_BLINDING, OUTPUT_BLINDING_SEED);
    }

    #[test]
    fn solana_owner_identity_matches_the_field_vector() {
        assert_eq!(
            solana_owner_identity(&SOLANA_PUBKEY).unwrap(),
            SOLANA_PK_HASH
        );
    }

    #[test]
    fn poseidon_rejects_a_bad_width_and_a_short_field() {
        assert!(matches!(
            poseidon_hash(&[]),
            Err(Error::Limit("poseidon width"))
        ));
        assert!(matches!(
            poseidon_hash(&[&[1, 2, 3]]),
            Err(Error::Limit("poseidon field"))
        ));
        assert!(matches!(right_align(&[0; 33]), Err(Error::Limit("field"))));
    }

    #[test]
    fn owner_hash_and_a_non_canonical_field() {
        let public = nullifier_pubkey(&SMALL_SEVEN).unwrap();
        assert!(owner_hash(&SOLANA_PUBKEY, &public).is_ok());
        assert!(poseidon_hash(&[&[0xff; 32]]).is_err());
        let wide: Vec<&[u8]> = std::iter::repeat_n(SMALL_SEVEN.as_slice(), 13).collect();
        assert!(hash_fields(&wide).is_err());
        assert_eq!(hash_bytes(&[1]).unwrap()[31], 1);
        assert_eq!(right_align(&[9]).unwrap()[31], 9);
    }

    #[test]
    fn empty_byte_string_hashes_to_zero() {
        assert_eq!(hash_bytes(&[]).unwrap(), [0; FIELD_BYTES]);
    }

    fn field(value: &serde_json::Value) -> [u8; 32] {
        let text = value.as_str().unwrap().trim_start_matches("0x");
        let mut bytes = [0u8; 32];

        for (index, slot) in bytes.iter_mut().enumerate() {
            let start = index * 2;
            *slot = u8::from_str_radix(&text[start..start + 2], 16).unwrap();
        }

        bytes
    }

    #[test]
    fn published_witness_nullifier_matches_the_commitment() {
        let value: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/transfer-2x2.json")).unwrap();
        let input = &value["inputs"][0];
        let utxo = &input["utxo"];
        let preimage = UtxoPreimage {
            domain: field(&utxo["domain"]),
            tree_id: field(&value["treeSlots"][0]["id"]),
            owner: field(&utxo["owner"]),
            asset: field(&utxo["asset"]),
            amount: field(&utxo["amount"]),
            blinding: field(&utxo["blinding"]),
            data_hash: field(&utxo["dataHash"]),
            ring_data_hash: field(&utxo["ringDataHash"]),
            ring_program_id: field(&utxo["ringProgramId"]),
        };
        let commitment = utxo_commitment(&preimage).unwrap();
        let spent = nullifier(
            &commitment,
            &preimage.blinding,
            &field(&input["nullifierSecret"]),
        )
        .unwrap();
        assert_eq!(spent, field(&input["nullifier"]));
    }
}
