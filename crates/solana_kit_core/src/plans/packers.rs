//! Transactional instruction packers using the exact Kit compiler.
use super::{Error, MaxInstructions, MessageDraft};
use crate::Instruction;
const REALLOC_LIMIT: usize = 10_240;
use alloc::vec::Vec;

/// Repeatable source of independently owned packer state.
pub trait PackerFactory {
    /// Concrete per-traversal cursor.
    type Packer: MessagePacker;
    /// Starts a new traversal without mutating the factory.
    fn create(&self) -> Self::Packer;
}

/// Caller-defined transactional message packing policy.
///
/// An error must leave the cursor unchanged. A successful pack must consume
/// input by appending at least one instruction, even when it becomes done. It must
/// preserve existing instructions and metadata, and pass exact compilation.
/// The planner validates the returned message before committing candidate output.
pub trait MessagePacker {
    /// Whether all input has been consumed.
    fn done(&self) -> bool;
    /// Appends the next fitting work while enforcing the passed instruction budget.
    ///
    /// # Errors
    /// Capacity errors may try another candidate; other failures abort planning.
    fn pack(
        &mut self,
        candidate: &MessageDraft,
        maximum: MaxInstructions,
    ) -> Result<MessageDraft, Error>;
}

/// Replayable greedy list packer, also used for realloc chunks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstructionBatch(pub Vec<Instruction>);

impl InstructionBatch {
    /// Builds realloc instructions in chunks of at most 10,240 bytes.
    #[must_use]
    pub fn realloc(total_size: usize, mut instruction: impl FnMut(usize) -> Instruction) -> Self {
        let mut remaining = total_size;
        let mut instructions = Vec::new();

        while remaining > 0 {
            let size = remaining.min(REALLOC_LIMIT);
            instructions.push(instruction(size));
            remaining -= size;
        }

        Self(instructions)
    }
}
impl PackerFactory for InstructionBatch {
    type Packer = InstructionBatchPacker;
    fn create(&self) -> Self::Packer {
        InstructionBatchPacker {
            instructions: self.0.clone(),
            next: 0,
        }
    }
}

/// Independent cursor over an instruction batch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstructionBatchPacker {
    instructions: Vec<Instruction>,
    next: usize,
}
impl MessagePacker for InstructionBatchPacker {
    fn done(&self) -> bool {
        self.next == self.instructions.len()
    }

    fn pack(
        &mut self,
        candidate: &MessageDraft,
        maximum: MaxInstructions,
    ) -> Result<MessageDraft, Error> {
        if self.done() {
            return Err(Error::PackerComplete);
        }

        let mut output = candidate.clone();
        let mut next = self.next;

        while let Some(instruction) = self.instructions.get(next) {
            let mut proposed = output.clone();
            proposed.instructions.push(instruction.clone());

            let Some(accepted) = fitting_candidate(proposed, maximum, next == self.next)? else {
                break;
            };
            output = accepted;
            next += 1;
        }

        self.next = next;
        Ok(output)
    }
}

/// Replayable writer split by exact compiled capacity, without approximate sizes.
#[derive(Clone, Debug)]
pub struct Linear<F> {
    /// Total bytes to consume.
    pub total_length: usize,
    /// Pure instruction constructor receiving offset and length.
    pub instruction: F,
}
impl<F: Fn(usize, usize) -> Instruction + Clone> PackerFactory for Linear<F> {
    type Packer = LinearPacker<F>;
    fn create(&self) -> Self::Packer {
        LinearPacker {
            source: self.clone(),
            offset: 0,
        }
    }
}

/// Independent cursor for a linear writer.
#[derive(Clone, Debug)]
pub struct LinearPacker<F> {
    source: Linear<F>,
    offset: usize,
}
impl<F: Fn(usize, usize) -> Instruction> MessagePacker for LinearPacker<F> {
    fn done(&self) -> bool {
        self.offset >= self.source.total_length
    }

    fn pack(
        &mut self,
        candidate: &MessageDraft,
        maximum: MaxInstructions,
    ) -> Result<MessageDraft, Error> {
        if self.done() {
            return Err(Error::PackerComplete);
        }

        // Test actual compiled candidates. A caller may have non-linear encoding
        // overhead, so a guessed byte allowance or monotonic binary search is unsafe.
        let remaining = self.source.total_length - self.offset;

        let cap = match candidate.version {
            crate::transaction::Version::V1 => crate::transaction::V1_PACKET_SIZE,
            _ => crate::transaction::PACKET_SIZE,
        };

        for length in (1..=remaining.min(cap)).rev() {
            let mut output = candidate.clone();
            output
                .instructions
                .push((self.source.instruction)(self.offset, length));

            if let Some(accepted) = fitting_candidate(output, maximum, length == 1)? {
                self.offset += length;

                return Ok(accepted);
            }
        }

        Err(Error::NoProgress)
    }
}

fn fitting_candidate(
    draft: MessageDraft,
    maximum: MaxInstructions,
    required: bool,
) -> Result<Option<MessageDraft>, Error> {
    match draft.compile(maximum) {
        Ok(_) => Ok(Some(draft)),

        Err(error) if !required && error.requires_new_candidate() => Ok(None),
        Err(error) => Err(error),
    }
}
