#![cfg(feature = "fixed-points")]

use solana_kit_core::fixed_points::{BigInt, Error, Shape, Signedness};

const MAX_WIDTH: u32 = 4096;
const DECIMAL_WIDE_SCALE: u32 = 400;
const BINARY_WIDE_SCALE: u32 = 2048;
const SUBNORMAL_BINARY_SCALE: u32 = 1074;
const MAX_FINITE_SHIFT: u32 = 971;
const F64_SIGNIFICAND_BITS: u32 = 53;

const DECIMAL_ROUNDING_SCALE: u32 = 25;
const DECIMAL_UNDERFLOW_SCALE: u32 = 309;
const BINARY_SUBNORMAL_SCALE: u32 = 1024;
const BINARY_ROUNDING_RAW: u64 = (1 << 54) + 3;

const DECIMAL_CANCELLATION_SCALE: u32 = 400;
const BINARY_CANCELLATION_SCALE: u32 = 2048;
const BINARY_CANCELLATION_EXPONENT: u32 = 1024;
const BINARY_2_POW_MINUS_1024: f64 = f64::from_bits(1u64 << 50);

const DECIMAL_ROUNDING_VALUE: f64 = 1e-25;
const DECIMAL_UNDERFLOW_VALUE: f64 = 1e-309;
const BINARY_ROUNDING_VALUE: f64 = 9_007_199_254_740_994.0;
const EXACT_ONE: f64 = 1.0;

const ONE_AND_A_QUARTER: f64 = 1.25;
const THREE_QUARTERS: f64 = 0.75;

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

/// Pinned Dart 3.13.3 converts through `BigInt.toDouble()` and iterated powers.
/// These literals are the deliberate Rust deviations recorded in
/// `docs/parity/fixed-point-composition.md`.
#[test]
fn pinned_dart_float_deviations_are_explicit() {
    // Dart: 1.0000000000000002e-25 (0x3abef2d0f5da7dda).
    assert_eq!(
        signed::<10>(BigInt::from(1), DECIMAL_ROUNDING_SCALE),
        Ok(DECIMAL_ROUNDING_VALUE)
    );

    // Dart: 9007199254740992.0 (0x4340000000000000).
    assert_eq!(
        signed::<2>(BigInt::from(BINARY_ROUNDING_RAW), 1),
        Ok(BINARY_ROUNDING_VALUE)
    );

    // Dart: 0.0 (0x0000000000000000), losing the finite subnormal.
    assert_eq!(
        signed::<10>(BigInt::from(1), DECIMAL_UNDERFLOW_SCALE),
        Ok(DECIMAL_UNDERFLOW_VALUE)
    );
    assert_eq!(
        signed::<2>(BigInt::from(1), BINARY_SUBNORMAL_SCALE),
        Ok(BINARY_2_POW_MINUS_1024)
    );

    // Dart: NaN (0x7ff8000000000000) for both exactly finite values.
    assert_eq!(
        signed::<10>(
            BigInt::from(10).pow(DECIMAL_CANCELLATION_SCALE),
            DECIMAL_CANCELLATION_SCALE
        ),
        Ok(EXACT_ONE)
    );
    assert_eq!(
        signed::<2>(
            BigInt::from(1) << BINARY_CANCELLATION_EXPONENT,
            BINARY_CANCELLATION_SCALE
        ),
        Ok(BINARY_2_POW_MINUS_1024)
    );

    // Dart: Infinity (0x7ff0000000000000) instead of a typed error.
    let beyond_finite = BigInt::from(1) << BINARY_CANCELLATION_EXPONENT;
    assert_eq!(signed::<10>(beyond_finite.clone(), 0), Err(Error::Overflow));
    assert_eq!(signed::<2>(beyond_finite, 0), Err(Error::Overflow));
}

#[test]
fn ordinary_lossy_presentation_values_remain_finite() {
    for sign in [-1, 1] {
        assert_eq!(
            signed::<10>(BigInt::from(sign * 125), 2),
            Ok(f64::from(sign) * ONE_AND_A_QUARTER)
        );
        assert_eq!(
            signed::<2>(BigInt::from(sign * 3), 2),
            Ok(f64::from(sign) * THREE_QUARTERS)
        );
    }
}
