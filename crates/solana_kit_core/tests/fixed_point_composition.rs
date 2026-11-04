#![cfg(feature = "fixed-points")]

//! Consumer-level substitutes for the reference's callback constructors.
//!
//! The pinned Dart API stores caller callbacks in `FixedPointEncoder`,
//! `FixedPointDecoder` and `FixedPointCodec`. Rust callers implement the public
//! `Encoder`/`Decoder` traits, declare their fixed size, and compose with
//! `Paired::try_new` or `Mapped`. No production callback factory is required.
use core::cell::Cell;
use solana_kit_core::{
    codecs::{
        Decoder, Encoder, Endian, Error as CodecError, Limits, Mapped, Number, PairError, Paired,
        Reader, Size, Writer,
    },
    fixed_points::{DecimalFixedPoint, FixedPoint, Shape, Signedness},
};

const WIRE_BYTES: usize = 2;
const SIGN_MAGNITUDE_MAX: i32 = 0x7fff;
const PREFIX: u8 = 0xa5;
const SUFFIX: u8 = 0x5a;

const LIMITS: Limits = Limits::new(16, 16);
const MAX_CENTS: i64 = 10_000;
const SAMPLE_CENTS: i64 = 125;
const SAMPLE_CENTS_WIRE: [u8; WIRE_BYTES] = [125, 0];
const OUT_OF_POLICY_CENTS: i64 = MAX_CENTS + 1;
const OUT_OF_POLICY_WIRE: [u8; WIRE_BYTES] = [0x11, 0x27];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Cents(i64);

/// Records callback entries independently for each wire direction.
#[derive(Default)]
struct Calls {
    writes: Cell<usize>,
    reads: Cell<usize>,
}

/// Application-owned sign-magnitude wire format with a declared fixed width.
///
/// This deliberately differs from the built-in two's-complement codec: the sign
/// bit is separate, negative zero is reserved, and the signed minimum is not
/// representable. Those are user-observable callback contracts.
struct SignMagnitude<const RADIX: u8> {
    shape: Shape<RADIX>,
    calls: Calls,
}

impl<const RADIX: u8> SignMagnitude<RADIX> {
    fn new(shape: Shape<RADIX>) -> Self {
        Self {
            shape,
            calls: Calls::default(),
        }
    }

    fn encoded(&self, value: &FixedPoint<RADIX>) -> Result<[u8; WIRE_BYTES], CodecError> {
        if value.shape() != self.shape {
            return Err(CodecError::InvalidValue);
        }

        let raw = i16::try_from(value.raw()).map_err(|_| CodecError::InvalidValue)?;

        if raw == i16::MIN {
            return Err(CodecError::InvalidValue);
        }

        let magnitude = raw.unsigned_abs();
        let encoded = if raw.is_negative() {
            magnitude | 0x8000
        } else {
            magnitude
        };

        Ok(encoded.to_be_bytes())
    }
}

impl<const RADIX: u8> Encoder<FixedPoint<RADIX>> for SignMagnitude<RADIX> {
    fn encode_size(&self) -> Result<Size, CodecError> {
        Ok(Size::Fixed(WIRE_BYTES))
    }

    fn encoded_len(&self, value: &FixedPoint<RADIX>) -> Result<usize, CodecError> {
        self.encoded(value)?;

        Ok(WIRE_BYTES)
    }

    fn write(&self, value: &FixedPoint<RADIX>, writer: &mut Writer<'_>) -> Result<(), CodecError> {
        self.calls.writes.set(self.calls.writes.get() + 1);
        let encoded = self.encoded(value)?;

        writer.put(&encoded)
    }
}

/// Declares two bytes but writes one; only the typed harness sees the mismatch.
struct ShortFixedPoint;

impl Encoder<DecimalFixedPoint> for ShortFixedPoint {
    fn encode_size(&self) -> Result<Size, CodecError> {
        Ok(Size::Fixed(WIRE_BYTES))
    }

    fn encoded_len(&self, _value: &DecimalFixedPoint) -> Result<usize, CodecError> {
        Ok(WIRE_BYTES)
    }

