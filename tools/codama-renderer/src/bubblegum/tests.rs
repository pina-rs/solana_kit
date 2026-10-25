//! Complete source controls and independent MIT Dart discriminator literals.
use super::*;
use std::collections::BTreeSet;

const FIXTURES: &str = include_str!("../../fixtures/mpl-bubblegum/instructions.tsv");

#[test]
fn every_literal_header_and_ordered_privilege_contract_is_present() {
    let root = source().unwrap();
    let instructions = root["program"]["instructions"].as_array().unwrap();
    let mut names = BTreeSet::new();

    for row in FIXTURES.lines().skip(1) {
        let fields = row.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 5);
        assert!(names.insert(fields[0]));
        let node = instructions
            .iter()
            .find(|item| item["name"] == fields[0])
            .unwrap();
        let expected = fields[1]
            .split(',')
            .map(|byte| format!("{:02x}", byte.parse::<u8>().unwrap()))
            .collect::<String>();
        assert_eq!(node["arguments"][0]["defaultValue"]["data"], expected);
        verify_roles(node, fields[0], fields[3]);
    }

    assert_eq!(names.len(), 36);
    assert_eq!(instructions.len(), 36);
    assert_eq!(root["program"]["accounts"].as_array().unwrap().len(), 2);
    assert_eq!(root["program"]["errors"].as_array().unwrap().len(), 60);
    optional_roles(instructions);
    assert!(render(&root).unwrap().contains("pub struct Voucher"));
}

fn verify_roles(node: &Value, operation: &str, flags: &str) {
    let roles = node["accounts"].as_array().unwrap();

    let flags = flags
        .chars()
        .filter(|flag| !flag.is_whitespace())
        .collect::<Vec<_>>();
    assert_eq!(roles.len(), flags.len(), "{operation}");

    for (index, (role, flag)) in roles.iter().zip(flags).enumerate() {
        let conditional = ["burn", "transfer"].contains(&operation) && [1, 2].contains(&index);
        let expected = if conditional {
            json!("either")
        } else {
            json!(matches!(flag, 's' | 'S' | 'W'))
        };

        assert_eq!(role["isSigner"], expected, "{operation}");
        assert_eq!(role["isWritable"], matches!(flag, 'w' | 'W'), "{operation}");
    }
}

fn optional_roles(instructions: &[Value]) {
    let fixture = include_str!("../../fixtures/mpl-bubblegum/optional-accounts.tsv");

    for instruction in instructions {
        let name = instruction["name"].as_str().unwrap();
        let expected = fixture
            .lines()
            .skip(1)
            .find_map(|row| {
                row.split_once('\t')
                    .filter(|(operation, _)| *operation == name)
                    .map(|(_, indices)| {
                        indices
                            .split(',')
                            .map(|index| index.parse::<usize>().unwrap())
                            .collect::<Vec<_>>()
                    })
            })
            .unwrap_or_default();
        let actual = instruction["accounts"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(index, role)| (role["isOptional"] == true).then_some(index))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected, "{name}");
    }
}

#[test]
fn invalid_models_fail_during_rendering() {
    let original = source().unwrap();

    for property in ["unexpected", "isOptional", "range"] {
        let mut changed = original.clone();
        changed["program"]["instructions"][0][property] = Value::Bool(true);
        assert!(render(&changed).is_err());
    }

    let mut duplicate = original.clone();
    let first = duplicate["program"]["instructions"][0].clone();
    duplicate["program"]["instructions"]
        .as_array_mut()
        .unwrap()
        .push(first);
    assert!(render(&duplicate).is_err());
    let mut invalid = original.clone();
    invalid["program"]["definedTypes"][0]["type"]["fields"][0]["type"]["kind"] =
        json!("foreignTypeNode");
    assert!(render(&invalid).is_err());

    let mut framing = original;
    framing["program"]["accounts"][1]["discriminators"] =
        framing["program"]["accounts"][0]["discriminators"].clone();
    assert!(render(&framing).is_err());
}
