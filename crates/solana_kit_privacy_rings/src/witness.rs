//! Gnark witness for a confidential transfer.
//!
//! The caller supplies opened notes, the Merkle paths that place them, and
//! the public fields the transact instruction already committed to. This
//! module fills the Poseidon nullifiers, output blindings, output
//! commitments, `privateTxHash`, and `publicInputHash`. The JSON object is
//! the body of `POST /prove/transfer_confidential_*`.

use serde_json::{Map, Value};

use crate::error::Error;
use crate::poseidon::{
    UtxoPreimage, derive_output_blinding_seed, derive_private_tx_blinding,
    derive_transact_output_blinding, hash_bytes, nullifier, poseidon_hash, right_align,
    solana_owner_identity, utxo_commitment,
};

const FIELD_BYTES: usize = 32;
const PUBLIC_SLOTS: usize = 3;
const INPUT_TREES: usize = 5;
const UTXO_DOMAIN: u8 = 3;
const ADDRESS_DOMAIN: u8 = 2;
const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

/// Eight note fields the transfer circuit hashes, each one field element.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NotePreimage {
    /// Circuit domain separator.
    pub domain: [u8; FIELD_BYTES],
    /// Owner field stored on the note.
    pub owner: [u8; FIELD_BYTES],
    /// Asset id.
    pub asset: [u8; FIELD_BYTES],
    /// Amount.
    pub amount: [u8; FIELD_BYTES],
    /// Note blinding. Output notes replace this with the derived blinding.
    pub blinding: [u8; FIELD_BYTES],
    /// Application data hash. Zero when the note carries none.
    pub data_hash: [u8; FIELD_BYTES],
    /// Ring data hash. Zero outside a custom ring.
    pub ring_data_hash: [u8; FIELD_BYTES],
    /// Ring program id field. Zero outside a custom ring.
    pub ring_program_id: [u8; FIELD_BYTES],
}

impl std::fmt::Debug for NotePreimage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Security: the amount and blinding are secrets and every field is
        // note content, so Debug keeps only the type.
        formatter
            .debug_struct("NotePreimage")
            .finish_non_exhaustive()
    }
}

/// One public tree the witness names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeSlotWitness {
    /// Tree id field.
    pub id: [u8; FIELD_BYTES],
    /// UTXO Merkle root.
    pub utxo_root: [u8; FIELD_BYTES],
    /// Nullifier Merkle root.
    pub nullifier_root: [u8; FIELD_BYTES],
}

/// One input note plus the inclusion and non-inclusion paths that spend it.
#[derive(Clone, PartialEq, Eq)]
pub struct SpendNote {
    /// Opened note. Its blinding is the value committed in the state tree.
    pub note: NotePreimage,
    /// Index into [`TransferWitness::tree_slots`]. The commitment uses that id.
    pub tree_slot: u64,
    /// Padding input.
    pub is_dummy: bool,
    /// Owner public-key hash.
    pub owner_public_key_hash: [u8; FIELD_BYTES],
    /// Nullifier secret. A short secret is right-aligned inside the field.
    pub nullifier_secret: [u8; FIELD_BYTES],
    /// State-tree siblings, leaf to root.
    pub state_path: Vec<[u8; FIELD_BYTES]>,
    /// State-tree leaf index, as a field element.
    pub state_path_index: [u8; FIELD_BYTES],
    /// Non-inclusion low element.
    pub nullifier_low_value: [u8; FIELD_BYTES],
    /// Non-inclusion next element.
    pub nullifier_next_value: [u8; FIELD_BYTES],
    /// Non-inclusion siblings.
    pub nullifier_low_path: Vec<[u8; FIELD_BYTES]>,
    /// Non-inclusion leaf index, as a field element.
    pub nullifier_low_path_index: [u8; FIELD_BYTES],
}

impl std::fmt::Debug for SpendNote {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Security: nullifier_secret and the note preimage are spend secrets
        // whose image is the published nullifier; Debug keeps only the shape.
        formatter
            .debug_struct("SpendNote")
            .field("tree_slot", &self.tree_slot)
            .field("is_dummy", &self.is_dummy)
            .field("state_path_len", &self.state_path.len())
            .field("nullifier_low_path_len", &self.nullifier_low_path.len())
            .finish_non_exhaustive()
    }
}

/// One output note. The circuit replaces `note.blinding`.
#[derive(Clone, PartialEq, Eq)]
pub struct OutputNote {
    /// Note preimage. `blinding` is ignored.
    pub note: NotePreimage,
    /// Padding output.
    pub is_dummy: bool,
    /// Owner public-key hash published for this output.
    pub owner_public_key_hash: [u8; FIELD_BYTES],
    /// Recipient nullifier public key.
    pub nullifier_public_key: [u8; FIELD_BYTES],
}

impl std::fmt::Debug for OutputNote {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Security: the note preimage carries the value and the derived
        // blinding; Debug keeps only the shape.
        formatter
            .debug_struct("OutputNote")
            .field("is_dummy", &self.is_dummy)
            .finish_non_exhaustive()
    }
}