    fn write(&self, _value: &DecimalFixedPoint, writer: &mut Writer<'_>) -> Result<(), CodecError> {
        writer.put(&[PREFIX])
    }
}

impl<const RADIX: u8> Decoder<FixedPoint<RADIX>> for SignMagnitude<RADIX> {
    fn decode_size(&self) -> Result<Size, CodecError> {
        Ok(Size::Fixed(WIRE_BYTES))
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<FixedPoint<RADIX>, CodecError> {
        self.calls.reads.set(self.calls.reads.get() + 1);
        let bytes = reader.take(WIRE_BYTES)?;
        let encoded = u16::from_be_bytes([bytes[0], bytes[1]]);
        let magnitude = encoded & 0x7fff;
        let negative = encoded & 0x8000 != 0;

        if negative && magnitude == 0 {
            return Err(CodecError::InvalidValue);
        }

        let magnitude = i32::from(magnitude);
        let raw = if negative { -magnitude } else { magnitude };

        self.shape.raw(raw).map_err(|_| CodecError::InvalidValue)
    }
}

/// Builds a checked `Cents` view over the built-in decimal wire codec.
fn cents_codec<'a>(
    shape: Shape<10>,
    writes: &'a Cell<usize>,
    reads: &'a Cell<usize>,
) -> impl Encoder<Cents> + Decoder<Cents> + 'a {
    Mapped::new(
        shape.codec(Endian::Little).unwrap(),
        move |value: &Cents| {
            writes.set(writes.get() + 1);

            if !(-MAX_CENTS..=MAX_CENTS).contains(&value.0) {
                return Err(CodecError::NumberOutOfRange);
            }

            shape.raw(value.0).map_err(|_| CodecError::NumberOutOfRange)
        },
        move |value: DecimalFixedPoint| {
            reads.set(reads.get() + 1);
            let cents = i64::try_from(value.raw()).map_err(|_| CodecError::Overflow)?;

            if !(-MAX_CENTS..=MAX_CENTS).contains(&cents) {
                return Err(CodecError::NumberOutOfRange);
            }

            Ok(Cents(cents))
        },
    )
}

#[test]
fn custom_encoder_is_a_public_encoder_with_offsets_and_callback_errors() {
    let shape = Shape::<10>::new(Signedness::Signed, 16, 2).unwrap();
    let encoder = SignMagnitude::new(shape);
    let value = shape.raw(-2).unwrap();

    assert_eq!(encoder.encode_size(), Ok(Size::Fixed(WIRE_BYTES)));
    assert_eq!(encoder.encoded_len(&value), Ok(WIRE_BYTES));
    assert_eq!(encoder.calls.writes.get(), 0);

    let mut buffer = [SUFFIX; 4];
    let mut writer = Writer::new(&mut buffer);
    writer.put(&[PREFIX]).unwrap();
    encoder.write(&value, &mut writer).unwrap();

    assert_eq!(writer.position(), 3);
    assert_eq!(buffer, [PREFIX, 0x80, 0x02, SUFFIX]);
    assert_eq!(encoder.calls.writes.get(), 1);

    assert_eq!(encoder.encode(&value, 1), Err(CodecError::LimitExceeded));
    assert_eq!(encoder.calls.writes.get(), 1);

    let mut too_small = [SUFFIX; 1];
    assert_eq!(
        encoder.write(&value, &mut Writer::new(&mut too_small)),
        Err(CodecError::Truncated)
    );
    assert_eq!(too_small, [SUFFIX]);
    assert_eq!(encoder.calls.writes.get(), 2);

    let signed_minimum = shape.raw(-SIGN_MAGNITUDE_MAX - 1).unwrap();
    let mut rejected = [SUFFIX; WIRE_BYTES];
    assert_eq!(
        encoder.write(&signed_minimum, &mut Writer::new(&mut rejected)),
        Err(CodecError::InvalidValue)
    );
    assert_eq!(rejected, [SUFFIX; WIRE_BYTES]);
    assert_eq!(encoder.calls.writes.get(), 3);

    let other_shape = Shape::<10>::new(Signedness::Signed, 16, 3).unwrap();
    assert_eq!(
        encoder.encode(&other_shape.raw(1).unwrap(), WIRE_BYTES),
        Err(CodecError::InvalidValue)
    );
    assert_eq!(encoder.calls.writes.get(), 3);

    assert_eq!(
        ShortFixedPoint.encode(&value, WIRE_BYTES),
        Err(CodecError::InvalidValue)
    );
}

