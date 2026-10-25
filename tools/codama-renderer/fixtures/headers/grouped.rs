extern crate alloc;
include!("generated.rs");
const MAX_BYTES: usize = 4096;
const MAX_ITEMS: usize = 64;
const FIRST_UNITS: u32 = 123;
const LAST_UNITS: u32 = 456;

fn main() {
    let limits = Limits::new(MAX_BYTES, MAX_ITEMS);
    let first = Variant0 { units: FIRST_UNITS }
        .instruction(&[], limits)
        .unwrap();
    let last = Variant255 { units: LAST_UNITS }
        .instruction(&[], limits)
        .unwrap();
    assert_eq!(first.data, [199, 0, 123, 0, 0, 0]);
    assert_eq!(last.data, [199, 255, 200, 1, 0, 0]);
    assert!(matches!(
        parse_instruction(&first, limits).unwrap(),
        ComputeBudgetInstructionData::Variant0(value) if value.units == FIRST_UNITS
    ));
    assert!(matches!(
        parse_instruction(&last, limits).unwrap(),
        ComputeBudgetInstructionData::Variant255(value) if value.units == LAST_UNITS
    ));
    let unknown = [198, 33, 0, 0, 0, 0];

    assert!(
        ComputeBudgetInstructionDataCodec
            .decode(&unknown, limits)
            .is_err()
    );
}