/// One Ed25519 owner signature carried by an in-circuit rail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedOwner {
    /// Owner public key, 32 bytes.
    pub public_key: [u8; FIELD_BYTES],
    /// Ed25519 signature, 64 bytes.
    pub signature: [u8; 64],
    /// Low 128 bits of the message digest, right-aligned.
    pub message_hash_low: [u8; FIELD_BYTES],
    /// High 128 bits of the message digest, right-aligned.
    pub message_hash_high: [u8; FIELD_BYTES],
}

/// Which transfer circuit the witness is for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransferCircuit {
    /// Default-ring confidential transfer. The Solana runtime checks the signers.
    Confidential,
    /// Default-ring confidential transfer whose owner signs inside the circuit.
    ConfidentialEddsa(SignedOwner),
    /// Custom-ring anonymous EdDSA transfer.
    EddsaRing {
        /// Owner signature over the transaction digest.
        owner: SignedOwner,
        /// Default-ring owner identity the anonymous rail publishes.
        default_owner_public_key_hash: [u8; FIELD_BYTES],
    },
}

/// Public inputs and private notes of one confidential transfer.
#[derive(Clone, PartialEq, Eq)]
pub struct TransferWitness {
    /// Notes being spent.
    pub inputs: Vec<SpendNote>,
    /// Notes being created.
    pub outputs: Vec<OutputNote>,
    /// Public tree slots. An input's `tree_slot` indexes this list.
    pub tree_slots: Vec<TreeSlotWitness>,
    /// Tree id mixed into every output commitment.
    pub output_tree_id: [u8; FIELD_BYTES],
    /// Hash of the public transact data.
    pub external_data_hash: [u8; FIELD_BYTES],
    /// Root blinding seed. It must be non-zero.
    pub blinding_seed: [u8; FIELD_BYTES],
    /// Three public asset fields.
    pub public_assets: [[u8; FIELD_BYTES]; PUBLIC_SLOTS],
    /// Three public amount fields, in the same order as `public_assets`.
    pub public_amounts: [[u8; FIELD_BYTES]; PUBLIC_SLOTS],
    /// Ring program id. Zero for the default ring.
    pub ring_program_id: [u8; FIELD_BYTES],
    /// Signer public-key hashes. The confidential circuit wants one more than the input count.
    pub signer_public_key_hashes: Vec<[u8; FIELD_BYTES]>,
    /// `1` when dummy inputs are allowed.
    pub input_flags: [u8; FIELD_BYTES],
    /// Owner hashes published beside the outputs, one per output.
    pub published_output_owner_public_key_hashes: Vec<[u8; FIELD_BYTES]>,
    /// Cache tree id. Zero when the spend does not read a cache.
    pub cache_tree_id: [u8; FIELD_BYTES],
    /// Poseidon chain of the cache reads.
    pub cache_read_hash_chain: [u8; FIELD_BYTES],
    /// Cache read hashes, one per input.
    pub cache_read_hashes: Vec<[u8; FIELD_BYTES]>,
    /// Whether each input was served from the cache.
    pub cache_is_cached: Vec<[u8; FIELD_BYTES]>,
    /// Cache leaf index of each input.
    pub cache_read_index: Vec<[u8; FIELD_BYTES]>,
    /// Circuit rail. It selects the public-input tail and the extra JSON fields.
    pub circuit: TransferCircuit,
}

impl std::fmt::Debug for TransferWitness {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Security: the inputs carry nullifier secrets, amounts, and
        // blindings, and the outputs carry derived blindings; Debug keeps
        // counts and the circuit variant only.
        formatter
            .debug_struct("TransferWitness")
            .field("inputs", &self.inputs.len())
            .field("outputs", &self.outputs.len())
            .field("tree_slots", &self.tree_slots.len())
            .field("circuit", &circuit_name(&self.circuit))
            .finish_non_exhaustive()
    }
}

/// One tree the indexed prover should read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedTree {
    /// Tree account, base58.
    pub tree: String,
    /// Tree id the prepared witness already uses.
    pub id: u64,
}

/// One inclusion or non-inclusion lookup inside an indexed proof request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedLookup {
    /// Tree slot of this input.
    pub tree_slot: u64,
    /// Commitment, base58, when the input is a state inclusion.
    pub commitment: Option<String>,
    /// Nullifier, base58, when the input is a non-inclusion.
    pub nullifier: Option<String>,
}

/// Builds the `transfer-confidential` witness.
///
/// # Errors
/// Rejects an empty spend, a zero blinding seed, a tree slot that is not in
/// `tree_slots`, or a Poseidon input the field cannot hold.
pub fn confidential_transfer_witness(witness: &TransferWitness) -> Result<Value, Error> {
    check_shape(witness)?;
    let spent = spend_inputs(witness)?;
    let created = create_outputs(witness, &spent.first_nullifier)?;
    #[allow(clippy::useless_conversion)]
    let private_tx = private_tx_hash(
        &spent.private_hashes,
        &created.private_hashes,
        &spent.address_nullifiers,
        &spent.first_nullifier,
        &witness.blinding_seed,
    )
    .map_err(Error::from)?;
    let public_hash = public_input_hash(witness, &spent.nullifiers, &created.hashes, &private_tx)?;
    Ok(witness_json(
        witness,
        &spent,
        &created,
        &private_tx,
        &public_hash,
    ))
}

