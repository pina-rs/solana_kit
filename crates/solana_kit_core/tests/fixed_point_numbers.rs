#![cfg(feature = "fixed-points")]

use solana_kit_core::fixed_points::{BigInt, Error, Shape, Signedness};

const MAX_WIDTH: u32 = 4096;
const DECIMAL_WIDE_SCALE: u32 = 400;
const BINARY_WIDE_SCALE: u32 = 2048;
const SUBNORMAL_BINARY_SCALE: u32 = 1074;
const MAX_FINITE_SHIFT: u32 = 971;
const F64_SIGNIFICAND_BITS: u32 = 53;

fn number<const RADIX: u8>(raw: BigInt, scale: u32, signedness: Signedness) -> Result<f64, Error> {
    Shape::<RADIX>::new(signedness, MAX_WIDTH, scale)
        .unwrap()
        .raw(raw)
        .unwrap()
        .to_number()
}

fn signed<const RADIX: u8>(raw: BigInt, scale: u32) -> Result<f64, Error> {
    number::<RADIX>(raw, scale, Signedness::Signed)
}

#[test]
fn nonfinite_results_are_explicit_overflow() {
    let beyond_finite = BigInt::from(1) << 1024u32;

    for raw in [beyond_finite.clone(), -beyond_finite] {
        assert_eq!(signed::<2>(raw.clone(), 0), Err(Error::Overflow));
        assert_eq!(signed::<10>(raw, 0), Err(Error::Overflow));
    }
}

#[test]
fn finite_ratios_survive_individually_overflowing_components() {
    let decimal = BigInt::from(10).pow(DECIMAL_WIDE_SCALE);
    let binary = BigInt::from(1) << BINARY_WIDE_SCALE;

    for sign in [-1, 1] {
        let expected = f64::from(sign);
        assert_eq!(
            signed::<10>(&decimal * sign, DECIMAL_WIDE_SCALE),
            Ok(expected)
        );
        assert_eq!(signed::<2>(&binary * sign, BINARY_WIDE_SCALE), Ok(expected));
        assert_eq!(
            signed::<2>(&binary * sign, BINARY_WIDE_SCALE + 1),
            Ok(expected / 2.0)
        );
    }
}

#[test]
fn finite_underflow_preserves_zero_and_sign() {
    for signedness in [Signedness::Signed, Signedness::Unsigned] {
        assert_eq!(number::<2>(BigInt::from(1), MAX_WIDTH, signedness), Ok(0.0));
        assert_eq!(
            number::<10>(BigInt::from(1), MAX_WIDTH, signedness),
            Ok(0.0)
        );
        assert_eq!(number::<2>(BigInt::from(0), MAX_WIDTH, signedness), Ok(0.0));
        assert_eq!(
            number::<10>(BigInt::from(0), MAX_WIDTH, signedness),
            Ok(0.0)
        );
    }

    assert_eq!(
        signed::<2>(BigInt::from(-1), MAX_WIDTH).unwrap().to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        signed::<10>(BigInt::from(-1), MAX_WIDTH).unwrap().to_bits(),
        (-0.0f64).to_bits()
    );
}

#[test]
fn finite_extremes_and_subnormal_values_remain_representable() {
    let max: BigInt = ((BigInt::from(1) << F64_SIGNIFICAND_BITS) - 1) << MAX_FINITE_SHIFT;

    assert_eq!(signed::<2>(max.clone(), 0), Ok(f64::MAX));
    assert_eq!(signed::<10>(-max, 0), Ok(-f64::MAX));
    assert_eq!(
        signed::<2>(BigInt::from(1), SUBNORMAL_BINARY_SCALE),
        Ok(f64::from_bits(1))
    );
    assert_eq!(
        signed::<2>(BigInt::from(1), SUBNORMAL_BINARY_SCALE + 1),
        Ok(0.0)
    );
    assert_eq!(signed::<10>(BigInt::from(1), 309), Ok(1e-309));
    assert_eq!(signed::<10>(BigInt::from(1), 323), Ok(1e-323));
    assert_eq!(signed::<10>(BigInt::from(1), 324), Ok(0.0));
}

#[test]
fn ordinary_lossy_presentation_values_remain_finite() {
    for sign in [-1, 1] {
        assert_eq!(
            signed::<10>(BigInt::from(sign * 125), 2),
            Ok(f64::from(sign) * 1.25)
        );
        assert_eq!(
            signed::<2>(BigInt::from(sign * 3), 2),
            Ok(f64::from(sign) * 0.75)
        );
    }
}
