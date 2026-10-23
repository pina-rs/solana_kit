//! An ordinary portable downstream consumer with no repository flags or patches.

use alloc::{boxed::Box, vec::Vec};
use solana_kit_anchor::{Coder, Idl, Limits, Values};
use solana_kit_core::{AccountMeta, Instruction};

const LIMITS: Limits = Limits::new(4096, 1024, 32);

/// Constructs a Kit instruction from bounded IDL data and caller-owned metadata.
pub fn instruction(
    idl: &Idl,
    accounts: Vec<AccountMeta>,
) -> Result<Instruction, Box<dyn core::error::Error>> {
    let data = Coder::new(idl, LIMITS).encode_instruction("close", &Values::new())?;

    Ok(Instruction {
        program_address: idl.address.parse()?,
        accounts,
        data,
    })
}
