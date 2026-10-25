//! Every schema tag has independent nonzero bytes, including full-width integers.
use solana_kit_attestation_service::*;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits},
};

const LIMITS: Limits = Limits::new(4096, 4096);
const BYTE_BUDGET: usize = 4096;
const I8_TAG: u8 = 5;
const CHAR_TAG: u8 = 11;
const STRING_TAG: u8 = 12;
const BYTE_VECTOR_TAG: u8 = 13;
const STRING_TEST_BYTE_BUDGET: usize = 100;
const STRING_TEST_ITEM_LIMIT: usize = 7;

const U8_SAMPLE_VALUE: u8 = 7;

type SchemaCase = (u8, FieldValue, Vec<u8>);

fn schema(tag: u8) -> Schema {
    Schema {
        credential: Address::from_bytes([31; 32]),
        name: vec![0xff],
        description: vec![0xfe],
        layout: vec![tag],
        field_names: vec![1, 0, 0, 0, b'x'],
        is_paused: false,
        version: 1,
    }
}

fn data(value: FieldValue) -> AttestationData {
    vec![AttestationField {
        name: "x".into(),
        value,
    }]
}

fn scalar_cases() -> Vec<SchemaCase> {
    let negative_i32 = vec![0xfe, 0xff, 0xff, 0xff];
    let fire_character = vec![0x25, 0xf5, 1, 0];
    vec![
        (0, FieldValue::U8(0xa5), vec![0xa5]),
        (1, FieldValue::U16(0x9234), vec![0x34, 0x92]),
        (2, FieldValue::U32(0x8003_0201), vec![1, 2, 3, 0x80]),
        (
            3,
            FieldValue::U64(0x8007_0605_0403_0201),
            vec![1, 2, 3, 4, 5, 6, 7, 0x80],
        ),
        (
            4,
            FieldValue::U128(0x800f_0e0d_0c0b_0a09_0807_0605_0403_0201),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0x80],
        ),
        (I8_TAG, FieldValue::I8(-2), vec![0xfe]),
        (6, FieldValue::I16(-2), vec![0xfe, 0xff]),
        (7, FieldValue::I32(-2), negative_i32),
        (
            8,
            FieldValue::I64(-2),
            vec![0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        ),
        (
            9,
            FieldValue::I128(-2),
            vec![
                0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff,
            ],
        ),
        (10, FieldValue::Bool(true), vec![1]),
        (CHAR_TAG, FieldValue::Char('🔥'), fire_character),
        (
            STRING_TAG,
            FieldValue::String("é".into()),
            vec![2, 0, 0, 0, 0xc3, 0xa9],
        ),
    ]
}

