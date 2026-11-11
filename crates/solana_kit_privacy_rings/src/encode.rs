//! Little-endian layouts that match `wincode` 0.5 `SchemaWrite` for zolana v0.8.0.
//!
//! Sequence lengths use the explicit `FixIntLen` width from the upstream struct,
//! not bincode's variable integer. Enum tags are the variant index unless a
//! `wincode(tag = N)` attribute says otherwise.

use crate::Error;
use crate::address::MAX_CACHE_WRITES;

pub(crate) struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    pub(crate) fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub(crate) fn finish(self) -> Vec<u8> {
        self.bytes
    }

    pub(crate) fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub(crate) fn u16(&mut self, value: u16) {
        self.bytes.extend(value.to_le_bytes());
    }

    pub(crate) fn u64(&mut self, value: u64) {
        self.bytes.extend(value.to_le_bytes());
    }

    pub(crate) fn raw(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub(crate) fn count(&mut self, len: usize) -> Result<(), Error> {
        let len = u8::try_from(len).map_err(|_| Error::Limit("wincode u8 length"))?;
        self.u8(len);
        Ok(())
    }

    pub(crate) fn bytes_u16(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let len = u16::try_from(bytes.len()).map_err(|_| Error::Limit("wincode u16 length"))?;
        self.u16(len);
        self.raw(bytes);
        Ok(())
    }

    pub(crate) fn option_array(&mut self, value: Option<[u8; 32]>) {
        match value {
            None => self.u8(0),
            Some(bytes) => {
                self.u8(1);
                self.raw(&bytes);
            }
        }
    }

    /// Fixed 33-byte option used by `FixedOptionOwnerTag`: absent is a zero tag.
    pub(crate) fn fixed_option_array(&mut self, value: Option<[u8; 32]>) {
        match value {
            None => {
                self.u8(0);
                self.raw(&[0; 32]);
            }
            Some(bytes) => {
                self.u8(1);
                self.raw(&bytes);
            }
        }
    }
}

/// One public settlement leg, in program variant order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferLeg {
    /// Public SOL into the pool.
    SolDeposit(u64),
    /// Public SOL out of the pool.
    SolWithdrawal(u64),
    /// SPL or Token-2022 into the pool.
    SplDeposit { amount: u64, interface_bump: u8 },
    /// SPL or Token-2022 out of the pool.
    SplWithdrawal { amount: u64, interface_bump: u8 },
}

impl TransferLeg {
    pub(crate) fn write(self, writer: &mut Writer) -> Result<(), Error> {
        if self.amount() == 0 {
            return Err(Error::ZeroAmount);
        }
        match self {
            Self::SolDeposit(amount) => {
                writer.u8(0);
                writer.u64(amount);
            }
            Self::SolWithdrawal(amount) => {
                writer.u8(1);
                writer.u64(amount);
            }
            Self::SplDeposit {
                amount,
                interface_bump,
            } => {
                writer.u8(2);
                writer.u64(amount);
                writer.u8(interface_bump);
            }
            Self::SplWithdrawal {
                amount,
                interface_bump,
            } => {
                writer.u8(3);
                writer.u64(amount);
                writer.u8(interface_bump);
            }
        }
        Ok(())
    }

    const fn amount(self) -> u64 {
        match self {
            Self::SolDeposit(amount) | Self::SolWithdrawal(amount) => amount,
            Self::SplDeposit { amount, .. } | Self::SplWithdrawal { amount, .. } => amount,
        }
    }
}

/// How an output owner tag is carried. Variant order matches `OwnerTag`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnerTag {
    /// The 32-byte tag is in the instruction.
    Inline([u8; 32]),
    /// Index into the instruction's account list.
    Account(u8),
}

impl OwnerTag {
    fn write(self, writer: &mut Writer) {
        match self {
            Self::Inline(bytes) => {
                writer.u8(0);
                writer.raw(&bytes);
            }
            Self::Account(index) => {
                writer.u8(1);
                writer.u8(index);
            }
        }
    }
}

/// One output slot in `transact` instruction data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputSlot {
    /// Output commitment.
    pub utxo_hash: [u8; 32],
    /// Owner tag published with the commitment.
    pub owner_tag: OwnerTag,
    /// Ciphertext, or `None` when a previous bundle covers this slot.
    pub data: Option<Vec<u8>>,
}

