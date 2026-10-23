mod support;

use solana_kit_anchor::{Coder, Error, Idl, Limits, Primitive, Type, Value};
use solana_kit_core::{AccountMeta, AccountRole, Instruction};
use support::{ADDRESS, LIMITS, bytes, fields, mini};

const INITIAL_AMOUNT: u128 = 12345;
const NEGATIVE_DELTA: i128 = -5;
const KITCHEN_AMOUNT: u128 = 16960;
const SIGNED_DELTA: i128 = -1234;
const SINGLE_FLOAT: f32 = 2.5;
const DOUBLE_FLOAT: f64 = -1.25;
const PRESENT_OPTION: u128 = 42;

const TUPLE_FIRST: u128 = 8;
const TUPLE_SECOND: u128 = 9;
const ACCOUNT_COUNT: u128 = 999;
const SMALL_BYTES: usize = 32;
const SMALL_ITEMS: usize = 8;
const SMALL_DEPTH: usize = 8;

fn initial() -> solana_kit_anchor::Values {
    fields([
        ("authority", Value::Address(ADDRESS.parse().unwrap())),
        ("amount", Value::Unsigned(INITIAL_AMOUNT)),
        ("name", Value::String("hello".to_string())),
        (
            "tags",
            Value::Sequence([7, 9, 11].map(Value::Unsigned).to_vec()),
        ),
        (
            "key",
            Value::Sequence([1, 2, 3, 4].map(Value::Unsigned).to_vec()),
        ),
        (
            "maybe",
            Value::Option(Some(Box::new(Value::Signed(NEGATIVE_DELTA)))),
        ),
    ])
}

#[test]
fn instruction_matches_original_literal_and_fresh_dart() {
    let idl = mini();

    let coder = Coder::new(&idl, LIMITS);

    let encoded = coder.encode_instruction("initialize", &initial()).unwrap();

    let literal = "afaf6d1f0d989bedda075cb2ff5ec6817613de530b692a8735477769da47430cbd8154335c4a832739300000000000000500000068656c6c6f0300000007000000090000000b0000000102030401fbffffffffffffff";

    assert_eq!(encoded, bytes(literal));

    assert_eq!(encoded, bytes(include_str!("fixtures/initialize.hex")));

    let decoded = coder.decode_instruction("initialize", &encoded).unwrap();

    assert_eq!(decoded.value, initial());

    assert_eq!(decoded.consumed, encoded.len());
}

#[test]
fn instructions_keep_explicit_zero_prefix_and_report_consumption() {
    let idl = mini();

    let coder = Coder::new(&idl, LIMITS);

    assert_eq!(
        coder.encode_instruction("close", &fields([])).unwrap(),
        vec![0; 8]
    );

    assert_eq!(
        coder.decode_instruction("close", &[0; 9]).unwrap().consumed,
        8
    );

    let error = coder.decode_instruction("close", &[0; 7]).unwrap_err();
    assert_eq!(
        error,
        Error::Discriminator {
            expected: vec![0; 8],
            found: vec![0; 7]
        }
    );

    assert!(matches!(
        coder.decode_instruction("close", &[1; 8]),
        Err(Error::Discriminator { .. })
    ));
}

#[test]
fn missing_names_fields_and_ranges_keep_error_attribution() {
    let idl = mini();

    let coder = Coder::new(&idl, LIMITS);

    assert!(matches!(
        coder.encode_instruction("absent", &fields([])),
        Err(Error::Unknown(_))
    ));

    assert!(coder.decode_instruction("absent", &[]).is_err());

    let error = coder
        .encode_instruction("initialize", &fields([]))
        .unwrap_err();
    let Error::Field { name, .. } = error else {
        panic!("missing field attribution")
    };
    assert_eq!(name, "authority");

    let mut values = initial();
    values.insert("amount".to_string(), Value::Unsigned(u128::MAX));

    let error = coder.encode_instruction("initialize", &values).unwrap_err();

    assert!(matches!(&error, Error::Field { name, .. } if name == "amount"));

    assert!(error.to_string().contains("amount"));

    assert!(std::error::Error::source(&error).is_some());
}

fn kitchen_values() -> solana_kit_anchor::Values {
    fields([
        ("flag", Value::Bool(true)),
        ("u8", Value::Unsigned(255)),
        ("u16", Value::Unsigned(0x1234)),
        ("u32", Value::Unsigned(0x12345)),
        ("u64", Value::Unsigned(KITCHEN_AMOUNT)),
        ("u128", Value::Unsigned(u128::MAX)),
        ("i8", Value::Signed(i8::MIN.into())),
        ("i16", Value::Signed(i16::MIN.into())),
        ("i32", Value::Signed(SIGNED_DELTA)),
        ("i64", Value::Signed(1)),
        ("i128", Value::Signed(-1)),
        ("f32", Value::F32(SINGLE_FLOAT)),
        ("f64", Value::F64(DOUBLE_FLOAT)),
        ("text", Value::String("anchor".to_string())),
        ("wallet", Value::Address(ADDRESS.parse().unwrap())),
        (
            "nums",
            Value::Sequence([1, 2, 3].map(Value::Unsigned).to_vec()),
        ),
        (
            "fixed",
            Value::Sequence([9, 8, 7].map(Value::Unsigned).to_vec()),
        ),
        (
            "some",
            Value::Option(Some(Box::new(Value::Unsigned(PRESENT_OPTION)))),
        ),
        ("none", Value::Option(None)),
        ("chunk", Value::Bytes(vec![1, 2, 3])),
    ])
}

