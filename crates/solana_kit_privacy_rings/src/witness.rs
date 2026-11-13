//! Gnark witness for a confidential transfer.
//!
//! The caller supplies opened notes, the Merkle paths that place them, and
//! the public fields the transact instruction already committed to. This
//! module fills the Poseidon nullifiers, output blindings, output
//! commitments, `privateTxHash`, and `publicInputHash`. The JSON object is
//! the body of `POST /prove/transfer_confidential_*`. The same witness also
//! builds the unresolved body of `POST /prove/{circuit}/indexed`, whose
//! paths the indexer fills after resolving the trees.

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
/// Trees one indexed request may name. The prover resolves at most this many.
const INDEXED_TREES: usize = 2;
const UTXO_DOMAIN: u8 = 3;
const ADDRESS_DOMAIN: u8 = 2;
const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;
/// Distinct addresses one transaction can carry.
const MAX_TRANSACTION_ADDRESSES: usize = 64;
/// Addresses every transact needs besides its nullifier PDAs and owner
/// signers: payer, input tree, the shielded-pool program, and the system
/// program.
const FIXED_TRANSACT_ADDRESSES: usize = 4;

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
    /// Index into [`TransferWitness::tree_slots`]. The commitment uses that
    /// id. An indexed request reads the same field as the slot of
    /// [`IndexedLookup`]: it must select a slot whose id the supplied tree
    /// at that position carries.
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
    /// Signer public-key hashes. The pinned circuit fixes the length at the
    /// signer width for the input count: `min(n, 64 - 4 - n) + 1` slots, one
    /// more than the inputs while the transaction can still carry that many
    /// addresses, capped once it cannot.
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
    /// Tree account, base58. It must decode to exactly one 32-byte value.
    pub tree: String,
    /// Tree id the witness's selected tree slot carries. Ids and addresses
    /// must be distinct across the list.
    pub id: u16,
}

/// One inclusion or non-inclusion lookup inside an indexed proof request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedLookup {
    /// Position of this input's tree in the request's tree list. It must
    /// equal the input's own tree slot, whose slot id the tree at this
    /// position carries.
    pub tree_slot: u8,
    /// Commitment, base58, when the input is a state inclusion. It must
    /// spell the commitment the witness derives.
    pub commitment: Option<String>,
    /// Nullifier, base58, when the input is a compact dummy that publishes
    /// zero: the indexer fetches the exclusion witness for this derived
    /// nullifier instead.
    pub nullifier: Option<String>,
}

/// Builds the `transfer-confidential` witness.
///
/// # Errors
/// Rejects an empty spend, a zero blinding seed, a tree slot that is not in
/// `tree_slots`, a rail or shape the pinned proving keys do not serve, a
/// signer vector that is not at the pinned circuit width, or a Poseidon
/// input the field cannot hold.
pub fn confidential_transfer_witness(witness: &TransferWitness) -> Result<Value, Error> {
    let built = build(witness)?;
    Ok(witness_json(
        witness,
        &built.spent,
        &built.created,
        &built.private_tx,
        &built.public_hash,
    ))
}

/// Body of `POST /prove/{circuit}/indexed`.
///
/// Everything derives from the same [`TransferWitness`] the full builder
/// takes. The embedded `prepared` object drops what the indexer owns: the
/// tree slots, the public-input hash, and every input's state and nullifier
/// paths, indices, and exclusion bounds, which the prover inserts after it
/// resolves `trees`. `publicInputs` is the public transcript without the
/// tree-slot chain: the server inserts the resolved tree hash at position 2
/// before folding, so the full proof and the indexed proof commit to the
/// same statement. Inputs need no Merkle roots or paths here. The trees
/// bind the request: each lookup's `tree_slot` is a position in `trees`,
/// that position's `id` must equal the id of the witness slot the input's
/// `tree_slot` selects, and every supplied tree must be named, so the
/// server resolves exactly the trees the commitments name. Commitments and
/// nullifiers in `lookups` stay base58, which is how the indexer spells
/// them.
///
/// # Errors
/// Rejects a witness the full builder rejects, trees or lookups that
/// violate the pinned envelope (tree count, widths, duplicate ids or
/// addresses, a lookup that disagrees with its prepared input, or a tree no
/// input names), or a `min_context_slot` that does not fit in a JSON
/// number.
pub fn indexed_proof_request(
    witness: &TransferWitness,
    trees: &[IndexedTree],
    lookups: &[IndexedLookup],
    min_context_slot: Option<u64>,
) -> Result<Value, Error> {
    if min_context_slot.is_some_and(|slot| slot > MAX_SAFE_INTEGER) {
        return Err(Error::Limit("min context slot"));
    }

    let built = build(witness)?;
    check_envelope(&built, witness, trees, lookups)?;
    let mut public_inputs = built.public_fields.clone();
    // Position 2 folds the five local tree-slot hashes. The server resolves
    // the supplied trees itself and inserts that hash there before folding.
    public_inputs.remove(2);

    let mut body = Map::new();
    body.insert(
        "circuitType".to_owned(),
        Value::from(circuit_name(&witness.circuit)),
    );
    body.insert(
        "prepared".to_owned(),
        prepared_json(witness, &built.spent, &built.created, &built.private_tx),
    );
    body.insert("trees".to_owned(), tree_json(trees));
    body.insert("inputs".to_owned(), lookup_json(lookups));
    body.insert("publicInputs".to_owned(), field_list(&public_inputs));

    if let Some(slot) = min_context_slot {
        body.insert("minContextSlot".to_owned(), Value::from(slot));
    }

    Ok(Value::Object(body))
}

