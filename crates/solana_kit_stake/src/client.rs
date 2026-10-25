//! Explicit program-aware decoding preserves caller-owned instruction metadata.
use crate::{StakeInstructionData, StakeInstructionDataCodec};
use solana_kit_core::{
    Address, Instruction,
    codecs::{Decoder, Error, Limits},
};

/// Decodes a complete instruction for the expected program, retaining all metadata.
///
/// Unlike the pinned data-only Dart parser, this rejects a different program and
/// trailing bytes. Use the public codec's prefix operation for prefix decoding.
/// # Errors
/// Rejects the wrong program, caller limits, and malformed or trailing data.
pub fn parse_instruction(
    instruction: &Instruction,
    program: Address,
    limits: Limits,
) -> Result<StakeInstructionData, Error> {
    if instruction.program_address != program {
        return Err(Error::InvalidValue);
    }

    if instruction.accounts.len() > limits.max_items {
        return Err(Error::LimitExceeded);
    }

    StakeInstructionDataCodec.decode(
        &instruction.data,
        Limits::new(
            limits.max_bytes,
            limits.max_items - instruction.accounts.len(),
        ),
    )
}

/// Decodes a fetched Stake allocation while preserving every source metadata field.
///
/// The source state is a prefix within its reserved allocation. Reserved bytes
/// follow that prefix; strict standalone codecs continue to reject trailing data.
/// # Errors
/// Rejects a foreign owner, inconsistent allocation or malformed state.
pub fn decode_account(
    account: solana_kit_core::accounts::EncodedAccount,
    limits: Limits,
) -> Result<
    solana_kit_core::accounts::Account<crate::StakeStateAccount>,
    solana_kit_core::accounts::AccountError,
> {
    account.decode(&Allocation, crate::PROGRAM, limits)
}

struct Allocation;

impl Decoder<crate::StakeStateAccount> for Allocation {
    fn read(
        &self,
        reader: &mut solana_kit_core::codecs::Reader<'_>,
    ) -> Result<crate::StakeStateAccount, Error> {
        let value = crate::StakeStateAccountCodec.read(reader)?;
        reader.take(reader.remaining().len())?;
        Ok(value)
    }
}
