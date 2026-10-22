#![cfg(feature = "fixed-points")]

use solana_kit_core::{
    codecs::{Decoder, Encoder, Endian, Limits, Writer},
    fixed_points::{
        BigInt, BinaryShape, DecimalShape, Error, FormatOptions, Rounding, Shape, Signedness,
    },
};

const OVER_LIMIT: u32 = 4097;
const MAX_WIDTH: u32 = 4096;
const OVER_INPUT: usize = 8193;
const U16_MAX: i32 = 65535;
const ROUNDING_RAW: i32 = 1235;

const HALF_BASE10_RAW: i32 = 5000;
const ODD_WIDTH: u32 = 257;

const MAX_WIRE_BYTES: usize = 512;
const WIDE_BYTE_BITS: u32 = 264;
const U256_BITS: u32 = 256;
const BYTE_MAX: u8 = 255;
const TWO_UNITS_RAW: i32 = 200;
const I128_BITS: u32 = 128;
const SIGNED_BYTE_MAX: i32 = 127;

const QUARTER_RAW: i32 = 125;
const ONE_UNIT_RAW: i32 = 100;
const U64_BITS: u32 = 64;
const CANARY: u8 = 42;

const U32_BITS: u32 = 32;
const U16_BITS: u32 = 16;
const SMALL_FRACTION_BITS: u32 = 12;
const U8_BITS: u32 = 8;
const UNALIGNED_BITS: u32 = 7;

#[test]
fn reference_parse_format_and_shape_boundaries() {
    let decimal = DecimalShape::new(Signedness::Signed, U64_BITS, 2).unwrap();
    let binary = BinaryShape::new(Signedness::Unsigned, U8_BITS, SMALL_FRACTION_BITS).unwrap();

    assert_eq!(
        decimal.parse("-.5", Rounding::Strict).unwrap().to_string(),
        "-0.5"
    );

    assert_eq!(
        decimal.parse("1.", Rounding::Strict).unwrap().raw(),
        &BigInt::from(ONE_UNIT_RAW)
    );

    assert_eq!(
        decimal.parse("1.000", Rounding::Strict),
        Err(Error::PrecisionLoss)
    );

    assert_eq!(
        binary
            .parse("0.062255859375", Rounding::Strict)
            .unwrap()
            .raw(),
        &BigInt::from(BYTE_MAX)
    );

    assert_eq!(
        binary.parse("0.0625", Rounding::Strict),
        Err(Error::Overflow)
    );
}

#[test]
fn malformed_input_and_shape_limits() {
    let decimal = DecimalShape::new(Signedness::Signed, U64_BITS, 2).unwrap();

    for bad in ["", ".", "-", "-.", "+1", " 1", "1 ", "1e2", "1.2.3", "١"] {
        assert_eq!(
            decimal.parse(bad, Rounding::Strict),
            Err(Error::InvalidSyntax),
            "{bad}"
        );
    }

    assert_eq!(
        BinaryShape::new(Signedness::Unsigned, 0, 0),
        Err(Error::InvalidShape)
    );

    assert_eq!(
        DecimalShape::new(Signedness::Signed, OVER_LIMIT, 0),
        Err(Error::LimitExceeded)
    );

    assert_eq!(
        DecimalShape::new(Signedness::Signed, U8_BITS, OVER_LIMIT),
        Err(Error::LimitExceeded)
    );

    assert_eq!(
        decimal.parse(&"1".repeat(OVER_INPUT), Rounding::Strict),
        Err(Error::LimitExceeded)
    );

    assert_eq!(
        BinaryShape::new(Signedness::Unsigned, U8_BITS, 2)
            .unwrap()
            .parse("-0", Rounding::Trunc),
        Err(Error::InvalidSyntax)
    );
}

