//! Complete original Anchor test bodies, adapted only for host-neutral reporting.
use alloc::{boxed::Box, string::ToString};
use serde_json::{Value as Json, json};
use solana_kit_anchor::{Definition, Error, Idl, Limits, Primitive, Type, type_name};

const LIMITS: Limits = Limits::new(1_000_000, 100_000, 64);
const WIDE_PREFIX_BYTES: usize = 8;
const NON_OBJECT: usize = 42;
const MALFORMED_ENTRY: usize = 13;
const INVALID_BYTE: usize = 256;

pub(super) fn entire_original_idl_parses_with_deferred_generics() -> Result<(), &'static str> {
    let idl = Idl::parse(
        include_str!("../../../../crates/solana_kit_anchor/tests/fixtures/source-idl.json"),
        LIMITS,
    )
    .unwrap();

    super::assert_eq!(idl.name, "idl");

    super::assert!(!idl.address.is_empty());

    super::assert!(!idl.instructions.is_empty());

    super::assert!(!idl.accounts.is_empty());

    super::assert!(!idl.events.is_empty());

    super::assert!(!idl.types.is_empty());

    super::assert!(idl.types.contains_key("NamedStruct"));

    super::assert!(!idl.types.contains_key("idl::NamedStruct"));

    super::assert_eq!(idl.errors[&500000].name, "SomeError");

    super::assert_eq!(
        idl.instructions["account_and_event_arg_and_field"].discriminator,
        solana_kit_anchor::instruction_discriminator("account_and_event_arg_and_field")
    );
    Ok(())
}

pub(super) fn scalar_compound_and_module_names_retain_source_contract() -> Result<(), &'static str>
{
    super::assert_eq!(type_name("idl::some_module::NamedStruct"), "NamedStruct");

    super::assert_eq!(type_name("Plain"), "Plain");

    super::assert_eq!(Type::parse(&json!("bytes"), LIMITS).unwrap(), Type::Bytes);

    super::assert_eq!(
        Type::parse(&json!({"bytes": {}}), LIMITS).unwrap(),
        Type::Bytes
    );

    super::assert_eq!(
        Type::parse(&json!({"vec": "u32"}), LIMITS).unwrap(),
        Type::Vec(Box::new(Type::Primitive(Primitive::U32)))
    );

    super::assert_eq!(
        Type::parse(&json!({"option": "u64", "prefix": 1}), LIMITS).unwrap(),
        Type::Option(Box::new(Type::Primitive(Primitive::U64)))
    );

    super::assert_eq!(
        Type::parse(&json!({"array": ["u8", "4"]}), LIMITS).unwrap(),
        Type::Array(Box::new(Type::Primitive(Primitive::Uint8)), 4)
    );

    super::assert_eq!(
        Type::parse(&json!({"defined": {"name": "a::Thing"}}), LIMITS).unwrap(),
        Type::Defined("Thing".to_string())
    );
    Ok(())
}

pub(super) fn every_unsupported_type_shape_is_an_error() -> Result<(), &'static str> {
    let nodes = [
        json!({"defined": {"name": "Box", "generics": []}}),
        json!("u4096"),
        json!({"wat": "u8"}),
        json!({"option": "u8", "prefix": WIDE_PREFIX_BYTES}),
        json!({"option": "u8", "prefix": "wide"}),
        json!({"array": ["u8", "many"]}),
        json!({"array": ["u8", -1]}),
        json!({"array": ["u8", 1.5]}),
        json!({"array": ["u8"]}),
        Json::Null,
    ];

    for node in nodes {
        super::assert!(Type::parse(&node, LIMITS).is_err(), "{node}");
    }

    Ok(())
}

pub(super) fn malformed_documents_entries_and_definition_bodies_are_rejected()
-> Result<(), &'static str> {
    let nodes = [
        json!(NON_OBJECT),
        json!({"instructions": [MALFORMED_ENTRY]}),
        json!({"instructions": [{"discriminator": [1]}]}),
        json!({"errors": [{"name": "NameOnly"}]}),
        json!({"types": [{"type": {"kind": "struct"}}]}),
        json!({"types": [{"name": "Bare"}]}),
        json!({"types": [{"name": "Alias", "type": {"kind": "alias"}}]}),
        json!({"types": [{"name": "Bare", "type": "u8"}]}),
        json!({"types": [{"name": "Bad", "type": {"kind": "enum", "variants": [NON_OBJECT]}}]}),
        json!({"instructions": [{"name": "x", "args": [{"name": "x", "type": null}]}]}),
    ];

    for node in nodes {
        super::assert!(Idl::from_value(&node, LIMITS).is_err(), "{node}");
    }

    super::assert!(Idl::parse("{", LIMITS).is_err());
    Ok(())
}

