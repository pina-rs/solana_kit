//! Structural identity compares raw bytes and exact opaque parsed JSON spellings.

use std::hash::{Hash, Hasher};

use super::{InstructionTrace, ResolvedInstruction, TracedInstruction};
use crate::{Presence, methods::transaction_results::LoadedAddresses};

impl PartialEq for LoadedAddresses {
    fn eq(&self, other: &Self) -> bool {
        self.writable == other.writable && self.readonly == other.readonly
    }
}

impl Eq for LoadedAddresses {}

impl Hash for LoadedAddresses {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.writable.hash(state);
        self.readonly.hash(state);
    }
}

impl Hash for InstructionTrace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            Self::Outer {
                index,
                stack_height,
            } => {
                index.hash(state);
                hash_depth(stack_height, state);
            }
            Self::Inner {
                outer_index,
                inner_index,
                stack_height,
            } => {
                outer_index.hash(state);
                inner_index.hash(state);
                hash_depth(stack_height, state);
            }
        }
    }
}

fn hash_depth<H: Hasher>(depth: &Presence<Option<u32>>, state: &mut H) {
    std::mem::discriminant(depth).hash(state);

    if let Presence::Present(depth) = depth {
        depth.hash(state);
    }
}

impl PartialEq for ResolvedInstruction<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Raw(left), Self::Raw(right)) => left == right,
            (Self::Parsed(left), Self::Parsed(right)) => {
                left.program == right.program
                    && left.program_id == right.program_id
                    && left.parsed.get() == right.parsed.get()
                    && left.stack_height == right.stack_height
            }
            _ => false,
        }
    }
}

impl Eq for ResolvedInstruction<'_> {}

impl Hash for ResolvedInstruction<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            Self::Raw(instruction) => {
                instruction.program_address.hash(state);
                instruction.data.hash(state);
                hash_accounts(&instruction.accounts, state);
            }
            Self::Parsed(instruction) => {
                instruction.program.hash(state);
                instruction.program_id.hash(state);
                instruction.parsed.get().hash(state);
                hash_depth(&instruction.stack_height, state);
            }
        }
    }
}

impl PartialEq for TracedInstruction<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.trace == other.trace && self.instruction == other.instruction
    }
}

fn hash_accounts<H: Hasher>(accounts: &[solana_kit_core::AccountMeta], state: &mut H) {
    accounts.len().hash(state);

    for account in accounts {
        account.address.hash(state);
        (account.role.is_signer(), account.role.is_writable()).hash(state);
    }
}

impl Eq for TracedInstruction<'_> {}

impl Hash for TracedInstruction<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.trace.hash(state);
        self.instruction.hash(state);
    }
}
