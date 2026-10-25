//! Exact scaling precedes the explicitly lossy presentation conversion.

use alloc::string::ToString;
use solana_kit_core::fixed_points::{BigInt, BinaryShape, DecimalShape, Error, Signedness};

const WIDE_BITS: u32 = 4096;
const DECIMAL_SCALE: u32 = 1000;
const BINARY_SCALE: u32 = 2048;
const SMALL_BITS: u32 = 16;

pub fn extreme_fixed_point_conversion() {
    let decimal = DecimalShape::new(Signedness::Signed, WIDE_BITS, DECIMAL_SCALE).unwrap();
    let one = decimal.raw(BigInt::from(10u8).pow(DECIMAL_SCALE)).unwrap();
    assert_eq!(one.to_number(), Ok(1.0));
    assert_eq!(one.to_string(), "1");

    let binary = BinaryShape::new(Signedness::Signed, WIDE_BITS, BINARY_SCALE).unwrap();
    let one_and_half =
        (BigInt::from(1u8) << BINARY_SCALE) + (BigInt::from(1u8) << (BINARY_SCALE - 1));
    let value = binary.raw(one_and_half).unwrap();
    assert_eq!(value.to_number(), Ok(1.5));
    assert_eq!(value.to_string(), "1.5");

    let small = DecimalShape::new(Signedness::Signed, SMALL_BITS, WIDE_BITS)
        .unwrap()
        .raw(-1)
        .unwrap();
    assert_eq!(small.to_number().unwrap().to_bits(), (-0.0f64).to_bits());
    let huge = DecimalShape::new(Signedness::Signed, WIDE_BITS, 0)
        .unwrap()
        .raw(BigInt::from(1u8) << BINARY_SCALE)
        .unwrap();
    assert_eq!(huge.to_number(), Err(Error::Overflow));
}