pub(super) fn unsupported_field_nodes_fail_at_access_rather_than_document_parse()
-> Result<(), &'static str> {
    let idl = Idl::from_value(
        &json!({"instructions": [{"name": "x", "args": [
            {"name": "x", "type": {"defined": {"name": "Box", "generics": []}}},
            {"name": "nameOnly"}, MALFORMED_ENTRY,
        ]}]}),
        LIMITS,
    )
    .unwrap();

    let instruction = &idl.instructions["x"];

    super::assert_eq!(instruction.args[0].name, "x");

    super::assert_eq!(instruction.args[1].name, "field1");

    super::assert_eq!(instruction.args[2].name, "field2");

    for field in &instruction.args {
        super::assert!(field.parsed_type(LIMITS).is_err());
    }

    Ok(())
}

pub(super) fn bare_variant_field_shapes_and_empty_definitions_are_preserved()
-> Result<(), &'static str> {
    let idl = Idl::from_value(
        &json!({"types": [
            {"name": "Tuple", "type": {"kind": "struct", "fields": ["u8", "u16"]}},
            {"name": "Empty", "type": {"kind": "struct"}},
            {"name": "Choice", "type": {"kind": "enum", "variants": [
                {"fields": {"defined": {"name": "Empty"}}}
            ]}},
        ]}),
        LIMITS,
    )
    .unwrap();

    let Definition::Struct(fields) = &idl.types["Tuple"] else {
        panic!()
    };

    super::assert_eq!(
        (fields[0].name.as_str(), fields[1].name.as_str()),
        ("field0", "field1")
    );

    let Definition::Enum(variants) = &idl.types["Choice"] else {
        panic!()
    };

    super::assert_eq!(variants[0].name, "variant");

    super::assert_eq!(variants[0].fields[0].name, "field0");
    Ok(())
}

pub(super) fn missing_prefixes_and_duplicate_names_match_last_write_semantics()
-> Result<(), &'static str> {
    let idl = Idl::from_value(&json!({
        "instructions": [{"name": "x", "discriminator": [9]}, {"name": "x"}],
        "accounts": [{}],
        "events": [{"name": "z", "discriminator": [1]}, {"name": "a"}, {"name": "z", "discriminator": [2]}],
    }), LIMITS).unwrap();

    super::assert!(idl.name.is_empty());

    super::assert!(idl.address.is_empty());

    super::assert!(idl.instructions["x"].discriminator.is_empty());

    super::assert!(idl.accounts[""].discriminator.is_empty());

    super::assert_eq!(idl.events[0].name, "z");

    super::assert_eq!(idl.events[0].discriminator, [2]);

    super::assert_eq!(idl.events[1].name, "a");
    Ok(())
}

pub(super) fn idl_json_bytes_nodes_and_depth_are_caller_bounded() -> Result<(), &'static str> {
    super::assert_eq!(Idl::parse("{}", Limits::new(1, 4, 4)), Err(Error::Limit));

    super::assert_eq!(
        Idl::from_value(&json!({"instructions": []}), Limits::new(100, 1, 4)),
        Err(Error::Limit)
    );

    super::assert_eq!(
        Type::parse(&json!({"vec": {"vec": "u8"}}), Limits::new(100, 20, 1)),
        Err(Error::Limit)
    );

    super::assert_eq!(
        Type::parse(&json!({"array": ["u8", 99]}), Limits::new(100, 8, 8)),
        Err(Error::Limit)
    );

    super::assert!(
        Idl::from_value(
            &json!({"instructions": [{"name": "x", "discriminator": [INVALID_BYTE]}]}),
            LIMITS
        )
        .is_err()
    );

    super::assert!(
        Idl::from_value(
            &json!({"instructions": [{"name": "x", "discriminator": [-1]}]}),
            LIMITS
        )
        .is_err()
    );
    Ok(())
}

pub(super) fn wrong_string_field_types_are_rejected_without_empty_fallbacks()
-> Result<(), &'static str> {
    let documents = [
        json!({"address": true}),
        json!({"accounts": [{"name": true}]}),
        json!({"events": [{"name": true}]}),
        json!({"errors": [{"code": 1, "name": "Oops", "msg": true}]}),
        json!({"instructions": [{"name": "x", "args": [{"name": true, "type": "u8"}]}]}),
    ];

    for document in documents {
        super::assert!(Idl::from_value(&document, LIMITS).is_err());
    }

    Ok(())
}
