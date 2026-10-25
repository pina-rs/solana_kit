extern crate alloc;
include!("generated.rs");
const MAX_BYTES: usize = 4096;
const MAX_ITEMS: usize = 64;

fn main() {
    let limits = Limits::new(MAX_BYTES, MAX_ITEMS);
    let value = SetComputeUnitLimit { units: 0x12345678 };
    let instruction = value.instruction(&[], limits).unwrap();
    let expected = [200, 7, 6, 5, 4, 3, 2, 2, 0x78, 0x56, 0x34, 0x12];
    assert_eq!(instruction.data, expected);
    assert_eq!(identify_instruction(&expected).unwrap(), &expected[..8]);
    assert!(matches!(
        parse_instruction(&instruction, limits).unwrap(),
        ComputeBudgetInstructionData::SetComputeUnitLimit(value) if value.units == 0x12345678
    ));

    for length in 0..expected.len() {
        assert!(
            SetComputeUnitLimitCodec
                .decode(&expected[..length], limits)
                .is_err()
        );
    }

    let mut trailing = expected.to_vec();
    trailing.push(0);
    assert!(SetComputeUnitLimitCodec.decode(&trailing, limits).is_err());
    let mut foreign = expected;
    foreign[7] = 99;
    assert!(identify_instruction(&foreign).is_err());
    let program = Address::from_bytes([9; 32]);
    let custom = value
        .instruction_with_program(program, &[], limits)
        .unwrap();

    assert!(parse_instruction(&custom, limits).is_err());
    assert!(parse_instruction_with_program(program, &custom, limits).is_ok());
}
