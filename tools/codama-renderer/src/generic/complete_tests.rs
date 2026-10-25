//! Complete-model capacity and conditional helper names preserve legacy admission.
use super::*;
use serde_json::json;

const LEGACY_DEFINITION_LIMIT: usize = 64;
const COMPLETE_DEFINITION_LIMIT: usize = 256;

fn policy() -> Policy<'static> {
    Policy {
        name: "computeBudget",
        address: "ComputeBudget111111111111111111111111111111"
            .parse()
            .unwrap(),
    }
}

fn fixture(count: usize) -> Value {
    let mut root: Value =
        serde_json::from_str(include_str!("../../fixtures/compute-budget.json")).unwrap();
    root["program"]["instructions"]
        .as_array_mut()
        .unwrap()
        .truncate(1);
    root["program"]["definedTypes"] = Value::Array(
        (0..count)
            .map(|index| {
                json!({"kind":"definedTypeNode","name":format!("CapacityProbe{index}"),
            "type":{"kind":"structTypeNode","fields":[]}})
            })
            .collect(),
    );
    root
}

fn complete(root: &Value) -> Result<String, crate::schema::Error> {
    let mut root = root.clone();
    root["version"] = json!("1.0.0");
    root["program"]["events"] = json!([]);
    root["program"].as_object_mut().unwrap().remove("prefix");
    root["program"].as_object_mut().unwrap().remove("origin");

    for instruction in root["program"]["instructions"].as_array_mut().unwrap() {
        instruction.as_object_mut().unwrap().remove("idlName");
        instruction.as_object_mut().unwrap().remove("display");
    }

    render_complete_program(&root, &policy())
}

#[test]
fn complete_256_and_legacy_64_are_separate_source_budgets() {
    assert!(render(&fixture(LEGACY_DEFINITION_LIMIT), &policy()).is_ok());
    assert_eq!(
        render(&fixture(LEGACY_DEFINITION_LIMIT + 1), &policy())
            .unwrap_err()
            .to_string(),
        "linked definition budget exceeded"
    );
    let rendered = complete(&fixture(COMPLETE_DEFINITION_LIMIT)).unwrap();
    assert!(rendered.contains("pub struct CapacityProbe255Codec"));
    assert_eq!(
        complete(&fixture(COMPLETE_DEFINITION_LIMIT + 1))
            .unwrap_err()
            .to_string(),
        "linked definition budget exceeded"
    );
}

#[test]
fn complete_budget_failure_preserves_destination_and_pending_file_bytes() {
    let directory =
        std::env::temp_dir().join(format!("kit-complete-budget-{}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    let output = directory.join("generated.rs");
    let pending = output.with_extension("rs.pending");
    std::fs::write(&output, b"existing destination").unwrap();
    std::fs::write(&pending, b"existing pending").unwrap();
    let result = complete(&fixture(COMPLETE_DEFINITION_LIMIT + 1))
        .and_then(|source| crate::generate::replace_source(&source, &output));

    assert_eq!(
        result.unwrap_err().to_string(),
        "linked definition budget exceeded"
    );
    assert_eq!(std::fs::read(&output).unwrap(), b"existing destination");
    assert_eq!(std::fs::read(&pending).unwrap(), b"existing pending");
    std::fs::remove_file(output).unwrap();
    std::fs::remove_file(pending).unwrap();
    std::fs::remove_dir(directory).unwrap();
}

#[test]
fn vector_helper_and_codec_names_are_reserved_only_when_the_helper_is_emitted() {
    let mut root = fixture(1);
    root["program"]["definedTypes"][0]["name"] = json!("OwnedVector");
    let rendered = complete(&root).unwrap();
    assert!(rendered.contains("pub struct OwnedVector"));
    assert!(!rendered.contains("struct OwnedVector<C>"));

    let field = json!({"kind":"instructionArgumentNode","name":"values","type":{
        "kind":"optionTypeNode","fixed":false,"prefix":number("u8"),"item":{
            "kind":"arrayTypeNode","item":number("u8"),
            "count":{"kind":"prefixedCountNode","prefix":number("u32")}}}});
    root["program"]["instructions"][0]["arguments"]
        .as_array_mut()
        .unwrap()
        .push(field);

    assert!(complete(&root).is_err(), "accepted helper collision");
    root["program"]["definedTypes"][0]["name"] = json!("OwnedVectorCodec");
    assert!(complete(&root).is_ok());

    root["program"]["definedTypes"][0]["name"] = json!("OwnedSlice");
    let rendered = complete(&root).unwrap();
    assert!(rendered.contains("pub struct OwnedSlice"));
    assert!(rendered.contains("struct OwnedVector<C>"));
    assert!(!rendered.contains("as OwnedSlice"));
}

#[test]
fn legacy_owned_string_name_remains_reserved_without_emission() {
    let mut root = fixture(1);
    root["program"]["definedTypes"][0]["name"] = json!("OwnedString");
    assert_eq!(
        render(&root, &policy()).unwrap_err().to_string(),
        "generated namespace collision"
    );
}

fn number(format: &str) -> Value {
    json!({"kind":"numberTypeNode","format":format,"endian":"le"})
}
