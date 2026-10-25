use super::*;
use serde_json::json;

#[test]
fn full_model_retains_all_instructions_accounts_named_values_and_error_contracts() {
    let root = source().unwrap();
    assert_eq!(
        root["program"]["instructions"].as_array().unwrap().len(),
        42
    );
    assert_eq!(root["program"]["accounts"].as_array().unwrap().len(), 6);
    assert_eq!(
        root["program"]["definedTypes"].as_array().unwrap().len(),
        76
    );
    assert_eq!(root["program"]["errors"].as_array().unwrap().len(), 57);
    let output = render(&root).unwrap();
    assert!(output.contains("pub enum MplCoreInstructionData"));
    assert!(output.contains("pub struct RelationshipEntry"));

    assert!(!output.contains("pub struct OwnedVector"));
    assert!(output.contains("struct OwnedVector"));
}

#[test]
fn unknown_properties_links_and_duplicate_headers_fail_without_replacing_output() {
    let destination = std::env::temp_dir().join(format!("mpl-core-invalid-{}", std::process::id()));
    std::fs::write(&destination, b"preserved output").unwrap();

    for (path, value) in [
        ("/program/instructions/0/unknown", json!(true)),
        (
            "/program/instructions/1/arguments/0/defaultValue/number",
            json!(0),
        ),
        (
            "/program/instructions/39/arguments/3/type/item/name",
            json!("crate"),
        ),
    ] {
        let mut root = source().unwrap();
        let parent = path.rsplit_once('/').unwrap();
        root.pointer_mut(parent.0).unwrap()[parent.1] = value;
        let result =
            render(&root).and_then(|source| crate::generate::replace_source(&source, &destination));
        assert!(result.is_err());
        assert_eq!(std::fs::read(&destination).unwrap(), b"preserved output");
    }
}

#[test]
fn complete_declaration_budget_is_finite_and_legacy_budget_stays_separate() {
    let mut root = source().unwrap();
    let types = root["program"]["definedTypes"].as_array_mut().unwrap();

    for index in 0..165 {
        types.push(
            json!({"kind":"definedTypeNode","name":format!("OverBudget{index}"),
            "type":{"kind":"structTypeNode","fields":[]}}),
        );
    }

    render(&root).unwrap();
    root["program"]["definedTypes"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "kind":"definedTypeNode","name":"OneOverBudget",
            "type":{"kind":"structTypeNode","fields":[]},
        }));
    let failure = render(&root).unwrap_err().to_string();
    assert_eq!(failure, "linked definition budget exceeded");
}