fn vector_cases() -> Vec<SchemaCase> {
    let negative_i8 = vec![1, 0, 0, 0, 0xfe];
    vec![
        (
            13,
            FieldValue::VecU8(vec![1, 0xff]),
            vec![2, 0, 0, 0, 1, 0xff],
        ),
        (
            14,
            FieldValue::VecU16(vec![0x9234]),
            vec![1, 0, 0, 0, 0x34, 0x92],
        ),
        (
            15,
            FieldValue::VecU32(vec![0x8003_0201]),
            vec![1, 0, 0, 0, 1, 2, 3, 0x80],
        ),
        (
            16,
            FieldValue::VecU64(vec![0x8007_0605_0403_0201]),
            vec![1, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 0x80],
        ),
        (
            17,
            FieldValue::VecU128(vec![0x800f_0e0d_0c0b_0a09_0807_0605_0403_0201]),
            vec![
                1, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0x80,
            ],
        ),
        (18, FieldValue::VecI8(vec![-2]), negative_i8),
        (
            19,
            FieldValue::VecI16(vec![-2]),
            vec![1, 0, 0, 0, 0xfe, 0xff],
        ),
        (
            20,
            FieldValue::VecI32(vec![-2]),
            vec![1, 0, 0, 0, 0xfe, 0xff, 0xff, 0xff],
        ),
        (
            21,
            FieldValue::VecI64(vec![-2]),
            vec![1, 0, 0, 0, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        ),
        (
            22,
            FieldValue::VecI128(vec![-2]),
            vec![
                1, 0, 0, 0, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff,
            ],
        ),
        (
            23,
            FieldValue::VecBool(vec![true, false]),
            vec![2, 0, 0, 0, 1, 0],
        ),
        (
            24,
            FieldValue::VecChar(vec!['A', '🔥']),
            vec![2, 0, 0, 0, 65, 0, 0, 0, 0x25, 0xf5, 1, 0],
        ),
        (
            25,
            FieldValue::VecString(vec!["x".into(), "é".into()]),
            vec![2, 0, 0, 0, 1, 0, 0, 0, b'x', 2, 0, 0, 0, 0xc3, 0xa9],
        ),
    ]
}

#[cfg_attr(test, test)]
pub fn every_schema_value_has_exact_literal_bytes() {
    let cases = scalar_cases().into_iter().chain(vector_cases());

    for (tag, value, bytes) in cases {
        let codec = AttestationDataCodec::from_schema(&schema(tag), LIMITS).unwrap();
        let expected = data(value);
        assert_eq!(
            codec.encode(expected.as_slice(), BYTE_BUDGET).unwrap(),
            bytes,
            "tag {tag}"
        );
        assert_eq!(codec.decode(&bytes, LIMITS).unwrap(), expected, "tag {tag}");
        assert_eq!(
            SchemaDataTypeCodec
                .encode(&codec.fields()[0].data_type, 1)
                .unwrap(),
            [tag]
        );

        for end in 0..bytes.len() {
            assert!(
                codec.decode(&bytes[..end], LIMITS).is_err(),
                "tag {tag} at {end}"
            );
        }

        let mut trailing = bytes.clone();
        trailing.push(0);
        assert_eq!(codec.decode(&trailing, LIMITS), Err(Error::TrailingBytes));
        assert!(
            codec
                .decode(&bytes, Limits::new(bytes.len() - 1, BYTE_BUDGET))
                .is_err()
        );
        assert!(codec.decode(&bytes, Limits::new(BYTE_BUDGET, 0)).is_err());
    }
}

#[cfg_attr(test, test)]
pub fn schema_names_tags_and_exact_keys_are_checked() {
    let model = schema(0);
    let codec = AttestationDataCodec::from_schema(&model, LIMITS).unwrap();
    assert!(codec.encode(&[], 10).is_err());
    assert!(
        codec
            .encode(data(FieldValue::U16(3)).as_slice(), 10)
            .is_err()
    );
    let mut unknown = data(FieldValue::U8(3));
    unknown[0].name = "other".into();
    assert!(codec.encode(unknown.as_slice(), 10).is_err());
    let mut duplicate = data(FieldValue::U8(3));

    duplicate.push(duplicate[0].clone());
    assert!(codec.encode(duplicate.as_slice(), 10).is_err());

    let mut invalid = model.clone();
    invalid.field_names[4] = 0xff;
    assert!(AttestationDataCodec::from_schema(&invalid, LIMITS).is_err());
    invalid = model.clone();
    invalid.layout.push(1);
    invalid.field_names.extend(&model.field_names);
    assert!(AttestationDataCodec::from_schema(&invalid, LIMITS).is_err());
    invalid = model.clone();

    invalid.layout.push(1);
    assert!(AttestationDataCodec::from_schema(&invalid, LIMITS).is_err());

    for tag in [26, 255] {
        assert!(AttestationDataCodec::from_schema(&schema(tag), LIMITS).is_err());
        assert!(decode_schema_layout(&[tag], LIMITS).is_err());
    }

    assert!(AttestationDataCodec::from_schema(&model, Limits::new(5, 4096)).is_err());
    assert!(AttestationDataCodec::from_schema(&model, Limits::new(4096, 0)).is_err());
    assert!(decode_schema_field_names(&[8, 0, 0, 0, b'x'], LIMITS).is_err());
    assert!(decode_schema_field_names(&[1, 0, 0, 0, 0xff], LIMITS).is_err());
    assert_eq!(
        decode_schema_field_names(&model.field_names, LIMITS).unwrap(),
        ["x"]
    );
    assert_eq!(decode_schema_text(&[0xff, 0xfe]), "\u{fffd}\u{fffd}");
}

#[cfg_attr(test, test)]
pub fn malformed_unicode_binary_strings_and_vector_budgets() {
    let character = AttestationDataCodec::from_schema(&schema(CHAR_TAG), LIMITS).unwrap();

    for bytes in [
        [0, 0xd8, 0, 0],
        [0xff, 0xdf, 0, 0],
        [0, 0, 0x11, 0],
        [0xff; 4],
    ] {
        assert!(character.decode(&bytes, LIMITS).is_err());
    }

    let boolean = AttestationDataCodec::from_schema(&schema(10), LIMITS).unwrap();
    assert!(boolean.decode(&[2], LIMITS).is_err());
    let strings = AttestationDataCodec::from_schema(&schema(STRING_TAG), LIMITS).unwrap();
    let binary = [2, 0, 0, 0, 0xff, 0xfe];
    let displayed = strings.decode(&binary, LIMITS).unwrap();
    assert_eq!(displayed, data(FieldValue::String("0xfffe".into())));
    assert_eq!(
        strings.encode(displayed.as_slice(), 100).unwrap(),
        [6, 0, 0, 0, b'0', b'x', b'f', b'f', b'f', b'e']
    );
    assert!(
        strings
            .decode(
                &binary,
                Limits::new(STRING_TEST_BYTE_BUDGET, STRING_TEST_ITEM_LIMIT)
            )
            .is_err()
    );

    let vector = AttestationDataCodec::from_schema(&schema(BYTE_VECTOR_TAG), LIMITS).unwrap();
    assert!(vector.decode(&[0xff; 4], LIMITS).is_err());
    assert_eq!(
        vector.decode(&[0, 0, 0, 0], LIMITS).unwrap(),
        data(FieldValue::VecU8(vec![]))
    );
}

#[cfg_attr(test, test)]
pub fn schema_wire_order_is_independent_of_named_input_order() {
    let mut model = schema(0);
    model.layout.push(STRING_TAG);
    model.field_names.extend([1, 0, 0, 0, b'y']);
    let codec = AttestationDataCodec::from_schema(&model, LIMITS).unwrap();
    let values = vec![
        AttestationField {
            name: "y".into(),
            value: FieldValue::String("hi".into()),
        },
        AttestationField {
            name: "x".into(),
            value: FieldValue::U8(U8_SAMPLE_VALUE),
        },
    ];
    assert_eq!(
        codec.encode(values.as_slice(), 100).unwrap(),
        [7, 2, 0, 0, 0, b'h', b'i']
    );
    let decoded = codec.decode(&[7, 2, 0, 0, 0, b'h', b'i'], LIMITS).unwrap();
    assert_eq!(decoded[0].name, "x");

    assert_eq!(decoded[1].name, "y");
}
