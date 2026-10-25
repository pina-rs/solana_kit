//! Independent literals exercise the real emitted codecs through an ordinary consumer.
use super::*;
use serde_json::json;

fn number(format: &str) -> Value {
    json!({"kind":"numberTypeNode","format":format,"endian":"le"})
}

fn text() -> Value {
    json!({"kind":"sizePrefixTypeNode","prefix":number("u32"),
        "type":{"kind":"stringTypeNode","encoding":"utf8"}})
}

fn option(item: Value) -> Value {
    json!({"kind":"optionTypeNode","fixed":false,"prefix":number("u8"),"item":item})
}

fn counted(item: Value) -> Value {
    json!({"kind":"arrayTypeNode","item":item,
        "count":{"kind":"prefixedCountNode","prefix":number("u32")}})
}

fn source(item: Value) -> Value {
    let mut root: Value =
        serde_json::from_str(include_str!("../../fixtures/compute-budget.json")).unwrap();
    root["version"] = json!("1.0.0");
    root["program"]["events"] = json!([]);
    root["program"]["definedTypes"] = json!([]);
    root["program"].as_object_mut().unwrap().remove("prefix");
    root["program"].as_object_mut().unwrap().remove("origin");
    let instructions = root["program"]["instructions"].as_array_mut().unwrap();

    instructions.truncate(1);
    instructions[0].as_object_mut().unwrap().remove("idlName");
    instructions[0].as_object_mut().unwrap().remove("display");
    let arguments = instructions[0]["arguments"].as_array_mut().unwrap();
    arguments.truncate(1);
    arguments.push(json!({"kind":"instructionArgumentNode","name":"payload","type":item}));
    root
}

fn policy() -> Policy<'static> {
    Policy {
        name: "computeBudget",
        address: "ComputeBudget111111111111111111111111111111"
            .parse()
            .unwrap(),
    }
}