/// Body of `POST /prove/{circuit}/indexed`.
///
/// `prepared` is the object [`confidential_transfer_witness`] returns.
/// Commitments and nullifiers in `lookups` stay base58, which is how the
/// indexer spells them.
///
/// # Errors
/// Rejects a `min_context_slot` that does not fit in a JSON number.
pub fn indexed_proof_request(
    prepared: &Value,
    trees: &[IndexedTree],
    lookups: &[IndexedLookup],
    public_inputs: &[[u8; FIELD_BYTES]],
    min_context_slot: Option<u64>,
) -> Result<Value, Error> {
    if min_context_slot.is_some_and(|slot| slot > MAX_SAFE_INTEGER) {
        return Err(Error::Limit("min context slot"));
    }

    let mut body = Map::new();
    body.insert("circuitType".to_owned(), prepared["circuitType"].clone());
    body.insert("prepared".to_owned(), prepared.clone());
    body.insert("trees".to_owned(), tree_json(trees));
    body.insert("inputs".to_owned(), lookup_json(lookups));
    body.insert("publicInputs".to_owned(), field_list(public_inputs));

    if let Some(slot) = min_context_slot {
        body.insert("minContextSlot".to_owned(), Value::from(slot));
    }

    Ok(Value::Object(body))
}

struct Spent {
    nullifiers: Vec<[u8; FIELD_BYTES]>,
    private_hashes: Vec<[u8; FIELD_BYTES]>,
    address_nullifiers: Vec<[u8; FIELD_BYTES]>,
    first_nullifier: [u8; FIELD_BYTES],
}

struct Created {
    notes: Vec<NotePreimage>,
    hashes: Vec<[u8; FIELD_BYTES]>,
    private_hashes: Vec<[u8; FIELD_BYTES]>,
}

fn check_shape(witness: &TransferWitness) -> Result<(), Error> {
    if witness.inputs.is_empty() || witness.outputs.is_empty() {
        return Err(Error::Limit("witness inputs"));
    }

    if witness.tree_slots.len() != INPUT_TREES {
        return Err(Error::Limit("tree slots"));
    }

    if witness.blinding_seed == [0; FIELD_BYTES] {
        return Err(Error::Attestation("blinding seed"));
    }

    if witness.signer_public_key_hashes.len() != witness.inputs.len() + 1 {
        return Err(Error::Limit("signer hashes"));
    }

    if witness.published_output_owner_public_key_hashes.len() != witness.outputs.len() {
        return Err(Error::Limit("output owners"));
    }

    if witness.cache_read_hashes.len() != witness.inputs.len()
        || witness.cache_is_cached.len() != witness.inputs.len()
        || witness.cache_read_index.len() != witness.inputs.len()
    {
        return Err(Error::Limit("cache reads"));
    }

    Ok(())
}

fn spend_inputs(witness: &TransferWitness) -> Result<Spent, Error> {
    let mut nullifiers = Vec::with_capacity(witness.inputs.len());
    let mut private_hashes = Vec::with_capacity(witness.inputs.len());
    let mut address_nullifiers = Vec::with_capacity(witness.inputs.len());

    for input in &witness.inputs {
        let tree_id = tree_id(witness, input.tree_slot)?;
        let commitment = commit(&input.note, &tree_id, &input.note.blinding)?;
        let spent = nullifier(&commitment, &input.note.blinding, &input.nullifier_secret)?;
        let domain = domain_tag(&input.note.domain);
        private_hashes.push(if domain == UTXO_DOMAIN {
            commitment
        } else {
            [0; FIELD_BYTES]
        });
        address_nullifiers.push(if domain == ADDRESS_DOMAIN {
            spent
        } else {
            [0; FIELD_BYTES]
        });

        nullifiers.push(spent);
    }

    let first_nullifier = nullifiers[0];
    Ok(Spent {
        nullifiers,
        private_hashes,
        address_nullifiers,
        first_nullifier,
    })
}

fn create_outputs(
    witness: &TransferWitness,
    first_nullifier: &[u8; FIELD_BYTES],
) -> Result<Created, Error> {
    let seed = derive_output_blinding_seed(first_nullifier, &witness.blinding_seed)?;
    let mut notes = Vec::with_capacity(witness.outputs.len());
    let mut hashes = Vec::with_capacity(witness.outputs.len());
    let mut private_hashes = Vec::with_capacity(witness.outputs.len());

    for (index, output) in witness.outputs.iter().enumerate() {
        let blinding = derive_transact_output_blinding(first_nullifier, &seed, index as u32)?;
        let mut note = output.note;
        note.blinding = blinding;
        let hash = commit(&note, &witness.output_tree_id, &blinding)?;
        private_hashes.push(if domain_tag(&note.domain) == UTXO_DOMAIN {
            hash
        } else {
            [0; FIELD_BYTES]
        });

        notes.push(note);
        hashes.push(hash);
    }

    Ok(Created {
        notes,
        hashes,
        private_hashes,
    })
}

fn tree_id(witness: &TransferWitness, slot: u64) -> Result<[u8; FIELD_BYTES], Error> {
    witness
        .tree_slots
        .get(slot as usize)
        .map(|slot| slot.id)
        .ok_or(Error::Limit("tree slot"))
}