/// A published ciphertext that is not bound to an output commitment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    /// Indexing tag.
    pub view_tag: [u8; 32],
    /// Ciphertext bytes.
    pub data: Vec<u8>,
}

/// One spent input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Input {
    /// Nullifier hash.
    pub nullifier_hash: [u8; 32],
    /// Index into the instruction's input-tree accounts.
    pub tree_index: u8,
}

/// Root indexes for one input tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeContext {
    /// UTXO tree root history index.
    pub utxo_tree_root_index: u16,
    /// Nullifier tree root history index.
    pub nullifier_tree_root_index: u16,
}

/// Groth16 proof bytes in the program's `a || b || c` order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Proof {
    /// Compressed G1 `a`, 32 bytes.
    pub a: [u8; 32],
    /// Raw big-endian G2 `b`, 128 bytes.
    pub b: [u8; 128],
    /// Compressed G1 `c`, 32 bytes.
    pub c: [u8; 32],
}

impl Proof {
    /// A zero proof. Useful before a prover response replaces it.
    #[must_use]
    pub const fn zeroed() -> Self {
        Self {
            a: [0; 32],
            b: [0; 128],
            c: [0; 32],
        }
    }

    fn write(self, writer: &mut Writer) {
        writer.raw(&self.a);
        writer.raw(&self.b);
        writer.raw(&self.c);
    }
}

/// One cache write destination.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CacheWrite {
    /// Output index.
    pub output: u8,
    /// Cache slot.
    pub slot: u8,
}

impl CacheWrite {
    /// The upstream sentinel for an unused write slot.
    pub const NONE: Self = Self {
        output: u8::MAX,
        slot: u8::MAX,
    };
}

/// Cache reads and writes bound into a cached circuit selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CacheAccess {
    /// Bit `i` is set when cache slot `i` is read. The bit is the slot in the
    /// cache account, not the input's position. Reading slot 5 sets bit 5.
    pub read_bitmap: u64,
    /// Write destinations. Unused entries are [`CacheWrite::NONE`].
    pub write_slots: [CacheWrite; MAX_CACHE_WRITES],
}

impl CacheAccess {
    pub(crate) fn reads(self) -> bool {
        self.read_bitmap != 0
    }

    /// Upstream treats writes as a prefix ended by [`CacheWrite::NONE`].
    pub(crate) fn writes_cache(self) -> bool {
        self.write_slots
            .first()
            .is_some_and(|entry| *entry != CacheWrite::NONE)
    }

    fn write(self, writer: &mut Writer) {
        writer.u64(self.read_bitmap);
        for slot in self.write_slots {
            writer.u8(slot.output);
            writer.u8(slot.slot);
        }
    }
}

/// BSB22 commitment carried by EdDSA circuit selectors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bsb22 {
    /// Commitment bytes.
    pub commitment: [u8; 32],
    /// Proof of knowledge.
    pub commitment_pok: [u8; 32],
}

/// Payload of `AnonymousRingEddsa`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnonymousRingProof {
    /// BSB22 commitment.
    pub bsb22: Bsb22,
    /// Default-ring owner tag, when the spend is not ring-only.
    pub default_owner_tag: Option<[u8; 32]>,
}

/// Payload of `ConfidentialEddsa`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EddsaProof {
    /// BSB22 commitment.
    pub bsb22: Bsb22,
    /// Owner Ed25519 key verified in-circuit.
    pub owner_pk: [u8; 32],
}

/// Circuit selector. Variant order and the `u16` tag match `CircuitId`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Circuit {
    /// Owner-signed confidential transfer or withdrawal.
    ConfidentialSigner(u8, u8, u8),
    /// Confidential ring with an EdDSA co-signer.
    ConfidentialRingEddsa(u8, u8, u8),
    /// Ring authority transaction.
    RingAuthority(u8, u8, u8),
    /// Anonymous ring EdDSA.
    AnonymousRingEddsa(u8, u8, u8, AnonymousRingProof),
    /// [`Self::ConfidentialSigner`] plus a cache selection.
    ConfidentialSignerCached(u8, u8, u8, CacheAccess),
    /// [`Self::ConfidentialRingEddsa`] plus a cache selection.
    ConfidentialRingEddsaCached(u8, u8, u8, CacheAccess),
    /// [`Self::AnonymousRingEddsa`] plus a cache selection.
    AnonymousRingEddsaCached(u8, u8, u8, AnonymousRingProof, CacheAccess),
    /// Confidential EdDSA.
    ConfidentialEddsa(u8, u8, u8, EddsaProof),
    /// [`Self::ConfidentialEddsa`] plus a cache selection.
    ConfidentialEddsaCached(u8, u8, u8, EddsaProof, CacheAccess),
}

