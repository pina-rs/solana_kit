//! Source-grounded pre-initialization operations for the pinned mint extensions.
use crate::*;
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Error, Limits},
};

/// Builds source-defined pre-initializeMint operations in caller extension order.
/// The original helper's unmatched extension variants produce no instruction.
pub fn pre_initialize_mint_extensions(
    mint: Address,
    extensions: &[Extension],
    program: Option<Address>,
    limits: Limits,
) -> Result<Vec<Instruction>, Error> {
    if extensions.len() > limits.max_items {
        return Err(Error::LimitExceeded);
    }

    let mut instructions = Vec::new();
    instructions
        .try_reserve_exact(extensions.len())
        .map_err(|_| Error::AllocationFailed)?;

    for extension in extensions {
        if let Some(instruction) = initialize(extension, mint, program.unwrap_or(PROGRAM), limits)?
        {
            instructions.push(instruction);
        }
    }

    Ok(instructions)
}

macro_rules! init {
    ($program:expr,$mint:expr,$limits:expr,$data:expr,$accounts:ident) => {
        $data.instruction_with_program($program, $accounts { mint: $mint }, $limits)?
    };
}
mod authorities;
mod pointers;
mod transfers;

fn initialize(
    extension: &Extension,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Option<Instruction>, Error> {
    for build in [
        pointers::initialize,
        transfers::initialize,
        authorities::initialize,
    ] {
        if let Some(instruction) = build(extension, mint, program, limits)? {
            return Ok(Some(instruction));
        }
    }
    Ok(None)
}