#[test]
fn arithmetic_conversions_and_formatting_match_reference() {
    let cents = DecimalShape::new(Signedness::Signed, U32_BITS, 2).unwrap();
    let rate = DecimalShape::new(Signedness::Signed, U32_BITS, 4).unwrap();
    let one = cents.parse("1", Rounding::Strict).unwrap();

    assert_eq!(
        one.checked_mul(
            &rate.parse("0.3333", Rounding::Strict).unwrap(),
            Rounding::Nearest
        )
        .unwrap()
        .to_string(),
        "0.33"
    );

    assert_eq!(
        one.checked_div_scalar(-3, Rounding::Floor)
            .unwrap()
            .to_string(),
        "-0.34"
    );

    assert_eq!(
        one.checked_div_scalar(0, Rounding::Strict),
        Err(Error::DivisionByZero)
    );

    assert_eq!(
        one.checked_add(&rate.raw(1).unwrap()),
        Err(Error::ShapeMismatch)
    );

    assert_eq!(
        one.checked_div_scalar(3, Rounding::Strict),
        Err(Error::PrecisionLoss)
    );

    assert_eq!(
        one.checked_mul_scalar(3)
            .unwrap()
            .checked_sub(&one)
            .unwrap()
            .raw(),
        &BigInt::from(TWO_UNITS_RAW)
    );

    assert_eq!(
        one.checked_div(
            &rate.parse("0.05", Rounding::Strict).unwrap(),
            Rounding::Strict
        )
        .unwrap()
        .to_string(),
        "20"
    );

    assert_eq!(
        cents
            .raw(-ROUNDING_RAW)
            .unwrap()
            .rescale(U32_BITS, 1, Rounding::Nearest)
            .unwrap()
            .to_string(),
        "-12.4"
    );

    assert_eq!(
        cents.raw(-1).unwrap().with_signedness(Signedness::Unsigned),
        Err(Error::Overflow)
    );
}

#[test]
fn signed_minimum_and_comparison_boundaries() {
    let cents = DecimalShape::new(Signedness::Signed, U32_BITS, 2).unwrap();
    let one = cents.raw(ONE_UNIT_RAW).unwrap();
    let signed8 = DecimalShape::new(Signedness::Signed, U8_BITS, 0).unwrap();

    assert_eq!(
        signed8.raw(i8::MIN).unwrap().checked_neg(),
        Err(Error::Overflow)
    );

    assert_eq!(
        signed8.raw(i8::MIN).unwrap().checked_abs(),
        Err(Error::Overflow)
    );

    assert_eq!(
        signed8
            .raw(SIGNED_BYTE_MAX)
            .unwrap()
            .checked_add(&signed8.raw(1).unwrap()),
        Err(Error::Overflow)
    );
    let wide = DecimalShape::new(Signedness::Unsigned, U64_BITS, 2)
        .unwrap()
        .raw(ONE_UNIT_RAW)
        .unwrap();

    assert!(one.compare(&wide).unwrap().is_eq());

    assert_eq!(one.compare_same_shape(&wide), Err(Error::ShapeMismatch));

    assert_ne!(one, wide);
}

#[test]
fn formatting_options_and_formatter_preserve_exact_values() {
    let cents = DecimalShape::new(Signedness::Signed, U32_BITS, 2).unwrap();
    let one = cents.raw(ONE_UNIT_RAW).unwrap();
    let padded = FormatOptions {
        decimals: Some(4),
        pad_trailing_zeros: true,
        ..FormatOptions::default()
    };

    assert_eq!(one.format(padded).unwrap(), "1.0000");

    assert_eq!(one.format_with(str::to_owned), "100E-2");

    assert_eq!(one.to_number().unwrap(), 1.0);

    assert_eq!(
        cents
            .raw(QUARTER_RAW)
            .unwrap()
            .format(FormatOptions {
                decimals: Some(1),
                rounding: Rounding::Nearest,
                ..FormatOptions::default()
            })
            .unwrap(),
        "1.3"
    );
}

#[test]
fn exact_binary_and_arbitrary_width_vectors() {
    let quarters = BinaryShape::new(Signedness::Signed, U16_BITS, 4).unwrap();
    let value = quarters.parse("0.5", Rounding::Strict).unwrap();

    assert_eq!(value.to_base10().raw(), &BigInt::from(HALF_BASE10_RAW));

    assert_eq!(value.to_base10().decimals(), 4);

    assert_eq!(
        value
            .format(FormatOptions {
                pad_trailing_zeros: true,
                ..FormatOptions::default()
            })
            .unwrap(),
        "0.5000"
    );

    assert_eq!(value.format_with(str::to_owned), "5000E-4");

    assert_eq!(value.to_number().unwrap(), 0.5);

    assert_eq!(
        quarters.parse("0.1", Rounding::Strict),
        Err(Error::PrecisionLoss)
    );

    assert_eq!(
        quarters.parse("0.09375", Rounding::Nearest).unwrap().raw(),
        &BigInt::from(2)
    );
    let shape = DecimalShape::new(Signedness::Unsigned, ODD_WIDTH, 0).unwrap();
    let max: BigInt = (BigInt::from(1) << 257u32) - 1;

    assert_eq!(shape.raw(max.clone()).unwrap().to_string(), max.to_string());

    assert_eq!(shape.raw(max + 1), Err(Error::Overflow));
    let tiny = BinaryShape::new(Signedness::Unsigned, 1, MAX_WIDTH)
        .unwrap()
        .raw(1)
        .unwrap();

    assert!(tiny.to_string().starts_with("0.000"));

    assert_eq!(
        tiny.rescale(MAX_WIDTH, 0, Rounding::Ceil).unwrap().raw(),
        &BigInt::from(1)
    );
}