fn commit(
    note: &NotePreimage,
    tree_id: &[u8; FIELD_BYTES],
    blinding: &[u8; FIELD_BYTES],
) -> Result<[u8; FIELD_BYTES], Error> {
    utxo_commitment(&UtxoPreimage {
        domain: note.domain,
        tree_id: *tree_id,
        owner: note.owner,
        asset: note.asset,
        amount: note.amount,
        blinding: *blinding,
        data_hash: note.data_hash,
        ring_data_hash: note.ring_data_hash,
        ring_program_id: note.ring_program_id,
    })
}

fn private_tx_hash(
    inputs: &[[u8; FIELD_BYTES]],
    outputs: &[[u8; FIELD_BYTES]],
    addresses: &[[u8; FIELD_BYTES]],
    first_nullifier: &[u8; FIELD_BYTES],
    blinding_seed: &[u8; FIELD_BYTES],
) -> Result<[u8; FIELD_BYTES], Error> {
    let blinding = derive_private_tx_blinding(first_nullifier, blinding_seed)?;
    poseidon_hash(&[
        &nonzero_hash_chain(inputs)?,
        &nonzero_hash_chain(outputs)?,
        &nonzero_hash_chain(addresses)?,
        &blinding,
    ])
}

fn public_input_hash(
    witness: &TransferWitness,
    nullifiers: &[[u8; FIELD_BYTES]],
    output_hashes: &[[u8; FIELD_BYTES]],
    private_tx: &[u8; FIELD_BYTES],
) -> Result<[u8; FIELD_BYTES], Error> {
    let slots = slot_hashes(&witness.tree_slots)?;
    let owners = &witness.published_output_owner_public_key_hashes;
    let mut fields = vec![
        right_hash_chain4(nullifiers)?,
        right_hash_chain4(output_hashes)?,
        right_hash_chain(&slots)?,
        witness.output_tree_id,
        *private_tx,
    ];
    fields.extend(after_private_tx(witness)?);
    fields.push(witness.external_data_hash);

    for slot in 0..PUBLIC_SLOTS {
        fields.push(witness.public_assets[slot]);
        fields.push(witness.public_amounts[slot]);
    }

    fields.push(witness.ring_program_id);
    fields.push(right_hash_chain(&witness.signer_public_key_hashes)?);
    fields.push(witness.input_flags);
    fields.push(right_hash_chain4(owners)?);
    fields.push(witness.cache_tree_id);
    fields.push(witness.cache_read_hash_chain);
    hash_chain4(&fields)
}

fn after_private_tx(witness: &TransferWitness) -> Result<Vec<[u8; FIELD_BYTES]>, Error> {
    match &witness.circuit {
        TransferCircuit::Confidential => Ok(Vec::new()),
        TransferCircuit::ConfidentialEddsa(owner) => signed_fields(owner, None),
        TransferCircuit::EddsaRing {
            owner,
            default_owner_public_key_hash,
        } => signed_fields(owner, Some(*default_owner_public_key_hash)),
    }
}

fn signed_fields(
    owner: &SignedOwner,
    default_owner: Option<[u8; FIELD_BYTES]>,
) -> Result<Vec<[u8; FIELD_BYTES]>, Error> {
    let digest = message_digest(&owner.message_hash_low, &owner.message_hash_high);

    let second = match default_owner {
        Some(owner_hash) => owner_hash,
        None => solana_owner_identity(&owner.public_key)?,
    };

    Ok(vec![hash_bytes(&digest)?, second])
}

fn message_digest(low: &[u8; FIELD_BYTES], high: &[u8; FIELD_BYTES]) -> [u8; FIELD_BYTES] {
    let mut digest = [0; FIELD_BYTES];
    digest[..16].copy_from_slice(&high[16..]);
    digest[16..].copy_from_slice(&low[16..]);
    digest
}

fn slot_hashes(slots: &[TreeSlotWitness]) -> Result<Vec<[u8; FIELD_BYTES]>, Error> {
    slots
        .iter()
        .map(|slot| poseidon_hash(&[&slot.id, &slot.utxo_root, &slot.nullifier_root]))
        .collect()
}

fn domain_tag(field: &[u8; FIELD_BYTES]) -> u8 {
    if field[..FIELD_BYTES - 1].iter().all(|byte| *byte == 0) {
        field[FIELD_BYTES - 1]
    } else {
        0
    }
}

fn nonzero_hash_chain(items: &[[u8; FIELD_BYTES]]) -> Result<[u8; FIELD_BYTES], Error> {
    let mut acc = [0; FIELD_BYTES];
    let mut started = false;

    for item in items {
        if *item == [0; FIELD_BYTES] {
            continue;
        }

        if !started {
            acc = *item;
            started = true;
            continue;
        }

        acc = poseidon_hash(&[&acc, item])?;
    }

    Ok(acc)
}

fn hash_chain4(items: &[[u8; FIELD_BYTES]]) -> Result<[u8; FIELD_BYTES], Error> {
    let Some((first, rest)) = items.split_first() else {
        return Ok([0; FIELD_BYTES]);
    };

    if rest.is_empty() {
        return Ok(*first);
    }

    let mut acc = *first;
    let mut start = 0;

    while start < rest.len() {
        let mut group = [acc, [0; FIELD_BYTES], [0; FIELD_BYTES], [0; FIELD_BYTES]];

        for (offset, item) in rest[start..].iter().take(3).enumerate() {
            group[1 + offset] = *item;
        }

        acc = poseidon_hash(&[&group[0], &group[1], &group[2], &group[3]])?;
        start += 3;
    }

    Ok(acc)
}