fn execute(item: Value, label: &str, body: &str) {
    let directory = std::env::temp_dir().join(format!("kit-owned-{label}-{}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    let output = render_complete_program(&source(item), &policy()).unwrap();
    crate::generate::replace_source(&output, &directory.join("generated.rs")).unwrap();
    let consumer =
        format!("extern crate alloc; include!(\"generated.rs\"); fn main() {{ {body} }}");
    boundary::execute_source(&directory, &consumer);
}

#[test]
fn optional_owned_strings_encode_none_empty_and_utf8_payloads() {
    execute(
        option(text()),
        "text",
        r#"
        for (payload, expected) in [
            (None, vec![0,0]),
            (Some(String::new()), vec![0,1,0,0,0,0]),
            (Some(String::from("é")), vec![0,1,2,0,0,0,195,169]),
        ] {
            let value = RequestUnits { payload };
            assert_eq!(RequestUnitsCodec.encode(&value, expected.len()).unwrap(), expected);
            assert_eq!(RequestUnitsCodec.decode(&expected, Limits::new(expected.len(), 8)).unwrap(), value);
            assert!(RequestUnitsCodec.encode(&value, expected.len()-1).is_err());
        }
        assert!(RequestUnitsCodec.decode(&[0,2], Limits::new(2,8)).is_err());
        assert!(RequestUnitsCodec.decode(&[0,1,1,0,0,0,255], Limits::new(7,8)).is_err());
    "#,
    );
}

#[test]
fn nested_optional_strings_keep_both_presence_tags() {
    execute(
        option(option(text())),
        "nested-text",
        r#"
        for (payload, expected) in [
            (None, vec![0,0]),
            (Some(None), vec![0,1,0]),
            (Some(Some(String::new())), vec![0,1,1,0,0,0,0]),
            (Some(Some(String::from("a"))), vec![0,1,1,1,0,0,0,97]),
        ] {
            let value = RequestUnits { payload };
            assert_eq!(RequestUnitsCodec.encode(&value, expected.len()).unwrap(), expected);
            assert_eq!(RequestUnitsCodec.decode(&expected, Limits::new(expected.len(),8)).unwrap(), value);
        }
        assert!(RequestUnitsCodec.decode(&[0,1,2], Limits::new(3,8)).is_err());
    "#,
    );
}

#[test]
fn optional_owned_vectors_encode_none_empty_and_nonempty_payloads() {
    execute(
        option(counted(number("u16"))),
        "vector",
        r#"
        for (payload, expected) in [
            (None, vec![0,0]),
            (Some(vec![]), vec![0,1,0,0,0,0]),
            (Some(vec![513u16,65535]), vec![0,1,2,0,0,0,1,2,255,255]),
        ] {
            let value = RequestUnits { payload };
            assert_eq!(RequestUnitsCodec.encode(&value, expected.len()).unwrap(), expected);
            assert_eq!(RequestUnitsCodec.decode(&expected, Limits::new(expected.len(),8)).unwrap(), value);
        }
        assert!(RequestUnitsCodec.decode(&[0,1,2,0,0,0,1,2,255,255], Limits::new(10,1)).is_err());
        assert!(RequestUnitsCodec.decode(&[0,1,255,255,255,255], Limits::new(6,8)).is_err());
    "#,
    );
}

#[test]
fn nested_optional_owned_vectors_keep_empty_and_absent_distinct() {
    execute(
        option(option(counted(number("u8")))),
        "nested-vector",
        r#"
        for (payload, expected) in [
            (None, vec![0,0]),
            (Some(None), vec![0,1,0]),
            (Some(Some(vec![])), vec![0,1,1,0,0,0,0]),
            (Some(Some(vec![147u8])), vec![0,1,1,1,0,0,0,147]),
        ] {
            let value = RequestUnits { payload };
            assert_eq!(RequestUnitsCodec.encode(&value, expected.len()).unwrap(), expected);
            assert_eq!(RequestUnitsCodec.decode(&expected, Limits::new(expected.len(),8)).unwrap(), value);
        }
    "#,
    );
}

#[test]
fn nested_counted_slice_encoders_preserve_empty_inner_vectors() {
    execute(
        counted(counted(number("u8"))),
        "slices",
        r#"
        for (payload, expected) in [
            (vec![], vec![0,0,0,0,0]),
            (vec![vec![],vec![147u8]], vec![0,2,0,0,0,0,0,0,0,1,0,0,0,147]),
        ] {
            let value = RequestUnits { payload };
            assert_eq!(RequestUnitsCodec.encode(&value, expected.len()).unwrap(), expected);
            let limits=Limits::new(expected.len(),8);
            assert_eq!(RequestUnitsCodec.decode(&expected, limits).unwrap(), value);
            assert!(RequestUnitsCodec.decode(&expected[..expected.len()-1], limits).is_err());
            let mut trailing=expected.clone(); trailing.push(0);
            assert!(RequestUnitsCodec.decode(&trailing, Limits::new(trailing.len(),8)).is_err());
        }
    "#,
    );
}

#[test]
fn framed_text_arrays_encode_independent_string_lengths() {
    execute(
        counted(text()),
        "text-array",
        r#"
        let value=RequestUnits {payload:vec![String::new(),String::from("é")]};
        let expected=[0,2,0,0,0,0,0,0,0,2,0,0,0,195,169];
        assert_eq!(RequestUnitsCodec.encode(&value,expected.len()).unwrap(),expected);
        // Two array entries and the two UTF-8 bytes share one allocation budget.
        assert_eq!(RequestUnitsCodec.decode(&expected,Limits::new(expected.len(),4)).unwrap(),value);
        for budget in [3,2] {
            assert_eq!(RequestUnitsCodec.decode(&expected,Limits::new(expected.len(),budget)),Err(Error::LimitExceeded));
        }
        assert!(RequestUnitsCodec.decode(&expected,Limits::new(expected.len(),1)).is_err());
    "#,
    );
    let root = source(counted(json!({"kind":"stringTypeNode","encoding":"utf8"})));
    assert_eq!(
        render_complete_program(&root, &policy())
            .unwrap_err()
            .to_string(),
        "borrowed payload needs an owned codec binding"
    );
}

#[test]
fn fixed_slice_and_tuple_composition_use_the_same_owned_vector_adapter() {
    let fixed = json!({"kind":"arrayTypeNode","item":counted(number("u8")),
        "count":{"kind":"fixedCountNode","value":2}});
    execute(
        fixed,
        "fixed-slices",
        r#"
        let value=RequestUnits {payload:vec![vec![7u8],vec![]]};
        let expected=[0,1,0,0,0,7,0,0,0,0];
        assert_eq!(RequestUnitsCodec.encode(&value,expected.len()).unwrap(),expected);
        assert_eq!(RequestUnitsCodec.decode(&expected,Limits::new(expected.len(),4)).unwrap(),value);
    "#,
    );
    let tuple = json!({"kind":"tupleTypeNode","items":[counted(number("u8")),text()]});
    execute(
        tuple,
        "tuple",
        r#"
        let value=RequestUnits {payload:(vec![147u8],String::from("a"))};
        let expected=[0,1,0,0,0,147,1,0,0,0,97];
        assert_eq!(RequestUnitsCodec.encode(&value,expected.len()).unwrap(),expected);
        assert_eq!(RequestUnitsCodec.decode(&expected,Limits::new(expected.len(),4)).unwrap(),value);
    "#,
    );
}