#[test]
fn codecs_preserve_boundaries_offsets_and_budgets() {
    for endian in [Endian::Little, Endian::Big] {
        for bits in [
            U8_BITS,
            U16_BITS,
            U64_BITS,
            I128_BITS,
            U256_BITS,
            WIDE_BYTE_BITS,
            MAX_WIDTH,
        ] {
            let shape =
                BinaryShape::new(Signedness::Signed, bits, bits + u32::from(bits < MAX_WIDTH))
                    .unwrap();
            let codec = shape.codec(endian).unwrap();
            let (min, max) = shape.raw_range();

            for raw in [min, BigInt::from(-1), BigInt::from(0), max] {
                let value = shape.raw(raw).unwrap();
                let bytes = codec.encode(&value, MAX_WIRE_BYTES).unwrap();

                assert_eq!(
                    codec
                        .decode(&bytes, Limits::new(MAX_WIRE_BYTES, MAX_WIRE_BYTES))
                        .unwrap(),
                    value
                );

                assert!(
                    codec
                        .decode(&bytes, Limits::new(MAX_WIRE_BYTES, 0))
                        .is_err()
                );

                assert!(
                    codec
                        .decode(
                            &bytes[..bytes.len() - 1],
                            Limits::new(MAX_WIRE_BYTES, MAX_WIRE_BYTES)
                        )
                        .is_err()
                );
            }
        }
    }
}

#[test]
fn codec_exact_bytes_offsets_and_atomic_shape_failure() {
    let shape = DecimalShape::new(Signedness::Signed, U16_BITS, 2).unwrap();
    let codec = shape.codec(Endian::Big).unwrap();

    assert_eq!(
        codec.encode(&shape.raw(-2).unwrap(), 2).unwrap(),
        [BYTE_MAX, BYTE_MAX - 1]
    );
    let unsigned = DecimalShape::new(Signedness::Unsigned, U16_BITS, 2).unwrap();

    assert_eq!(
        unsigned
            .codec(Endian::Little)
            .unwrap()
            .encode(&unsigned.raw(U16_MAX).unwrap(), 2)
            .unwrap(),
        [BYTE_MAX, BYTE_MAX]
    );
    let mut buffer = [CANARY; 4];
    let mut writer = Writer::new(&mut buffer);
    writer.put(&[CANARY]).unwrap();
    codec.write(&shape.raw(-2).unwrap(), &mut writer).unwrap();

    assert_eq!(writer.position(), 3);

    assert_eq!(buffer, [CANARY, BYTE_MAX, 254, CANARY]);

    assert_eq!(
        codec
            .decode_prefix(&buffer, 1, Limits::new(4, 2))
            .unwrap()
            .1,
        3
    );

    assert!(codec.decode(&buffer[1..], Limits::new(4, 2)).is_err());
    let mut buffer = [CANARY; 2];

    assert!(
        codec
            .write(&unsigned.raw(1).unwrap(), &mut Writer::new(&mut buffer))
            .is_err()
    );

    assert_eq!(buffer, [CANARY; 2]);

    assert!(
        DecimalShape::new(Signedness::Signed, UNALIGNED_BITS, 0)
            .unwrap()
            .codec(Endian::Little)
            .is_err()
    );
}

/// Euclidean floor and ceiling bracket the rational independently of truncating BigInt division.
fn oracle(numerator: i128, denominator: i128, rounding: Rounding) -> Result<i128, Error> {
    if denominator < 0 {
        return oracle(-numerator, -denominator, rounding);
    }

    let floor = numerator.div_euclid(denominator);
    let ceil = floor + i128::from(numerator.rem_euclid(denominator) != 0);

    match rounding {
        Rounding::Strict if floor != ceil => Err(Error::PrecisionLoss),
        Rounding::Strict | Rounding::Floor => Ok(floor),
        Rounding::Ceil => Ok(ceil),
        Rounding::Trunc => Ok(if numerator < 0 { ceil } else { floor }),
        Rounding::Nearest => Ok(nearest(numerator, denominator, floor, ceil)),
    }
}

