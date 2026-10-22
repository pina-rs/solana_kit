//! Private client-wire experiment; never an SDK or on-chain program dependency.

mod decode;
mod encode;

pub use decode::decode;
pub use encode::encode;

pub const MAX_PACKET: usize = 4096;
const MAX_SIGNERS: u8 = 12;
const MAX_ADDRESSES: usize = 64;
const MAX_INSTRUCTIONS: usize = 64;
const HEAP_BIT: u32 = 16;
const MIN_HEAP: u32 = 32768;
const MAX_HEAP: u32 = 262144;
const HEAP_ALIGNMENT: u32 = 1024;
const KNOWN_MASK: u32 = 31;

/// Specified v1 message fields, with no lookup-table representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub header: [u8; 3],
    pub mask: u32,
    pub lifetime: [u8; 32],
    pub addresses: Vec<[u8; 32]>,
    pub config: Vec<u8>,
    pub instructions: Vec<Instruction>,
}

/// A compiled instruction separates its header from its payload on the wire.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instruction {
    pub program: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
}

/// Explicit malformed, unsupported, or out-of-bounds private experiment input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Invalid;

impl std::fmt::Display for Invalid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unsupported or malformed v1 wire")
    }
}

impl std::error::Error for Invalid {}

impl Message {
    /// Applies SIMD-0385 structure and configured-mask constraints.
    pub fn validate(&self) -> Result<(), Invalid> {
        let [signers, readonly_signers, readonly_unsigned] = self.header;
        let minimum = usize::from(signers) + usize::from(readonly_unsigned);
        let unique: std::collections::BTreeSet<_> = self.addresses.iter().collect();

        if signers > MAX_SIGNERS || readonly_signers >= signers || self.addresses.len() < minimum {
            return Err(Invalid);
        }

        if self.addresses.len() > MAX_ADDRESSES || self.instructions.len() > MAX_INSTRUCTIONS {
            return Err(Invalid);
        }

        if unique.len() != self.addresses.len() || self.config.len() != config_size(self.mask)? {
            return Err(Invalid);
        }

        self.validate_heap()?;

        for instruction in &self.instructions {
            self.validate_instruction(instruction)?;
        }

        Ok(())
    }

    fn validate_heap(&self) -> Result<(), Invalid> {
        if self.mask & HEAP_BIT == 0 {
            return Ok(());
        }

        let bytes: [u8; 4] = self.config[self.config.len() - 4..]
            .try_into()
            .map_err(|_| Invalid)?;
        let heap = u32::from_le_bytes(bytes);

        if !(MIN_HEAP..=MAX_HEAP).contains(&heap) || !heap.is_multiple_of(HEAP_ALIGNMENT) {
            return Err(Invalid);
        }

        Ok(())
    }

    fn validate_instruction(&self, instruction: &Instruction) -> Result<(), Invalid> {
        if instruction.program == 0 || usize::from(instruction.program) >= self.addresses.len() {
            return Err(Invalid);
        }

        if instruction.accounts.len() > usize::from(u8::MAX)
            || instruction.data.len() > usize::from(u16::MAX)
        {
            return Err(Invalid);
        }

        if instruction
            .accounts
            .iter()
            .any(|index| usize::from(*index) >= self.addresses.len())
        {
            return Err(Invalid);
        }

        Ok(())
    }
}

/// Each supported mask bit represents four bytes; the priority pair is atomic.
pub fn config_size(mask: u32) -> Result<usize, Invalid> {
    if mask & !KNOWN_MASK != 0 || matches!(mask & 3, 1 | 2) {
        return Err(Invalid);
    }

    Ok(mask.count_ones() as usize * 4)
}
