//! Complete original Anchor test bodies, adapted only for host-neutral reporting.
use super::consumer::instruction;
use alloc::{boxed::Box, string::ToString, vec};
use solana_kit_anchor::{Coder, Field, Idl, Limits, Primitive, Type, Value, Values, Variant};
use solana_kit_core::{AccountMeta, AccountRole};

const LIMITS: Limits = Limits::new(4096, 1024, 32);

pub(super) fn ordinary_consumer_builds_an_instruction_with_exact_duplicate_metadata()
-> Result<(), &'static str> {
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

    super::assert_eq!(instruction.accounts, [meta, meta]);
    super::assert_eq!(instruction.data, [9, 8, 7]);
    Ok(())
}

pub(super) fn public_type_spelling_and_deferred_field_parsing_are_usable()
-> Result<(), &'static str> {
    let idl = Idl::parse(r#"{"instructions":[{"name":"x","args":["u8"]}]}"#, LIMITS).unwrap();
    let field: Field = idl.instructions["x"].args[0].clone();
    let reparsed = Field::parse(&field.raw_type, 0, LIMITS).unwrap();

    super::assert_eq!(reparsed.name, "field0");
    super::assert_eq!(field.parsed_type(LIMITS).unwrap().to_string(), "u8");
    super::assert_eq!(Primitive::U128.name(), "u128");
    super::assert_eq!(Type::Bytes.to_string(), "bytes");
    super::assert_eq!(Type::Defined("Thing".to_string()).to_string(), "Thing");

    let variant = Variant::parse(&field.raw_type, LIMITS);
    super::assert!(variant.is_err());
    Ok(())
}

pub(super) fn empty_explicit_prefix_and_nested_options_have_unambiguous_values()
-> Result<(), &'static str> {
    let idl = Idl::parse(r#"{"instructions":[{"name":"x"}]}"#, LIMITS).unwrap();
    let coder = Coder::new(&idl, LIMITS);

    super::assert!(
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

    super::assert_eq!(codec.encode(&value).unwrap(), [1, 0]);
    super::assert_eq!(codec.decode(&[1, 0]).unwrap(), value);
    super::assert!(codec.decode(&[2]).is_err());
    Ok(())
}
