//! The native boundary and packed NPM consumer share this exact Codama AST.
use super::render_root;
use serde_json::{Value, json};

fn root() -> Value {
    let program: Value = serde_json::from_str(include_str!(
        "../../../../packages/codama-renderers-rust-solana-kit/test/numeric.json"
    ))
    .unwrap();

    json!({"kind":"rootNode","standard":"codama","version":"1.8.0","program":program})
}

#[test]
fn numeric_ast_reaches_the_public_emitter_without_the_verification_feature() {
    let files = render_root(&root()).unwrap();
    let source = &files
        .iter()
        .find(|file| file.path == "numeric_contracts.rs")
        .unwrap()
        .source;
    syn::parse_file(source).unwrap();

    for format in ["i32", "u128", "i128", "f32"] {
        for endian in ["LE", "BE"] {
            assert!(source.contains(&format!("Number::<{format}>::{endian}")));
        }
    }

    assert!(source.contains("Number::<u16>::BE"));
    assert!(source.contains("Number::<u64>::BE"));
    assert!(source.contains("Nullable"));
    assert!(source.contains("Boolean"));
    assert!(!source.contains("#[derive(Clone, Debug, Eq, PartialEq)]"));
}

#[test]
fn unknown_numeric_contracts_fail_before_any_files_are_returned() {
    for (key, value) in [("format", "u256"), ("endian", "middle")] {
        let mut input = root();
        input["program"]["definedTypes"][0]["type"][key] = json!(value);
        assert!(render_root(&input).is_err());
    }
}

#[test]
fn count_and_size_prefixes_reject_signed_and_floating_domains() {
    for format in ["i32", "f32", "u128"] {
        let mut count = root();
        count["program"]["definedTypes"][9]["type"]["count"]["prefix"]["format"] = json!(format);
        assert!(render_root(&count).is_err());

        let mut size = root();
        size["program"]["definedTypes"][10]["type"]["prefix"]["format"] = json!(format);
        assert!(render_root(&size).is_err());
    }
}

/// Non-symmetric literals expose a dropped byte order in either dispatch variant.
#[test]
fn instruction_tags_keep_both_declared_byte_orders() {
    for (endian, expected) in [
        ("be", ["1, 2, 3, 4", "1, 2, 3, 5"]),
        ("le", ["4, 3, 2, 1", "5, 3, 2, 1"]),
    ] {
        let mut input = root();

        for instruction in input["program"]["instructions"].as_array_mut().unwrap() {
            instruction["arguments"][0]["type"]["endian"] = json!(endian);
        }

        let files = render_root(&input).unwrap();
        let source = &files
            .iter()
            .find(|file| file.path == "numeric_contracts.rs")
            .unwrap()
            .source;

        for literal in expected {
            assert!(source.contains(literal));
        }
    }
}

#[test]
fn instruction_tags_reject_unsupported_formats_and_overflow() {
    for format in ["u16", "u64", "i32", "u128", "i128", "f32"] {
        let mut input = root();
        input["program"]["instructions"][0]["arguments"][0]["type"]["format"] = json!(format);
        assert!(render_root(&input).is_err());
    }

    let mut input = root();
    input["program"]["instructions"][0]["arguments"][0]["defaultValue"]["number"] =
        json!(u64::from(u32::MAX) + 1);
    assert!(render_root(&input).is_err());
}
