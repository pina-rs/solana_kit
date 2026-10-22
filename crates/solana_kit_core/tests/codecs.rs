//! Fixed byte contracts from the pinned Dart tests and independently specified
//! Borsh layouts. Fixture provenance lives in docs/verification/core-codecs.md.

use solana_kit_core::codecs::{
    Affix, Bool, Bytes, Constant, Count, Decoder, Encoder, Error, FixedBytes, Limits, Number,
    OptionCodec, Reader, Sequence, ShortU16, SizePrefix, SizePrefix64, Tuple, Utf8, Writer,
};

const LIMITS: Limits = Limits::new(1024, 128);
const EXAMPLE: u16 = 42;
const EXAMPLE_BYTE: u8 = 42;
const NEXT_BYTE: u8 = 43;
const PREFIX_END: usize = 5;
const SMALL_BUDGET: usize = 20;
const C_OPTION_SIZE: usize = 6;
const MAX_SHORT_SINGLE: u16 = 127;
const U64_EXAMPLE: &[u8] = &[255, 255, 255, 255, 0, 0, 0, 0];
const I64_MINIMUM: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 128];

#[test]
fn integer_widths_orders_and_signed_boundaries() {
    macro_rules! fixture {
        ($kind:ty, $value:expr, $bytes:expr) => {
            let codec = Number::<$kind>::LE;
            let value: $kind = $value;
            let bytes: &[u8] = $bytes;
            assert_eq!(codec.encoded_len(&value), Ok(bytes.len()));
            assert_eq!(codec.encode(&value, 32).unwrap(), bytes);
            assert_eq!(codec.decode(bytes, LIMITS), Ok(value));
            let reversed: Vec<_> = bytes.iter().rev().copied().collect();
            assert_eq!(Number::<$kind>::BE.encode(&value, 32).unwrap(), reversed);
            assert_eq!(Number::<$kind>::BE.decode(&reversed, LIMITS), Ok(value));
            assert_eq!(
                codec.decode(&bytes[..bytes.len() - 1], LIMITS),
                Err(Error::Truncated)
            );
        };
    }

    fixture!(u8, 255, &[255]);
    fixture!(i8, -128, &[128]);
    fixture!(u16, 0xabcd, &[0xcd, 0xab]);
    fixture!(i16, -32768, &[0, 128]);
    fixture!(u32, 65535, &[255, 255, 0, 0]);

    fixture!(i32, i32::MIN, &[0, 0, 0, 128]);
    fixture!(u64, 0xffff_ffff, U64_EXAMPLE);
    fixture!(i64, i64::MIN, I64_MINIMUM);
    fixture!(u128, u128::MAX, &[255; 16]);
    fixture!(i128, -1, &[255; 16]);
}

