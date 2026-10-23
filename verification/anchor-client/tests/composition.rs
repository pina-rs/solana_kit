use anchor_client_contract_consumer::instruction;
use solana_kit_anchor::{Coder, Field, Idl, Limits, Primitive, Type, Value, Values, Variant};
use solana_kit_core::{AccountMeta, AccountRole};

const LIMITS: Limits = Limits::new(4096, 1024, 32);

#[test]
fn ordinary_consumer_builds_an_instruction_with_exact_duplicate_metadata() {
    let idl = Idl::parse(
        r#"{
        "address":"11111111111111111111111111111111",
        "instructions":[{"name":"close","discriminator":[9,8,7]}]
    }"#,
        LIMITS,
    )
    .unwrap();
    let meta = AccountMeta {
        address: idl.address.parse().unwrap(),
        role: AccountRole::ReadonlySigner,
    };
    let instruction = instruction(&idl, vec![meta, meta]).unwrap();

    assert_eq!(instruction.accounts, [meta, meta]);
    assert_eq!(instruction.data, [9, 8, 7]);
}

#[test]
fn public_type_spelling_and_deferred_field_parsing_are_usable() {
    let idl = Idl::parse(r#"{"instructions":[{"name":"x","args":["u8"]}]}"#, LIMITS).unwrap();
    let field: Field = idl.instructions["x"].args[0].clone();
    let reparsed = Field::parse(&field.raw_type, 0, LIMITS).unwrap();

    assert_eq!(reparsed.name, "field0");
    assert_eq!(field.parsed_type(LIMITS).unwrap().to_string(), "u8");
    assert_eq!(Primitive::U128.name(), "u128");
    assert_eq!(Type::Bytes.to_string(), "bytes");
    assert_eq!(Type::Defined("Thing".to_string()).to_string(), "Thing");

    let variant = Variant::parse(&field.raw_type, LIMITS);
    assert!(variant.is_err());
}

#[test]
fn empty_explicit_prefix_and_nested_options_have_unambiguous_values() {
    let idl = Idl::parse(r#"{"instructions":[{"name":"x"}]}"#, LIMITS).unwrap();
    let coder = Coder::new(&idl, LIMITS);

    assert!(
        coder
            .encode_instruction("x", &Values::new())
            .unwrap()
            .is_empty()
    );

    let ty = Type::Option(Box::new(Type::Option(Box::new(Type::Primitive(
        Primitive::Uint8,
    )))));
    let codec = coder.codec_for(&ty).unwrap();
    let value = Value::Option(Some(Box::new(Value::Option(None))));

    assert_eq!(codec.encode(&value).unwrap(), [1, 0]);
    assert_eq!(codec.decode(&[1, 0]).unwrap(), value);
    assert!(codec.decode(&[2]).is_err());
}
