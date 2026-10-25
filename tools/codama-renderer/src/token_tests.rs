//! Whole-input rejection and atomic publication over the expanded shared model.
use crate::{generic, token};
use serde_json::{Value, json};
use std::fs;
const ROOT_SHAPE_CASE_OFFSET: usize = 100;

#[test]
fn token_source_is_deterministic_and_matches_committed_output() {
    let root = token::input().unwrap();
    let frozen: Value =
        serde_json::from_str(include_str!("../fixtures/token-normalized.json")).unwrap();
    assert_eq!(generic::normalize_input(&root).unwrap(), frozen);
    let policy = token::policy().unwrap();
    let path = std::env::temp_dir().join(format!("kit-token-repeat-{}.rs", std::process::id()));

    for _ in 0..2 {
        generic::replace(&root, &policy, &path).unwrap();
        assert_eq!(
            fs::read(&path).unwrap(),
            include_bytes!("../../../crates/solana_kit_token/src/generated.rs")
        );
    }

    let pending = path.with_extension("rs.pending");
    fs::write(&pending, b"pending").unwrap();

    assert!(generic::replace(&root, &policy, &path).is_err());
    assert_eq!(fs::read(&pending).unwrap(), b"pending");
    assert_eq!(
        fs::read(&path).unwrap(),
        include_bytes!("../../../crates/solana_kit_token/src/generated.rs")
    );
    fs::remove_file(path).unwrap();
    fs::remove_file(pending).unwrap();
}

fn mutations() -> Vec<Value> {
    let changes: Vec<(String, Value)> =
        serde_json::from_str(include_str!("../fixtures/token-negative.json")).unwrap();
    let mut output = Vec::new();

    for (pointer, value) in changes {
        let mut root = token::input().unwrap();
        *root.pointer_mut(&pointer).unwrap() = value;
        output.push(root);
    }
    for pointer in [
        "/program/accounts/0/data",
        "/program/accounts/0/data/fields/0/type",
        "/program/instructions/3/accounts/0",
        "/program/instructions/3/remainingAccounts/0",
        "/program/errors/0",
    ] {
        let mut root = token::input().unwrap();
        root.pointer_mut(pointer).unwrap()["unsupportedConstraint"] = json!(1);
        output.push(root);
    }
    let mut cycle = token::input().unwrap();
    cycle["program"]["definedTypes"][0]["type"] = json!({"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","name":"selfLink","type":{"kind":"optionTypeNode","item":{"kind":"definedTypeLinkNode","name":"accountState"},"prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"},"fixed":false}}]});

    output.push(cycle);
    output
}

#[test]
fn every_expanded_semantic_rejection_preserves_output_and_pending_states() {
    for (index, root) in mutations().iter().enumerate() {
        for state in 0..4 {
            preserve_state(root, index, state);
        }
    }
}

/// Each mutation runs against all four output/pending existence combinations.
fn preserve_state(root: &Value, index: usize, state: u8) {
    let path = std::env::temp_dir().join(format!(
        "kit-token-reject-{}-{index}-{state}.rs",
        std::process::id()
    ));
    let pending = path.with_extension("rs.pending");

    if state & 1 != 0 {
        fs::write(&path, b"output").unwrap();
    }

    if state & 2 != 0 {
        fs::write(&pending, b"pending").unwrap();
    }

    assert!(
        generic::replace(root, &token::policy().unwrap(), &path).is_err(),
        "mutation {index}"
    );
    assert_eq!(path.exists(), state & 1 != 0);
    assert_eq!(pending.exists(), state & 2 != 0);

    if path.exists() {
        assert_eq!(fs::read(&path).unwrap(), b"output");
        fs::remove_file(&path).unwrap();
    }

    if pending.exists() {
        assert_eq!(fs::read(&pending).unwrap(), b"pending");
        fs::remove_file(&pending).unwrap();
    }
}

#[test]
fn nonobject_roots_reject_before_any_output_or_pending_mutation() {
    let roots = [
        json!(null),
        json!(false),
        json!("root"),
        json!([]),
        json!({"program":false}),
        json!({"program":null}),
    ];

    for (index, root) in roots.iter().enumerate() {
        for state in 0..4 {
            preserve_state(root, ROOT_SHAPE_CASE_OFFSET + index, state);
        }
    }
}