#[test]
fn custom_decoder_is_a_public_decoder_with_offsets_and_callback_errors() {
    let shape = Shape::<2>::new(Signedness::Signed, 16, 4).unwrap();
    let decoder = SignMagnitude::new(shape);
    let value = shape.raw(2).unwrap();

    assert_eq!(decoder.decode_size(), Ok(Size::Fixed(WIRE_BYTES)));
    assert_eq!(decoder.calls.reads.get(), 0);

    let input = [PREFIX, 0x00, 0x02, SUFFIX];
    assert_eq!(
        decoder.decode_prefix(&input, 1, LIMITS),
        Ok((value.clone(), 3))
    );
    assert_eq!(decoder.calls.reads.get(), 1);

    assert_eq!(decoder.decode(&[0x00, 0x02], Limits::new(2, 0)), Ok(value));
    assert_eq!(decoder.calls.reads.get(), 2);

    assert_eq!(decoder.decode(&[0x00], LIMITS), Err(CodecError::Truncated));
    assert_eq!(decoder.calls.reads.get(), 3);

    assert_eq!(
        decoder.decode(&[0x80, 0x00], LIMITS),
        Err(CodecError::InvalidValue)
    );
    assert_eq!(decoder.calls.reads.get(), 4);

    assert_eq!(
        decoder.decode(&[0x00, 0x02, SUFFIX], LIMITS),
        Err(CodecError::TrailingBytes)
    );
    assert_eq!(decoder.calls.reads.get(), 5);

    assert_eq!(
        decoder.decode(&[0x00, 0x02], Limits::new(1, 0)),
        Err(CodecError::LimitExceeded)
    );
    assert_eq!(decoder.calls.reads.get(), 5);

    assert_eq!(
        decoder.decode(&[0xff, 0xff], LIMITS),
        Ok(shape.raw(-SIGN_MAGNITUDE_MAX).unwrap())
    );
    assert_eq!(
        decoder.decode(&[0x7f, 0xff], LIMITS),
        Ok(shape.raw(SIGN_MAGNITUDE_MAX).unwrap())
    );
}

#[test]
fn paired_try_new_composes_declared_custom_halves() {
    let shape = Shape::<10>::new(Signedness::Signed, 16, 2).unwrap();
    let encoder = SignMagnitude::new(shape);
    let decoder = SignMagnitude::new(shape);
    let value = shape.raw(-2).unwrap();

    let paired = Paired::try_new(&encoder, &decoder).unwrap();
    assert_eq!(paired.encode_size(), Ok(Size::Fixed(WIRE_BYTES)));
    assert_eq!(paired.decode_size(), Ok(Size::Fixed(WIRE_BYTES)));
    assert_eq!(
        (encoder.calls.writes.get(), decoder.calls.reads.get()),
        (0, 0)
    );

    assert_eq!(paired.encode(&value, WIRE_BYTES), Ok([0x80, 0x02].to_vec()));
    assert_eq!(paired.decode(&[0x80, 0x02], LIMITS), Ok(value));
    assert_eq!(
        (encoder.calls.writes.get(), decoder.calls.reads.get()),
        (1, 1)
    );

    let mismatch = Paired::<_, _, DecimalFixedPoint, u32>::try_new(&encoder, Number::<u32>::LE);
    assert_eq!(
        mismatch.err(),
        Some(PairError::FixedSizeMismatch {
            encoder: WIRE_BYTES,
            decoder: 4
        })
    );
    assert_eq!(
        (encoder.calls.writes.get(), decoder.calls.reads.get()),
        (1, 1)
    );
}