#[test]
fn all_fifteen_scalars_and_containers_match_original_and_fresh_vectors() {
    let idl = Idl::parse(include_str!("fixtures/kitchen.json"), LIMITS).unwrap();

    let coder = Coder::new(&idl, LIMITS);

    let values = kitchen_values();

    let actual = coder.encode_instruction("kitchen_sink", &values).unwrap();

    let literal = "5dfa6bc423451aca01ff3412452301004042000000000000ffffffffffffffffffffffffffffffff8000802efbffff0100000000000000ffffffffffffffffffffffffffffffff00002040000000000000f4bf06000000616e63686f72da075cb2ff5ec6817613de530b692a8735477769da47430cbd8154335c4a832703000000010000000200000003000000090807012a0003000000010203";

    assert_eq!(actual, bytes(literal));

    assert_eq!(actual, bytes(include_str!("fixtures/kitchen.hex")));

    assert_eq!(
        coder
            .decode_instruction("kitchen_sink", &actual)
            .unwrap()
            .value,
        values
    );

    for end in 0..actual.len() {
        assert!(
            coder
                .decode_instruction("kitchen_sink", &actual[..end])
                .is_err(),
            "truncated at {end}"
        );
    }
}

fn account(variant: &str) -> Value {
    let variant_fields = match variant {
        "Running" => fields([
            ("step", Value::Unsigned(3)),
            ("label", Value::String("go".to_string())),
        ]),
        "Tuple" => fields([
            ("field0", Value::Unsigned(TUPLE_FIRST)),
            ("field1", Value::Unsigned(TUPLE_SECOND)),
        ]),
        _ => fields([]),
    };

    Value::Struct(fields([
        ("authority", Value::Address(ADDRESS.parse().unwrap())),
        ("count", Value::Unsigned(ACCOUNT_COUNT)),
        (
            "state",
            Value::Enum {
                variant: variant.to_string(),
                fields: variant_fields,
            },
        ),
    ]))
}

#[test]
fn accounts_and_all_enum_shapes_match_fresh_source_bytes() {
    let idl = mini();

    let coder = Coder::new(&idl, LIMITS);

    for row in include_str!("fixtures/accounts.tsv").lines() {
        let (variant, expected) = row.split_once('\t').unwrap();
        let input = account(variant);
        let encoded = coder.encode_account("Counter", &input).unwrap();
        assert_eq!(encoded, bytes(expected));
        let mut padded = encoded.clone();
        padded.extend_from_slice(&[55, 99]);
        let decoded = coder.decode_account("Counter", &padded).unwrap();
        assert_eq!(decoded.data, input);
        assert_eq!(decoded.consumed, encoded.len());
        assert_eq!(decoded.discriminator, vec![246, 28, 6, 87, 251, 45, 50, 42]);
    }
}

#[test]
fn account_corruption_and_definition_failures_are_visible() {
    let mut idl = mini();

    let encoded = Coder::new(&idl, LIMITS)
        .encode_account("Counter", &account("Unit"))
        .unwrap();

    let mut corrupted = encoded;
    corrupted[5] ^= 255;

    assert!(matches!(
        Coder::new(&idl, LIMITS).decode_account("Counter", &corrupted),
        Err(Error::Discriminator { .. })
    ));

    assert!(
        Coder::new(&idl, LIMITS)
            .encode_account("Missing", &account("Unit"))
            .is_err()
    );

    assert!(
        Coder::new(&idl, LIMITS)
            .decode_account("Missing", &[])
            .is_err()
    );
    idl.types.remove("Counter");

    assert!(
        Coder::new(&idl, LIMITS)
            .decode_account("Counter", &[246, 28, 6, 87, 251, 45, 50, 42])
            .is_err()
    );
}

#[test]
fn codec_for_composes_exact_and_prefix_reads() {
    let idl = mini();

    let coder = Coder::new(&idl, LIMITS);

    let codec = coder.codec_for(&Type::Bytes).unwrap();

    let input = Value::Bytes(vec![1, 2, 3]);

    assert_eq!(codec.encode(&input).unwrap(), vec![3, 0, 0, 0, 1, 2, 3]);

    assert_eq!(codec.decode(&[3, 0, 0, 0, 1, 2, 3]).unwrap(), input);

    assert!(codec.decode(&[3, 0, 0, 0, 1, 2, 3, 4]).is_err());

    assert_eq!(
        codec
            .decode_prefix(&[3, 0, 0, 0, 1, 2, 3, 4])
            .unwrap()
            .consumed,
        7
    );

    assert!(
        coder
            .codec_for(&Type::Defined("Missing".to_string()))
            .is_err()
    );
}

