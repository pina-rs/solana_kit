//! Source admission controls are authored here and remain unexecuted in unit390.
use super::*;

fn lowered_definitions(root: &Value) -> usize {
    let types = root["program"]["definedTypes"].as_array().unwrap();
    let payloads = types
        .iter()
        .flat_map(|node| node["type"]["variants"].as_array().into_iter().flatten())
        .filter(|variant| variant["kind"] == "enumStructVariantTypeNode")
        .count();

    types.len() + payloads + root["program"]["accounts"].as_array().unwrap().len()
}

#[test]
fn complete_model_counts_and_nested_owned_vectors_reach_the_shared_renderer() {
    let root = source().unwrap();
    assert_eq!(
        root["program"]["instructions"].as_array().unwrap().len(),
        58
    );
    assert_eq!(root["program"]["accounts"].as_array().unwrap().len(), 14);
    assert_eq!(
        root["program"]["definedTypes"].as_array().unwrap().len(),
        45
    );
    assert_eq!(root["program"]["errors"].as_array().unwrap().len(), 203);
    assert_eq!(lowered_definitions(&root), 94);
    let output = render(&root).unwrap();
    assert!(output.contains("pub struct SeedsVec"));

    assert!(output.contains("pub seeds: Vec<Vec<u8>>"));
    assert!(output.contains("OwnedVector(CheckedCounted"));
    assert!(output.contains("pub enum MplTokenMetadataError"));
}

#[test]
fn source_definition_budget_stays_bounded_and_does_not_change_public_decode_limits() {
    let mut root = source().unwrap();

    for index in 0..163 {
        root["program"]["definedTypes"]
            .as_array_mut()
            .unwrap()
            .push(json!({
            "kind":"definedTypeNode","name":format!("BudgetProbe{index}"),
            "type":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode",
                "name":"byte","type":schema::number("u8")}]} }));
    }

    assert_eq!(lowered_definitions(&root), 257);
    assert!(
        render(&root)
            .unwrap_err()
            .to_string()
            .contains("definition budget")
    );
    root["program"]["definedTypes"]
        .as_array_mut()
        .unwrap()
        .pop();
    assert_eq!(lowered_definitions(&root), 256);
    assert!(render(&root).is_ok());
}

#[test]
fn malformed_complete_models_fail_before_the_atomic_writer() {
    for (pointer, value) in [
        (
            "/program/publicKey",
            json!("11111111111111111111111111111111"),
        ),
        (
            "/program/instructions/0/arguments/0/defaultValue/number",
            json!(1),
        ),
        (
            "/program/instructions/41/optionalAccountStrategy",
            json!("guess"),
        ),
        ("/program/accounts/0/discriminators/0/offset", json!(1)),
        (
            "/program/definedTypes/0/type/fields/0/type/kind",
            json!("rawJsonTypeNode"),
        ),
    ] {
        let mut root = source().unwrap();
        *root.pointer_mut(pointer).unwrap() = value;
        assert!(render(&root).is_err(), "{pointer}");
    }

    let mut root = source().unwrap();
    root["program"]["accounts"][0]["unexpected"] = json!(true);
    assert!(render(&root).is_err());
    root = source().unwrap();
    root["program"]["definedTypes"][0]["name"] = json!("OwnedVector");
    assert!(render(&root).is_err());
}

#[test]
fn rejected_model_preserves_existing_destination_and_pending_bytes() {
    let directory = std::env::temp_dir().join(format!(
        "metadata-model-preservation-{}",
        std::process::id()
    ));
    std::fs::create_dir(&directory).unwrap();
    let output = directory.join("generated.rs");
    let pending = output.with_extension("rs.pending");
    std::fs::write(&output, b"existing-output").unwrap();
    std::fs::write(&pending, b"existing-pending").unwrap();
    let mut root = source().unwrap();
    root["program"]["publicKey"] = json!("11111111111111111111111111111111");

    let result = render(&root).and_then(|source| crate::generate::replace_source(&source, &output));
    assert!(result.is_err());
    assert_eq!(std::fs::read(&output).unwrap(), b"existing-output");
    assert_eq!(std::fs::read(&pending).unwrap(), b"existing-pending");
    std::fs::remove_file(output).unwrap();
    std::fs::remove_file(pending).unwrap();
    std::fs::remove_dir(directory).unwrap();
}
