//! Public boundary tests are independent of the private proof-dispatch feature.

use super::render_root;
use serde_json::{Value, json};

fn root() -> Value {
    json!({
        "kind": "rootNode", "standard": "codama", "version": "1.8.0",
        "program": {
            "kind": "programNode", "name": "counter",
            "publicKey": "11111111111111111111111111111111", "version": "0.0.0",
            "instructions": [{
                "kind": "instructionNode", "name": "setValue",
                "arguments": [
                    { "kind": "instructionArgumentNode", "name": "discriminator",
                      "type": { "kind": "numberTypeNode", "format": "u8", "endian": "le" },
                      "defaultValue": { "kind": "numberValueNode", "number": 7 },
                      "defaultValueStrategy": "omitted" },
                    { "kind": "instructionArgumentNode", "name": "value",
                      "type": { "kind": "numberTypeNode", "format": "u64", "endian": "le" } }
                ],
                "discriminators": [{ "kind": "fieldDiscriminatorNode", "name": "discriminator" }]
            }]
        }
    })
}

#[test]
fn ordinary_factory_defaults_resolve_before_any_output() {
    let input = root();
    let first = render_root(&input).unwrap();
    assert_eq!(first, render_root(&input).unwrap());
    assert_eq!(
        first
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>(),
        ["counter.rs", "mod.rs"]
    );
    assert!(first[0].source.contains("pub struct SetValue"));
}

#[test]
fn unknown_semantic_properties_and_duplicate_modules_fail() {
    let mut unknown = root();
    unknown["program"]["instructions"][0]["unknownWireConstraint"] = json!(true);
    assert!(render_root(&unknown).is_err());
    let mut duplicate = root();
    duplicate["additionalPrograms"] = json!([duplicate["program"].clone()]);
    assert!(render_root(&duplicate).is_err());
}

#[test]
fn unsupported_versions_and_invalid_addresses_fail_without_files() {
    let mut version = root();
    version["version"] = json!("99.0.0");
    assert!(render_root(&version).is_err());
    let mut address = root();
    address["program"]["publicKey"] = json!("invalid");
    assert!(render_root(&address).is_err());
}

#[test]
fn factory_defaults_preserve_explicit_semantic_constraints() {
    let mut input = root();
    input["program"]["instructions"][0]["discriminators"][0]["offset"] = json!(1);
    assert!(render_root(&input).is_err());

    let mut input = root();
    input["program"]["instructions"][0]["optionalAccountStrategy"] = json!("unsupported");
    assert!(render_root(&input).is_err());
}