#[test]
fn scalar_and_container_malformed_inputs_never_succeed() {
    let idl = mini();

    let coder = Coder::new(&idl, LIMITS);

    let boolean = Type::Primitive(Primitive::Bool);

    assert!(coder.codec_for(&boolean).unwrap().decode(&[2]).is_err());

    let option = Type::Option(Box::new(Type::Primitive(Primitive::Uint8)));

    assert!(coder.codec_for(&option).unwrap().decode(&[2, 0]).is_err());

    let utf8 = Type::Primitive(Primitive::String);

    assert!(
        coder
            .codec_for(&utf8)
            .unwrap()
            .decode(&[1, 0, 0, 0, 255])
            .is_err()
    );

    let array = Type::Array(Box::new(Type::Primitive(Primitive::Uint8)), 2);

    assert!(
        coder
            .codec_for(&array)
            .unwrap()
            .encode(&Value::Sequence(vec![Value::Unsigned(1)]))
            .is_err()
    );

    assert!(
        coder
            .codec_for(&boolean)
            .unwrap()
            .encode(&Value::Unsigned(1))
            .is_err()
    );
}

#[test]
fn caller_budgets_reject_nested_zero_width_and_oversized_inputs() {
    let idl = mini();

    let coder = Coder::new(&idl, Limits::new(SMALL_BYTES, SMALL_ITEMS, SMALL_DEPTH));

    let tiny = Coder::new(&idl, Limits::new(1, SMALL_ITEMS, SMALL_DEPTH));
    assert_eq!(tiny.decode_instruction("close", &[]), Err(Error::Limit));

    let empty_struct = Type::Defined("Legacy".to_string());

    let sequence = Type::Vec(Box::new(empty_struct));

    assert!(
        coder
            .codec_for(&sequence)
            .unwrap()
            .decode(&[255, 255, 255, 255])
            .is_err()
    );

    assert!(
        coder
            .codec_for(&Type::Bytes)
            .unwrap()
            .decode(&[255, 255, 255, 255])
            .is_err()
    );

    assert!(coder.encode_instruction("initialize", &initial()).is_err());

    assert!(coder.decode_instruction("close", &[0; 33]).is_err());

    let deep = Type::Option(Box::new(Type::Option(Box::new(Type::Bytes))));

    assert!(
        Coder::new(&idl, Limits::new(SMALL_BYTES, SMALL_ITEMS, 1))
            .codec_for(&deep)
            .is_err()
    );
}

#[test]
fn kit_instruction_composition_preserves_duplicate_positions_and_bytes() {
    let idl = mini();

    let program = ADDRESS.parse().unwrap();

    let meta = AccountMeta {
        address: program,
        role: AccountRole::WritableSigner,
    };

    let data = Coder::new(&idl, LIMITS)
        .encode_instruction("initialize", &initial())
        .unwrap();

    let instruction = Instruction {
        program_address: program,
        accounts: vec![meta, meta],
        data: data.clone(),
    };

    assert_eq!(instruction.accounts, vec![meta, meta]);

    assert_eq!(instruction.data, data);

    assert_eq!(instruction.program_address, program);
}

#[test]
fn full_width_signed_and_unsigned_extremes_match_literal_bytes() {
    let idl = mini();
    let coder = Coder::new(&idl, LIMITS);
    let cases = [
        (
            Primitive::I128,
            Value::Signed(i128::MIN),
            "00000000000000000000000000000080",
        ),
        (
            Primitive::I128,
            Value::Signed(i128::MAX),
            "ffffffffffffffffffffffffffffff7f",
        ),
        (
            Primitive::U128,
            Value::Unsigned(u128::MAX),
            "ffffffffffffffffffffffffffffffff",
        ),
        (
            Primitive::I64,
            Value::Signed(i64::MIN.into()),
            "0000000000000080",
        ),
        (
            Primitive::U64,
            Value::Unsigned(u64::MAX.into()),
            "ffffffffffffffff",
        ),
    ];

    for (primitive, input, expected) in cases {
        let ty = Type::Primitive(primitive);
        let codec = coder.codec_for(&ty).unwrap();

        assert_eq!(codec.encode(&input).unwrap(), bytes(expected));
        assert_eq!(codec.decode(&bytes(expected)).unwrap(), input);
    }

    let small = Type::Primitive(Primitive::U16);
    let overflow = Value::Unsigned(u128::from(u16::MAX) + 1);

    assert!(coder.codec_for(&small).unwrap().encode(&overflow).is_err());
}
