//! Selected program identity must resolve before atomic builder replacement.

use crate::builders;
use serde_json::{Value, json};
use solana_kit_core::Address;
use std::{fs, path::Path};

fn root() -> Value {
    serde_json::from_str(include_str!("../fixtures/system.json")).unwrap()
}

fn foreign_program() -> Value {
    let address = Address::from_bytes([1; 32]).to_string();
    assert_eq!(address.parse::<Address>().unwrap().as_bytes(), &[1; 32]);
    json!(address)
}

#[test]
fn foreign_valid_program_cannot_render_fixed_system_builders() {
    let mut malformed = root();
    malformed["program"]["publicKey"] = foreign_program();
    assert!(builders::render(&malformed).is_err());
}

#[test]
fn invalid_selected_program_preserves_all_destination_states() {
    let directory =
        std::env::temp_dir().join(format!("system-program-binding-{}", std::process::id()));
    fs::create_dir(&directory).unwrap();
    let output = directory.join("builders.rs");
    let mut missing = root();
    missing["program"]
        .as_object_mut()
        .unwrap()
        .remove("publicKey");
    reject(&missing, &output);

    for identity in [
        foreign_program(),
        Value::Null,
        json!(false),
        json!(1),
        json!([]),
        json!({}),
        json!("invalid"),
        json!("111111111111111111111111111111111"),
        json!(" 11111111111111111111111111111111"),
    ] {
        let mut malformed = root();
        malformed["program"]["publicKey"] = identity;
        reject(&malformed, &output);
    }

    builders::replace(&root(), &output).unwrap();
    let first = fs::read(&output).unwrap();
    builders::replace(&root(), &output).unwrap();
    assert_eq!(fs::read(&output).unwrap(), first);
    assert_eq!(
        first,
        include_bytes!("../../../crates/solana_kit_system/src/builders.rs")
    );
    fs::remove_dir_all(directory).unwrap();
}

fn reject(malformed: &Value, output: &Path) {
    for (existing, staged) in [(false, false), (true, false), (true, true), (false, true)] {
        reject_state(malformed, output, existing, staged);
    }
}

fn reject_state(malformed: &Value, output: &Path, existing: bool, staged: bool) {
    let pending = output.with_extension("rs.pending");

    if existing {
        fs::write(output, "output sentinel").unwrap();
    }

    if staged {
        fs::write(&pending, "pending sentinel").unwrap();
    }

    let error = builders::replace(malformed, output).unwrap_err();
    assert_eq!(
        error.to_string(),
        "unsupported pinned builder program identity"
    );
    assert_eq!(output.exists(), existing);
    assert_eq!(pending.exists(), staged);

    if existing {
        assert_eq!(fs::read_to_string(output).unwrap(), "output sentinel");
        fs::remove_file(output).unwrap();
    }

    if staged {
        assert_eq!(fs::read_to_string(&pending).unwrap(), "pending sentinel");
        fs::remove_file(&pending).unwrap();
    }
}