#[test]
fn mapped_callbacks_convert_checked_domain_values() {
    let shape = Shape::<10>::new(Signedness::Signed, 16, 2).unwrap();
    let writes = Cell::new(0);
    let reads = Cell::new(0);
    let mapped = cents_codec(shape, &writes, &reads);

    assert_eq!(mapped.encode_size(), Ok(Size::Fixed(WIRE_BYTES)));
    assert_eq!(mapped.decode_size(), Ok(Size::Fixed(WIRE_BYTES)));
    assert_eq!((writes.get(), reads.get()), (0, 0));

    assert_eq!(mapped.encoded_len(&Cents(SAMPLE_CENTS)), Ok(WIRE_BYTES));
    assert_eq!(writes.get(), 1);

    assert_eq!(
        mapped.encode(&Cents(SAMPLE_CENTS), WIRE_BYTES),
        Ok(SAMPLE_CENTS_WIRE.to_vec())
    );
    assert_eq!(writes.get(), 3);

    assert_eq!(
        mapped.decode(&SAMPLE_CENTS_WIRE, LIMITS),
        Ok(Cents(SAMPLE_CENTS))
    );
    assert_eq!(reads.get(), 1);

    let input = [PREFIX, SAMPLE_CENTS_WIRE[0], SAMPLE_CENTS_WIRE[1], SUFFIX];
    assert_eq!(
        mapped.decode_prefix(&input, 1, LIMITS),
        Ok((Cents(SAMPLE_CENTS), 3))
    );
    assert_eq!(reads.get(), 2);
}

#[test]
fn mapped_callbacks_reject_policy_errors_truncation_and_budget() {
    let shape = Shape::<10>::new(Signedness::Signed, 16, 2).unwrap();
    let writes = Cell::new(0);
    let reads = Cell::new(0);
    let mapped = cents_codec(shape, &writes, &reads);

    let mut trailing = SAMPLE_CENTS_WIRE.to_vec();
    trailing.push(SUFFIX);
    assert_eq!(
        mapped.decode(&trailing, LIMITS),
        Err(CodecError::TrailingBytes)
    );
    assert_eq!(reads.get(), 1);

    assert_eq!(
        mapped.decode(&SAMPLE_CENTS_WIRE[..1], LIMITS),
        Err(CodecError::Truncated)
    );
    assert_eq!(reads.get(), 1);

    assert_eq!(
        mapped.encode(&Cents(OUT_OF_POLICY_CENTS), WIRE_BYTES),
        Err(CodecError::NumberOutOfRange)
    );
    assert_eq!(writes.get(), 1);

    assert_eq!(
        mapped.decode(&OUT_OF_POLICY_WIRE, LIMITS),
        Err(CodecError::NumberOutOfRange)
    );
    assert_eq!(reads.get(), 2);

    assert_eq!(
        mapped.decode(&SAMPLE_CENTS_WIRE, Limits::new(1, 0)),
        Err(CodecError::LimitExceeded)
    );
    assert_eq!(reads.get(), 2);

    let paired = Paired::<_, _, Cents, Cents>::try_new(&mapped, &mapped).unwrap();
    assert_eq!(paired.encode_size(), Ok(Size::Fixed(WIRE_BYTES)));
    assert_eq!((writes.get(), reads.get()), (1, 2));
}

#[test]
fn built_in_fixed_point_halves_pair_without_an_adapter() {
    let shape = Shape::<10>::new(Signedness::Signed, 16, 2).unwrap();
    let codec = shape.codec(Endian::Little).unwrap();
    let paired = Paired::<_, _, DecimalFixedPoint, DecimalFixedPoint>::try_new(codec, codec)
        .expect("built-in fixed width is declared");
    let value = shape.raw(-SAMPLE_CENTS).unwrap();
    let bytes = paired.encode(&value, WIRE_BYTES).unwrap();

    assert_eq!(bytes, [0x83, 0xff]);
    assert_eq!(paired.decode(&bytes, LIMITS), Ok(value));
    let wider = Shape::<10>::new(Signedness::Signed, 32, 2)
        .unwrap()
        .codec(Endian::Little)
        .unwrap();
    let mismatch = Paired::<_, _, DecimalFixedPoint, DecimalFixedPoint>::try_new(codec, wider);
    assert_eq!(
        mismatch.err(),
        Some(PairError::FixedSizeMismatch {
            encoder: WIRE_BYTES,
            decoder: 4,
        })
    );
}