/// One pass over a validated witness: the derived nullifiers, notes, the
/// private transaction hash, and the public transcript fields.
struct Built {
    spent: Spent,
    created: Created,
    private_tx: [u8; FIELD_BYTES],
    /// Public transcript the full proof folds. Position 2 is the tree-slot
    /// chain the indexed prover replaces with its own resolution.
    public_fields: Vec<[u8; FIELD_BYTES]>,
    public_hash: [u8; FIELD_BYTES],
}

fn build(witness: &TransferWitness) -> Result<Built, Error> {
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
    let public_fields =
        public_transcript(witness, &spent.nullifiers, &created.hashes, &private_tx)?;
    let public_hash = hash_chain4(&public_fields)?;
    Ok(Built {
        spent,
        created,
        private_tx,
        public_fields,
        public_hash,
    })
}

struct Spent {
    nullifiers: Vec<[u8; FIELD_BYTES]>,
    commitments: Vec<[u8; FIELD_BYTES]>,
    private_hashes: Vec<[u8; FIELD_BYTES]>,
    address_nullifiers: Vec<[u8; FIELD_BYTES]>,
    first_nullifier: [u8; FIELD_BYTES],
}

struct Created {
    notes: Vec<NotePreimage>,
    hashes: Vec<[u8; FIELD_BYTES]>,
    private_hashes: Vec<[u8; FIELD_BYTES]>,
}

/// Width of the public signer vector the pinned circuits reserve for
/// `n_inputs` inputs: one slot per input while the transaction can still
/// carry that many addresses after the fixed accounts and one nullifier PDA
/// per input, capped, plus the one extra slot beyond those. `None` when no
/// width exists.
fn signer_width(n_inputs: usize) -> Option<usize> {
    let remaining =
        MAX_TRANSACTION_ADDRESSES.checked_sub(FIXED_TRANSACT_ADDRESSES.checked_add(n_inputs)?)?;
    n_inputs.min(remaining).checked_add(1)
}

/// Key-name prefix of the pinned proving keys each rail proves under.
fn rail_prefix(circuit: &TransferCircuit) -> &'static str {
    match circuit {
        TransferCircuit::Confidential => "transfer_confidential",
        TransferCircuit::ConfidentialEddsa(_) => "transfer_confidential_eddsa",
        TransferCircuit::EddsaRing { .. } => "transfer_eddsa_ring",
    }
}

/// Whether the pinned proving keys serve this rail and shape.
///
/// The pinned key names are the one shape source: a witness whose input and
/// output counts name no pinned key cannot be proven anywhere, so the
/// builder refuses it instead of emitting an unservable body.
fn pinned_shape(circuit: &TransferCircuit, inputs: usize, outputs: usize) -> bool {
    let name = format!("{}_{inputs}_{outputs}", rail_prefix(circuit));
    crate::prover::proving_key_sha256(&name).is_some()
}