fn right_hash_chain4(items: &[[u8; FIELD_BYTES]]) -> Result<[u8; FIELD_BYTES], Error> {
    let Some(last) = items.last() else {
        return Ok([0; FIELD_BYTES]);
    };

    if items.len() == 1 {
        return Ok(*last);
    }

    let mut acc = *last;
    let mut end = items.len() - 1;

    while end > 0 {
        let start = end.saturating_sub(3);
        let mut group = [[0; FIELD_BYTES], [0; FIELD_BYTES], [0; FIELD_BYTES], acc];

        for (offset, item) in items[start..end].iter().enumerate() {
            group[offset] = *item;
        }

        acc = poseidon_hash(&[&group[0], &group[1], &group[2], &group[3]])?;
        end = start;
    }

    Ok(acc)
}

fn right_hash_chain(items: &[[u8; FIELD_BYTES]]) -> Result<[u8; FIELD_BYTES], Error> {
    let Some((last, prefix)) = items.split_last() else {
        return Ok([0; FIELD_BYTES]);
    };

    let mut acc = *last;

    for item in prefix.iter().rev() {
        acc = poseidon_hash(&[item, &acc])?;
    }

    Ok(acc)
}

fn witness_json(
    witness: &TransferWitness,
    spent: &Spent,
    created: &Created,
    private_tx: &[u8; FIELD_BYTES],
    public_hash: &[u8; FIELD_BYTES],
) -> Value {
    let mut pairs = vec![
        ("circuitType", Value::from(circuit_name(&witness.circuit))),
        ("nInputs", Value::from(witness.inputs.len())),
        ("nOutputs", Value::from(witness.outputs.len())),
        ("inputs", input_json(witness, spent)),
        ("outputs", output_json(witness, created)),
        ("treeSlots", slot_json(&witness.tree_slots)),
        ("outputTreeId", field_value(&witness.output_tree_id)),
        ("externalDataHash", field_value(&witness.external_data_hash)),
        ("privateTxHash", field_value(private_tx)),
        ("blindingSeed", field_value(&witness.blinding_seed)),
        ("publicAssets", field_list(&witness.public_assets)),
        ("publicAmounts", field_list(&witness.public_amounts)),
        ("ringProgramId", field_value(&witness.ring_program_id)),
        (
            "signerPkHashes",
            field_list(&witness.signer_public_key_hashes),
        ),
        ("inputFlags", field_value(&witness.input_flags)),
        (
            "publishedOutputOwnerPkHashes",
            field_list(&witness.published_output_owner_public_key_hashes),
        ),
        ("cacheTreeId", field_value(&witness.cache_tree_id)),
        (
            "cacheReadHashChain",
            field_value(&witness.cache_read_hash_chain),
        ),
        ("cacheReadHashes", field_list(&witness.cache_read_hashes)),
        ("cacheIsCached", field_list(&witness.cache_is_cached)),
        ("cacheReadIndex", field_list(&witness.cache_read_index)),
        ("publicInputHash", field_value(public_hash)),
    ];
    pairs.extend(signature_pairs(&witness.circuit));
    json_object(&pairs)
}

fn circuit_name(circuit: &TransferCircuit) -> &'static str {
    match circuit {
        TransferCircuit::Confidential => "transfer-confidential",
        TransferCircuit::ConfidentialEddsa(_) => "transfer-confidential-eddsa",
        TransferCircuit::EddsaRing { .. } => "transfer-eddsa-ring",
    }
}

fn signature_pairs(circuit: &TransferCircuit) -> Vec<(&str, Value)> {
    let (owner, default_owner) = match circuit {
        TransferCircuit::Confidential => return Vec::new(),
        TransferCircuit::ConfidentialEddsa(owner) => (owner, None),
        TransferCircuit::EddsaRing {
            owner,
            default_owner_public_key_hash,
        } => (owner, Some(*default_owner_public_key_hash)),
    };

    let mut pairs = vec![
        ("ownerPublicKey", field_value(&owner.public_key)),
        ("ownerSignature", hex_value(&owner.signature)),
        ("messageHashLow", field_value(&owner.message_hash_low)),
        ("messageHashHigh", field_value(&owner.message_hash_high)),
    ];

    if let Some(owner_hash) = default_owner {
        pairs.push(("defaultOwnerPkHash", field_value(&owner_hash)));
    }

    pairs
}

fn input_json(witness: &TransferWitness, spent: &Spent) -> Value {
    let inputs = witness.inputs.iter().enumerate().map(|(index, input)| {
        json_object(&[
            ("utxo", note_json(&input.note)),
            ("isDummy", flag_value(input.is_dummy)),
            ("statePathElements", field_list(&input.state_path)),
            ("statePathIndex", field_value(&input.state_path_index)),
            ("nullifierLowValue", field_value(&input.nullifier_low_value)),
            (
                "nullifierNextValue",
                field_value(&input.nullifier_next_value),
            ),
            (
                "nullifierLowPathElements",
                field_list(&input.nullifier_low_path),
            ),
            (
                "nullifierLowPathIndex",
                field_value(&input.nullifier_low_path_index),
            ),
            ("treeSlot", field_value(&slot_field(input.tree_slot))),
            ("nullifier", field_value(&spent.nullifiers[index])),
            ("ownerPkHash", field_value(&input.owner_public_key_hash)),
            ("nullifierSecret", field_value(&input.nullifier_secret)),
        ])
    });
    Value::Array(inputs.collect())
}

