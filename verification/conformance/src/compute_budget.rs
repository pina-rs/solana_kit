//! Isolated portable cases awaiting shared registry and runtime integration.
use solana_kit_compute_budget::*;
use solana_kit_core::codecs::{Encoder, Error, Limits};

const LIMITS: Limits = Limits::new(16, 0);

/// Exercises full unsigned prices, fixed program identity and strict trailing rejection.
pub fn full_width_price() -> Result<(), &'static str> {
    let value = SetComputeUnitPrice {
        micro_lamports: u64::MAX,
    };
    let mut instruction = value.instruction(LIMITS).map_err(|_| "price builder")?;

    if instruction.data != [3, 255, 255, 255, 255, 255, 255, 255, 255]
        || instruction.program_address != PROGRAM
        || !instruction.accounts.is_empty()
        || SetComputeUnitPrice::parse(&instruction, LIMITS) != Ok(value)
    {
        return Err("full width price contract");
    }

    instruction.data.push(0);

    if parse_instruction(&instruction, LIMITS).is_ok() {
        return Err("trailing price accepted");
    }

    Ok(())
}

/// Exercises immutable list ordering and a historical deprecated wire layout.
pub fn ordering_and_historical_layout() -> Result<(), &'static str> {
    let historical = RequestUnits {
        units: u32::MAX,
        additional_fee: 0,
    };
    let bytes = RequestUnitsCodec
        .encode(&historical, LIMITS.max_bytes)
        .map_err(|_| "historical wire")?;

    if bytes != [0, 255, 255, 255, 255, 0, 0, 0, 0] {
        return Err("historical bytes");
    }

    let first = SetComputeUnitLimit { units: 1 }
        .instruction(LIMITS)
        .map_err(|_| "limit builder")?;
    let original = [first.clone(), first];
    let updated = update_or_append_compute_unit_limit(&original, |old| old.unwrap_or(0) + 1)
        .map_err(|_| "limit update")?;

    if updated.len() != 2
        || updated[1] != original[1]
        || find_compute_unit_limit(&updated) != Ok(Some((0, 2)))
    {
        return Err("duplicate order");
    }

    let mut malformed = original[0].clone();
    malformed.data[0] = u8::MAX;

    if find_compute_unit_limit(&[malformed]) != Err(Error::InvalidDiscriminator) {
        return Err("unknown limit tag accepted");
    }

    Ok(())
}