fn nearest(numerator: i128, denominator: i128, floor: i128, ceil: i128) -> i128 {
    let low = numerator - floor * denominator;
    let high = ceil * denominator - numerator;

    if low < high || (low == high && numerator < 0) {
        return floor;
    }

    ceil
}

fn check_ratio<const RADIX: u8>(scale: u32, numerator: i128, denominator: i128) {
    let shape = Shape::<RADIX>::new(Signedness::Signed, u64::BITS, scale).unwrap();

    for rounding in [
        Rounding::Strict,
        Rounding::Floor,
        Rounding::Ceil,
        Rounding::Trunc,
        Rounding::Nearest,
    ] {
        let expected = oracle(
            numerator * i128::from(RADIX).pow(scale),
            denominator,
            rounding,
        )
        .map(BigInt::from);
        let actual = shape
            .ratio(numerator, denominator, rounding)
            .map(|v| v.raw().clone());

        assert_eq!(
            actual, expected,
            "{RADIX} {scale} {numerator}/{denominator} {rounding:?}"
        );
    }
}

#[test]
fn independent_rational_oracle_covers_signs_ties_and_precision() {
    const NUMERATOR_RADIUS: i128 = 24;
    const DENOMINATOR_RADIUS: i128 = 11;

    for numerator in -NUMERATOR_RADIUS..=NUMERATOR_RADIUS {
        for denominator in (-DENOMINATOR_RADIUS..=DENOMINATOR_RADIUS).filter(|value| *value != 0) {
            for scale in [0, 1, 3] {
                check_ratio::<2>(scale, numerator, denominator);
                check_ratio::<10>(scale, numerator, denominator);
            }
        }
    }
}

#[test]
fn immutable_guards_hashes_and_binary_arithmetic() {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    let shape = BinaryShape::new(Signedness::Signed, U16_BITS, 4).unwrap();
    let one = shape.parse("1", Rounding::Strict).unwrap();
    let quarter = shape.parse("0.25", Rounding::Strict).unwrap();
    assert_eq!(
        one.checked_mul(&quarter, Rounding::Strict).unwrap(),
        quarter
    );
    assert_eq!(
        one.checked_div(&quarter, Rounding::Strict)
            .unwrap()
            .to_string(),
        "4"
    );
    assert_eq!(
        quarter.checked_neg().unwrap().checked_abs().unwrap(),
        quarter
    );
    assert_eq!(one.kind(), "binaryFixedPoint");
    assert!(one.has_shape(Some(Signedness::Signed), Some(U16_BITS), Some(4)));
    assert!(one.assert_shape(None, Some(U8_BITS), None).is_err());
    let mut hasher = DefaultHasher::new();
    one.hash(&mut hasher);
    let mut other = DefaultHasher::new();
    one.clone().hash(&mut other);
    assert_eq!(hasher.finish(), other.finish());
    let base = one.to_base10();
    assert_eq!(
        solana_kit_core::fixed_points::Base10::new(base.raw().clone(), base.decimals()).unwrap(),
        base
    );
}

#[test]
fn oversized_operands_and_unsigned_failures_are_explicit() {
    const OPERAND_LIMIT: u32 = 32768;
    let shape = DecimalShape::new(Signedness::Unsigned, U8_BITS, 0).unwrap();
    let huge = BigInt::from(1) << OPERAND_LIMIT;
    assert_eq!(
        shape.ratio(huge.clone(), 1, Rounding::Strict),
        Err(Error::LimitExceeded)
    );
    let one = shape.raw(1).unwrap();
    assert_eq!(one.checked_mul_scalar(huge), Err(Error::LimitExceeded));
    assert_eq!(one.checked_neg(), Err(Error::UnsignedNegation));
    assert_eq!(
        shape.raw(0).unwrap().checked_sub(&one),
        Err(Error::Overflow)
    );
    assert_eq!(
        shape
            .raw(u8::MAX)
            .unwrap()
            .with_signedness(Signedness::Signed),
        Err(Error::Overflow)
    );
    assert_eq!(one.kind(), "decimalFixedPoint");
}
