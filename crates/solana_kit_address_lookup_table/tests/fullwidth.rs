use solana_kit_address_lookup_table::*;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Limits},
    program_error::{InstructionFailure, ProgramError},
};

const LIMITS: Limits = Limits::new(10_000, 1000);
const AUTHORITY: Address = Address::from_bytes([7; 32]);

#[cfg_attr(test, test)]
pub fn fullwidth_create_and_nonzero_authority_are_literal() {
    let value = InstructionData::Create {
        recent_slot: u64::MAX,
        bump: 255,
    };
    let literal = [0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    assert_eq!(
        InstructionDataCodec
            .encode(&value, LIMITS.max_bytes)
            .unwrap(),
        literal
    );
    assert_eq!(
        InstructionDataCodec.decode(&literal, LIMITS).unwrap(),
        value
    );
    let value = AddressLookupTable {
        deactivation_slot: u64::MAX,
        last_extended_slot: u64::MAX,
        last_extended_slot_start_index: 255,
        authority: Some(AUTHORITY),
        padding: 0x1234,
        addresses: vec![AUTHORITY],
    };
    let mut literal = vec![1, 0, 0, 0];
    literal.extend([255; 16]);
    literal.extend([255, 1]);
    literal.extend([7; 32]);
    literal.extend([0x34, 0x12]);
    literal.extend([7; 32]);
    assert_eq!(
        AddressLookupTableCodec
            .encode(&value, LIMITS.max_bytes)
            .unwrap(),
        literal
    );
    assert_eq!(
        AddressLookupTableCodec.decode(&literal, LIMITS).unwrap(),
        value
    );
}

#[cfg_attr(test, test)]
pub fn upstream_error_codes_require_actual_program_and_instruction_index() {
    let codes = [0, 1, 2, 10, 11, 3, u32::MAX];
    let foreign = Address::from_bytes([8; 32]);

    for code in codes {
        let classified = AddressLookupTableError::from_code(code);
        assert_eq!(classified.code(), code);
        let mut failure = InstructionFailure {
            index: 1,
            error: ProgramError::Custom(code),
        };
        assert_eq!(
            AddressLookupTableError::from_failure(&failure, &[foreign, PROGRAM], PROGRAM),
            Some(classified)
        );
        assert_eq!(
            AddressLookupTableError::from_failure(&failure, &[PROGRAM, foreign], PROGRAM),
            None
        );
        failure.index = 2;
        assert_eq!(
            AddressLookupTableError::from_failure(&failure, &[PROGRAM, PROGRAM], PROGRAM),
            None
        );
    }

    assert_eq!(
        AddressLookupTableError::from_code(10),
        AddressLookupTableError::ReadonlyDataModified
    );
    assert_eq!(
        AddressLookupTableError::from_code(11),
        AddressLookupTableError::ReadonlyLamportsChanged
    );
    assert_eq!(
        AddressLookupTableError::from_code(u32::MAX),
        AddressLookupTableError::Unknown(u32::MAX)
    );
}
