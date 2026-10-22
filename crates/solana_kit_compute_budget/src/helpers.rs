//! Instruction-list helpers preserve the pinned message helper ordering contracts.
use crate::{
    PROGRAM, SetComputeUnitLimit, SetComputeUnitPrice, SetComputeUnitPriceCodec,
    identify_instruction,
};
use alloc::vec::Vec;
use solana_kit_core::{
    Instruction,
    codecs::{Decoder, Error, Limits},
};

/// Limit inserted before resource estimation.
pub const PROVISORY_COMPUTE_UNIT_LIMIT: u32 = 0;
/// Documented validator maximum. Wire construction does not clamp values.
pub const MAX_COMPUTE_UNIT_LIMIT: u32 = 1_400_000;
/// Position and exact units of the first matching limit instruction.
pub type ComputeUnitLimitDetails = (usize, u32);
/// Position and full unsigned price of the first matching price instruction.
pub type ComputeUnitPriceDetails = (usize, u64);

const LIMIT_SIZE: usize = 5;
const PRICE_SIZE: usize = 9;

fn find_instruction(
    instructions: &[Instruction],
    tag: u8,
    size: usize,
) -> Result<Option<(usize, &Instruction)>, Error> {
    for (index, instruction) in instructions.iter().enumerate() {
        if instruction.program_address != PROGRAM || instruction.data.len() < size {
            continue;
        }

        if identify_instruction(&instruction.data)? == tag {
            return Ok(Some((index, instruction)));
        }
    }

    Ok(None)
}

/// Finds the first limit. As in Dart, trailing bytes and account metadata are ignored.
///
/// Returns an error for an unknown qualifying Compute Budget tag before the match.
pub fn find_compute_unit_limit(
    instructions: &[Instruction],
) -> Result<Option<ComputeUnitLimitDetails>, Error> {
    let Some((index, instruction)) = find_instruction(instructions, 2, LIMIT_SIZE)? else {
        return Ok(None);
    };
    let bytes = instruction.data[1..5]
        .try_into()
        .map_err(|_| Error::Truncated)?;
    Ok(Some((index, u32::from_le_bytes(bytes))))
}

/// Finds the first price. As in Dart, trailing bytes and account metadata are ignored.
/// Unknown qualifying Compute Budget tags before the match also return an error.
pub fn find_compute_unit_price(
    instructions: &[Instruction],
) -> Result<Option<ComputeUnitPriceDetails>, Error> {
    let Some((index, instruction)) = find_instruction(instructions, 3, PRICE_SIZE)? else {
        return Ok(None);
    };
    let value = SetComputeUnitPriceCodec
        .decode(&instruction.data[..PRICE_SIZE], Limits::new(PRICE_SIZE, 0))?;
    Ok(Some((index, value.micro_lamports)))
}

fn replace_or_append(
    instructions: &[Instruction],
    index: Option<usize>,
    replacement: Instruction,
) -> Result<Vec<Instruction>, Error> {
    let mut output = Vec::new();
    output
        .try_reserve_exact(
            instructions
                .len()
                .checked_add(usize::from(index.is_none()))
                .ok_or(Error::Overflow)?,
        )
        .map_err(|_| Error::AllocationFailed)?;
    output.extend_from_slice(instructions);

    if let Some(index) = index {
        output[index] = replacement;
    } else {
        output.push(replacement);
    }

    Ok(output)
}

/// Updates only the first matching limit, or appends. Later duplicates stay ordered.
pub fn update_or_append_compute_unit_limit(
    instructions: &[Instruction],
    update: impl FnOnce(Option<u32>) -> u32,
) -> Result<Vec<Instruction>, Error> {
    let details = find_compute_unit_limit(instructions)?;
    let units = update(details.map(|(_, units)| units));

    if details.is_some_and(|(_, old)| old == units) {
        return Ok(instructions.to_vec());
    }

    let replacement = SetComputeUnitLimit { units }.instruction(Limits::new(LIMIT_SIZE, 0))?;
    replace_or_append(instructions, details.map(|(index, _)| index), replacement)
}

/// Updates only the first matching price, or appends, preserving full u64 precision.
pub fn update_or_append_compute_unit_price(
    instructions: &[Instruction],
    update: impl FnOnce(Option<u64>) -> u64,
) -> Result<Vec<Instruction>, Error> {
    let details = find_compute_unit_price(instructions)?;
    let micro_lamports = update(details.map(|(_, price)| price));

    if details.is_some_and(|(_, old)| old == micro_lamports) {
        return Ok(instructions.to_vec());
    }

    let replacement =
        SetComputeUnitPrice { micro_lamports }.instruction(Limits::new(PRICE_SIZE, 0))?;
    replace_or_append(instructions, details.map(|(index, _)| index), replacement)
}

/// Appends a price even when earlier prices exist, matching the source helper.
pub fn append_compute_unit_price(
    instructions: &[Instruction],
    micro_lamports: u64,
) -> Result<Vec<Instruction>, Error> {
    let replacement =
        SetComputeUnitPrice { micro_lamports }.instruction(Limits::new(PRICE_SIZE, 0))?;
    replace_or_append(instructions, None, replacement)
}

/// Inserts a zero limit only when no matching limit exists.
pub fn fill_provisory_compute_unit_limit(
    instructions: &[Instruction],
) -> Result<Vec<Instruction>, Error> {
    update_or_append_compute_unit_limit(instructions, |old| {
        old.unwrap_or(PROVISORY_COMPUTE_UNIT_LIMIT)
    })
}