fn output_json(witness: &TransferWitness, created: &Created) -> Value {
    let outputs = witness.outputs.iter().enumerate().map(|(index, output)| {
        json_object(&[
            ("utxo", note_json(&created.notes[index])),
            ("isDummy", flag_value(output.is_dummy)),
            ("hash", field_value(&created.hashes[index])),
            ("ownerPkHash", field_value(&output.owner_public_key_hash)),
            ("nullifierPk", field_value(&output.nullifier_public_key)),
        ])
    });
    Value::Array(outputs.collect())
}

fn slot_json(slots: &[TreeSlotWitness]) -> Value {
    Value::Array(
        slots
            .iter()
            .map(|slot| {
                json_object(&[
                    ("id", field_value(&slot.id)),
                    ("utxoRoot", field_value(&slot.utxo_root)),
                    ("nullifierRoot", field_value(&slot.nullifier_root)),
                ])
            })
            .collect(),
    )
}

fn note_json(note: &NotePreimage) -> Value {
    json_object(&[
        ("domain", field_value(&note.domain)),
        ("owner", field_value(&note.owner)),
        ("asset", field_value(&note.asset)),
        ("amount", field_value(&note.amount)),
        ("blinding", field_value(&note.blinding)),
        ("dataHash", field_value(&note.data_hash)),
        ("ringDataHash", field_value(&note.ring_data_hash)),
        ("ringProgramId", field_value(&note.ring_program_id)),
    ])
}

fn tree_json(trees: &[IndexedTree]) -> Value {
    Value::Array(
        trees
            .iter()
            .map(|tree| {
                json_object(&[
                    ("tree", Value::from(tree.tree.clone())),
                    ("id", Value::from(tree.id)),
                ])
            })
            .collect(),
    )
}

fn lookup_json(lookups: &[IndexedLookup]) -> Value {
    Value::Array(
        lookups
            .iter()
            .map(|lookup| {
                json_object(&[
                    ("treeSlot", Value::from(lookup.tree_slot)),
                    ("commitment", optional_string(lookup.commitment.clone())),
                    ("nullifier", optional_string(lookup.nullifier.clone())),
                ])
            })
            .collect(),
    )
}

fn optional_string(value: Option<String>) -> Value {
    value.map_or(Value::Null, Value::from)
}

fn flag_value(dummy: bool) -> Value {
    field_value(&slot_field(u64::from(dummy)))
}

fn slot_field(slot: u64) -> [u8; FIELD_BYTES] {
    right_align(&slot.to_be_bytes()).unwrap_or([0; FIELD_BYTES])
}

fn field_list(fields: &[[u8; FIELD_BYTES]]) -> Value {
    Value::Array(fields.iter().map(field_value).collect())
}

fn field_value(bytes: &[u8; FIELD_BYTES]) -> Value {
    Value::from(field_hex(bytes))
}

fn field_hex(bytes: &[u8; FIELD_BYTES]) -> String {
    hex_text(bytes)
}

fn hex_value(bytes: &[u8]) -> Value {
    Value::from(hex_text(bytes))
}

fn hex_text(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(2 + bytes.len() * 2);
    text.push_str("0x");

    for byte in bytes {
        text.push(HEX[(*byte >> 4) as usize] as char);
        text.push(HEX[(*byte & 0x0f) as usize] as char);
    }

    text
}

const HEX: &[u8; 16] = b"0123456789abcdef";