impl Circuit {
    /// Tag 16 accepts these selectors. Ring selectors use a different instruction.
    pub(crate) fn fits_transact(self) -> bool {
        matches!(
            self,
            Self::ConfidentialSigner(..)
                | Self::ConfidentialSignerCached(..)
                | Self::ConfidentialEddsa(..)
                | Self::ConfidentialEddsaCached(..)
        )
    }

    pub(crate) fn cache(self) -> Option<CacheAccess> {
        match self {
            Self::ConfidentialSignerCached(.., cache)
            | Self::ConfidentialRingEddsaCached(.., cache)
            | Self::ConfidentialEddsaCached(.., cache)
            | Self::AnonymousRingEddsaCached(.., cache) => Some(cache),
            _ => None,
        }
    }

    fn write(self, writer: &mut Writer) {
        let tag: u16 = match self {
            Self::ConfidentialSigner(..) => 0,
            Self::ConfidentialRingEddsa(..) => 1,
            Self::RingAuthority(..) => 2,
            Self::AnonymousRingEddsa(..) => 3,
            Self::ConfidentialSignerCached(..) => 4,
            Self::ConfidentialRingEddsaCached(..) => 5,
            Self::AnonymousRingEddsaCached(..) => 6,
            Self::ConfidentialEddsa(..) => 7,
            Self::ConfidentialEddsaCached(..) => 8,
        };
        writer.u16(tag);
        match self {
            Self::ConfidentialSigner(inputs, outputs, assets)
            | Self::ConfidentialRingEddsa(inputs, outputs, assets)
            | Self::RingAuthority(inputs, outputs, assets) => {
                writer.u8(inputs);
                writer.u8(outputs);
                writer.u8(assets);
            }
            Self::AnonymousRingEddsa(inputs, outputs, assets, proof) => {
                write_shape(writer, inputs, outputs, assets);
                write_anonymous(writer, proof);
            }
            Self::ConfidentialSignerCached(inputs, outputs, assets, cache)
            | Self::ConfidentialRingEddsaCached(inputs, outputs, assets, cache) => {
                write_shape(writer, inputs, outputs, assets);
                cache.write(writer);
            }
            Self::AnonymousRingEddsaCached(inputs, outputs, assets, proof, cache) => {
                write_shape(writer, inputs, outputs, assets);
                write_anonymous(writer, proof);
                cache.write(writer);
            }
            Self::ConfidentialEddsa(inputs, outputs, assets, proof) => {
                write_shape(writer, inputs, outputs, assets);
                write_eddsa(writer, proof);
            }
            Self::ConfidentialEddsaCached(inputs, outputs, assets, proof, cache) => {
                write_shape(writer, inputs, outputs, assets);
                write_eddsa(writer, proof);
                cache.write(writer);
            }
        }
    }
}

fn write_shape(writer: &mut Writer, inputs: u8, outputs: u8, assets: u8) {
    writer.u8(inputs);
    writer.u8(outputs);
    writer.u8(assets);
}

fn write_anonymous(writer: &mut Writer, proof: AnonymousRingProof) {
    writer.raw(&proof.bsb22.commitment);
    writer.raw(&proof.bsb22.commitment_pok);
    writer.fixed_option_array(proof.default_owner_tag);
}

fn write_eddsa(writer: &mut Writer, proof: EddsaProof) {
    writer.raw(&proof.bsb22.commitment);
    writer.raw(&proof.bsb22.commitment_pok);
    writer.raw(&proof.owner_pk);
}

