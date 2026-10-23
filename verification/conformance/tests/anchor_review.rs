use serde_json::json;
use solana_kit_anchor::{Coder, Idl, Limits, Primitive, Type, Value, Values};

const LIMITS: Limits = Limits::new(4096, 1024, 32);
const BYTE_LIMIT: usize = 4096;
const ITEM_LIMIT: usize = 1024;
const SIGN_BIT: u32 = 63;
const SAMPLE_BYTE: u128 = 7;

#[test]
fn unicode_strings_use_utf8_byte_lengths_and_reject_each_truncation() {
    let idl = Idl::parse("{}", LIMITS).unwrap();
    let coder = Coder::new(&idl, LIMITS);
    let ty = Type::Primitive(Primitive::String);
    let codec = coder.codec_for(&ty).unwrap();
    let value = Value::String("é🦀".to_string());
    let expected = [6, 0, 0, 0, 195, 169, 240, 159, 166, 128];

    assert_eq!(codec.encode(&value).unwrap(), expected);
    assert_eq!(codec.decode(&expected).unwrap(), value);

    for end in 0..expected.len() {
        assert!(codec.decode(&expected[..end]).is_err());
    }
}

#[test]
fn floating_signed_zero_and_infinity_preserve_exact_bits() {
    let idl = Idl::parse("{}", LIMITS).unwrap();
    let coder = Coder::new(&idl, LIMITS);
    let ty = Type::Primitive(Primitive::F64);
    let codec = coder.codec_for(&ty).unwrap();

    for bits in [
        0_u64,
        1_u64 << SIGN_BIT,
        0x7ff0_0000_0000_0000,
        0xfff0_0000_0000_0000,
    ] {
        let value = Value::F64(f64::from_bits(bits));
        let encoded = codec.encode(&value).unwrap();
        assert_eq!(encoded, bits.to_le_bytes());
        let Value::F64(decoded) = codec.decode(&encoded).unwrap() else {
            panic!("floating value changed kind")
        };
        assert_eq!(decoded.to_bits(), bits);
    }
}

#[test]
fn recursive_optional_definition_encodes_finite_instruction_but_codec_validation_is_bounded() {
    let idl = Idl::from_value(&json!({
        "instructions": [{"name": "x", "args": [{"name": "node", "type": {"defined": {"name": "Node"}}}]}],
        "types": [{"name": "Node", "type": {"kind": "struct", "fields": [
            {"name": "next", "type": {"option": {"defined": {"name": "Node"}}}}
        ]}}]
    }), LIMITS).unwrap();
    let coder = Coder::new(&idl, LIMITS);
    let fields = Values::from([("next".to_string(), Value::Option(None))]);
    let args = Values::from([("node".to_string(), Value::Struct(fields))]);

    assert_eq!(coder.encode_instruction("x", &args).unwrap(), [0]);
    assert_eq!(coder.decode_instruction("x", &[0]).unwrap().value, args);
    assert!(coder.codec_for(&Type::Defined("Node".to_string())).is_err());
}

#[test]
fn empty_vector_depth_admission_matches_decode_and_nonempty_values() {
    let idl = Idl::from_value(
        &json!({"instructions": [
            {"name": "x", "args": [{"name": "v", "type": {"vec": "u8"}}]}
        ]}),
        LIMITS,
    )
    .unwrap();
    let below = Coder::new(&idl, Limits::new(BYTE_LIMIT, ITEM_LIMIT, 1));
    let exact = Coder::new(&idl, Limits::new(BYTE_LIMIT, ITEM_LIMIT, 2));

    for values in [vec![], vec![Value::Unsigned(SAMPLE_BYTE)]] {
        let args = Values::from([("v".to_string(), Value::Sequence(values))]);
        let bytes = exact.encode_instruction("x", &args).unwrap();
        assert_eq!(exact.decode_instruction("x", &bytes).unwrap().value, args);
        assert!(below.encode_instruction("x", &args).is_err());
        assert!(below.decode_instruction("x", &bytes).is_err());
    }
}

#[test]
fn zero_length_fixed_array_respects_account_depth_boundary() {
    let idl = Idl::from_value(
        &json!({
            "accounts": [{"name": "Item"}],
            "types": [{"name": "Item", "type": {"kind": "struct", "fields": [
                {"name": "v", "type": {"array": ["u8", 0]}}
            ]}}]
        }),
        LIMITS,
    )
    .unwrap();
    let value = Value::Struct(Values::from([("v".to_string(), Value::Sequence(vec![]))]));
    let below = Coder::new(&idl, Limits::new(BYTE_LIMIT, ITEM_LIMIT, 2));
    let exact = Coder::new(&idl, Limits::new(BYTE_LIMIT, ITEM_LIMIT, 3));

    assert_eq!(exact.encode_account("Item", &value).unwrap(), [0_u8; 0]);
    assert_eq!(exact.decode_account("Item", &[]).unwrap().data, value);
    assert!(below.encode_account("Item", &value).is_err());
    assert!(below.decode_account("Item", &[]).is_err());
}