fn json_object(pairs: &[(&str, Value)]) -> Value {
    let mut object = Map::new();

    for (key, value) in pairs {
        object.insert((*key).to_owned(), value.clone());
    }

    Value::Object(object)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRANSFER: &str = include_str!("../tests/fixtures/transfer-2x2.json");

    fn parse_field(value: &Value) -> [u8; 32] {
        let text = value.as_str().unwrap().trim_start_matches("0x");
        let mut bytes = [0u8; 32];

        for (index, slot) in bytes.iter_mut().enumerate() {
            let start = index * 2;
            *slot = u8::from_str_radix(&text[start..start + 2], 16).unwrap();
        }

        bytes
    }

    fn note_from(value: &Value) -> NotePreimage {
        NotePreimage {
            domain: parse_field(&value["domain"]),
            owner: parse_field(&value["owner"]),
            asset: parse_field(&value["asset"]),
            amount: parse_field(&value["amount"]),
            blinding: parse_field(&value["blinding"]),
            data_hash: parse_field(&value["dataHash"]),
            ring_data_hash: parse_field(&value["ringDataHash"]),
            ring_program_id: parse_field(&value["ringProgramId"]),
        }
    }

    fn fields(value: &Value) -> Vec<[u8; 32]> {
        value.as_array().unwrap().iter().map(parse_field).collect()
    }

    fn load(json: &str) -> (Value, TransferWitness) {
        let value: Value = serde_json::from_str(json).unwrap();
        let inputs = value["inputs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|input| SpendNote {
                note: note_from(&input["utxo"]),
                tree_slot: field_u64(&input["treeSlot"]),
                is_dummy: field_u64(&input["isDummy"]) == 1,
                owner_public_key_hash: parse_field(&input["ownerPkHash"]),
                nullifier_secret: parse_field(&input["nullifierSecret"]),
                state_path: fields(&input["statePathElements"]),
                state_path_index: parse_field(&input["statePathIndex"]),
                nullifier_low_value: parse_field(&input["nullifierLowValue"]),
                nullifier_next_value: parse_field(&input["nullifierNextValue"]),
                nullifier_low_path: fields(&input["nullifierLowPathElements"]),
                nullifier_low_path_index: parse_field(&input["nullifierLowPathIndex"]),
            });
        let outputs = value["outputs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|output| OutputNote {
                note: note_from(&output["utxo"]),
                is_dummy: field_u64(&output["isDummy"]) == 1,
                owner_public_key_hash: parse_field(&output["ownerPkHash"]),
                nullifier_public_key: parse_field(&output["nullifierPk"]),
            });
        let tree_slots =
            value["treeSlots"]
                .as_array()
                .unwrap()
                .iter()
                .map(|slot| TreeSlotWitness {
                    id: parse_field(&slot["id"]),
                    utxo_root: parse_field(&slot["utxoRoot"]),
                    nullifier_root: parse_field(&slot["nullifierRoot"]),
                });
        let witness = TransferWitness {
            inputs: inputs.collect(),
            outputs: outputs.collect(),
            tree_slots: tree_slots.collect(),
            output_tree_id: parse_field(&value["outputTreeId"]),
            external_data_hash: parse_field(&value["externalDataHash"]),
            blinding_seed: parse_field(&value["blindingSeed"]),
            public_assets: array3(&value["publicAssets"]),
            public_amounts: array3(&value["publicAmounts"]),
            ring_program_id: parse_field(&value["ringProgramId"]),
            signer_public_key_hashes: fields(&value["signerPkHashes"]),
            input_flags: parse_field(&value["inputFlags"]),
            published_output_owner_public_key_hashes: fields(
                &value["publishedOutputOwnerPkHashes"],
            ),
            cache_tree_id: parse_field(&value["cacheTreeId"]),
            cache_read_hash_chain: parse_field(&value["cacheReadHashChain"]),
            cache_read_hashes: fields(&value["cacheReadHashes"]),
            cache_is_cached: fields(&value["cacheIsCached"]),
            cache_read_index: fields(&value["cacheReadIndex"]),
            circuit: circuit_of(&value),
        };
        (value, witness)
    }

    fn array3(value: &Value) -> [[u8; 32]; 3] {
        let parsed = fields(value);
        [parsed[0], parsed[1], parsed[2]]
    }

    fn parse_signature(value: &Value) -> [u8; 64] {
        let text = value.as_str().unwrap().trim_start_matches("0x");
        let mut bytes = [0u8; 64];

        for (index, slot) in bytes.iter_mut().enumerate() {
            let start = index * 2;
            *slot = u8::from_str_radix(&text[start..start + 2], 16).unwrap();
        }

        bytes
    }

    fn owner_of(value: &Value) -> SignedOwner {
        SignedOwner {
            public_key: parse_field(&value["ownerPublicKey"]),
            signature: parse_signature(&value["ownerSignature"]),
            message_hash_low: parse_field(&value["messageHashLow"]),
            message_hash_high: parse_field(&value["messageHashHigh"]),
        }
    }

    fn circuit_of(value: &Value) -> TransferCircuit {
        match value["circuitType"].as_str().unwrap() {
            "transfer-confidential-eddsa" => TransferCircuit::ConfidentialEddsa(owner_of(value)),
            "transfer-eddsa-ring" => TransferCircuit::EddsaRing {
                owner: owner_of(value),
                default_owner_public_key_hash: parse_field(&value["defaultOwnerPkHash"]),
            },
            _ => TransferCircuit::Confidential,
        }
    }

    fn field_u64(value: &Value) -> u64 {
        let bytes = parse_field(value);
        u64::from_be_bytes(bytes[24..].try_into().unwrap())
    }

    #[test]
    fn published_transfer_witness_rebuilds_from_notes_and_paths() {
        let (published, witness) = load(TRANSFER);
        let built = confidential_transfer_witness(&witness).expect("witness");
        assert_eq!(built, published);
    }

    #[test]
    fn debug_output_redacts_secrets_but_keeps_the_shape() {
        let (_, mut witness) = load(TRANSFER);
        witness.inputs[0].nullifier_secret = ascii_field(b"nullifier-secret-dummy");
        witness.inputs[0].note.amount = ascii_field(b"input-amount-dummy");
        witness.inputs[0].note.blinding = ascii_field(b"input-blinding-dummy");
        witness.outputs[0].note.amount = ascii_field(b"output-amount-dummy");
        let secret = witness.inputs[0].nullifier_secret;
        let amount = witness.inputs[0].note.amount;
        let blinding = witness.inputs[0].note.blinding;
        let output_amount = witness.outputs[0].note.amount;

        let rendered = format!("{witness:?}");
        for field in [&secret, &amount, &blinding, &output_amount] {
            assert!(!rendered.contains(format!("{field:?}").as_str()));
            assert!(!rendered.contains(hex_text(field).as_str()));
        }

        assert!(rendered.contains("TransferWitness"));
        assert!(rendered.contains("inputs: 2"));
        assert!(rendered.contains("outputs: 2"));
        assert!(rendered.contains("transfer-confidential"));
        let spend = format!("{:?}", witness.inputs[0]);
        assert!(spend.contains("SpendNote"));
        assert!(!spend.contains(format!("{secret:?}").as_str()));
        assert!(spend.contains("state_path_len"));
        let note = format!("{:?}", witness.inputs[0].note);
        assert!(note.contains("NotePreimage"));
        assert!(!note.contains(format!("{amount:?}").as_str()));
        let output = format!("{:?}", witness.outputs[0]);
        assert!(output.contains("OutputNote"));
        assert!(!output.contains(format!("{output_amount:?}").as_str()));
    }

    /// Distinctive ASCII fields so a leaked rendering is obvious in the diff.
    fn ascii_field(text: &[u8]) -> [u8; 32] {
        let mut field = [0u8; 32];
        field[..text.len()].copy_from_slice(text);
        field
    }

    #[test]
    fn published_signed_fixtures_rebuild() {
        for json in [
            include_str!("../tests/fixtures/confidential-eddsa-2x8.json"),
            include_str!("../tests/fixtures/eddsa-ring-2x2.json"),
        ] {
            let (published, witness) = load(json);
            let built = confidential_transfer_witness(&witness).expect("witness");
            assert_eq!(built, published);
        }
    }

    #[test]
    fn address_and_dummy_slots_leave_the_spend_chain() {
        let (_, mut witness) = load(TRANSFER);
        witness.inputs[0].note.domain = slot_field(ADDRESS_DOMAIN.into());
        witness.outputs[0].note.domain = slot_field(1);
        witness.inputs[1].note.domain[0] = 9;
        assert!(confidential_transfer_witness(&witness).is_ok());
        let one = [7u8; 32];
        assert_eq!(hash_chain4(&[one]).unwrap(), one);
        assert_eq!(right_hash_chain4(&[one]).unwrap(), one);

        assert_eq!(nonzero_hash_chain(&[[0; 32], one]).unwrap(), one);
        witness.tree_slots.pop();
        assert!(matches!(
            confidential_transfer_witness(&witness),
            Err(Error::Limit("tree slots"))
        ));
    }

    #[test]
    fn witness_bounds_reject_an_empty_or_zero_seed() {
        let (_, mut witness) = load(TRANSFER);
        witness.inputs.clear();
        assert!(matches!(
            confidential_transfer_witness(&witness),
            Err(Error::Limit("witness inputs"))
        ));
        let (_, mut witness) = load(TRANSFER);
        witness.blinding_seed = [0; 32];
        assert!(matches!(
            confidential_transfer_witness(&witness),
            Err(Error::Attestation("blinding seed"))
        ));
        witness.blinding_seed = [1; 32];
        witness.signer_public_key_hashes.pop();

        assert!(matches!(
            confidential_transfer_witness(&witness),
            Err(Error::Limit("signer hashes"))
        ));
        let (_, mut witness) = load(TRANSFER);
        witness.published_output_owner_public_key_hashes.pop();
        assert!(matches!(
            confidential_transfer_witness(&witness),
            Err(Error::Limit("output owners"))
        ));
        witness
            .published_output_owner_public_key_hashes
            .push([0; 32]);
        witness.cache_read_hashes.pop();
        assert!(matches!(
            confidential_transfer_witness(&witness),
            Err(Error::Limit("cache reads"))
        ));
        let (_, mut witness) = load(TRANSFER);
        witness.inputs[0].tree_slot = 9;
        assert!(matches!(
            confidential_transfer_witness(&witness),
            Err(Error::Limit("tree slot"))
        ));
    }

    #[test]
    fn indexed_request_wraps_the_prepared_witness() {
        let (_, witness) = load(TRANSFER);
        let prepared = confidential_transfer_witness(&witness).expect("witness");
        let request = indexed_proof_request(
            &prepared,
            &[IndexedTree {
                tree: "tree".to_owned(),
                id: 7,
            }],
            &[IndexedLookup {
                tree_slot: 0,
                commitment: Some("commit".to_owned()),
                nullifier: None,
            }],
            &[[0; 32]],
            Some(9),
        )
        .expect("indexed");
        assert_eq!(request["circuitType"], "transfer-confidential");
        assert_eq!(request["trees"][0]["id"], 7);
        assert_eq!(request["inputs"][0]["commitment"], "commit");
        assert!(request["inputs"][0]["nullifier"].is_null());
        assert_eq!(request["minContextSlot"], 9);

        assert!(indexed_proof_request(&prepared, &[], &[], &[], Some(1 << 53)).is_err());
        assert_eq!(nonzero_hash_chain(&[]).unwrap(), [0; 32]);
        assert_eq!(hash_chain4(&[]).unwrap(), [0; 32]);
        assert_eq!(right_hash_chain4(&[]).unwrap(), [0; 32]);
        assert_eq!(right_hash_chain(&[]).unwrap(), [0; 32]);
    }
}