/// Fields of `TransactIxData` other than the settlement legs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactBody {
    /// Unix expiry checked by the program.
    pub expiry_unix_ts: u64,
    /// Tagged X25519 key, `0x04 || u`, shared by every ciphertext.
    pub tx_viewing_pk: [u8; 33],
    /// 16-byte salt shared by every ciphertext.
    pub salt: [u8; 16],
    /// Optional external data digest.
    pub data_hash: Option<[u8; 32]>,
    /// Optional ring data digest.
    pub ring_data_hash: Option<[u8; 32]>,
    /// Outputs in tree-append order.
    pub outputs: Vec<OutputSlot>,
    /// Ciphertexts that are not output commitments.
    pub messages: Vec<Message>,
    /// Private transaction hash.
    pub private_tx_hash: [u8; 32],
    /// Verifying-key selector.
    pub circuit: Circuit,
    /// Groth16 proof.
    pub proof: Proof,
    /// Spent inputs. Order matches the nullifier accounts.
    pub inputs: Vec<Input>,
    /// One context per input tree, in account order.
    pub tree_contexts: Vec<TreeContext>,
}

pub(crate) fn write_transact(body: &TransactBody, legs: &[TransferLeg]) -> Result<Vec<u8>, Error> {
    let mut writer = Writer::new();
    writer.u64(body.expiry_unix_ts);
    writer.raw(&body.tx_viewing_pk);
    writer.raw(&body.salt);
    writer.count(legs.len())?;
    for leg in legs {
        leg.write(&mut writer)?;
    }
    writer.option_array(body.data_hash);
    writer.option_array(body.ring_data_hash);
    writer.count(body.outputs.len())?;
    for output in &body.outputs {
        writer.raw(&output.utxo_hash);
        output.owner_tag.write(&mut writer);
        match &output.data {
            None => writer.u8(0),
            Some(data) => {
                writer.u8(1);
                writer.bytes_u16(data)?;
            }
        }
    }
    writer.count(body.messages.len())?;
    for message in &body.messages {
        writer.raw(&message.view_tag);
        writer.bytes_u16(&message.data)?;
    }
    writer.raw(&body.private_tx_hash);
    body.circuit.write(&mut writer);
    body.proof.write(&mut writer);
    writer.count(body.inputs.len())?;
    for input in &body.inputs {
        writer.raw(&input.nullifier_hash);
        writer.u8(input.tree_index);
    }
    writer.count(body.tree_contexts.len())?;
    for context in &body.tree_contexts {
        writer.u16(context.utxo_tree_root_index);
        writer.u16(context.nullifier_tree_root_index);
    }
    Ok(writer.finish())
}

/// Confidential plaintext after AES-CTR decryption. Source: `ConfidentialOutputPlaintext`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfidentialPlaintext {
    /// Registry asset id.
    pub asset_id: u64,
    /// Token amount in base units.
    pub amount: u64,
    /// Note blinding.
    pub blinding: [u8; 32],
    /// Ring program, when the note belongs to a custom ring.
    pub ring_program_id: Option<[u8; 32]>,
    /// Remaining application-data bytes, including their length prefix.
    pub records: Vec<u8>,
}

/// Encodes a confidential note in the v0.8.0 `wincode` layout.
#[must_use]
pub fn write_confidential(note: &ConfidentialPlaintext) -> Vec<u8> {
    let mut writer = Writer::new();
    writer.u64(note.asset_id);
    writer.u64(note.amount);
    writer.raw(&note.blinding);
    writer.option_array(note.ring_program_id);
    if note.records.is_empty() {
        writer.u8(0);
    } else {
        writer.raw(&note.records);
    }
    writer.finish()
}

/// Reads a confidential note. Bytes after the ring field stay in `records`.
///
/// # Errors
/// Rejects a buffer shorter than the fixed prefix or a ring tag other than 0 or 1.
pub fn read_confidential(bytes: &[u8]) -> Result<ConfidentialPlaintext, Error> {
    if bytes.len() < 8 + 8 + 32 + 1 {
        return Err(Error::Plaintext);
    }
    let asset_id = u64::from_le_bytes(bytes[0..8].try_into().expect("8 bytes"));
    let amount = u64::from_le_bytes(bytes[8..16].try_into().expect("8 bytes"));
    let mut blinding = [0; 32];
    blinding.copy_from_slice(&bytes[16..48]);
    let (ring_program_id, rest) = match bytes[48] {
        0 => (None, &bytes[49..]),
        1 => {
            if bytes.len() < 49 + 32 {
                return Err(Error::Plaintext);
            }
            let mut ring = [0; 32];
            ring.copy_from_slice(&bytes[49..81]);
            (Some(ring), &bytes[81..])
        }
        _ => return Err(Error::Plaintext),
    };
    Ok(ConfidentialPlaintext {
        asset_id,
        amount,
        blinding,
        ring_program_id,
        records: rest.to_vec(),
    })
}