#[test]
fn floating_point_wire_bits_are_preserved() {
    let nan = f32::from_bits(0x7fc0_0123);
    let encoded = Number::<f32>::LE.encode(&nan, 4).unwrap();
    assert_eq!(encoded, [0x23, 0x01, 0xc0, 0x7f]);
    assert_eq!(
        Number::<f32>::LE
            .decode(&encoded, LIMITS)
            .unwrap()
            .to_bits(),
        nan.to_bits()
    );
    assert_eq!(
        Number::<f64>::BE.encode(&-0.0, 8).unwrap(),
        [128, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        Number::<f64>::BE.decode(&[127, 240, 0, 0, 0, 0, 0, 0], LIMITS),
        Ok(f64::INFINITY)
    );

    assert_eq!(Number::<f32>::BE.encode(&1.0, 4).unwrap(), [63, 128, 0, 0]);
    assert_eq!(Number::<f32>::BE.decode(&[63, 128, 0, 0], LIMITS), Ok(1.0));
    assert_eq!(
        Number::<f64>::LE.encode(&1.0, 8).unwrap(),
        [0, 0, 0, 0, 0, 0, 240, 63]
    );
    assert_eq!(
        Number::<f64>::LE.decode(&[0, 0, 0, 0, 0, 0, 240, 63], LIMITS),
        Ok(1.0)
    );
}

#[test]
fn short_u16_fixed_vectors_and_all_values() {
    for (value, bytes) in [
        (0, &[0][..]),
        (MAX_SHORT_SINGLE, &[127]),
        (128, &[128, 1]),
        (16383, &[255, 127]),
        (16384, &[128, 128, 1]),
        (65535, &[255, 255, 3]),
    ] {
        assert_eq!(ShortU16.encode(&value, 3).unwrap(), bytes);
        assert_eq!(ShortU16.decode(bytes, LIMITS), Ok(value));
    }

    for value in 0..=u16::MAX {
        let bytes = ShortU16.encode(&value, 3).unwrap();
        assert_eq!(ShortU16.decode(&bytes, LIMITS), Ok(value));
    }
}

#[test]
fn malformed_short_u16_and_cursor_policies() {
    assert_eq!(ShortU16.decode(&[128, 0], LIMITS), Err(Error::NonCanonical));
    assert_eq!(
        ShortU16.decode(&[128, 128, 0], LIMITS),
        Err(Error::NonCanonical)
    );
    assert_eq!(
        ShortU16.decode(&[255, 255, 4], LIMITS),
        Err(Error::Overflow)
    );
    assert_eq!(
        ShortU16.decode(&[255, 255, 128], LIMITS),
        Err(Error::Overflow)
    );
    assert_eq!(ShortU16.decode(&[128], LIMITS), Err(Error::Truncated));

    assert_eq!(
        Number::<u8>::LE.decode(&[1, 2], LIMITS),
        Err(Error::TrailingBytes)
    );
    assert_eq!(
        Number::<u32>::LE.decode_prefix(&[99, 42, 0, 0, 0, 88], 1, LIMITS),
        Ok((u32::from(EXAMPLE), PREFIX_END))
    );
    assert_eq!(
        Number::<u8>::LE.decode_prefix(&[1], 2, LIMITS),
        Err(Error::Truncated)
    );
    assert_eq!(Number::<u8>::LE.encode(&1, 0), Err(Error::LimitExceeded));
}

#[test]
fn borsh_string_and_vector_fixtures() {
    let text = SizePrefix(Utf8);
    let bytes = [3, 0, 0, 0, b'h', 0xc3, 0xa9];
    assert_eq!(text.encode("hé", 7).unwrap(), bytes);
    assert_eq!(text.decode(&bytes, LIMITS).unwrap(), "hé");

    let vector = Sequence {
        item: Number::<u16>::LE,
        count: Count::U32,
    };
    let bytes = [2, 0, 0, 0, 1, 0, 255, 0];
    assert_eq!(vector.encode(&[1, 255], 8).unwrap(), bytes);
    assert_eq!(vector.decode(&bytes, LIMITS).unwrap(), [1, 255]);

    let fixed = Sequence {
        item: Bool,
        count: Count::Fixed(2),
    };
    assert_eq!(fixed.encode(&[true, false], 2).unwrap(), [1, 0]);
    assert_eq!(fixed.decode(&[1, 0], LIMITS).unwrap(), [true, false]);
    assert_eq!(fixed.encode(&[true], 2), Err(Error::InvalidValue));
}

#[test]
fn lengths_allocations_utf8_and_frame_boundaries_are_checked() {
    let vector = Sequence {
        item: Constant(&[]),
        count: Count::U32,
    };
    assert_eq!(vector.decode(&[255; 4], LIMITS), Err(Error::LimitExceeded));
    assert_eq!(vector.decode(&[], LIMITS), Err(Error::Truncated));

    assert_eq!(
        SizePrefix(Utf8).decode(&[1, 0, 0, 0, 255], LIMITS),
        Err(Error::InvalidUtf8)
    );
    assert_eq!(
        SizePrefix(Bytes).decode(&[255; 4], LIMITS),
        Err(Error::Truncated)
    );
    assert_eq!(
        SizePrefix(Number::<u8>::LE).decode(&[2, 0, 0, 0, 4, 5], LIMITS),
        Err(Error::TrailingBytes)
    );
    assert_eq!(
        SizePrefix(Number::<u16>::LE).decode(&[1, 0, 0, 0, 4, 5], LIMITS),
        Err(Error::Truncated)
    );
    let nested = Sequence {
        item: SizePrefix(Bytes),
        count: Count::U32,
    };

    let bytes = [2, 0, 0, 0, 1, 0, 0, 0, 42, 1, 0, 0, 0, 43];
    assert_eq!(
        nested.decode(&bytes, Limits::new(SMALL_BUDGET, 3)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        nested.decode(&bytes, Limits::new(SMALL_BUDGET, 4)).unwrap(),
        [vec![EXAMPLE_BYTE], vec![NEXT_BYTE]]
    );
    assert_eq!(
        Bytes.decode(&[1, 2], Limits::new(1, 10)),
        Err(Error::LimitExceeded)
    );
}

#[test]
fn options_have_distinct_strict_wire_contracts() {
    let item = Number::<u16>::LE;
    assert_eq!(
        OptionCodec::u8(item).encode(&Some(EXAMPLE), 3).unwrap(),
        [1, 42, 0]
    );
    assert_eq!(OptionCodec::u8(item).encode(&None, 1).unwrap(), [0]);
    assert_eq!(OptionCodec::u32(item).encode(&None, 4).unwrap(), [0; 4]);
}

#[test]
fn c_options_have_fixed_payload_storage() {
    let item = Number::<u16>::LE;

    assert_eq!(
        OptionCodec::c_option(item, 2)
            .encode(&None, C_OPTION_SIZE)
            .unwrap(),
        [0; 6]
    );
    assert_eq!(
        OptionCodec::c_option(item, 2).decode(&[0, 0, 0, 0, 99, 88], LIMITS),
        Ok(None)
    );
    assert_eq!(
        OptionCodec::c_option(item, 2).decode(&[1, 0, 0, 0, 42, 0], LIMITS),
        Ok(Some(EXAMPLE))
    );
    assert_eq!(
        OptionCodec::c_option(item, 2).decode(&[0; 5], LIMITS),
        Err(Error::Truncated)
    );
    assert_eq!(
        OptionCodec::c_option(item, 1).encode(&Some(EXAMPLE), C_OPTION_SIZE),
        Err(Error::InvalidValue)
    );
}

#[test]
fn invalid_tags_and_remainder_options() {
    let item = Number::<u16>::LE;

    assert_eq!(
        OptionCodec::u8(item).decode(&[2, 0, 0], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(
        OptionCodec::u32(item).decode(&[0, 1, 0, 0], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(OptionCodec::remainder(item).decode(&[], LIMITS), Ok(None));
    assert_eq!(
        OptionCodec::remainder(item).decode(&[42, 0], LIMITS),
        Ok(Some(EXAMPLE))
    );
    assert_eq!(
        OptionCodec::remainder(item)
            .encode(&Some(EXAMPLE), 2)
            .unwrap(),
        [42, 0]
    );
    assert_eq!(
        OptionCodec::remainder(Constant(&[])).encode(&Some(()), 0),
        Err(Error::InvalidValue)
    );
}

#[test]
fn tuple_constants_and_fixed_byte_layouts() {
    let codec = Affix {
        prefix: &[1, 2, 3],
        item: Tuple(Number::<u8>::LE, Bool),
        suffix: &[4, 5],
    };
    let bytes = [1, 2, 3, 42, 1, 4, 5];
    assert_eq!(codec.encode(&(42, true), 7).unwrap(), bytes);
    assert_eq!(codec.decode(&bytes, LIMITS), Ok((42, true)));

    assert_eq!(
        codec.decode(&[1, 2, 4, 42, 1, 4, 5], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(
        codec.decode(&[1, 2, 3, 42, 1, 4, 6], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(
        Bool.decode(&[255], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(FixedBytes::<3>.decode(&[1, 2, 3], LIMITS), Ok([1, 2, 3]));

    assert_eq!(FixedBytes::<3>.encode(&[1, 2, 3], 3).unwrap(), [1, 2, 3]);
    assert_eq!(Constant(&[1, 2, 3]).encode(&(), 3).unwrap(), [1, 2, 3]);
}

#[test]
fn remainder_progress_and_failed_cursor_ranges() {
    let remainder = Sequence {
        item: Number::<u8>::LE,
        count: Count::Remainder,
    };
    assert_eq!(remainder.decode(&[1, 2, 3], LIMITS).unwrap(), [1, 2, 3]);
    let stalled = Sequence {
        item: Constant(&[]),
        count: Count::Remainder,
    };
    assert_eq!(stalled.decode(&[1], LIMITS), Err(Error::InvalidValue));

    let mut reader = Reader::new(&[1, 2], LIMITS).unwrap();
    assert_eq!(reader.take(3), Err(Error::Truncated));
    assert_eq!(reader.position(), 0);
    reader.take(1).unwrap();
    assert_eq!(reader.take(usize::MAX), Err(Error::Overflow));
    assert_eq!(reader.finish(), Err(Error::TrailingBytes));

    let mut output = [99; 2];
    let mut writer = Writer::new(&mut output);
    assert_eq!(writer.put(&[1, 2, 3]), Err(Error::Truncated));
    assert_eq!(writer.position(), 0);
    writer.fill(2, EXAMPLE_BYTE).unwrap();
    assert_eq!(output, [42, 42]);
}

#[test]
fn u64_length_prefix_is_distinct_and_bounded() {
    let codec = SizePrefix64(Utf8);
    let bytes = [3, 0, 0, 0, 0, 0, 0, 0, b'h', 0xc3, 0xa9];
    assert_eq!(codec.encode("hé", bytes.len()).unwrap(), bytes);
    assert_eq!(codec.decode(&bytes, LIMITS).unwrap(), "hé");
    assert_eq!(codec.decode(&[255; 8], LIMITS), Err(Error::Overflow));
    assert_eq!(
        codec.decode(&[2, 0, 0, 0, 0, 0, 0, 0, 42], LIMITS),
        Err(Error::Truncated)
    );
}
