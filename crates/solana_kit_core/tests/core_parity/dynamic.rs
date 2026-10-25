//! Runtime schemas preserve heterogeneous values through an explicit Rust enum.

use alloc::vec;
use solana_kit_core::{
    codecs::{
        Bool, Bytes, Decoder, Encoder, Error, Limits, Mapped, Number, Reader, RecordDecoder,
        RecordEncoder, Size, SizePrefix, TupleDecoder, TupleEncoder, Writer,
    },
    collections::OrderedMap,
};

#[derive(Debug, Eq, PartialEq)]
enum Field {
    Count(u16),
    Flag(bool),
}

fn count(value: &Field) -> Result<u16, Error> {
    match value {
        Field::Count(value) => Ok(*value),
        _ => Err(Error::InvalidValue),
    }
}

fn flag(value: &Field) -> Result<bool, Error> {
    match value {
        Field::Flag(value) => Ok(*value),
        _ => Err(Error::InvalidValue),
    }
}

pub fn dynamic_tuples() {
    let count = Mapped::new(Number::<u16>::LE, count, |value| Ok(Field::Count(value)));
    let flag = Mapped::new(Bool, flag, |value| Ok(Field::Flag(value)));
    let encoder = TupleEncoder {
        items: &[&count, &flag],
    };
    let decoder = TupleDecoder {
        items: &[&count, &flag],
    };
    let value = [Field::Count(0x1234), Field::Flag(true)];

    assert_eq!(encoder.encode(&value, 3).unwrap(), [0x34, 0x12, 1]);
    assert_eq!(
        decoder.decode(&[0x34, 0x12, 1], Limits::new(3, 2)),
        Ok(vec![Field::Count(0x1234), Field::Flag(true)])
    );
    assert_eq!(encoder.encode_size(), Ok(Size::Fixed(3)));
    assert_eq!(decoder.decode_size(), Ok(Size::Fixed(3)));

    assert_eq!(encoder.encode(&value[..1], 3), Err(Error::InvalidValue));
    assert_eq!(
        decoder.decode(&[0x34, 0x12, 2], Limits::new(3, 2)),
        Err(Error::InvalidBoolean { value: 2 })
    );
    assert_eq!(
        decoder.decode(&[0x34, 0x12, 1], Limits::new(3, 1)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        decoder.decode(&[0x34], Limits::new(3, 2)),
        Err(Error::Truncated)
    );
    let empty: TupleDecoder<'_, Field> = TupleDecoder { items: &[] };
    assert_eq!(empty.decode(&[], Limits::new(0, 0)), Ok(vec![]));
    assert_eq!(empty.decode_size(), Ok(Size::Fixed(0)));

    declaration_overflow();
}

struct Declared(Size);

impl Encoder<u8> for Declared {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(self.0)
    }

    fn encoded_len(&self, _: &u8) -> Result<usize, Error> {
        Err(Error::InvalidValue)
    }

    fn write(&self, _: &u8, _: &mut Writer<'_>) -> Result<(), Error> {
        Err(Error::InvalidValue)
    }
}

impl Decoder<u8> for Declared {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(self.0)
    }

    fn read(&self, _: &mut Reader<'_>) -> Result<u8, Error> {
        Err(Error::InvalidValue)
    }
}

fn declaration_overflow() {
    let unknown = Declared(Size::Unknown);
    let maximum = Declared(Size::Fixed(usize::MAX));
    let one = Declared(Size::Fixed(1));
    let encoder = TupleEncoder {
        items: &[&unknown, &maximum, &one],
    };
    let decoder = TupleDecoder {
        items: &[&unknown, &maximum, &one],
    };
    assert_eq!(encoder.encode_size(), Err(Error::Overflow));
    assert_eq!(decoder.decode_size(), Err(Error::Overflow));

    let encoders: [(&str, &dyn Encoder<u8>); 3] = [("a", &unknown), ("b", &maximum), ("c", &one)];
    let decoders: [(&str, &dyn Decoder<u8>); 3] = [("a", &unknown), ("b", &maximum), ("c", &one)];
    assert_eq!(
        RecordEncoder::try_new(&encoders).unwrap().encode_size(),
        Err(Error::Overflow)
    );
    assert_eq!(
        RecordDecoder::try_new(&decoders).unwrap().decode_size(),
        Err(Error::Overflow)
    );
}

pub fn dynamic_records() {
    let count = Mapped::new(Number::<u16>::LE, count, |value| Ok(Field::Count(value)));
    let flag = Mapped::new(Bool, flag, |value| Ok(Field::Flag(value)));
    let encoder_fields: [(&str, &dyn Encoder<Field>); 2] = [("count", &count), ("flag", &flag)];
    let decoder_fields: [(&str, &dyn Decoder<Field>); 2] = [("count", &count), ("flag", &flag)];
    let encoder = RecordEncoder::try_new(&encoder_fields).unwrap();
    let decoder = RecordDecoder::try_new(&decoder_fields).unwrap();
    let value = OrderedMap::from_entries(vec![
        ("flag", Field::Flag(true)),
        ("unused", Field::Flag(false)),
        ("count", Field::Count(0x1234)),
    ]);

    assert_eq!(encoder.encode(&value, 3).unwrap(), [0x34, 0x12, 1]);
    assert_eq!(
        decoder
            .decode(&[0x34, 0x12, 1], Limits::new(3, 2))
            .unwrap()
            .as_slice(),
        [("count", Field::Count(0x1234)), ("flag", Field::Flag(true))]
    );
    assert_eq!(encoder.encode_size(), Ok(Size::Fixed(3)));
    assert_eq!(decoder.decode_size(), Ok(Size::Fixed(3)));

    assert!(RecordEncoder::try_new(&[("count", &count), ("count", &count)]).is_err());
    assert!(RecordDecoder::try_new(&[("flag", &flag), ("flag", &flag)]).is_err());
    let missing = OrderedMap::from_entries(vec![("flag", Field::Flag(true))]);
    let mut output = [99; 3];
    assert_eq!(
        encoder.write(&missing, &mut Writer::new(&mut output)),
        Err(Error::InvalidValue)
    );
    assert_eq!(output, [99; 3]);
    assert_eq!(
        decoder.decode(&[0x34, 0x12, 1], Limits::new(3, 1)),
        Err(Error::LimitExceeded)
    );

    nested_record_budget();
}

fn nested_record_budget() {
    let bytes = SizePrefix(Bytes);
    let nested_fields: [(&str, &dyn Decoder<alloc::vec::Vec<u8>>); 2] =
        [("a", &bytes), ("b", &bytes)];
    let nested = RecordDecoder::try_new(&nested_fields).unwrap();
    let literal = [1, 0, 0, 0, 7, 2, 0, 0, 0, 8, 9];
    let complete_budget = Limits::new(literal.len(), 5);
    let insufficient_budget = Limits::new(literal.len(), 4);

    assert_eq!(
        nested.decode(&literal, complete_budget).unwrap().as_slice(),
        [("a", vec![7]), ("b", vec![8, 9])]
    );
    assert_eq!(
        nested.decode(&literal, insufficient_budget),
        Err(Error::LimitExceeded)
    );
}
