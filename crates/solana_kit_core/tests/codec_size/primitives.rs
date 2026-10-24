use alloc::vec::Vec;
use solana_kit_core::codecs::{
    Bool, Bytes, Count, Decoder, Encoder, Error, FixedBytes, Int256, Limits, Number, Sequence,
    ShortU16, Size, Uint256, Unit,
};

const LIMITS: Limits = Limits::new(64, 64);
const U64_WIDTH: usize = 8;
const U128_WIDTH: usize = 16;
const U256_WIDTH: usize = 32;
const ONE_BYTE_BOUNDARY: (u16, &[u8]) = (127, &[127]);
const FRAME_SUFFIX: u8 = 88;

/// Numeric declarations use the same widths for both byte orders and directions.
fn fixed<T, C: Encoder<T> + Decoder<T>>(codec: C, width: usize) {
    assert_eq!(codec.encode_size(), Ok(Size::Fixed(width)));
    assert_eq!(codec.decode_size(), Ok(Size::Fixed(width)));
    assert_eq!(
        <&C as Encoder<T>>::encode_size(&&codec),
        Ok(Size::Fixed(width))
    );
    assert_eq!(
        <&C as Decoder<T>>::decode_size(&&codec),
        Ok(Size::Fixed(width))
    );
}

/// All first-slice built-ins declare their formats without broader propagation.
pub fn declarations() {
    macro_rules! number {
        ($($ty:ty => $size:expr),+ $(,)?) => {$(
            fixed::<$ty, _>(Number::<$ty>::LE, $size);
            fixed::<$ty, _>(Number::<$ty>::BE, $size);
        )+};
    }

    number!(
        u8 => 1, i8 => 1, u16 => 2, i16 => 2, u32 => 4, i32 => 4,
        u64 => U64_WIDTH, i64 => U64_WIDTH,
        u128 => U128_WIDTH, i128 => U128_WIDTH, f32 => 4, f64 => U64_WIDTH,
        Uint256 => U256_WIDTH, Int256 => U256_WIDTH
    );

    fixed::<bool, _>(Bool, 1);
    fixed::<[u8; 0], _>(FixedBytes::<0>, 0);
    fixed::<[u8; 3], _>(FixedBytes::<3>, 3);
    fixed::<(), _>(Unit, 0);

    assert_eq!(Bytes.encode_size(), Ok(Size::Variable { max: None }));
    assert_eq!(Bytes.decode_size(), Ok(Size::Variable { max: None }));
    assert_eq!(ShortU16.encode_size(), Ok(Size::Variable { max: Some(3) }));
    assert_eq!(ShortU16.decode_size(), Ok(Size::Variable { max: Some(3) }));

    let unclassified = Sequence {
        item: Number::<u8>::LE,
        count: Count::Fixed(1),
    };

    assert_eq!(unclassified.encode_size(), Ok(Size::Unknown));
    assert_eq!(unclassified.decode_size(), Ok(Size::Unknown));
    assert_eq!(unclassified.encode(&[7][..], 1), Ok(alloc::vec![7]));
    assert_eq!(unclassified.decode(&[7], LIMITS), Ok(alloc::vec![7u8]));
    assert_eq!(Unit.encode(&(), 0), Ok(Vec::new()));
    assert_eq!(FixedBytes::<0>.decode(&[], LIMITS), Ok([]));
}

/// Compact metadata is tied to boundary bytes and nonzero prefix offsets.
pub fn compact_boundaries() {
    let cases: &[(u16, &[u8])] = &[
        (0, &[0]),
        ONE_BYTE_BOUNDARY,
        (128, &[128, 1]),
        (16383, &[255, 127]),
        (16384, &[128, 128, 1]),
        (65535, &[255, 255, 3]),
    ];

    for (value, bytes) in cases {
        assert_eq!(ShortU16.encoded_len(value), Ok(bytes.len()));
        assert_eq!(ShortU16.encode(value, 3).as_deref(), Ok(*bytes));
        let mut framed = alloc::vec![99];
        framed.extend_from_slice(bytes);
        framed.push(FRAME_SUFFIX);
        assert_eq!(
            ShortU16.decode_prefix(&framed, 1, LIMITS),
            Ok((*value, bytes.len() + 1))
        );
        assert_eq!(
            ShortU16.decode(&framed[1..], LIMITS),
            Err(Error::TrailingBytes)
        );
    }

    assert_eq!(ShortU16.decode(&[128, 0], LIMITS), Err(Error::NonCanonical));
    assert_eq!(
        ShortU16.decode(&[255, 255, 4], LIMITS),
        Err(Error::Overflow)
    );
    assert_eq!(ShortU16.decode(&[128], LIMITS), Err(Error::Truncated));
}