fn check_shape(witness: &TransferWitness) -> Result<(), Error> {
    // Each check names the field it refuses, and the shape gates come first:
    // a witness whose rail and counts name no pinned proving key, or whose
    // signer vector is not at the circuit width, cannot prove anywhere, so
    // the per-field bounds below never see it.
    if witness.inputs.is_empty() || witness.outputs.is_empty() {
        return Err(Error::Limit("witness inputs"));
    }

    if !pinned_shape(
        &witness.circuit,
        witness.inputs.len(),
        witness.outputs.len(),
    ) {
        return Err(Error::Limit("shape"));
    }

    if witness.tree_slots.len() != INPUT_TREES {
        return Err(Error::Limit("tree slots"));
    }

    if witness.blinding_seed == [0; FIELD_BYTES] {
        return Err(Error::Attestation("blinding seed"));
    }

    let width = signer_width(witness.inputs.len()).ok_or(Error::Limit("signer hashes"))?;
    if witness.signer_public_key_hashes.len() != width {
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
    // Every input derives its own commitment and nullifier, and the private
    // transcript folds the two domains on separate chains: only a utxo
    // commitment rides the private hash and only an address nullifier rides
    // the address chain, so a mixed spend keeps each rail's members in
    // input order and zeroes the rest.
    let mut nullifiers = Vec::with_capacity(witness.inputs.len());
    let mut commitments = Vec::with_capacity(witness.inputs.len());
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
        commitments.push(commitment);
    }

    let first_nullifier = nullifiers[0];
    Ok(Spent {
        nullifiers,
        commitments,
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
    let index = usize::try_from(slot).map_err(|_| Error::Limit("tree slot"))?;
    witness
        .tree_slots
        .get(index)
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

/// The public transcript the full proof folds.
///
/// Position 2 is the tree-slot chain. The indexed prover resolves the
/// supplied trees itself and inserts that hash at the same position, so the
/// indexed request carries this list with position 2 removed.
fn public_transcript(
    witness: &TransferWitness,
    nullifiers: &[[u8; FIELD_BYTES]],
    output_hashes: &[[u8; FIELD_BYTES]],
    private_tx: &[u8; FIELD_BYTES],
) -> Result<Vec<[u8; FIELD_BYTES]>, Error> {
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
    Ok(fields)
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

/// The unresolved prepared object an indexed request embeds.
///
/// It carries the same public fields the full body carries, minus what the
/// indexer owns: no `treeSlots`, no `publicInputHash`, and each input
/// reduced to the six fields the caller supplies. The prover fills every
/// state and nullifier path, index, and exclusion bound after resolving the
/// trees.
fn prepared_json(
    witness: &TransferWitness,
    spent: &Spent,
    created: &Created,
    private_tx: &[u8; FIELD_BYTES],
) -> Value {
    let inputs = witness.inputs.iter().enumerate().map(|(index, input)| {
        json_object(&[
            ("utxo", note_json(&input.note)),
            ("isDummy", flag_value(input.is_dummy)),
            ("treeSlot", field_value(&slot_field(input.tree_slot))),
            ("nullifier", field_value(&spent.nullifiers[index])),
            ("ownerPkHash", field_value(&input.owner_public_key_hash)),
            ("nullifierSecret", field_value(&input.nullifier_secret)),
        ])
    });
    let mut pairs = vec![
        ("circuitType", Value::from(circuit_name(&witness.circuit))),
        ("nInputs", Value::from(witness.inputs.len())),
        ("nOutputs", Value::from(witness.outputs.len())),
        ("inputs", Value::Array(inputs.collect())),
        ("outputs", output_json(witness, created)),
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
    ];
    pairs.extend(signature_pairs(&witness.circuit));
    json_object(&pairs)
}

/// Validates the trees and lookups against the pinned indexed envelope.
///
/// The prover refuses, before any indexer lookup, a body whose trees are
/// not one or two distinct 32-byte base58 addresses with distinct ids,
/// whose lookups do not line up with the prepared inputs one for one, whose
/// per-input slot or commitment or nullifier disagrees with what the
/// prepared input proves, or whose tree no input names. Applying the same
/// rules here fails malformed caller data before transport.
fn check_envelope(
    built: &Built,
    witness: &TransferWitness,
    trees: &[IndexedTree],
    lookups: &[IndexedLookup],
) -> Result<(), Error> {
    check_trees(trees)?;
    if lookups.len() != witness.inputs.len() {
        return Err(Error::Limit("indexed lookups"));
    }

    let mut opened = [false; INDEXED_TREES];
    for (index, (input, lookup)) in witness.inputs.iter().zip(lookups).enumerate() {
        let slot = usize::from(lookup.tree_slot);
        if slot >= trees.len() {
            return Err(Error::Limit("indexed tree slot"));
        }

        check_lookup_tree(
            input,
            lookup,
            &trees[slot],
            &witness.tree_slots[slot].id,
            opened[slot],
        )?;
        check_lookup_flags(
            input,
            lookup,
            cached(&witness.cache_is_cached[index]),
            &built.spent.commitments[index],
            &built.spent.nullifiers[index],
        )?;
        opened[slot] = true;
    }

    if !opened[..trees.len()].iter().all(|named| *named) {
        return Err(Error::Limit("indexed tree"));
    }

    Ok(())
}

/// Validates the supplied trees: one or two distinct entries whose
/// addresses decode to exactly one 32-byte field each.
fn check_trees(trees: &[IndexedTree]) -> Result<(), Error> {
    if trees.is_empty() || trees.len() > INDEXED_TREES {
        return Err(Error::Limit("indexed trees"));
    }

    for (index, tree) in trees.iter().enumerate() {
        if !is_base58_field(&tree.tree) {
            return Err(Error::Limit("indexed tree address"));
        }

        if trees[..index]
            .iter()
            .any(|previous| previous.id == tree.id || previous.tree == tree.tree)
        {
            return Err(Error::Limit("indexed tree"));
        }
    }

    Ok(())
}

/// Binds one lookup to its tree.
///
/// The input's own slot must name the same position, the tree's id must be
/// the id of the witness slot the input selects — the server resolves the
/// tree at this position and the circuit commits to the selected slot's id,
/// so the two must agree — and a dummy may only join a tree an earlier
/// input opened.
fn check_lookup_tree(
    input: &SpendNote,
    lookup: &IndexedLookup,
    tree: &IndexedTree,
    tree_slot_id: &[u8; FIELD_BYTES],
    opened: bool,
) -> Result<(), Error> {
    if input.tree_slot != u64::from(lookup.tree_slot)
        || *tree_slot_id != slot_field(u64::from(tree.id))
    {
        return Err(Error::Limit("indexed tree slot"));
    }

    if input.is_dummy && !opened {
        return Err(Error::Limit("indexed lookup"));
    }

    Ok(())
}

/// Whether a per-input cache flag marks the input as served from the cache.
fn cached(flags: &[u8; FIELD_BYTES]) -> bool {
    *flags != [0; FIELD_BYTES]
}

/// Validates one lookup against what its prepared input proves.
///
/// The input's dummy, cache, and address flags decide whether the lookup
/// carries a commitment, spelled as the derived commitment; only a compact
/// dummy that publishes zero carries a nullifier, spelled as the derived
/// nullifier the indexer fetches the exclusion witness for.
fn check_lookup_flags(
    input: &SpendNote,
    lookup: &IndexedLookup,
    cached: bool,
    derived_commitment: &[u8; FIELD_BYTES],
    derived_nullifier: &[u8; FIELD_BYTES],
) -> Result<(), Error> {
    let address = domain_tag(&input.note.domain) == ADDRESS_DOMAIN;

    if input.is_dummy && cached {
        return Err(Error::Limit("indexed lookup"));
    }

    if (input.is_dummy || cached || address) != lookup.commitment.is_none() {
        return Err(Error::Limit("indexed lookup"));
    }

    if lookup
        .commitment
        .as_deref()
        .is_some_and(|commitment| commitment != base58_field(derived_commitment))
    {
        return Err(Error::Limit("indexed lookup"));
    }

    // Compact padding publishes nullifier zero but proves non-inclusion of
    // its derived dummy nullifier, which only the lookup carries.
    let compact = input.is_dummy && *derived_nullifier == [0; FIELD_BYTES];
    if compact != lookup.nullifier.is_some()
        || lookup
            .nullifier
            .as_deref()
            .is_some_and(|nullifier| nullifier != base58_field(derived_nullifier))
    {
        return Err(Error::Limit("indexed lookup"));
    }

    Ok(())
}

/// Canonical base58 spelling of one 32-byte field, the form the indexer
/// uses for commitments and nullifiers.
fn base58_field(field: &[u8; FIELD_BYTES]) -> String {
    bs58::encode(field).into_string()
}

/// Whether `text` decodes to exactly one 32-byte field, the width every
/// base58 value in the indexed envelope carries.
fn is_base58_field(text: &str) -> bool {
    bs58::decode(text)
        .into_vec()
        .is_ok_and(|decoded| decoded.len() == FIELD_BYTES)
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
        assert_eq!(nonzero_hash_chain(&[]).unwrap(), [0; 32]);
        assert_eq!(hash_chain4(&[]).unwrap(), [0; 32]);
        assert_eq!(right_hash_chain4(&[]).unwrap(), [0; 32]);
        assert_eq!(right_hash_chain(&[]).unwrap(), [0; 32]);
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

    /// Encodes fixture bytes, including intentionally invalid field widths.
    fn base58(field: &[u8]) -> String {
        bs58::encode(field).into_string()
    }

    /// A distinct 32-byte tree address for the position it names.
    fn tree_address(position: usize) -> String {
        let mut address = [9u8; 32];
        address[0] += position as u8;
        base58(&address)
    }

    /// Repeats the first input, and its per-input cache fields, until the
    /// witness names `inputs` inputs.
    fn scale_inputs(witness: &mut TransferWitness, inputs: usize) {
        let input = witness.inputs[0].clone();
        witness.inputs = vec![input; inputs];
        let hashes = witness.cache_read_hashes[0];
        witness.cache_read_hashes = vec![hashes; inputs];
        let cached = witness.cache_is_cached[0];
        witness.cache_is_cached = vec![cached; inputs];
        let index = witness.cache_read_index[0];
        witness.cache_read_index = vec![index; inputs];
    }

    #[test]
    fn signer_width_follows_the_pinned_shape_contract() {
        for (inputs, width) in [
            (1usize, 2usize),
            (2, 3),
            (30, 31),
            (31, 30),
            (32, 29),
            (40, 21),
            (48, 13),
            (49, 12),
        ] {
            assert_eq!(signer_width(inputs), Some(width), "{inputs} inputs");
        }

        assert_eq!(signer_width(60), Some(1));
        assert_eq!(signer_width(61), None);
        assert_eq!(signer_width(usize::MAX), None);
    }

    #[test]
    fn large_pinned_shapes_build_at_the_circuit_signer_width() {
        let (_, mut witness) = load(TRANSFER);

        for inputs in [32usize, 40, 48, 49] {
            scale_inputs(&mut witness, inputs);
            let width = signer_width(inputs).expect("supported shape");
            witness.signer_public_key_hashes = vec![[9; FIELD_BYTES]; width];
            assert!(
                confidential_transfer_witness(&witness).is_ok(),
                "{inputs} inputs at width {width}"
            );
            // The input-count-plus-one rule the builder replaced stops
            // holding one past thirty inputs.
            witness.signer_public_key_hashes = vec![[9; FIELD_BYTES]; inputs + 1];
            assert!(
                matches!(
                    confidential_transfer_witness(&witness),
                    Err(Error::Limit("signer hashes"))
                ),
                "{inputs} inputs at width {}",
                inputs + 1
            );
        }
    }

    #[test]
    fn confidential_eddsa_large_shapes_build_at_the_circuit_width() {
        let json = include_str!("../tests/fixtures/confidential-eddsa-2x8.json");
        let (_, mut witness) = load(json);
        scale_inputs(&mut witness, 48);
        let width = signer_width(48).expect("supported shape");
        witness.signer_public_key_hashes = vec![[1; FIELD_BYTES]; width];
        assert!(confidential_transfer_witness(&witness).is_ok(), "48x8");

        let output = witness.outputs[0].clone();
        witness.outputs = vec![output; 16];
        let owner = witness.published_output_owner_public_key_hashes[0];
        witness.published_output_owner_public_key_hashes = vec![owner; 16];
        assert!(confidential_transfer_witness(&witness).is_ok(), "48x16");

        witness.signer_public_key_hashes = vec![[1; FIELD_BYTES]; 49];
        assert!(matches!(
            confidential_transfer_witness(&witness),
            Err(Error::Limit("signer hashes"))
        ));
    }

    #[test]
    fn unsupported_shapes_name_no_pinned_key() {
        let (_, mut witness) = load(TRANSFER);
        scale_inputs(&mut witness, 33);
        witness.signer_public_key_hashes = vec![[0; FIELD_BYTES]; signer_width(33).expect("width")];
        assert!(matches!(
            confidential_transfer_witness(&witness),
            Err(Error::Limit("shape"))
        ));

        let (_, mut witness) = load(TRANSFER);
        let output = witness.outputs[0].clone();
        witness.outputs.push(output);
        let owner = witness.published_output_owner_public_key_hashes[0];
        witness.published_output_owner_public_key_hashes.push(owner);
        assert!(matches!(
            confidential_transfer_witness(&witness),
            Err(Error::Limit("shape"))
        ));

        let (_, mut witness) = load(TRANSFER);
        scale_inputs(&mut witness, 49);
        witness.signer_public_key_hashes = vec![[0; FIELD_BYTES]; 50];
        assert!(matches!(
            confidential_transfer_witness(&witness),
            Err(Error::Limit("signer hashes"))
        ));
    }

    #[test]
    fn a_tree_slot_beyond_the_word_width_is_rejected() {
        // The smallest slot a 32-bit `usize` cannot carry; truncation on
        // such a target keeps only the low bits.
        const WORD_WIDTH: u64 = 1 << 32;
        let (_, mut witness) = load(TRANSFER);

        for slot in 0..INPUT_TREES as u64 {
            witness.inputs[0].tree_slot = slot;
            assert!(
                confidential_transfer_witness(&witness).is_ok(),
                "slot {slot}"
            );
        }

        for slot in [INPUT_TREES as u64, WORD_WIDTH + 3, u64::MAX] {
            witness.inputs[0].tree_slot = slot;
            assert!(
                matches!(
                    confidential_transfer_witness(&witness),
                    Err(Error::Limit("tree slot"))
                ),
                "slot {slot}"
            );
        }
    }

    /// The indexed public-input prefix the published fixture must produce,
    /// assembled from the fixture's own published fields in transcript
    /// order and without the tree-slot chain at position 2.
    fn published_indexed_public_inputs(published: &Value) -> Vec<[u8; 32]> {
        let nullifiers: Vec<[u8; 32]> = published["inputs"]
            .as_array()
            .expect("inputs")
            .iter()
            .map(|input| parse_field(&input["nullifier"]))
            .collect();
        let hashes: Vec<[u8; 32]> = published["outputs"]
            .as_array()
            .expect("outputs")
            .iter()
            .map(|output| parse_field(&output["hash"]))
            .collect();
        let mut transcript = vec![
            right_hash_chain4(&nullifiers).expect("nullifiers"),
            right_hash_chain4(&hashes).expect("outputs"),
            parse_field(&published["outputTreeId"]),
            parse_field(&published["privateTxHash"]),
        ];

        // The signed rails add the message digest and the owner field right
        // after the private transaction hash, exactly where the full
        // transcript folds them.
        match published["circuitType"].as_str().expect("circuit") {
            "transfer-confidential-eddsa" => {
                let owner = owner_of(published);
                let digest = message_digest(&owner.message_hash_low, &owner.message_hash_high);
                transcript.push(hash_bytes(&digest).expect("digest"));
                transcript.push(solana_owner_identity(&owner.public_key).expect("identity"));
            }
            "transfer-eddsa-ring" => {
                let owner = owner_of(published);
                let digest = message_digest(&owner.message_hash_low, &owner.message_hash_high);
                transcript.push(hash_bytes(&digest).expect("digest"));
                transcript.push(parse_field(&published["defaultOwnerPkHash"]));
            }
            _ => {}
        }

        transcript.push(parse_field(&published["externalDataHash"]));

        for slot in 0..PUBLIC_SLOTS {
            transcript.push(parse_field(&published["publicAssets"][slot]));
            transcript.push(parse_field(&published["publicAmounts"][slot]));
        }

        transcript.push(parse_field(&published["ringProgramId"]));
        transcript.push(right_hash_chain(&fields(&published["signerPkHashes"])).expect("signers"));
        transcript.push(parse_field(&published["inputFlags"]));
        transcript.push(
            right_hash_chain4(&fields(&published["publishedOutputOwnerPkHashes"])).expect("owners"),
        );
        transcript.push(parse_field(&published["cacheTreeId"]));
        transcript.push(parse_field(&published["cacheReadHashChain"]));
        transcript
    }

    /// Trees and lookups one valid indexed request for `witness`: one tree
    /// entry per distinct named slot, in first-use order, carrying the
    /// commitments the witness derives.
    fn fixture_trees_and_lookups(
        witness: &TransferWitness,
    ) -> (Vec<IndexedTree>, Vec<IndexedLookup>) {
        let mut trees: Vec<IndexedTree> = Vec::new();
        let mut lookups = Vec::new();

        for input in &witness.inputs {
            let slot = usize::try_from(input.tree_slot).expect("slot");
            let raw = u64::from_be_bytes(witness.tree_slots[slot].id[24..].try_into().expect("id"));
            let id = u16::try_from(raw).expect("tree id");

            let position = match trees.iter().position(|tree| tree.id == id) {
                Some(position) => position,
                None => {
                    trees.push(IndexedTree {
                        tree: tree_address(trees.len()),
                        id,
                    });
                    trees.len() - 1
                }
            };
            let tree = tree_id(witness, input.tree_slot).expect("tree slot");
            let commitment = commit(&input.note, &tree, &input.note.blinding).expect("commitment");
            lookups.push(IndexedLookup {
                tree_slot: u8::try_from(position).expect("position"),
                commitment: Some(base58(&commitment)),
                nullifier: None,
            });
        }

        (trees, lookups)
    }

    #[test]
    fn indexed_request_derives_the_unresolved_envelope() {
        let (published, witness) = load(TRANSFER);
        let (trees, lookups) = fixture_trees_and_lookups(&witness);
        let request = indexed_proof_request(&witness, &trees, &lookups, Some(9)).expect("indexed");

        let keys: Vec<&str> = request
            .as_object()
            .expect("object")
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            keys,
            [
                "circuitType",
                "inputs",
                "minContextSlot",
                "prepared",
                "publicInputs",
                "trees"
            ]
        );
        assert_eq!(request["circuitType"], "transfer-confidential");
        assert_eq!(request["trees"][0]["id"], 7);
        assert_eq!(
            request["inputs"][0]["commitment"].as_str(),
            lookups[0].commitment.as_deref()
        );
        assert!(request["inputs"][0]["nullifier"].is_null());
        assert_eq!(request["minContextSlot"], 9);

        // The prepared object omits everything the indexer owns.
        let prepared = &request["prepared"];
        assert!(prepared.get("treeSlots").is_none());
        assert!(prepared.get("publicInputHash").is_none());

        for input in prepared["inputs"].as_array().expect("prepared inputs") {
            let keys: Vec<&str> = input
                .as_object()
                .expect("input")
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(
                keys,
                [
                    "isDummy",
                    "nullifier",
                    "nullifierSecret",
                    "ownerPkHash",
                    "treeSlot",
                    "utxo"
                ]
            );
        }

        assert_eq!(prepared["blindingSeed"], published["blindingSeed"]);
        assert_eq!(prepared["privateTxHash"], published["privateTxHash"]);
        assert_eq!(prepared["signerPkHashes"], published["signerPkHashes"]);
        assert_eq!(prepared["cacheIsCached"], published["cacheIsCached"]);
        assert_eq!(prepared["nInputs"], published["nInputs"]);

        // The indexed public inputs are the full transcript without the
        // tree-slot chain, and inserting the resolved chain at position 2
        // reconstitutes the published hash.
        let expected = published_indexed_public_inputs(&published);
        assert_eq!(expected.len(), CONFIDENTIAL_PREFIX);
        assert_eq!(field_list(&expected), request["publicInputs"]);
        let chain =
            right_hash_chain(&slot_hashes(&witness.tree_slots).expect("slots")).expect("chain");
        let mut resolved = expected;
        resolved.insert(2, chain);
        assert_eq!(
            hash_chain4(&resolved).expect("hash"),
            parse_field(&published["publicInputHash"])
        );

        let unsigned = indexed_proof_request(&witness, &trees, &lookups, None).expect("unsigned");
        assert!(unsigned.get("minContextSlot").is_none());
    }

    /// Public-input count of the confidential transcript prefix without the
    /// tree-slot chain.
    const CONFIDENTIAL_PREFIX: usize = 17;
    /// The signed rails add the digest and the owner field.
    const SIGNED_PREFIX: usize = 19;

    /// Runs one expected indexed refusal and names the case that failed.
    fn refused(
        witness: &TransferWitness,
        trees: &[IndexedTree],
        lookups: &[IndexedLookup],
        label: &'static str,
    ) {
        let error = indexed_proof_request(witness, trees, lookups, Some(9)).expect_err(label);
        assert!(matches!(error, Error::Limit(_)), "{label}: {error:?}");
    }

    /// Rewrites every lookup so each carries `commitment` and `nullifier`.
    fn rewritten_lookups(
        lookups: &[IndexedLookup],
        commitment: Option<String>,
        nullifier: Option<String>,
    ) -> Vec<IndexedLookup> {
        lookups
            .iter()
            .map(|lookup| IndexedLookup {
                commitment: commitment.clone(),
                nullifier: nullifier.clone(),
                ..lookup.clone()
            })
            .collect()
    }

    #[test]
    fn indexed_public_inputs_follow_the_signed_rails() {
        for json in [
            include_str!("../tests/fixtures/confidential-eddsa-2x8.json"),
            include_str!("../tests/fixtures/eddsa-ring-2x2.json"),
        ] {
            let (published, witness) = load(json);
            let (trees, lookups) = fixture_trees_and_lookups(&witness);
            let request = indexed_proof_request(&witness, &trees, &lookups, None).expect("indexed");
            let expected = published_indexed_public_inputs(&published);
            let circuit = request["circuitType"].as_str().expect("circuit");

            let width = match circuit {
                "transfer-confidential" => CONFIDENTIAL_PREFIX,
                _ => SIGNED_PREFIX,
            };
            assert_eq!(expected.len(), width, "{circuit}");
            assert_eq!(field_list(&expected), request["publicInputs"]);

            let chain =
                right_hash_chain(&slot_hashes(&witness.tree_slots).expect("slots")).expect("chain");
            let mut resolved = expected;
            resolved.insert(2, chain);
            assert_eq!(
                hash_chain4(&resolved).expect("hash"),
                parse_field(&published["publicInputHash"]),
                "{circuit}"
            );
        }
    }

    #[test]
    fn indexed_request_rejects_malformed_trees() {
        let (_, witness) = load(TRANSFER);
        let (trees, lookups) = fixture_trees_and_lookups(&witness);
        let tree = &trees[0];

        refused(&witness, &[], &lookups, "no trees");
        refused(
            &witness,
            &[
                tree.clone(),
                IndexedTree {
                    tree: tree_address(1),
                    id: 1,
                },
                IndexedTree {
                    tree: tree_address(2),
                    id: 2,
                },
            ],
            &lookups,
            "three trees",
        );

        let duplicate_id = [
            tree.clone(),
            IndexedTree {
                tree: tree_address(1),
                id: tree.id,
            },
        ];
        refused(&witness, &duplicate_id, &lookups, "duplicate tree id");

        let duplicate_address = [
            tree.clone(),
            IndexedTree {
                tree: tree.tree.clone(),
                id: 3,
            },
        ];
        refused(
            &witness,
            &duplicate_address,
            &lookups,
            "duplicate tree address",
        );

        let unbound = [IndexedTree {
            tree: tree.tree.clone(),
            id: 99,
        }];
        refused(&witness, &unbound, &lookups, "unbound tree id");

        let not_base58 = [IndexedTree {
            tree: "0x07".to_owned(),
            id: tree.id,
        }];
        refused(&witness, &not_base58, &lookups, "address not base58");

        let narrow = [4u8; FIELD_BYTES / 2];
        let too_narrow = [IndexedTree {
            tree: base58(&narrow),
            id: tree.id,
        }];
        refused(&witness, &too_narrow, &lookups, "address too narrow");

        // The request binds each tree to the id of the witness slot its
        // inputs select; a tree the inputs never name cannot resolve.
        let unused = [
            tree.clone(),
            IndexedTree {
                tree: tree_address(1),
                id: 3,
            },
        ];
        refused(&witness, &unused, &lookups, "tree no input names");
    }

    #[test]
    fn indexed_request_rejects_malformed_lookups() {
        let (_, witness) = load(TRANSFER);
        let (trees, lookups) = fixture_trees_and_lookups(&witness);

        refused(&witness, &trees, &[], "no lookups");
        refused(&witness, &trees, &[lookups[0].clone()], "lookup count");
        refused(
            &witness,
            &trees,
            &[IndexedLookup {
                tree_slot: 1,
                commitment: lookups[0].commitment.clone(),
                nullifier: None,
            }],
            "slot beyond trees",
        );
        refused(
            &witness,
            &trees,
            &rewritten_lookups(&lookups, None, None),
            "missing commitment",
        );

        let foreign = base58(&[1; FIELD_BYTES]);
        refused(
            &witness,
            &trees,
            &rewritten_lookups(&lookups, Some(foreign), None),
            "wrong commitment",
        );
        let stray = base58(&[2; FIELD_BYTES]);
        refused(
            &witness,
            &trees,
            &rewritten_lookups(&lookups, lookups[0].commitment.clone(), Some(stray)),
            "unexpected nullifier",
        );
    }

    #[test]
    fn indexed_request_rejects_unproven_lookup_flags() {
        let (_, witness) = load(TRANSFER);
        let (trees, lookups) = fixture_trees_and_lookups(&witness);
        let without = rewritten_lookups(&lookups, None, None);

        // A dummy joins only a tree an earlier input opened, and neither a
        // dummy nor a cached input carries a commitment.
        let mut dummy = witness.clone();
        dummy.inputs[1].is_dummy = true;
        let dummy_lookups = Vec::from([lookups[0].clone(), without[1].clone()]);
        assert!(indexed_proof_request(&dummy, &trees, &dummy_lookups, None).is_ok());
        refused(&dummy, &trees, &lookups, "commitment on dummy");

        let mut first_dummy = witness.clone();
        first_dummy.inputs[0].is_dummy = true;
        let opened_late = Vec::from([without[0].clone(), lookups[1].clone()]);
        refused(
            &first_dummy,
            &trees,
            &opened_late,
            "tree starts with a dummy",
        );

        let mut cached = witness.clone();
        cached.cache_is_cached[0] = slot_field(1);
        let cached_lookups = Vec::from([without[0].clone(), lookups[1].clone()]);
        assert!(indexed_proof_request(&cached, &trees, &cached_lookups, None).is_ok());
        refused(&cached, &trees, &lookups, "commitment on cached input");

        let mut cached_dummy = dummy;
        cached_dummy.cache_is_cached[1] = slot_field(1);
        refused(&cached_dummy, &trees, &dummy_lookups, "cached dummy");

        assert!(matches!(
            indexed_proof_request(&witness, &trees, &lookups, Some(MAX_SAFE_INTEGER + 1)),
            Err(Error::Limit("min context slot"))
        ));
    }
}
